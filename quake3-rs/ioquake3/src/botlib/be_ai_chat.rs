// =============== BEGIN be_ai_chat_h ================
pub type bot_consolemessage_t = crate::src::botlib::be_ai_chat::bot_consolemessage_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bot_consolemessage_s {
    pub handle: i32,
    pub time: f32,
    pub type_0: i32,
    pub message: [libc::c_char; 256],
    pub prev: *mut crate::src::botlib::be_ai_chat::bot_consolemessage_s,
    pub next: *mut crate::src::botlib::be_ai_chat::bot_consolemessage_s,
}

pub type bot_matchvariable_t = crate::src::botlib::be_ai_chat::bot_matchvariable_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bot_matchvariable_s {
    pub offset: libc::c_char,
    pub length: i32,
}

pub type bot_match_t = crate::src::botlib::be_ai_chat::bot_match_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bot_match_s {
    pub string: [libc::c_char; 256],
    pub type_0: i32,
    pub subtype: i32,
    pub variables: [crate::src::botlib::be_ai_chat::bot_matchvariable_t; 8],
}
use ::libc;

pub mod ctype_h {
    #[inline]

    pub unsafe extern "C" fn toupper(mut __c: i32) -> i32 {
        return if __c >= -(128 as i32) && __c < 256 as i32 {
            *(*crate::stdlib::__ctype_toupper_loc()).offset(__c as isize)
        } else {
            __c
        };
    }
}

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
pub use crate::src::botlib::l_precomp::PC_CheckTokenString;
pub use crate::src::botlib::l_precomp::PC_ExpectAnyToken;
pub use crate::src::botlib::l_precomp::PC_ExpectTokenString;
pub use crate::src::botlib::l_precomp::PC_ExpectTokenType;
pub use crate::src::botlib::l_precomp::PC_ReadToken;
pub use crate::src::botlib::l_precomp::PC_SetBaseFolder;
pub use crate::src::botlib::l_precomp::PC_UnreadLastToken;
pub use crate::src::botlib::l_precomp::SourceError;
pub use crate::src::botlib::l_precomp::SourceWarning;
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
pub use crate::src::qcommon::q_shared::va;
pub use crate::src::qcommon::q_shared::vec3_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::qcommon::q_shared::Q_stricmp;
pub use crate::src::qcommon::q_shared::Q_strncpyz;
pub use crate::src::qcommon::q_shared::FS_APPEND;
pub use crate::src::qcommon::q_shared::FS_APPEND_SYNC;
pub use crate::src::qcommon::q_shared::FS_READ;
pub use crate::src::qcommon::q_shared::FS_WRITE;

pub use crate::src::botlib::be_ai_chat::ctype_h::toupper;

pub use crate::stdlib::__ctype_toupper_loc;
//reply chat

pub type bot_replychat_t = bot_replychat_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bot_replychat_s {
    pub keys: *mut bot_replychatkey_t,
    pub priority: f32,
    pub numchatmessages: i32,
    pub firstchatmessage: *mut bot_chatmessage_t,
    pub next: *mut bot_replychat_s,
}
//the actuall chat messages

pub type bot_chatmessage_t = bot_chatmessage_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bot_chatmessage_s {
    pub chatmessage: *mut libc::c_char,
    pub time: f32,
    pub next: *mut bot_chatmessage_s,
}
//chat message string
//last time used
//next chat message in a list
//reply chat key

pub type bot_replychatkey_t = bot_replychatkey_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bot_replychatkey_s {
    pub flags: i32,
    pub string: *mut libc::c_char,
    pub match_0: *mut bot_matchpiece_t,
    pub next: *mut bot_replychatkey_s,
}
//piece of a match template

pub type bot_matchpiece_t = bot_matchpiece_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bot_matchpiece_s {
    pub type_0: i32,
    pub firststring: *mut bot_matchstring_t,
    pub variable: i32,
    pub next: *mut bot_matchpiece_s,
}
//fixed match string

pub type bot_matchstring_t = bot_matchstring_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bot_matchstring_s {
    pub string: *mut libc::c_char,
    pub next: *mut bot_matchstring_s,
}
//list with random strings

pub type bot_randomlist_t = bot_randomlist_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bot_randomlist_s {
    pub string: *mut libc::c_char,
    pub numstrings: i32,
    pub firstrandomstring: *mut bot_randomstring_t,
    pub next: *mut bot_randomlist_s,
}
//random string

pub type bot_randomstring_t = bot_randomstring_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bot_randomstring_s {
    pub string: *mut libc::c_char,
    pub next: *mut bot_randomstring_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bot_matchtemplate_s {
    pub context: libc::c_ulong,
    pub type_0: i32,
    pub subtype: i32,
    pub first: *mut bot_matchpiece_t,
    pub next: *mut bot_matchtemplate_s,
}
//list with synonyms

pub type bot_synonymlist_t = bot_synonymlist_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bot_synonymlist_s {
    pub context: libc::c_ulong,
    pub totalweight: f32,
    pub firstsynonym: *mut bot_synonym_t,
    pub next: *mut bot_synonymlist_s,
}
//synonym

pub type bot_synonym_t = bot_synonym_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bot_synonym_s {
    pub string: *mut libc::c_char,
    pub weight: f32,
    pub next: *mut bot_synonym_s,
}
//chat state of a bot

pub type bot_chatstate_t = bot_chatstate_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bot_chatstate_s {
    pub gender: i32,
    pub client: i32,
    pub name: [libc::c_char; 32],
    pub chatmessage: [libc::c_char; 256],
    pub handle: i32,
    pub firstmessage: *mut crate::src::botlib::be_ai_chat::bot_consolemessage_t,
    pub lastmessage: *mut crate::src::botlib::be_ai_chat::bot_consolemessage_t,
    pub numconsolemessages: i32,
    pub chat: *mut bot_chat_t,
}
//0=it, 1=female, 2=male
//client number
//name of the bot
//the console messages visible to the bot
//first message is the first typed message
//last message is the last typed message, bottom of console
//number of console messages stored in the state
//the bot chat lines
//bot chat lines

pub type bot_chat_t = bot_chat_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bot_chat_s {
    pub types: *mut bot_chattype_t,
}
//bot chat type with chat lines

pub type bot_chattype_t = bot_chattype_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bot_chattype_s {
    pub name: [libc::c_char; 32],
    pub numchatmessages: i32,
    pub firstchatmessage: *mut bot_chatmessage_t,
    pub next: *mut bot_chattype_s,
}
//match template

pub type bot_matchtemplate_t = bot_matchtemplate_s;
//string list

pub type bot_stringlist_t = bot_stringlist_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bot_stringlist_s {
    pub string: *mut libc::c_char,
    pub next: *mut bot_stringlist_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bot_ichatdata_t {
    pub chat: *mut bot_chat_t,
    pub filename: [libc::c_char; 64],
    pub chatname: [libc::c_char; 64],
}
#[no_mangle]

pub static mut ichatdata: [*mut bot_ichatdata_t; 64] =
    [0 as *const bot_ichatdata_t as *mut bot_ichatdata_t; 64];
#[no_mangle]

pub static mut botchatstates: [*mut bot_chatstate_t; 65] =
    [0 as *const bot_chatstate_t as *mut bot_chatstate_t; 65];
//console message heap
#[no_mangle]

pub static mut consolemessageheap: *mut crate::src::botlib::be_ai_chat::bot_consolemessage_t = 0
    as *const crate::src::botlib::be_ai_chat::bot_consolemessage_t
    as *mut crate::src::botlib::be_ai_chat::bot_consolemessage_t;
#[no_mangle]

pub static mut freeconsolemessages: *mut crate::src::botlib::be_ai_chat::bot_consolemessage_t = 0
    as *const crate::src::botlib::be_ai_chat::bot_consolemessage_t
    as *mut crate::src::botlib::be_ai_chat::bot_consolemessage_t;
//list with match strings
#[no_mangle]

pub static mut matchtemplates: *mut bot_matchtemplate_t =
    0 as *const bot_matchtemplate_t as *mut bot_matchtemplate_t;
//list with synonyms
#[no_mangle]

pub static mut synonyms: *mut bot_synonymlist_t =
    0 as *const bot_synonymlist_t as *mut bot_synonymlist_t;
//list with random strings
#[no_mangle]

pub static mut randomstrings: *mut bot_randomlist_t =
    0 as *const bot_randomlist_t as *mut bot_randomlist_t;
//reply chats
#[no_mangle]

pub static mut replychats: *mut bot_replychat_t =
    0 as *const bot_replychat_t as *mut bot_replychat_t;
//========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotChatStateFromHandle(mut handle: i32) -> *mut bot_chatstate_t {
    if handle <= 0 as i32 || handle > 64 as i32 {
        crate::src::botlib::be_interface::botimport
            .Print
            .expect("non-null function pointer")(
            4 as i32,
            b"chat state handle %d out of range\n\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            handle,
        ); //end if
        return 0 as *mut bot_chatstate_t;
    } //end if
    if botchatstates[handle as usize].is_null() {
        crate::src::botlib::be_interface::botimport
            .Print
            .expect("non-null function pointer")(
            4 as i32,
            b"invalid chat state %d\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            handle,
        );
        return 0 as *mut bot_chatstate_t;
    }
    return botchatstates[handle as usize];
}
//end of the function BotChatStateFromHandle
//===========================================================================
// initialize the heap with unused console messages
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn InitConsoleMessageHeap() {
    let mut i: i32 = 0;
    let mut max_messages: i32 = 0;
    if !consolemessageheap.is_null() {
        crate::src::botlib::l_memory::FreeMemory(consolemessageheap as *mut libc::c_void);
    }
    //
    max_messages = crate::src::botlib::l_libvar::LibVarValue(
        b"max_messages\x00" as *const u8 as *const libc::c_char,
        b"1024\x00" as *const u8 as *const libc::c_char,
    ) as i32; //end for
    consolemessageheap = crate::src::botlib::l_memory::GetClearedHunkMemory(
        (max_messages as libc::c_ulong).wrapping_mul(::std::mem::size_of::<
            crate::src::botlib::be_ai_chat::bot_consolemessage_t,
        >() as libc::c_ulong),
    ) as *mut crate::src::botlib::be_ai_chat::bot_consolemessage_t;
    let ref mut fresh0 = (*consolemessageheap.offset(0 as i32 as isize)).prev;
    *fresh0 = 0 as *mut crate::src::botlib::be_ai_chat::bot_consolemessage_s;
    let ref mut fresh1 = (*consolemessageheap.offset(0 as i32 as isize)).next;
    *fresh1 = &mut *consolemessageheap.offset(1 as i32 as isize)
        as *mut crate::src::botlib::be_ai_chat::bot_consolemessage_t;
    i = 1 as i32;
    while i < max_messages - 1 as i32 {
        let ref mut fresh2 = (*consolemessageheap.offset(i as isize)).prev;
        *fresh2 = &mut *consolemessageheap.offset((i - 1 as i32) as isize)
            as *mut crate::src::botlib::be_ai_chat::bot_consolemessage_t;
        let ref mut fresh3 = (*consolemessageheap.offset(i as isize)).next;
        *fresh3 = &mut *consolemessageheap.offset((i + 1 as i32) as isize)
            as *mut crate::src::botlib::be_ai_chat::bot_consolemessage_t;
        i += 1
    }
    let ref mut fresh4 = (*consolemessageheap.offset((max_messages - 1 as i32) as isize)).prev;
    *fresh4 = &mut *consolemessageheap.offset((max_messages - 2 as i32) as isize)
        as *mut crate::src::botlib::be_ai_chat::bot_consolemessage_t;
    let ref mut fresh5 = (*consolemessageheap.offset((max_messages - 1 as i32) as isize)).next;
    *fresh5 = 0 as *mut crate::src::botlib::be_ai_chat::bot_consolemessage_s;
    //pointer to the free console messages
    freeconsolemessages = consolemessageheap;
}
//end of the function InitConsoleMessageHeap
//===========================================================================
// allocate one console message from the heap
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AllocConsoleMessage(
) -> *mut crate::src::botlib::be_ai_chat::bot_consolemessage_t {
    let mut message: *mut crate::src::botlib::be_ai_chat::bot_consolemessage_t =
        0 as *mut crate::src::botlib::be_ai_chat::bot_consolemessage_t;
    message = freeconsolemessages;
    if !freeconsolemessages.is_null() {
        freeconsolemessages = (*freeconsolemessages).next
    }
    if !freeconsolemessages.is_null() {
        (*freeconsolemessages).prev = 0 as *mut crate::src::botlib::be_ai_chat::bot_consolemessage_s
    }
    return message;
}
//end of the function AllocConsoleMessage
//===========================================================================
// deallocate one console message from the heap
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn FreeConsoleMessage(
    mut message: *mut crate::src::botlib::be_ai_chat::bot_consolemessage_t,
) {
    if !freeconsolemessages.is_null() {
        (*freeconsolemessages).prev = message
    }
    (*message).prev = 0 as *mut crate::src::botlib::be_ai_chat::bot_consolemessage_s;
    (*message).next = freeconsolemessages;
    freeconsolemessages = message;
}
//removes the console message from the chat state
//end of the function FreeConsoleMessage
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotRemoveConsoleMessage(mut chatstate: i32, mut handle: i32) {
    let mut m: *mut crate::src::botlib::be_ai_chat::bot_consolemessage_t =
        0 as *mut crate::src::botlib::be_ai_chat::bot_consolemessage_t;
    let mut nextm: *mut crate::src::botlib::be_ai_chat::bot_consolemessage_t =
        0 as *mut crate::src::botlib::be_ai_chat::bot_consolemessage_t;
    let mut cs: *mut bot_chatstate_t = 0 as *mut bot_chatstate_t;
    cs = BotChatStateFromHandle(chatstate);
    if cs.is_null() {
        return;
    }
    m = (*cs).firstmessage;
    while !m.is_null() {
        nextm = (*m).next;
        if (*m).handle == handle {
            if !(*m).next.is_null() {
                (*(*m).next).prev = (*m).prev
            } else {
                (*cs).lastmessage = (*m).prev
            }
            if !(*m).prev.is_null() {
                (*(*m).prev).next = (*m).next
            } else {
                (*cs).firstmessage = (*m).next
            }
            FreeConsoleMessage(m);
            (*cs).numconsolemessages -= 1;
            break;
        } else {
            m = nextm
        }
        //end if
    }
    //end for
}
//adds a console message to the chat state
//end of the function BotRemoveConsoleMessage
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotQueueConsoleMessage(
    mut chatstate: i32,
    mut type_0: i32,
    mut message: *mut libc::c_char,
) {
    let mut m: *mut crate::src::botlib::be_ai_chat::bot_consolemessage_t =
        0 as *mut crate::src::botlib::be_ai_chat::bot_consolemessage_t; //end if
    let mut cs: *mut bot_chatstate_t = 0 as *mut bot_chatstate_t; //end if
    cs = BotChatStateFromHandle(chatstate); //end if
    if cs.is_null() {
        return;
    }
    m = AllocConsoleMessage();
    if m.is_null() {
        crate::src::botlib::be_interface::botimport
            .Print
            .expect("non-null function pointer")(
            3 as i32,
            b"empty console message heap\n\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return;
    }
    (*cs).handle += 1;
    if (*cs).handle <= 0 as i32 || (*cs).handle > 8192 as i32 {
        (*cs).handle = 1 as i32
    }
    (*m).handle = (*cs).handle;
    (*m).time = crate::src::botlib::be_aas_main::AAS_Time();
    (*m).type_0 = type_0;
    crate::src::qcommon::q_shared::Q_strncpyz((*m).message.as_mut_ptr(), message, 256 as i32);
    (*m).next = 0 as *mut crate::src::botlib::be_ai_chat::bot_consolemessage_s;
    if !(*cs).lastmessage.is_null() {
        (*(*cs).lastmessage).next = m;
        (*m).prev = (*cs).lastmessage;
        (*cs).lastmessage = m
    } else {
        (*cs).lastmessage = m;
        (*cs).firstmessage = m;
        (*m).prev = 0 as *mut crate::src::botlib::be_ai_chat::bot_consolemessage_s
    }
    (*cs).numconsolemessages += 1;
}
//returns the next console message from the state
//end of the function BotQueueConsoleMessage
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotNextConsoleMessage(
    mut chatstate: i32,
    mut cm: *mut crate::src::botlib::be_ai_chat::bot_consolemessage_t,
) -> i32 {
    let mut cs: *mut bot_chatstate_t = 0 as *mut bot_chatstate_t; //end if
    let mut firstmsg: *mut crate::src::botlib::be_ai_chat::bot_consolemessage_t =
        0 as *mut crate::src::botlib::be_ai_chat::bot_consolemessage_t;
    cs = BotChatStateFromHandle(chatstate);
    if cs.is_null() {
        return 0 as i32;
    }
    firstmsg = (*cs).firstmessage;
    if !firstmsg.is_null() {
        (*cm).handle = (*firstmsg).handle;
        (*cm).time = (*firstmsg).time;
        (*cm).type_0 = (*firstmsg).type_0;
        crate::src::qcommon::q_shared::Q_strncpyz(
            (*cm).message.as_mut_ptr(),
            (*firstmsg).message.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as i32,
        );
        /* We omit setting the two pointers in cm because pointer
         * size in the VM differs between the size in the engine on
         * 64 bit machines, which would lead to a buffer overflow if
         * this functions is called from the VM. The pointers are
         * of no interest to functions calling
         * BotNextConsoleMessage anyways.
         */
        return (*cm).handle;
    }
    return 0 as i32;
}
//returns the number of console messages currently stored in the state
//end of the function BotConsoleMessage
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotNumConsoleMessages(mut chatstate: i32) -> i32 {
    let mut cs: *mut bot_chatstate_t = 0 as *mut bot_chatstate_t;
    cs = BotChatStateFromHandle(chatstate);
    if cs.is_null() {
        return 0 as i32;
    }
    return (*cs).numconsolemessages;
}
//end of the function BotNumConsoleMessages
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn IsWhiteSpace(mut c: libc::c_char) -> i32 {
    if c as i32 >= 'a' as i32 && c as i32 <= 'z' as i32
        || c as i32 >= 'A' as i32 && c as i32 <= 'Z' as i32
        || c as i32 >= '0' as i32 && c as i32 <= '9' as i32
        || c as i32 == '(' as i32
        || c as i32 == ')' as i32
        || c as i32 == '?' as i32
        || c as i32 == ':' as i32
        || c as i32 == '\'' as i32
        || c as i32 == '/' as i32
        || c as i32 == ',' as i32
        || c as i32 == '.' as i32
        || c as i32 == '[' as i32
        || c as i32 == ']' as i32
        || c as i32 == '-' as i32
        || c as i32 == '_' as i32
        || c as i32 == '+' as i32
        || c as i32 == '=' as i32
    {
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    return crate::src::qcommon::q_shared::qtrue as i32;
}
//end of the function IsWhiteSpace
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotRemoveTildes(mut message: *mut libc::c_char) {
    let mut i: i32 = 0;
    //remove all tildes from the chat message
    i = 0 as i32;
    while *message.offset(i as isize) != 0 {
        if *message.offset(i as isize) as i32 == '~' as i32 {
            crate::stdlib::memmove(
                &mut *message.offset(i as isize) as *mut libc::c_char as *mut libc::c_void,
                &mut *message.offset((i + 1 as i32) as isize) as *mut libc::c_char
                    as *const libc::c_void,
                crate::stdlib::strlen(&mut *message.offset((i + 1 as i32) as isize))
                    .wrapping_add(1 as i32 as libc::c_ulong),
            );
        }
        i += 1
        //end if
    }
    //end for
}
//unify all the white spaces in the string
//end of the function BotRemoveTildes
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn UnifyWhiteSpaces(mut string: *mut libc::c_char) {
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char; //end if
    let mut oldptr: *mut libc::c_char = 0 as *mut libc::c_char;
    oldptr = string;
    ptr = oldptr;
    while *ptr != 0 {
        while *ptr as i32 != 0 && IsWhiteSpace(*ptr) != 0 {
            ptr = ptr.offset(1)
        }
        if ptr > oldptr {
            //if not at the start and not at the end of the string
            //write only one space
            if oldptr > string && *ptr as i32 != 0 {
                let fresh6 = oldptr;
                oldptr = oldptr.offset(1);
                *fresh6 = ' ' as i32 as libc::c_char
            }
            //remove all other white spaces
            if ptr > oldptr {
                crate::stdlib::memmove(
                    oldptr as *mut libc::c_void,
                    ptr as *const libc::c_void,
                    crate::stdlib::strlen(ptr).wrapping_add(1 as i32 as libc::c_ulong),
                );
            }
        }
        while *ptr as i32 != 0 && IsWhiteSpace(*ptr) == 0 {
            ptr = ptr.offset(1)
        }
        oldptr = ptr
    }
    //end while
}
//checks if the first string contains the second one, returns index into first string or -1 if not found
//end of the function UnifyWhiteSpaces
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn StringContains(
    mut str1: *mut libc::c_char,
    mut str2: *mut libc::c_char,
    mut casesensitive: i32,
) -> i32 {
    let mut len: i32 = 0; //end for
    let mut i: i32 = 0; //end for
    let mut j: i32 = 0; //end if
    let mut index: i32 = 0;
    if str1.is_null() || str2.is_null() {
        return -(1 as i32);
    }
    len = crate::stdlib::strlen(str1).wrapping_sub(crate::stdlib::strlen(str2)) as i32;
    index = 0 as i32;
    i = 0 as i32;
    while i <= len {
        j = 0 as i32;
        while *str2.offset(j as isize) != 0 {
            if casesensitive != 0 {
                if *str1.offset(j as isize) as i32 != *str2.offset(j as isize) as i32 {
                    break;
                }
            } else if ({
                let mut __res: i32 = 0;
                if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                    > 1 as i32 as libc::c_ulong
                {
                    if 0 != 0 {
                        let mut __c: i32 = *str1.offset(j as isize) as i32;
                        __res = if __c < -(128 as i32) || __c > 255 as i32 {
                            __c
                        } else {
                            *(*crate::stdlib::__ctype_toupper_loc()).offset(__c as isize)
                        }
                    } else {
                        __res = toupper(*str1.offset(j as isize) as i32)
                    }
                } else {
                    __res = *(*crate::stdlib::__ctype_toupper_loc())
                        .offset(*str1.offset(j as isize) as i32 as isize)
                }
                __res
            }) != ({
                let mut __res: i32 = 0;
                if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                    > 1 as i32 as libc::c_ulong
                {
                    if 0 != 0 {
                        let mut __c: i32 = *str2.offset(j as isize) as i32;
                        __res = if __c < -(128 as i32) || __c > 255 as i32 {
                            __c
                        } else {
                            *(*crate::stdlib::__ctype_toupper_loc()).offset(__c as isize)
                        }
                    } else {
                        __res = toupper(*str2.offset(j as isize) as i32)
                    }
                } else {
                    __res = *(*crate::stdlib::__ctype_toupper_loc())
                        .offset(*str2.offset(j as isize) as i32 as isize)
                }
                __res
            }) {
                break;
            }
            j += 1
            //end else
        }
        if *str2.offset(j as isize) == 0 {
            return index;
        }
        i += 1;
        str1 = str1.offset(1);
        index += 1
    }
    return -(1 as i32);
}
//end of the function StringContains
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn StringContainsWord(
    mut str1: *mut libc::c_char,
    mut str2: *mut libc::c_char,
    mut casesensitive: i32,
) -> *mut libc::c_char {
    let mut len: i32 = 0; //end for
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    len = crate::stdlib::strlen(str1).wrapping_sub(crate::stdlib::strlen(str2)) as i32;
    i = 0 as i32;
    while i <= len {
        //if not at the start of the string
        if i != 0 {
            //end for
            //skip to the start of the next word
            while *str1 as i32 != 0
                && *str1 as i32 != ' ' as i32
                && *str1 as i32 != '.' as i32
                && *str1 as i32 != ',' as i32
                && *str1 as i32 != '!' as i32
            {
                str1 = str1.offset(1)
            }
            if *str1 == 0 {
                break;
            }
            str1 = str1.offset(1)
        }
        //compare the word
        j = 0 as i32; //end for
        while *str2.offset(j as isize) != 0 {
            if casesensitive != 0 {
                if *str1.offset(j as isize) as i32 != *str2.offset(j as isize) as i32 {
                    break; //end if
                }
            } else if ({
                let mut __res: i32 = 0;
                if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                    > 1 as i32 as libc::c_ulong
                {
                    if 0 != 0 {
                        let mut __c: i32 = *str1.offset(j as isize) as i32;
                        __res = if __c < -(128 as i32) || __c > 255 as i32 {
                            __c
                        } else {
                            *(*crate::stdlib::__ctype_toupper_loc()).offset(__c as isize)
                        }
                    } else {
                        __res = toupper(*str1.offset(j as isize) as i32)
                    }
                } else {
                    __res = *(*crate::stdlib::__ctype_toupper_loc())
                        .offset(*str1.offset(j as isize) as i32 as isize)
                }
                __res
            }) != ({
                let mut __res: i32 = 0;
                if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                    > 1 as i32 as libc::c_ulong
                {
                    if 0 != 0 {
                        let mut __c: i32 = *str2.offset(j as isize) as i32;
                        __res = if __c < -(128 as i32) || __c > 255 as i32 {
                            __c
                        } else {
                            *(*crate::stdlib::__ctype_toupper_loc()).offset(__c as isize)
                        }
                    } else {
                        __res = toupper(*str2.offset(j as isize) as i32)
                    }
                } else {
                    __res = *(*crate::stdlib::__ctype_toupper_loc())
                        .offset(*str2.offset(j as isize) as i32 as isize)
                }
                __res
            }) {
                break;
            }
            j += 1
            //end else
        }
        //if there was a word match
        if *str2.offset(j as isize) == 0 {
            //if the first string has an end of word
            if *str1.offset(j as isize) == 0
                || *str1.offset(j as isize) as i32 == ' ' as i32
                || *str1.offset(j as isize) as i32 == '.' as i32
                || *str1.offset(j as isize) as i32 == ',' as i32
                || *str1.offset(j as isize) as i32 == '!' as i32
            {
                return str1;
            }
        }
        i += 1;
        str1 = str1.offset(1)
        //end if
    }
    return 0 as *mut libc::c_char;
}
//end of the function StringContainsWord
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn StringReplaceWords(
    mut string: *mut libc::c_char,
    mut synonym: *mut libc::c_char,
    mut replacement: *mut libc::c_char,
) {
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut str2: *mut libc::c_char = 0 as *mut libc::c_char;
    //find the synonym in the string
    str = StringContainsWord(
        string,
        synonym,
        crate::src::qcommon::q_shared::qfalse as i32,
    );
    //if the synonym occurred in the string
    while !str.is_null() {
        //if the synonym isn't part of the replacement which is already in the string
        //useful for abbreviations
        str2 = StringContainsWord(
            string,
            replacement,
            crate::src::qcommon::q_shared::qfalse as i32,
        ); //end while
        while !str2.is_null() {
            if str2 <= str && str < str2.offset(crate::stdlib::strlen(replacement) as isize) {
                break; //end if
            }
            str2 = StringContainsWord(
                str2.offset(1 as i32 as isize),
                replacement,
                crate::src::qcommon::q_shared::qfalse as i32,
            )
        }
        if str2.is_null() {
            crate::stdlib::memmove(
                str.offset(crate::stdlib::strlen(replacement) as isize) as *mut libc::c_void,
                str.offset(crate::stdlib::strlen(synonym) as isize) as *const libc::c_void,
                crate::stdlib::strlen(str.offset(crate::stdlib::strlen(synonym) as isize))
                    .wrapping_add(1 as i32 as libc::c_ulong),
            );
            //append the synonum replacement
            crate::stdlib::memcpy(
                str as *mut libc::c_void,
                replacement as *const libc::c_void,
                crate::stdlib::strlen(replacement),
            );
        }
        //find the next synonym in the string
        str = StringContainsWord(
            str.offset(crate::stdlib::strlen(replacement) as isize),
            synonym,
            crate::src::qcommon::q_shared::qfalse as i32,
        )
    }
    //end if
}
//end of the function StringReplaceWords
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotDumpSynonymList(mut synlist: *mut bot_synonymlist_t) {
    let mut fp: *mut crate::stdlib::FILE = 0 as *mut crate::stdlib::FILE; //end for
    let mut syn: *mut bot_synonymlist_t = 0 as *mut bot_synonymlist_t;
    let mut synonym: *mut bot_synonym_t = 0 as *mut bot_synonym_t;
    fp = crate::src::botlib::l_log::Log_FilePointer() as *mut crate::stdlib::_IO_FILE;
    if fp.is_null() {
        return;
    }
    syn = synlist;
    while !syn.is_null() {
        crate::stdlib::fprintf(
            fp,
            b"%ld : [\x00" as *const u8 as *const libc::c_char,
            (*syn).context,
        );
        synonym = (*syn).firstsynonym;
        while !synonym.is_null() {
            crate::stdlib::fprintf(
                fp,
                b"(\"%s\", %1.2f)\x00" as *const u8 as *const libc::c_char,
                (*synonym).string,
                (*synonym).weight as f64,
            );
            if !(*synonym).next.is_null() {
                crate::stdlib::fprintf(fp, b", \x00" as *const u8 as *const libc::c_char);
            }
            synonym = (*synonym).next
        }
        crate::stdlib::fprintf(fp, b"]\n\x00" as *const u8 as *const libc::c_char);
        syn = (*syn).next
    }
    //end for
}
//end of the function BotDumpSynonymList
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotLoadSynonyms(
    mut filename: *mut libc::c_char,
) -> *mut bot_synonymlist_t {
    let mut pass: i32 = 0; //make compiler happy
    let mut size: i32 = 0; //make compiler happy
    let mut contextlevel: i32 = 0; //make compiler happy
    let mut numsynonyms: i32 = 0;
    let mut context: libc::c_ulong = 0;
    let mut contextstack: [libc::c_ulong; 32] = [0; 32];
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut source: *mut crate::src::botlib::l_precomp::source_t =
        0 as *mut crate::src::botlib::l_precomp::source_t;
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
    let mut synlist: *mut bot_synonymlist_t = 0 as *mut bot_synonymlist_t;
    let mut lastsyn: *mut bot_synonymlist_t = 0 as *mut bot_synonymlist_t;
    let mut syn: *mut bot_synonymlist_t = 0 as *mut bot_synonymlist_t;
    let mut synonym: *mut bot_synonym_t = 0 as *mut bot_synonym_t;
    let mut lastsynonym: *mut bot_synonym_t = 0 as *mut bot_synonym_t;
    size = 0 as i32;
    synlist = 0 as *mut bot_synonymlist_t;
    syn = 0 as *mut bot_synonymlist_t;
    synonym = 0 as *mut bot_synonym_t;
    //the synonyms are parsed in two phases
    pass = 0 as i32; //end for
    while pass < 2 as i32 {
        //
        if pass != 0 && size != 0 {
            ptr = crate::src::botlib::l_memory::GetClearedHunkMemory(size as libc::c_ulong)
                as *mut libc::c_char
        }
        //end if
        crate::src::botlib::l_precomp::PC_SetBaseFolder(
            b"botfiles\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        source = crate::src::botlib::l_precomp::LoadSourceFile(filename)
            as *mut crate::src::botlib::l_precomp::source_s;
        if source.is_null() {
            crate::src::botlib::be_interface::botimport
                .Print
                .expect("non-null function pointer")(
                3 as i32,
                b"counldn\'t load %s\n\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                filename,
            );
            return 0 as *mut bot_synonymlist_t;
        }
        context = 0 as i32 as libc::c_ulong;
        contextlevel = 0 as i32;
        synlist = 0 as *mut bot_synonymlist_t;
        lastsyn = 0 as *mut bot_synonymlist_t;
        while crate::src::botlib::l_precomp::PC_ReadToken(
            source as *mut crate::src::botlib::l_precomp::source_s,
            &mut token as *mut _ as *mut crate::src::botlib::l_script::token_s,
        ) != 0
        //
        //end if
        //
        //list synonyms
        //last synonym in the list
        //
        {
            if token.type_0 == 3 as i32 {
                context |= token.intvalue;
                contextstack[contextlevel as usize] = token.intvalue;
                contextlevel += 1; //end while
                                   //end if
                if contextlevel >= 32 as i32 {
                    crate::src::botlib::l_precomp::SourceError(
                        source as *mut crate::src::botlib::l_precomp::source_s,
                        b"more than 32 context levels\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    ); //end if
                    crate::src::botlib::l_precomp::FreeSource(
                        source as *mut crate::src::botlib::l_precomp::source_s,
                    ); //end if
                    return 0 as *mut bot_synonymlist_t;
                } //end if
                if crate::src::botlib::l_precomp::PC_ExpectTokenString(
                    source as *mut crate::src::botlib::l_precomp::source_s,
                    b"{\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ) == 0
                {
                    crate::src::botlib::l_precomp::FreeSource(
                        source as *mut crate::src::botlib::l_precomp::source_s,
                    );
                    return 0 as *mut bot_synonymlist_t;
                }
            } else if token.type_0 == 5 as i32 {
                if ::libc::strcmp(
                    token.string.as_mut_ptr(),
                    b"}\x00" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    contextlevel -= 1;
                    if contextlevel < 0 as i32 {
                        crate::src::botlib::l_precomp::SourceError(
                            source as *mut crate::src::botlib::l_precomp::source_s,
                            b"too many }\x00" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                        );
                        crate::src::botlib::l_precomp::FreeSource(
                            source as *mut crate::src::botlib::l_precomp::source_s,
                        );
                        return 0 as *mut bot_synonymlist_t;
                    }
                    context &= !contextstack[contextlevel as usize]
                } else if ::libc::strcmp(
                    token.string.as_mut_ptr(),
                    b"[\x00" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    size = (size as libc::c_ulong)
                        .wrapping_add(::std::mem::size_of::<bot_synonymlist_t>() as libc::c_ulong)
                        as i32 as i32;
                    //end if
                    if pass != 0 && !ptr.is_null() {
                        syn = ptr as *mut bot_synonymlist_t; //end if
                        ptr = ptr
                            .offset(::std::mem::size_of::<bot_synonymlist_t>() as libc::c_ulong
                                as isize); //end while
                        (*syn).context = context; //end if
                        (*syn).firstsynonym = 0 as *mut bot_synonym_t; //end if
                        (*syn).next = 0 as *mut bot_synonymlist_s; //end if
                        if !lastsyn.is_null() {
                            (*lastsyn).next = syn
                        } else {
                            synlist = syn
                        }
                        lastsyn = syn
                    }
                    numsynonyms = 0 as i32;
                    lastsynonym = 0 as *mut bot_synonym_t;
                    loop {
                        let mut len: crate::stddef_h::size_t = 0;
                        if crate::src::botlib::l_precomp::PC_ExpectTokenString(
                            source as *mut crate::src::botlib::l_precomp::source_s,
                            b"(\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        ) == 0
                            || crate::src::botlib::l_precomp::PC_ExpectTokenType(
                                source as *mut crate::src::botlib::l_precomp::source_s,
                                1 as i32,
                                0 as i32,
                                &mut token as *mut _ as *mut crate::src::botlib::l_script::token_s,
                            ) == 0
                        {
                            crate::src::botlib::l_precomp::FreeSource(
                                source as *mut crate::src::botlib::l_precomp::source_s,
                            );
                            return 0 as *mut bot_synonymlist_t;
                        }
                        crate::src::botlib::l_script::StripDoubleQuotes(token.string.as_mut_ptr());
                        if crate::stdlib::strlen(token.string.as_mut_ptr())
                            <= 0 as i32 as libc::c_ulong
                        {
                            crate::src::botlib::l_precomp::SourceError(
                                source as *mut crate::src::botlib::l_precomp::source_s,
                                b"empty string\x00" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                            );
                            crate::src::botlib::l_precomp::FreeSource(
                                source as *mut crate::src::botlib::l_precomp::source_s,
                            );
                            return 0 as *mut bot_synonymlist_t;
                        }
                        len = crate::stdlib::strlen(token.string.as_mut_ptr())
                            .wrapping_add(1 as i32 as libc::c_ulong);
                        len = len
                            .wrapping_add(::std::mem::size_of::<isize>() as libc::c_ulong)
                            .wrapping_sub(1 as i32 as libc::c_ulong)
                            & !(::std::mem::size_of::<isize>() as libc::c_ulong)
                                .wrapping_sub(1 as i32 as libc::c_ulong);
                        size = (size as libc::c_ulong).wrapping_add(
                            (::std::mem::size_of::<bot_synonym_t>() as libc::c_ulong)
                                .wrapping_add(len),
                        ) as i32 as i32;
                        if pass != 0 && !ptr.is_null() {
                            synonym = ptr as *mut bot_synonym_t;
                            ptr = ptr
                                .offset(::std::mem::size_of::<bot_synonym_t>() as libc::c_ulong
                                    as isize);
                            (*synonym).string = ptr;
                            ptr = ptr.offset(len as isize);
                            ::libc::strcpy((*synonym).string, token.string.as_mut_ptr());
                            //
                            if !lastsynonym.is_null() {
                                (*lastsynonym).next = synonym
                            } else {
                                (*syn).firstsynonym = synonym
                            } //end if
                            lastsynonym = synonym
                        } //end if
                        numsynonyms += 1;
                        if crate::src::botlib::l_precomp::PC_ExpectTokenString(
                            source as *mut crate::src::botlib::l_precomp::source_s,
                            b",\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        ) == 0
                            || crate::src::botlib::l_precomp::PC_ExpectTokenType(
                                source as *mut crate::src::botlib::l_precomp::source_s,
                                3 as i32,
                                0 as i32,
                                &mut token as *mut _ as *mut crate::src::botlib::l_script::token_s,
                            ) == 0
                            || crate::src::botlib::l_precomp::PC_ExpectTokenString(
                                source as *mut crate::src::botlib::l_precomp::source_s,
                                b")\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                            ) == 0
                        {
                            crate::src::botlib::l_precomp::FreeSource(
                                source as *mut crate::src::botlib::l_precomp::source_s,
                            );
                            return 0 as *mut bot_synonymlist_t;
                        }
                        if pass != 0 && !ptr.is_null() {
                            (*synonym).weight = token.floatvalue;
                            (*syn).totalweight += (*synonym).weight
                        }
                        if crate::src::botlib::l_precomp::PC_CheckTokenString(
                            source as *mut crate::src::botlib::l_precomp::source_s,
                            b"]\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        ) != 0
                        {
                            break;
                        }
                        if crate::src::botlib::l_precomp::PC_ExpectTokenString(
                            source as *mut crate::src::botlib::l_precomp::source_s,
                            b",\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        ) == 0
                        {
                            crate::src::botlib::l_precomp::FreeSource(
                                source as *mut crate::src::botlib::l_precomp::source_s,
                            );
                            return 0 as *mut bot_synonymlist_t;
                        }
                    }
                    if numsynonyms < 2 as i32 {
                        crate::src::botlib::l_precomp::SourceError(
                            source as *mut crate::src::botlib::l_precomp::source_s,
                            b"synonym must have at least two entries\x00" as *const u8
                                as *const libc::c_char
                                as *mut libc::c_char,
                        );
                        crate::src::botlib::l_precomp::FreeSource(
                            source as *mut crate::src::botlib::l_precomp::source_s,
                        );
                        return 0 as *mut bot_synonymlist_t;
                    }
                } else {
                    crate::src::botlib::l_precomp::SourceError(
                        source as *mut crate::src::botlib::l_precomp::source_s,
                        b"unexpected %s\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        token.string.as_mut_ptr(),
                    );
                    crate::src::botlib::l_precomp::FreeSource(
                        source as *mut crate::src::botlib::l_precomp::source_s,
                    );
                    return 0 as *mut bot_synonymlist_t;
                }
                //end if
            }
            //end else if
        }
        crate::src::botlib::l_precomp::FreeSource(
            source as *mut crate::src::botlib::l_precomp::source_s,
        );
        if contextlevel > 0 as i32 {
            crate::src::botlib::l_precomp::SourceError(
                source as *mut crate::src::botlib::l_precomp::source_s,
                b"missing }\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            return 0 as *mut bot_synonymlist_t;
        }
        pass += 1
    }
    crate::src::botlib::be_interface::botimport
        .Print
        .expect("non-null function pointer")(
        1 as i32,
        b"loaded %s\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        filename,
    );
    //
    //
    //
    //BotDumpSynonymList(synlist);
    //
    return synlist;
}
//replace all the context related synonyms in the string
//end of the function BotLoadSynonyms
//===========================================================================
// replace all the synonyms in the string
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotReplaceSynonyms(
    mut string: *mut libc::c_char,
    mut context: libc::c_ulong,
) {
    let mut syn: *mut bot_synonymlist_t = 0 as *mut bot_synonymlist_t;
    let mut synonym: *mut bot_synonym_t = 0 as *mut bot_synonym_t;
    syn = synonyms;
    while !syn.is_null() {
        if !((*syn).context & context == 0) {
            synonym = (*(*syn).firstsynonym).next;
            while !synonym.is_null() {
                StringReplaceWords(string, (*synonym).string, (*(*syn).firstsynonym).string);
                synonym = (*synonym).next
            }
        }
        syn = (*syn).next
        //end for
    }
    //end for
}
//end of the function BotReplaceSynonyms
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotReplaceWeightedSynonyms(
    mut string: *mut libc::c_char,
    mut context: libc::c_ulong,
) {
    let mut syn: *mut bot_synonymlist_t = 0 as *mut bot_synonymlist_t;
    let mut synonym: *mut bot_synonym_t = 0 as *mut bot_synonym_t;
    let mut replacement: *mut bot_synonym_t = 0 as *mut bot_synonym_t;
    let mut weight: f32 = 0.;
    let mut curweight: f32 = 0.;
    syn = synonyms;
    while !syn.is_null() {
        if !((*syn).context & context == 0) {
            //choose a weighted random replacement synonym
            weight =
                (::libc::rand() & 0x7fff as i32) as f32 / 0x7fff as i32 as f32 * (*syn).totalweight; //end for
            if !(weight == 0.) {
                curweight = 0 as i32 as f32;
                replacement = (*syn).firstsynonym;
                while !replacement.is_null() {
                    curweight += (*replacement).weight;
                    if weight < curweight {
                        break;
                    }
                    replacement = (*replacement).next
                }
                if !replacement.is_null() {
                    //replace all synonyms with the replacement
                    synonym = (*syn).firstsynonym;
                    while !synonym.is_null() {
                        if !(synonym == replacement) {
                            StringReplaceWords(string, (*synonym).string, (*replacement).string);
                        }
                        synonym = (*synonym).next
                    }
                }
            }
        }
        syn = (*syn).next
        //end for
    }
    //end for
}
//end of the function BotReplaceWeightedSynonyms
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotReplaceReplySynonyms(
    mut string: *mut libc::c_char,
    mut context: libc::c_ulong,
) {
    let mut str1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut str2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut replacement: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut syn: *mut bot_synonymlist_t = 0 as *mut bot_synonymlist_t;
    let mut synonym: *mut bot_synonym_t = 0 as *mut bot_synonym_t;
    str1 = string;
    while *str1 != 0 {
        //go to the start of the next word
        while *str1 as i32 != 0 && *str1 as i32 <= ' ' as i32 {
            str1 = str1.offset(1)
        }
        if *str1 == 0 {
            break;
        }
        //
        syn = synonyms; //end for
        while !syn.is_null() {
            if !((*syn).context & context == 0) {
                synonym = (*(*syn).firstsynonym).next; //end for
                while !synonym.is_null() {
                    //if the synonym is not at the front of the string continue
                    str2 = StringContainsWord(
                        str1,
                        (*synonym).string,
                        crate::src::qcommon::q_shared::qfalse as i32,
                    );
                    if !(str2.is_null() || str2 != str1) {
                        //
                        replacement = (*(*syn).firstsynonym).string;
                        //if the replacement IS in front of the string continue
                        str2 = StringContainsWord(
                            str1,
                            replacement,
                            crate::src::qcommon::q_shared::qfalse as i32,
                        );
                        if !(!str2.is_null() && str2 == str1) {
                            //
                            crate::stdlib::memmove(
                                str1.offset(crate::stdlib::strlen(replacement) as isize)
                                    as *mut libc::c_void,
                                str1.offset(crate::stdlib::strlen((*synonym).string) as isize)
                                    as *const libc::c_void,
                                crate::stdlib::strlen(
                                    str1.offset(crate::stdlib::strlen((*synonym).string) as isize),
                                )
                                .wrapping_add(1 as i32 as libc::c_ulong),
                            );
                            //append the synonum replacement
                            crate::stdlib::memcpy(
                                str1 as *mut libc::c_void,
                                replacement as *const libc::c_void,
                                crate::stdlib::strlen(replacement),
                            );
                            break;
                        }
                    }
                    synonym = (*synonym).next
                }
                //if a synonym has been replaced
                if !synonym.is_null() {
                    break;
                }
            }
            syn = (*syn).next
        }
        //skip over this word
        while *str1 as i32 != 0 && *str1 as i32 > ' ' as i32 {
            str1 = str1.offset(1)
        }
        if *str1 == 0 {
            break;
        }
    }
    //end while
}
//end of the function BotReplaceReplySynonyms
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotLoadChatMessage(
    mut source: *mut crate::src::botlib::l_precomp::source_t,
    mut chatmessagestring: *mut libc::c_char,
) -> i32 {
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
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
    ptr = chatmessagestring;
    *ptr = 0 as i32 as libc::c_char;
    loop
    //
    {
        if crate::src::botlib::l_precomp::PC_ExpectAnyToken(
            source as *mut crate::src::botlib::l_precomp::source_s,
            &mut token as *mut _ as *mut crate::src::botlib::l_script::token_s,
        ) == 0
        {
            return crate::src::qcommon::q_shared::qfalse as i32;
        } //end while
          //fixed string
        if token.type_0 == 1 as i32 {
            //end else
            crate::src::botlib::l_script::StripDoubleQuotes(token.string.as_mut_ptr()); //end else if
            if crate::stdlib::strlen(ptr)
                .wrapping_add(crate::stdlib::strlen(token.string.as_mut_ptr()))
                .wrapping_add(1 as i32 as libc::c_ulong)
                > 256 as i32 as libc::c_ulong
            {
                crate::src::botlib::l_precomp::SourceError(
                    source as *mut crate::src::botlib::l_precomp::source_s,
                    b"chat message too long\x00" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                ); //end if
                return crate::src::qcommon::q_shared::qfalse as i32;
            }
            ::libc::strcat(ptr, token.string.as_mut_ptr());
        } else if token.type_0 == 3 as i32 && token.subtype & 0x1000 as i32 != 0 {
            //variable string
            if crate::stdlib::strlen(ptr).wrapping_add(7 as i32 as libc::c_ulong)
                > 256 as i32 as libc::c_ulong
            {
                crate::src::botlib::l_precomp::SourceError(
                    source as *mut crate::src::botlib::l_precomp::source_s,
                    b"chat message too long\x00" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                ); //end if
                return crate::src::qcommon::q_shared::qfalse as i32;
            } //end if
            ::libc::sprintf(
                &mut *ptr.offset((crate::stdlib::strlen
                    as unsafe extern "C" fn(_: *const libc::c_char) -> libc::c_ulong)(
                    ptr
                ) as isize) as *mut libc::c_char,
                b"%cv%ld%c\x00" as *const u8 as *const libc::c_char,
                0x1 as i32,
                token.intvalue,
                0x1 as i32,
            );
        } else if token.type_0 == 4 as i32 {
            //random string
            if crate::stdlib::strlen(ptr).wrapping_add(7 as i32 as libc::c_ulong)
                > 256 as i32 as libc::c_ulong
            {
                crate::src::botlib::l_precomp::SourceError(
                    source as *mut crate::src::botlib::l_precomp::source_s,
                    b"chat message too long\x00" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                ); //end else if
                return crate::src::qcommon::q_shared::qfalse as i32;
            } //end if
            ::libc::sprintf(
                &mut *ptr.offset((crate::stdlib::strlen
                    as unsafe extern "C" fn(_: *const libc::c_char) -> libc::c_ulong)(
                    ptr
                ) as isize) as *mut libc::c_char,
                b"%cr%s%c\x00" as *const u8 as *const libc::c_char,
                0x1 as i32,
                token.string.as_mut_ptr(),
                0x1 as i32,
            );
        } else {
            crate::src::botlib::l_precomp::SourceError(
                source as *mut crate::src::botlib::l_precomp::source_s,
                b"unknown message component %s\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                token.string.as_mut_ptr(),
            );
            return crate::src::qcommon::q_shared::qfalse as i32;
        }
        if crate::src::botlib::l_precomp::PC_CheckTokenString(
            source as *mut crate::src::botlib::l_precomp::source_s,
            b";\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) != 0
        {
            break;
        }
        if crate::src::botlib::l_precomp::PC_ExpectTokenString(
            source as *mut crate::src::botlib::l_precomp::source_s,
            b",\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) == 0
        {
            return crate::src::qcommon::q_shared::qfalse as i32;
        }
    }
    //
    return crate::src::qcommon::q_shared::qtrue as i32;
}
//end of the function BotLoadChatMessage
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotDumpRandomStringList(mut randomlist: *mut bot_randomlist_t) {
    let mut fp: *mut crate::stdlib::FILE = 0 as *mut crate::stdlib::FILE;
    let mut random: *mut bot_randomlist_t = 0 as *mut bot_randomlist_t;
    let mut rs: *mut bot_randomstring_t = 0 as *mut bot_randomstring_t;
    fp = crate::src::botlib::l_log::Log_FilePointer() as *mut crate::stdlib::_IO_FILE;
    if fp.is_null() {
        return;
    }
    random = randomlist;
    while !random.is_null() {
        crate::stdlib::fprintf(
            fp,
            b"%s = {\x00" as *const u8 as *const libc::c_char,
            (*random).string,
        );
        rs = (*random).firstrandomstring;
        while !rs.is_null() {
            crate::stdlib::fprintf(
                fp,
                b"\"%s\"\x00" as *const u8 as *const libc::c_char,
                (*rs).string,
            );
            if !(*rs).next.is_null() {
                crate::stdlib::fprintf(fp, b", \x00" as *const u8 as *const libc::c_char);
            } else {
                crate::stdlib::fprintf(fp, b"}\n\x00" as *const u8 as *const libc::c_char);
            }
            rs = (*rs).next
        }
        random = (*random).next
        //end for
    }
    //end for
}
//end of the function BotDumpRandomStringList
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotLoadRandomStrings(
    mut filename: *mut libc::c_char,
) -> *mut bot_randomlist_t {
    let mut pass: i32 = 0;
    let mut size: i32 = 0;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut chatmessagestring: [libc::c_char; 256] = [0; 256];
    let mut source: *mut crate::src::botlib::l_precomp::source_t =
        0 as *mut crate::src::botlib::l_precomp::source_t;
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
    let mut randomlist: *mut bot_randomlist_t = 0 as *mut bot_randomlist_t;
    let mut lastrandom: *mut bot_randomlist_t = 0 as *mut bot_randomlist_t;
    let mut random: *mut bot_randomlist_t = 0 as *mut bot_randomlist_t;
    let mut randomstring: *mut bot_randomstring_t = 0 as *mut bot_randomstring_t;
    //DEBUG
    size = 0 as i32;
    randomlist = 0 as *mut bot_randomlist_t;
    random = 0 as *mut bot_randomlist_t;
    //the synonyms are parsed in two phases
    pass = 0 as i32; //end for
    while pass < 2 as i32 {
        //
        if pass != 0 && size != 0 {
            ptr = crate::src::botlib::l_memory::GetClearedHunkMemory(size as libc::c_ulong)
                as *mut libc::c_char
        }
        //
        crate::src::botlib::l_precomp::PC_SetBaseFolder(
            b"botfiles\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ); //end if
        source = crate::src::botlib::l_precomp::LoadSourceFile(filename)
            as *mut crate::src::botlib::l_precomp::source_s;
        if source.is_null() {
            crate::src::botlib::be_interface::botimport
                .Print
                .expect("non-null function pointer")(
                3 as i32,
                b"counldn\'t load %s\n\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                filename,
            );
            return 0 as *mut bot_randomlist_t;
        }
        //
        randomlist = 0 as *mut bot_randomlist_t; //list
        lastrandom = 0 as *mut bot_randomlist_t; //last
                                                 //
        while crate::src::botlib::l_precomp::PC_ReadToken(
            source as *mut crate::src::botlib::l_precomp::source_s,
            &mut token as *mut _ as *mut crate::src::botlib::l_script::token_s,
        ) != 0
        {
            let mut len: crate::stddef_h::size_t = 0; //end while
                                                      //end while
            if token.type_0 != 4 as i32 {
                crate::src::botlib::l_precomp::SourceError(
                    source as *mut crate::src::botlib::l_precomp::source_s,
                    b"unknown random %s\x00" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    token.string.as_mut_ptr(),
                ); //end if
                crate::src::botlib::l_precomp::FreeSource(
                    source as *mut crate::src::botlib::l_precomp::source_s,
                ); //end if
                return 0 as *mut bot_randomlist_t;
            }
            len = crate::stdlib::strlen(token.string.as_mut_ptr())
                .wrapping_add(1 as i32 as libc::c_ulong);
            len = len
                .wrapping_add(::std::mem::size_of::<isize>() as libc::c_ulong)
                .wrapping_sub(1 as i32 as libc::c_ulong)
                & !(::std::mem::size_of::<isize>() as libc::c_ulong)
                    .wrapping_sub(1 as i32 as libc::c_ulong);
            size = (size as libc::c_ulong).wrapping_add(
                (::std::mem::size_of::<bot_randomlist_t>() as libc::c_ulong).wrapping_add(len),
            ) as i32 as i32;
            if pass != 0 && !ptr.is_null() {
                random = ptr as *mut bot_randomlist_t;
                ptr =
                    ptr.offset(::std::mem::size_of::<bot_randomlist_t>() as libc::c_ulong as isize);
                (*random).string = ptr;
                ptr = ptr.offset(len as isize);
                ::libc::strcpy((*random).string, token.string.as_mut_ptr());
                (*random).firstrandomstring = 0 as *mut bot_randomstring_t;
                (*random).numstrings = 0 as i32;
                //
                if !lastrandom.is_null() {
                    (*lastrandom).next = random
                } else {
                    randomlist = random
                } //end if
                lastrandom = random
            } //end if
            if crate::src::botlib::l_precomp::PC_ExpectTokenString(
                source as *mut crate::src::botlib::l_precomp::source_s,
                b"=\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) == 0
                || crate::src::botlib::l_precomp::PC_ExpectTokenString(
                    source as *mut crate::src::botlib::l_precomp::source_s,
                    b"{\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ) == 0
            {
                crate::src::botlib::l_precomp::FreeSource(
                    source as *mut crate::src::botlib::l_precomp::source_s,
                );
                return 0 as *mut bot_randomlist_t;
            }
            while crate::src::botlib::l_precomp::PC_CheckTokenString(
                source as *mut crate::src::botlib::l_precomp::source_s,
                b"}\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) == 0
            {
                if BotLoadChatMessage(source, chatmessagestring.as_mut_ptr()) == 0 {
                    crate::src::botlib::l_precomp::FreeSource(
                        source as *mut crate::src::botlib::l_precomp::source_s,
                    );
                    return 0 as *mut bot_randomlist_t;
                }
                len = crate::stdlib::strlen(chatmessagestring.as_mut_ptr())
                    .wrapping_add(1 as i32 as libc::c_ulong);
                len = len
                    .wrapping_add(::std::mem::size_of::<isize>() as libc::c_ulong)
                    .wrapping_sub(1 as i32 as libc::c_ulong)
                    & !(::std::mem::size_of::<isize>() as libc::c_ulong)
                        .wrapping_sub(1 as i32 as libc::c_ulong);
                size = (size as libc::c_ulong).wrapping_add(
                    (::std::mem::size_of::<bot_randomstring_t>() as libc::c_ulong)
                        .wrapping_add(len),
                ) as i32 as i32;
                if pass != 0 && !ptr.is_null() {
                    randomstring = ptr as *mut bot_randomstring_t;
                    ptr = ptr.offset(
                        ::std::mem::size_of::<bot_randomstring_t>() as libc::c_ulong as isize
                    );
                    (*randomstring).string = ptr;
                    ptr = ptr.offset(len as isize);
                    ::libc::strcpy((*randomstring).string, chatmessagestring.as_mut_ptr());
                    //end if
                    //
                    (*random).numstrings += 1;
                    (*randomstring).next = (*random).firstrandomstring;
                    (*random).firstrandomstring = randomstring
                }
            }
        }
        //free the source after one pass
        crate::src::botlib::l_precomp::FreeSource(
            source as *mut crate::src::botlib::l_precomp::source_s,
        );
        pass += 1
    }
    crate::src::botlib::be_interface::botimport
        .Print
        .expect("non-null function pointer")(
        1 as i32,
        b"loaded %s\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        filename,
    );
    //
    //DEBUG
    //
    return randomlist;
}
//end of the function BotLoadRandomStrings
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn RandomString(mut name: *mut libc::c_char) -> *mut libc::c_char {
    let mut random: *mut bot_randomlist_t = 0 as *mut bot_randomlist_t; //end for
    let mut rs: *mut bot_randomstring_t = 0 as *mut bot_randomstring_t;
    let mut i: i32 = 0;
    random = randomstrings;
    while !random.is_null() {
        if ::libc::strcmp((*random).string, name) == 0 {
            i = ((::libc::rand() & 0x7fff as i32) as f32 / 0x7fff as i32 as f32
                * (*random).numstrings as f32) as i32;
            //end if
            rs = (*random).firstrandomstring; //end for
            while !rs.is_null() {
                i -= 1;
                if i < 0 as i32 {
                    break;
                }
                rs = (*rs).next
            }
            if !rs.is_null() {
                return (*rs).string;
            }
        }
        random = (*random).next
        //end for
    }
    return 0 as *mut libc::c_char;
}
//end of the function RandomString
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotDumpMatchTemplates(mut matches: *mut bot_matchtemplate_t) {
    let mut fp: *mut crate::stdlib::FILE = 0 as *mut crate::stdlib::FILE; //end for
    let mut mt: *mut bot_matchtemplate_t = 0 as *mut bot_matchtemplate_t; //end else if
    let mut mp: *mut bot_matchpiece_t = 0 as *mut bot_matchpiece_t;
    let mut ms: *mut bot_matchstring_t = 0 as *mut bot_matchstring_t;
    fp = crate::src::botlib::l_log::Log_FilePointer() as *mut crate::stdlib::_IO_FILE;
    if fp.is_null() {
        return;
    }
    mt = matches;
    while !mt.is_null() {
        crate::stdlib::fprintf(fp, b"{ \x00" as *const u8 as *const libc::c_char);
        mp = (*mt).first;
        while !mp.is_null() {
            if (*mp).type_0 == 2 as i32 {
                ms = (*mp).firststring;
                while !ms.is_null() {
                    crate::stdlib::fprintf(
                        fp,
                        b"\"%s\"\x00" as *const u8 as *const libc::c_char,
                        (*ms).string,
                    );
                    if !(*ms).next.is_null() {
                        crate::stdlib::fprintf(fp, b"|\x00" as *const u8 as *const libc::c_char);
                    }
                    ms = (*ms).next
                }
            //end for
            } else if (*mp).type_0 == 1 as i32 {
                crate::stdlib::fprintf(
                    fp,
                    b"%d\x00" as *const u8 as *const libc::c_char,
                    (*mp).variable,
                );
            }
            if !(*mp).next.is_null() {
                crate::stdlib::fprintf(fp, b", \x00" as *const u8 as *const libc::c_char);
            }
            mp = (*mp).next
        }
        crate::stdlib::fprintf(
            fp,
            b" = (%d, %d);}\n\x00" as *const u8 as *const libc::c_char,
            (*mt).type_0,
            (*mt).subtype,
        );
        mt = (*mt).next
    }
    //end for
}
//end of the function BotDumpMatchTemplates
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotFreeMatchPieces(mut matchpieces: *mut bot_matchpiece_t) {
    let mut mp: *mut bot_matchpiece_t = 0 as *mut bot_matchpiece_t; //end if
    let mut nextmp: *mut bot_matchpiece_t = 0 as *mut bot_matchpiece_t;
    let mut ms: *mut bot_matchstring_t = 0 as *mut bot_matchstring_t;
    let mut nextms: *mut bot_matchstring_t = 0 as *mut bot_matchstring_t;
    mp = matchpieces;
    while !mp.is_null() {
        nextmp = (*mp).next;
        if (*mp).type_0 == 2 as i32 {
            ms = (*mp).firststring;
            while !ms.is_null() {
                nextms = (*ms).next;
                crate::src::botlib::l_memory::FreeMemory(ms as *mut libc::c_void);
                ms = nextms
            }
            //end for
        }
        crate::src::botlib::l_memory::FreeMemory(mp as *mut libc::c_void);
        mp = nextmp
    }
    //end for
}
//end of the function BotFreeMatchPieces
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotLoadMatchPieces(
    mut source: *mut crate::src::botlib::l_precomp::source_t,
    mut endtoken: *mut libc::c_char,
) -> *mut bot_matchpiece_t {
    let mut lastwasvariable: i32 = 0;
    let mut emptystring: i32 = 0;
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
    let mut matchpiece: *mut bot_matchpiece_t = 0 as *mut bot_matchpiece_t;
    let mut firstpiece: *mut bot_matchpiece_t = 0 as *mut bot_matchpiece_t;
    let mut lastpiece: *mut bot_matchpiece_t = 0 as *mut bot_matchpiece_t;
    let mut matchstring: *mut bot_matchstring_t = 0 as *mut bot_matchstring_t;
    let mut lastmatchstring: *mut bot_matchstring_t = 0 as *mut bot_matchstring_t;
    firstpiece = 0 as *mut bot_matchpiece_t;
    lastpiece = 0 as *mut bot_matchpiece_t;
    //
    lastwasvariable = crate::src::qcommon::q_shared::qfalse as i32;
    //
    while crate::src::botlib::l_precomp::PC_ReadToken(
        source as *mut crate::src::botlib::l_precomp::source_s,
        &mut token as *mut _ as *mut crate::src::botlib::l_script::token_s,
    ) != 0
    {
        //end while
        if token.type_0 == 3 as i32 && token.subtype & 0x1000 as i32 != 0 {
            //end else
            if token.intvalue >= 8 as i32 as libc::c_ulong {
                crate::src::botlib::l_precomp::SourceError(
                    source as *mut crate::src::botlib::l_precomp::source_s,
                    b"can\'t have more than %d match variables\x00" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    8 as i32,
                ); //end if
                crate::src::botlib::l_precomp::FreeSource(
                    source as *mut crate::src::botlib::l_precomp::source_s,
                ); //end if
                BotFreeMatchPieces(firstpiece); //end if
                return 0 as *mut bot_matchpiece_t;
            }
            if lastwasvariable != 0 {
                crate::src::botlib::l_precomp::SourceError(
                    source as *mut crate::src::botlib::l_precomp::source_s,
                    b"not allowed to have adjacent variables\x00" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                );
                crate::src::botlib::l_precomp::FreeSource(
                    source as *mut crate::src::botlib::l_precomp::source_s,
                );
                BotFreeMatchPieces(firstpiece);
                return 0 as *mut bot_matchpiece_t;
            }
            lastwasvariable = crate::src::qcommon::q_shared::qtrue as i32;
            //
            matchpiece = crate::src::botlib::l_memory::GetClearedHunkMemory(::std::mem::size_of::<
                bot_matchpiece_t,
            >()
                as libc::c_ulong) as *mut bot_matchpiece_t; //end if
            (*matchpiece).type_0 = 1 as i32;
            (*matchpiece).variable = token.intvalue as i32;
            (*matchpiece).next = 0 as *mut bot_matchpiece_s;
            if !lastpiece.is_null() {
                (*lastpiece).next = matchpiece
            } else {
                firstpiece = matchpiece
            }
            lastpiece = matchpiece
        } else if token.type_0 == 1 as i32 {
            //
            matchpiece = crate::src::botlib::l_memory::GetClearedHunkMemory(::std::mem::size_of::<
                bot_matchpiece_t,
            >()
                as libc::c_ulong) as *mut bot_matchpiece_t;
            (*matchpiece).firststring = 0 as *mut bot_matchstring_t;
            (*matchpiece).type_0 = 2 as i32;
            (*matchpiece).variable = 0 as i32;
            (*matchpiece).next = 0 as *mut bot_matchpiece_s;
            if !lastpiece.is_null() {
                (*lastpiece).next = matchpiece
            } else {
                firstpiece = matchpiece
            }
            lastpiece = matchpiece;
            //
            lastmatchstring = 0 as *mut bot_matchstring_t;
            emptystring = crate::src::qcommon::q_shared::qfalse as i32;
            loop
            //
            {
                if !(*matchpiece).firststring.is_null() {
                    if crate::src::botlib::l_precomp::PC_ExpectTokenType(
                        source as *mut crate::src::botlib::l_precomp::source_s,
                        1 as i32,
                        0 as i32,
                        &mut token as *mut _ as *mut crate::src::botlib::l_script::token_s,
                    ) == 0
                    {
                        crate::src::botlib::l_precomp::FreeSource(
                            source as *mut crate::src::botlib::l_precomp::source_s,
                        ); //end if
                        BotFreeMatchPieces(firstpiece);
                        return 0 as *mut bot_matchpiece_t;
                    }
                    //end if
                }
                crate::src::botlib::l_script::StripDoubleQuotes(token.string.as_mut_ptr());
                matchstring = crate::src::botlib::l_memory::GetClearedHunkMemory(
                    (::std::mem::size_of::<bot_matchstring_t>() as libc::c_ulong)
                        .wrapping_add(crate::stdlib::strlen(token.string.as_mut_ptr()))
                        .wrapping_add(1 as i32 as libc::c_ulong),
                ) as *mut bot_matchstring_t;
                (*matchstring).string = (matchstring as *mut libc::c_char)
                    .offset(::std::mem::size_of::<bot_matchstring_t>() as libc::c_ulong as isize);
                ::libc::strcpy((*matchstring).string, token.string.as_mut_ptr());
                if crate::stdlib::strlen(token.string.as_mut_ptr()) == 0 {
                    emptystring = crate::src::qcommon::q_shared::qtrue as i32
                }
                (*matchstring).next = 0 as *mut bot_matchstring_s;
                if !lastmatchstring.is_null() {
                    (*lastmatchstring).next = matchstring
                } else {
                    (*matchpiece).firststring = matchstring
                }
                lastmatchstring = matchstring;
                if !(crate::src::botlib::l_precomp::PC_CheckTokenString(
                    source as *mut crate::src::botlib::l_precomp::source_s,
                    b"|\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ) != 0)
                {
                    break;
                }
            }
            //if there was no empty string found
            if emptystring == 0 {
                lastwasvariable = crate::src::qcommon::q_shared::qfalse as i32
            }
        } else {
            crate::src::botlib::l_precomp::SourceError(
                source as *mut crate::src::botlib::l_precomp::source_s,
                b"invalid token %s\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                token.string.as_mut_ptr(),
            );
            crate::src::botlib::l_precomp::FreeSource(
                source as *mut crate::src::botlib::l_precomp::source_s,
            );
            BotFreeMatchPieces(firstpiece);
            return 0 as *mut bot_matchpiece_t;
        }
        if crate::src::botlib::l_precomp::PC_CheckTokenString(
            source as *mut crate::src::botlib::l_precomp::source_s,
            endtoken,
        ) != 0
        {
            break;
        }
        if crate::src::botlib::l_precomp::PC_ExpectTokenString(
            source as *mut crate::src::botlib::l_precomp::source_s,
            b",\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) == 0
        {
            crate::src::botlib::l_precomp::FreeSource(
                source as *mut crate::src::botlib::l_precomp::source_s,
            );
            BotFreeMatchPieces(firstpiece);
            return 0 as *mut bot_matchpiece_t;
        }
        //end if
    }
    return firstpiece;
}
//end of the function BotLoadMatchPieces
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotFreeMatchTemplates(mut mt: *mut bot_matchtemplate_t) {
    let mut nextmt: *mut bot_matchtemplate_t = 0 as *mut bot_matchtemplate_t;
    while !mt.is_null() {
        nextmt = (*mt).next;
        BotFreeMatchPieces((*mt).first);
        crate::src::botlib::l_memory::FreeMemory(mt as *mut libc::c_void);
        mt = nextmt
    }
    //end for
}
//end of the function BotFreeMatchTemplates
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotLoadMatchTemplates(
    mut matchfile: *mut libc::c_char,
) -> *mut bot_matchtemplate_t {
    let mut source: *mut crate::src::botlib::l_precomp::source_t =
        0 as *mut crate::src::botlib::l_precomp::source_t; //end if
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
    let mut matchtemplate: *mut bot_matchtemplate_t = 0 as *mut bot_matchtemplate_t;
    let mut matches: *mut bot_matchtemplate_t = 0 as *mut bot_matchtemplate_t;
    let mut lastmatch: *mut bot_matchtemplate_t = 0 as *mut bot_matchtemplate_t;
    let mut context: libc::c_ulong = 0;
    crate::src::botlib::l_precomp::PC_SetBaseFolder(
        b"botfiles\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    source = crate::src::botlib::l_precomp::LoadSourceFile(matchfile)
        as *mut crate::src::botlib::l_precomp::source_s;
    if source.is_null() {
        crate::src::botlib::be_interface::botimport
            .Print
            .expect("non-null function pointer")(
            3 as i32,
            b"counldn\'t load %s\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            matchfile,
        );
        return 0 as *mut bot_matchtemplate_t;
    }
    //
    matches = 0 as *mut bot_matchtemplate_t; //list with matches
    lastmatch = 0 as *mut bot_matchtemplate_t; //last match in the list
                                               //end while
    while crate::src::botlib::l_precomp::PC_ReadToken(
        source as *mut crate::src::botlib::l_precomp::source_s,
        &mut token as *mut _ as *mut crate::src::botlib::l_script::token_s,
    ) != 0
    {
        if token.type_0 != 3 as i32 || token.subtype & 0x1000 as i32 == 0 {
            crate::src::botlib::l_precomp::SourceError(
                source as *mut crate::src::botlib::l_precomp::source_s,
                b"expected integer, found %s\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                token.string.as_mut_ptr(),
            ); //end while
            BotFreeMatchTemplates(matches); //end if
            crate::src::botlib::l_precomp::FreeSource(
                source as *mut crate::src::botlib::l_precomp::source_s,
            );
            return 0 as *mut bot_matchtemplate_t;
        }
        context = token.intvalue;
        if crate::src::botlib::l_precomp::PC_ExpectTokenString(
            source as *mut crate::src::botlib::l_precomp::source_s,
            b"{\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) == 0
        {
            BotFreeMatchTemplates(matches);
            crate::src::botlib::l_precomp::FreeSource(
                source as *mut crate::src::botlib::l_precomp::source_s,
            );
            return 0 as *mut bot_matchtemplate_t;
        }
        while crate::src::botlib::l_precomp::PC_ReadToken(
            source as *mut crate::src::botlib::l_precomp::source_s,
            &mut token as *mut _ as *mut crate::src::botlib::l_script::token_s,
        ) != 0
        //the context
        //
        //end if
        //
        {
            if ::libc::strcmp(
                token.string.as_mut_ptr(),
                b"}\x00" as *const u8 as *const libc::c_char,
            ) == 0
            {
                break;
                //end if
            }
            //
            crate::src::botlib::l_precomp::PC_UnreadLastToken(
                source as *mut crate::src::botlib::l_precomp::source_s,
            );
            //
            matchtemplate =
                crate::src::botlib::l_memory::GetClearedHunkMemory(::std::mem::size_of::<
                    bot_matchtemplate_t,
                >()
                    as libc::c_ulong) as *mut bot_matchtemplate_t;
            (*matchtemplate).context = context;
            (*matchtemplate).next = 0 as *mut bot_matchtemplate_s;
            //add the match template to the list
            if !lastmatch.is_null() {
                (*lastmatch).next = matchtemplate
            } else {
                matches = matchtemplate
            }
            lastmatch = matchtemplate;
            //load the match template
            (*matchtemplate).first = BotLoadMatchPieces(
                source,
                b"=\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ); //end if
            if (*matchtemplate).first.is_null() {
                BotFreeMatchTemplates(matches);
                return 0 as *mut bot_matchtemplate_t;
            }
            //read the match type
            if crate::src::botlib::l_precomp::PC_ExpectTokenString(
                source as *mut crate::src::botlib::l_precomp::source_s,
                b"(\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) == 0
                || crate::src::botlib::l_precomp::PC_ExpectTokenType(
                    source as *mut crate::src::botlib::l_precomp::source_s,
                    3 as i32,
                    0x1000 as i32,
                    &mut token as *mut _ as *mut crate::src::botlib::l_script::token_s,
                ) == 0
            {
                BotFreeMatchTemplates(matches); //end if
                crate::src::botlib::l_precomp::FreeSource(
                    source as *mut crate::src::botlib::l_precomp::source_s,
                );
                return 0 as *mut bot_matchtemplate_t;
            }
            (*matchtemplate).type_0 = token.intvalue as i32;
            //read the match subtype
            if crate::src::botlib::l_precomp::PC_ExpectTokenString(
                source as *mut crate::src::botlib::l_precomp::source_s,
                b",\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) == 0
                || crate::src::botlib::l_precomp::PC_ExpectTokenType(
                    source as *mut crate::src::botlib::l_precomp::source_s,
                    3 as i32,
                    0x1000 as i32,
                    &mut token as *mut _ as *mut crate::src::botlib::l_script::token_s,
                ) == 0
            {
                BotFreeMatchTemplates(matches); //end if
                crate::src::botlib::l_precomp::FreeSource(
                    source as *mut crate::src::botlib::l_precomp::source_s,
                );
                return 0 as *mut bot_matchtemplate_t;
            }
            (*matchtemplate).subtype = token.intvalue as i32;
            if crate::src::botlib::l_precomp::PC_ExpectTokenString(
                source as *mut crate::src::botlib::l_precomp::source_s,
                b")\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) == 0
                || crate::src::botlib::l_precomp::PC_ExpectTokenString(
                    source as *mut crate::src::botlib::l_precomp::source_s,
                    b";\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ) == 0
            {
                BotFreeMatchTemplates(matches);
                crate::src::botlib::l_precomp::FreeSource(
                    source as *mut crate::src::botlib::l_precomp::source_s,
                );
                return 0 as *mut bot_matchtemplate_t;
            }
        }
    }
    //read trailing punctuations
    //free the source
    crate::src::botlib::l_precomp::FreeSource(
        source as *mut crate::src::botlib::l_precomp::source_s,
    );
    crate::src::botlib::be_interface::botimport
        .Print
        .expect("non-null function pointer")(
        1 as i32,
        b"loaded %s\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        matchfile,
    );
    return matches;
}
//
//BotDumpMatchTemplates(matches);
//
//end of the function BotLoadMatchTemplates
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn StringsMatch(
    mut pieces: *mut bot_matchpiece_t,
    mut match_0: *mut crate::src::botlib::be_ai_chat::bot_match_t,
) -> i32 {
    let mut lastvariable: i32 = 0;
    let mut index: i32 = 0;
    let mut strptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut newstrptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut mp: *mut bot_matchpiece_t = 0 as *mut bot_matchpiece_t;
    let mut ms: *mut bot_matchstring_t = 0 as *mut bot_matchstring_t;
    //no last variable
    lastvariable = -(1 as i32);
    //pointer to the string to compare the match string with
    strptr = (*match_0).string.as_mut_ptr();
    //Log_Write("match: %s", strptr);
    //compare the string with the current match string
    mp = pieces; //end for
    while !mp.is_null() {
        //if it is a piece of string
        if (*mp).type_0 == 2 as i32 {
            newstrptr = 0 as *mut libc::c_char; //end if
            ms = (*mp).firststring; //end for
            while !ms.is_null() {
                if crate::stdlib::strlen((*ms).string) == 0 {
                    newstrptr = strptr; //end if
                    break;
                } else {
                    //Log_Write("MT_STRING: %s", mp->string);
                    index = StringContains(
                        strptr,
                        (*ms).string,
                        crate::src::qcommon::q_shared::qfalse as i32,
                    ); //end else
                    if index >= 0 as i32 {
                        newstrptr = strptr.offset(index as isize); //end if
                        if lastvariable >= 0 as i32 {
                            (*match_0).variables[lastvariable as usize].length =
                                (newstrptr.offset_from((*match_0).string.as_mut_ptr())
                                    as isize
                                    - (*match_0).variables[lastvariable as usize].offset
                                        as isize) as i32;
                            //newstrptr - match->variables[lastvariable].ptr;
                            lastvariable = -(1 as i32);
                            break;
                        } else {
                            if index == 0 as i32 {
                                break;
                            }
                            newstrptr = 0 as *mut libc::c_char
                        }
                    }
                    ms = (*ms).next
                }
                //end if
            }
            if newstrptr.is_null() {
                return crate::src::qcommon::q_shared::qfalse as i32;
            }
            strptr = newstrptr.offset(crate::stdlib::strlen((*ms).string) as isize)
        } else if (*mp).type_0 == 1 as i32 {
            //if it is a variable piece of string
            //Log_Write("MT_VARIABLE");
            (*match_0).variables[(*mp).variable as usize].offset =
                strptr.offset_from((*match_0).string.as_mut_ptr()) as isize as libc::c_char;
            lastvariable = (*mp).variable
        }
        mp = (*mp).next
        //end else if
    }
    //if a match was found
    if mp.is_null() && (lastvariable >= 0 as i32 || crate::stdlib::strlen(strptr) == 0) {
        //end if
        //if the last piece was a variable string
        if lastvariable >= 0 as i32 {
            (*match_0).variables[lastvariable as usize].length = crate::stdlib::strlen(
                &mut *(*match_0).string.as_mut_ptr().offset(
                    (*(*match_0)
                        .variables
                        .as_mut_ptr()
                        .offset(lastvariable as isize))
                    .offset as i32 as isize,
                ),
            ) as i32
        } //end if
        return crate::src::qcommon::q_shared::qtrue as i32;
    }
    return crate::src::qcommon::q_shared::qfalse as i32;
}
//finds a match for the given string using the match templates
//end of the function StringsMatch
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotFindMatch(
    mut str: *mut libc::c_char,
    mut match_0: *mut crate::src::botlib::be_ai_chat::bot_match_t,
    mut context: libc::c_ulong,
) -> i32 {
    let mut i: i32 = 0;
    let mut ms: *mut bot_matchtemplate_t = 0 as *mut bot_matchtemplate_t;
    crate::src::qcommon::q_shared::Q_strncpyz((*match_0).string.as_mut_ptr(), str, 256 as i32);
    //remove any trailing enters
    while crate::stdlib::strlen((*match_0).string.as_mut_ptr()) != 0
        && (*match_0).string[crate::stdlib::strlen((*match_0).string.as_mut_ptr())
            .wrapping_sub(1 as i32 as libc::c_ulong) as usize] as i32
            == '\n' as i32
    {
        (*match_0).string[crate::stdlib::strlen((*match_0).string.as_mut_ptr())
            .wrapping_sub(1 as i32 as libc::c_ulong) as usize] = '\u{0}' as i32 as libc::c_char
    } //end while
      //compare the string with all the match strings
    ms = matchtemplates; //end for
    while !ms.is_null() {
        if !((*ms).context & context == 0) {
            //reset the match variable offsets
            i = 0 as i32;
            while i < 8 as i32 {
                (*match_0).variables[i as usize].offset = -(1 as i32) as libc::c_char;
                i += 1
            }
            //
            if StringsMatch((*ms).first, match_0) != 0 {
                (*match_0).type_0 = (*ms).type_0;
                (*match_0).subtype = (*ms).subtype;
                return crate::src::qcommon::q_shared::qtrue as i32;
            }
        }
        ms = (*ms).next
        //end if
    }
    return crate::src::qcommon::q_shared::qfalse as i32;
}
//returns a variable from a match
//end of the function BotFindMatch
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotMatchVariable(
    mut match_0: *mut crate::src::botlib::be_ai_chat::bot_match_t,
    mut variable: i32,
    mut buf: *mut libc::c_char,
    mut size: i32,
) {
    if variable < 0 as i32 || variable >= 8 as i32 {
        crate::src::botlib::be_interface::botimport
            .Print
            .expect("non-null function pointer")(
            4 as i32,
            b"BotMatchVariable: variable out of range\n\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        ); //end if
        ::libc::strcpy(buf, b"\x00" as *const u8 as *const libc::c_char); //end if
        return;
    }
    if (*match_0).variables[variable as usize].offset as i32 >= 0 as i32 {
        if (*match_0).variables[variable as usize].length < size {
            size = (*match_0).variables[variable as usize].length + 1 as i32
        }
        crate::stdlib::strncpy(
            buf,
            &mut *(*match_0).string.as_mut_ptr().offset(
                (*(*match_0).variables.as_mut_ptr().offset(variable as isize)).offset as i32
                    as isize,
            ),
            (size - 1 as i32) as libc::c_ulong,
        );
        *buf.offset((size - 1 as i32) as isize) = '\u{0}' as i32 as libc::c_char
    } else {
        ::libc::strcpy(buf, b"\x00" as *const u8 as *const libc::c_char);
    };
    //end else
}
//end of the function BotMatchVariable
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotFindStringInList(
    mut list: *mut bot_stringlist_t,
    mut string: *mut libc::c_char,
) -> *mut bot_stringlist_t {
    let mut s: *mut bot_stringlist_t = 0 as *mut bot_stringlist_t; //end for
    s = list;
    while !s.is_null() {
        if ::libc::strcmp((*s).string, string) == 0 {
            return s;
        }
        s = (*s).next
    }
    return 0 as *mut bot_stringlist_t;
}
//end of the function BotFindStringInList
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotCheckChatMessageIntegrety(
    mut message: *mut libc::c_char,
    mut stringlist: *mut bot_stringlist_t,
) -> *mut bot_stringlist_t {
    let mut i: i32 = 0;
    let mut msgptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut temp: [libc::c_char; 256] = [0; 256];
    let mut s: *mut bot_stringlist_t = 0 as *mut bot_stringlist_t;
    msgptr = message;
    //
    while *msgptr != 0 {
        if *msgptr as i32 == 0x1 as i32 {
            msgptr = msgptr.offset(1);
            match *msgptr as i32 {
                118 => {
                    //end while
                    //end switch
                    //variable
                    //step over the 'v'
                    msgptr = msgptr.offset(1);
                    while *msgptr as i32 != 0 && *msgptr as i32 != 0x1 as i32 {
                        msgptr = msgptr.offset(1)
                    }
                    //end default
                    if *msgptr != 0 {
                        msgptr = msgptr.offset(1)
                    }
                }
                114 => {
                    //step over the trailing escape char
                    //random
                    msgptr = msgptr.offset(1); //end case
                                               //step over the 'r'
                    i = 0 as i32; //end while
                    while *msgptr as i32 != 0 && *msgptr as i32 != 0x1 as i32 {
                        let fresh7 = msgptr;
                        msgptr = msgptr.offset(1);
                        temp[i as usize] = *fresh7;
                        i += 1
                    }
                    temp[i as usize] = '\u{0}' as i32 as libc::c_char;
                    //step over the trailing escape char
                    if *msgptr != 0 {
                        msgptr = msgptr.offset(1)
                    }
                    //find the random keyword
                    if RandomString(temp.as_mut_ptr()).is_null() {
                        if BotFindStringInList(stringlist, temp.as_mut_ptr()).is_null() {
                            crate::src::botlib::l_log::Log_Write(
                                b"%s = {\"%s\"} //MISSING RANDOM\r\n\x00" as *const u8
                                    as *const libc::c_char
                                    as *mut libc::c_char,
                                temp.as_mut_ptr(),
                                temp.as_mut_ptr(),
                            ); //end if
                            s = crate::src::botlib::l_memory::GetClearedMemory(
                                (::std::mem::size_of::<bot_stringlist_t>() as libc::c_ulong)
                                    .wrapping_add(crate::stdlib::strlen(temp.as_mut_ptr()))
                                    .wrapping_add(1 as i32 as libc::c_ulong),
                            ) as *mut bot_stringlist_t;
                            (*s).string = (s as *mut libc::c_char).offset(::std::mem::size_of::<
                                bot_stringlist_t,
                            >(
                            )
                                as libc::c_ulong
                                as isize);
                            ::libc::strcpy((*s).string, temp.as_mut_ptr());
                            (*s).next = stringlist;
                            stringlist = s
                        }
                        //end if
                    }
                }
                _ => {
                    crate::src::botlib::be_interface::botimport
                        .Print
                        .expect("non-null function pointer")(
                        4 as i32,
                        b"BotCheckChatMessageIntegrety: message \"%s\" invalid escape char\n\x00"
                            as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        message,
                    );
                }
            }
        } else {
            msgptr = msgptr.offset(1)
        }
        //end else
    }
    return stringlist;
}
//end of the function BotCheckChatMessageIntegrety
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotCheckInitialChatIntegrety(mut chat: *mut bot_chat_t) {
    let mut t: *mut bot_chattype_t = 0 as *mut bot_chattype_t; //end for
    let mut cm: *mut bot_chatmessage_t = 0 as *mut bot_chatmessage_t;
    let mut stringlist: *mut bot_stringlist_t = 0 as *mut bot_stringlist_t;
    let mut s: *mut bot_stringlist_t = 0 as *mut bot_stringlist_t;
    let mut nexts: *mut bot_stringlist_t = 0 as *mut bot_stringlist_t;
    stringlist = 0 as *mut bot_stringlist_t;
    t = (*chat).types;
    while !t.is_null() {
        cm = (*t).firstchatmessage;
        while !cm.is_null() {
            stringlist = BotCheckChatMessageIntegrety((*cm).chatmessage, stringlist);
            cm = (*cm).next
        }
        t = (*t).next
        //end for
    }
    s = stringlist;
    while !s.is_null() {
        nexts = (*s).next;
        crate::src::botlib::l_memory::FreeMemory(s as *mut libc::c_void);
        s = nexts
    }
    //end for
}
//end of the function BotCheckInitialChatIntegrety
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotCheckReplyChatIntegrety(mut replychat: *mut bot_replychat_t) {
    let mut rp: *mut bot_replychat_t = 0 as *mut bot_replychat_t; //end for
    let mut cm: *mut bot_chatmessage_t = 0 as *mut bot_chatmessage_t;
    let mut stringlist: *mut bot_stringlist_t = 0 as *mut bot_stringlist_t;
    let mut s: *mut bot_stringlist_t = 0 as *mut bot_stringlist_t;
    let mut nexts: *mut bot_stringlist_t = 0 as *mut bot_stringlist_t;
    stringlist = 0 as *mut bot_stringlist_t;
    rp = replychat;
    while !rp.is_null() {
        cm = (*rp).firstchatmessage;
        while !cm.is_null() {
            stringlist = BotCheckChatMessageIntegrety((*cm).chatmessage, stringlist);
            cm = (*cm).next
        }
        rp = (*rp).next
        //end for
    }
    s = stringlist;
    while !s.is_null() {
        nexts = (*s).next;
        crate::src::botlib::l_memory::FreeMemory(s as *mut libc::c_void);
        s = nexts
    }
    //end for
}
//end of the function BotCheckReplyChatIntegrety
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotDumpReplyChat(mut replychat: *mut bot_replychat_t) {
    let mut fp: *mut crate::stdlib::FILE = 0 as *mut crate::stdlib::FILE; //end for
    let mut rp: *mut bot_replychat_t = 0 as *mut bot_replychat_t;
    let mut key: *mut bot_replychatkey_t = 0 as *mut bot_replychatkey_t;
    let mut cm: *mut bot_chatmessage_t = 0 as *mut bot_chatmessage_t;
    let mut mp: *mut bot_matchpiece_t = 0 as *mut bot_matchpiece_t;
    fp = crate::src::botlib::l_log::Log_FilePointer() as *mut crate::stdlib::_IO_FILE;
    if fp.is_null() {
        return;
    }
    crate::stdlib::fprintf(
        fp,
        b"BotDumpReplyChat:\n\x00" as *const u8 as *const libc::c_char,
    );
    rp = replychat;
    while !rp.is_null() {
        crate::stdlib::fprintf(fp, b"[\x00" as *const u8 as *const libc::c_char);
        key = (*rp).keys;
        while !key.is_null() {
            if (*key).flags & 1 as i32 != 0 {
                crate::stdlib::fprintf(fp, b"&\x00" as *const u8 as *const libc::c_char);
            } else if (*key).flags & 2 as i32 != 0 {
                crate::stdlib::fprintf(fp, b"!\x00" as *const u8 as *const libc::c_char);
            }
            //
            if (*key).flags & 4 as i32 != 0 {
                crate::stdlib::fprintf(fp, b"name\x00" as *const u8 as *const libc::c_char);
            //end if
            } else if (*key).flags & 64 as i32 != 0 {
                crate::stdlib::fprintf(fp, b"female\x00" as *const u8 as *const libc::c_char);
            //end if
            } else if (*key).flags & 128 as i32 != 0 {
                crate::stdlib::fprintf(fp, b"male\x00" as *const u8 as *const libc::c_char);
            //end for
            } else if (*key).flags & 256 as i32 != 0 {
                crate::stdlib::fprintf(fp, b"it\x00" as *const u8 as *const libc::c_char);
            //end for
            } else if (*key).flags & 16 as i32 != 0 {
                crate::stdlib::fprintf(fp, b"(\x00" as *const u8 as *const libc::c_char);
                mp = (*key).match_0;
                while !mp.is_null() {
                    if (*mp).type_0 == 2 as i32 {
                        crate::stdlib::fprintf(
                            fp,
                            b"\"%s\"\x00" as *const u8 as *const libc::c_char,
                            (*(*mp).firststring).string,
                        );
                    } else {
                        crate::stdlib::fprintf(
                            fp,
                            b"%d\x00" as *const u8 as *const libc::c_char,
                            (*mp).variable,
                        );
                    }
                    if !(*mp).next.is_null() {
                        crate::stdlib::fprintf(fp, b", \x00" as *const u8 as *const libc::c_char);
                    }
                    mp = (*mp).next
                }
                crate::stdlib::fprintf(fp, b")\x00" as *const u8 as *const libc::c_char);
            } else if (*key).flags & 8 as i32 != 0 {
                crate::stdlib::fprintf(
                    fp,
                    b"\"%s\"\x00" as *const u8 as *const libc::c_char,
                    (*key).string,
                );
            }
            if !(*key).next.is_null() {
                crate::stdlib::fprintf(fp, b", \x00" as *const u8 as *const libc::c_char);
            } else {
                crate::stdlib::fprintf(
                    fp,
                    b"] = %1.0f\n\x00" as *const u8 as *const libc::c_char,
                    (*rp).priority as f64,
                );
            }
            key = (*key).next
        }
        crate::stdlib::fprintf(fp, b"{\n\x00" as *const u8 as *const libc::c_char);
        cm = (*rp).firstchatmessage;
        while !cm.is_null() {
            crate::stdlib::fprintf(
                fp,
                b"\t\"%s\";\n\x00" as *const u8 as *const libc::c_char,
                (*cm).chatmessage,
            );
            cm = (*cm).next
        }
        crate::stdlib::fprintf(fp, b"}\n\x00" as *const u8 as *const libc::c_char);
        rp = (*rp).next
    }
    //end for
}
//end of the function BotDumpReplyChat
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotFreeReplyChat(mut replychat: *mut bot_replychat_t) {
    let mut rp: *mut bot_replychat_t = 0 as *mut bot_replychat_t; //end for
    let mut nextrp: *mut bot_replychat_t = 0 as *mut bot_replychat_t; //end for
    let mut key: *mut bot_replychatkey_t = 0 as *mut bot_replychatkey_t;
    let mut nextkey: *mut bot_replychatkey_t = 0 as *mut bot_replychatkey_t;
    let mut cm: *mut bot_chatmessage_t = 0 as *mut bot_chatmessage_t;
    let mut nextcm: *mut bot_chatmessage_t = 0 as *mut bot_chatmessage_t;
    rp = replychat;
    while !rp.is_null() {
        nextrp = (*rp).next;
        key = (*rp).keys;
        while !key.is_null() {
            nextkey = (*key).next;
            if !(*key).match_0.is_null() {
                BotFreeMatchPieces((*key).match_0);
            }
            if !(*key).string.is_null() {
                crate::src::botlib::l_memory::FreeMemory((*key).string as *mut libc::c_void);
            }
            crate::src::botlib::l_memory::FreeMemory(key as *mut libc::c_void);
            key = nextkey
        }
        cm = (*rp).firstchatmessage;
        while !cm.is_null() {
            nextcm = (*cm).next;
            crate::src::botlib::l_memory::FreeMemory(cm as *mut libc::c_void);
            cm = nextcm
        }
        crate::src::botlib::l_memory::FreeMemory(rp as *mut libc::c_void);
        rp = nextrp
    }
    //end for
}
//end of the function BotFreeReplyChat
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotCheckValidReplyChatKeySet(
    mut source: *mut crate::src::botlib::l_precomp::source_t,
    mut keys: *mut bot_replychatkey_t,
) {
    let mut allprefixed: i32 = 0;
    let mut hasvariableskey: i32 = 0;
    let mut hasstringkey: i32 = 0;
    let mut m: *mut bot_matchpiece_t = 0 as *mut bot_matchpiece_t;
    let mut ms: *mut bot_matchstring_t = 0 as *mut bot_matchstring_t;
    let mut key: *mut bot_replychatkey_t = 0 as *mut bot_replychatkey_t;
    let mut key2: *mut bot_replychatkey_t = 0 as *mut bot_replychatkey_t;
    //
    allprefixed = crate::src::qcommon::q_shared::qtrue as i32; //end for
    hasstringkey = crate::src::qcommon::q_shared::qfalse as i32; //end else
    hasvariableskey = hasstringkey;
    key = keys;
    while !key.is_null() {
        if (*key).flags & (1 as i32 | 2 as i32) == 0 {
            allprefixed = crate::src::qcommon::q_shared::qfalse as i32;
            if (*key).flags & 16 as i32 != 0 {
                m = (*key).match_0;
                while !m.is_null() {
                    if (*m).type_0 == 1 as i32 {
                        hasvariableskey = crate::src::qcommon::q_shared::qtrue as i32
                    }
                    m = (*m).next
                }
            //end else if
            //end for
            } else if (*key).flags & 8 as i32 != 0 {
                hasstringkey = crate::src::qcommon::q_shared::qtrue as i32
            }
        } else if (*key).flags & 1 as i32 != 0 && (*key).flags & 8 as i32 != 0 {
            key2 = keys; //end for
            while !key2.is_null() {
                if !(key2 == key) {
                    if !((*key2).flags & 2 as i32 != 0) {
                        if (*key2).flags & 16 as i32 != 0 {
                            m = (*key2).match_0; //end if
                            while !m.is_null() {
                                if (*m).type_0 == 2 as i32 {
                                    ms = (*m).firststring; //end for
                                    while !ms.is_null() {
                                        if StringContains(
                                            (*ms).string,
                                            (*key).string,
                                            crate::src::qcommon::q_shared::qfalse as i32,
                                        ) != -(1 as i32)
                                        {
                                            break;
                                            //end if
                                        }
                                        ms = (*ms).next
                                    }
                                    if !ms.is_null() {
                                        break;
                                    }
                                } else if (*m).type_0 == 1 as i32 {
                                    break;
                                }
                                m = (*m).next
                                //end if
                            }
                            if m.is_null() {
                                crate::src::botlib::l_precomp::SourceWarning(source as *mut crate::src::botlib::l_precomp::source_s,
                                              b"one of the match templates does not leave space for the key %s with the & prefix\x00"
                                                  as *const u8 as
                                                  *const libc::c_char as
                                                  *mut libc::c_char,
                                              (*key).string);
                            }
                            //end if
                        }
                    }
                }
                key2 = (*key2).next
                //end if
            }
            //end for
        }
        if (*key).flags & 2 as i32 != 0 && (*key).flags & 8 as i32 != 0 {
            key2 = keys;
            while !key2.is_null() {
                if !(key2 == key) {
                    if !((*key2).flags & 2 as i32 != 0) {
                        if (*key2).flags & 8 as i32 != 0 {
                            if StringContains(
                                (*key2).string,
                                (*key).string,
                                crate::src::qcommon::q_shared::qfalse as i32,
                            ) != -(1 as i32)
                            {
                                crate::src::botlib::l_precomp::SourceWarning(
                                    source as *mut crate::src::botlib::l_precomp::source_s,
                                    b"the key %s with prefix ! is inside the key %s\x00"
                                        as *const u8
                                        as *const libc::c_char
                                        as *mut libc::c_char,
                                    (*key).string,
                                    (*key2).string,
                                );
                            }
                        //end if
                        //end if
                        } else if (*key2).flags & 16 as i32 != 0 {
                            m = (*key2).match_0;
                            while !m.is_null() {
                                if (*m).type_0 == 2 as i32 {
                                    ms = (*m).firststring;
                                    while !ms.is_null() {
                                        if StringContains(
                                            (*ms).string,
                                            (*key).string,
                                            crate::src::qcommon::q_shared::qfalse as i32,
                                        ) != -(1 as i32)
                                        {
                                            crate::src::botlib::l_precomp::SourceWarning(source as *mut crate::src::botlib::l_precomp::source_s,
                                                          b"the key %s with prefix ! is inside the match template string %s\x00"
                                                              as *const u8 as
                                                              *const libc::c_char
                                                              as
                                                              *mut libc::c_char,
                                                          (*key).string,
                                                          (*ms).string);
                                        }
                                        ms = (*ms).next
                                        //end if
                                    }
                                    //end for
                                }
                                m = (*m).next
                                //end if
                            }
                            //end for
                        }
                    }
                }
                key2 = (*key2).next
                //end else if
            }
            //end for
        }
        key = (*key).next
    }
    if allprefixed != 0 {
        crate::src::botlib::l_precomp::SourceWarning(
            source as *mut crate::src::botlib::l_precomp::source_s,
            b"all keys have a & or ! prefix\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    if hasvariableskey != 0 && hasstringkey != 0 {
        crate::src::botlib::l_precomp::SourceWarning(source as *mut crate::src::botlib::l_precomp::source_s,
                      b"variables from the match template(s) could be invalid when outputting one of the chat messages\x00"
                          as *const u8 as *const libc::c_char as
                          *mut libc::c_char);
    };
    //end if
}
//end of the function BotCheckValidReplyChatKeySet
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotLoadReplyChat(mut filename: *mut libc::c_char) -> *mut bot_replychat_t {
    let mut chatmessagestring: [libc::c_char; 256] = [0; 256]; //end if
    let mut namebuffer: [libc::c_char; 256] = [0; 256];
    let mut source: *mut crate::src::botlib::l_precomp::source_t =
        0 as *mut crate::src::botlib::l_precomp::source_t;
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
    let mut chatmessage: *mut bot_chatmessage_t = 0 as *mut bot_chatmessage_t;
    let mut replychat: *mut bot_replychat_t = 0 as *mut bot_replychat_t;
    let mut replychatlist: *mut bot_replychat_t = 0 as *mut bot_replychat_t;
    let mut key: *mut bot_replychatkey_t = 0 as *mut bot_replychatkey_t;
    crate::src::botlib::l_precomp::PC_SetBaseFolder(
        b"botfiles\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    source = crate::src::botlib::l_precomp::LoadSourceFile(filename)
        as *mut crate::src::botlib::l_precomp::source_s;
    if source.is_null() {
        crate::src::botlib::be_interface::botimport
            .Print
            .expect("non-null function pointer")(
            3 as i32,
            b"counldn\'t load %s\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            filename,
        );
        return 0 as *mut bot_replychat_t;
    }
    //
    replychatlist = 0 as *mut bot_replychat_t;
    //
    while crate::src::botlib::l_precomp::PC_ReadToken(
        source as *mut crate::src::botlib::l_precomp::source_s,
        &mut token as *mut _ as *mut crate::src::botlib::l_script::token_s,
    ) != 0
    {
        //end while
        if ::libc::strcmp(
            token.string.as_mut_ptr(),
            b"[\x00" as *const u8 as *const libc::c_char,
        ) != 0
        {
            crate::src::botlib::l_precomp::SourceError(
                source as *mut crate::src::botlib::l_precomp::source_s,
                b"expected [, found %s\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                token.string.as_mut_ptr(),
            ); //end if
            BotFreeReplyChat(replychatlist);
            crate::src::botlib::l_precomp::FreeSource(
                source as *mut crate::src::botlib::l_precomp::source_s,
            );
            return 0 as *mut bot_replychat_t;
        }
        //end while
        replychat = crate::src::botlib::l_memory::GetClearedHunkMemory(::std::mem::size_of::<
            bot_replychat_t,
        >() as libc::c_ulong) as *mut bot_replychat_t;
        (*replychat).keys = 0 as *mut bot_replychatkey_t;
        (*replychat).next = replychatlist;
        replychatlist = replychat;
        loop
        //
        //read the keys, there must be at least one key
        //allocate a key
        {
            key = crate::src::botlib::l_memory::GetClearedHunkMemory(::std::mem::size_of::<
                bot_replychatkey_t,
            >()
                as libc::c_ulong) as *mut bot_replychatkey_t;
            (*key).flags = 0 as i32;
            (*key).string = 0 as *mut libc::c_char;
            (*key).match_0 = 0 as *mut bot_matchpiece_t;
            (*key).next = (*replychat).keys;
            (*replychat).keys = key;
            //check for MUST BE PRESENT and MUST BE ABSENT keys
            if crate::src::botlib::l_precomp::PC_CheckTokenString(
                source as *mut crate::src::botlib::l_precomp::source_s,
                b"&\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) != 0
            {
                (*key).flags |= 1 as i32
            } else if crate::src::botlib::l_precomp::PC_CheckTokenString(
                source as *mut crate::src::botlib::l_precomp::source_s,
                b"!\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) != 0
            {
                (*key).flags |= 2 as i32
            }
            //special keys
            if crate::src::botlib::l_precomp::PC_CheckTokenString(
                source as *mut crate::src::botlib::l_precomp::source_s,
                b"name\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) != 0
            {
                (*key).flags |= 4 as i32
            } else if crate::src::botlib::l_precomp::PC_CheckTokenString(
                source as *mut crate::src::botlib::l_precomp::source_s,
                b"female\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) != 0
            {
                (*key).flags |= 64 as i32
            } else if crate::src::botlib::l_precomp::PC_CheckTokenString(
                source as *mut crate::src::botlib::l_precomp::source_s,
                b"male\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) != 0
            {
                (*key).flags |= 128 as i32
            } else if crate::src::botlib::l_precomp::PC_CheckTokenString(
                source as *mut crate::src::botlib::l_precomp::source_s,
                b"it\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) != 0
            {
                (*key).flags |= 256 as i32
            } else if crate::src::botlib::l_precomp::PC_CheckTokenString(
                source as *mut crate::src::botlib::l_precomp::source_s,
                b"(\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) != 0
            {
                //end else
                //match key
                (*key).flags |= 16 as i32;
                (*key).match_0 = BotLoadMatchPieces(
                    source,
                    b")\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
                if (*key).match_0.is_null() {
                    BotFreeReplyChat(replychatlist); //end else if
                    return 0 as *mut bot_replychat_t;
                }
            //end if
            } else if crate::src::botlib::l_precomp::PC_CheckTokenString(
                source as *mut crate::src::botlib::l_precomp::source_s,
                b"<\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) != 0
            {
                //bot names
                (*key).flags |= 32 as i32; //end else if
                ::libc::strcpy(
                    namebuffer.as_mut_ptr(),
                    b"\x00" as *const u8 as *const libc::c_char,
                ); //end if
                loop {
                    if crate::src::botlib::l_precomp::PC_ExpectTokenType(
                        source as *mut crate::src::botlib::l_precomp::source_s,
                        1 as i32,
                        0 as i32,
                        &mut token as *mut _ as *mut crate::src::botlib::l_script::token_s,
                    ) == 0
                    {
                        BotFreeReplyChat(replychatlist); //end if
                        crate::src::botlib::l_precomp::FreeSource(
                            source as *mut crate::src::botlib::l_precomp::source_s,
                        );
                        return 0 as *mut bot_replychat_t;
                    }
                    crate::src::botlib::l_script::StripDoubleQuotes(token.string.as_mut_ptr());
                    if crate::stdlib::strlen(namebuffer.as_mut_ptr()) != 0 {
                        ::libc::strcat(
                            namebuffer.as_mut_ptr(),
                            b"\\\x00" as *const u8 as *const libc::c_char,
                        );
                    }
                    ::libc::strcat(namebuffer.as_mut_ptr(), token.string.as_mut_ptr());
                    if !(crate::src::botlib::l_precomp::PC_CheckTokenString(
                        source as *mut crate::src::botlib::l_precomp::source_s,
                        b",\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    ) != 0)
                    {
                        break;
                    }
                }
                if crate::src::botlib::l_precomp::PC_ExpectTokenString(
                    source as *mut crate::src::botlib::l_precomp::source_s,
                    b">\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ) == 0
                {
                    BotFreeReplyChat(replychatlist);
                    crate::src::botlib::l_precomp::FreeSource(
                        source as *mut crate::src::botlib::l_precomp::source_s,
                    );
                    return 0 as *mut bot_replychat_t;
                }
                (*key).string = crate::src::botlib::l_memory::GetClearedHunkMemory(
                    crate::stdlib::strlen(namebuffer.as_mut_ptr())
                        .wrapping_add(1 as i32 as libc::c_ulong),
                ) as *mut libc::c_char;
                ::libc::strcpy((*key).string, namebuffer.as_mut_ptr());
            } else {
                //normal string key
                (*key).flags |= 8 as i32; //end if
                if crate::src::botlib::l_precomp::PC_ExpectTokenType(
                    source as *mut crate::src::botlib::l_precomp::source_s,
                    1 as i32,
                    0 as i32,
                    &mut token as *mut _ as *mut crate::src::botlib::l_script::token_s,
                ) == 0
                {
                    BotFreeReplyChat(replychatlist);
                    crate::src::botlib::l_precomp::FreeSource(
                        source as *mut crate::src::botlib::l_precomp::source_s,
                    );
                    return 0 as *mut bot_replychat_t;
                }
                crate::src::botlib::l_script::StripDoubleQuotes(token.string.as_mut_ptr());
                (*key).string = crate::src::botlib::l_memory::GetClearedHunkMemory(
                    crate::stdlib::strlen(token.string.as_mut_ptr())
                        .wrapping_add(1 as i32 as libc::c_ulong),
                ) as *mut libc::c_char;
                ::libc::strcpy((*key).string, token.string.as_mut_ptr());
            }
            //
            crate::src::botlib::l_precomp::PC_CheckTokenString(
                source as *mut crate::src::botlib::l_precomp::source_s,
                b",\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            if !(crate::src::botlib::l_precomp::PC_CheckTokenString(
                source as *mut crate::src::botlib::l_precomp::source_s,
                b"]\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) == 0)
            {
                break;
            }
        }
        BotCheckValidReplyChatKeySet(source, (*replychat).keys);
        if crate::src::botlib::l_precomp::PC_ExpectTokenString(
            source as *mut crate::src::botlib::l_precomp::source_s,
            b"=\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) == 0
            || crate::src::botlib::l_precomp::PC_ExpectTokenType(
                source as *mut crate::src::botlib::l_precomp::source_s,
                3 as i32,
                0 as i32,
                &mut token as *mut _ as *mut crate::src::botlib::l_script::token_s,
            ) == 0
        {
            BotFreeReplyChat(replychatlist);
            crate::src::botlib::l_precomp::FreeSource(
                source as *mut crate::src::botlib::l_precomp::source_s,
            );
            return 0 as *mut bot_replychat_t;
        }
        (*replychat).priority = token.floatvalue;
        if crate::src::botlib::l_precomp::PC_ExpectTokenString(
            source as *mut crate::src::botlib::l_precomp::source_s,
            b"{\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) == 0
        {
            BotFreeReplyChat(replychatlist);
            crate::src::botlib::l_precomp::FreeSource(
                source as *mut crate::src::botlib::l_precomp::source_s,
            );
            return 0 as *mut bot_replychat_t;
        }
        (*replychat).numchatmessages = 0 as i32;
        while crate::src::botlib::l_precomp::PC_CheckTokenString(
            source as *mut crate::src::botlib::l_precomp::source_s,
            b"}\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) == 0
        //
        //read the = sign and the priority
        //end if
        //read the leading {
        //end if
        //while the trailing } is not found
        {
            if BotLoadChatMessage(source, chatmessagestring.as_mut_ptr()) == 0 {
                BotFreeReplyChat(replychatlist); //end if
                crate::src::botlib::l_precomp::FreeSource(
                    source as *mut crate::src::botlib::l_precomp::source_s,
                );
                return 0 as *mut bot_replychat_t;
            }
            chatmessage = crate::src::botlib::l_memory::GetClearedHunkMemory(
                (::std::mem::size_of::<bot_chatmessage_t>() as libc::c_ulong)
                    .wrapping_add(crate::stdlib::strlen(chatmessagestring.as_mut_ptr()))
                    .wrapping_add(1 as i32 as libc::c_ulong),
            ) as *mut bot_chatmessage_t;
            (*chatmessage).chatmessage = (chatmessage as *mut libc::c_char)
                .offset(::std::mem::size_of::<bot_chatmessage_t>() as libc::c_ulong as isize);
            ::libc::strcpy((*chatmessage).chatmessage, chatmessagestring.as_mut_ptr());
            (*chatmessage).time = (-(2 as i32) * 20 as i32) as f32;
            (*chatmessage).next = (*replychat).firstchatmessage;
            //add the chat message to the reply chat
            (*replychat).firstchatmessage = chatmessage;
            (*replychat).numchatmessages += 1
        }
    }
    crate::src::botlib::l_precomp::FreeSource(
        source as *mut crate::src::botlib::l_precomp::source_s,
    );
    crate::src::botlib::be_interface::botimport
        .Print
        .expect("non-null function pointer")(
        1 as i32,
        b"loaded %s\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        filename,
    );
    //
    //BotDumpReplyChat(replychatlist);
    if crate::src::botlib::be_interface::botDeveloper != 0 {
        BotCheckReplyChatIntegrety(replychatlist); //end if
    }
    //
    if replychatlist.is_null() {
        crate::src::botlib::be_interface::botimport
            .Print
            .expect("non-null function pointer")(
            1 as i32,
            b"no rchats\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    //
    return replychatlist;
}
//end of the function BotLoadReplyChat
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotDumpInitialChat(mut chat: *mut bot_chat_t) {
    let mut t: *mut bot_chattype_t = 0 as *mut bot_chattype_t; //end for
    let mut m: *mut bot_chatmessage_t = 0 as *mut bot_chatmessage_t; //end for
    crate::src::botlib::l_log::Log_Write(
        b"{\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    t = (*chat).types;
    while !t.is_null() {
        crate::src::botlib::l_log::Log_Write(
            b" type \"%s\"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            (*t).name.as_mut_ptr(),
        );
        crate::src::botlib::l_log::Log_Write(
            b" {\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        crate::src::botlib::l_log::Log_Write(
            b"  numchatmessages = %d\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            (*t).numchatmessages,
        );
        m = (*t).firstchatmessage;
        while !m.is_null() {
            crate::src::botlib::l_log::Log_Write(
                b"  \"%s\"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                (*m).chatmessage,
            );
            m = (*m).next
        }
        crate::src::botlib::l_log::Log_Write(
            b" }\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        t = (*t).next
    }
    crate::src::botlib::l_log::Log_Write(
        b"}\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
//end of the function BotDumpInitialChat
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotLoadInitialChat(
    mut chatfile: *mut libc::c_char,
    mut chatname: *mut libc::c_char,
) -> *mut bot_chat_t {
    let mut pass: i32 = 0;
    let mut foundchat: i32 = 0;
    let mut indent: i32 = 0;
    let mut size: i32 = 0;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut chatmessagestring: [libc::c_char; 256] = [0; 256];
    let mut source: *mut crate::src::botlib::l_precomp::source_t =
        0 as *mut crate::src::botlib::l_precomp::source_t;
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
    let mut chat: *mut bot_chat_t = 0 as *mut bot_chat_t;
    let mut chattype: *mut bot_chattype_t = 0 as *mut bot_chattype_t;
    let mut chatmessage: *mut bot_chatmessage_t = 0 as *mut bot_chatmessage_t;
    //DEBUG
    //
    size = 0 as i32;
    foundchat = crate::src::qcommon::q_shared::qfalse as i32;
    //a bot chat is parsed in two phases
    pass = 0 as i32; //end for
    while pass < 2 as i32 {
        //allocate memory
        if pass != 0 && size != 0 {
            ptr = crate::src::botlib::l_memory::GetClearedMemory(size as libc::c_ulong)
                as *mut libc::c_char
        }
        //end if
        crate::src::botlib::l_precomp::PC_SetBaseFolder(
            b"botfiles\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        source = crate::src::botlib::l_precomp::LoadSourceFile(chatfile)
            as *mut crate::src::botlib::l_precomp::source_s;
        if source.is_null() {
            crate::src::botlib::be_interface::botimport
                .Print
                .expect("non-null function pointer")(
                3 as i32,
                b"counldn\'t load %s\n\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                chatfile,
            );
            return 0 as *mut bot_chat_t;
        }
        if pass != 0 {
            chat = ptr as *mut bot_chat_t;
            ptr = ptr.offset(::std::mem::size_of::<bot_chat_t>() as libc::c_ulong as isize)
        }
        size = ::std::mem::size_of::<bot_chat_t>() as libc::c_ulong as i32;
        while crate::src::botlib::l_precomp::PC_ReadToken(
            source as *mut crate::src::botlib::l_precomp::source_s,
            &mut token as *mut _ as *mut crate::src::botlib::l_script::token_s,
        ) != 0
        //load the source file
        //end if
        //chat structure
        //end if
        //
        {
            if ::libc::strcmp(
                token.string.as_mut_ptr(),
                b"chat\x00" as *const u8 as *const libc::c_char,
            ) == 0
            {
                //end while
                if crate::src::botlib::l_precomp::PC_ExpectTokenType(
                    source as *mut crate::src::botlib::l_precomp::source_s,
                    1 as i32,
                    0 as i32,
                    &mut token as *mut _ as *mut crate::src::botlib::l_script::token_s,
                ) == 0
                {
                    crate::src::botlib::l_precomp::FreeSource(
                        source as *mut crate::src::botlib::l_precomp::source_s,
                    ); //end if
                    return 0 as *mut bot_chat_t;
                }
                crate::src::botlib::l_script::StripDoubleQuotes(token.string.as_mut_ptr());
                //end else
                if crate::src::botlib::l_precomp::PC_ExpectTokenString(
                    source as *mut crate::src::botlib::l_precomp::source_s,
                    b"{\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ) == 0
                {
                    crate::src::botlib::l_precomp::FreeSource(
                        source as *mut crate::src::botlib::l_precomp::source_s,
                    );
                    return 0 as *mut bot_chat_t;
                }
                if crate::src::qcommon::q_shared::Q_stricmp(token.string.as_mut_ptr(), chatname)
                    == 0
                {
                    foundchat = crate::src::qcommon::q_shared::qtrue as i32;
                    loop
                    //after the chat name we expect an opening brace
                    //end if
                    //if the chat name is found
                    //end while
                    //read the chat types
                    {
                        if crate::src::botlib::l_precomp::PC_ExpectAnyToken(
                            source as *mut crate::src::botlib::l_precomp::source_s,
                            &mut token as *mut _ as *mut crate::src::botlib::l_script::token_s,
                        ) == 0
                        {
                            crate::src::botlib::l_precomp::FreeSource(
                                source as *mut crate::src::botlib::l_precomp::source_s,
                            ); //end if
                            return 0 as *mut bot_chat_t;
                        } //end if
                        if ::libc::strcmp(
                            token.string.as_mut_ptr(),
                            b"}\x00" as *const u8 as *const libc::c_char,
                        ) == 0
                        {
                            break;
                        }
                        if ::libc::strcmp(
                            token.string.as_mut_ptr(),
                            b"type\x00" as *const u8 as *const libc::c_char,
                        ) != 0
                        {
                            crate::src::botlib::l_precomp::SourceError(
                                source as *mut crate::src::botlib::l_precomp::source_s,
                                b"expected type found %s\x00" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                                token.string.as_mut_ptr(),
                            );
                            crate::src::botlib::l_precomp::FreeSource(
                                source as *mut crate::src::botlib::l_precomp::source_s,
                            );
                            return 0 as *mut bot_chat_t;
                        }
                        //expect the chat type name
                        if crate::src::botlib::l_precomp::PC_ExpectTokenType(
                            source as *mut crate::src::botlib::l_precomp::source_s,
                            1 as i32,
                            0 as i32,
                            &mut token as *mut _ as *mut crate::src::botlib::l_script::token_s,
                        ) == 0
                            || crate::src::botlib::l_precomp::PC_ExpectTokenString(
                                source as *mut crate::src::botlib::l_precomp::source_s,
                                b"{\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                            ) == 0
                        {
                            crate::src::botlib::l_precomp::FreeSource(
                                source as *mut crate::src::botlib::l_precomp::source_s,
                            ); //end if
                            return 0 as *mut bot_chat_t;
                        } //end if
                        crate::src::botlib::l_script::StripDoubleQuotes(token.string.as_mut_ptr());
                        if pass != 0 && !ptr.is_null() {
                            chattype = ptr as *mut bot_chattype_t;
                            crate::src::qcommon::q_shared::Q_strncpyz(
                                (*chattype).name.as_mut_ptr(),
                                token.string.as_mut_ptr(),
                                32 as i32,
                            );
                            (*chattype).firstchatmessage = 0 as *mut bot_chatmessage_t;
                            //add the chat type to the chat
                            (*chattype).next = (*chat).types;
                            (*chat).types = chattype;
                            //
                            ptr = ptr
                                .offset(::std::mem::size_of::<bot_chattype_t>() as libc::c_ulong
                                    as isize)
                        }
                        size = (size as libc::c_ulong)
                            .wrapping_add(::std::mem::size_of::<bot_chattype_t>() as libc::c_ulong)
                            as i32 as i32;
                        //read the chat messages
                        while crate::src::botlib::l_precomp::PC_CheckTokenString(
                            source as *mut crate::src::botlib::l_precomp::source_s,
                            b"}\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        ) == 0
                        {
                            let mut len: crate::stddef_h::size_t = 0; //end if
                            if BotLoadChatMessage(source, chatmessagestring.as_mut_ptr()) == 0 {
                                crate::src::botlib::l_precomp::FreeSource(
                                    source as *mut crate::src::botlib::l_precomp::source_s,
                                ); //end if
                                return 0 as *mut bot_chat_t;
                            }
                            len = crate::stdlib::strlen(chatmessagestring.as_mut_ptr())
                                .wrapping_add(1 as i32 as libc::c_ulong);
                            len =
                                len.wrapping_add(
                                    ::std::mem::size_of::<isize>() as libc::c_ulong
                                )
                                .wrapping_sub(1 as i32 as libc::c_ulong)
                                    & !(::std::mem::size_of::<isize>() as libc::c_ulong)
                                        .wrapping_sub(1 as i32 as libc::c_ulong);
                            if pass != 0 && !ptr.is_null() {
                                chatmessage = ptr as *mut bot_chatmessage_t;
                                (*chatmessage).time = (-(2 as i32) * 20 as i32) as f32;
                                //put the chat message in the list
                                (*chatmessage).next = (*chattype).firstchatmessage;
                                (*chattype).firstchatmessage = chatmessage;
                                //store the chat message
                                ptr = ptr.offset(::std::mem::size_of::<bot_chatmessage_t>()
                                    as libc::c_ulong
                                    as isize);
                                (*chatmessage).chatmessage = ptr;
                                ::libc::strcpy(
                                    (*chatmessage).chatmessage,
                                    chatmessagestring.as_mut_ptr(),
                                );
                                ptr = ptr.offset(len as isize);
                                //the number of chat messages increased
                                (*chattype).numchatmessages += 1
                            }
                            size = (size as libc::c_ulong).wrapping_add(
                                (::std::mem::size_of::<bot_chatmessage_t>() as libc::c_ulong)
                                    .wrapping_add(len),
                            ) as i32 as i32
                        }
                    }
                } else {
                    //skip the bot chat
                    indent = 1 as i32;
                    while indent != 0 {
                        //end while
                        if crate::src::botlib::l_precomp::PC_ExpectAnyToken(
                            source as *mut crate::src::botlib::l_precomp::source_s,
                            &mut token as *mut _ as *mut crate::src::botlib::l_script::token_s,
                        ) == 0
                        {
                            crate::src::botlib::l_precomp::FreeSource(
                                source as *mut crate::src::botlib::l_precomp::source_s,
                            ); //end if
                            return 0 as *mut bot_chat_t;
                        }
                        if ::libc::strcmp(
                            token.string.as_mut_ptr(),
                            b"{\x00" as *const u8 as *const libc::c_char,
                        ) == 0
                        {
                            indent += 1
                        } else if ::libc::strcmp(
                            token.string.as_mut_ptr(),
                            b"}\x00" as *const u8 as *const libc::c_char,
                        ) == 0
                        {
                            indent -= 1
                        }
                    }
                }
            } else {
                crate::src::botlib::l_precomp::SourceError(
                    source as *mut crate::src::botlib::l_precomp::source_s,
                    b"unknown definition %s\x00" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    token.string.as_mut_ptr(),
                );
                crate::src::botlib::l_precomp::FreeSource(
                    source as *mut crate::src::botlib::l_precomp::source_s,
                );
                return 0 as *mut bot_chat_t;
            }
            //end else
        }
        crate::src::botlib::l_precomp::FreeSource(
            source as *mut crate::src::botlib::l_precomp::source_s,
        );
        if foundchat == 0 {
            crate::src::botlib::be_interface::botimport
                .Print
                .expect("non-null function pointer")(
                3 as i32,
                b"couldn\'t find chat %s in %s\n\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                chatname,
                chatfile,
            );
            return 0 as *mut bot_chat_t;
        }
        pass += 1
    }
    //free the source
    //if the requested character is not found
    //
    crate::src::botlib::be_interface::botimport
        .Print
        .expect("non-null function pointer")(
        1 as i32,
        b"loaded %s from %s\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        chatname,
        chatfile,
    );
    //
    //BotDumpInitialChat(chat);
    if crate::src::botlib::be_interface::botDeveloper != 0 {
        BotCheckInitialChatIntegrety(chat); //end if
    }
    //DEBUG
    //character was read successfully
    return chat;
}
//end of the function BotLoadInitialChat
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotFreeChatFile(mut chatstate: i32) {
    let mut cs: *mut bot_chatstate_t = 0 as *mut bot_chatstate_t;
    cs = BotChatStateFromHandle(chatstate);
    if cs.is_null() {
        return;
    }
    if !(*cs).chat.is_null() {
        crate::src::botlib::l_memory::FreeMemory((*cs).chat as *mut libc::c_void);
    }
    (*cs).chat = 0 as *mut bot_chat_t;
}
//loads a chat file for the chat state
//end of the function BotFreeChatFile
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotLoadChatFile(
    mut chatstate: i32,
    mut chatfile: *mut libc::c_char,
    mut chatname: *mut libc::c_char,
) -> i32 {
    let mut cs: *mut bot_chatstate_t = 0 as *mut bot_chatstate_t;
    let mut n: i32 = 0;
    let mut avail: i32 = 0 as i32;
    cs = BotChatStateFromHandle(chatstate);
    if cs.is_null() {
        return 8 as i32;
    }
    BotFreeChatFile(chatstate);
    if crate::src::botlib::l_libvar::LibVarGetValue(
        b"bot_reloadcharacters\x00" as *const u8 as *const libc::c_char,
    ) == 0.
    {
        avail = -(1 as i32);
        n = 0 as i32;
        while n < 64 as i32 {
            if ichatdata[n as usize].is_null() {
                if avail == -(1 as i32) {
                    avail = n
                }
            } else if !(::libc::strcmp(chatfile, (*ichatdata[n as usize]).filename.as_mut_ptr())
                != 0 as i32)
            {
                if !(::libc::strcmp(chatname, (*ichatdata[n as usize]).chatname.as_mut_ptr())
                    != 0 as i32)
                {
                    (*cs).chat = (*ichatdata[n as usize]).chat;
                    //		botimport.Print( PRT_MESSAGE, "retained %s from %s\n", chatname, chatfile );
                    return 0 as i32;
                }
            } //end if
            n += 1
        } //end if
        if avail == -(1 as i32) {
            crate::src::botlib::be_interface::botimport
                .Print
                .expect("non-null function pointer")(
                4 as i32,
                b"ichatdata table full; couldn\'t load chat %s from %s\n\x00" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                chatname,
                chatfile,
            );
            return 8 as i32;
        }
    }
    (*cs).chat = BotLoadInitialChat(chatfile, chatname);
    if (*cs).chat.is_null() {
        crate::src::botlib::be_interface::botimport
            .Print
            .expect("non-null function pointer")(
            4 as i32,
            b"couldn\'t load chat %s from %s\n\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            chatname,
            chatfile,
        );
        return 8 as i32;
    }
    if crate::src::botlib::l_libvar::LibVarGetValue(
        b"bot_reloadcharacters\x00" as *const u8 as *const libc::c_char,
    ) == 0.
    {
        ichatdata[avail as usize] = crate::src::botlib::l_memory::GetClearedMemory(
            ::std::mem::size_of::<bot_ichatdata_t>() as libc::c_ulong,
        ) as *mut bot_ichatdata_t;
        (*ichatdata[avail as usize]).chat = (*cs).chat;
        crate::src::qcommon::q_shared::Q_strncpyz(
            (*ichatdata[avail as usize]).chatname.as_mut_ptr(),
            chatname,
            ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as i32,
        );
        crate::src::qcommon::q_shared::Q_strncpyz(
            (*ichatdata[avail as usize]).filename.as_mut_ptr(),
            chatfile,
            ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as i32,
        );
    }
    return 0 as i32;
}
//end of the function BotLoadChatFile
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotExpandChatMessage(
    mut outmessage: *mut libc::c_char,
    mut message: *mut libc::c_char,
    mut mcontext: libc::c_ulong,
    mut match_0: *mut crate::src::botlib::be_ai_chat::bot_match_t,
    mut vcontext: libc::c_ulong,
    mut reply: i32,
) -> i32 {
    let mut num: i32 = 0;
    let mut len: i32 = 0;
    let mut i: i32 = 0;
    let mut expansion: i32 = 0;
    let mut outputbuf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut msgptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut temp: [libc::c_char; 256] = [0; 256];
    expansion = crate::src::qcommon::q_shared::qfalse as i32;
    msgptr = message;
    outputbuf = outmessage;
    len = 0 as i32;
    //
    while *msgptr != 0 {
        if *msgptr as i32 == 0x1 as i32 {
            msgptr = msgptr.offset(1);
            match *msgptr as i32 {
                118 => {
                    //end while
                    //end switch
                    //variable
                    msgptr = msgptr.offset(1);
                    num = 0 as i32;
                    //end default
                    while *msgptr as i32 != 0 && *msgptr as i32 != 0x1 as i32 {
                        let fresh8 = msgptr; //end while
                        msgptr = msgptr.offset(1);
                        num = num * 10 as i32 + *fresh8 as i32 - '0' as i32
                    }
                    //step over the trailing escape char
                    if *msgptr != 0 {
                        msgptr = msgptr.offset(1)
                    } //end if
                    if num > 8 as i32 {
                        crate::src::botlib::be_interface::botimport
                            .Print
                            .expect("non-null function pointer")(
                            3 as i32,
                            b"BotConstructChat: message %s variable %d out of range\n\x00"
                                as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                            message,
                            num,
                        ); //end if
                        return crate::src::qcommon::q_shared::qfalse as i32;
                    } //end for
                    if (*match_0).variables[num as usize].offset as i32 >= 0 as i32 {
                        ptr = &mut *(*match_0).string.as_mut_ptr().offset(
                            (*(*match_0).variables.as_mut_ptr().offset(num as isize)).offset as i32
                                as isize,
                        ) as *mut libc::c_char;
                        i = 0 as i32;
                        while i < (*match_0).variables[num as usize].length {
                            temp[i as usize] = *ptr.offset(i as isize);
                            i += 1
                        }
                        temp[i as usize] = 0 as i32 as libc::c_char;
                        //if it's a reply message
                        if reply != 0 {
                            //end else
                            BotReplaceReplySynonyms(temp.as_mut_ptr(), vcontext);
                        //end if
                        } else {
                            //replace the reply synonyms in the variables
                            //replace synonyms in the variable context
                            BotReplaceSynonyms(temp.as_mut_ptr(), vcontext);
                        }
                        //
                        if (len as libc::c_ulong)
                            .wrapping_add(crate::stdlib::strlen(temp.as_mut_ptr()))
                            >= 256 as i32 as libc::c_ulong
                        {
                            crate::src::botlib::be_interface::botimport
                                .Print
                                .expect("non-null function pointer")(
                                3 as i32,
                                b"BotConstructChat: message %s too long\n\x00" as *const u8
                                    as *const libc::c_char
                                    as *mut libc::c_char,
                                message,
                            ); //end if
                            return crate::src::qcommon::q_shared::qfalse as i32;
                        }
                        ::libc::strcpy(&mut *outputbuf.offset(len as isize), temp.as_mut_ptr());
                        len = (len as libc::c_ulong)
                            .wrapping_add(crate::stdlib::strlen(temp.as_mut_ptr()))
                            as i32 as i32
                    }
                }
                114 => {
                    //random
                    msgptr = msgptr.offset(1); //end case
                    i = 0 as i32; //end while
                    while *msgptr as i32 != 0 && *msgptr as i32 != 0x1 as i32 {
                        let fresh9 = msgptr;
                        msgptr = msgptr.offset(1);
                        temp[i as usize] = *fresh9;
                        i += 1
                    }
                    temp[i as usize] = '\u{0}' as i32 as libc::c_char;
                    //step over the trailing escape char
                    if *msgptr != 0 {
                        msgptr = msgptr.offset(1)
                    }
                    //find the random keyword
                    ptr = RandomString(temp.as_mut_ptr()); //end if
                    if ptr.is_null() {
                        crate::src::botlib::be_interface::botimport
                            .Print
                            .expect("non-null function pointer")(
                            3 as i32,
                            b"BotConstructChat: unknown random string %s\n\x00" as *const u8
                                as *const libc::c_char
                                as *mut libc::c_char,
                            temp.as_mut_ptr(),
                        ); //end if
                        return crate::src::qcommon::q_shared::qfalse as i32;
                    }
                    if (len as libc::c_ulong).wrapping_add(crate::stdlib::strlen(ptr))
                        >= 256 as i32 as libc::c_ulong
                    {
                        crate::src::botlib::be_interface::botimport
                            .Print
                            .expect("non-null function pointer")(
                            3 as i32,
                            b"BotConstructChat: message \"%s\" too long\n\x00" as *const u8
                                as *const libc::c_char
                                as *mut libc::c_char,
                            message,
                        );
                        return crate::src::qcommon::q_shared::qfalse as i32;
                    }
                    ::libc::strcpy(&mut *outputbuf.offset(len as isize), ptr);
                    len = (len as libc::c_ulong).wrapping_add(crate::stdlib::strlen(ptr)) as i32
                        as i32;
                    expansion = crate::src::qcommon::q_shared::qtrue as i32
                }
                _ => {
                    crate::src::botlib::be_interface::botimport
                        .Print
                        .expect("non-null function pointer")(
                        4 as i32,
                        b"BotConstructChat: message \"%s\" invalid escape char\n\x00" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                        message,
                    );
                }
            }
        } else {
            let fresh10 = msgptr;
            msgptr = msgptr.offset(1);
            let fresh11 = len;
            len = len + 1;
            *outputbuf.offset(fresh11 as isize) = *fresh10;
            if !(len >= 256 as i32) {
                continue;
            }
            crate::src::botlib::be_interface::botimport
                .Print
                .expect("non-null function pointer")(
                3 as i32,
                b"BotConstructChat: message \"%s\" too long\n\x00" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                message,
            );
            break;
            //end if
        }
        //end else
    }
    *outputbuf.offset(len as isize) = '\u{0}' as i32 as libc::c_char;
    //replace synonyms weighted in the message context
    BotReplaceWeightedSynonyms(outputbuf, mcontext);
    //return true if a random was expanded
    return expansion;
}
//end of the function BotExpandChatMessage
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotConstructChatMessage(
    mut chatstate: *mut bot_chatstate_t,
    mut message: *mut libc::c_char,
    mut mcontext: libc::c_ulong,
    mut match_0: *mut crate::src::botlib::be_ai_chat::bot_match_t,
    mut vcontext: libc::c_ulong,
    mut reply: i32,
) {
    let mut i: i32 = 0; //end for
    let mut srcmessage: [libc::c_char; 256] = [0; 256]; //end if
    ::libc::strcpy(srcmessage.as_mut_ptr(), message);
    i = 0 as i32;
    while i < 10 as i32 {
        if BotExpandChatMessage(
            (*chatstate).chatmessage.as_mut_ptr(),
            srcmessage.as_mut_ptr(),
            mcontext,
            match_0,
            vcontext,
            reply,
        ) == 0
        {
            break;
        }
        ::libc::strcpy(
            srcmessage.as_mut_ptr(),
            (*chatstate).chatmessage.as_mut_ptr(),
        );
        i += 1
    }
    if i >= 10 as i32 {
        crate::src::botlib::be_interface::botimport
            .Print
            .expect("non-null function pointer")(
            2 as i32,
            b"too many expansions in chat message\n\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        crate::src::botlib::be_interface::botimport
            .Print
            .expect("non-null function pointer")(
            2 as i32,
            b"%s\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            (*chatstate).chatmessage.as_mut_ptr(),
        );
    };
    //end if
}
//end of the function BotConstructChatMessage
//===========================================================================
// randomly chooses one of the chat message of the given type
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotChooseInitialChatMessage(
    mut cs: *mut bot_chatstate_t,
    mut type_0: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut n: i32 = 0; //end for
    let mut numchatmessages: i32 = 0; //end if
    let mut besttime: f32 = 0.;
    let mut t: *mut bot_chattype_t = 0 as *mut bot_chattype_t;
    let mut m: *mut bot_chatmessage_t = 0 as *mut bot_chatmessage_t;
    let mut bestchatmessage: *mut bot_chatmessage_t = 0 as *mut bot_chatmessage_t;
    let mut chat: *mut bot_chat_t = 0 as *mut bot_chat_t;
    chat = (*cs).chat;
    t = (*chat).types;
    while !t.is_null() {
        if crate::src::qcommon::q_shared::Q_stricmp((*t).name.as_mut_ptr(), type_0) == 0 {
            numchatmessages = 0 as i32;
            m = (*t).firstchatmessage;
            while !m.is_null() {
                if !((*m).time > crate::src::botlib::be_aas_main::AAS_Time()) {
                    numchatmessages += 1
                }
                m = (*m).next
            }
            //if all chat messages have been used recently
            if numchatmessages <= 0 as i32 {
                //end else
                besttime = 0 as i32 as f32; //end if
                bestchatmessage = 0 as *mut bot_chatmessage_t; //end for
                m = (*t).firstchatmessage;
                while !m.is_null() {
                    if besttime == 0. || (*m).time < besttime {
                        bestchatmessage = m;
                        besttime = (*m).time
                    }
                    m = (*m).next
                    //end if
                }
                if !bestchatmessage.is_null() {
                    return (*bestchatmessage).chatmessage;
                }
            } else {
                //choose a chat message randomly
                n = ((::libc::rand() & 0x7fff as i32) as f32 / 0x7fff as i32 as f32
                    * numchatmessages as f32) as i32;
                m = (*t).firstchatmessage;
                while !m.is_null() {
                    if !((*m).time > crate::src::botlib::be_aas_main::AAS_Time()) {
                        n -= 1;
                        if n < 0 as i32 {
                            (*m).time =
                                crate::src::botlib::be_aas_main::AAS_Time() + 20 as i32 as f32;
                            return (*m).chatmessage;
                        }
                    }
                    m = (*m).next
                    //end for
                    //end if
                }
            }
            return 0 as *mut libc::c_char;
        }
        t = (*t).next
        //end if
    }
    return 0 as *mut libc::c_char;
}
//returns the number of initial chat messages of the given type
//end of the function BotChooseInitialChatMessage
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotNumInitialChats(
    mut chatstate: i32,
    mut type_0: *mut libc::c_char,
) -> i32 {
    let mut cs: *mut bot_chatstate_t = 0 as *mut bot_chatstate_t; //end for
    let mut t: *mut bot_chattype_t = 0 as *mut bot_chattype_t;
    cs = BotChatStateFromHandle(chatstate);
    if cs.is_null() {
        return 0 as i32;
    }
    t = (*(*cs).chat).types;
    while !t.is_null() {
        if crate::src::qcommon::q_shared::Q_stricmp((*t).name.as_mut_ptr(), type_0) == 0 {
            if crate::src::botlib::l_libvar::LibVarGetValue(
                b"bot_testichat\x00" as *const u8 as *const libc::c_char,
            ) != 0.
            {
                crate::src::botlib::be_interface::botimport
                    .Print
                    .expect("non-null function pointer")(
                    1 as i32,
                    b"%s has %d chat lines\n\x00" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    type_0,
                    (*t).numchatmessages,
                );
                crate::src::botlib::be_interface::botimport
                    .Print
                    .expect("non-null function pointer")(
                    1 as i32,
                    b"-------------------\n\x00" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
            }
            return (*t).numchatmessages;
        }
        t = (*t).next
        //end if
    }
    return 0 as i32;
}
//selects a chat message of the given type
//end of the function BotNumInitialChats
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotInitialChat(
    mut chatstate: i32,
    mut type_0: *mut libc::c_char,
    mut mcontext: i32,
    mut var0: *mut libc::c_char,
    mut var1: *mut libc::c_char,
    mut var2: *mut libc::c_char,
    mut var3: *mut libc::c_char,
    mut var4: *mut libc::c_char,
    mut var5: *mut libc::c_char,
    mut var6: *mut libc::c_char,
    mut var7: *mut libc::c_char,
) {
    let mut message: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut index: i32 = 0;
    let mut match_0: crate::src::botlib::be_ai_chat::bot_match_t =
        crate::src::botlib::be_ai_chat::bot_match_t {
            string: [0; 256],
            type_0: 0,
            subtype: 0,
            variables: [crate::src::botlib::be_ai_chat::bot_matchvariable_t {
                offset: 0,
                length: 0,
            }; 8],
        };
    let mut cs: *mut bot_chatstate_t = 0 as *mut bot_chatstate_t;
    cs = BotChatStateFromHandle(chatstate);
    if cs.is_null() {
        return;
    }
    //if no chat file is loaded
    if (*cs).chat.is_null() {
        return;
    }
    //choose a chat message randomly of the given type
    message = BotChooseInitialChatMessage(cs, type_0);
    //if there's no message of the given type
    if message.is_null() {
        //end if
        //DEBUG
        return;
    }
    //
    crate::stdlib::memset(
        &mut match_0 as *mut crate::src::botlib::be_ai_chat::bot_match_t as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<crate::src::botlib::be_ai_chat::bot_match_t>() as libc::c_ulong,
    );
    index = 0 as i32;
    if !var0.is_null() {
        ::libc::strcat(match_0.string.as_mut_ptr(), var0);
        match_0.variables[0 as i32 as usize].offset = index as libc::c_char;
        match_0.variables[0 as i32 as usize].length = crate::stdlib::strlen(var0) as i32;
        index = (index as libc::c_ulong).wrapping_add(crate::stdlib::strlen(var0)) as i32 as i32
    }
    if !var1.is_null() {
        ::libc::strcat(match_0.string.as_mut_ptr(), var1);
        match_0.variables[1 as i32 as usize].offset = index as libc::c_char;
        match_0.variables[1 as i32 as usize].length = crate::stdlib::strlen(var1) as i32;
        index = (index as libc::c_ulong).wrapping_add(crate::stdlib::strlen(var1)) as i32 as i32
    }
    if !var2.is_null() {
        ::libc::strcat(match_0.string.as_mut_ptr(), var2);
        match_0.variables[2 as i32 as usize].offset = index as libc::c_char;
        match_0.variables[2 as i32 as usize].length = crate::stdlib::strlen(var2) as i32;
        index = (index as libc::c_ulong).wrapping_add(crate::stdlib::strlen(var2)) as i32 as i32
    }
    if !var3.is_null() {
        ::libc::strcat(match_0.string.as_mut_ptr(), var3);
        match_0.variables[3 as i32 as usize].offset = index as libc::c_char;
        match_0.variables[3 as i32 as usize].length = crate::stdlib::strlen(var3) as i32;
        index = (index as libc::c_ulong).wrapping_add(crate::stdlib::strlen(var3)) as i32 as i32
    }
    if !var4.is_null() {
        ::libc::strcat(match_0.string.as_mut_ptr(), var4);
        match_0.variables[4 as i32 as usize].offset = index as libc::c_char;
        match_0.variables[4 as i32 as usize].length = crate::stdlib::strlen(var4) as i32;
        index = (index as libc::c_ulong).wrapping_add(crate::stdlib::strlen(var4)) as i32 as i32
    }
    if !var5.is_null() {
        ::libc::strcat(match_0.string.as_mut_ptr(), var5);
        match_0.variables[5 as i32 as usize].offset = index as libc::c_char;
        match_0.variables[5 as i32 as usize].length = crate::stdlib::strlen(var5) as i32;
        index = (index as libc::c_ulong).wrapping_add(crate::stdlib::strlen(var5)) as i32 as i32
    }
    if !var6.is_null() {
        ::libc::strcat(match_0.string.as_mut_ptr(), var6);
        match_0.variables[6 as i32 as usize].offset = index as libc::c_char;
        match_0.variables[6 as i32 as usize].length = crate::stdlib::strlen(var6) as i32;
        index = (index as libc::c_ulong).wrapping_add(crate::stdlib::strlen(var6)) as i32 as i32
    }
    if !var7.is_null() {
        ::libc::strcat(match_0.string.as_mut_ptr(), var7);
        match_0.variables[7 as i32 as usize].offset = index as libc::c_char;
        match_0.variables[7 as i32 as usize].length = crate::stdlib::strlen(var7) as i32
    }
    //
    BotConstructChatMessage(
        cs,
        message,
        mcontext as libc::c_ulong,
        &mut match_0,
        0 as i32 as libc::c_ulong,
        crate::src::qcommon::q_shared::qfalse as i32,
    );
}
//end of the function BotInitialChat
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotPrintReplyChatKeys(mut replychat: *mut bot_replychat_t) {
    let mut key: *mut bot_replychatkey_t = 0 as *mut bot_replychatkey_t; //end for
    let mut mp: *mut bot_matchpiece_t = 0 as *mut bot_matchpiece_t;
    crate::src::botlib::be_interface::botimport
        .Print
        .expect("non-null function pointer")(
        1 as i32,
        b"[\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    key = (*replychat).keys;
    while !key.is_null() {
        if (*key).flags & 1 as i32 != 0 {
            crate::src::botlib::be_interface::botimport
                .Print
                .expect("non-null function pointer")(
                1 as i32,
                b"&\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        } else if (*key).flags & 2 as i32 != 0 {
            crate::src::botlib::be_interface::botimport
                .Print
                .expect("non-null function pointer")(
                1 as i32,
                b"!\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        }
        //
        if (*key).flags & 4 as i32 != 0 {
            crate::src::botlib::be_interface::botimport
                .Print
                .expect("non-null function pointer")(
                1 as i32,
                b"name\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ); //end if
        } else if (*key).flags & 64 as i32 != 0 {
            crate::src::botlib::be_interface::botimport
                .Print
                .expect("non-null function pointer")(
                1 as i32,
                b"female\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ); //end if
        } else if (*key).flags & 128 as i32 != 0 {
            crate::src::botlib::be_interface::botimport
                .Print
                .expect("non-null function pointer")(
                1 as i32,
                b"male\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ); //end for
        } else if (*key).flags & 256 as i32 != 0 {
            crate::src::botlib::be_interface::botimport
                .Print
                .expect("non-null function pointer")(
                1 as i32,
                b"it\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        } else if (*key).flags & 16 as i32 != 0 {
            crate::src::botlib::be_interface::botimport
                .Print
                .expect("non-null function pointer")(
                1 as i32,
                b"(\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            mp = (*key).match_0;
            while !mp.is_null() {
                if (*mp).type_0 == 2 as i32 {
                    crate::src::botlib::be_interface::botimport
                        .Print
                        .expect("non-null function pointer")(
                        1 as i32,
                        b"\"%s\"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        (*(*mp).firststring).string,
                    );
                } else {
                    crate::src::botlib::be_interface::botimport
                        .Print
                        .expect("non-null function pointer")(
                        1 as i32,
                        b"%d\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        (*mp).variable,
                    );
                }
                if !(*mp).next.is_null() {
                    crate::src::botlib::be_interface::botimport
                        .Print
                        .expect("non-null function pointer")(
                        1 as i32,
                        b", \x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    );
                }
                mp = (*mp).next
            }
            crate::src::botlib::be_interface::botimport
                .Print
                .expect("non-null function pointer")(
                1 as i32,
                b")\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        } else if (*key).flags & 8 as i32 != 0 {
            crate::src::botlib::be_interface::botimport
                .Print
                .expect("non-null function pointer")(
                1 as i32,
                b"\"%s\"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                (*key).string,
            );
        }
        if !(*key).next.is_null() {
            crate::src::botlib::be_interface::botimport
                .Print
                .expect("non-null function pointer")(
                1 as i32,
                b", \x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        } else {
            crate::src::botlib::be_interface::botimport
                .Print
                .expect("non-null function pointer")(
                1 as i32,
                b"] = %1.0f\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                (*replychat).priority as f64,
            );
        }
        key = (*key).next
    }
    crate::src::botlib::be_interface::botimport
        .Print
        .expect("non-null function pointer")(
        1 as i32,
        b"{\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
//find and select a reply for the given message
//end of the function BotPrintReplyChatKeys
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotReplyChat(
    mut chatstate: i32,
    mut message: *mut libc::c_char,
    mut mcontext: i32,
    mut vcontext: i32,
    mut var0: *mut libc::c_char,
    mut var1: *mut libc::c_char,
    mut var2: *mut libc::c_char,
    mut var3: *mut libc::c_char,
    mut var4: *mut libc::c_char,
    mut var5: *mut libc::c_char,
    mut var6: *mut libc::c_char,
    mut var7: *mut libc::c_char,
) -> i32 {
    let mut rchat: *mut bot_replychat_t = 0 as *mut bot_replychat_t;
    let mut bestrchat: *mut bot_replychat_t = 0 as *mut bot_replychat_t;
    let mut key: *mut bot_replychatkey_t = 0 as *mut bot_replychatkey_t;
    let mut m: *mut bot_chatmessage_t = 0 as *mut bot_chatmessage_t;
    let mut bestchatmessage: *mut bot_chatmessage_t = 0 as *mut bot_chatmessage_t;
    let mut match_0: crate::src::botlib::be_ai_chat::bot_match_t =
        crate::src::botlib::be_ai_chat::bot_match_t {
            string: [0; 256],
            type_0: 0,
            subtype: 0,
            variables: [crate::src::botlib::be_ai_chat::bot_matchvariable_t {
                offset: 0,
                length: 0,
            }; 8],
        };
    let mut bestmatch: crate::src::botlib::be_ai_chat::bot_match_t =
        crate::src::botlib::be_ai_chat::bot_match_t {
            string: [0; 256],
            type_0: 0,
            subtype: 0,
            variables: [crate::src::botlib::be_ai_chat::bot_matchvariable_t {
                offset: 0,
                length: 0,
            }; 8],
        };
    let mut bestpriority: i32 = 0;
    let mut num: i32 = 0;
    let mut found: i32 = 0;
    let mut res: i32 = 0;
    let mut numchatmessages: i32 = 0;
    let mut index: i32 = 0;
    let mut cs: *mut bot_chatstate_t = 0 as *mut bot_chatstate_t;
    cs = BotChatStateFromHandle(chatstate);
    if cs.is_null() {
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    crate::stdlib::memset(
        &mut match_0 as *mut crate::src::botlib::be_ai_chat::bot_match_t as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<crate::src::botlib::be_ai_chat::bot_match_t>() as libc::c_ulong,
    );
    ::libc::strcpy(match_0.string.as_mut_ptr(), message);
    bestpriority = -(1 as i32);
    bestchatmessage = 0 as *mut bot_chatmessage_t;
    bestrchat = 0 as *mut bot_replychat_t;
    //go through all the reply chats
    rchat = replychats; //end for
    while !rchat.is_null() {
        found = crate::src::qcommon::q_shared::qfalse as i32;
        //end if
        key = (*rchat).keys; //end for
        while !key.is_null() {
            res = crate::src::qcommon::q_shared::qfalse as i32;
            //end else
            //get the match result
            if (*key).flags & 4 as i32 != 0 {
                res = (StringContains(
                    message,
                    (*cs).name.as_mut_ptr(),
                    crate::src::qcommon::q_shared::qfalse as i32,
                ) != -(1 as i32)) as i32
            } else if (*key).flags & 32 as i32 != 0 {
                res = (StringContains(
                    (*key).string,
                    (*cs).name.as_mut_ptr(),
                    crate::src::qcommon::q_shared::qfalse as i32,
                ) != -(1 as i32)) as i32
            } else if (*key).flags & 64 as i32 != 0 {
                res = ((*cs).gender == 1 as i32) as i32
            } else if (*key).flags & 128 as i32 != 0 {
                res = ((*cs).gender == 2 as i32) as i32
            } else if (*key).flags & 256 as i32 != 0 {
                res = ((*cs).gender == 0 as i32) as i32
            } else if (*key).flags & 16 as i32 != 0 {
                res = StringsMatch((*key).match_0, &mut match_0)
            } else if (*key).flags & 8 as i32 != 0 {
                res = (StringContainsWord(
                    message,
                    (*key).string,
                    crate::src::qcommon::q_shared::qfalse as i32,
                ) != 0 as *mut libc::c_void as *mut libc::c_char) as i32
            }
            //if the key must be present
            if (*key).flags & 1 as i32 != 0 {
                if res == 0 {
                    found = crate::src::qcommon::q_shared::qfalse as i32;
                    break;
                }
            //end if
            } else if (*key).flags & 2 as i32 != 0 {
                if res != 0 {
                    found = crate::src::qcommon::q_shared::qfalse as i32;
                    break;
                }
            //if the key must be absent
            //end if
            } else if res != 0 {
                found = crate::src::qcommon::q_shared::qtrue as i32
            }
            key = (*key).next
        }
        if found != 0 {
            if (*rchat).priority > bestpriority as f32 {
                numchatmessages = 0 as i32;
                //
                //end if
                m = (*rchat).firstchatmessage; //end if
                while !m.is_null() {
                    if !((*m).time > crate::src::botlib::be_aas_main::AAS_Time()) {
                        numchatmessages += 1
                    } //end for
                    m = (*m).next
                }
                num = ((::libc::rand() & 0x7fff as i32) as f32 / 0x7fff as i32 as f32
                    * numchatmessages as f32) as i32;
                m = (*rchat).firstchatmessage;
                while !m.is_null() {
                    num -= 1;
                    if num < 0 as i32 {
                        break;
                    }
                    // check message timestamp
                    m = (*m).next
                }
                if !m.is_null() {
                    crate::stdlib::memcpy(
                        &mut bestmatch as *mut crate::src::botlib::be_ai_chat::bot_match_t
                            as *mut libc::c_void,
                        &mut match_0 as *mut crate::src::botlib::be_ai_chat::bot_match_t
                            as *const libc::c_void,
                        ::std::mem::size_of::<crate::src::botlib::be_ai_chat::bot_match_t>()
                            as libc::c_ulong,
                    );
                    bestchatmessage = m;
                    bestrchat = rchat;
                    bestpriority = (*rchat).priority as i32
                }
            }
            //if the reply chat has a message
            //end if
        } //end if
        rchat = (*rchat).next
    } //end else
    if !bestchatmessage.is_null() {
        index = crate::stdlib::strlen(bestmatch.string.as_mut_ptr()) as i32;
        if !var0.is_null() {
            ::libc::strcat(bestmatch.string.as_mut_ptr(), var0);
            bestmatch.variables[0 as i32 as usize].offset = index as libc::c_char;
            bestmatch.variables[0 as i32 as usize].length = crate::stdlib::strlen(var0) as i32;
            index = (index as libc::c_ulong).wrapping_add(crate::stdlib::strlen(var0)) as i32 as i32
        }
        if !var1.is_null() {
            ::libc::strcat(bestmatch.string.as_mut_ptr(), var1);
            bestmatch.variables[1 as i32 as usize].offset = index as libc::c_char;
            bestmatch.variables[1 as i32 as usize].length = crate::stdlib::strlen(var1) as i32;
            index = (index as libc::c_ulong).wrapping_add(crate::stdlib::strlen(var1)) as i32 as i32
        }
        if !var2.is_null() {
            ::libc::strcat(bestmatch.string.as_mut_ptr(), var2);
            bestmatch.variables[2 as i32 as usize].offset = index as libc::c_char;
            bestmatch.variables[2 as i32 as usize].length = crate::stdlib::strlen(var2) as i32;
            index = (index as libc::c_ulong).wrapping_add(crate::stdlib::strlen(var2)) as i32 as i32
        }
        if !var3.is_null() {
            ::libc::strcat(bestmatch.string.as_mut_ptr(), var3);
            bestmatch.variables[3 as i32 as usize].offset = index as libc::c_char;
            bestmatch.variables[3 as i32 as usize].length = crate::stdlib::strlen(var3) as i32;
            index = (index as libc::c_ulong).wrapping_add(crate::stdlib::strlen(var3)) as i32 as i32
        }
        if !var4.is_null() {
            ::libc::strcat(bestmatch.string.as_mut_ptr(), var4);
            bestmatch.variables[4 as i32 as usize].offset = index as libc::c_char;
            bestmatch.variables[4 as i32 as usize].length = crate::stdlib::strlen(var4) as i32;
            index = (index as libc::c_ulong).wrapping_add(crate::stdlib::strlen(var4)) as i32 as i32
        }
        if !var5.is_null() {
            ::libc::strcat(bestmatch.string.as_mut_ptr(), var5);
            bestmatch.variables[5 as i32 as usize].offset = index as libc::c_char;
            bestmatch.variables[5 as i32 as usize].length = crate::stdlib::strlen(var5) as i32;
            index = (index as libc::c_ulong).wrapping_add(crate::stdlib::strlen(var5)) as i32 as i32
        }
        if !var6.is_null() {
            ::libc::strcat(bestmatch.string.as_mut_ptr(), var6);
            bestmatch.variables[6 as i32 as usize].offset = index as libc::c_char;
            bestmatch.variables[6 as i32 as usize].length = crate::stdlib::strlen(var6) as i32;
            index = (index as libc::c_ulong).wrapping_add(crate::stdlib::strlen(var6)) as i32 as i32
        }
        if !var7.is_null() {
            ::libc::strcat(bestmatch.string.as_mut_ptr(), var7);
            bestmatch.variables[7 as i32 as usize].offset = index as libc::c_char;
            bestmatch.variables[7 as i32 as usize].length = crate::stdlib::strlen(var7) as i32
        }
        if crate::src::botlib::l_libvar::LibVarGetValue(
            b"bot_testrchat\x00" as *const u8 as *const libc::c_char,
        ) != 0.
        {
            m = (*bestrchat).firstchatmessage;
            while !m.is_null() {
                BotConstructChatMessage(
                    cs,
                    (*m).chatmessage,
                    mcontext as libc::c_ulong,
                    &mut bestmatch,
                    vcontext as libc::c_ulong,
                    crate::src::qcommon::q_shared::qtrue as i32,
                );
                BotRemoveTildes((*cs).chatmessage.as_mut_ptr());
                crate::src::botlib::be_interface::botimport
                    .Print
                    .expect("non-null function pointer")(
                    1 as i32,
                    b"%s\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    (*cs).chatmessage.as_mut_ptr(),
                );
                m = (*m).next
            }
        //end if
        } else {
            (*bestchatmessage).time =
                crate::src::botlib::be_aas_main::AAS_Time() + 20 as i32 as f32;
            BotConstructChatMessage(
                cs,
                (*bestchatmessage).chatmessage,
                mcontext as libc::c_ulong,
                &mut bestmatch,
                vcontext as libc::c_ulong,
                crate::src::qcommon::q_shared::qtrue as i32,
            );
        }
        return crate::src::qcommon::q_shared::qtrue as i32;
    }
    return crate::src::qcommon::q_shared::qfalse as i32;
}
//returns the length of the currently selected chat message
//end of the function BotReplyChat
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotChatLength(mut chatstate: i32) -> i32 {
    let mut cs: *mut bot_chatstate_t = 0 as *mut bot_chatstate_t;
    cs = BotChatStateFromHandle(chatstate);
    if cs.is_null() {
        return 0 as i32;
    }
    return crate::stdlib::strlen((*cs).chatmessage.as_mut_ptr()) as i32;
}
//enters the selected chat message
//end of the function BotChatLength
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotEnterChat(mut chatstate: i32, mut clientto: i32, mut sendto: i32) {
    let mut cs: *mut bot_chatstate_t = 0 as *mut bot_chatstate_t;
    cs = BotChatStateFromHandle(chatstate);
    if cs.is_null() {
        return;
    }
    if crate::stdlib::strlen((*cs).chatmessage.as_mut_ptr()) != 0 {
        BotRemoveTildes((*cs).chatmessage.as_mut_ptr());
        if crate::src::botlib::l_libvar::LibVarGetValue(
            b"bot_testichat\x00" as *const u8 as *const libc::c_char,
        ) != 0.
        {
            crate::src::botlib::be_interface::botimport
                .Print
                .expect("non-null function pointer")(
                1 as i32,
                b"%s\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                (*cs).chatmessage.as_mut_ptr(),
            );
        } else {
            match sendto {
                1 => {
                    crate::src::botlib::be_ea::EA_Command(
                        (*cs).client,
                        crate::src::qcommon::q_shared::va(
                            b"say_team %s\x00" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                            (*cs).chatmessage.as_mut_ptr(),
                        ),
                    );
                }
                2 => {
                    crate::src::botlib::be_ea::EA_Command(
                        (*cs).client,
                        crate::src::qcommon::q_shared::va(
                            b"tell %d %s\x00" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                            clientto,
                            (*cs).chatmessage.as_mut_ptr(),
                        ),
                    );
                }
                _ => {
                    //CHAT_ALL
                    crate::src::botlib::be_ea::EA_Command(
                        (*cs).client,
                        crate::src::qcommon::q_shared::va(
                            b"say %s\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                            (*cs).chatmessage.as_mut_ptr(),
                        ),
                    );
                }
            }
        }
        //clear the chat message from the state
        ::libc::strcpy(
            (*cs).chatmessage.as_mut_ptr(),
            b"\x00" as *const u8 as *const libc::c_char,
        );
    };
    //end if
}
//get the chat message ready to be output
//end of the function BotEnterChat
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotGetChatMessage(
    mut chatstate: i32,
    mut buf: *mut libc::c_char,
    mut size: i32,
) {
    let mut cs: *mut bot_chatstate_t = 0 as *mut bot_chatstate_t;
    cs = BotChatStateFromHandle(chatstate);
    if cs.is_null() {
        return;
    }
    BotRemoveTildes((*cs).chatmessage.as_mut_ptr());
    crate::stdlib::strncpy(
        buf,
        (*cs).chatmessage.as_mut_ptr(),
        (size - 1 as i32) as libc::c_ulong,
    );
    *buf.offset((size - 1 as i32) as isize) = '\u{0}' as i32 as libc::c_char;
    //clear the chat message from the state
    ::libc::strcpy(
        (*cs).chatmessage.as_mut_ptr(),
        b"\x00" as *const u8 as *const libc::c_char,
    );
}
//store the gender of the bot in the chat state
//end of the function BotGetChatMessage
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotSetChatGender(mut chatstate: i32, mut gender: i32) {
    let mut cs: *mut bot_chatstate_t = 0 as *mut bot_chatstate_t;
    cs = BotChatStateFromHandle(chatstate);
    if cs.is_null() {
        return;
    }
    match gender {
        1 => (*cs).gender = 1 as i32,
        2 => (*cs).gender = 2 as i32,
        _ => (*cs).gender = 0 as i32,
    };
    //end switch
}
//store the bot name in the chat state
//end of the function BotSetChatGender
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotSetChatName(
    mut chatstate: i32,
    mut name: *mut libc::c_char,
    mut client: i32,
) {
    let mut cs: *mut bot_chatstate_t = 0 as *mut bot_chatstate_t;
    cs = BotChatStateFromHandle(chatstate);
    if cs.is_null() {
        return;
    }
    (*cs).client = client;
    crate::stdlib::memset(
        (*cs).name.as_mut_ptr() as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
    );
    crate::stdlib::strncpy(
        (*cs).name.as_mut_ptr(),
        name,
        (::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong)
            .wrapping_sub(1 as i32 as libc::c_ulong),
    );
    (*cs).name[(::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong)
        .wrapping_sub(1 as i32 as libc::c_ulong) as usize] = '\u{0}' as i32 as libc::c_char;
}
//end of the function BotSetChatName
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotResetChatAI() {
    let mut rchat: *mut bot_replychat_t = 0 as *mut bot_replychat_t;
    let mut m: *mut bot_chatmessage_t = 0 as *mut bot_chatmessage_t;
    rchat = replychats;
    while !rchat.is_null() {
        m = (*rchat).firstchatmessage;
        while !m.is_null() {
            (*m).time = 0 as i32 as f32;
            m = (*m).next
        }
        rchat = (*rchat).next
        //end for
    }
    //end for
}
//returns the handle to a newly allocated chat state
//end of the function BotResetChatAI
//========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotAllocChatState() -> i32 {
    let mut i: i32 = 0; //end for
    i = 1 as i32;
    while i <= 64 as i32 {
        if botchatstates[i as usize].is_null() {
            botchatstates[i as usize] = crate::src::botlib::l_memory::GetClearedMemory(
                ::std::mem::size_of::<bot_chatstate_t>() as libc::c_ulong,
            ) as *mut bot_chatstate_t;
            return i;
        }
        i += 1
        //end if
    }
    return 0 as i32;
}
//frees the chatstate
//end of the function BotAllocChatState
//========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotFreeChatState(mut handle: i32) {
    let mut m: crate::src::botlib::be_ai_chat::bot_consolemessage_t =
        crate::src::botlib::be_ai_chat::bot_consolemessage_t {
            handle: 0,
            time: 0.,
            type_0: 0,
            message: [0; 256],
            prev: 0 as *mut crate::src::botlib::be_ai_chat::bot_consolemessage_s,
            next: 0 as *mut crate::src::botlib::be_ai_chat::bot_consolemessage_s,
        }; //end if
    let mut h: i32 = 0; //end if
    if handle <= 0 as i32 || handle > 64 as i32 {
        crate::src::botlib::be_interface::botimport
            .Print
            .expect("non-null function pointer")(
            4 as i32,
            b"chat state handle %d out of range\n\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            handle,
        ); //end if
        return;
    }
    if botchatstates[handle as usize].is_null() {
        crate::src::botlib::be_interface::botimport
            .Print
            .expect("non-null function pointer")(
            4 as i32,
            b"invalid chat state %d\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            handle,
        );
        return;
    }
    if crate::src::botlib::l_libvar::LibVarGetValue(
        b"bot_reloadcharacters\x00" as *const u8 as *const libc::c_char,
    ) != 0.
    {
        BotFreeChatFile(handle);
    }
    //free all the console messages left in the chat state
    h = BotNextConsoleMessage(handle, &mut m); //end for
    while h != 0 {
        //remove the console message
        BotRemoveConsoleMessage(handle, h);
        h = BotNextConsoleMessage(handle, &mut m)
    }
    crate::src::botlib::l_memory::FreeMemory(botchatstates[handle as usize] as *mut libc::c_void);
    botchatstates[handle as usize] = 0 as *mut bot_chatstate_t;
}
//setup the chat AI
//end of the function BotFreeChatState
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotSetupChatAI() -> i32 {
    let mut file: *mut libc::c_char = 0 as *mut libc::c_char;
    //DEBUG
    file = crate::src::botlib::l_libvar::LibVarString(
        b"synfile\x00" as *const u8 as *const libc::c_char,
        b"syn.c\x00" as *const u8 as *const libc::c_char,
    );
    synonyms = BotLoadSynonyms(file);
    file = crate::src::botlib::l_libvar::LibVarString(
        b"rndfile\x00" as *const u8 as *const libc::c_char,
        b"rnd.c\x00" as *const u8 as *const libc::c_char,
    );
    randomstrings = BotLoadRandomStrings(file);
    file = crate::src::botlib::l_libvar::LibVarString(
        b"matchfile\x00" as *const u8 as *const libc::c_char,
        b"match.c\x00" as *const u8 as *const libc::c_char,
    );
    matchtemplates = BotLoadMatchTemplates(file);
    //
    if crate::src::botlib::l_libvar::LibVarValue(
        b"nochat\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
    ) == 0.
    {
        file = crate::src::botlib::l_libvar::LibVarString(
            b"rchatfile\x00" as *const u8 as *const libc::c_char,
            b"rchat.c\x00" as *const u8 as *const libc::c_char,
        ); //end if
        replychats = BotLoadReplyChat(file)
    }
    InitConsoleMessageHeap();
    //DEBUG
    return 0 as i32;
}
//shutdown the chat AI
//end of the function BotSetupChatAI
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotShutdownChatAI() {
    let mut i: i32 = 0;
    //free all remaining chat states
    i = 0 as i32; //end for
    while i < 64 as i32 {
        if !botchatstates[i as usize].is_null() {
            BotFreeChatState(i);
        }
        i += 1
        //end if
    }
    //free all cached chats
    i = 0 as i32; //end for
    while i < 64 as i32 {
        if !ichatdata[i as usize].is_null() {
            crate::src::botlib::l_memory::FreeMemory(
                (*ichatdata[i as usize]).chat as *mut libc::c_void,
            );
            crate::src::botlib::l_memory::FreeMemory(ichatdata[i as usize] as *mut libc::c_void);
            ichatdata[i as usize] = 0 as *mut bot_ichatdata_t
        }
        i += 1
        //end if
    }
    if !consolemessageheap.is_null() {
        crate::src::botlib::l_memory::FreeMemory(consolemessageheap as *mut libc::c_void);
    }
    consolemessageheap = 0 as *mut crate::src::botlib::be_ai_chat::bot_consolemessage_t;
    if !matchtemplates.is_null() {
        BotFreeMatchTemplates(matchtemplates);
    }
    matchtemplates = 0 as *mut bot_matchtemplate_t;
    if !randomstrings.is_null() {
        crate::src::botlib::l_memory::FreeMemory(randomstrings as *mut libc::c_void);
    }
    randomstrings = 0 as *mut bot_randomlist_t;
    if !synonyms.is_null() {
        crate::src::botlib::l_memory::FreeMemory(synonyms as *mut libc::c_void);
    }
    synonyms = 0 as *mut bot_synonymlist_t;
    if !replychats.is_null() {
        BotFreeReplyChat(replychats);
    }
    replychats = 0 as *mut bot_replychat_t;
}
//end of the function BotShutdownChatAI

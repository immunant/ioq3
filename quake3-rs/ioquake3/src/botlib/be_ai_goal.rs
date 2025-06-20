// =============== BEGIN be_ai_goal_h ================
pub type bot_goal_t = crate::src::botlib::be_ai_goal::bot_goal_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bot_goal_s {
    pub origin: crate::src::qcommon::q_shared::vec3_t,
    pub areanum: i32,
    pub mins: crate::src::qcommon::q_shared::vec3_t,
    pub maxs: crate::src::qcommon::q_shared::vec3_t,
    pub entitynum: i32,
    pub number: i32,
    pub flags: i32,
    pub iteminfo: i32,
}
use ::libc;

pub mod q_shared_h {

    #[inline]

    pub unsafe extern "C" fn VectorLength(
        mut v: *const crate::src::qcommon::q_shared::vec_t,
    ) -> crate::src::qcommon::q_shared::vec_t {
        return crate::stdlib::sqrt(
            (*v.offset(0 as i32 as isize) * *v.offset(0 as i32 as isize)
                + *v.offset(1 as i32 as isize) * *v.offset(1 as i32 as isize)
                + *v.offset(2 as i32 as isize) * *v.offset(2 as i32 as isize)) as f64,
        ) as crate::src::qcommon::q_shared::vec_t;
    }

    // __Q_SHARED_H
}

pub use crate::stddef_h::size_t;

pub use crate::be_aas_h::aas_entityinfo_s;
pub use crate::be_aas_h::aas_entityinfo_t;
pub use crate::botlib_h::botlib_import_s;
pub use crate::botlib_h::botlib_import_t;
pub use crate::botlib_h::bsp_surface_s;
pub use crate::botlib_h::bsp_surface_t;
pub use crate::botlib_h::bsp_trace_s;
pub use crate::botlib_h::bsp_trace_t;
pub use crate::src::botlib::be_ai_goal::q_shared_h::VectorLength;
pub use crate::src::botlib::be_ai_weight::fuzzyseperator_s;
pub use crate::src::botlib::be_ai_weight::weight_s;
pub use crate::src::botlib::be_ai_weight::weight_t;
pub use crate::src::botlib::be_ai_weight::weightconfig_s;
pub use crate::src::botlib::be_ai_weight::weightconfig_t;
pub use crate::src::botlib::be_ai_weight::EvolveWeightConfig;
pub use crate::src::botlib::be_ai_weight::FindFuzzyWeight;
pub use crate::src::botlib::be_ai_weight::FreeWeightConfig;
pub use crate::src::botlib::be_ai_weight::FuzzyWeightUndecided;
pub use crate::src::botlib::be_ai_weight::InterbreedWeightConfigs;
pub use crate::src::botlib::be_ai_weight::ReadWeightConfig;
pub use crate::src::botlib::l_libvar::libvar_s;
pub use crate::src::botlib::l_libvar::libvar_t;
pub use crate::src::botlib::l_libvar::LibVar;
pub use crate::src::botlib::l_libvar::LibVarSet;
pub use crate::src::botlib::l_libvar::LibVarString;
pub use crate::src::botlib::l_libvar::LibVarValue;

pub use crate::src::botlib::l_precomp::define_s;
pub use crate::src::botlib::l_precomp::define_t;
pub use crate::src::botlib::l_precomp::indent_s;
pub use crate::src::botlib::l_precomp::indent_t;
pub use crate::src::botlib::l_precomp::source_s;
pub use crate::src::botlib::l_precomp::source_t;
pub use crate::src::botlib::l_precomp::FreeSource;
pub use crate::src::botlib::l_precomp::LoadSourceFile;
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
pub use crate::src::botlib::l_struct::fielddef_s;
pub use crate::src::botlib::l_struct::fielddef_t;
pub use crate::src::botlib::l_struct::structdef_s;
pub use crate::src::botlib::l_struct::structdef_t;
pub use crate::src::botlib::l_struct::ReadStructure;
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
pub use crate::src::qcommon::q_shared::Q_stricmp;
pub use crate::src::qcommon::q_shared::Q_strncpyz;
pub use crate::src::qcommon::q_shared::FS_APPEND;
pub use crate::src::qcommon::q_shared::FS_APPEND_SYNC;
pub use crate::src::qcommon::q_shared::FS_READ;
pub use crate::src::qcommon::q_shared::FS_WRITE;

//goal state

pub type bot_goalstate_t = bot_goalstate_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bot_goalstate_s {
    pub itemweightconfig: *mut weightconfig_s,
    pub itemweightindex: *mut i32,
    pub client: i32,
    pub lastreachabilityarea: i32,
    pub goalstack: [crate::src::botlib::be_ai_goal::bot_goal_t; 8],
    pub goalstacktop: i32,
    pub avoidgoals: [i32; 256],
    pub avoidgoaltimes: [f32; 256],
}

pub type itemconfig_t = itemconfig_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct itemconfig_s {
    pub numiteminfo: i32,
    pub iteminfo: *mut iteminfo_t,
}

pub type iteminfo_t = iteminfo_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iteminfo_s {
    pub classname: [libc::c_char; 32],
    pub name: [libc::c_char; 80],
    pub model: [libc::c_char; 80],
    pub modelindex: i32,
    pub type_0: i32,
    pub index: i32,
    pub respawntime: f32,
    pub mins: vec3_t,
    pub maxs: vec3_t,
    pub number: i32,
}
//weight config
//index from item to weight
//
//client using this goal state
//last area with reachabilities the bot was in
//
//goal stack
//the top of the goal stack
//
//goals to avoid
//times to avoid the goals
//classname of the item
//name of the item
//model of the item
//model index
//item type
//index in the inventory
//respawn time
//mins of the item
//maxs of the item
//number of the item info
//bot roam goal
//location in the map "target_location"

pub type maplocation_t = maplocation_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct maplocation_s {
    pub origin: vec3_t,
    pub areanum: i32,
    pub name: [libc::c_char; 128],
    pub next: *mut maplocation_s,
}

pub type levelitem_t = levelitem_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct levelitem_s {
    pub number: i32,
    pub iteminfo: i32,
    pub flags: i32,
    pub weight: f32,
    pub origin: vec3_t,
    pub goalareanum: i32,
    pub goalorigin: vec3_t,
    pub entitynum: i32,
    pub timeout: f32,
    pub prev: *mut levelitem_s,
    pub next: *mut levelitem_s,
}
//number of the level item
//index into the item info
//item flags
//fixed roam weight
//origin of the item
//area the item is in
//goal origin within the area
//entity number
//item is removed after this time
//camp spots "info_camp"

pub type campspot_t = campspot_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct campspot_s {
    pub origin: vec3_t,
    pub areanum: i32,
    pub name: [libc::c_char; 128],
    pub range: f32,
    pub weight: f32,
    pub wait: f32,
    pub random: f32,
    pub next: *mut campspot_s,
}

pub const GT_TEAM: C2RustUnnamed_3 = 3;

pub const GT_SINGLE_PLAYER: C2RustUnnamed_3 = 2;

pub type C2RustUnnamed_3 = u32;

pub const GT_MAX_GAME_TYPE: C2RustUnnamed_3 = 5;

pub const GT_CTF: C2RustUnnamed_3 = 4;

pub const GT_TOURNAMENT: C2RustUnnamed_3 = 1;

pub const GT_FFA: C2RustUnnamed_3 = 0;
// Initialized in run_static_initializers
#[no_mangle]

pub static mut iteminfo_fields: [fielddef_t; 9] = [fielddef_t {
    name: std::ptr::null_mut(),
    offset: 0,
    type_0: 0,
    maxarray: 0,
    floatmin: 0.,
    floatmax: 0.,
    substruct: std::ptr::null_mut(),
}; 9];
#[no_mangle]

pub static mut iteminfo_struct: structdef_t = unsafe {
    {
        let mut init = structdef_s {
            size: ::std::mem::size_of::<iteminfo_t>() as usize as i32,
            fields: iteminfo_fields.as_ptr() as *mut _,
        };
        init
    }
};
#[no_mangle]

pub static mut botgoalstates: [*mut bot_goalstate_t; 65] = [std::ptr::null_mut(); 65];
// FIXME: init?
//item configuration
#[no_mangle]

pub static mut itemconfig: *mut itemconfig_t = std::ptr::null_mut();
//level items
#[no_mangle]

pub static mut levelitemheap: *mut levelitem_t = std::ptr::null_mut();
#[no_mangle]

pub static mut freelevelitems: *mut levelitem_t = std::ptr::null_mut();
#[no_mangle]

pub static mut levelitems: *mut levelitem_t = std::ptr::null_mut();
#[no_mangle]

pub static mut numlevelitems: i32 = 0 as i32;
//map locations
#[no_mangle]

pub static mut maplocations: *mut maplocation_t = std::ptr::null_mut();
//camp spots
#[no_mangle]

pub static mut campspots: *mut campspot_t = std::ptr::null_mut();
//the game type
#[no_mangle]

pub static mut g_gametype: i32 = 0 as i32;
//additional dropped item weight
#[no_mangle]

pub static mut droppedweight: *mut libvar_t = std::ptr::null_mut();
//========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotGoalStateFromHandle(mut handle: i32) -> *mut bot_goalstate_t {
    if handle <= 0 as i32 || handle > 64 as i32 {
        crate::src::botlib::be_interface::botimport
            .Print
            .expect("non-null function pointer")(
            4 as i32,
            b"goal state handle %d out of range\n\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            handle,
        ); //end if
        return std::ptr::null_mut();
    } //end if
    if botgoalstates[handle as usize].is_null() {
        crate::src::botlib::be_interface::botimport
            .Print
            .expect("non-null function pointer")(
            4 as i32,
            b"invalid goal state %d\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            handle,
        );
        return std::ptr::null_mut();
    }
    return botgoalstates[handle as usize];
}
//interbreed the goal fuzzy logic
//end of the function BotGoalStateFromHandle
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotInterbreedGoalFuzzyLogic(
    mut parent1: i32,
    mut parent2: i32,
    mut child: i32,
) {
    let mut p1: *mut bot_goalstate_t = std::ptr::null_mut();
    let mut p2: *mut bot_goalstate_t = std::ptr::null_mut();
    let mut c: *mut bot_goalstate_t = std::ptr::null_mut();
    p1 = BotGoalStateFromHandle(parent1);
    p2 = BotGoalStateFromHandle(parent2);
    c = BotGoalStateFromHandle(child);
    if p1.is_null() || p2.is_null() || c.is_null() {
        return;
    }
    InterbreedWeightConfigs(
        (*p1).itemweightconfig as *mut weightconfig_s,
        (*p2).itemweightconfig as *mut weightconfig_s,
        (*c).itemweightconfig as *mut weightconfig_s,
    );
}
//save the goal fuzzy logic to disk
//end of the function BotInterbreedingGoalFuzzyLogic
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotSaveGoalFuzzyLogic(
    mut _goalstate: i32,
    mut _filename: *mut libc::c_char,
) {
    //bot_goalstate_t *gs;
    //gs = BotGoalStateFromHandle(goalstate);
    //if (!gs) return;
    //WriteWeightConfig(filename, gs->itemweightconfig);
}
//mutate the goal fuzzy logic
//end of the function BotSaveGoalFuzzyLogic
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotMutateGoalFuzzyLogic(mut goalstate: i32, mut _range: f32) {
    let mut gs: *mut bot_goalstate_t = std::ptr::null_mut();
    gs = BotGoalStateFromHandle(goalstate);
    if gs.is_null() {
        return;
    }
    EvolveWeightConfig((*gs).itemweightconfig as *mut weightconfig_s);
}
//end of the function BotMutateGoalFuzzyLogic
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn LoadItemConfig(mut filename: *mut libc::c_char) -> *mut itemconfig_t {
    let mut max_iteminfo: i32 = 0; //end if
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
    let mut path: [libc::c_char; 64] = [0; 64];
    let mut source: *mut source_t = std::ptr::null_mut();
    let mut ic: *mut itemconfig_t = std::ptr::null_mut();
    let mut ii: *mut iteminfo_t = std::ptr::null_mut();
    max_iteminfo = LibVarValue(
        b"max_iteminfo\x00" as *const u8 as *const libc::c_char,
        b"256\x00" as *const u8 as *const libc::c_char,
    ) as i32;
    if max_iteminfo < 0 as i32 {
        crate::src::botlib::be_interface::botimport
            .Print
            .expect("non-null function pointer")(
            3 as i32,
            b"max_iteminfo = %d\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            max_iteminfo,
        );
        max_iteminfo = 256 as i32;
        LibVarSet(
            b"max_iteminfo\x00" as *const u8 as *const libc::c_char,
            b"256\x00" as *const u8 as *const libc::c_char,
        );
    }
    Q_strncpyz(
        path.as_mut_ptr(),
        filename,
        ::std::mem::size_of::<[libc::c_char; 64]>() as usize as i32,
    );
    PC_SetBaseFolder(b"botfiles\x00" as *const u8 as *const libc::c_char as *mut libc::c_char);
    source = LoadSourceFile(path.as_mut_ptr()) as *mut source_s;
    if source.is_null() {
        crate::src::botlib::be_interface::botimport
            .Print
            .expect("non-null function pointer")(
            3 as i32,
            b"counldn\'t load %s\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            path.as_mut_ptr(),
        );
        return std::ptr::null_mut();
    }
    //initialize item config
    ic = crate::src::botlib::l_memory::GetClearedHunkMemory(
        (::std::mem::size_of::<itemconfig_t>() as usize).wrapping_add(
            (max_iteminfo as usize).wrapping_mul(::std::mem::size_of::<iteminfo_t>() as usize),
        ),
    ) as *mut itemconfig_t;
    (*ic).iteminfo = (ic as *mut libc::c_char)
        .offset(::std::mem::size_of::<itemconfig_t>() as usize as isize)
        as *mut iteminfo_t;
    (*ic).numiteminfo = 0 as i32;
    //parse the item config file
    while PC_ReadToken(
        source as *mut source_s,
        &mut token as *mut _ as *mut token_s,
    ) != 0
    {
        if libc::strcmp(
            token.string.as_mut_ptr(),
            b"iteminfo\x00" as *const u8 as *const libc::c_char,
        ) == 0
        {
            //end while
            if (*ic).numiteminfo >= max_iteminfo {
                SourceError(
                    source as *mut source_s,
                    b"more than %d item info defined\x00" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    max_iteminfo,
                ); //end if
                crate::src::botlib::l_memory::FreeMemory(ic as *mut libc::c_void); //end if
                FreeSource(source as *mut source_s); //end if
                return std::ptr::null_mut();
            } //end if
            ii = &mut *(*ic).iteminfo.offset((*ic).numiteminfo as isize) as *mut iteminfo_t;
            crate::stdlib::memset(
                ii as *mut libc::c_void,
                0 as i32,
                ::std::mem::size_of::<iteminfo_t>() as usize,
            );
            if PC_ExpectTokenType(
                source as *mut source_s,
                1 as i32,
                0 as i32,
                &mut token as *mut _ as *mut token_s,
            ) == 0
            {
                crate::src::botlib::l_memory::FreeMemory(ic as *mut libc::c_void);
                FreeSource(source as *mut source_s);
                return std::ptr::null_mut();
            }
            StripDoubleQuotes(token.string.as_mut_ptr());
            Q_strncpyz(
                (*ii).classname.as_mut_ptr(),
                token.string.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 32]>() as usize as i32,
            );
            if ReadStructure(
                source as *mut source_s,
                &mut iteminfo_struct as *mut _ as *mut structdef_s,
                ii as *mut libc::c_char,
            ) == 0
            {
                crate::src::botlib::l_memory::FreeMemory(ic as *mut libc::c_void);
                FreeSource(source as *mut source_s);
                return std::ptr::null_mut();
            }
            (*ii).number = (*ic).numiteminfo;
            (*ic).numiteminfo += 1
        } else {
            SourceError(
                source as *mut source_s,
                b"unknown definition %s\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                token.string.as_mut_ptr(),
            );
            crate::src::botlib::l_memory::FreeMemory(ic as *mut libc::c_void);
            FreeSource(source as *mut source_s);
            return std::ptr::null_mut();
        }
        //end else
    }
    FreeSource(source as *mut source_s);
    //
    if (*ic).numiteminfo == 0 {
        crate::src::botlib::be_interface::botimport
            .Print
            .expect("non-null function pointer")(
            2 as i32,
            b"no item info loaded\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    crate::src::botlib::be_interface::botimport
        .Print
        .expect("non-null function pointer")(
        1 as i32,
        b"loaded %s\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        path.as_mut_ptr(),
    );
    return ic;
}
//end of the function LoadItemConfig
//===========================================================================
// index to find the weight function of an iteminfo
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn ItemWeightIndex(
    mut iwc: *mut weightconfig_t,
    mut ic: *mut itemconfig_t,
) -> *mut i32 {
    let mut index: *mut i32 = std::ptr::null_mut();
    let mut i: i32 = 0;
    //initialize item weight index
    index = crate::src::botlib::l_memory::GetClearedMemory(
        (::std::mem::size_of::<i32>() as usize).wrapping_mul((*ic).numiteminfo as usize),
    ) as *mut i32; //end for
    i = 0 as i32;
    while i < (*ic).numiteminfo {
        *index.offset(i as isize) = FindFuzzyWeight(
            iwc as *mut weightconfig_s,
            (*(*ic).iteminfo.offset(i as isize)).classname.as_mut_ptr(),
        );
        if *index.offset(i as isize) < 0 as i32 {
            crate::src::botlib::l_log::Log_Write(
                b"item info %d \"%s\" has no fuzzy weight\r\n\x00" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                i,
                (*(*ic).iteminfo.offset(i as isize)).classname.as_mut_ptr(),
            );
        }
        i += 1
        //end if
    }
    return index;
}
//end of the function ItemWeightIndex
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn InitLevelItemHeap() {
    let mut i: i32 = 0; //end for
    let mut max_levelitems: i32 = 0;
    if !levelitemheap.is_null() {
        crate::src::botlib::l_memory::FreeMemory(levelitemheap as *mut libc::c_void);
    }
    max_levelitems = LibVarValue(
        b"max_levelitems\x00" as *const u8 as *const libc::c_char,
        b"256\x00" as *const u8 as *const libc::c_char,
    ) as i32;
    levelitemheap = crate::src::botlib::l_memory::GetClearedMemory(
        (max_levelitems as usize).wrapping_mul(::std::mem::size_of::<levelitem_t>() as usize),
    ) as *mut levelitem_t;
    i = 0 as i32;
    while i < max_levelitems - 1 as i32 {
        let ref mut fresh0 = (*levelitemheap.offset(i as isize)).next;
        *fresh0 = &mut *levelitemheap.offset((i + 1 as i32) as isize) as *mut levelitem_t;
        i += 1
    }
    let ref mut fresh1 = (*levelitemheap.offset((max_levelitems - 1 as i32) as isize)).next;
    *fresh1 = std::ptr::null_mut();
    //
    freelevelitems = levelitemheap;
}
//end of the function InitLevelItemHeap
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AllocLevelItem() -> *mut levelitem_t {
    let mut li: *mut levelitem_t = std::ptr::null_mut(); //end if
    li = freelevelitems;
    if li.is_null() {
        crate::src::botlib::be_interface::botimport
            .Print
            .expect("non-null function pointer")(
            4 as i32,
            b"out of level items\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return std::ptr::null_mut();
    }
    //
    freelevelitems = (*freelevelitems).next;
    crate::stdlib::memset(
        li as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<levelitem_t>() as usize,
    );
    return li;
}
//end of the function AllocLevelItem
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn FreeLevelItem(mut li: *mut levelitem_t) {
    (*li).next = freelevelitems;
    freelevelitems = li;
}
//end of the function FreeLevelItem
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AddLevelItemToList(mut li: *mut levelitem_t) {
    if !levelitems.is_null() {
        (*levelitems).prev = li
    }
    (*li).prev = std::ptr::null_mut();
    (*li).next = levelitems;
    levelitems = li;
}
//end of the function AddLevelItemToList
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn RemoveLevelItemFromList(mut li: *mut levelitem_t) {
    if !(*li).prev.is_null() {
        (*(*li).prev).next = (*li).next
    } else {
        levelitems = (*li).next
    }
    if !(*li).next.is_null() {
        (*(*li).next).prev = (*li).prev
    };
}
//end of the function RemoveLevelItemFromList
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotFreeInfoEntities() {
    let mut ml: *mut maplocation_t = std::ptr::null_mut(); //end for
    let mut nextml: *mut maplocation_t = std::ptr::null_mut(); //end for
    let mut cs: *mut campspot_t = std::ptr::null_mut();
    let mut nextcs: *mut campspot_t = std::ptr::null_mut();
    ml = maplocations;
    while !ml.is_null() {
        nextml = (*ml).next;
        crate::src::botlib::l_memory::FreeMemory(ml as *mut libc::c_void);
        ml = nextml
    }
    maplocations = std::ptr::null_mut();
    cs = campspots;
    while !cs.is_null() {
        nextcs = (*cs).next;
        crate::src::botlib::l_memory::FreeMemory(cs as *mut libc::c_void);
        cs = nextcs
    }
    campspots = std::ptr::null_mut();
}
//end of the function BotFreeInfoEntities
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotInitInfoEntities() {
    let mut classname: [libc::c_char; 128] = [0; 128];
    let mut ml: *mut maplocation_t = std::ptr::null_mut();
    let mut cs: *mut campspot_t = std::ptr::null_mut();
    let mut ent: i32 = 0;
    let mut numlocations: i32 = 0;
    let mut numcampspots: i32 = 0;
    BotFreeInfoEntities();
    //
    numlocations = 0 as i32; //end for
    numcampspots = 0 as i32;
    ent = crate::src::botlib::be_aas_bspq3::AAS_NextBSPEntity(0 as i32);
    while ent != 0 {
        if !(crate::src::botlib::be_aas_bspq3::AAS_ValueForBSPEpairKey(
            ent,
            b"classname\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            classname.as_mut_ptr(),
            128 as i32,
        ) == 0)
        {
            //map locations
            if libc::strcmp(
                classname.as_mut_ptr(),
                b"target_location\x00" as *const u8 as *const libc::c_char,
            ) == 0
            {
                ml = crate::src::botlib::l_memory::GetClearedMemory(::std::mem::size_of::<
                    maplocation_t,
                >() as usize) as *mut maplocation_t; //end if
                crate::src::botlib::be_aas_bspq3::AAS_VectorForBSPEpairKey(
                    ent,
                    b"origin\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    (*ml).origin.as_mut_ptr(),
                );
                crate::src::botlib::be_aas_bspq3::AAS_ValueForBSPEpairKey(
                    ent,
                    b"message\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    (*ml).name.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 128]>() as usize as i32,
                );
                (*ml).areanum =
                    crate::src::botlib::be_aas_sample::AAS_PointAreaNum((*ml).origin.as_mut_ptr());
                (*ml).next = maplocations;
                maplocations = ml;
                numlocations += 1
            } else if libc::strcmp(
                classname.as_mut_ptr(),
                b"info_camp\x00" as *const u8 as *const libc::c_char,
            ) == 0
            {
                cs = crate::src::botlib::l_memory::GetClearedMemory(
                    ::std::mem::size_of::<campspot_t>() as usize,
                ) as *mut campspot_t;
                crate::src::botlib::be_aas_bspq3::AAS_VectorForBSPEpairKey(
                    ent,
                    b"origin\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    (*cs).origin.as_mut_ptr(),
                );
                //camp spots
                //cs->origin[2] += 16;
                crate::src::botlib::be_aas_bspq3::AAS_ValueForBSPEpairKey(
                    ent,
                    b"message\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    (*cs).name.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 128]>() as usize as i32,
                ); //end if
                crate::src::botlib::be_aas_bspq3::AAS_FloatForBSPEpairKey(
                    ent,
                    b"range\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    &mut (*cs).range,
                );
                crate::src::botlib::be_aas_bspq3::AAS_FloatForBSPEpairKey(
                    ent,
                    b"weight\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    &mut (*cs).weight,
                );
                crate::src::botlib::be_aas_bspq3::AAS_FloatForBSPEpairKey(
                    ent,
                    b"wait\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    &mut (*cs).wait,
                );
                crate::src::botlib::be_aas_bspq3::AAS_FloatForBSPEpairKey(
                    ent,
                    b"random\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    &mut (*cs).random,
                );
                (*cs).areanum =
                    crate::src::botlib::be_aas_sample::AAS_PointAreaNum((*cs).origin.as_mut_ptr());
                if (*cs).areanum == 0 {
                    crate::src::botlib::be_interface::botimport
                        .Print
                        .expect("non-null function pointer")(
                        1 as i32,
                        b"camp spot at %1.1f %1.1f %1.1f in solid\n\x00" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                        (*cs).origin[0 as i32 as usize] as f64,
                        (*cs).origin[1 as i32 as usize] as f64,
                        (*cs).origin[2 as i32 as usize] as f64,
                    );
                    crate::src::botlib::l_memory::FreeMemory(cs as *mut libc::c_void);
                } else {
                    (*cs).next = campspots;
                    campspots = cs;
                    //AAS_DrawPermanentCross(cs->origin, 4, LINECOLOR_YELLOW);
                    numcampspots += 1
                }
            }
        }
        ent = crate::src::botlib::be_aas_bspq3::AAS_NextBSPEntity(ent)
        //end else if
    }
    if crate::src::botlib::be_interface::botDeveloper != 0 {
        crate::src::botlib::be_interface::botimport
            .Print
            .expect("non-null function pointer")(
            1 as i32,
            b"%d map locations\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            numlocations,
        );
        crate::src::botlib::be_interface::botimport
            .Print
            .expect("non-null function pointer")(
            1 as i32,
            b"%d camp spots\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            numcampspots,
        );
    };
    //end if
}
//initializes the items in the level
//end of the function BotInitInfoEntities
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotInitLevelItems() {
    let mut i: i32 = 0;
    let mut spawnflags: i32 = 0;
    let mut value: i32 = 0;
    let mut classname: [libc::c_char; 128] = [0; 128];
    let mut origin: vec3_t = [0.; 3];
    let mut end: vec3_t = [0.; 3];
    let mut ent: i32 = 0;
    let mut goalareanum: i32 = 0;
    let mut ic: *mut itemconfig_t = std::ptr::null_mut();
    let mut li: *mut levelitem_t = std::ptr::null_mut();
    let mut trace: bsp_trace_t = bsp_trace_t {
        allsolid: qfalse,
        startsolid: qfalse,
        fraction: 0.,
        endpos: [0.; 3],
        plane: cplane_t {
            normal: [0.; 3],
            dist: 0.,
            type_0: 0,
            signbits: 0,
            pad: [0; 2],
        },
        exp_dist: 0.,
        sidenum: 0,
        surface: bsp_surface_t {
            name: [0; 16],
            flags: 0,
            value: 0,
        },
        contents: 0,
        ent: 0,
    };
    //initialize the map locations and camp spots
    BotInitInfoEntities();
    //initialize the level item heap
    InitLevelItemHeap();
    levelitems = std::ptr::null_mut();
    numlevelitems = 0 as i32;
    //
    ic = itemconfig;
    if ic.is_null() {
        return;
    }
    //if there's no AAS file loaded
    if crate::src::botlib::be_aas_main::AAS_Loaded() == 0 {
        return;
    }
    //validate the modelindexes of the item info
    i = 0 as i32; //end for
    while i < (*ic).numiteminfo {
        if (*(*ic).iteminfo.offset(i as isize)).modelindex == 0 {
            crate::src::botlib::l_log::Log_Write(
                b"item %s has modelindex 0\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                (*(*ic).iteminfo.offset(i as isize)).classname.as_mut_ptr(),
            );
        }
        i += 1
        //end if
    } //end for
    let mut current_block_67: u64;
    ent = crate::src::botlib::be_aas_bspq3::AAS_NextBSPEntity(0 as i32);
    while ent != 0 {
        if !(crate::src::botlib::be_aas_bspq3::AAS_ValueForBSPEpairKey(
            ent,
            b"classname\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            classname.as_mut_ptr(),
            128 as i32,
        ) == 0)
        {
            //
            spawnflags = 0 as i32;
            crate::src::botlib::be_aas_bspq3::AAS_IntForBSPEpairKey(
                ent,
                b"spawnflags\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                &mut spawnflags,
            );
            //
            i = 0 as i32; //end for
            while i < (*ic).numiteminfo {
                if libc::strcmp(
                    classname.as_mut_ptr(),
                    (*(*ic).iteminfo.offset(i as isize)).classname.as_mut_ptr(),
                ) == 0
                {
                    break; //end if
                }
                i += 1
            }
            if i >= (*ic).numiteminfo {
                crate::src::botlib::l_log::Log_Write(
                    b"entity %s unknown item\r\n\x00" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    classname.as_mut_ptr(),
                );
            } else if crate::src::botlib::be_aas_bspq3::AAS_VectorForBSPEpairKey(
                ent,
                b"origin\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                origin.as_mut_ptr(),
            ) == 0
            {
                crate::src::botlib::be_interface::botimport
                    .Print
                    .expect("non-null function pointer")(
                    3 as i32,
                    b"item %s without origin\n\x00" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    classname.as_mut_ptr(),
                );
            } else {
                //get the origin of the item
                //end else
                //
                goalareanum = 0 as i32;
                //if it is a floating item
                if spawnflags & 1 as i32 != 0 {
                    //end if
                    //if the item is not floating in water
                    if crate::src::botlib::be_aas_bspq3::AAS_PointContents(origin.as_mut_ptr())
                        & 32 as i32
                        == 0
                    {
                        end[0 as i32 as usize] = origin[0 as i32 as usize];
                        end[1 as i32 as usize] = origin[1 as i32 as usize];
                        end[2 as i32 as usize] = origin[2 as i32 as usize];
                        end[2 as i32 as usize] -= 32 as i32 as f32;
                        trace = crate::src::botlib::be_aas_bspq3::AAS_Trace(
                            origin.as_mut_ptr(),
                            (*(*ic).iteminfo.offset(i as isize)).mins.as_mut_ptr(),
                            (*(*ic).iteminfo.offset(i as isize)).maxs.as_mut_ptr(),
                            end.as_mut_ptr(),
                            -(1 as i32),
                            1 as i32 | 0x10000 as i32,
                        ) as bsp_trace_s;
                        //end if
                        if trace.fraction >= 1 as i32 as f32 {
                            //if the item not near the ground
                            //if the item is not reachable from a jumppad
                            goalareanum =
                                crate::src::botlib::be_aas_reach::AAS_BestReachableFromJumpPadArea(
                                    origin.as_mut_ptr(),
                                    (*(*ic).iteminfo.offset(i as isize)).mins.as_mut_ptr(),
                                    (*(*ic).iteminfo.offset(i as isize)).maxs.as_mut_ptr(),
                                );
                            crate::src::botlib::l_log::Log_Write(
                                b"item %s reachable from jumppad area %d\r\n\x00" as *const u8
                                    as *const libc::c_char
                                    as *mut libc::c_char,
                                (*(*ic).iteminfo.offset(i as isize)).classname.as_mut_ptr(),
                                goalareanum,
                            );
                            //botimport.Print(PRT_MESSAGE, "item %s reachable from jumppad area %d\r\n", ic->iteminfo[i].classname, goalareanum);
                            if goalareanum == 0 {
                                current_block_67 = 5689001924483802034;
                            } else {
                                current_block_67 = 5330834795799507926;
                            }
                        } else {
                            current_block_67 = 5330834795799507926;
                        }
                    } else {
                        current_block_67 = 5330834795799507926;
                    }
                //end if
                } else {
                    current_block_67 = 5330834795799507926;
                }
                match current_block_67 {
                    5689001924483802034 => {}
                    _ => {
                        li = AllocLevelItem();
                        if li.is_null() {
                            return;
                        }
                        //
                        numlevelitems += 1;
                        (*li).number = numlevelitems;
                        (*li).timeout = 0 as i32 as f32;
                        (*li).entitynum = 0 as i32;
                        //
                        (*li).flags = 0 as i32; //end if
                        crate::src::botlib::be_aas_bspq3::AAS_IntForBSPEpairKey(
                            ent,
                            b"notfree\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                            &mut value,
                        );
                        if value != 0 {
                            (*li).flags |= 1 as i32
                        }
                        crate::src::botlib::be_aas_bspq3::AAS_IntForBSPEpairKey(
                            ent,
                            b"notteam\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                            &mut value,
                        );
                        if value != 0 {
                            (*li).flags |= 2 as i32
                        }
                        crate::src::botlib::be_aas_bspq3::AAS_IntForBSPEpairKey(
                            ent,
                            b"notsingle\x00" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                            &mut value,
                        );
                        if value != 0 {
                            (*li).flags |= 4 as i32
                        }
                        crate::src::botlib::be_aas_bspq3::AAS_IntForBSPEpairKey(
                            ent,
                            b"notbot\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                            &mut value,
                        );
                        if value != 0 {
                            (*li).flags |= 8 as i32
                        }
                        if libc::strcmp(
                            classname.as_mut_ptr(),
                            b"item_botroam\x00" as *const u8 as *const libc::c_char,
                        ) == 0
                        {
                            (*li).flags |= 16 as i32;
                            crate::src::botlib::be_aas_bspq3::AAS_FloatForBSPEpairKey(
                                ent,
                                b"weight\x00" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                                &mut (*li).weight,
                            );
                        }
                        //if not a stationary item
                        if spawnflags & 1 as i32 == 0 {
                            if crate::src::botlib::be_aas_move::AAS_DropToFloor(
                                origin.as_mut_ptr(),
                                (*(*ic).iteminfo.offset(i as isize)).mins.as_mut_ptr(),
                                (*(*ic).iteminfo.offset(i as isize)).maxs.as_mut_ptr(),
                            ) == 0
                            {
                                crate::src::botlib::be_interface::botimport
                                    .Print
                                    .expect("non-null function pointer")(
                                    1 as i32,
                                    b"%s in solid at (%1.1f %1.1f %1.1f)\n\x00" as *const u8
                                        as *const libc::c_char
                                        as *mut libc::c_char,
                                    classname.as_mut_ptr(),
                                    origin[0 as i32 as usize] as f64,
                                    origin[1 as i32 as usize] as f64,
                                    origin[2 as i32 as usize] as f64,
                                ); //end if
                            }
                            //end if
                        }
                        //item info of the level item
                        (*li).iteminfo = i;
                        //origin of the item
                        (*li).origin[0 as i32 as usize] = origin[0 as i32 as usize];
                        (*li).origin[1 as i32 as usize] = origin[1 as i32 as usize];
                        (*li).origin[2 as i32 as usize] = origin[2 as i32 as usize];
                        //
                        if goalareanum != 0 {
                            //end else
                            (*li).goalareanum = goalareanum; //end if
                            (*li).goalorigin[0 as i32 as usize] = origin[0 as i32 as usize];
                            (*li).goalorigin[1 as i32 as usize] = origin[1 as i32 as usize];
                            (*li).goalorigin[2 as i32 as usize] = origin[2 as i32 as usize]
                        } else {
                            //get the item goal area and goal origin
                            (*li).goalareanum =
                                crate::src::botlib::be_aas_reach::AAS_BestReachableArea(
                                    origin.as_mut_ptr(),
                                    (*(*ic).iteminfo.offset(i as isize)).mins.as_mut_ptr(),
                                    (*(*ic).iteminfo.offset(i as isize)).maxs.as_mut_ptr(),
                                    (*li).goalorigin.as_mut_ptr(),
                                );
                            if (*li).goalareanum == 0 {
                                crate::src::botlib::be_interface::botimport
                                    .Print
                                    .expect("non-null function pointer")(
                                    1 as i32,
                                    b"%s not reachable for bots at (%1.1f %1.1f %1.1f)\n\x00"
                                        as *const u8
                                        as *const libc::c_char
                                        as *mut libc::c_char,
                                    classname.as_mut_ptr(),
                                    origin[0 as i32 as usize] as f64,
                                    origin[1 as i32 as usize] as f64,
                                    origin[2 as i32 as usize] as f64,
                                );
                            }
                            //end if
                        }
                        //
                        AddLevelItemToList(li);
                    }
                }
            }
        }
        ent = crate::src::botlib::be_aas_bspq3::AAS_NextBSPEntity(ent)
    }
    crate::src::botlib::be_interface::botimport
        .Print
        .expect("non-null function pointer")(
        1 as i32,
        b"found %d level items\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        numlevelitems,
    );
}
//get the name name of the goal with the given number
//end of the function BotInitLevelItems
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotGoalName(mut number: i32, mut name: *mut libc::c_char, mut size: i32) {
    let mut li: *mut levelitem_t = std::ptr::null_mut();
    if itemconfig.is_null() {
        return;
    }
    //
    li = levelitems; //end for
    while !li.is_null() {
        if (*li).number == number {
            Q_strncpyz(
                name,
                (*(*itemconfig).iteminfo.offset((*li).iteminfo as isize))
                    .name
                    .as_mut_ptr(),
                size,
            );
            return;
        }
        li = (*li).next
        //end for
    }
    libc::strcpy(name, b"\x00" as *const u8 as *const libc::c_char);
}
//reset avoid goals
//end of the function BotGoalName
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotResetAvoidGoals(mut goalstate: i32) {
    let mut gs: *mut bot_goalstate_t = std::ptr::null_mut();
    gs = BotGoalStateFromHandle(goalstate);
    if gs.is_null() {
        return;
    }
    crate::stdlib::memset(
        (*gs).avoidgoals.as_mut_ptr() as *mut libc::c_void,
        0 as i32,
        (256 as i32 as usize).wrapping_mul(::std::mem::size_of::<i32>() as usize),
    );
    crate::stdlib::memset(
        (*gs).avoidgoaltimes.as_mut_ptr() as *mut libc::c_void,
        0 as i32,
        (256 as i32 as usize).wrapping_mul(::std::mem::size_of::<f32>() as usize),
    );
}
//dump the avoid goals
//end of the function BotResetAvoidGoals
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotDumpAvoidGoals(mut goalstate: i32) {
    let mut i: i32 = 0;
    let mut gs: *mut bot_goalstate_t = std::ptr::null_mut();
    let mut name: [libc::c_char; 32] = [0; 32];
    gs = BotGoalStateFromHandle(goalstate);
    if gs.is_null() {
        return;
    }
    i = 0 as i32;
    while i < 256 as i32 {
        if (*gs).avoidgoaltimes[i as usize] >= crate::src::botlib::be_aas_main::AAS_Time() {
            BotGoalName((*gs).avoidgoals[i as usize], name.as_mut_ptr(), 32 as i32);
            crate::src::botlib::l_log::Log_Write(
                b"avoid goal %s, number %d for %f seconds\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                name.as_mut_ptr(),
                (*gs).avoidgoals[i as usize],
                ((*gs).avoidgoaltimes[i as usize] - crate::src::botlib::be_aas_main::AAS_Time())
                    as f64,
            );
        }
        i += 1
        //end if
    }
    //end for
}
//end of the function BotDumpAvoidGoals
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotAddToAvoidGoals(
    mut gs: *mut bot_goalstate_t,
    mut number: i32,
    mut avoidtime: f32,
) {
    let mut i: i32 = 0; //end for
    i = 0 as i32;
    while i < 256 as i32 {
        //if the avoid goal is already stored
        if (*gs).avoidgoals[i as usize] == number {
            (*gs).avoidgoals[i as usize] = number;
            (*gs).avoidgoaltimes[i as usize] =
                crate::src::botlib::be_aas_main::AAS_Time() + avoidtime;
            return;
        }
        i += 1
        //end if
    }
    i = 0 as i32;
    while i < 256 as i32 {
        //if this avoid goal has expired
        if (*gs).avoidgoaltimes[i as usize] < crate::src::botlib::be_aas_main::AAS_Time() {
            (*gs).avoidgoals[i as usize] = number;
            (*gs).avoidgoaltimes[i as usize] =
                crate::src::botlib::be_aas_main::AAS_Time() + avoidtime;
            return;
        }
        i += 1
        //end if
    }
    //end for
}
//remove the goal with the given number from the avoid goals
//end of the function BotAddToAvoidGoals
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotRemoveFromAvoidGoals(mut goalstate: i32, mut number: i32) {
    let mut i: i32 = 0;
    let mut gs: *mut bot_goalstate_t = std::ptr::null_mut();
    gs = BotGoalStateFromHandle(goalstate);
    if gs.is_null() {
        return;
    }
    //don't use the goals the bot wants to avoid
    i = 0 as i32;
    while i < 256 as i32 {
        if (*gs).avoidgoals[i as usize] == number
            && (*gs).avoidgoaltimes[i as usize] >= crate::src::botlib::be_aas_main::AAS_Time()
        {
            (*gs).avoidgoaltimes[i as usize] = 0 as i32 as f32;
            return;
        }
        i += 1
        //end if
    }
    //end for
}
//returns the avoid goal time
//end of the function BotRemoveFromAvoidGoals
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotAvoidGoalTime(mut goalstate: i32, mut number: i32) -> f32 {
    let mut i: i32 = 0;
    let mut gs: *mut bot_goalstate_t = std::ptr::null_mut();
    gs = BotGoalStateFromHandle(goalstate);
    if gs.is_null() {
        return 0 as i32 as f32;
    }
    //don't use the goals the bot wants to avoid
    i = 0 as i32; //end for
    while i < 256 as i32 {
        if (*gs).avoidgoals[i as usize] == number
            && (*gs).avoidgoaltimes[i as usize] >= crate::src::botlib::be_aas_main::AAS_Time()
        {
            return (*gs).avoidgoaltimes[i as usize] - crate::src::botlib::be_aas_main::AAS_Time();
        }
        i += 1
        //end if
    }
    return 0 as i32 as f32;
}
//set the avoid goal time
//end of the function BotAvoidGoalTime
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotSetAvoidGoalTime(
    mut goalstate: i32,
    mut number: i32,
    mut avoidtime: f32,
) {
    let mut gs: *mut bot_goalstate_t = std::ptr::null_mut(); //end if
    let mut li: *mut levelitem_t = std::ptr::null_mut();
    gs = BotGoalStateFromHandle(goalstate);
    if gs.is_null() {
        return;
    }
    if avoidtime < 0 as i32 as f32 {
        if itemconfig.is_null() {
            return;
        }
        //
        li = levelitems; //end for
        while !li.is_null() {
            if (*li).number == number {
                avoidtime = (*(*itemconfig).iteminfo.offset((*li).iteminfo as isize)).respawntime;
                if avoidtime == 0. {
                    avoidtime = 30 as i32 as f32
                }
                if avoidtime < 10 as i32 as f32 {
                    avoidtime = 10 as i32 as f32
                }
                BotAddToAvoidGoals(gs, number, avoidtime);
                return;
            }
            li = (*li).next
            //end for
        }
        return;
    } else {
        BotAddToAvoidGoals(gs, number, avoidtime);
    };
    //end else
}
//search for a goal for the given classname, the index can be used
//as a start point for the search when multiple goals are available with that same classname
//end of the function BotSetAvoidGoalTime
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotGetLevelItemGoal(
    mut index: i32,
    mut name: *mut libc::c_char,
    mut goal: *mut crate::src::botlib::be_ai_goal::bot_goal_t,
) -> i32 {
    let mut li: *mut levelitem_t = std::ptr::null_mut(); //end for
    if itemconfig.is_null() {
        return -(1 as i32);
    }
    li = levelitems;
    if index >= 0 as i32 {
        while !li.is_null() {
            if (*li).number == index {
                li = (*li).next;
                break;
            } else {
                li = (*li).next
            }
            //end if
        }
        //end for
    }
    let mut current_block_19: u64;
    //end if
    while !li.is_null() {
        //end for
        //
        if g_gametype == GT_SINGLE_PLAYER as i32 {
            if (*li).flags & 4 as i32 != 0 {
                current_block_19 = 11650488183268122163;
            } else {
                current_block_19 = 6057473163062296781;
            }
        } else if g_gametype >= GT_TEAM as i32 {
            if (*li).flags & 2 as i32 != 0 {
                current_block_19 = 11650488183268122163;
            } else {
                current_block_19 = 6057473163062296781;
            }
        } else if (*li).flags & 1 as i32 != 0 {
            current_block_19 = 11650488183268122163;
        } else {
            current_block_19 = 6057473163062296781;
        }
        match current_block_19 {
            6057473163062296781 => {
                if !((*li).flags & 8 as i32 != 0) {
                    //
                    if Q_stricmp(
                        name,
                        (*(*itemconfig).iteminfo.offset((*li).iteminfo as isize))
                            .name
                            .as_mut_ptr(),
                    ) == 0
                    {
                        (*goal).areanum = (*li).goalareanum;
                        (*goal).origin[0 as i32 as usize] = (*li).goalorigin[0 as i32 as usize];
                        (*goal).origin[1 as i32 as usize] = (*li).goalorigin[1 as i32 as usize];
                        (*goal).origin[2 as i32 as usize] = (*li).goalorigin[2 as i32 as usize];
                        (*goal).entitynum = (*li).entitynum;
                        (*goal).mins[0 as i32 as usize] =
                            (*(*itemconfig).iteminfo.offset((*li).iteminfo as isize)).mins
                                [0 as i32 as usize];
                        (*goal).mins[1 as i32 as usize] =
                            (*(*itemconfig).iteminfo.offset((*li).iteminfo as isize)).mins
                                [1 as i32 as usize];
                        (*goal).mins[2 as i32 as usize] =
                            (*(*itemconfig).iteminfo.offset((*li).iteminfo as isize)).mins
                                [2 as i32 as usize];
                        (*goal).maxs[0 as i32 as usize] =
                            (*(*itemconfig).iteminfo.offset((*li).iteminfo as isize)).maxs
                                [0 as i32 as usize];
                        (*goal).maxs[1 as i32 as usize] =
                            (*(*itemconfig).iteminfo.offset((*li).iteminfo as isize)).maxs
                                [1 as i32 as usize];
                        (*goal).maxs[2 as i32 as usize] =
                            (*(*itemconfig).iteminfo.offset((*li).iteminfo as isize)).maxs
                                [2 as i32 as usize];
                        (*goal).number = (*li).number;
                        (*goal).flags = 1 as i32;
                        if (*li).timeout != 0. {
                            (*goal).flags |= 4 as i32
                        }
                        (*goal).iteminfo = (*li).iteminfo;
                        //botimport.Print(PRT_MESSAGE, "found li %s\n", itemconfig->iteminfo[li->iteminfo].name);
                        return (*li).number;
                    }
                }
            }
            _ => {}
        }
        li = (*li).next
    }
    return -(1 as i32);
}
//get the map location with the given name
//end of the function BotGetLevelItemGoal
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotGetMapLocationGoal(
    mut name: *mut libc::c_char,
    mut goal: *mut crate::src::botlib::be_ai_goal::bot_goal_t,
) -> i32 {
    let mut ml: *mut maplocation_t = std::ptr::null_mut(); //end for
    let mut mins: vec3_t = [
        -(8 as i32) as vec_t,
        -(8 as i32) as vec_t,
        -(8 as i32) as vec_t,
    ];
    let mut maxs: vec3_t = [8 as i32 as vec_t, 8 as i32 as vec_t, 8 as i32 as vec_t];
    ml = maplocations;
    while !ml.is_null() {
        if Q_stricmp((*ml).name.as_mut_ptr(), name) == 0 {
            (*goal).areanum = (*ml).areanum;
            (*goal).origin[0 as i32 as usize] = (*ml).origin[0 as i32 as usize];
            (*goal).origin[1 as i32 as usize] = (*ml).origin[1 as i32 as usize];
            (*goal).origin[2 as i32 as usize] = (*ml).origin[2 as i32 as usize];
            (*goal).entitynum = 0 as i32;
            (*goal).mins[0 as i32 as usize] = mins[0 as i32 as usize];
            (*goal).mins[1 as i32 as usize] = mins[1 as i32 as usize];
            (*goal).mins[2 as i32 as usize] = mins[2 as i32 as usize];
            (*goal).maxs[0 as i32 as usize] = maxs[0 as i32 as usize];
            (*goal).maxs[1 as i32 as usize] = maxs[1 as i32 as usize];
            (*goal).maxs[2 as i32 as usize] = maxs[2 as i32 as usize];
            (*goal).number = 0 as i32;
            (*goal).flags = 0 as i32;
            (*goal).iteminfo = 0 as i32;
            return qtrue as i32;
        }
        ml = (*ml).next
        //end if
    }
    return qfalse as i32;
}
//get the next camp spot in the map
//end of the function BotGetMapLocationGoal
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotGetNextCampSpotGoal(
    mut num: i32,
    mut goal: *mut crate::src::botlib::be_ai_goal::bot_goal_t,
) -> i32 {
    let mut i: i32 = 0; //end for
    let mut cs: *mut campspot_t = std::ptr::null_mut();
    let mut mins: vec3_t = [
        -(8 as i32) as vec_t,
        -(8 as i32) as vec_t,
        -(8 as i32) as vec_t,
    ];
    let mut maxs: vec3_t = [8 as i32 as vec_t, 8 as i32 as vec_t, 8 as i32 as vec_t];
    if num < 0 as i32 {
        num = 0 as i32
    }
    i = num;
    cs = campspots;
    while !cs.is_null() {
        i -= 1;
        if i < 0 as i32 {
            (*goal).areanum = (*cs).areanum;
            (*goal).origin[0 as i32 as usize] = (*cs).origin[0 as i32 as usize];
            (*goal).origin[1 as i32 as usize] = (*cs).origin[1 as i32 as usize];
            (*goal).origin[2 as i32 as usize] = (*cs).origin[2 as i32 as usize];
            (*goal).entitynum = 0 as i32;
            (*goal).mins[0 as i32 as usize] = mins[0 as i32 as usize];
            (*goal).mins[1 as i32 as usize] = mins[1 as i32 as usize];
            (*goal).mins[2 as i32 as usize] = mins[2 as i32 as usize];
            (*goal).maxs[0 as i32 as usize] = maxs[0 as i32 as usize];
            (*goal).maxs[1 as i32 as usize] = maxs[1 as i32 as usize];
            (*goal).maxs[2 as i32 as usize] = maxs[2 as i32 as usize];
            (*goal).number = 0 as i32;
            (*goal).flags = 0 as i32;
            (*goal).iteminfo = 0 as i32;
            return num + 1 as i32;
        }
        cs = (*cs).next
        //end if
    }
    return 0 as i32;
}
//end of the function BotGetNextCampSpotGoal
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotFindEntityForLevelItem(mut li: *mut levelitem_t) {
    let mut ent: i32 = 0;
    let mut modelindex: i32 = 0;
    let mut ic: *mut itemconfig_t = std::ptr::null_mut();
    let mut entinfo: aas_entityinfo_t = aas_entityinfo_t {
        valid: 0,
        type_0: 0,
        flags: 0,
        ltime: 0.,
        update_time: 0.,
        number: 0,
        origin: [0.; 3],
        angles: [0.; 3],
        old_origin: [0.; 3],
        lastvisorigin: [0.; 3],
        mins: [0.; 3],
        maxs: [0.; 3],
        groundent: 0,
        solid: 0,
        modelindex: 0,
        modelindex2: 0,
        frame: 0,
        event: 0,
        eventParm: 0,
        powerups: 0,
        weapon: 0,
        legsAnim: 0,
        torsoAnim: 0,
    };
    let mut dir: vec3_t = [0.; 3];
    ic = itemconfig;
    if itemconfig.is_null() {
        return;
    }
    ent = crate::src::botlib::be_aas_entity::AAS_NextEntity(0 as i32);
    while ent != 0 {
        //get the model index of the entity
        modelindex = crate::src::botlib::be_aas_entity::AAS_EntityModelindex(ent);
        //end if
        //
        if !(modelindex == 0) {
            //get info about the entity
            crate::src::botlib::be_aas_entity::AAS_EntityInfo(
                ent,
                &mut entinfo as *mut _ as *mut aas_entityinfo_s,
            );
            //if the entity is still moving
            if !(entinfo.origin[0 as i32 as usize] != entinfo.lastvisorigin[0 as i32 as usize]
                || entinfo.origin[1 as i32 as usize] != entinfo.lastvisorigin[1 as i32 as usize]
                || entinfo.origin[2 as i32 as usize] != entinfo.lastvisorigin[2 as i32 as usize])
            {
                //
                if (*(*ic).iteminfo.offset((*li).iteminfo as isize)).modelindex == modelindex {
                    //check if the entity is very close
                    dir[0 as i32 as usize] =
                        (*li).origin[0 as i32 as usize] - entinfo.origin[0 as i32 as usize];
                    dir[1 as i32 as usize] =
                        (*li).origin[1 as i32 as usize] - entinfo.origin[1 as i32 as usize];
                    dir[2 as i32 as usize] =
                        (*li).origin[2 as i32 as usize] - entinfo.origin[2 as i32 as usize];
                    if VectorLength(dir.as_mut_ptr() as *const vec_t) < 30 as i32 as f32 {
                        //end if
                        //found an entity for this level item
                        (*li).entitynum = ent
                    }
                }
            }
        }
        ent = crate::src::botlib::be_aas_entity::AAS_NextEntity(ent)
    }
    //end for
}
//regularly update dynamic entity items (dropped weapons, flags etc.)
#[no_mangle]

pub unsafe extern "C" fn BotUpdateEntityItems() {
    let mut ent: i32 = 0;
    let mut i: i32 = 0;
    let mut modelindex: i32 = 0;
    let mut dir: vec3_t = [0.; 3];
    let mut li: *mut levelitem_t = std::ptr::null_mut();
    let mut nextli: *mut levelitem_t = std::ptr::null_mut();
    let mut entinfo: aas_entityinfo_t = aas_entityinfo_t {
        valid: 0,
        type_0: 0,
        flags: 0,
        ltime: 0.,
        update_time: 0.,
        number: 0,
        origin: [0.; 3],
        angles: [0.; 3],
        old_origin: [0.; 3],
        lastvisorigin: [0.; 3],
        mins: [0.; 3],
        maxs: [0.; 3],
        groundent: 0,
        solid: 0,
        modelindex: 0,
        modelindex2: 0,
        frame: 0,
        event: 0,
        eventParm: 0,
        powerups: 0,
        weapon: 0,
        legsAnim: 0,
        torsoAnim: 0,
    };
    let mut ic: *mut itemconfig_t = std::ptr::null_mut();
    //timeout current entity items if necessary
    li = levelitems; //end for
    while !li.is_null() {
        nextli = (*li).next;
        //end if
        if (*li).timeout != 0. {
            //if it is an item that will time out
            //timeout the item
            if (*li).timeout < crate::src::botlib::be_aas_main::AAS_Time() {
                RemoveLevelItemFromList(li);
                FreeLevelItem(li);
            }
            //end if
        }
        li = nextli
    }
    //find new entity items
    ic = itemconfig;
    if itemconfig.is_null() {
        return;
    }
    //
    ent = crate::src::botlib::be_aas_entity::AAS_NextEntity(0 as i32);
    while ent != 0 {
        if !(crate::src::botlib::be_aas_entity::AAS_EntityType(ent) != 2 as i32) {
            //get the model index of the entity
            modelindex = crate::src::botlib::be_aas_entity::AAS_EntityModelindex(ent);
            //
            if !(modelindex == 0) {
                //get info about the entity
                crate::src::botlib::be_aas_entity::AAS_EntityInfo(
                    ent,
                    &mut entinfo as *mut _ as *mut aas_entityinfo_s,
                );
                //FIXME: don't do this
                //skip all floating items for now
                //if (entinfo.groundent != ENTITYNUM_WORLD) continue;
                //if the entity is still moving
                if !(entinfo.origin[0 as i32 as usize] != entinfo.lastvisorigin[0 as i32 as usize]
                    || entinfo.origin[1 as i32 as usize]
                        != entinfo.lastvisorigin[1 as i32 as usize]
                    || entinfo.origin[2 as i32 as usize]
                        != entinfo.lastvisorigin[2 as i32 as usize])
                {
                    //check if the entity is already stored as a level item
                    li = levelitems; //end for
                    while !li.is_null() {
                        //if the level item is linked to an entity
                        if (*li).entitynum != 0 && (*li).entitynum == ent {
                            //the entity is re-used if the models are different
                            if (*(*ic).iteminfo.offset((*li).iteminfo as isize)).modelindex
                                != modelindex
                            {
                                RemoveLevelItemFromList(li); //end if
                                FreeLevelItem(li);
                                li = std::ptr::null_mut();
                                break;
                            } else {
                                //remove this level item
                                if entinfo.origin[0 as i32 as usize]
                                    != (*li).origin[0 as i32 as usize]
                                    || entinfo.origin[1 as i32 as usize]
                                        != (*li).origin[1 as i32 as usize]
                                    || entinfo.origin[2 as i32 as usize]
                                        != (*li).origin[2 as i32 as usize]
                                {
                                    (*li).origin[0 as i32 as usize] =
                                        entinfo.origin[0 as i32 as usize]; //end if
                                    (*li).origin[1 as i32 as usize] =
                                        entinfo.origin[1 as i32 as usize];
                                    (*li).origin[2 as i32 as usize] =
                                        entinfo.origin[2 as i32 as usize];
                                    //also update the goal area number
                                    (*li).goalareanum =
                                        crate::src::botlib::be_aas_reach::AAS_BestReachableArea(
                                            (*li).origin.as_mut_ptr(),
                                            (*(*ic).iteminfo.offset((*li).iteminfo as isize))
                                                .mins
                                                .as_mut_ptr(),
                                            (*(*ic).iteminfo.offset((*li).iteminfo as isize))
                                                .maxs
                                                .as_mut_ptr(),
                                            (*li).goalorigin.as_mut_ptr(),
                                        )
                                }
                                break;
                            }
                        //end else
                        } else {
                            li = (*li).next
                        }
                        //end if
                    }
                    if li.is_null() {
                        let mut current_block_31: u64;
                        //try to link the entity to a level item
                        li = levelitems; //end for
                        while !li.is_null() {
                            //if this level item is already linked
                            if !((*li).entitynum != 0) {
                                //
                                if g_gametype == GT_SINGLE_PLAYER as i32 {
                                    if (*li).flags & 4 as i32 != 0 {
                                        current_block_31 = 15597372965620363352;
                                    } else {
                                        current_block_31 = 14775119014532381840;
                                    }
                                } else if g_gametype >= GT_TEAM as i32 {
                                    if (*li).flags & 2 as i32 != 0 {
                                        current_block_31 = 15597372965620363352;
                                    } else {
                                        current_block_31 = 14775119014532381840;
                                    }
                                } else if (*li).flags & 1 as i32 != 0 {
                                    current_block_31 = 15597372965620363352;
                                } else {
                                    current_block_31 = 14775119014532381840;
                                }
                                match current_block_31 {
                                    15597372965620363352 => {}
                                    _ =>
                                    //if the model of the level item and the entity are the same
                                    {
                                        if (*(*ic).iteminfo.offset((*li).iteminfo as isize))
                                            .modelindex
                                            == modelindex
                                        {
                                            //check if the entity is very close
                                            dir[0 as i32 as usize] = (*li).origin
                                                [0 as i32 as usize]
                                                - entinfo.origin[0 as i32 as usize];
                                            dir[1 as i32 as usize] = (*li).origin
                                                [1 as i32 as usize]
                                                - entinfo.origin[1 as i32 as usize];
                                            dir[2 as i32 as usize] = (*li).origin
                                                [2 as i32 as usize]
                                                - entinfo.origin[2 as i32 as usize];
                                            if VectorLength(dir.as_mut_ptr() as *const vec_t)
                                                < 30 as i32 as f32
                                            {
                                                //end if
                                                //found an entity for this level item
                                                (*li).entitynum = ent;
                                                //if the origin is different
                                                if entinfo.origin[0 as i32 as usize]
                                                    != (*li).origin[0 as i32 as usize]
                                                    || entinfo.origin[1 as i32 as usize]
                                                        != (*li).origin[1 as i32 as usize]
                                                    || entinfo.origin[2 as i32 as usize]
                                                        != (*li).origin[2 as i32 as usize]
                                                {
                                                    //end if
                                                    //update the level item origin
                                                    (*li).origin[0 as i32 as usize] =
                                                        entinfo.origin[0 as i32 as usize];
                                                    (*li).origin[1 as i32 as usize] =
                                                        entinfo.origin[1 as i32 as usize];
                                                    (*li).origin[2 as i32 as usize] =
                                                        entinfo.origin[2 as i32 as usize];
                                                    //also update the goal area number
                                                    (*li).goalareanum =
                                                        crate::src::botlib::be_aas_reach::AAS_BestReachableArea((*li).origin.as_mut_ptr(),
                                                                              (*(*ic).iteminfo.offset((*li).iteminfo
                                                                                                          as
                                                                                                          isize)).mins.as_mut_ptr(),
                                                                              (*(*ic).iteminfo.offset((*li).iteminfo
                                                                                                          as
                                                                                                          isize)).maxs.as_mut_ptr(),
                                                                              (*li).goalorigin.as_mut_ptr())
                                                }
                                                break;
                                            }
                                        }
                                    }
                                }
                            }
                            li = (*li).next
                            //end else
                        }
                        if li.is_null() {
                            //check if the model is from a known item
                            i = 0 as i32; //end for
                            while i < (*ic).numiteminfo {
                                if (*(*ic).iteminfo.offset(i as isize)).modelindex == modelindex {
                                    break;
                                    //end if
                                }
                                i += 1
                            }
                            //if the model is not from a known item
                            if !(i >= (*ic).numiteminfo) {
                                //allocate a new level item
                                li = AllocLevelItem();
                                //
                                if !li.is_null() {
                                    //entity number of the level item
                                    (*li).entitynum = ent;
                                    //number for the level item
                                    (*li).number = numlevelitems + ent;
                                    //set the item info index for the level item
                                    (*li).iteminfo = i;
                                    //origin of the item
                                    (*li).origin[0 as i32 as usize] =
                                        entinfo.origin[0 as i32 as usize];
                                    (*li).origin[1 as i32 as usize] =
                                        entinfo.origin[1 as i32 as usize];
                                    (*li).origin[2 as i32 as usize] =
                                        entinfo.origin[2 as i32 as usize];
                                    //get the item goal area and goal origin
                                    (*li).goalareanum =
                                        crate::src::botlib::be_aas_reach::AAS_BestReachableArea(
                                            (*li).origin.as_mut_ptr(),
                                            (*(*ic).iteminfo.offset(i as isize)).mins.as_mut_ptr(),
                                            (*(*ic).iteminfo.offset(i as isize)).maxs.as_mut_ptr(),
                                            (*li).goalorigin.as_mut_ptr(),
                                        );
                                    //never go for items dropped into jumppads
                                    if crate::src::botlib::be_aas_reach::AAS_AreaJumpPad(
                                        (*li).goalareanum,
                                    ) != 0
                                    {
                                        FreeLevelItem(li); //end if
                                    } else {
                                        //time this item out after 30 seconds
                                        //dropped items disappear after 30 seconds
                                        (*li).timeout = crate::src::botlib::be_aas_main::AAS_Time()
                                            + 30 as i32 as f32;
                                        //add the level item to the list
                                        AddLevelItemToList(li);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        ent = crate::src::botlib::be_aas_entity::AAS_NextEntity(ent)
        //botimport.Print(PRT_MESSAGE, "found new level item %s\n", ic->iteminfo[i].classname);
    }
    //end for
    /*
    for (li = levelitems; li; li = li->next)
    {
        if (!li->entitynum)
        {
            BotFindEntityForLevelItem(li);
        } //end if
    } //end for*/
}
//dump the goal stack
//end of the function BotUpdateEntityItems
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotDumpGoalStack(mut goalstate: i32) {
    let mut i: i32 = 0;
    let mut gs: *mut bot_goalstate_t = std::ptr::null_mut();
    let mut name: [libc::c_char; 32] = [0; 32];
    gs = BotGoalStateFromHandle(goalstate);
    if gs.is_null() {
        return;
    }
    i = 1 as i32;
    while i <= (*gs).goalstacktop {
        BotGoalName(
            (*gs).goalstack[i as usize].number,
            name.as_mut_ptr(),
            32 as i32,
        );
        crate::src::botlib::l_log::Log_Write(
            b"%d: %s\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            i,
            name.as_mut_ptr(),
        );
        i += 1
    }
    //end for
}
//push a goal onto the goal stack
//end of the function BotDumpGoalStack
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotPushGoal(
    mut goalstate: i32,
    mut goal: *mut crate::src::botlib::be_ai_goal::bot_goal_t,
) {
    let mut gs: *mut bot_goalstate_t = std::ptr::null_mut(); //end if
    gs = BotGoalStateFromHandle(goalstate);
    if gs.is_null() {
        return;
    }
    if (*gs).goalstacktop >= 8 as i32 - 1 as i32 {
        crate::src::botlib::be_interface::botimport
            .Print
            .expect("non-null function pointer")(
            3 as i32,
            b"goal heap overflow\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        BotDumpGoalStack(goalstate);
        return;
    }
    (*gs).goalstacktop += 1;
    crate::stdlib::memcpy(
        &mut *(*gs)
            .goalstack
            .as_mut_ptr()
            .offset((*gs).goalstacktop as isize)
            as *mut crate::src::botlib::be_ai_goal::bot_goal_t as *mut libc::c_void,
        goal as *const libc::c_void,
        ::std::mem::size_of::<crate::src::botlib::be_ai_goal::bot_goal_t>() as usize,
    );
}
//pop a goal from the goal stack
//end of the function BotPushGoal
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotPopGoal(mut goalstate: i32) {
    let mut gs: *mut bot_goalstate_t = std::ptr::null_mut();
    gs = BotGoalStateFromHandle(goalstate);
    if gs.is_null() {
        return;
    }
    if (*gs).goalstacktop > 0 as i32 {
        (*gs).goalstacktop -= 1
    };
}
//empty the bot's goal stack
//end of the function BotPopGoal
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotEmptyGoalStack(mut goalstate: i32) {
    let mut gs: *mut bot_goalstate_t = std::ptr::null_mut();
    gs = BotGoalStateFromHandle(goalstate);
    if gs.is_null() {
        return;
    }
    (*gs).goalstacktop = 0 as i32;
}
//get the top goal from the stack
//end of the function BotEmptyGoalStack
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotGetTopGoal(
    mut goalstate: i32,
    mut goal: *mut crate::src::botlib::be_ai_goal::bot_goal_t,
) -> i32 {
    let mut gs: *mut bot_goalstate_t = std::ptr::null_mut();
    gs = BotGoalStateFromHandle(goalstate);
    if gs.is_null() {
        return qfalse as i32;
    }
    if (*gs).goalstacktop == 0 {
        return qfalse as i32;
    }
    crate::stdlib::memcpy(
        goal as *mut libc::c_void,
        &mut *(*gs)
            .goalstack
            .as_mut_ptr()
            .offset((*gs).goalstacktop as isize)
            as *mut crate::src::botlib::be_ai_goal::bot_goal_t as *const libc::c_void,
        ::std::mem::size_of::<crate::src::botlib::be_ai_goal::bot_goal_t>() as usize,
    );
    return qtrue as i32;
}
//get the second goal on the stack
//end of the function BotGetTopGoal
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotGetSecondGoal(
    mut goalstate: i32,
    mut goal: *mut crate::src::botlib::be_ai_goal::bot_goal_t,
) -> i32 {
    let mut gs: *mut bot_goalstate_t = std::ptr::null_mut();
    gs = BotGoalStateFromHandle(goalstate);
    if gs.is_null() {
        return qfalse as i32;
    }
    if (*gs).goalstacktop <= 1 as i32 {
        return qfalse as i32;
    }
    crate::stdlib::memcpy(
        goal as *mut libc::c_void,
        &mut *(*gs)
            .goalstack
            .as_mut_ptr()
            .offset(((*gs).goalstacktop - 1 as i32) as isize)
            as *mut crate::src::botlib::be_ai_goal::bot_goal_t as *const libc::c_void,
        ::std::mem::size_of::<crate::src::botlib::be_ai_goal::bot_goal_t>() as usize,
    );
    return qtrue as i32;
}
//choose the best long term goal item for the bot
//end of the function BotGetSecondGoal
//===========================================================================
// pops a new long term goal on the goal stack in the goalstate
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotChooseLTGItem(
    mut goalstate: i32,
    mut origin: *mut vec_t,
    mut inventory: *mut i32,
    mut travelflags: i32,
) -> i32 {
    let mut areanum: i32 = 0;
    let mut t: i32 = 0;
    let mut weightnum: i32 = 0;
    let mut weight: f32 = 0.;
    let mut bestweight: f32 = 0.;
    let mut avoidtime: f32 = 0.;
    let mut iteminfo: *mut iteminfo_t = std::ptr::null_mut();
    let mut ic: *mut itemconfig_t = std::ptr::null_mut();
    let mut li: *mut levelitem_t = std::ptr::null_mut();
    let mut bestitem: *mut levelitem_t = std::ptr::null_mut();
    let mut goal: crate::src::botlib::be_ai_goal::bot_goal_t =
        crate::src::botlib::be_ai_goal::bot_goal_t {
            origin: [0.; 3],
            areanum: 0,
            mins: [0.; 3],
            maxs: [0.; 3],
            entitynum: 0,
            number: 0,
            flags: 0,
            iteminfo: 0,
        };
    let mut gs: *mut bot_goalstate_t = std::ptr::null_mut();
    gs = BotGoalStateFromHandle(goalstate);
    if gs.is_null() {
        return qfalse as i32;
    }
    if (*gs).itemweightconfig.is_null() {
        return qfalse as i32;
    }
    //get the area the bot is in
    areanum = crate::src::botlib::be_ai_move::BotReachabilityArea(origin, (*gs).client);
    //if the bot is in solid or if the area the bot is in has no reachability links
    if areanum == 0 || crate::src::botlib::be_aas_reach::AAS_AreaReachability(areanum) == 0 {
        //end if
        //use the last valid area the bot was in
        areanum = (*gs).lastreachabilityarea
    }
    //remember the last area with reachabilities the bot was in
    (*gs).lastreachabilityarea = areanum;
    //if still in solid
    if areanum == 0 {
        return qfalse as i32;
    }
    //the item configuration
    ic = itemconfig;
    if itemconfig.is_null() {
        return qfalse as i32;
    }
    //best weight and item so far
    bestweight = 0 as i32 as f32;
    bestitem = std::ptr::null_mut();
    crate::stdlib::memset(
        &mut goal as *mut crate::src::botlib::be_ai_goal::bot_goal_t as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<crate::src::botlib::be_ai_goal::bot_goal_t>() as usize,
    );
    let mut current_block_32: u64;
    //go through the items in the level
    li = levelitems; //end for
    while !li.is_null() {
        if g_gametype == GT_SINGLE_PLAYER as i32 {
            if (*li).flags & 4 as i32 != 0 {
                current_block_32 = 2838571290723028321;
            } else {
                current_block_32 = 3437258052017859086;
            }
        } else if g_gametype >= GT_TEAM as i32 {
            if (*li).flags & 2 as i32 != 0 {
                current_block_32 = 2838571290723028321;
            } else {
                current_block_32 = 3437258052017859086;
            }
        } else if (*li).flags & 1 as i32 != 0 {
            current_block_32 = 2838571290723028321;
        } else {
            current_block_32 = 3437258052017859086;
        }
        match current_block_32 {
            3437258052017859086 => {
                if !((*li).flags & 8 as i32 != 0) {
                    //end if
                    //if the item is not in a possible goal area
                    if !((*li).goalareanum == 0) {
                        //FIXME: is this a good thing? added this for items that never spawned into the game (f.i. CTF flags in obelisk)
                        if !((*li).entitynum == 0 && (*li).flags & 16 as i32 == 0) {
                            //get the fuzzy weight function for this item
                            iteminfo = &mut *(*ic).iteminfo.offset((*li).iteminfo as isize)
                                as *mut iteminfo_t;
                            weightnum = *(*gs).itemweightindex.offset((*iteminfo).number as isize);
                            if !(weightnum < 0 as i32) {
                                weight = FuzzyWeightUndecided(
                                    inventory,
                                    (*gs).itemweightconfig as *mut weightconfig_s,
                                    weightnum,
                                );
                                //UNDECIDEDFUZZY
                                //HACK: to make dropped items more attractive
                                if (*li).timeout != 0. {
                                    weight += (*droppedweight).value
                                }
                                //DROPPEDWEIGHT
                                //use weight scale for item_botroam
                                if (*li).flags & 16 as i32 != 0 {
                                    weight *= (*li).weight
                                }
                                //
                                if weight > 0 as i32 as f32 {
                                    //get the travel time towards the goal area
                                    t =
                                        crate::src::botlib::be_aas_route::AAS_AreaTravelTimeToGoalArea(areanum,
                                                                     origin,
                                                                     (*li).goalareanum,
                                                                     travelflags);
                                    //end if
                                    if t > 0 as i32 {
                                        //if the goal is reachable
                                        //if this item won't respawn before we get there
                                        avoidtime = BotAvoidGoalTime(goalstate, (*li).number);
                                        if !(avoidtime as f64 - t as f64 * 0.009f64
                                            > 0 as i32 as f64)
                                        {
                                            //end if
                                            //
                                            weight = (weight as f64 / (t as f32 as f64 * 0.01f64))
                                                as f32;
                                            //
                                            if weight > bestweight {
                                                bestweight = weight;
                                                bestitem = li
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            _ => {}
        }
        li = (*li).next
    }
    //if no goal item found
    if bestitem.is_null() {
        //end if
        /*
                //if not in lava or slime
                if (!AAS_AreaLava(areanum) && !AAS_AreaSlime(areanum))
                {
                    if (AAS_RandomGoalArea(areanum, travelflags, &goal.areanum, goal.origin))
                    {
                        VectorSet(goal.mins, -15, -15, -15);
                        VectorSet(goal.maxs, 15, 15, 15);
                        goal.entitynum = 0;
                        goal.number = 0;
                        goal.flags = GFL_ROAM;
                        goal.iteminfo = 0;
                        //push the goal on the stack
                        BotPushGoal(goalstate, &goal);
                        //
        #ifdef DEBUG
                        botimport.Print(PRT_MESSAGE, "chosen roam goal area %d\n", goal.areanum);
        #endif //DEBUG
                        return qtrue;
                    } //end if
                } //end if
                */
        return qfalse as i32;
    }
    //create a bot goal for this item
    iteminfo = &mut *(*ic).iteminfo.offset((*bestitem).iteminfo as isize) as *mut iteminfo_t;
    goal.origin[0 as i32 as usize] = (*bestitem).goalorigin[0 as i32 as usize];
    goal.origin[1 as i32 as usize] = (*bestitem).goalorigin[1 as i32 as usize];
    goal.origin[2 as i32 as usize] = (*bestitem).goalorigin[2 as i32 as usize];
    goal.mins[0 as i32 as usize] = (*iteminfo).mins[0 as i32 as usize];
    goal.mins[1 as i32 as usize] = (*iteminfo).mins[1 as i32 as usize];
    goal.mins[2 as i32 as usize] = (*iteminfo).mins[2 as i32 as usize];
    goal.maxs[0 as i32 as usize] = (*iteminfo).maxs[0 as i32 as usize];
    goal.maxs[1 as i32 as usize] = (*iteminfo).maxs[1 as i32 as usize];
    goal.maxs[2 as i32 as usize] = (*iteminfo).maxs[2 as i32 as usize];
    goal.areanum = (*bestitem).goalareanum;
    goal.entitynum = (*bestitem).entitynum;
    goal.number = (*bestitem).number;
    goal.flags = 1 as i32;
    if (*bestitem).timeout != 0. {
        goal.flags |= 4 as i32
    }
    if (*bestitem).flags & 16 as i32 != 0 {
        goal.flags |= 2 as i32
    }
    goal.iteminfo = (*bestitem).iteminfo;
    //if it's a dropped item
    if (*bestitem).timeout != 0. {
        //end else
        avoidtime = 10 as i32 as f32
    } else {
        avoidtime = (*iteminfo).respawntime; //end if
        if avoidtime == 0. {
            avoidtime = 30 as i32 as f32
        }
        if avoidtime < 10 as i32 as f32 {
            avoidtime = 10 as i32 as f32
        }
    }
    //add the chosen goal to the goals to avoid for a while
    BotAddToAvoidGoals(gs, (*bestitem).number, avoidtime);
    //push the goal on the stack
    BotPushGoal(goalstate, &mut goal);
    //
    return qtrue as i32;
}
//choose the best nearby goal item for the bot
//the item may not be further away from the current bot position than maxtime
//also the travel time from the nearby goal towards the long term goal may not
//be larger than the travel time towards the long term goal from the current bot position
//end of the function BotChooseLTGItem
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotChooseNBGItem(
    mut goalstate: i32,
    mut origin: *mut vec_t,
    mut inventory: *mut i32,
    mut travelflags: i32,
    mut ltg: *mut crate::src::botlib::be_ai_goal::bot_goal_t,
    mut maxtime: f32,
) -> i32 {
    let mut areanum: i32 = 0;
    let mut t: i32 = 0;
    let mut weightnum: i32 = 0;
    let mut ltg_time: i32 = 0;
    let mut weight: f32 = 0.;
    let mut bestweight: f32 = 0.;
    let mut avoidtime: f32 = 0.;
    let mut iteminfo: *mut iteminfo_t = std::ptr::null_mut();
    let mut ic: *mut itemconfig_t = std::ptr::null_mut();
    let mut li: *mut levelitem_t = std::ptr::null_mut();
    let mut bestitem: *mut levelitem_t = std::ptr::null_mut();
    let mut goal: crate::src::botlib::be_ai_goal::bot_goal_t =
        crate::src::botlib::be_ai_goal::bot_goal_t {
            origin: [0.; 3],
            areanum: 0,
            mins: [0.; 3],
            maxs: [0.; 3],
            entitynum: 0,
            number: 0,
            flags: 0,
            iteminfo: 0,
        };
    let mut gs: *mut bot_goalstate_t = std::ptr::null_mut();
    gs = BotGoalStateFromHandle(goalstate);
    if gs.is_null() {
        return qfalse as i32;
    }
    if (*gs).itemweightconfig.is_null() {
        return qfalse as i32;
    }
    //get the area the bot is in
    areanum = crate::src::botlib::be_ai_move::BotReachabilityArea(origin, (*gs).client);
    //if the bot is in solid or if the area the bot is in has no reachability links
    if areanum == 0 || crate::src::botlib::be_aas_reach::AAS_AreaReachability(areanum) == 0 {
        //end if
        //use the last valid area the bot was in
        areanum = (*gs).lastreachabilityarea
    }
    //remember the last area with reachabilities the bot was in
    (*gs).lastreachabilityarea = areanum;
    //if still in solid
    if areanum == 0 {
        return qfalse as i32;
    }
    //
    if !ltg.is_null() {
        ltg_time = crate::src::botlib::be_aas_route::AAS_AreaTravelTimeToGoalArea(
            areanum,
            origin,
            (*ltg).areanum,
            travelflags,
        )
    } else {
        ltg_time = 99999 as i32
    }
    //the item configuration
    ic = itemconfig;
    if itemconfig.is_null() {
        return qfalse as i32;
    }
    //best weight and item so far
    bestweight = 0 as i32 as f32;
    bestitem = std::ptr::null_mut();
    crate::stdlib::memset(
        &mut goal as *mut crate::src::botlib::be_ai_goal::bot_goal_t as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<crate::src::botlib::be_ai_goal::bot_goal_t>() as usize,
    );
    let mut current_block_41: u64;
    //go through the items in the level
    li = levelitems; //end for
    while !li.is_null() {
        if g_gametype == GT_SINGLE_PLAYER as i32 {
            if (*li).flags & 4 as i32 != 0 {
                current_block_41 = 18317007320854588510;
            } else {
                current_block_41 = 11636175345244025579;
            }
        } else if g_gametype >= GT_TEAM as i32 {
            if (*li).flags & 2 as i32 != 0 {
                current_block_41 = 18317007320854588510;
            } else {
                current_block_41 = 11636175345244025579;
            }
        } else if (*li).flags & 1 as i32 != 0 {
            current_block_41 = 18317007320854588510;
        } else {
            current_block_41 = 11636175345244025579;
        }
        match current_block_41 {
            11636175345244025579 => {
                if !((*li).flags & 8 as i32 != 0) {
                    //end if
                    //if the item is in a possible goal area
                    if !((*li).goalareanum == 0) {
                        //FIXME: is this a good thing? added this for items that never spawned into the game (f.i. CTF flags in obelisk)
                        if !((*li).entitynum == 0 && (*li).flags & 16 as i32 == 0) {
                            //get the fuzzy weight function for this item
                            iteminfo = &mut *(*ic).iteminfo.offset((*li).iteminfo as isize)
                                as *mut iteminfo_t;
                            weightnum = *(*gs).itemweightindex.offset((*iteminfo).number as isize);
                            if !(weightnum < 0 as i32) {
                                //
                                weight = FuzzyWeightUndecided(
                                    inventory,
                                    (*gs).itemweightconfig as *mut weightconfig_s,
                                    weightnum,
                                );
                                //UNDECIDEDFUZZY
                                //HACK: to make dropped items more attractive
                                if (*li).timeout != 0. {
                                    weight += (*droppedweight).value
                                }
                                //DROPPEDWEIGHT
                                //use weight scale for item_botroam
                                if (*li).flags & 16 as i32 != 0 {
                                    weight *= (*li).weight
                                }
                                //
                                if weight > 0 as i32 as f32 {
                                    //get the travel time towards the goal area
                                    t =
                                        crate::src::botlib::be_aas_route::AAS_AreaTravelTimeToGoalArea(areanum,
                                                                     origin,
                                                                     (*li).goalareanum,
                                                                     travelflags);
                                    //end if
                                    if t > 0 as i32 && (t as f32) < maxtime {
                                        //if the goal is reachable
                                        //if this item won't respawn before we get there
                                        avoidtime = BotAvoidGoalTime(goalstate, (*li).number);
                                        if !(avoidtime as f64 - t as f64 * 0.009f64
                                            > 0 as i32 as f64)
                                        {
                                            //end if
                                            //
                                            weight = (weight as f64 / (t as f32 as f64 * 0.01f64))
                                                as f32;
                                            //
                                            if weight > bestweight {
                                                t = 0 as i32;
                                                //end if
                                                if !ltg.is_null() && (*li).timeout == 0. {
                                                    //end if
                                                    //get the travel time from the goal to the long term goal
                                                    t =
                                                        crate::src::botlib::be_aas_route::AAS_AreaTravelTimeToGoalArea((*li).goalareanum,
                                                                                     (*li).goalorigin.as_mut_ptr(),
                                                                                     (*ltg).areanum,
                                                                                     travelflags)
                                                }
                                                if t <= ltg_time {
                                                    bestweight = weight;
                                                    bestitem = li
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            _ => {}
        }
        li = (*li).next
    }
    //if the travel back is possible and doesn't take too long
    //if no goal item found
    if bestitem.is_null() {
        return qfalse as i32;
    }
    //create a bot goal for this item
    iteminfo = &mut *(*ic).iteminfo.offset((*bestitem).iteminfo as isize) as *mut iteminfo_t;
    goal.origin[0 as i32 as usize] = (*bestitem).goalorigin[0 as i32 as usize];
    goal.origin[1 as i32 as usize] = (*bestitem).goalorigin[1 as i32 as usize];
    goal.origin[2 as i32 as usize] = (*bestitem).goalorigin[2 as i32 as usize];
    goal.mins[0 as i32 as usize] = (*iteminfo).mins[0 as i32 as usize];
    goal.mins[1 as i32 as usize] = (*iteminfo).mins[1 as i32 as usize];
    goal.mins[2 as i32 as usize] = (*iteminfo).mins[2 as i32 as usize];
    goal.maxs[0 as i32 as usize] = (*iteminfo).maxs[0 as i32 as usize];
    goal.maxs[1 as i32 as usize] = (*iteminfo).maxs[1 as i32 as usize];
    goal.maxs[2 as i32 as usize] = (*iteminfo).maxs[2 as i32 as usize];
    goal.areanum = (*bestitem).goalareanum;
    goal.entitynum = (*bestitem).entitynum;
    goal.number = (*bestitem).number;
    goal.flags = 1 as i32;
    if (*bestitem).timeout != 0. {
        goal.flags |= 4 as i32
    }
    if (*bestitem).flags & 16 as i32 != 0 {
        goal.flags |= 2 as i32
    }
    goal.iteminfo = (*bestitem).iteminfo;
    //if it's a dropped item
    if (*bestitem).timeout != 0. {
        //end else
        avoidtime = 10 as i32 as f32
    } else {
        avoidtime = (*iteminfo).respawntime; //end if
        if avoidtime == 0. {
            avoidtime = 30 as i32 as f32
        }
        if avoidtime < 10 as i32 as f32 {
            avoidtime = 10 as i32 as f32
        }
    }
    //add the chosen goal to the goals to avoid for a while
    BotAddToAvoidGoals(gs, (*bestitem).number, avoidtime);
    //push the goal on the stack
    BotPushGoal(goalstate, &mut goal);
    //
    return qtrue as i32;
}
//returns true if the bot touches the goal
//end of the function BotChooseNBGItem
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotTouchingGoal(
    mut origin: *mut vec_t,
    mut goal: *mut crate::src::botlib::be_ai_goal::bot_goal_t,
) -> i32 {
    let mut i: i32 = 0; //{4, 4, 10};
    let mut boxmins: vec3_t = [0.; 3]; //{-4, -4, 0};
    let mut boxmaxs: vec3_t = [0.; 3];
    let mut absmins: vec3_t = [0.; 3];
    let mut absmaxs: vec3_t = [0.; 3];
    let mut safety_maxs: vec3_t = [0 as i32 as vec_t, 0 as i32 as vec_t, 0 as i32 as vec_t];
    let mut safety_mins: vec3_t = [0 as i32 as vec_t, 0 as i32 as vec_t, 0 as i32 as vec_t];
    crate::src::botlib::be_aas_sample::AAS_PresenceTypeBoundingBox(
        2 as i32,
        boxmins.as_mut_ptr(),
        boxmaxs.as_mut_ptr(),
    );
    absmins[0 as i32 as usize] = (*goal).mins[0 as i32 as usize] - boxmaxs[0 as i32 as usize];
    absmins[1 as i32 as usize] = (*goal).mins[1 as i32 as usize] - boxmaxs[1 as i32 as usize];
    absmins[2 as i32 as usize] = (*goal).mins[2 as i32 as usize] - boxmaxs[2 as i32 as usize];
    absmaxs[0 as i32 as usize] = (*goal).maxs[0 as i32 as usize] - boxmins[0 as i32 as usize];
    absmaxs[1 as i32 as usize] = (*goal).maxs[1 as i32 as usize] - boxmins[1 as i32 as usize];
    absmaxs[2 as i32 as usize] = (*goal).maxs[2 as i32 as usize] - boxmins[2 as i32 as usize];
    absmins[0 as i32 as usize] = absmins[0 as i32 as usize] + (*goal).origin[0 as i32 as usize];
    absmins[1 as i32 as usize] = absmins[1 as i32 as usize] + (*goal).origin[1 as i32 as usize];
    absmins[2 as i32 as usize] = absmins[2 as i32 as usize] + (*goal).origin[2 as i32 as usize];
    absmaxs[0 as i32 as usize] = absmaxs[0 as i32 as usize] + (*goal).origin[0 as i32 as usize];
    absmaxs[1 as i32 as usize] = absmaxs[1 as i32 as usize] + (*goal).origin[1 as i32 as usize];
    absmaxs[2 as i32 as usize] = absmaxs[2 as i32 as usize] + (*goal).origin[2 as i32 as usize];
    //make the box a little smaller for safety
    absmaxs[0 as i32 as usize] = absmaxs[0 as i32 as usize] - safety_maxs[0 as i32 as usize]; //end for
    absmaxs[1 as i32 as usize] = absmaxs[1 as i32 as usize] - safety_maxs[1 as i32 as usize];
    absmaxs[2 as i32 as usize] = absmaxs[2 as i32 as usize] - safety_maxs[2 as i32 as usize];
    absmins[0 as i32 as usize] = absmins[0 as i32 as usize] - safety_mins[0 as i32 as usize];
    absmins[1 as i32 as usize] = absmins[1 as i32 as usize] - safety_mins[1 as i32 as usize];
    absmins[2 as i32 as usize] = absmins[2 as i32 as usize] - safety_mins[2 as i32 as usize];
    i = 0 as i32;
    while i < 3 as i32 {
        if *origin.offset(i as isize) < absmins[i as usize]
            || *origin.offset(i as isize) > absmaxs[i as usize]
        {
            return qfalse as i32;
        }
        i += 1
    }
    return qtrue as i32;
}
//returns true if the goal should be visible but isn't
//end of the function BotTouchingGoal
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotItemGoalInVisButNotVisible(
    mut viewer: i32,
    mut eye: *mut vec_t,
    mut _viewangles: *mut vec_t,
    mut goal: *mut crate::src::botlib::be_ai_goal::bot_goal_t,
) -> i32 {
    let mut entinfo: aas_entityinfo_t = aas_entityinfo_t {
        valid: 0,
        type_0: 0,
        flags: 0,
        ltime: 0.,
        update_time: 0.,
        number: 0,
        origin: [0.; 3],
        angles: [0.; 3],
        old_origin: [0.; 3],
        lastvisorigin: [0.; 3],
        mins: [0.; 3],
        maxs: [0.; 3],
        groundent: 0,
        solid: 0,
        modelindex: 0,
        modelindex2: 0,
        frame: 0,
        event: 0,
        eventParm: 0,
        powerups: 0,
        weapon: 0,
        legsAnim: 0,
        torsoAnim: 0,
    };
    let mut trace: bsp_trace_t = bsp_trace_t {
        allsolid: qfalse,
        startsolid: qfalse,
        fraction: 0.,
        endpos: [0.; 3],
        plane: cplane_t {
            normal: [0.; 3],
            dist: 0.,
            type_0: 0,
            signbits: 0,
            pad: [0; 2],
        },
        exp_dist: 0.,
        sidenum: 0,
        surface: bsp_surface_t {
            name: [0; 16],
            flags: 0,
            value: 0,
        },
        contents: 0,
        ent: 0,
    };
    let mut middle: vec3_t = [0.; 3];
    if (*goal).flags & 1 as i32 == 0 {
        return qfalse as i32;
    }
    //
    middle[0 as i32 as usize] = (*goal).mins[0 as i32 as usize] + (*goal).mins[0 as i32 as usize];
    middle[1 as i32 as usize] = (*goal).mins[1 as i32 as usize] + (*goal).mins[1 as i32 as usize];
    middle[2 as i32 as usize] = (*goal).mins[2 as i32 as usize] + (*goal).mins[2 as i32 as usize];
    middle[0 as i32 as usize] = (middle[0 as i32 as usize] as f64 * 0.5f64) as vec_t;
    middle[1 as i32 as usize] = (middle[1 as i32 as usize] as f64 * 0.5f64) as vec_t;
    middle[2 as i32 as usize] = (middle[2 as i32 as usize] as f64 * 0.5f64) as vec_t;
    middle[0 as i32 as usize] = (*goal).origin[0 as i32 as usize] + middle[0 as i32 as usize];
    middle[1 as i32 as usize] = (*goal).origin[1 as i32 as usize] + middle[1 as i32 as usize];
    middle[2 as i32 as usize] = (*goal).origin[2 as i32 as usize] + middle[2 as i32 as usize];
    //
    trace = crate::src::botlib::be_aas_bspq3::AAS_Trace(
        eye,
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        middle.as_mut_ptr(),
        viewer,
        1 as i32,
    ) as bsp_trace_s;
    //if the goal middle point is visible
    if trace.fraction >= 1 as i32 as f32 {
        //end if
        //the goal entity number doesn't have to be valid
        //just assume it's valid
        if (*goal).entitynum <= 0 as i32 {
            return qfalse as i32;
        }
        //
        //if the entity data isn't valid
        crate::src::botlib::be_aas_entity::AAS_EntityInfo(
            (*goal).entitynum,
            &mut entinfo as *mut _ as *mut aas_entityinfo_s,
        );
        //NOTE: for some wacko reason entities are sometimes
        // not updated
        //if (!entinfo.valid) return qtrue;
        if (entinfo.ltime as f64) < crate::src::botlib::be_aas_main::AAS_Time() as f64 - 0.5f64 {
            return qtrue as i32;
        }
    }
    return qfalse as i32;
}
//reset the whole goal state, but keep the item weights
//end of the function BotItemGoalInVisButNotVisible
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotResetGoalState(mut goalstate: i32) {
    let mut gs: *mut bot_goalstate_t = std::ptr::null_mut();
    gs = BotGoalStateFromHandle(goalstate);
    if gs.is_null() {
        return;
    }
    crate::stdlib::memset(
        (*gs).goalstack.as_mut_ptr() as *mut libc::c_void,
        0 as i32,
        (8 as i32 as usize).wrapping_mul(::std::mem::size_of::<
            crate::src::botlib::be_ai_goal::bot_goal_t,
        >() as usize),
    );
    (*gs).goalstacktop = 0 as i32;
    BotResetAvoidGoals(goalstate);
}
//loads item weights for the bot
//end of the function BotResetGoalState
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotLoadItemWeights(
    mut goalstate: i32,
    mut filename: *mut libc::c_char,
) -> i32 {
    let mut gs: *mut bot_goalstate_t = std::ptr::null_mut();
    gs = BotGoalStateFromHandle(goalstate);
    if gs.is_null() {
        return 9 as i32;
    }
    //load the weight configuration
    (*gs).itemweightconfig = ReadWeightConfig(filename) as *mut weightconfig_s; //end if
    if (*gs).itemweightconfig.is_null() {
        crate::src::botlib::be_interface::botimport
            .Print
            .expect("non-null function pointer")(
            4 as i32,
            b"couldn\'t load weights\n\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return 9 as i32;
    }
    //if there's no item configuration
    if itemconfig.is_null() {
        return 9 as i32;
    }
    //create the item weight index
    (*gs).itemweightindex = ItemWeightIndex((*gs).itemweightconfig, itemconfig);
    //everything went ok
    return 0 as i32;
}
//frees the item weights of the bot
//end of the function BotLoadItemWeights
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotFreeItemWeights(mut goalstate: i32) {
    let mut gs: *mut bot_goalstate_t = std::ptr::null_mut();
    gs = BotGoalStateFromHandle(goalstate);
    if gs.is_null() {
        return;
    }
    if !(*gs).itemweightconfig.is_null() {
        FreeWeightConfig((*gs).itemweightconfig as *mut weightconfig_s);
    }
    if !(*gs).itemweightindex.is_null() {
        crate::src::botlib::l_memory::FreeMemory((*gs).itemweightindex as *mut libc::c_void);
    };
}
//returns the handle of a newly allocated goal state
//end of the function BotFreeItemWeights
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotAllocGoalState(mut client: i32) -> i32 {
    let mut i: i32 = 0; //end for
    i = 1 as i32;
    while i <= 64 as i32 {
        if botgoalstates[i as usize].is_null() {
            botgoalstates[i as usize] = crate::src::botlib::l_memory::GetClearedMemory(
                ::std::mem::size_of::<bot_goalstate_t>() as usize,
            ) as *mut bot_goalstate_t;
            (*botgoalstates[i as usize]).client = client;
            return i;
        }
        i += 1
        //end if
    }
    return 0 as i32;
}
//free the given goal state
//end of the function BotAllocGoalState
//========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotFreeGoalState(mut handle: i32) {
    if handle <= 0 as i32 || handle > 64 as i32 {
        crate::src::botlib::be_interface::botimport
            .Print
            .expect("non-null function pointer")(
            4 as i32,
            b"goal state handle %d out of range\n\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            handle,
        ); //end if
        return;
    } //end if
    if botgoalstates[handle as usize].is_null() {
        crate::src::botlib::be_interface::botimport
            .Print
            .expect("non-null function pointer")(
            4 as i32,
            b"invalid goal state handle %d\n\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            handle,
        );
        return;
    }
    BotFreeItemWeights(handle);
    crate::src::botlib::l_memory::FreeMemory(botgoalstates[handle as usize] as *mut libc::c_void);
    botgoalstates[handle as usize] = std::ptr::null_mut();
}
//setup the goal AI
//end of the function BotFreeGoalState
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotSetupGoalAI() -> i32 {
    let mut filename: *mut libc::c_char = std::ptr::null_mut();
    //check if teamplay is on
    g_gametype = LibVarValue(
        b"g_gametype\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
    ) as i32;
    //item configuration file
    filename = LibVarString(
        b"itemconfig\x00" as *const u8 as *const libc::c_char,
        b"items.c\x00" as *const u8 as *const libc::c_char,
    );
    //load the item configuration
    itemconfig = LoadItemConfig(filename); //end if
    if itemconfig.is_null() {
        crate::src::botlib::be_interface::botimport
            .Print
            .expect("non-null function pointer")(
            4 as i32,
            b"couldn\'t load item config\n\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return 10 as i32;
    }
    //
    droppedweight = LibVar(
        b"droppedweight\x00" as *const u8 as *const libc::c_char,
        b"1000\x00" as *const u8 as *const libc::c_char,
    ) as *mut libvar_s;
    //everything went ok
    return 0 as i32;
}
//shut down the goal AI
//end of the function BotSetupGoalAI
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotShutdownGoalAI() {
    let mut i: i32 = 0;
    if !itemconfig.is_null() {
        crate::src::botlib::l_memory::FreeMemory(itemconfig as *mut libc::c_void);
    }
    itemconfig = std::ptr::null_mut();
    if !levelitemheap.is_null() {
        crate::src::botlib::l_memory::FreeMemory(levelitemheap as *mut libc::c_void);
    }
    levelitemheap = std::ptr::null_mut();
    freelevelitems = std::ptr::null_mut();
    levelitems = std::ptr::null_mut();
    numlevelitems = 0 as i32;
    BotFreeInfoEntities();
    i = 1 as i32;
    while i <= 64 as i32 {
        if !botgoalstates[i as usize].is_null() {
            BotFreeGoalState(i);
        }
        i += 1
        //end if
    }
    //end for
}
unsafe extern "C" fn run_static_initializers() {
    iteminfo_fields = [
        {
            let mut init = fielddef_s {
                name: b"name\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut (*(std::ptr::null_mut::<iteminfo_s>())).name as *mut [libc::c_char; 80]
                    as size_t as i32,
                type_0: 4 as i32,
                maxarray: 0,
                floatmin: 0.,
                floatmax: 0.,
                substruct: std::ptr::null_mut(),
            };
            init
        },
        {
            let mut init = fielddef_s {
                name: b"model\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut (*(std::ptr::null_mut::<iteminfo_s>())).model
                    as *mut [libc::c_char; 80] as size_t as i32,
                type_0: 4 as i32,
                maxarray: 0,
                floatmin: 0.,
                floatmax: 0.,
                substruct: std::ptr::null_mut(),
            };
            init
        },
        {
            let mut init = fielddef_s {
                name: b"modelindex\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut (*(std::ptr::null_mut::<iteminfo_s>())).modelindex as *mut i32
                    as size_t as i32,
                type_0: 2 as i32,
                maxarray: 0,
                floatmin: 0.,
                floatmax: 0.,
                substruct: std::ptr::null_mut(),
            };
            init
        },
        {
            let mut init = fielddef_s {
                name: b"type\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut (*(std::ptr::null_mut::<iteminfo_s>())).type_0 as *mut i32 as size_t
                    as i32,
                type_0: 2 as i32,
                maxarray: 0,
                floatmin: 0.,
                floatmax: 0.,
                substruct: std::ptr::null_mut(),
            };
            init
        },
        {
            let mut init = fielddef_s {
                name: b"index\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut (*(std::ptr::null_mut::<iteminfo_s>())).index as *mut i32 as size_t
                    as i32,
                type_0: 2 as i32,
                maxarray: 0,
                floatmin: 0.,
                floatmax: 0.,
                substruct: std::ptr::null_mut(),
            };
            init
        },
        {
            let mut init = fielddef_s {
                name: b"respawntime\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut (*(std::ptr::null_mut::<iteminfo_s>())).respawntime as *mut f32
                    as size_t as i32,
                type_0: 3 as i32,
                maxarray: 0,
                floatmin: 0.,
                floatmax: 0.,
                substruct: std::ptr::null_mut(),
            };
            init
        },
        {
            let mut init = fielddef_s {
                name: b"mins\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut (*(std::ptr::null_mut::<iteminfo_s>())).mins as *mut vec3_t as size_t
                    as i32,
                type_0: 3 as i32 | 0x100 as i32,
                maxarray: 3 as i32,
                floatmin: 0.,
                floatmax: 0.,
                substruct: std::ptr::null_mut(),
            };
            init
        },
        {
            let mut init = fielddef_s {
                name: b"maxs\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut (*(std::ptr::null_mut::<iteminfo_s>())).maxs as *mut vec3_t as size_t
                    as i32,
                type_0: 3 as i32 | 0x100 as i32,
                maxarray: 3 as i32,
                floatmin: 0.,
                floatmax: 0.,
                substruct: std::ptr::null_mut(),
            };
            init
        },
        {
            let mut init = fielddef_s {
                name: std::ptr::null_mut(),
                offset: 0 as i32,
                type_0: 0 as i32,
                maxarray: 0,
                floatmin: 0.,
                floatmax: 0.,
                substruct: std::ptr::null_mut(),
            };
            init
        },
    ]
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
//end of the function BotShutdownGoalAI

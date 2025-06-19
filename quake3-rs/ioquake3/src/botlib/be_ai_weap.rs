// =============== BEGIN be_ai_weap_h ================
pub type projectileinfo_t = crate::src::botlib::be_ai_weap::projectileinfo_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct projectileinfo_s {
    pub name: [libc::c_char; 80],
    pub model: [libc::c_char; 80],
    pub flags: i32,
    pub gravity: f32,
    pub damage: i32,
    pub radius: f32,
    pub visdamage: i32,
    pub damagetype: i32,
    pub healthinc: i32,
    pub push: f32,
    pub detonation: f32,
    pub bounce: f32,
    pub bouncefric: f32,
    pub bouncestop: f32,
}

pub type weaponinfo_t = crate::src::botlib::be_ai_weap::weaponinfo_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct weaponinfo_s {
    pub valid: i32,
    pub number: i32,
    pub name: [libc::c_char; 80],
    pub model: [libc::c_char; 80],
    pub level: i32,
    pub weaponindex: i32,
    pub flags: i32,
    pub projectile: [libc::c_char; 80],
    pub numprojectiles: i32,
    pub hspread: f32,
    pub vspread: f32,
    pub speed: f32,
    pub acceleration: f32,
    pub recoil: crate::src::qcommon::q_shared::vec3_t,
    pub offset: crate::src::qcommon::q_shared::vec3_t,
    pub angleoffset: crate::src::qcommon::q_shared::vec3_t,
    pub extrazvelocity: f32,
    pub ammoamount: i32,
    pub ammoindex: i32,
    pub activate: f32,
    pub reload: f32,
    pub spinup: f32,
    pub spindown: f32,
    pub proj: crate::src::botlib::be_ai_weap::projectileinfo_t,
}
use ::libc;

pub use crate::stddef_h::size_t;

pub use crate::botlib_h::botlib_import_s;
pub use crate::botlib_h::botlib_import_t;
pub use crate::botlib_h::bsp_surface_s;
pub use crate::botlib_h::bsp_surface_t;
pub use crate::botlib_h::bsp_trace_s;
pub use crate::botlib_h::bsp_trace_t;
pub use crate::src::botlib::be_ai_weight::fuzzyseperator_s;
pub use crate::src::botlib::be_ai_weight::weight_s;
pub use crate::src::botlib::be_ai_weight::weight_t;
pub use crate::src::botlib::be_ai_weight::weightconfig_s;
pub use crate::src::botlib::be_ai_weight::weightconfig_t;
pub use crate::src::botlib::be_ai_weight::FindFuzzyWeight;
pub use crate::src::botlib::be_ai_weight::FreeWeightConfig;
pub use crate::src::botlib::be_ai_weight::FuzzyWeight;
pub use crate::src::botlib::be_ai_weight::ReadWeightConfig;

pub use crate::src::botlib::l_precomp::define_s;
pub use crate::src::botlib::l_precomp::define_t;
pub use crate::src::botlib::l_precomp::indent_s;
pub use crate::src::botlib::l_precomp::indent_t;
pub use crate::src::botlib::l_precomp::source_s;
pub use crate::src::botlib::l_precomp::source_t;
pub use crate::src::botlib::l_precomp::FreeSource;
pub use crate::src::botlib::l_precomp::LoadSourceFile;
pub use crate::src::botlib::l_precomp::PC_ReadToken;
pub use crate::src::botlib::l_precomp::PC_SetBaseFolder;
pub use crate::src::botlib::l_script::punctuation_s;
pub use crate::src::botlib::l_script::punctuation_t;
pub use crate::src::botlib::l_script::script_s;
pub use crate::src::botlib::l_script::script_t;
pub use crate::src::botlib::l_script::token_s;
pub use crate::src::botlib::l_script::token_t;
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
pub use crate::src::qcommon::q_shared::Q_strncpyz;
pub use crate::src::qcommon::q_shared::FS_APPEND;
pub use crate::src::qcommon::q_shared::FS_APPEND_SYNC;
pub use crate::src::qcommon::q_shared::FS_READ;
pub use crate::src::qcommon::q_shared::FS_WRITE;

//weapon configuration: set of weapons with projectiles

pub type weaponconfig_t = weaponconfig_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct weaponconfig_s {
    pub numweapons: i32,
    pub numprojectiles: i32,
    pub projectileinfo: *mut crate::src::botlib::be_ai_weap::projectileinfo_t,
    pub weaponinfo: *mut crate::src::botlib::be_ai_weap::weaponinfo_t,
}
//the bot weapon state

pub type bot_weaponstate_t = bot_weaponstate_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bot_weaponstate_s {
    pub weaponweightconfig: *mut weightconfig_s,
    pub weaponweightindex: *mut i32,
}
//weapon weight configuration
//weapon weight index
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
/* ****************************************************************************
 * name:		be_ai_weap.c
 *
 * desc:		weapon AI
 *
 * $Archive: /MissionPack/code/botlib/be_ai_weap.c $
 *
 *****************************************************************************/
//#define DEBUG_AI_WEAP
//structure field offsets
//weapon definition
// Initialized in run_static_initializers

static mut weaponinfo_fields: [fielddef_t; 23] = [fielddef_t {
    name: 0 as *mut libc::c_char,
    offset: 0,
    type_0: 0,
    maxarray: 0,
    floatmin: 0.,
    floatmax: 0.,
    substruct: 0 as *mut structdef_s,
}; 23];
//projectile definition
// Initialized in run_static_initializers

static mut projectileinfo_fields: [fielddef_t; 15] = [fielddef_t {
    name: 0 as *mut libc::c_char,
    offset: 0,
    type_0: 0,
    maxarray: 0,
    floatmin: 0.,
    floatmax: 0.,
    substruct: 0 as *mut structdef_s,
}; 15];

static mut weaponinfo_struct: structdef_t = unsafe {
    {
        let mut init = structdef_s {
            size: ::std::mem::size_of::<crate::src::botlib::be_ai_weap::weaponinfo_t>()
                as usize as i32,
            fields: weaponinfo_fields.as_ptr() as *mut _,
        };
        init
    }
};

static mut projectileinfo_struct: structdef_t = unsafe {
    {
        let mut init = structdef_s {
            size: ::std::mem::size_of::<crate::src::botlib::be_ai_weap::projectileinfo_t>()
                as usize as i32,
            fields: projectileinfo_fields.as_ptr() as *mut _,
        };
        init
    }
};

static mut botweaponstates: [*mut bot_weaponstate_t; 65] =
    [0 as *const bot_weaponstate_t as *mut bot_weaponstate_t; 65];

static mut weaponconfig: *mut weaponconfig_t = 0 as *const weaponconfig_t as *mut weaponconfig_t;
//========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotValidWeaponNumber(mut weaponnum: i32) -> i32 {
    if weaponnum <= 0 as i32 || weaponnum > (*weaponconfig).numweapons {
        crate::src::botlib::be_interface::botimport
            .Print
            .expect("non-null function pointer")(
            3 as i32,
            b"weapon number out of range\n\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        ); //end if
        return qfalse as i32;
    }
    return qtrue as i32;
}
//end of the function BotValidWeaponNumber
//========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotWeaponStateFromHandle(mut handle: i32) -> *mut bot_weaponstate_t {
    if handle <= 0 as i32 || handle > 64 as i32 {
        crate::src::botlib::be_interface::botimport
            .Print
            .expect("non-null function pointer")(
            4 as i32,
            b"weapon state handle %d out of range\n\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            handle,
        ); //end if
        return 0 as *mut bot_weaponstate_t;
    } //end if
    if botweaponstates[handle as usize].is_null() {
        crate::src::botlib::be_interface::botimport
            .Print
            .expect("non-null function pointer")(
            4 as i32,
            b"invalid weapon state %d\n\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            handle,
        );
        return 0 as *mut bot_weaponstate_t;
    }
    return botweaponstates[handle as usize];
}
//end of the function BotWeaponStateFromHandle
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
//DEBUG_AI_WEAP
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn LoadWeaponConfig(mut filename: *mut libc::c_char) -> *mut weaponconfig_t {
    let mut max_weaponinfo: i32 = 0; //end if
    let mut max_projectileinfo: i32 = 0; //end if
    let mut token: token_t = token_t {
        string: [0; 1024],
        type_0: 0,
        subtype: 0,
        intvalue: 0,
        floatvalue: 0.,
        whitespace_p: 0 as *mut libc::c_char,
        endwhitespace_p: 0 as *mut libc::c_char,
        line: 0,
        linescrossed: 0,
        next: 0 as *mut token_s,
    }; //end if
    let mut path: [libc::c_char; 64] = [0; 64];
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut source: *mut source_t = 0 as *mut source_t;
    let mut wc: *mut weaponconfig_t = 0 as *mut weaponconfig_t;
    let mut weaponinfo: crate::src::botlib::be_ai_weap::weaponinfo_t =
        crate::src::botlib::be_ai_weap::weaponinfo_t {
            valid: 0,
            number: 0,
            name: [0; 80],
            model: [0; 80],
            level: 0,
            weaponindex: 0,
            flags: 0,
            projectile: [0; 80],
            numprojectiles: 0,
            hspread: 0.,
            vspread: 0.,
            speed: 0.,
            acceleration: 0.,
            recoil: [0.; 3],
            offset: [0.; 3],
            angleoffset: [0.; 3],
            extrazvelocity: 0.,
            ammoamount: 0,
            ammoindex: 0,
            activate: 0.,
            reload: 0.,
            spinup: 0.,
            spindown: 0.,
            proj: crate::src::botlib::be_ai_weap::projectileinfo_t {
                name: [0; 80],
                model: [0; 80],
                flags: 0,
                gravity: 0.,
                damage: 0,
                radius: 0.,
                visdamage: 0,
                damagetype: 0,
                healthinc: 0,
                push: 0.,
                detonation: 0.,
                bounce: 0.,
                bouncefric: 0.,
                bouncestop: 0.,
            },
        };
    max_weaponinfo = crate::src::botlib::l_libvar::LibVarValue(
        b"max_weaponinfo\x00" as *const u8 as *const libc::c_char,
        b"32\x00" as *const u8 as *const libc::c_char,
    ) as i32;
    if max_weaponinfo < 0 as i32 {
        crate::src::botlib::be_interface::botimport
            .Print
            .expect("non-null function pointer")(
            3 as i32,
            b"max_weaponinfo = %d\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            max_weaponinfo,
        );
        max_weaponinfo = 32 as i32;
        crate::src::botlib::l_libvar::LibVarSet(
            b"max_weaponinfo\x00" as *const u8 as *const libc::c_char,
            b"32\x00" as *const u8 as *const libc::c_char,
        );
    }
    max_projectileinfo = crate::src::botlib::l_libvar::LibVarValue(
        b"max_projectileinfo\x00" as *const u8 as *const libc::c_char,
        b"32\x00" as *const u8 as *const libc::c_char,
    ) as i32;
    if max_projectileinfo < 0 as i32 {
        crate::src::botlib::be_interface::botimport
            .Print
            .expect("non-null function pointer")(
            3 as i32,
            b"max_projectileinfo = %d\n\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            max_projectileinfo,
        );
        max_projectileinfo = 32 as i32;
        crate::src::botlib::l_libvar::LibVarSet(
            b"max_projectileinfo\x00" as *const u8 as *const libc::c_char,
            b"32\x00" as *const u8 as *const libc::c_char,
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
        return 0 as *mut weaponconfig_t;
    }
    //initialize weapon config
    wc = crate::src::botlib::l_memory::GetClearedHunkMemory(
        (::std::mem::size_of::<weaponconfig_t>() as usize)
            .wrapping_add(
                (max_weaponinfo as usize).wrapping_mul(::std::mem::size_of::<
                    crate::src::botlib::be_ai_weap::weaponinfo_t,
                >()
                    as usize),
            )
            .wrapping_add((max_projectileinfo as usize).wrapping_mul(
                ::std::mem::size_of::<crate::src::botlib::be_ai_weap::projectileinfo_t>()
                    as usize,
            )),
    ) as *mut weaponconfig_t;
    (*wc).weaponinfo = (wc as *mut libc::c_char)
        .offset(::std::mem::size_of::<weaponconfig_t>() as usize as isize)
        as *mut crate::src::botlib::be_ai_weap::weaponinfo_t;
    (*wc).projectileinfo = ((*wc).weaponinfo as *mut libc::c_char).offset(
        (max_weaponinfo as usize).wrapping_mul(::std::mem::size_of::<
            crate::src::botlib::be_ai_weap::weaponinfo_t,
        >() as usize) as isize,
    ) as *mut crate::src::botlib::be_ai_weap::projectileinfo_t;
    (*wc).numweapons = max_weaponinfo;
    (*wc).numprojectiles = 0 as i32;
    //parse the source file
    while PC_ReadToken(
        source as *mut source_s,
        &mut token as *mut _ as *mut token_s,
    ) != 0
    {
        if libc::strcmp(
            token.string.as_mut_ptr(),
            b"weaponinfo\x00" as *const u8 as *const libc::c_char,
        ) == 0
        {
            //end while
            crate::stdlib::memset(
                &mut weaponinfo as *mut crate::src::botlib::be_ai_weap::weaponinfo_t
                    as *mut libc::c_void,
                0 as i32,
                ::std::mem::size_of::<crate::src::botlib::be_ai_weap::weaponinfo_t>()
                    as usize,
            ); //end if
            if ReadStructure(
                source as *mut source_s,
                &mut weaponinfo_struct as *mut _ as *mut structdef_s,
                &mut weaponinfo as *mut crate::src::botlib::be_ai_weap::weaponinfo_t
                    as *mut libc::c_char,
            ) == 0
            {
                crate::src::botlib::l_memory::FreeMemory(wc as *mut libc::c_void); //end if
                FreeSource(source as *mut source_s); //end if
                return 0 as *mut weaponconfig_t;
            } //end if
            if weaponinfo.number < 0 as i32 || weaponinfo.number >= max_weaponinfo {
                crate::src::botlib::be_interface::botimport
                    .Print
                    .expect("non-null function pointer")(
                    3 as i32,
                    b"weapon info number %d out of range in %s\n\x00" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    weaponinfo.number,
                    path.as_mut_ptr(),
                ); //end if
                crate::src::botlib::l_memory::FreeMemory(wc as *mut libc::c_void); //end if
                FreeSource(source as *mut source_s);
                return 0 as *mut weaponconfig_t;
            }
            crate::stdlib::memcpy(
                &mut *(*wc).weaponinfo.offset(weaponinfo.number as isize)
                    as *mut crate::src::botlib::be_ai_weap::weaponinfo_t
                    as *mut libc::c_void,
                &mut weaponinfo as *mut crate::src::botlib::be_ai_weap::weaponinfo_t
                    as *const libc::c_void,
                ::std::mem::size_of::<crate::src::botlib::be_ai_weap::weaponinfo_t>()
                    as usize,
            );
            (*(*wc).weaponinfo.offset(weaponinfo.number as isize)).valid = qtrue as i32
        } else if libc::strcmp(
            token.string.as_mut_ptr(),
            b"projectileinfo\x00" as *const u8 as *const libc::c_char,
        ) == 0
        {
            if (*wc).numprojectiles >= max_projectileinfo {
                crate::src::botlib::be_interface::botimport
                    .Print
                    .expect("non-null function pointer")(
                    3 as i32,
                    b"more than %d projectiles defined in %s\n\x00" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    max_projectileinfo,
                    path.as_mut_ptr(),
                );
                crate::src::botlib::l_memory::FreeMemory(wc as *mut libc::c_void);
                FreeSource(source as *mut source_s);
                return 0 as *mut weaponconfig_t;
            }
            crate::stdlib::memset(
                &mut *(*wc).projectileinfo.offset((*wc).numprojectiles as isize)
                    as *mut crate::src::botlib::be_ai_weap::projectileinfo_t
                    as *mut libc::c_void,
                0 as i32,
                ::std::mem::size_of::<crate::src::botlib::be_ai_weap::projectileinfo_t>()
                    as usize,
            );
            if ReadStructure(
                source as *mut source_s,
                &mut projectileinfo_struct as *mut _ as *mut structdef_s,
                &mut *(*wc).projectileinfo.offset((*wc).numprojectiles as isize)
                    as *mut crate::src::botlib::be_ai_weap::projectileinfo_t
                    as *mut libc::c_char,
            ) == 0
            {
                crate::src::botlib::l_memory::FreeMemory(wc as *mut libc::c_void);
                FreeSource(source as *mut source_s);
                return 0 as *mut weaponconfig_t;
            }
            (*wc).numprojectiles += 1
        } else {
            crate::src::botlib::be_interface::botimport
                .Print
                .expect("non-null function pointer")(
                3 as i32,
                b"unknown definition %s in %s\n\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                token.string.as_mut_ptr(),
                path.as_mut_ptr(),
            );
            crate::src::botlib::l_memory::FreeMemory(wc as *mut libc::c_void);
            FreeSource(source as *mut source_s);
            return 0 as *mut weaponconfig_t;
        }
        //end else
    }
    FreeSource(source as *mut source_s);
    //fix up weapons
    i = 0 as i32; //end for
    while i < (*wc).numweapons {
        if !((*(*wc).weaponinfo.offset(i as isize)).valid == 0) {
            if (*(*wc).weaponinfo.offset(i as isize)).name[0 as i32 as usize] == 0 {
                crate::src::botlib::be_interface::botimport
                    .Print
                    .expect("non-null function pointer")(
                    3 as i32,
                    b"weapon %d has no name in %s\n\x00" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    i,
                    path.as_mut_ptr(),
                ); //end if
                crate::src::botlib::l_memory::FreeMemory(wc as *mut libc::c_void); //end if
                return 0 as *mut weaponconfig_t;
            }
            if (*(*wc).weaponinfo.offset(i as isize)).projectile[0 as i32 as usize] == 0 {
                crate::src::botlib::be_interface::botimport
                    .Print
                    .expect("non-null function pointer")(
                    3 as i32,
                    b"weapon %s has no projectile in %s\n\x00" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    (*(*wc).weaponinfo.offset(i as isize)).name.as_mut_ptr(),
                    path.as_mut_ptr(),
                );
                crate::src::botlib::l_memory::FreeMemory(wc as *mut libc::c_void);
                return 0 as *mut weaponconfig_t;
            }
            //find the projectile info and copy it to the weapon info
            j = 0 as i32; //end for
            while j < (*wc).numprojectiles {
                if libc::strcmp(
                    (*(*wc).projectileinfo.offset(j as isize)).name.as_mut_ptr(),
                    (*(*wc).weaponinfo.offset(i as isize))
                        .projectile
                        .as_mut_ptr(),
                ) == 0
                {
                    crate::stdlib::memcpy(
                        &mut (*(*wc).weaponinfo.offset(i as isize)).proj
                            as *mut crate::src::botlib::be_ai_weap::projectileinfo_t
                            as *mut libc::c_void,
                        &mut *(*wc).projectileinfo.offset(j as isize)
                            as *mut crate::src::botlib::be_ai_weap::projectileinfo_t
                            as *const libc::c_void,
                        ::std::mem::size_of::<crate::src::botlib::be_ai_weap::projectileinfo_t>()
                            as usize,
                    );
                    break;
                } else {
                    j += 1
                }
                //end if
            }
            if j == (*wc).numprojectiles {
                crate::src::botlib::be_interface::botimport
                    .Print
                    .expect("non-null function pointer")(
                    3 as i32,
                    b"weapon %s uses undefined projectile in %s\n\x00" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    (*(*wc).weaponinfo.offset(i as isize)).name.as_mut_ptr(),
                    path.as_mut_ptr(),
                );
                crate::src::botlib::l_memory::FreeMemory(wc as *mut libc::c_void);
                return 0 as *mut weaponconfig_t;
            }
        }
        i += 1
        //end if
    }
    if (*wc).numweapons == 0 {
        crate::src::botlib::be_interface::botimport
            .Print
            .expect("non-null function pointer")(
            2 as i32,
            b"no weapon info loaded\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    crate::src::botlib::be_interface::botimport
        .Print
        .expect("non-null function pointer")(
        1 as i32,
        b"loaded %s\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        path.as_mut_ptr(),
    );
    return wc;
}
//end of the function LoadWeaponConfig
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn WeaponWeightIndex(
    mut wwc: *mut weightconfig_t,
    mut wc: *mut weaponconfig_t,
) -> *mut i32 {
    let mut index: *mut i32 = 0 as *mut i32;
    let mut i: i32 = 0;
    //initialize item weight index
    index = crate::src::botlib::l_memory::GetClearedMemory(
        (::std::mem::size_of::<i32>() as usize)
            .wrapping_mul((*wc).numweapons as usize),
    ) as *mut i32; //end for
    i = 0 as i32;
    while i < (*wc).numweapons {
        *index.offset(i as isize) = FindFuzzyWeight(
            wwc as *mut weightconfig_s,
            (*(*wc).weaponinfo.offset(i as isize)).name.as_mut_ptr(),
        );
        i += 1
    }
    return index;
}
//end of the function WeaponWeightIndex
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotFreeWeaponWeights(mut weaponstate: i32) {
    let mut ws: *mut bot_weaponstate_t = 0 as *mut bot_weaponstate_t;
    ws = BotWeaponStateFromHandle(weaponstate);
    if ws.is_null() {
        return;
    }
    if !(*ws).weaponweightconfig.is_null() {
        FreeWeightConfig((*ws).weaponweightconfig as *mut weightconfig_s);
    }
    if !(*ws).weaponweightindex.is_null() {
        crate::src::botlib::l_memory::FreeMemory((*ws).weaponweightindex as *mut libc::c_void);
    };
}
//loads the weapon weights
//end of the function BotFreeWeaponWeights
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotLoadWeaponWeights(
    mut weaponstate: i32,
    mut filename: *mut libc::c_char,
) -> i32 {
    let mut ws: *mut bot_weaponstate_t = 0 as *mut bot_weaponstate_t;
    ws = BotWeaponStateFromHandle(weaponstate);
    if ws.is_null() {
        return 11 as i32;
    }
    BotFreeWeaponWeights(weaponstate);
    //
    (*ws).weaponweightconfig = ReadWeightConfig(filename) as *mut weightconfig_s; //end if
    if (*ws).weaponweightconfig.is_null() {
        crate::src::botlib::be_interface::botimport
            .Print
            .expect("non-null function pointer")(
            4 as i32,
            b"couldn\'t load weapon config %s\n\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            filename,
        );
        return 11 as i32;
    }
    if weaponconfig.is_null() {
        return 12 as i32;
    }
    (*ws).weaponweightindex = WeaponWeightIndex((*ws).weaponweightconfig, weaponconfig);
    return 0 as i32;
}
//returns the information of the current weapon
//end of the function BotLoadWeaponWeights
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotGetWeaponInfo(
    mut weaponstate: i32,
    mut weapon: i32,
    mut weaponinfo: *mut crate::src::botlib::be_ai_weap::weaponinfo_t,
) {
    let mut ws: *mut bot_weaponstate_t = 0 as *mut bot_weaponstate_t;
    if BotValidWeaponNumber(weapon) == 0 {
        return;
    }
    ws = BotWeaponStateFromHandle(weaponstate);
    if ws.is_null() {
        return;
    }
    if weaponconfig.is_null() {
        return;
    }
    crate::stdlib::memcpy(
        weaponinfo as *mut libc::c_void,
        &mut *(*weaponconfig).weaponinfo.offset(weapon as isize)
            as *mut crate::src::botlib::be_ai_weap::weaponinfo_t as *const libc::c_void,
        ::std::mem::size_of::<crate::src::botlib::be_ai_weap::weaponinfo_t>() as usize,
    );
}
//returns the best weapon to fight with
//end of the function BotGetWeaponInfo
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotChooseBestFightWeapon(
    mut weaponstate: i32,
    mut inventory: *mut i32,
) -> i32 {
    let mut i: i32 = 0;
    let mut index: i32 = 0;
    let mut bestweapon: i32 = 0;
    let mut weight: f32 = 0.;
    let mut bestweight: f32 = 0.;
    let mut wc: *mut weaponconfig_t = 0 as *mut weaponconfig_t;
    let mut ws: *mut bot_weaponstate_t = 0 as *mut bot_weaponstate_t;
    ws = BotWeaponStateFromHandle(weaponstate);
    if ws.is_null() {
        return 0 as i32;
    }
    wc = weaponconfig;
    if weaponconfig.is_null() {
        return 0 as i32;
    }
    //if the bot has no weapon weight configuration
    if (*ws).weaponweightconfig.is_null() {
        return 0 as i32;
    } //end for
    bestweight = 0 as i32 as f32;
    bestweapon = 0 as i32;
    i = 0 as i32;
    while i < (*wc).numweapons {
        if !((*(*wc).weaponinfo.offset(i as isize)).valid == 0) {
            index = *(*ws).weaponweightindex.offset(i as isize);
            if !(index < 0 as i32) {
                weight = FuzzyWeight(
                    inventory,
                    (*ws).weaponweightconfig as *mut weightconfig_s,
                    index,
                );
                if weight > bestweight {
                    bestweight = weight;
                    bestweapon = i
                }
            }
        }
        i += 1
        //end if
    }
    return bestweapon;
}
//resets the whole weapon state
//end of the function BotChooseBestFightWeapon
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotResetWeaponState(mut _weaponstate: i32) {}
//returns a handle to a newly allocated weapon state
//end of the function BotResetWeaponState
//========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotAllocWeaponState() -> i32 {
    let mut i: i32 = 0; //end for
    i = 1 as i32;
    while i <= 64 as i32 {
        if botweaponstates[i as usize].is_null() {
            botweaponstates[i as usize] =
                crate::src::botlib::l_memory::GetClearedMemory(::std::mem::size_of::<
                    bot_weaponstate_t,
                >() as usize) as *mut bot_weaponstate_t;
            return i;
        }
        i += 1
        //end if
    }
    return 0 as i32;
}
//frees the weapon state
//end of the function BotAllocWeaponState
//========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotFreeWeaponState(mut handle: i32) {
    if handle <= 0 as i32 || handle > 64 as i32 {
        crate::src::botlib::be_interface::botimport
            .Print
            .expect("non-null function pointer")(
            4 as i32,
            b"weapon state handle %d out of range\n\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            handle,
        ); //end if
        return;
    } //end if
    if botweaponstates[handle as usize].is_null() {
        crate::src::botlib::be_interface::botimport
            .Print
            .expect("non-null function pointer")(
            4 as i32,
            b"invalid weapon state %d\n\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            handle,
        );
        return;
    }
    BotFreeWeaponWeights(handle);
    crate::src::botlib::l_memory::FreeMemory(botweaponstates[handle as usize] as *mut libc::c_void);
    botweaponstates[handle as usize] = 0 as *mut bot_weaponstate_t;
}
//setup the weapon AI
//end of the function BotFreeWeaponState
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotSetupWeaponAI() -> i32 {
    let mut file: *mut libc::c_char = 0 as *mut libc::c_char; //end if
    file = crate::src::botlib::l_libvar::LibVarString(
        b"weaponconfig\x00" as *const u8 as *const libc::c_char,
        b"weapons.c\x00" as *const u8 as *const libc::c_char,
    );
    weaponconfig = LoadWeaponConfig(file);
    if weaponconfig.is_null() {
        crate::src::botlib::be_interface::botimport
            .Print
            .expect("non-null function pointer")(
            4 as i32,
            b"couldn\'t load the weapon config\n\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return 12 as i32;
    }
    //DEBUG_AI_WEAP
    //
    return 0 as i32;
}
//shut down the weapon AI
//end of the function BotSetupWeaponAI
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotShutdownWeaponAI() {
    let mut i: i32 = 0;
    if !weaponconfig.is_null() {
        crate::src::botlib::l_memory::FreeMemory(weaponconfig as *mut libc::c_void);
    }
    weaponconfig = 0 as *mut weaponconfig_t;
    i = 1 as i32;
    while i <= 64 as i32 {
        if !botweaponstates[i as usize].is_null() {
            BotFreeWeaponState(i);
        }
        i += 1
        //end if
    }
    //end for
}
unsafe extern "C" fn run_static_initializers() {
    weaponinfo_fields = [
        {
            let mut init = fielddef_s {
                name: b"number\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut (*(0 as *mut crate::src::botlib::be_ai_weap::weaponinfo_t)).number
                    as *mut i32 as size_t as i32,
                type_0: 2 as i32,
                maxarray: 0,
                floatmin: 0.,
                floatmax: 0.,
                substruct: 0 as *mut structdef_s,
            };
            init
        },
        {
            let mut init = fielddef_s {
                name: b"name\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut (*(0 as *mut crate::src::botlib::be_ai_weap::weaponinfo_t)).name
                    as *mut [libc::c_char; 80] as size_t as i32,
                type_0: 4 as i32,
                maxarray: 0,
                floatmin: 0.,
                floatmax: 0.,
                substruct: 0 as *mut structdef_s,
            };
            init
        },
        {
            let mut init = fielddef_s {
                name: b"level\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut (*(0 as *mut crate::src::botlib::be_ai_weap::weaponinfo_t)).level
                    as *mut i32 as size_t as i32,
                type_0: 2 as i32,
                maxarray: 0,
                floatmin: 0.,
                floatmax: 0.,
                substruct: 0 as *mut structdef_s,
            };
            init
        },
        {
            let mut init = fielddef_s {
                name: b"model\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut (*(0 as *mut crate::src::botlib::be_ai_weap::weaponinfo_t)).model
                    as *mut [libc::c_char; 80] as size_t as i32,
                type_0: 4 as i32,
                maxarray: 0,
                floatmin: 0.,
                floatmax: 0.,
                substruct: 0 as *mut structdef_s,
            };
            init
        },
        {
            let mut init = fielddef_s {
                name: b"weaponindex\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut (*(0 as *mut crate::src::botlib::be_ai_weap::weaponinfo_t)).weaponindex
                    as *mut i32 as size_t as i32,
                type_0: 2 as i32,
                maxarray: 0,
                floatmin: 0.,
                floatmax: 0.,
                substruct: 0 as *mut structdef_s,
            };
            init
        },
        {
            let mut init = fielddef_s {
                name: b"flags\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut (*(0 as *mut crate::src::botlib::be_ai_weap::weaponinfo_t)).flags
                    as *mut i32 as size_t as i32,
                type_0: 2 as i32,
                maxarray: 0,
                floatmin: 0.,
                floatmax: 0.,
                substruct: 0 as *mut structdef_s,
            };
            init
        },
        {
            let mut init = fielddef_s {
                name: b"projectile\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut (*(0 as *mut crate::src::botlib::be_ai_weap::weaponinfo_t)).projectile
                    as *mut [libc::c_char; 80] as size_t as i32,
                type_0: 4 as i32,
                maxarray: 0,
                floatmin: 0.,
                floatmax: 0.,
                substruct: 0 as *mut structdef_s,
            };
            init
        },
        {
            let mut init = fielddef_s {
                name: b"numprojectiles\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                offset: &mut (*(0 as *mut crate::src::botlib::be_ai_weap::weaponinfo_t))
                    .numprojectiles as *mut i32 as size_t as i32,
                type_0: 2 as i32,
                maxarray: 0,
                floatmin: 0.,
                floatmax: 0.,
                substruct: 0 as *mut structdef_s,
            };
            init
        },
        {
            let mut init = fielddef_s {
                name: b"hspread\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut (*(0 as *mut crate::src::botlib::be_ai_weap::weaponinfo_t)).hspread
                    as *mut f32 as size_t as i32,
                type_0: 3 as i32,
                maxarray: 0,
                floatmin: 0.,
                floatmax: 0.,
                substruct: 0 as *mut structdef_s,
            };
            init
        },
        {
            let mut init = fielddef_s {
                name: b"vspread\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut (*(0 as *mut crate::src::botlib::be_ai_weap::weaponinfo_t)).vspread
                    as *mut f32 as size_t as i32,
                type_0: 3 as i32,
                maxarray: 0,
                floatmin: 0.,
                floatmax: 0.,
                substruct: 0 as *mut structdef_s,
            };
            init
        },
        {
            let mut init = fielddef_s {
                name: b"speed\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut (*(0 as *mut crate::src::botlib::be_ai_weap::weaponinfo_t)).speed
                    as *mut f32 as size_t as i32,
                type_0: 3 as i32,
                maxarray: 0,
                floatmin: 0.,
                floatmax: 0.,
                substruct: 0 as *mut structdef_s,
            };
            init
        },
        {
            let mut init = fielddef_s {
                name: b"acceleration\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut (*(0 as *mut crate::src::botlib::be_ai_weap::weaponinfo_t))
                    .acceleration as *mut f32 as size_t as i32,
                type_0: 3 as i32,
                maxarray: 0,
                floatmin: 0.,
                floatmax: 0.,
                substruct: 0 as *mut structdef_s,
            };
            init
        },
        {
            let mut init = fielddef_s {
                name: b"recoil\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut (*(0 as *mut crate::src::botlib::be_ai_weap::weaponinfo_t)).recoil
                    as *mut vec3_t as size_t as i32,
                type_0: 3 as i32 | 0x100 as i32,
                maxarray: 3 as i32,
                floatmin: 0.,
                floatmax: 0.,
                substruct: 0 as *mut structdef_s,
            };
            init
        },
        {
            let mut init = fielddef_s {
                name: b"offset\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut (*(0 as *mut crate::src::botlib::be_ai_weap::weaponinfo_t)).offset
                    as *mut vec3_t as size_t as i32,
                type_0: 3 as i32 | 0x100 as i32,
                maxarray: 3 as i32,
                floatmin: 0.,
                floatmax: 0.,
                substruct: 0 as *mut structdef_s,
            };
            init
        },
        {
            let mut init = fielddef_s {
                name: b"angleoffset\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut (*(0 as *mut crate::src::botlib::be_ai_weap::weaponinfo_t)).angleoffset
                    as *mut vec3_t as size_t as i32,
                type_0: 3 as i32 | 0x100 as i32,
                maxarray: 3 as i32,
                floatmin: 0.,
                floatmax: 0.,
                substruct: 0 as *mut structdef_s,
            };
            init
        },
        {
            let mut init = fielddef_s {
                name: b"extrazvelocity\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                offset: &mut (*(0 as *mut crate::src::botlib::be_ai_weap::weaponinfo_t))
                    .extrazvelocity as *mut f32 as size_t as i32,
                type_0: 3 as i32,
                maxarray: 0,
                floatmin: 0.,
                floatmax: 0.,
                substruct: 0 as *mut structdef_s,
            };
            init
        },
        {
            let mut init = fielddef_s {
                name: b"ammoamount\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut (*(0 as *mut crate::src::botlib::be_ai_weap::weaponinfo_t)).ammoamount
                    as *mut i32 as size_t as i32,
                type_0: 2 as i32,
                maxarray: 0,
                floatmin: 0.,
                floatmax: 0.,
                substruct: 0 as *mut structdef_s,
            };
            init
        },
        {
            let mut init = fielddef_s {
                name: b"ammoindex\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut (*(0 as *mut crate::src::botlib::be_ai_weap::weaponinfo_t)).ammoindex
                    as *mut i32 as size_t as i32,
                type_0: 2 as i32,
                maxarray: 0,
                floatmin: 0.,
                floatmax: 0.,
                substruct: 0 as *mut structdef_s,
            };
            init
        },
        {
            let mut init = fielddef_s {
                name: b"activate\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut (*(0 as *mut crate::src::botlib::be_ai_weap::weaponinfo_t)).activate
                    as *mut f32 as size_t as i32,
                type_0: 3 as i32,
                maxarray: 0,
                floatmin: 0.,
                floatmax: 0.,
                substruct: 0 as *mut structdef_s,
            };
            init
        },
        {
            let mut init = fielddef_s {
                name: b"reload\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut (*(0 as *mut crate::src::botlib::be_ai_weap::weaponinfo_t)).reload
                    as *mut f32 as size_t as i32,
                type_0: 3 as i32,
                maxarray: 0,
                floatmin: 0.,
                floatmax: 0.,
                substruct: 0 as *mut structdef_s,
            };
            init
        },
        {
            let mut init = fielddef_s {
                name: b"spinup\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut (*(0 as *mut crate::src::botlib::be_ai_weap::weaponinfo_t)).spinup
                    as *mut f32 as size_t as i32,
                type_0: 3 as i32,
                maxarray: 0,
                floatmin: 0.,
                floatmax: 0.,
                substruct: 0 as *mut structdef_s,
            };
            init
        },
        {
            let mut init = fielddef_s {
                name: b"spindown\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut (*(0 as *mut crate::src::botlib::be_ai_weap::weaponinfo_t)).spindown
                    as *mut f32 as size_t as i32,
                type_0: 3 as i32,
                maxarray: 0,
                floatmin: 0.,
                floatmax: 0.,
                substruct: 0 as *mut structdef_s,
            };
            init
        },
        {
            let mut init = fielddef_s {
                name: 0 as *mut libc::c_char,
                offset: 0 as i32,
                type_0: 0 as i32,
                maxarray: 0 as i32,
                floatmin: 0.,
                floatmax: 0.,
                substruct: 0 as *mut structdef_s,
            };
            init
        },
    ];
    projectileinfo_fields = [
        {
            let mut init = fielddef_s {
                name: b"name\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut (*(0 as *mut crate::src::botlib::be_ai_weap::projectileinfo_t)).name
                    as *mut [libc::c_char; 80] as size_t as i32,
                type_0: 4 as i32,
                maxarray: 0,
                floatmin: 0.,
                floatmax: 0.,
                substruct: 0 as *mut structdef_s,
            };
            init
        },
        {
            let mut init = fielddef_s {
                name: b"model\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut (*(0 as *mut crate::src::botlib::be_ai_weap::projectileinfo_t)).model
                    as *mut [libc::c_char; 80] as size_t as i32,
                type_0: 4 as i32,
                maxarray: 0,
                floatmin: 0.,
                floatmax: 0.,
                substruct: 0 as *mut structdef_s,
            };
            init
        },
        {
            let mut init = fielddef_s {
                name: b"flags\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut (*(0 as *mut crate::src::botlib::be_ai_weap::projectileinfo_t)).flags
                    as *mut i32 as size_t as i32,
                type_0: 2 as i32,
                maxarray: 0,
                floatmin: 0.,
                floatmax: 0.,
                substruct: 0 as *mut structdef_s,
            };
            init
        },
        {
            let mut init = fielddef_s {
                name: b"gravity\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut (*(0 as *mut crate::src::botlib::be_ai_weap::projectileinfo_t)).gravity
                    as *mut f32 as size_t as i32,
                type_0: 3 as i32,
                maxarray: 0,
                floatmin: 0.,
                floatmax: 0.,
                substruct: 0 as *mut structdef_s,
            };
            init
        },
        {
            let mut init = fielddef_s {
                name: b"damage\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut (*(0 as *mut crate::src::botlib::be_ai_weap::projectileinfo_t)).damage
                    as *mut i32 as size_t as i32,
                type_0: 2 as i32,
                maxarray: 0,
                floatmin: 0.,
                floatmax: 0.,
                substruct: 0 as *mut structdef_s,
            };
            init
        },
        {
            let mut init = fielddef_s {
                name: b"radius\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut (*(0 as *mut crate::src::botlib::be_ai_weap::projectileinfo_t)).radius
                    as *mut f32 as size_t as i32,
                type_0: 3 as i32,
                maxarray: 0,
                floatmin: 0.,
                floatmax: 0.,
                substruct: 0 as *mut structdef_s,
            };
            init
        },
        {
            let mut init = fielddef_s {
                name: b"visdamage\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut (*(0 as *mut crate::src::botlib::be_ai_weap::projectileinfo_t))
                    .visdamage as *mut i32 as size_t as i32,
                type_0: 2 as i32,
                maxarray: 0,
                floatmin: 0.,
                floatmax: 0.,
                substruct: 0 as *mut structdef_s,
            };
            init
        },
        {
            let mut init = fielddef_s {
                name: b"damagetype\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut (*(0 as *mut crate::src::botlib::be_ai_weap::projectileinfo_t))
                    .damagetype as *mut i32 as size_t as i32,
                type_0: 2 as i32,
                maxarray: 0,
                floatmin: 0.,
                floatmax: 0.,
                substruct: 0 as *mut structdef_s,
            };
            init
        },
        {
            let mut init = fielddef_s {
                name: b"healthinc\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut (*(0 as *mut crate::src::botlib::be_ai_weap::projectileinfo_t))
                    .healthinc as *mut i32 as size_t as i32,
                type_0: 2 as i32,
                maxarray: 0,
                floatmin: 0.,
                floatmax: 0.,
                substruct: 0 as *mut structdef_s,
            };
            init
        },
        {
            let mut init = fielddef_s {
                name: b"push\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut (*(0 as *mut crate::src::botlib::be_ai_weap::projectileinfo_t)).push
                    as *mut f32 as size_t as i32,
                type_0: 3 as i32,
                maxarray: 0,
                floatmin: 0.,
                floatmax: 0.,
                substruct: 0 as *mut structdef_s,
            };
            init
        },
        {
            let mut init = fielddef_s {
                name: b"detonation\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut (*(0 as *mut crate::src::botlib::be_ai_weap::projectileinfo_t))
                    .detonation as *mut f32 as size_t as i32,
                type_0: 3 as i32,
                maxarray: 0,
                floatmin: 0.,
                floatmax: 0.,
                substruct: 0 as *mut structdef_s,
            };
            init
        },
        {
            let mut init = fielddef_s {
                name: b"bounce\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut (*(0 as *mut crate::src::botlib::be_ai_weap::projectileinfo_t)).bounce
                    as *mut f32 as size_t as i32,
                type_0: 3 as i32,
                maxarray: 0,
                floatmin: 0.,
                floatmax: 0.,
                substruct: 0 as *mut structdef_s,
            };
            init
        },
        {
            let mut init = fielddef_s {
                name: b"bouncefric\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut (*(0 as *mut crate::src::botlib::be_ai_weap::projectileinfo_t))
                    .bouncefric as *mut f32 as size_t as i32,
                type_0: 3 as i32,
                maxarray: 0,
                floatmin: 0.,
                floatmax: 0.,
                substruct: 0 as *mut structdef_s,
            };
            init
        },
        {
            let mut init = fielddef_s {
                name: b"bouncestop\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut (*(0 as *mut crate::src::botlib::be_ai_weap::projectileinfo_t))
                    .bouncestop as *mut f32 as size_t as i32,
                type_0: 3 as i32,
                maxarray: 0,
                floatmin: 0.,
                floatmax: 0.,
                substruct: 0 as *mut structdef_s,
            };
            init
        },
        {
            let mut init = fielddef_s {
                name: 0 as *mut libc::c_char,
                offset: 0 as i32,
                type_0: 0 as i32,
                maxarray: 0 as i32,
                floatmin: 0.,
                floatmax: 0.,
                substruct: 0 as *mut structdef_s,
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
//end of the function BotShutdownWeaponAI

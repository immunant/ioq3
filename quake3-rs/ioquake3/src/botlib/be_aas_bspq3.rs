use ::libc;

pub mod stdlib_float_h {
    #[inline]

    pub unsafe extern "C" fn atof(mut __nptr: *const libc::c_char) -> f64 {
        return ::libc::strtod(__nptr, 0 as *mut libc::c_void as *mut *mut libc::c_char);
    }
}

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

pub use crate::be_aas_def_h::bsp_link_s;
pub use crate::be_aas_def_h::bsp_link_t;
pub use crate::botlib_h::botlib_import_s;
pub use crate::botlib_h::botlib_import_t;
pub use crate::botlib_h::bsp_surface_s;
pub use crate::botlib_h::bsp_surface_t;
pub use crate::botlib_h::bsp_trace_s;
pub use crate::botlib_h::bsp_trace_t;
pub use crate::src::botlib::be_aas_bspq3::stdlib_float_h::atof;
pub use crate::src::botlib::l_script::punctuation_s;
pub use crate::src::botlib::l_script::punctuation_t;
pub use crate::src::botlib::l_script::script_s;
pub use crate::src::botlib::l_script::script_t;
pub use crate::src::botlib::l_script::token_s;
pub use crate::src::botlib::l_script::token_t;
pub use crate::src::botlib::l_script::FreeScript;
pub use crate::src::botlib::l_script::LoadScriptMemory;
pub use crate::src::botlib::l_script::PS_ExpectTokenType;
pub use crate::src::botlib::l_script::PS_ReadToken;
pub use crate::src::botlib::l_script::ScriptError;
pub use crate::src::botlib::l_script::SetScriptFlags;
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

pub use crate::src::botlib::be_aas_bspq3::stdlib_h::atoi;

pub use ::libc::strtod;
pub use ::libc::strtol;
extern "C" {
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
     * name:		be_aas_bspq3.c
     *
     * desc:		BSP, Environment Sampling
     *
     * $Archive: /MissionPack/code/botlib/be_aas_bspq3.c $
     *
     *****************************************************************************/
    #[no_mangle]
    pub static mut botimport: crate::botlib_h::botlib_import_t;
}
//id Software BSP data

pub type bsp_t = bsp_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bsp_s {
    pub loaded: i32,
    pub entdatasize: i32,
    pub dentdata: *mut libc::c_char,
    pub numentities: i32,
    pub entities: [bsp_entity_t; 2048],
}
//true when bsp file is loaded
//entity data
//bsp entities
//bsp data entity

pub type bsp_entity_t = bsp_entity_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bsp_entity_s {
    pub epairs: *mut bsp_epair_t,
}
//bsp entity epair

pub type bsp_epair_t = bsp_epair_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bsp_epair_s {
    pub key: *mut libc::c_char,
    pub value: *mut libc::c_char,
    pub next: *mut bsp_epair_s,
}
//global bsp
#[no_mangle]

pub static mut bspworld: bsp_t = bsp_t {
    loaded: 0,
    entdatasize: 0,
    dentdata: 0 as *const libc::c_char as *mut libc::c_char,
    numentities: 0,
    entities: [bsp_entity_t {
        epairs: 0 as *const bsp_epair_t as *mut bsp_epair_t,
    }; 2048],
};
// BSP_DEBUG
//===========================================================================
// traces axial boxes of any size through the world
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_Trace(
    mut start: *mut crate::src::qcommon::q_shared::vec_t,
    mut mins: *mut crate::src::qcommon::q_shared::vec_t,
    mut maxs: *mut crate::src::qcommon::q_shared::vec_t,
    mut end: *mut crate::src::qcommon::q_shared::vec_t,
    mut passent: i32,
    mut contentmask: i32,
) -> crate::botlib_h::bsp_trace_t {
    let mut bsptrace: crate::botlib_h::bsp_trace_t = crate::botlib_h::bsp_trace_t {
        allsolid: crate::src::qcommon::q_shared::qfalse,
        startsolid: crate::src::qcommon::q_shared::qfalse,
        fraction: 0.,
        endpos: [0.; 3],
        plane: crate::src::qcommon::q_shared::cplane_t {
            normal: [0.; 3],
            dist: 0.,
            type_0: 0,
            signbits: 0,
            pad: [0; 2],
        },
        exp_dist: 0.,
        sidenum: 0,
        surface: crate::botlib_h::bsp_surface_t {
            name: [0; 16],
            flags: 0,
            value: 0,
        },
        contents: 0,
        ent: 0,
    };
    botimport.Trace.expect("non-null function pointer")(
        &mut bsptrace,
        start,
        mins,
        maxs,
        end,
        passent,
        contentmask,
    );
    return bsptrace;
}
//end of the function AAS_Trace
//===========================================================================
// returns the contents at the given point
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_PointContents(
    mut point: *mut crate::src::qcommon::q_shared::vec_t,
) -> i32 {
    return botimport.PointContents.expect("non-null function pointer")(point);
}
//end of the function AAS_PointContents
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_EntityCollision(
    mut entnum: i32,
    mut start: *mut crate::src::qcommon::q_shared::vec_t,
    mut boxmins: *mut crate::src::qcommon::q_shared::vec_t,
    mut boxmaxs: *mut crate::src::qcommon::q_shared::vec_t,
    mut end: *mut crate::src::qcommon::q_shared::vec_t,
    mut contentmask: i32,
    mut trace: *mut crate::botlib_h::bsp_trace_t,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut enttrace: crate::botlib_h::bsp_trace_t = crate::botlib_h::bsp_trace_t {
        allsolid: crate::src::qcommon::q_shared::qfalse,
        startsolid: crate::src::qcommon::q_shared::qfalse,
        fraction: 0.,
        endpos: [0.; 3],
        plane: crate::src::qcommon::q_shared::cplane_t {
            normal: [0.; 3],
            dist: 0.,
            type_0: 0,
            signbits: 0,
            pad: [0; 2],
        },
        exp_dist: 0.,
        sidenum: 0,
        surface: crate::botlib_h::bsp_surface_t {
            name: [0; 16],
            flags: 0,
            value: 0,
        },
        contents: 0,
        ent: 0,
    }; //end if
    botimport.EntityTrace.expect("non-null function pointer")(
        &mut enttrace,
        start,
        boxmins,
        boxmaxs,
        end,
        entnum,
        contentmask,
    );
    if enttrace.fraction < (*trace).fraction {
        crate::stdlib::memcpy(
            trace as *mut libc::c_void,
            &mut enttrace as *mut crate::botlib_h::bsp_trace_t as *const libc::c_void,
            ::std::mem::size_of::<crate::botlib_h::bsp_trace_t>() as libc::c_ulong,
        );
        return crate::src::qcommon::q_shared::qtrue;
    }
    return crate::src::qcommon::q_shared::qfalse;
}
//end of the function AAS_EntityCollision
//===========================================================================
// returns true if in Potentially Hearable Set
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_inPVS(
    mut p1: *mut crate::src::qcommon::q_shared::vec_t,
    mut p2: *mut crate::src::qcommon::q_shared::vec_t,
) -> crate::src::qcommon::q_shared::qboolean {
    return botimport.inPVS.expect("non-null function pointer")(p1, p2)
        as crate::src::qcommon::q_shared::qboolean;
}
//end of the function AAS_InPVS
//===========================================================================
// returns true if in Potentially Visible Set
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_inPHS(
    mut _p1: *mut crate::src::qcommon::q_shared::vec_t,
    mut _p2: *mut crate::src::qcommon::q_shared::vec_t,
) -> crate::src::qcommon::q_shared::qboolean {
    return crate::src::qcommon::q_shared::qtrue;
}
//end of the function AAS_inPHS
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_BSPModelMinsMaxsOrigin(
    mut modelnum: i32,
    mut angles: *mut crate::src::qcommon::q_shared::vec_t,
    mut mins: *mut crate::src::qcommon::q_shared::vec_t,
    mut maxs: *mut crate::src::qcommon::q_shared::vec_t,
    mut origin: *mut crate::src::qcommon::q_shared::vec_t,
) {
    botimport
        .BSPModelMinsMaxsOrigin
        .expect("non-null function pointer")(modelnum, angles, mins, maxs, origin);
}
//end of the function AAS_BSPModelMinsMaxs
//===========================================================================
// unlinks the entity from all leaves
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_UnlinkFromBSPLeaves(
    mut _leaves: *mut crate::be_aas_def_h::bsp_link_t,
) {
}
//end of the function AAS_UnlinkFromBSPLeaves
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_BSPLinkEntity(
    mut _absmins: *mut crate::src::qcommon::q_shared::vec_t,
    mut _absmaxs: *mut crate::src::qcommon::q_shared::vec_t,
    mut _entnum: i32,
    mut _modelnum: i32,
) -> *mut crate::be_aas_def_h::bsp_link_t {
    return 0 as *mut crate::be_aas_def_h::bsp_link_t;
}
//end of the function AAS_BSPLinkEntity
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_BoxEntities(
    mut _absmins: *mut crate::src::qcommon::q_shared::vec_t,
    mut _absmaxs: *mut crate::src::qcommon::q_shared::vec_t,
    mut _list: *mut i32,
    mut _maxcount: i32,
) -> i32 {
    return 0 as i32;
}
//end of the function AAS_BoxEntities
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_NextBSPEntity(mut ent: i32) -> i32 {
    ent += 1;
    if ent >= 1 as i32 && ent < bspworld.numentities {
        return ent;
    }
    return 0 as i32;
}
//end of the function AAS_NextBSPEntity
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_BSPEntityInRange(mut ent: i32) -> i32 {
    if ent <= 0 as i32 || ent >= bspworld.numentities {
        botimport.Print.expect("non-null function pointer")(
            1 as i32,
            b"bsp entity out of range\n\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        ); //end if
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    return crate::src::qcommon::q_shared::qtrue as i32;
}
//end of the function AAS_BSPEntityInRange
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_ValueForBSPEpairKey(
    mut ent: i32,
    mut key: *mut libc::c_char,
    mut value: *mut libc::c_char,
    mut size: i32,
) -> i32 {
    let mut epair: *mut bsp_epair_t = 0 as *mut bsp_epair_t; //end for
    *value.offset(0 as i32 as isize) = '\u{0}' as i32 as libc::c_char;
    if AAS_BSPEntityInRange(ent) == 0 {
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    epair = bspworld.entities[ent as usize].epairs;
    while !epair.is_null() {
        if ::libc::strcmp((*epair).key, key) == 0 {
            crate::stdlib::strncpy(
                value,
                (*epair).value,
                (size - 1 as i32) as libc::c_ulong,
            );
            *value.offset((size - 1 as i32) as isize) = '\u{0}' as i32 as libc::c_char;
            return crate::src::qcommon::q_shared::qtrue as i32;
        }
        epair = (*epair).next
        //end if
    }
    return crate::src::qcommon::q_shared::qfalse as i32;
}
//end of the function AAS_FindBSPEpair
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_VectorForBSPEpairKey(
    mut ent: i32,
    mut key: *mut libc::c_char,
    mut v: *mut crate::src::qcommon::q_shared::vec_t,
) -> i32 {
    let mut buf: [libc::c_char; 128] = [0; 128];
    let mut v1: f64 = 0.;
    let mut v2: f64 = 0.;
    let mut v3: f64 = 0.;
    let ref mut fresh0 = *v.offset(2 as i32 as isize);
    *fresh0 = 0 as i32 as crate::src::qcommon::q_shared::vec_t;
    let ref mut fresh1 = *v.offset(1 as i32 as isize);
    *fresh1 = *fresh0;
    *v.offset(0 as i32 as isize) = *fresh1;
    if AAS_ValueForBSPEpairKey(ent, key, buf.as_mut_ptr(), 128 as i32) == 0 {
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    //scanf into doubles, then assign, so it is vec_t size independent
    v3 = 0 as i32 as f64;
    v2 = v3;
    v1 = v2;
    ::libc::sscanf(
        buf.as_mut_ptr(),
        b"%lf %lf %lf\x00" as *const u8 as *const libc::c_char,
        &mut v1 as *mut f64,
        &mut v2 as *mut f64,
        &mut v3 as *mut f64,
    );
    *v.offset(0 as i32 as isize) = v1 as crate::src::qcommon::q_shared::vec_t;
    *v.offset(1 as i32 as isize) = v2 as crate::src::qcommon::q_shared::vec_t;
    *v.offset(2 as i32 as isize) = v3 as crate::src::qcommon::q_shared::vec_t;
    return crate::src::qcommon::q_shared::qtrue as i32;
}
//end of the function AAS_VectorForBSPEpairKey
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_FloatForBSPEpairKey(
    mut ent: i32,
    mut key: *mut libc::c_char,
    mut value: *mut f32,
) -> i32 {
    let mut buf: [libc::c_char; 128] = [0; 128];
    *value = 0 as i32 as f32;
    if AAS_ValueForBSPEpairKey(ent, key, buf.as_mut_ptr(), 128 as i32) == 0 {
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    *value = atof(buf.as_mut_ptr()) as f32;
    return crate::src::qcommon::q_shared::qtrue as i32;
}
//unlink the given entity from the bsp tree leaves
//link the given entity to the bsp tree leaves of the given model
//calculates collision with given entity
//for debugging
//
//AASINTERN
//AASINTERN
//trace through the world
//trace through the world
//returns the contents at the given point
//returns the contents at the given point
//returns true when p2 is in the PVS of p1
//returns true when p2 is in the PVS of p1
//returns true when p2 is in the PHS of p1
//returns true when p2 is in the PHS of p1
//returns true if the given areas are connected
//returns true if the given areas are connected
//creates a list with entities totally or partly within the given box
//creates a list with entities totally or partly within the given box
//gets the mins, maxs and origin of a BSP model
//gets the mins, maxs and origin of a BSP model
//handle to the next bsp entity
//handle to the next bsp entity
//return the value of the BSP epair key
//return the value of the BSP epair key
//get a vector for the BSP epair key
//get a vector for the BSP epair key
//get a float for the BSP epair key
//get a float for the BSP epair key
//get an integer for the BSP epair key
//get an integer for the BSP epair key
//end of the function AAS_FloatForBSPEpairKey
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_IntForBSPEpairKey(
    mut ent: i32,
    mut key: *mut libc::c_char,
    mut value: *mut i32,
) -> i32 {
    let mut buf: [libc::c_char; 128] = [0; 128];
    *value = 0 as i32;
    if AAS_ValueForBSPEpairKey(ent, key, buf.as_mut_ptr(), 128 as i32) == 0 {
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    *value = atoi(buf.as_mut_ptr());
    return crate::src::qcommon::q_shared::qtrue as i32;
}
//end of the function AAS_IntForBSPEpairKey
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_FreeBSPEntities() {
    let mut i: i32 = 0; //end for
    let mut ent: *mut bsp_entity_t = 0 as *mut bsp_entity_t;
    let mut epair: *mut bsp_epair_t = 0 as *mut bsp_epair_t;
    let mut nextepair: *mut bsp_epair_t = 0 as *mut bsp_epair_t;
    i = 1 as i32;
    while i < bspworld.numentities {
        ent = &mut *bspworld.entities.as_mut_ptr().offset(i as isize) as *mut bsp_entity_t;
        epair = (*ent).epairs;
        while !epair.is_null() {
            nextepair = (*epair).next;
            //end for
            //
            if !(*epair).key.is_null() {
                crate::src::botlib::l_memory::FreeMemory((*epair).key as *mut libc::c_void);
            }
            if !(*epair).value.is_null() {
                crate::src::botlib::l_memory::FreeMemory((*epair).value as *mut libc::c_void);
            }
            crate::src::botlib::l_memory::FreeMemory(epair as *mut libc::c_void);
            epair = nextepair
        }
        i += 1
    }
    bspworld.numentities = 0 as i32;
}
//end of the function AAS_FreeBSPEntities
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_ParseBSPEntities() {
    let mut script: *mut crate::src::botlib::l_script::script_t =
        0 as *mut crate::src::botlib::l_script::script_t; //SCFL_PRIMITIVE);
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
    let mut ent: *mut bsp_entity_t = 0 as *mut bsp_entity_t;
    let mut epair: *mut bsp_epair_t = 0 as *mut bsp_epair_t;
    script = crate::src::botlib::l_script::LoadScriptMemory(
        bspworld.dentdata,
        bspworld.entdatasize,
        b"entdata\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut crate::src::botlib::l_script::script_s;
    crate::src::botlib::l_script::SetScriptFlags(
        script as *mut crate::src::botlib::l_script::script_s,
        0x4 as i32 | 0x8 as i32,
    );
    bspworld.numentities = 1 as i32;
    //end if
    while crate::src::botlib::l_script::PS_ReadToken(
        script as *mut crate::src::botlib::l_script::script_s,
        &mut token as *mut _ as *mut crate::src::botlib::l_script::token_s,
    ) != 0
    {
        if ::libc::strcmp(
            token.string.as_mut_ptr(),
            b"{\x00" as *const u8 as *const libc::c_char,
        ) != 0
        {
            crate::src::botlib::l_script::ScriptError(
                script as *mut crate::src::botlib::l_script::script_s,
                b"invalid %s\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                token.string.as_mut_ptr(),
            ); //end while
            AAS_FreeBSPEntities(); //end if
            crate::src::botlib::l_script::FreeScript(
                script as *mut crate::src::botlib::l_script::script_s,
            ); //end if
            return;
        } //end while
        if bspworld.numentities >= 2048 as i32 {
            botimport.Print.expect("non-null function pointer")(
                1 as i32,
                b"too many entities in BSP file\n\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            ); //end if
            break; //end if
        } else {
            ent = &mut *bspworld
                .entities
                .as_mut_ptr()
                .offset(bspworld.numentities as isize) as *mut bsp_entity_t;
            bspworld.numentities += 1;
            (*ent).epairs = 0 as *mut bsp_epair_t;
            while crate::src::botlib::l_script::PS_ReadToken(
                script as *mut crate::src::botlib::l_script::script_s,
                &mut token as *mut _ as *mut crate::src::botlib::l_script::token_s,
            ) != 0
            {
                if ::libc::strcmp(
                    token.string.as_mut_ptr(),
                    b"}\x00" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    break;
                }
                epair = crate::src::botlib::l_memory::GetClearedHunkMemory(::std::mem::size_of::<
                    bsp_epair_t,
                >()
                    as libc::c_ulong) as *mut bsp_epair_t;
                (*epair).next = (*ent).epairs;
                (*ent).epairs = epair;
                if token.type_0 != 1 as i32 {
                    crate::src::botlib::l_script::ScriptError(
                        script as *mut crate::src::botlib::l_script::script_s,
                        b"invalid %s\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        token.string.as_mut_ptr(),
                    );
                    AAS_FreeBSPEntities();
                    crate::src::botlib::l_script::FreeScript(
                        script as *mut crate::src::botlib::l_script::script_s,
                    );
                    return;
                }
                crate::src::botlib::l_script::StripDoubleQuotes(token.string.as_mut_ptr());
                (*epair).key = crate::src::botlib::l_memory::GetHunkMemory(
                    crate::stdlib::strlen(token.string.as_mut_ptr())
                        .wrapping_add(1 as i32 as libc::c_ulong),
                ) as *mut libc::c_char;
                ::libc::strcpy((*epair).key, token.string.as_mut_ptr());
                if crate::src::botlib::l_script::PS_ExpectTokenType(
                    script as *mut crate::src::botlib::l_script::script_s,
                    1 as i32,
                    0 as i32,
                    &mut token as *mut _ as *mut crate::src::botlib::l_script::token_s,
                ) == 0
                {
                    AAS_FreeBSPEntities();
                    crate::src::botlib::l_script::FreeScript(
                        script as *mut crate::src::botlib::l_script::script_s,
                    );
                    return;
                }
                crate::src::botlib::l_script::StripDoubleQuotes(token.string.as_mut_ptr());
                (*epair).value = crate::src::botlib::l_memory::GetHunkMemory(
                    crate::stdlib::strlen(token.string.as_mut_ptr())
                        .wrapping_add(1 as i32 as libc::c_ulong),
                ) as *mut libc::c_char;
                ::libc::strcpy((*epair).value, token.string.as_mut_ptr());
            }
            if ::libc::strcmp(
                token.string.as_mut_ptr(),
                b"}\x00" as *const u8 as *const libc::c_char,
            ) != 0
            {
                crate::src::botlib::l_script::ScriptError(
                    script as *mut crate::src::botlib::l_script::script_s,
                    b"missing }\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
                AAS_FreeBSPEntities();
                crate::src::botlib::l_script::FreeScript(
                    script as *mut crate::src::botlib::l_script::script_s,
                );
                return;
            }
        }
    }
    crate::src::botlib::l_script::FreeScript(script as *mut crate::src::botlib::l_script::script_s);
}
//end of the function AAS_ParseBSPEntities
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_BSPTraceLight(
    mut _start: *mut crate::src::qcommon::q_shared::vec_t,
    mut _end: *mut crate::src::qcommon::q_shared::vec_t,
    mut _endpos: *mut crate::src::qcommon::q_shared::vec_t,
    mut _red: *mut i32,
    mut _green: *mut i32,
    mut _blue: *mut i32,
) -> i32 {
    return 0 as i32;
}
//dump the loaded BSP data
//end of the function AAS_BSPTraceLight
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_DumpBSPData() {
    AAS_FreeBSPEntities();
    if !bspworld.dentdata.is_null() {
        crate::src::botlib::l_memory::FreeMemory(bspworld.dentdata as *mut libc::c_void);
    }
    bspworld.dentdata = 0 as *mut libc::c_char;
    bspworld.entdatasize = 0 as i32;
    //
    bspworld.loaded = crate::src::qcommon::q_shared::qfalse as i32;
    crate::stdlib::memset(
        &mut bspworld as *mut bsp_t as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<bsp_t>() as libc::c_ulong,
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
 * name:		be_aas_bsp.h
 *
 * desc:		AAS
 *
 * $Archive: /source/code/botlib/be_aas_bsp.h $
 *
 *****************************************************************************/
/* ****************************************************************************
 * name:		be_aas_bsp.h
 *
 * desc:		AAS
 *
 * $Archive: /source/code/botlib/be_aas_bsp.h $
 *
 *****************************************************************************/
//loads the given BSP file
//end of the function AAS_DumpBSPData
//===========================================================================
// load a .bsp file
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_LoadBSPFile() -> i32 {
    AAS_DumpBSPData();
    bspworld.entdatasize =
        crate::stdlib::strlen(botimport.BSPEntityData.expect("non-null function pointer")())
            .wrapping_add(1 as i32 as libc::c_ulong) as i32;
    bspworld.dentdata =
        crate::src::botlib::l_memory::GetClearedHunkMemory(bspworld.entdatasize as libc::c_ulong)
            as *mut libc::c_char;
    crate::stdlib::memcpy(
        bspworld.dentdata as *mut libc::c_void,
        botimport.BSPEntityData.expect("non-null function pointer")() as *const libc::c_void,
        bspworld.entdatasize as libc::c_ulong,
    );
    AAS_ParseBSPEntities();
    bspworld.loaded = crate::src::qcommon::q_shared::qtrue as i32;
    return 0 as i32;
}
//end of the function AAS_LoadBSPFile

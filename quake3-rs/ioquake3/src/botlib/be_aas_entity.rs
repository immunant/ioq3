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
    #[inline]

    pub unsafe extern "C" fn VectorCompare(
        mut v1: *const crate::src::qcommon::q_shared::vec_t,
        mut v2: *const crate::src::qcommon::q_shared::vec_t,
    ) -> i32 {
        if *v1.offset(0 as i32 as isize) != *v2.offset(0 as i32 as isize)
            || *v1.offset(1 as i32 as isize) != *v2.offset(1 as i32 as isize)
            || *v1.offset(2 as i32 as isize) != *v2.offset(2 as i32 as isize)
        {
            return 0 as i32;
        }
        return 1 as i32;
    }

    // __Q_SHARED_H
}

pub use crate::aasfile_h::aas_area_s;
pub use crate::aasfile_h::aas_area_t;
pub use crate::aasfile_h::aas_areasettings_s;
pub use crate::aasfile_h::aas_areasettings_t;
pub use crate::aasfile_h::aas_bbox_s;
pub use crate::aasfile_h::aas_bbox_t;
pub use crate::aasfile_h::aas_cluster_s;
pub use crate::aasfile_h::aas_cluster_t;
pub use crate::aasfile_h::aas_edge_s;
pub use crate::aasfile_h::aas_edge_t;
pub use crate::aasfile_h::aas_edgeindex_t;
pub use crate::aasfile_h::aas_face_s;
pub use crate::aasfile_h::aas_face_t;
pub use crate::aasfile_h::aas_faceindex_t;
pub use crate::aasfile_h::aas_node_s;
pub use crate::aasfile_h::aas_node_t;
pub use crate::aasfile_h::aas_plane_s;
pub use crate::aasfile_h::aas_plane_t;
pub use crate::aasfile_h::aas_portal_s;
pub use crate::aasfile_h::aas_portal_t;
pub use crate::aasfile_h::aas_portalindex_t;
pub use crate::aasfile_h::aas_reachability_s;
pub use crate::aasfile_h::aas_reachability_t;
pub use crate::aasfile_h::aas_vertex_t;
pub use crate::be_aas_def_h::aas_entity_s;
pub use crate::be_aas_def_h::aas_entity_t;
pub use crate::be_aas_def_h::aas_link_s;
pub use crate::be_aas_def_h::aas_link_t;
pub use crate::be_aas_def_h::aas_reachabilityareas_s;
pub use crate::be_aas_def_h::aas_reachabilityareas_t;
pub use crate::be_aas_def_h::aas_reversedlink_s;
pub use crate::be_aas_def_h::aas_reversedlink_t;
pub use crate::be_aas_def_h::aas_reversedreachability_s;
pub use crate::be_aas_def_h::aas_reversedreachability_t;
pub use crate::be_aas_def_h::aas_routingcache_s;
pub use crate::be_aas_def_h::aas_routingcache_t;
pub use crate::be_aas_def_h::aas_routingupdate_s;
pub use crate::be_aas_def_h::aas_routingupdate_t;
pub use crate::be_aas_def_h::aas_s;
pub use crate::be_aas_def_h::aas_t;
pub use crate::be_aas_def_h::bsp_entdata_s;
pub use crate::be_aas_def_h::bsp_entdata_t;
pub use crate::be_aas_def_h::bsp_link_s;
pub use crate::be_aas_def_h::bsp_link_t;
pub use crate::be_aas_h::aas_entityinfo_s;
pub use crate::be_aas_h::aas_entityinfo_t;
pub use crate::be_aas_h::C2RustUnnamed_0;
pub use crate::be_aas_h::SOLID_BBOX;
pub use crate::be_aas_h::SOLID_BSP;
pub use crate::be_aas_h::SOLID_NOT;
pub use crate::be_aas_h::SOLID_TRIGGER;
pub use crate::botlib_h::bot_entitystate_s;
pub use crate::botlib_h::bot_entitystate_t;
pub use crate::botlib_h::botlib_import_s;
pub use crate::botlib_h::botlib_import_t;
pub use crate::botlib_h::bsp_surface_s;
pub use crate::botlib_h::bsp_surface_t;
pub use crate::botlib_h::bsp_trace_s;
pub use crate::botlib_h::bsp_trace_t;
pub use crate::src::botlib::be_aas_entity::q_shared_h::VectorCompare;
pub use crate::src::botlib::be_aas_entity::q_shared_h::VectorLength;

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

pub const ET_MOVER: C2RustUnnamed_1 = 4;
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
 * name:		be_aas_entity.c
 *
 * desc:		AAS entities
 *
 * $Archive: /MissionPack/code/botlib/be_aas_entity.c $
 *
 *****************************************************************************/
//FIXME: these might change

pub type C2RustUnnamed_1 = u32;

pub const ET_MISSILE: C2RustUnnamed_1 = 3;

pub const ET_ITEM: C2RustUnnamed_1 = 2;

pub const ET_PLAYER: C2RustUnnamed_1 = 1;

pub const ET_GENERAL: C2RustUnnamed_1 = 0;
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_UpdateEntity(
    mut entnum: i32,
    mut state: *mut bot_entitystate_t,
) -> i32 {
    let mut relink: i32 = 0; //end if
    let mut ent: *mut aas_entity_t = 0 as *mut aas_entity_t;
    let mut absmins: vec3_t = [0.; 3];
    let mut absmaxs: vec3_t = [0.; 3];
    if crate::src::botlib::be_aas_main::aasworld.loaded == 0 {
        crate::src::botlib::be_interface::botimport
            .Print
            .expect("non-null function pointer")(
            1 as i32,
            b"AAS_UpdateEntity: not loaded\n\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return 3 as i32;
    }
    ent = &mut *crate::src::botlib::be_aas_main::aasworld
        .entities
        .offset(entnum as isize) as *mut aas_entity_t;
    if state.is_null() {
        //unlink the entity
        crate::src::botlib::be_aas_sample::AAS_UnlinkFromAreas((*ent).areas as *mut aas_link_s);
        //unlink the entity from the BSP leaves
        crate::src::botlib::be_aas_bspq3::AAS_UnlinkFromBSPLeaves((*ent).leaves as *mut bsp_link_s);
        //
        (*ent).areas = 0 as *mut aas_link_t;
        //
        (*ent).leaves = 0 as *mut bsp_link_t;
        return 0 as i32;
    }
    (*ent).i.update_time = crate::src::botlib::be_aas_main::AAS_Time() - (*ent).i.ltime;
    (*ent).i.type_0 = (*state).type_0;
    (*ent).i.flags = (*state).flags;
    (*ent).i.ltime = crate::src::botlib::be_aas_main::AAS_Time();
    (*ent).i.lastvisorigin[0 as i32 as usize] = (*ent).i.origin[0 as i32 as usize];
    (*ent).i.lastvisorigin[1 as i32 as usize] = (*ent).i.origin[1 as i32 as usize];
    (*ent).i.lastvisorigin[2 as i32 as usize] = (*ent).i.origin[2 as i32 as usize];
    (*ent).i.old_origin[0 as i32 as usize] = (*state).old_origin[0 as i32 as usize];
    (*ent).i.old_origin[1 as i32 as usize] = (*state).old_origin[1 as i32 as usize];
    (*ent).i.old_origin[2 as i32 as usize] = (*state).old_origin[2 as i32 as usize];
    (*ent).i.solid = (*state).solid;
    (*ent).i.groundent = (*state).groundent;
    (*ent).i.modelindex = (*state).modelindex;
    (*ent).i.modelindex2 = (*state).modelindex2;
    (*ent).i.frame = (*state).frame;
    (*ent).i.event = (*state).event;
    (*ent).i.eventParm = (*state).eventParm;
    (*ent).i.powerups = (*state).powerups;
    (*ent).i.weapon = (*state).weapon;
    (*ent).i.legsAnim = (*state).legsAnim;
    (*ent).i.torsoAnim = (*state).torsoAnim;
    //number of the entity
    (*ent).i.number = entnum;
    //updated so set valid flag
    (*ent).i.valid = qtrue as i32;
    //link everything the first frame
    if crate::src::botlib::be_aas_main::aasworld.numframes == 1 as i32 {
        relink = qtrue as i32
    } else {
        relink = qfalse as i32
    }
    //
    if (*ent).i.solid == SOLID_BSP as i32 {
        //end if
        if VectorCompare(
            (*state).angles.as_mut_ptr() as *const vec_t,
            (*ent).i.angles.as_mut_ptr() as *const vec_t,
        ) == 0
        {
            (*ent).i.angles[0 as i32 as usize] = (*state).angles[0 as i32 as usize]; //end if
            (*ent).i.angles[1 as i32 as usize] = (*state).angles[1 as i32 as usize];
            (*ent).i.angles[2 as i32 as usize] = (*state).angles[2 as i32 as usize];
            relink = qtrue as i32
        }
        //if the angles of the model changed
        //end if
        //get the mins and maxs of the model
        //FIXME: rotate mins and maxs
        crate::src::botlib::be_aas_bspq3::AAS_BSPModelMinsMaxsOrigin(
            (*ent).i.modelindex,
            (*ent).i.angles.as_mut_ptr(),
            (*ent).i.mins.as_mut_ptr(),
            (*ent).i.maxs.as_mut_ptr(),
            0 as *mut vec_t,
        );
    } else if (*ent).i.solid == SOLID_BBOX as i32 {
        //if the bounding box size changed
        if VectorCompare(
            (*state).mins.as_mut_ptr() as *const vec_t,
            (*ent).i.mins.as_mut_ptr() as *const vec_t,
        ) == 0
            || VectorCompare(
                (*state).maxs.as_mut_ptr() as *const vec_t,
                (*ent).i.maxs.as_mut_ptr() as *const vec_t,
            ) == 0
        {
            (*ent).i.mins[0 as i32 as usize] = (*state).mins[0 as i32 as usize]; //end if
            (*ent).i.mins[1 as i32 as usize] = (*state).mins[1 as i32 as usize];
            (*ent).i.mins[2 as i32 as usize] = (*state).mins[2 as i32 as usize];
            (*ent).i.maxs[0 as i32 as usize] = (*state).maxs[0 as i32 as usize];
            (*ent).i.maxs[1 as i32 as usize] = (*state).maxs[1 as i32 as usize];
            (*ent).i.maxs[2 as i32 as usize] = (*state).maxs[2 as i32 as usize];
            relink = qtrue as i32
        }
        (*ent).i.angles[0 as i32 as usize] = (*state).angles[0 as i32 as usize];
        (*ent).i.angles[1 as i32 as usize] = (*state).angles[1 as i32 as usize];
        (*ent).i.angles[2 as i32 as usize] = (*state).angles[2 as i32 as usize]
    }
    //if the origin changed
    if VectorCompare(
        (*state).origin.as_mut_ptr() as *const vec_t,
        (*ent).i.origin.as_mut_ptr() as *const vec_t,
    ) == 0
    {
        (*ent).i.origin[0 as i32 as usize] = (*state).origin[0 as i32 as usize]; //end if
        (*ent).i.origin[1 as i32 as usize] = (*state).origin[1 as i32 as usize];
        (*ent).i.origin[2 as i32 as usize] = (*state).origin[2 as i32 as usize];
        relink = qtrue as i32
    }
    //if the entity should be relinked
    if relink != 0 {
        //end if
        //don't link the world model
        if entnum != ((1 as i32) << 10 as i32) - 2 as i32 {
            //absolute mins and maxs
            absmins[0 as i32 as usize] =
                (*ent).i.mins[0 as i32 as usize] + (*ent).i.origin[0 as i32 as usize];
            absmins[1 as i32 as usize] =
                (*ent).i.mins[1 as i32 as usize] + (*ent).i.origin[1 as i32 as usize];
            absmins[2 as i32 as usize] =
                (*ent).i.mins[2 as i32 as usize] + (*ent).i.origin[2 as i32 as usize];
            absmaxs[0 as i32 as usize] =
                (*ent).i.maxs[0 as i32 as usize] + (*ent).i.origin[0 as i32 as usize];
            absmaxs[1 as i32 as usize] =
                (*ent).i.maxs[1 as i32 as usize] + (*ent).i.origin[1 as i32 as usize];
            absmaxs[2 as i32 as usize] =
                (*ent).i.maxs[2 as i32 as usize] + (*ent).i.origin[2 as i32 as usize];
            //unlink the entity
            crate::src::botlib::be_aas_sample::AAS_UnlinkFromAreas((*ent).areas as *mut aas_link_s);
            //relink the entity to the AAS areas (use the larges bbox)
            (*ent).areas = crate::src::botlib::be_aas_sample::AAS_LinkEntityClientBBox(
                absmins.as_mut_ptr(),
                absmaxs.as_mut_ptr(),
                entnum,
                2 as i32,
            ) as *mut aas_link_s;
            //unlink the entity from the BSP leaves
            crate::src::botlib::be_aas_bspq3::AAS_UnlinkFromBSPLeaves(
                (*ent).leaves as *mut bsp_link_s,
            );
            //link the entity to the world BSP tree
            (*ent).leaves = crate::src::botlib::be_aas_bspq3::AAS_BSPLinkEntity(
                absmins.as_mut_ptr(),
                absmaxs.as_mut_ptr(),
                entnum,
                0 as i32,
            ) as *mut bsp_link_s
        }
        //end if
    }
    return 0 as i32;
}
//end of the function AAS_UpdateEntity
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_EntityInfo(mut entnum: i32, mut info: *mut aas_entityinfo_t) {
    if crate::src::botlib::be_aas_main::aasworld.initialized == 0 {
        crate::src::botlib::be_interface::botimport
            .Print
            .expect("non-null function pointer")(
            4 as i32,
            b"AAS_EntityInfo: aasworld not initialized\n\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        ); //end if
        crate::stdlib::memset(
            info as *mut libc::c_void,
            0 as i32,
            ::std::mem::size_of::<aas_entityinfo_t>() as libc::c_ulong,
        ); //end if
        return;
    }
    if entnum < 0 as i32 || entnum >= crate::src::botlib::be_aas_main::aasworld.maxentities {
        crate::src::botlib::be_interface::botimport
            .Print
            .expect("non-null function pointer")(
            4 as i32,
            b"AAS_EntityInfo: entnum %d out of range\n\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            entnum,
        );
        crate::stdlib::memset(
            info as *mut libc::c_void,
            0 as i32,
            ::std::mem::size_of::<aas_entityinfo_t>() as libc::c_ulong,
        );
        return;
    }
    crate::stdlib::memcpy(
        info as *mut libc::c_void,
        &mut (*crate::src::botlib::be_aas_main::aasworld
            .entities
            .offset(entnum as isize))
        .i as *mut aas_entityinfo_t as *const libc::c_void,
        ::std::mem::size_of::<aas_entityinfo_t>() as libc::c_ulong,
    );
}
//end of the function AAS_EntityInfo
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_EntityOrigin(mut entnum: i32, mut origin: *mut vec_t) {
    if entnum < 0 as i32 || entnum >= crate::src::botlib::be_aas_main::aasworld.maxentities {
        crate::src::botlib::be_interface::botimport
            .Print
            .expect("non-null function pointer")(
            4 as i32,
            b"AAS_EntityOrigin: entnum %d out of range\n\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            entnum,
        ); //end if
        let ref mut fresh0 = *origin.offset(2 as i32 as isize);
        *fresh0 = 0 as i32 as vec_t;
        let ref mut fresh1 = *origin.offset(1 as i32 as isize);
        *fresh1 = *fresh0;
        *origin.offset(0 as i32 as isize) = *fresh1;
        return;
    }
    *origin.offset(0 as i32 as isize) = (*crate::src::botlib::be_aas_main::aasworld
        .entities
        .offset(entnum as isize))
    .i
    .origin[0 as i32 as usize];
    *origin.offset(1 as i32 as isize) = (*crate::src::botlib::be_aas_main::aasworld
        .entities
        .offset(entnum as isize))
    .i
    .origin[1 as i32 as usize];
    *origin.offset(2 as i32 as isize) = (*crate::src::botlib::be_aas_main::aasworld
        .entities
        .offset(entnum as isize))
    .i
    .origin[2 as i32 as usize];
}
//returns the origin of an entity with the given model number
//returns the origin of an entity with the given model number
//returns the best reachable area the entity is situated in
//returns the best reachable area the entity is situated in
//returns the info of the given entity
//returns the info of the given entity
//returns the next entity
//returns the next entity
//returns the origin of the entity
//returns the origin of the entity
//returns the entity type
//returns the entity type
//returns the model index of the entity
//returns the model index of the entity
//end of the function AAS_EntityOrigin
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_EntityModelindex(mut entnum: i32) -> i32 {
    if entnum < 0 as i32 || entnum >= crate::src::botlib::be_aas_main::aasworld.maxentities {
        crate::src::botlib::be_interface::botimport
            .Print
            .expect("non-null function pointer")(
            4 as i32,
            b"AAS_EntityModelindex: entnum %d out of range\n\x00" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            entnum,
        ); //end if
        return 0 as i32;
    }
    return (*crate::src::botlib::be_aas_main::aasworld
        .entities
        .offset(entnum as isize))
    .i
    .modelindex;
}
//end of the function AAS_EntityModelindex
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_EntityType(mut entnum: i32) -> i32 {
    if crate::src::botlib::be_aas_main::aasworld.initialized == 0 {
        return 0 as i32;
    } //end if
    if entnum < 0 as i32 || entnum >= crate::src::botlib::be_aas_main::aasworld.maxentities {
        crate::src::botlib::be_interface::botimport
            .Print
            .expect("non-null function pointer")(
            4 as i32,
            b"AAS_EntityType: entnum %d out of range\n\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            entnum,
        );
        return 0 as i32;
    }
    return (*crate::src::botlib::be_aas_main::aasworld
        .entities
        .offset(entnum as isize))
    .i
    .type_0;
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
 * name:		be_aas_entity.h
 *
 * desc:		AAS
 *
 * $Archive: /source/code/botlib/be_aas_entity.h $
 *
 *****************************************************************************/
/* ****************************************************************************
 * name:		be_aas_entity.h
 *
 * desc:		AAS
 *
 * $Archive: /source/code/botlib/be_aas_entity.h $
 *
 *****************************************************************************/
//invalidates all entity infos
//unlink not updated entities
//resets the entity AAS and BSP links (sets areas and leaves pointers to NULL)
//updates an entity
//gives the entity data used for collision detection
//AASINTERN
//AASINTERN
//returns the size of the entity bounding box in mins and maxs
//returns the size of the entity bounding box in mins and maxs
//returns the BSP model number of the entity
//returns the BSP model number of the entity
//end of the AAS_EntityType
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_EntityModelNum(mut entnum: i32) -> i32 {
    if crate::src::botlib::be_aas_main::aasworld.initialized == 0 {
        return 0 as i32;
    } //end if
    if entnum < 0 as i32 || entnum >= crate::src::botlib::be_aas_main::aasworld.maxentities {
        crate::src::botlib::be_interface::botimport
            .Print
            .expect("non-null function pointer")(
            4 as i32,
            b"AAS_EntityModelNum: entnum %d out of range\n\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            entnum,
        );
        return 0 as i32;
    }
    return (*crate::src::botlib::be_aas_main::aasworld
        .entities
        .offset(entnum as isize))
    .i
    .modelindex;
}
//end of the function AAS_EntityModelNum
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_OriginOfMoverWithModelNum(
    mut modelnum: i32,
    mut origin: *mut vec_t,
) -> i32 {
    let mut i: i32 = 0; //end for
    let mut ent: *mut aas_entity_t = 0 as *mut aas_entity_t;
    i = 0 as i32;
    while i < crate::src::botlib::be_aas_main::aasworld.maxentities {
        ent = &mut *crate::src::botlib::be_aas_main::aasworld
            .entities
            .offset(i as isize) as *mut aas_entity_t;
        if (*ent).i.type_0 == ET_MOVER as i32 {
            if (*ent).i.modelindex == modelnum {
                *origin.offset(0 as i32 as isize) = (*ent).i.origin[0 as i32 as usize];
                *origin.offset(1 as i32 as isize) = (*ent).i.origin[1 as i32 as usize];
                *origin.offset(2 as i32 as isize) = (*ent).i.origin[2 as i32 as usize];
                return qtrue as i32;
            }
            //end if
            //end if
        }
        i += 1
    }
    return qfalse as i32;
}
//end of the function AAS_OriginOfMoverWithModelNum
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_EntitySize(
    mut entnum: i32,
    mut mins: *mut vec_t,
    mut maxs: *mut vec_t,
) {
    let mut ent: *mut aas_entity_t = 0 as *mut aas_entity_t; //end if
    if crate::src::botlib::be_aas_main::aasworld.initialized == 0 {
        return;
    }
    if entnum < 0 as i32 || entnum >= crate::src::botlib::be_aas_main::aasworld.maxentities {
        crate::src::botlib::be_interface::botimport
            .Print
            .expect("non-null function pointer")(
            4 as i32,
            b"AAS_EntitySize: entnum %d out of range\n\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            entnum,
        );
        return;
    }
    ent = &mut *crate::src::botlib::be_aas_main::aasworld
        .entities
        .offset(entnum as isize) as *mut aas_entity_t;
    *mins.offset(0 as i32 as isize) = (*ent).i.mins[0 as i32 as usize];
    *mins.offset(1 as i32 as isize) = (*ent).i.mins[1 as i32 as usize];
    *mins.offset(2 as i32 as isize) = (*ent).i.mins[2 as i32 as usize];
    *maxs.offset(0 as i32 as isize) = (*ent).i.maxs[0 as i32 as usize];
    *maxs.offset(1 as i32 as isize) = (*ent).i.maxs[1 as i32 as usize];
    *maxs.offset(2 as i32 as isize) = (*ent).i.maxs[2 as i32 as usize];
}
//end of the function AAS_EntitySize
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_EntityBSPData(mut entnum: i32, mut entdata: *mut bsp_entdata_t) {
    let mut ent: *mut aas_entity_t = 0 as *mut aas_entity_t;
    ent = &mut *crate::src::botlib::be_aas_main::aasworld
        .entities
        .offset(entnum as isize) as *mut aas_entity_t;
    (*entdata).origin[0 as i32 as usize] = (*ent).i.origin[0 as i32 as usize];
    (*entdata).origin[1 as i32 as usize] = (*ent).i.origin[1 as i32 as usize];
    (*entdata).origin[2 as i32 as usize] = (*ent).i.origin[2 as i32 as usize];
    (*entdata).angles[0 as i32 as usize] = (*ent).i.angles[0 as i32 as usize];
    (*entdata).angles[1 as i32 as usize] = (*ent).i.angles[1 as i32 as usize];
    (*entdata).angles[2 as i32 as usize] = (*ent).i.angles[2 as i32 as usize];
    (*entdata).absmins[0 as i32 as usize] =
        (*ent).i.origin[0 as i32 as usize] + (*ent).i.mins[0 as i32 as usize];
    (*entdata).absmins[1 as i32 as usize] =
        (*ent).i.origin[1 as i32 as usize] + (*ent).i.mins[1 as i32 as usize];
    (*entdata).absmins[2 as i32 as usize] =
        (*ent).i.origin[2 as i32 as usize] + (*ent).i.mins[2 as i32 as usize];
    (*entdata).absmaxs[0 as i32 as usize] =
        (*ent).i.origin[0 as i32 as usize] + (*ent).i.maxs[0 as i32 as usize];
    (*entdata).absmaxs[1 as i32 as usize] =
        (*ent).i.origin[1 as i32 as usize] + (*ent).i.maxs[1 as i32 as usize];
    (*entdata).absmaxs[2 as i32 as usize] =
        (*ent).i.origin[2 as i32 as usize] + (*ent).i.maxs[2 as i32 as usize];
    (*entdata).solid = (*ent).i.solid;
    (*entdata).modelnum = (*ent).i.modelindex - 1 as i32;
}
//end of the function AAS_EntityBSPData
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_ResetEntityLinks() {
    let mut i: i32 = 0;
    i = 0 as i32;
    while i < crate::src::botlib::be_aas_main::aasworld.maxentities {
        let ref mut fresh2 = (*crate::src::botlib::be_aas_main::aasworld
            .entities
            .offset(i as isize))
        .areas;
        *fresh2 = 0 as *mut aas_link_t;
        let ref mut fresh3 = (*crate::src::botlib::be_aas_main::aasworld
            .entities
            .offset(i as isize))
        .leaves;
        *fresh3 = 0 as *mut bsp_link_t;
        i += 1
    }
    //end for
}
//end of the function AAS_ResetEntityLinks
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_InvalidateEntities() {
    let mut i: i32 = 0;
    i = 0 as i32;
    while i < crate::src::botlib::be_aas_main::aasworld.maxentities {
        (*crate::src::botlib::be_aas_main::aasworld
            .entities
            .offset(i as isize))
        .i
        .valid = qfalse as i32;
        (*crate::src::botlib::be_aas_main::aasworld
            .entities
            .offset(i as isize))
        .i
        .number = i;
        i += 1
    }
    //end for
}
//end of the function AAS_InvalidateEntities
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_UnlinkInvalidEntities() {
    let mut i: i32 = 0;
    let mut ent: *mut aas_entity_t = 0 as *mut aas_entity_t;
    i = 0 as i32;
    while i < crate::src::botlib::be_aas_main::aasworld.maxentities {
        ent = &mut *crate::src::botlib::be_aas_main::aasworld
            .entities
            .offset(i as isize) as *mut aas_entity_t;
        if (*ent).i.valid == 0 {
            crate::src::botlib::be_aas_sample::AAS_UnlinkFromAreas((*ent).areas as *mut aas_link_s);
            (*ent).areas = 0 as *mut aas_link_t;
            crate::src::botlib::be_aas_bspq3::AAS_UnlinkFromBSPLeaves(
                (*ent).leaves as *mut bsp_link_s,
            );
            (*ent).leaves = 0 as *mut bsp_link_t
        }
        i += 1
        //end for
    }
    //end for
}
//end of the function AAS_UnlinkInvalidEntities
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_NearestEntity(mut origin: *mut vec_t, mut modelindex: i32) -> i32 {
    let mut i: i32 = 0; //end for
    let mut bestentnum: i32 = 0;
    let mut dist: f32 = 0.;
    let mut bestdist: f32 = 0.;
    let mut ent: *mut aas_entity_t = 0 as *mut aas_entity_t;
    let mut dir: vec3_t = [0.; 3];
    bestentnum = 0 as i32;
    bestdist = 99999 as i32 as f32;
    i = 0 as i32;
    while i < crate::src::botlib::be_aas_main::aasworld.maxentities {
        ent = &mut *crate::src::botlib::be_aas_main::aasworld
            .entities
            .offset(i as isize) as *mut aas_entity_t;
        if !((*ent).i.modelindex != modelindex) {
            dir[0 as i32 as usize] =
                (*ent).i.origin[0 as i32 as usize] - *origin.offset(0 as i32 as isize);
            dir[1 as i32 as usize] =
                (*ent).i.origin[1 as i32 as usize] - *origin.offset(1 as i32 as isize);
            dir[2 as i32 as usize] =
                (*ent).i.origin[2 as i32 as usize] - *origin.offset(2 as i32 as isize);
            if crate::stdlib::fabsf(dir[0 as i32 as usize]) < 40 as i32 as f32 {
                if crate::stdlib::fabsf(dir[1 as i32 as usize]) < 40 as i32 as f32 {
                    dist = VectorLength(dir.as_mut_ptr() as *const vec_t);
                    if dist < bestdist {
                        bestdist = dist;
                        bestentnum = i
                    }
                    //end if
                    //end if
                }
                //end if
            }
        }
        i += 1
    }
    return bestentnum;
}
//end of the function AAS_NearestEntity
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_BestReachableEntityArea(mut entnum: i32) -> i32 {
    let mut ent: *mut aas_entity_t = 0 as *mut aas_entity_t;
    ent = &mut *crate::src::botlib::be_aas_main::aasworld
        .entities
        .offset(entnum as isize) as *mut aas_entity_t;
    return crate::src::botlib::be_aas_reach::AAS_BestReachableLinkArea(
        (*ent).areas as *mut aas_link_s,
    );
}
//end of the function AAS_BestReachableEntityArea
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_NextEntity(mut entnum: i32) -> i32 {
    if crate::src::botlib::be_aas_main::aasworld.loaded == 0 {
        return 0 as i32;
    } //end while
    if entnum < 0 as i32 {
        entnum = -(1 as i32)
    }
    loop {
        entnum += 1;
        if !(entnum < crate::src::botlib::be_aas_main::aasworld.maxentities) {
            break;
        }
        if (*crate::src::botlib::be_aas_main::aasworld
            .entities
            .offset(entnum as isize))
        .i
        .valid
            != 0
        {
            return entnum;
        }
    }
    return 0 as i32;
}
//end of the function AAS_NextEntity

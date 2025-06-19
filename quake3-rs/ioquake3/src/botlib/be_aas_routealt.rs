use ::libc;

pub mod q_shared_h {
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
    // q_shared.h -- included first by ALL program modules.
    // A user mod should never modify this file
    // Heartbeat for dpmaster protocol. You shouldn't change this unless you know what you're doing
    // When com_gamename is LEGACY_MASTER_GAMENAME, use quake3 master protocol.
    // You shouldn't change this unless you know what you're doing
    // number of supported master servers
    // standard demo extension
    //Ignore __attribute__ on non-gcc platforms
    /* *********************************************************************
     VM Considerations

     The VM can not use the standard system headers because we aren't really
     using the compiler they were meant for.  We use bg_lib.h which contains
     prototypes for the functions we define for our own use in bg_lib.c.

     When writing mods, please add needed headers HERE, do not start including
     stuff like <stdio.h> in the various .c files that make up each of the VMs
     since you will be including system headers files can will have issues.

     Remember, if you use a C library function that is not defined in bg_lib.c,
     you will have to add your own version for support in the VM.

    **********************************************************************/
    //=============================================================

    /*
    ==============================================================

    MATHLIB

    ==============================================================
    */

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
pub use crate::be_aas_def_h::bsp_link_s;
pub use crate::be_aas_def_h::bsp_link_t;
pub use crate::be_aas_h::aas_altroutegoal_s;
pub use crate::be_aas_h::aas_altroutegoal_t;
pub use crate::be_aas_h::aas_entityinfo_s;
pub use crate::be_aas_h::aas_entityinfo_t;

pub use crate::src::botlib::be_aas_routealt::q_shared_h::VectorLength;

pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::vec3_t;
pub use crate::src::qcommon::q_shared::vec_t;

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
 * name:		be_aas_routealt.c
 *
 * desc:		AAS
 *
 * $Archive: /MissionPack/code/botlib/be_aas_routealt.c $
 *
 *****************************************************************************/
//#define ALTROUTE_DEBUG

pub type midrangearea_t = midrangearea_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct midrangearea_s {
    pub valid: i32,
    pub starttime: u16,
    pub goaltime: u16,
}
#[no_mangle]

pub static mut midrangeareas: *mut midrangearea_t =
    0 as *const midrangearea_t as *mut midrangearea_t;
#[no_mangle]

pub static mut clusterareas: *mut i32 = 0 as *const i32 as *mut i32;
#[no_mangle]

pub static mut numclusterareas: i32 = 0;
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_AltRoutingFloodCluster_r(mut areanum: i32) {
    let mut i: i32 = 0;
    let mut otherareanum: i32 = 0;
    let mut area: *mut aas_area_t = 0 as *mut aas_area_t;
    let mut face: *mut aas_face_t = 0 as *mut aas_face_t;
    //add the current area to the areas of the current cluster
    *clusterareas.offset(numclusterareas as isize) = areanum;
    numclusterareas += 1;
    //remove the area from the mid range areas
    (*midrangeareas.offset(areanum as isize)).valid = qfalse as i32;
    //flood to other areas through the faces of this area
    area = &mut *crate::src::botlib::be_aas_main::aasworld
        .areas
        .offset(areanum as isize) as *mut aas_area_t;
    i = 0 as i32;
    while i < (*area).numfaces {
        face = &mut *crate::src::botlib::be_aas_main::aasworld
            .faces
            .offset((libc::abs as unsafe extern "C" fn(_: i32) -> i32)(
                *crate::src::botlib::be_aas_main::aasworld
                    .faceindex
                    .offset(((*area).firstface + i) as isize),
            ) as isize) as *mut aas_face_t;
        //get the area at the other side of the face
        if (*face).frontarea == areanum {
            otherareanum = (*face).backarea
        } else {
            otherareanum = (*face).frontarea
        }
        //if there is an area at the other side of this face
        if !(otherareanum == 0) {
            //if the other area is not a midrange area
            if !((*midrangeareas.offset(otherareanum as isize)).valid == 0) {
                //
                AAS_AltRoutingFloodCluster_r(otherareanum);
            }
        }
        i += 1
    }
    //end for
}
//AASINTERN
//AASINTERN
//end of the function AAS_AltRoutingFloodCluster_r
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_AlternativeRouteGoals(
    mut start: *mut vec_t,
    mut startareanum: i32,
    mut _goal: *mut vec_t,
    mut goalareanum: i32,
    mut travelflags: i32,
    mut altroutegoals: *mut aas_altroutegoal_t,
    mut maxaltroutegoals: i32,
    mut type_0: i32,
) -> i32 {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut bestareanum: i32 = 0;
    let mut numaltroutegoals: i32 = 0;
    let mut nummidrangeareas: i32 = 0;
    let mut starttime: i32 = 0;
    let mut goaltime: i32 = 0;
    let mut goaltraveltime: i32 = 0;
    let mut dist: f32 = 0.;
    let mut bestdist: f32 = 0.;
    let mut mid: vec3_t = [0.; 3];
    let mut dir: vec3_t = [0.; 3];
    if startareanum == 0 || goalareanum == 0 {
        return 0 as i32;
    }
    //travel time towards the goal area
    goaltraveltime = crate::src::botlib::be_aas_route::AAS_AreaTravelTimeToGoalArea(
        startareanum,
        start,
        goalareanum,
        travelflags,
    );
    //clear the midrange areas
    crate::stdlib::memset(
        midrangeareas as *mut libc::c_void,
        0 as i32,
        (crate::src::botlib::be_aas_main::aasworld.numareas as usize)
            .wrapping_mul(::std::mem::size_of::<midrangearea_t>() as usize),
    );
    numaltroutegoals = 0 as i32;
    //
    nummidrangeareas = 0 as i32;
    let mut current_block_13: u64;
    //
    i = 1 as i32; //end for
    while i < crate::src::botlib::be_aas_main::aasworld.numareas {
        //
        if type_0 & 1 as i32 == 0 {
            if !(type_0 & 2 as i32 != 0
                && (*crate::src::botlib::be_aas_main::aasworld
                    .areasettings
                    .offset(i as isize))
                .contents
                    & 8 as i32
                    != 0)
            {
                if !(type_0 & 4 as i32 != 0
                    && (*crate::src::botlib::be_aas_main::aasworld
                        .areasettings
                        .offset(i as isize))
                    .contents
                        & 512 as i32
                        != 0)
                {
                    current_block_13 = 8515828400728868193; //end if
                } else {
                    current_block_13 = 10048703153582371463;
                }
            //end if
            } else {
                current_block_13 = 10048703153582371463;
            }
        //end if
        } else {
            current_block_13 = 10048703153582371463;
        }
        match current_block_13 {
            10048703153582371463 =>
            //if the area has no reachabilities
            {
                if !(crate::src::botlib::be_aas_reach::AAS_AreaReachability(i) == 0) {
                    //tavel time from the area to the start area
                    starttime = crate::src::botlib::be_aas_route::AAS_AreaTravelTimeToGoalArea(
                        startareanum,
                        start,
                        i,
                        travelflags,
                    );
                    if !(starttime == 0) {
                        //if the travel time from the start to the area is greater than the shortest goal travel time
                        if !(starttime as f32 > 1.1f64 as f32 * goaltraveltime as f32) {
                            //travel time from the area to the goal area
                            goaltime =
                                crate::src::botlib::be_aas_route::AAS_AreaTravelTimeToGoalArea(
                                    i,
                                    0 as *mut vec_t,
                                    goalareanum,
                                    travelflags,
                                );
                            if !(goaltime == 0) {
                                //if the travel time from the area to the goal is greater than the shortest goal travel time
                                if !(goaltime as f32 > 0.8f64 as f32 * goaltraveltime as f32) {
                                    //this is a mid range area
                                    (*midrangeareas.offset(i as isize)).valid = qtrue as i32;
                                    (*midrangeareas.offset(i as isize)).starttime =
                                        starttime as u16;
                                    (*midrangeareas.offset(i as isize)).goaltime = goaltime as u16;
                                    crate::src::botlib::l_log::Log_Write(
                                        b"%d midrange area %d\x00" as *const u8
                                            as *const libc::c_char
                                            as *mut libc::c_char,
                                        nummidrangeareas,
                                        i,
                                    );
                                    nummidrangeareas += 1
                                }
                            }
                        }
                    }
                }
            }
            _ => {}
        }
        i += 1
    }
    //
    i = 1 as i32; //end for
    while i < crate::src::botlib::be_aas_main::aasworld.numareas {
        if !((*midrangeareas.offset(i as isize)).valid == 0) {
            //get the areas in one cluster
            numclusterareas = 0 as i32;
            AAS_AltRoutingFloodCluster_r(i);
            //now we've got a cluster with areas through which an alternative route could go
            //get the 'center' of the cluster
            mid[2 as i32 as usize] = 0 as i32 as vec_t; //end for
            mid[1 as i32 as usize] = mid[2 as i32 as usize];
            mid[0 as i32 as usize] = mid[1 as i32 as usize];
            j = 0 as i32;
            while j < numclusterareas {
                mid[0 as i32 as usize] = mid[0 as i32 as usize]
                    + (*crate::src::botlib::be_aas_main::aasworld
                        .areas
                        .offset(*clusterareas.offset(j as isize) as isize))
                    .center[0 as i32 as usize];
                mid[1 as i32 as usize] = mid[1 as i32 as usize]
                    + (*crate::src::botlib::be_aas_main::aasworld
                        .areas
                        .offset(*clusterareas.offset(j as isize) as isize))
                    .center[1 as i32 as usize];
                mid[2 as i32 as usize] = mid[2 as i32 as usize]
                    + (*crate::src::botlib::be_aas_main::aasworld
                        .areas
                        .offset(*clusterareas.offset(j as isize) as isize))
                    .center[2 as i32 as usize];
                j += 1
            }
            mid[0 as i32 as usize] =
                (mid[0 as i32 as usize] as f64 * (1.0f64 / numclusterareas as f64)) as vec_t;
            mid[1 as i32 as usize] =
                (mid[1 as i32 as usize] as f64 * (1.0f64 / numclusterareas as f64)) as vec_t;
            mid[2 as i32 as usize] =
                (mid[2 as i32 as usize] as f64 * (1.0f64 / numclusterareas as f64)) as vec_t;
            //get the area closest to the center of the cluster
            bestdist = 999999 as i32 as f32; //end for
            bestareanum = 0 as i32;
            j = 0 as i32;
            while j < numclusterareas {
                dir[0 as i32 as usize] = mid[0 as i32 as usize]
                    - (*crate::src::botlib::be_aas_main::aasworld
                        .areas
                        .offset(*clusterareas.offset(j as isize) as isize))
                    .center[0 as i32 as usize];
                dir[1 as i32 as usize] = mid[1 as i32 as usize]
                    - (*crate::src::botlib::be_aas_main::aasworld
                        .areas
                        .offset(*clusterareas.offset(j as isize) as isize))
                    .center[1 as i32 as usize];
                dir[2 as i32 as usize] = mid[2 as i32 as usize]
                    - (*crate::src::botlib::be_aas_main::aasworld
                        .areas
                        .offset(*clusterareas.offset(j as isize) as isize))
                    .center[2 as i32 as usize];
                dist = VectorLength(dir.as_mut_ptr() as *const vec_t);
                if dist < bestdist {
                    bestdist = dist;
                    bestareanum = *clusterareas.offset(j as isize)
                }
                j += 1
                //end if
            }
            //now we've got an area for an alternative route
            //FIXME: add alternative goal origin
            (*altroutegoals.offset(numaltroutegoals as isize)).origin[0 as i32 as usize] =
                (*crate::src::botlib::be_aas_main::aasworld
                    .areas
                    .offset(bestareanum as isize))
                .center[0 as i32 as usize];
            (*altroutegoals.offset(numaltroutegoals as isize)).origin[1 as i32 as usize] =
                (*crate::src::botlib::be_aas_main::aasworld
                    .areas
                    .offset(bestareanum as isize))
                .center[1 as i32 as usize];
            (*altroutegoals.offset(numaltroutegoals as isize)).origin[2 as i32 as usize] =
                (*crate::src::botlib::be_aas_main::aasworld
                    .areas
                    .offset(bestareanum as isize))
                .center[2 as i32 as usize];
            (*altroutegoals.offset(numaltroutegoals as isize)).areanum = bestareanum;
            (*altroutegoals.offset(numaltroutegoals as isize)).starttraveltime =
                (*midrangeareas.offset(bestareanum as isize)).starttime;
            (*altroutegoals.offset(numaltroutegoals as isize)).goaltraveltime =
                (*midrangeareas.offset(bestareanum as isize)).goaltime;
            (*altroutegoals.offset(numaltroutegoals as isize)).extratraveltime =
                ((*midrangeareas.offset(bestareanum as isize)).starttime as i32
                    + (*midrangeareas.offset(bestareanum as isize)).goaltime as i32
                    - goaltraveltime) as u16;
            numaltroutegoals += 1;
            //
            //don't return more than the maximum alternative route goals
            if numaltroutegoals >= maxaltroutegoals {
                break;
            }
        }
        i += 1
    }
    return numaltroutegoals;
}
//end of the function AAS_AlternativeRouteGoals
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_InitAlternativeRouting() {
    if !midrangeareas.is_null() {
        crate::src::botlib::l_memory::FreeMemory(midrangeareas as *mut libc::c_void);
    }
    midrangeareas = crate::src::botlib::l_memory::GetMemory(
        (crate::src::botlib::be_aas_main::aasworld.numareas as usize)
            .wrapping_mul(::std::mem::size_of::<midrangearea_t>() as usize),
    ) as *mut midrangearea_t;
    if !clusterareas.is_null() {
        crate::src::botlib::l_memory::FreeMemory(clusterareas as *mut libc::c_void);
    }
    clusterareas = crate::src::botlib::l_memory::GetMemory(
        (crate::src::botlib::be_aas_main::aasworld.numareas as usize)
            .wrapping_mul(::std::mem::size_of::<i32>() as usize),
    ) as *mut i32;
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
 * name:		be_aas_routealt.h
 *
 * desc:		AAS
 *
 * $Archive: /source/code/botlib/be_aas_routealt.h $
 *
 *****************************************************************************/
/* ****************************************************************************
 * name:		be_aas_routealt.h
 *
 * desc:		AAS
 *
 * $Archive: /source/code/botlib/be_aas_routealt.h $
 *
 *****************************************************************************/
//end of the function AAS_InitAlternativeRouting
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_ShutdownAlternativeRouting() {
    if !midrangeareas.is_null() {
        crate::src::botlib::l_memory::FreeMemory(midrangeareas as *mut libc::c_void);
    }
    midrangeareas = 0 as *mut midrangearea_t;
    if !clusterareas.is_null() {
        crate::src::botlib::l_memory::FreeMemory(clusterareas as *mut libc::c_void);
    }
    clusterareas = 0 as *mut i32;
    numclusterareas = 0 as i32;
}
//end of the function AAS_ShutdownAlternativeRouting

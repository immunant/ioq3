use ::libc;

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
pub use crate::be_aas_h::aas_entityinfo_s;
pub use crate::be_aas_h::aas_entityinfo_t;
pub use crate::botlib_h::botlib_import_s;
pub use crate::botlib_h::botlib_import_t;
pub use crate::botlib_h::bsp_surface_s;
pub use crate::botlib_h::bsp_surface_t;
pub use crate::botlib_h::bsp_trace_s;
pub use crate::botlib_h::bsp_trace_t;

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
     * name:		be_aas_cluster.c
     *
     * desc:		area clustering
     *
     * $Archive: /MissionPack/code/botlib/be_aas_cluster.c $
     *
     *****************************************************************************/
    #[no_mangle]
    pub static mut botimport: botlib_import_t;
}
// do not flood through area faces, only use reachabilities
#[no_mangle]

pub static mut nofaceflood: i32 = qtrue as i32;
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_RemoveClusterAreas() {
    let mut i: i32 = 0;
    i = 1 as i32;
    while i < crate::src::botlib::be_aas_main::aasworld.numareas {
        (*crate::src::botlib::be_aas_main::aasworld
            .areasettings
            .offset(i as isize))
        .cluster = 0 as i32;
        i += 1
    }
    //end for
}
//end of the function AAS_RemoveClusterAreas
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_ClearCluster(mut clusternum: i32) {
    let mut i: i32 = 0;
    i = 1 as i32;
    while i < crate::src::botlib::be_aas_main::aasworld.numareas {
        if (*crate::src::botlib::be_aas_main::aasworld
            .areasettings
            .offset(i as isize))
        .cluster
            == clusternum
        {
            (*crate::src::botlib::be_aas_main::aasworld
                .areasettings
                .offset(i as isize))
            .cluster = 0 as i32
        }
        i += 1
        //end if
    }
    //end for
}
//end of the function AAS_ClearCluster
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_RemovePortalsClusterReference(mut clusternum: i32) {
    let mut portalnum: i32 = 0; //end if
    portalnum = 1 as i32;
    while portalnum < crate::src::botlib::be_aas_main::aasworld.numportals {
        if (*crate::src::botlib::be_aas_main::aasworld
            .portals
            .offset(portalnum as isize))
        .frontcluster
            == clusternum
        {
            (*crate::src::botlib::be_aas_main::aasworld
                .portals
                .offset(portalnum as isize))
            .frontcluster = 0 as i32
        }
        if (*crate::src::botlib::be_aas_main::aasworld
            .portals
            .offset(portalnum as isize))
        .backcluster
            == clusternum
        {
            (*crate::src::botlib::be_aas_main::aasworld
                .portals
                .offset(portalnum as isize))
            .backcluster = 0 as i32
        }
        portalnum += 1
        //end if
    }
    //end for
}
//end of the function AAS_RemovePortalsClusterReference
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_UpdatePortal(mut areanum: i32, mut clusternum: i32) -> i32 {
    let mut portalnum: i32 = 0;
    let mut portal: *mut aas_portal_t = 0 as *mut aas_portal_t;
    let mut cluster: *mut aas_cluster_t =
        0 as *mut aas_cluster_t;
    //find the portal of the area
    portalnum = 1 as i32; //end for
    while portalnum < crate::src::botlib::be_aas_main::aasworld.numportals {
        if (*crate::src::botlib::be_aas_main::aasworld
            .portals
            .offset(portalnum as isize))
        .areanum
            == areanum
        {
            break;
        }
        portalnum += 1
    }
    //
    if portalnum == crate::src::botlib::be_aas_main::aasworld.numportals {
        crate::src::botlib::be_aas_main::AAS_Error(
            b"no portal of area %d\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            areanum,
        ); //end if
        return qtrue as i32;
    }
    //
    portal = &mut *crate::src::botlib::be_aas_main::aasworld
        .portals
        .offset(portalnum as isize) as *mut aas_portal_t;
    //if the portal is already fully updated
    if (*portal).frontcluster == clusternum {
        return qtrue as i32;
    }
    if (*portal).backcluster == clusternum {
        return qtrue as i32;
    }
    //if the portal has no front cluster yet
    if (*portal).frontcluster == 0 {
        //end else
        (*portal).frontcluster = clusternum
    } else if (*portal).backcluster == 0 {
        //end if
        //if the portal has no back cluster yet
        (*portal).backcluster = clusternum
    } else {
        //end else if
        //remove the cluster portal flag contents
        (*crate::src::botlib::be_aas_main::aasworld
            .areasettings
            .offset(areanum as isize))
        .contents &= !(8 as i32); //end if
        crate::src::botlib::l_log::Log_Write(
            b"portal area %d is separating more than two clusters\r\n\x00" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            areanum,
        );
        return qfalse as i32;
    }
    if crate::src::botlib::be_aas_main::aasworld.portalindexsize >= 65536 as i32 {
        crate::src::botlib::be_aas_main::AAS_Error(
            b"AAS_MAX_PORTALINDEXSIZE\n\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return qtrue as i32;
    }
    //set the area cluster number to the negative portal number
    (*crate::src::botlib::be_aas_main::aasworld
        .areasettings
        .offset(areanum as isize))
    .cluster = -portalnum;
    //add the portal to the cluster using the portal index
    cluster = &mut *crate::src::botlib::be_aas_main::aasworld
        .clusters
        .offset(clusternum as isize) as *mut aas_cluster_t;
    *crate::src::botlib::be_aas_main::aasworld
        .portalindex
        .offset(((*cluster).firstportal + (*cluster).numportals) as isize) = portalnum;
    crate::src::botlib::be_aas_main::aasworld.portalindexsize += 1;
    (*cluster).numportals += 1;
    return qtrue as i32;
}
//end of the function AAS_UpdatePortal
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_FloodClusterAreas_r(mut areanum: i32, mut clusternum: i32) -> i32 {
    let mut area: *mut aas_area_t = 0 as *mut aas_area_t;
    let mut face: *mut aas_face_t = 0 as *mut aas_face_t;
    let mut facenum: i32 = 0;
    let mut i: i32 = 0;
    //
    if areanum <= 0 as i32 || areanum >= crate::src::botlib::be_aas_main::aasworld.numareas {
        crate::src::botlib::be_aas_main::AAS_Error(
            b"AAS_FloodClusterAreas_r: areanum out of range\n\x00" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        ); //end if
        return qfalse as i32;
    }
    //if the area is already part of a cluster
    if (*crate::src::botlib::be_aas_main::aasworld
        .areasettings
        .offset(areanum as isize))
    .cluster
        > 0 as i32
    {
        if (*crate::src::botlib::be_aas_main::aasworld
            .areasettings
            .offset(areanum as isize))
        .cluster
            == clusternum
        {
            return qtrue as i32;
        } //end if
          //
          //there's a reachability going from one cluster to another only in one direction
          //
        crate::src::botlib::be_aas_main::AAS_Error(
            b"cluster %d touched cluster %d at area %d\n\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            clusternum,
            (*crate::src::botlib::be_aas_main::aasworld
                .areasettings
                .offset(areanum as isize))
            .cluster,
            areanum,
        );
        return qfalse as i32;
    }
    //don't add the cluster portal areas to the clusters
    if (*crate::src::botlib::be_aas_main::aasworld
        .areasettings
        .offset(areanum as isize))
    .contents
        & 8 as i32
        != 0
    {
        return AAS_UpdatePortal(areanum, clusternum);
    } //end if
      //set the area cluster number
    (*crate::src::botlib::be_aas_main::aasworld
        .areasettings
        .offset(areanum as isize))
    .cluster = clusternum;
    (*crate::src::botlib::be_aas_main::aasworld
        .areasettings
        .offset(areanum as isize))
    .clusterareanum = (*crate::src::botlib::be_aas_main::aasworld
        .clusters
        .offset(clusternum as isize))
    .numareas;
    //the cluster has an extra area
    let ref mut fresh0 = (*crate::src::botlib::be_aas_main::aasworld
        .clusters
        .offset(clusternum as isize))
    .numareas;
    *fresh0 += 1;
    area = &mut *crate::src::botlib::be_aas_main::aasworld
        .areas
        .offset(areanum as isize) as *mut aas_area_t;
    //use area faces to flood into adjacent areas
    if nofaceflood == 0 {
        i = 0 as i32; //end if
        while i < (*area).numfaces {
            facenum = libc::abs(
                *crate::src::botlib::be_aas_main::aasworld
                    .faceindex
                    .offset(((*area).firstface + i) as isize),
            );
            face = &mut *crate::src::botlib::be_aas_main::aasworld
                .faces
                .offset(facenum as isize) as *mut aas_face_t;
            if (*face).frontarea == areanum {
                //end else
                if (*face).backarea != 0 {
                    if AAS_FloodClusterAreas_r((*face).backarea, clusternum) == 0 {
                        return qfalse as i32;
                    }
                }
            } else if (*face).frontarea != 0 {
                if AAS_FloodClusterAreas_r((*face).frontarea, clusternum) == 0 {
                    return qfalse as i32;
                }
            } //end if
            i += 1
        }
        //end for
    }
    //use the reachabilities to flood into other areas
    i = 0 as i32; //end for
    while i
        < (*crate::src::botlib::be_aas_main::aasworld
            .areasettings
            .offset(areanum as isize))
        .numreachableareas
    {
        if !((*crate::src::botlib::be_aas_main::aasworld
            .reachability
            .offset(
                ((*crate::src::botlib::be_aas_main::aasworld
                    .areasettings
                    .offset(areanum as isize))
                .firstreachablearea
                    + i) as isize,
            ))
        .areanum
            == 0)
        {
            if AAS_FloodClusterAreas_r(
                (*crate::src::botlib::be_aas_main::aasworld
                    .reachability
                    .offset(
                        ((*crate::src::botlib::be_aas_main::aasworld
                            .areasettings
                            .offset(areanum as isize))
                        .firstreachablearea
                            + i) as isize,
                    ))
                .areanum,
                clusternum,
            ) == 0
            {
                return qfalse as i32;
            }
        } //end if
        i += 1
    }
    return qtrue as i32;
}
//end of the function AAS_FloodClusterAreas_r
//===========================================================================
// try to flood from all areas without cluster into areas with a cluster set
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_FloodClusterAreasUsingReachabilities(mut clusternum: i32) -> i32 {
    let mut i: i32 = 0; //end for
    let mut j: i32 = 0;
    let mut areanum: i32 = 0;
    i = 1 as i32;
    while i < crate::src::botlib::be_aas_main::aasworld.numareas {
        //if this area already has a cluster set
        if !((*crate::src::botlib::be_aas_main::aasworld
            .areasettings
            .offset(i as isize))
        .cluster
            != 0)
        {
            //if this area is a cluster portal
            if !((*crate::src::botlib::be_aas_main::aasworld
                .areasettings
                .offset(i as isize))
            .contents
                & 8 as i32
                != 0)
            {
                //loop over the reachable areas from this area
                j = 0 as i32;
                while j
                    < (*crate::src::botlib::be_aas_main::aasworld
                        .areasettings
                        .offset(i as isize))
                    .numreachableareas
                {
                    //the reachable area
                    areanum = (*crate::src::botlib::be_aas_main::aasworld
                        .reachability
                        .offset(
                            ((*crate::src::botlib::be_aas_main::aasworld
                                .areasettings
                                .offset(i as isize))
                            .firstreachablearea
                                + j) as isize,
                        ))
                    .areanum;
                    //end if
                    //if this area is a cluster portal
                    if !((*crate::src::botlib::be_aas_main::aasworld
                        .areasettings
                        .offset(areanum as isize))
                    .contents
                        & 8 as i32
                        != 0)
                    {
                        //if this area has a cluster set
                        if (*crate::src::botlib::be_aas_main::aasworld
                            .areasettings
                            .offset(areanum as isize))
                        .cluster
                            != 0
                        {
                            if AAS_FloodClusterAreas_r(i, clusternum) == 0 {
                                return qfalse as i32;
                            }
                            i = 0 as i32;
                            break;
                        }
                    }
                    j += 1
                }
            }
        }
        i += 1
        //end for
    }
    return qtrue as i32;
}
//end of the function AAS_FloodClusterAreasUsingReachabilities
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_NumberClusterPortals(mut clusternum: i32) {
    let mut i: i32 = 0;
    let mut portalnum: i32 = 0;
    let mut cluster: *mut aas_cluster_t =
        0 as *mut aas_cluster_t;
    let mut portal: *mut aas_portal_t = 0 as *mut aas_portal_t;
    cluster = &mut *crate::src::botlib::be_aas_main::aasworld
        .clusters
        .offset(clusternum as isize) as *mut aas_cluster_t;
    i = 0 as i32;
    while i < (*cluster).numportals {
        portalnum = *crate::src::botlib::be_aas_main::aasworld
            .portalindex
            .offset(((*cluster).firstportal + i) as isize);
        portal = &mut *crate::src::botlib::be_aas_main::aasworld
            .portals
            .offset(portalnum as isize) as *mut aas_portal_t;
        if (*portal).frontcluster == clusternum {
            //end else
            let fresh1 = (*cluster).numareas; //end if
            (*cluster).numareas = (*cluster).numareas + 1;
            (*portal).clusterareanum[0 as i32 as usize] = fresh1
        } else {
            let fresh2 = (*cluster).numareas;
            (*cluster).numareas = (*cluster).numareas + 1;
            (*portal).clusterareanum[1 as i32 as usize] = fresh2
        }
        i += 1
    }
    //end for
}
//end of the function AAS_NumberClusterPortals
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_NumberClusterAreas(mut clusternum: i32) {
    let mut i: i32 = 0;
    let mut portalnum: i32 = 0;
    let mut cluster: *mut aas_cluster_t =
        0 as *mut aas_cluster_t;
    let mut portal: *mut aas_portal_t = 0 as *mut aas_portal_t;
    (*crate::src::botlib::be_aas_main::aasworld
        .clusters
        .offset(clusternum as isize))
    .numareas = 0 as i32;
    (*crate::src::botlib::be_aas_main::aasworld
        .clusters
        .offset(clusternum as isize))
    .numreachabilityareas = 0 as i32;
    //number all areas in this cluster WITH reachabilities
    i = 1 as i32; //end for
    while i < crate::src::botlib::be_aas_main::aasworld.numareas {
        //
        if !((*crate::src::botlib::be_aas_main::aasworld
            .areasettings
            .offset(i as isize))
        .cluster
            != clusternum)
        {
            //
            if !(crate::src::botlib::be_aas_reach::AAS_AreaReachability(i) == 0) {
                //
                (*crate::src::botlib::be_aas_main::aasworld
                    .areasettings
                    .offset(i as isize))
                .clusterareanum = (*crate::src::botlib::be_aas_main::aasworld
                    .clusters
                    .offset(clusternum as isize))
                .numareas;
                //the cluster has an extra area
                let ref mut fresh3 = (*crate::src::botlib::be_aas_main::aasworld
                    .clusters
                    .offset(clusternum as isize))
                .numareas;
                *fresh3 += 1;
                let ref mut fresh4 = (*crate::src::botlib::be_aas_main::aasworld
                    .clusters
                    .offset(clusternum as isize))
                .numreachabilityareas;
                *fresh4 += 1
            }
        }
        i += 1
    }
    //number all portals in this cluster WITH reachabilities
    cluster = &mut *crate::src::botlib::be_aas_main::aasworld
        .clusters
        .offset(clusternum as isize) as *mut aas_cluster_t; //end for
    i = 0 as i32;
    while i < (*cluster).numportals {
        portalnum = *crate::src::botlib::be_aas_main::aasworld
            .portalindex
            .offset(((*cluster).firstportal + i) as isize);
        portal = &mut *crate::src::botlib::be_aas_main::aasworld
            .portals
            .offset(portalnum as isize) as *mut aas_portal_t;
        if !(crate::src::botlib::be_aas_reach::AAS_AreaReachability((*portal).areanum) == 0) {
            if (*portal).frontcluster == clusternum {
                //end else
                let fresh5 = (*cluster).numareas; //end if
                (*cluster).numareas = (*cluster).numareas + 1;
                (*portal).clusterareanum[0 as i32 as usize] = fresh5;
                let ref mut fresh6 = (*crate::src::botlib::be_aas_main::aasworld
                    .clusters
                    .offset(clusternum as isize))
                .numreachabilityareas;
                *fresh6 += 1
            } else {
                let fresh7 = (*cluster).numareas;
                (*cluster).numareas = (*cluster).numareas + 1;
                (*portal).clusterareanum[1 as i32 as usize] = fresh7;
                let ref mut fresh8 = (*crate::src::botlib::be_aas_main::aasworld
                    .clusters
                    .offset(clusternum as isize))
                .numreachabilityareas;
                *fresh8 += 1
            }
        }
        i += 1
    }
    //number all areas in this cluster WITHOUT reachabilities
    i = 1 as i32; //end for
    while i < crate::src::botlib::be_aas_main::aasworld.numareas {
        //
        if !((*crate::src::botlib::be_aas_main::aasworld
            .areasettings
            .offset(i as isize))
        .cluster
            != clusternum)
        {
            //
            if !(crate::src::botlib::be_aas_reach::AAS_AreaReachability(i) != 0) {
                //
                (*crate::src::botlib::be_aas_main::aasworld
                    .areasettings
                    .offset(i as isize))
                .clusterareanum = (*crate::src::botlib::be_aas_main::aasworld
                    .clusters
                    .offset(clusternum as isize))
                .numareas;
                //the cluster has an extra area
                let ref mut fresh9 = (*crate::src::botlib::be_aas_main::aasworld
                    .clusters
                    .offset(clusternum as isize))
                .numareas;
                *fresh9 += 1
            }
        }
        i += 1
    }
    //number all portals in this cluster WITHOUT reachabilities
    cluster = &mut *crate::src::botlib::be_aas_main::aasworld
        .clusters
        .offset(clusternum as isize) as *mut aas_cluster_t;
    i = 0 as i32;
    while i < (*cluster).numportals {
        portalnum = *crate::src::botlib::be_aas_main::aasworld
            .portalindex
            .offset(((*cluster).firstportal + i) as isize);
        portal = &mut *crate::src::botlib::be_aas_main::aasworld
            .portals
            .offset(portalnum as isize) as *mut aas_portal_t;
        if !(crate::src::botlib::be_aas_reach::AAS_AreaReachability((*portal).areanum) != 0) {
            if (*portal).frontcluster == clusternum {
                //end else
                let fresh10 = (*cluster).numareas; //end if
                (*cluster).numareas = (*cluster).numareas + 1;
                (*portal).clusterareanum[0 as i32 as usize] = fresh10
            } else {
                let fresh11 = (*cluster).numareas;
                (*cluster).numareas = (*cluster).numareas + 1;
                (*portal).clusterareanum[1 as i32 as usize] = fresh11
            }
        }
        i += 1
    }
    //end for
}
//end of the function AAS_NumberClusterAreas
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_FindClusters() -> i32 {
    let mut i: i32 = 0;
    let mut cluster: *mut aas_cluster_t =
        0 as *mut aas_cluster_t;
    AAS_RemoveClusterAreas();
    let mut current_block_16: u64;
    //
    i = 1 as i32; //end for
    while i < crate::src::botlib::be_aas_main::aasworld.numareas {
        //if the area is already part of a cluster
        if !((*crate::src::botlib::be_aas_main::aasworld
            .areasettings
            .offset(i as isize))
        .cluster
            != 0)
        {
            // if not flooding through faces only use areas that have reachabilities
            if nofaceflood != 0 {
                if (*crate::src::botlib::be_aas_main::aasworld
                    .areasettings
                    .offset(i as isize))
                .numreachableareas
                    == 0
                {
                    current_block_16 = 14155750587950065367; //end if
                } else {
                    current_block_16 = 6937071982253665452;
                }
            } else {
                current_block_16 = 6937071982253665452;
            }
            match current_block_16 {
                14155750587950065367 => {}
                _ =>
                //if the area is a cluster portal
                {
                    if !((*crate::src::botlib::be_aas_main::aasworld
                        .areasettings
                        .offset(i as isize))
                    .contents
                        & 8 as i32
                        != 0)
                    {
                        if crate::src::botlib::be_aas_main::aasworld.numclusters >= 65536 as i32 {
                            crate::src::botlib::be_aas_main::AAS_Error(
                                b"AAS_MAX_CLUSTERS\n\x00" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                            ); //end if
                            return qfalse as i32;
                        }
                        cluster = &mut *crate::src::botlib::be_aas_main::aasworld
                            .clusters
                            .offset(crate::src::botlib::be_aas_main::aasworld.numclusters as isize)
                            as *mut aas_cluster_t;
                        (*cluster).numareas = 0 as i32;
                        (*cluster).numreachabilityareas = 0 as i32;
                        (*cluster).firstportal =
                            crate::src::botlib::be_aas_main::aasworld.portalindexsize;
                        (*cluster).numportals = 0 as i32;
                        //flood the areas in this cluster
                        if AAS_FloodClusterAreas_r(
                            i,
                            crate::src::botlib::be_aas_main::aasworld.numclusters,
                        ) == 0
                        {
                            return qfalse as i32;
                        }
                        if AAS_FloodClusterAreasUsingReachabilities(
                            crate::src::botlib::be_aas_main::aasworld.numclusters,
                        ) == 0
                        {
                            return qfalse as i32;
                        }
                        //number the cluster areas
                        //AAS_NumberClusterPortals(aasworld.numclusters);
                        AAS_NumberClusterAreas(
                            crate::src::botlib::be_aas_main::aasworld.numclusters,
                        );
                        //Log_Write("cluster %d has %d areas\r\n", aasworld.numclusters, cluster->numareas);
                        crate::src::botlib::be_aas_main::aasworld.numclusters += 1
                    }
                }
            }
        }
        i += 1
    }
    return qtrue as i32;
}
//end of the function AAS_FindClusters
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_CreatePortals() {
    let mut i: i32 = 0;
    let mut portal: *mut aas_portal_t = 0 as *mut aas_portal_t;
    i = 1 as i32;
    while i < crate::src::botlib::be_aas_main::aasworld.numareas {
        //if the area is a cluster portal
        if (*crate::src::botlib::be_aas_main::aasworld
            .areasettings
            .offset(i as isize))
        .contents
            & 8 as i32
            != 0
        {
            if crate::src::botlib::be_aas_main::aasworld.numportals >= 65536 as i32 {
                crate::src::botlib::be_aas_main::AAS_Error(
                    b"AAS_MAX_PORTALS\n\x00" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                ); //end if
                return;
            }
            portal = &mut *crate::src::botlib::be_aas_main::aasworld
                .portals
                .offset(crate::src::botlib::be_aas_main::aasworld.numportals as isize)
                as *mut aas_portal_t;
            (*portal).areanum = i;
            (*portal).frontcluster = 0 as i32;
            (*portal).backcluster = 0 as i32;
            crate::src::botlib::be_aas_main::aasworld.numportals += 1
        }
        i += 1
        //end if
    }
    //end for
}
//end of the function AAS_CreatePortals
/*
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
int AAS_MapContainsTeleporters(void)
{
    bsp_entity_t *entities, *ent;
    char *classname;

    entities = AAS_ParseBSPEntities();

    for (ent = entities; ent; ent = ent->next)
    {
        classname = AAS_ValueForBSPEpairKey(ent, "classname");
        if (classname && !strcmp(classname, "misc_teleporter"))
        {
            AAS_FreeBSPEntities(entities);
            return qtrue;
        } //end if
    } //end for
    return qfalse;
} //end of the function AAS_MapContainsTeleporters
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
int AAS_NonConvexFaces(aas_face_t *face1, aas_face_t *face2, int side1, int side2)
{
    int i, j, edgenum;
    aas_plane_t *plane1, *plane2;
    aas_edge_t *edge;


    plane1 = &aasworld.planes[face1->planenum ^ side1];
    plane2 = &aasworld.planes[face2->planenum ^ side2];

    //check if one of the points of face1 is at the back of the plane of face2
    for (i = 0; i < face1->numedges; i++)
    {
        edgenum = abs(aasworld.edgeindex[face1->firstedge + i]);
        edge = &aasworld.edges[edgenum];
        for (j = 0; j < 2; j++)
        {
            if (DotProduct(plane2->normal, aasworld.vertexes[edge->v[j]]) -
                            plane2->dist < -0.01) return qtrue;
        } //end for
    } //end for
    for (i = 0; i < face2->numedges; i++)
    {
        edgenum = abs(aasworld.edgeindex[face2->firstedge + i]);
        edge = &aasworld.edges[edgenum];
        for (j = 0; j < 2; j++)
        {
            if (DotProduct(plane1->normal, aasworld.vertexes[edge->v[j]]) -
                            plane1->dist < -0.01) return qtrue;
        } //end for
    } //end for

    return qfalse;
} //end of the function AAS_NonConvexFaces
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
qboolean AAS_CanMergeAreas(int *areanums, int numareas)
{
    int i, j, s, face1num, face2num, side1, side2, fn1, fn2;
    aas_face_t *face1, *face2;
    aas_area_t *area1, *area2;

    for (i = 0; i < numareas; i++)
    {
        area1 = &aasworld.areas[areanums[i]];
        for (fn1 = 0; fn1 < area1->numfaces; fn1++)
        {
            face1num = abs(aasworld.faceindex[area1->firstface + fn1]);
            face1 = &aasworld.faces[face1num];
            side1 = face1->frontarea != areanums[i];
            //check if the face isn't a shared one with one of the other areas
            for (s = 0; s < numareas; s++)
            {
                if (s == i) continue;
                if (face1->frontarea == s || face1->backarea == s) break;
            } //end for
            //if the face was a shared one
            if (s != numareas) continue;
            //
            for (j = 0; j < numareas; j++)
            {
                if (j == i) continue;
                area2 = &aasworld.areas[areanums[j]];
                for (fn2 = 0; fn2 < area2->numfaces; fn2++)
                {
                    face2num = abs(aasworld.faceindex[area2->firstface + fn2]);
                    face2 = &aasworld.faces[face2num];
                    side2 = face2->frontarea != areanums[j];
                    //check if the face isn't a shared one with one of the other areas
                    for (s = 0; s < numareas; s++)
                    {
                        if (s == j) continue;
                        if (face2->frontarea == s || face2->backarea == s) break;
                    } //end for
                    //if the face was a shared one
                    if (s != numareas) continue;
                    //
                    if (AAS_NonConvexFaces(face1, face2, side1, side2)) return qfalse;
                } //end for
            } //end for
        } //end for
    } //end for
    return qtrue;
} //end of the function AAS_CanMergeAreas
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
qboolean AAS_NonConvexEdges(aas_edge_t *edge1, aas_edge_t *edge2, int side1, int side2, int planenum)
{
    int i;
    vec3_t edgevec1, edgevec2, normal1, normal2;
    float dist1, dist2;
    aas_plane_t *plane;

    plane = &aasworld.planes[planenum];
    VectorSubtract(aasworld.vertexes[edge1->v[1]], aasworld.vertexes[edge1->v[0]], edgevec1);
    VectorSubtract(aasworld.vertexes[edge2->v[1]], aasworld.vertexes[edge2->v[0]], edgevec2);
    if (side1) VectorInverse(edgevec1);
    if (side2) VectorInverse(edgevec2);
    //
    CrossProduct(edgevec1, plane->normal, normal1);
    dist1 = DotProduct(normal1, aasworld.vertexes[edge1->v[0]]);
    CrossProduct(edgevec2, plane->normal, normal2);
    dist2 = DotProduct(normal2, aasworld.vertexes[edge2->v[0]]);

    for (i = 0; i < 2; i++)
    {
        if (DotProduct(aasworld.vertexes[edge1->v[i]], normal2) - dist2 < -0.01) return qfalse;
    } //end for
    for (i = 0; i < 2; i++)
    {
        if (DotProduct(aasworld.vertexes[edge2->v[i]], normal1) - dist1 < -0.01) return qfalse;
    } //end for
    return qtrue;
} //end of the function AAS_NonConvexEdges
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
qboolean AAS_CanMergeFaces(int *facenums, int numfaces, int planenum)
{
    int i, j, s, edgenum1, edgenum2, side1, side2, en1, en2, ens;
    aas_face_t *face1, *face2, *otherface;
    aas_edge_t *edge1, *edge2;

    for (i = 0; i < numfaces; i++)
    {
        face1 = &aasworld.faces[facenums[i]];
        for (en1 = 0; en1 < face1->numedges; en1++)
        {
            edgenum1 = aasworld.edgeindex[face1->firstedge + en1];
            side1 = (edgenum1 < 0) ^ (face1->planenum != planenum);
            edgenum1 = abs(edgenum1);
            edge1 = &aasworld.edges[edgenum1];
            //check if the edge is shared with another face
            for (s = 0; s < numfaces; s++)
            {
                if (s == i) continue;
                otherface = &aasworld.faces[facenums[s]];
                for (ens = 0; ens < otherface->numedges; ens++)
                {
                    if (edgenum1 == abs(aasworld.edgeindex[otherface->firstedge + ens])) break;
                } //end for
                if (ens != otherface->numedges) break;
            } //end for
            //if the edge was shared
            if (s != numfaces) continue;
            //
            for (j = 0; j < numfaces; j++)
            {
                if (j == i) continue;
                face2 = &aasworld.faces[facenums[j]];
                for (en2 = 0; en2 < face2->numedges; en2++)
                {
                    edgenum2 = aasworld.edgeindex[face2->firstedge + en2];
                    side2 = (edgenum2 < 0) ^ (face2->planenum != planenum);
                    edgenum2 = abs(edgenum2);
                    edge2 = &aasworld.edges[edgenum2];
                    //check if the edge is shared with another face
                    for (s = 0; s < numfaces; s++)
                    {
                        if (s == i) continue;
                        otherface = &aasworld.faces[facenums[s]];
                        for (ens = 0; ens < otherface->numedges; ens++)
                        {
                            if (edgenum2 == abs(aasworld.edgeindex[otherface->firstedge + ens])) break;
                        } //end for
                        if (ens != otherface->numedges) break;
                    } //end for
                    //if the edge was shared
                    if (s != numfaces) continue;
                    //
                    if (AAS_NonConvexEdges(edge1, edge2, side1, side2, planenum)) return qfalse;
                } //end for
            } //end for
        } //end for
    } //end for
    return qtrue;
} //end of the function AAS_CanMergeFaces*/
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_ConnectedAreas_r(
    mut areanums: *mut i32,
    mut numareas: i32,
    mut connectedareas: *mut i32,
    mut curarea: i32,
) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut otherareanum: i32 = 0;
    let mut facenum: i32 = 0;
    let mut area: *mut aas_area_t = 0 as *mut aas_area_t;
    let mut face: *mut aas_face_t = 0 as *mut aas_face_t;
    *connectedareas.offset(curarea as isize) = qtrue as i32;
    area = &mut *crate::src::botlib::be_aas_main::aasworld
        .areas
        .offset(*areanums.offset(curarea as isize) as isize)
        as *mut aas_area_t;
    i = 0 as i32;
    while i < (*area).numfaces {
        facenum = libc::abs(
            *crate::src::botlib::be_aas_main::aasworld
                .faceindex
                .offset(((*area).firstface + i) as isize),
        );
        face = &mut *crate::src::botlib::be_aas_main::aasworld
            .faces
            .offset(facenum as isize) as *mut aas_face_t;
        //if the face is solid
        if !((*face).faceflags & 1 as i32 != 0) {
            //get the area at the other side of the face
            if (*face).frontarea != *areanums.offset(curarea as isize) {
                otherareanum = (*face).frontarea
            } else {
                otherareanum = (*face).backarea
            }
            //check if the face is leading to one of the other areas
            j = 0 as i32; //end for
            while j < numareas {
                if *areanums.offset(j as isize) == otherareanum {
                    break;
                }
                j += 1
            }
            //if the face isn't leading to one of the other areas
            if !(j == numareas) {
                //if the other area is already connected
                if !(*connectedareas.offset(j as isize) != 0) {
                    //recursively proceed with the other area
                    AAS_ConnectedAreas_r(areanums, numareas, connectedareas, j);
                }
            }
        }
        i += 1
    }
    //end for
}
//end of the function AAS_ConnectedAreas_r
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_ConnectedAreas(
    mut areanums: *mut i32,
    mut numareas: i32,
) -> qboolean {
    let mut connectedareas: [i32; 1024] = [0; 1024]; //end for
    let mut i: i32 = 0;
    crate::stdlib::memset(
        connectedareas.as_mut_ptr() as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<[i32; 1024]>() as libc::c_ulong,
    );
    if numareas < 1 as i32 {
        return qfalse;
    }
    if numareas == 1 as i32 {
        return qtrue;
    }
    AAS_ConnectedAreas_r(areanums, numareas, connectedareas.as_mut_ptr(), 0 as i32);
    i = 0 as i32;
    while i < numareas {
        if connectedareas[i as usize] == 0 {
            return qfalse;
        }
        i += 1
    }
    return qtrue;
}
//end of the function AAS_ConnectedAreas
//===========================================================================
// gets adjacent areas with less presence types recursively
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_GetAdjacentAreasWithLessPresenceTypes_r(
    mut areanums: *mut i32,
    mut numareas: i32,
    mut curareanum: i32,
) -> i32 {
    let mut i: i32 = 0; //end for
    let mut j: i32 = 0;
    let mut presencetype: i32 = 0;
    let mut otherpresencetype: i32 = 0;
    let mut otherareanum: i32 = 0;
    let mut facenum: i32 = 0;
    let mut area: *mut aas_area_t = 0 as *mut aas_area_t;
    let mut face: *mut aas_face_t = 0 as *mut aas_face_t;
    let fresh12 = numareas;
    numareas = numareas + 1;
    *areanums.offset(fresh12 as isize) = curareanum;
    area = &mut *crate::src::botlib::be_aas_main::aasworld
        .areas
        .offset(curareanum as isize) as *mut aas_area_t;
    presencetype = (*crate::src::botlib::be_aas_main::aasworld
        .areasettings
        .offset(curareanum as isize))
    .presencetype;
    i = 0 as i32;
    while i < (*area).numfaces {
        facenum = libc::abs(
            *crate::src::botlib::be_aas_main::aasworld
                .faceindex
                .offset(((*area).firstface + i) as isize),
        );
        face = &mut *crate::src::botlib::be_aas_main::aasworld
            .faces
            .offset(facenum as isize) as *mut aas_face_t;
        //end if
        //if the face is solid
        if !((*face).faceflags & 1 as i32 != 0) {
            //the area at the other side of the face
            if (*face).frontarea != curareanum {
                otherareanum = (*face).frontarea
            } else {
                otherareanum = (*face).backarea
            }
            //
            otherpresencetype = (*crate::src::botlib::be_aas_main::aasworld
                .areasettings
                .offset(otherareanum as isize))
            .presencetype;
            //if the other area has less presence types
            if presencetype & !otherpresencetype != 0 && otherpresencetype & !presencetype == 0 {
                //check if the other area isn't already in the list
                j = 0 as i32; //end for
                while j < numareas {
                    if otherareanum == *areanums.offset(j as isize) {
                        break;
                    }
                    j += 1
                }
                //end if
                if j == numareas {
                    //if the other area isn't already in the list
                    if numareas >= 1024 as i32 {
                        crate::src::botlib::be_aas_main::AAS_Error(
                            b"MAX_PORTALAREAS\n\x00" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                        ); //end if
                        return numareas;
                    }
                    numareas = AAS_GetAdjacentAreasWithLessPresenceTypes_r(
                        areanums,
                        numareas,
                        otherareanum,
                    )
                }
            }
        }
        i += 1
    }
    return numareas;
}
//end of the function AAS_GetAdjacentAreasWithLessPresenceTypes_r
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_CheckAreaForPossiblePortals(mut areanum: i32) -> i32 {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut fen: i32 = 0;
    let mut ben: i32 = 0;
    let mut frontedgenum: i32 = 0;
    let mut backedgenum: i32 = 0;
    let mut facenum: i32 = 0;
    let mut areanums: [i32; 1024] = [0; 1024];
    let mut numareas: i32 = 0;
    let mut otherareanum: i32 = 0;
    let mut numareafrontfaces: [i32; 1024] = [0; 1024];
    let mut numareabackfaces: [i32; 1024] = [0; 1024];
    let mut frontfacenums: [i32; 1024] = [0; 1024];
    let mut backfacenums: [i32; 1024] = [0; 1024];
    let mut numfrontfaces: i32 = 0;
    let mut numbackfaces: i32 = 0;
    let mut frontareanums: [i32; 1024] = [0; 1024];
    let mut backareanums: [i32; 1024] = [0; 1024];
    let mut numfrontareas: i32 = 0;
    let mut numbackareas: i32 = 0;
    let mut frontplanenum: i32 = 0;
    let mut backplanenum: i32 = 0;
    let mut faceplanenum: i32 = 0;
    let mut area: *mut aas_area_t = 0 as *mut aas_area_t;
    let mut frontface: *mut aas_face_t = 0 as *mut aas_face_t;
    let mut backface: *mut aas_face_t = 0 as *mut aas_face_t;
    let mut face: *mut aas_face_t = 0 as *mut aas_face_t;
    //if it isn't already a portal
    if (*crate::src::botlib::be_aas_main::aasworld
        .areasettings
        .offset(areanum as isize))
    .contents
        & 8 as i32
        != 0
    {
        return 0 as i32;
    }
    //it must be a grounded area
    if (*crate::src::botlib::be_aas_main::aasworld
        .areasettings
        .offset(areanum as isize))
    .areaflags
        & 1 as i32
        == 0
    {
        return 0 as i32;
    }
    //
    crate::stdlib::memset(
        numareafrontfaces.as_mut_ptr() as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<[i32; 1024]>() as libc::c_ulong,
    );
    crate::stdlib::memset(
        numareabackfaces.as_mut_ptr() as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<[i32; 1024]>() as libc::c_ulong,
    );
    numbackfaces = 0 as i32;
    numfrontfaces = numbackfaces;
    numbackareas = 0 as i32;
    numfrontareas = numbackareas;
    backplanenum = -(1 as i32);
    frontplanenum = backplanenum;
    //add any adjacent areas with less presence types
    numareas =
        AAS_GetAdjacentAreasWithLessPresenceTypes_r(areanums.as_mut_ptr(), 0 as i32, areanum);
    //
    i = 0 as i32; //end for
    while i < numareas {
        area = &mut *crate::src::botlib::be_aas_main::aasworld
            .areas
            .offset(*areanums.as_mut_ptr().offset(i as isize) as isize)
            as *mut aas_area_t;
        j = 0 as i32;
        while j < (*area).numfaces {
            facenum = libc::abs(
                *crate::src::botlib::be_aas_main::aasworld
                    .faceindex
                    .offset(((*area).firstface + j) as isize),
            );
            face = &mut *crate::src::botlib::be_aas_main::aasworld
                .faces
                .offset(facenum as isize) as *mut aas_face_t;
            //end for
            //end else
            //if the face is solid
            if !((*face).faceflags & 1 as i32 != 0) {
                //check if the face is shared with one of the other areas
                k = 0 as i32; //end for
                while k < numareas {
                    if !(k == i) {
                        if (*face).frontarea == areanums[k as usize]
                            || (*face).backarea == areanums[k as usize]
                        {
                            break;
                        }
                    }
                    k += 1
                }
                //if the face is shared
                if !(k != numareas) {
                    //the number of the area at the other side of the face
                    if (*face).frontarea == areanums[i as usize] {
                        otherareanum = (*face).backarea
                    } else {
                        otherareanum = (*face).frontarea
                    }
                    //if the other area already is a cluter portal
                    if (*crate::src::botlib::be_aas_main::aasworld
                        .areasettings
                        .offset(otherareanum as isize))
                    .contents
                        & 8 as i32
                        != 0
                    {
                        return 0 as i32;
                    }
                    //number of the plane of the area
                    faceplanenum = (*face).planenum & !(1 as i32);
                    //
                    if frontplanenum < 0 as i32 || faceplanenum == frontplanenum {
                        frontplanenum = faceplanenum; //end if
                        let fresh13 = numfrontfaces; //end for
                        numfrontfaces = numfrontfaces + 1; //end else
                        frontfacenums[fresh13 as usize] = facenum; //end for
                        k = 0 as i32;
                        while k < numfrontareas {
                            if frontareanums[k as usize] == otherareanum {
                                break;
                            }
                            k += 1
                        }
                        if k == numfrontareas {
                            let fresh14 = numfrontareas;
                            numfrontareas = numfrontareas + 1;
                            frontareanums[fresh14 as usize] = otherareanum
                        }
                        numareafrontfaces[i as usize] += 1
                    } else if backplanenum < 0 as i32 || faceplanenum == backplanenum {
                        backplanenum = faceplanenum;
                        let fresh15 = numbackfaces;
                        numbackfaces = numbackfaces + 1;
                        backfacenums[fresh15 as usize] = facenum;
                        k = 0 as i32;
                        while k < numbackareas {
                            if backareanums[k as usize] == otherareanum {
                                break;
                            }
                            k += 1
                        }
                        if k == numbackareas {
                            let fresh16 = numbackareas;
                            numbackareas = numbackareas + 1;
                            backareanums[fresh16 as usize] = otherareanum
                        }
                        numareabackfaces[i as usize] += 1
                    } else {
                        return 0 as i32;
                    }
                }
            }
            j += 1
        }
        i += 1
    }
    //every area should have at least one front face and one back face
    i = 0 as i32; //end for
    while i < numareas {
        if numareafrontfaces[i as usize] == 0 || numareabackfaces[i as usize] == 0 {
            return 0 as i32;
        }
        i += 1
    }
    //the front areas should all be connected
    if AAS_ConnectedAreas(frontareanums.as_mut_ptr(), numfrontareas) as u64 == 0 {
        return 0 as i32;
    }
    //the back areas should all be connected
    if AAS_ConnectedAreas(backareanums.as_mut_ptr(), numbackareas) as u64 == 0 {
        return 0 as i32;
    }
    //none of the front faces should have a shared edge with a back face
    i = 0 as i32; //end for
    while i < numfrontfaces {
        frontface = &mut *crate::src::botlib::be_aas_main::aasworld
            .faces
            .offset(*frontfacenums.as_mut_ptr().offset(i as isize) as isize)
            as *mut aas_face_t; //end for
        fen = 0 as i32; //end for
        while fen < (*frontface).numedges {
            frontedgenum = libc::abs(
                *crate::src::botlib::be_aas_main::aasworld
                    .edgeindex
                    .offset(((*frontface).firstedge + fen) as isize),
            ); //end for
            j = 0 as i32;
            while j < numbackfaces {
                backface = &mut *crate::src::botlib::be_aas_main::aasworld
                    .faces
                    .offset(*backfacenums.as_mut_ptr().offset(j as isize) as isize)
                    as *mut aas_face_t;
                ben = 0 as i32;
                while ben < (*backface).numedges {
                    backedgenum = libc::abs(
                        *crate::src::botlib::be_aas_main::aasworld
                            .edgeindex
                            .offset(((*backface).firstedge + ben) as isize),
                    );
                    if frontedgenum == backedgenum {
                        break;
                    }
                    ben += 1
                }
                if ben != (*backface).numedges {
                    break;
                }
                j += 1
            }
            if j != numbackfaces {
                break;
            }
            fen += 1
        }
        if fen != (*frontface).numedges {
            break;
        }
        i += 1
    }
    if i != numfrontfaces {
        return 0 as i32;
    }
    //set the cluster portal contents
    i = 0 as i32; //end for
    while i < numareas {
        (*crate::src::botlib::be_aas_main::aasworld
            .areasettings
            .offset(areanums[i as usize] as isize))
        .contents |= 8 as i32;
        //this area can be used as a route portal
        (*crate::src::botlib::be_aas_main::aasworld
            .areasettings
            .offset(areanums[i as usize] as isize))
        .contents |= 32 as i32;
        crate::src::botlib::l_log::Log_Write(
            b"possible portal: %d\r\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            areanums[i as usize],
        );
        i += 1
    }
    //
    return numareas;
}
//end of the function AAS_CheckAreaForPossiblePortals
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_FindPossiblePortals() {
    let mut i: i32 = 0; //end for
    let mut numpossibleportals: i32 = 0;
    numpossibleportals = 0 as i32;
    i = 1 as i32;
    while i < crate::src::botlib::be_aas_main::aasworld.numareas {
        numpossibleportals += AAS_CheckAreaForPossiblePortals(i);
        i += 1
    }
    botimport.Print.expect("non-null function pointer")(
        1 as i32,
        b"\r%6d possible portal areas\n\x00" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        numpossibleportals,
    );
}
//end of the function AAS_FindPossiblePortals
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_RemoveAllPortals() {
    let mut i: i32 = 0;
    i = 1 as i32;
    while i < crate::src::botlib::be_aas_main::aasworld.numareas {
        (*crate::src::botlib::be_aas_main::aasworld
            .areasettings
            .offset(i as isize))
        .contents &= !(8 as i32);
        i += 1
    }
    //end for
}
//end of the function AAS_RemoveAllPortals
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_TestPortals() -> i32 {
    let mut i: i32 = 0; //end for
    let mut portal: *mut aas_portal_t = 0 as *mut aas_portal_t;
    i = 1 as i32;
    while i < crate::src::botlib::be_aas_main::aasworld.numportals {
        portal = &mut *crate::src::botlib::be_aas_main::aasworld
            .portals
            .offset(i as isize) as *mut aas_portal_t;
        //end if
        if (*portal).frontcluster == 0 {
            (*crate::src::botlib::be_aas_main::aasworld
                .areasettings
                .offset((*portal).areanum as isize))
            .contents &= !(8 as i32); //end if
            crate::src::botlib::l_log::Log_Write(
                b"portal area %d has no front cluster\r\n\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                (*portal).areanum,
            );
            return qfalse as i32;
        }
        if (*portal).backcluster == 0 {
            (*crate::src::botlib::be_aas_main::aasworld
                .areasettings
                .offset((*portal).areanum as isize))
            .contents &= !(8 as i32);
            crate::src::botlib::l_log::Log_Write(
                b"portal area %d has no back cluster\r\n\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                (*portal).areanum,
            );
            return qfalse as i32;
        }
        i += 1
    }
    return qtrue as i32;
}
//end of the function
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_CountForcedClusterPortals() {
    let mut num: i32 = 0; //end for
    let mut i: i32 = 0;
    num = 0 as i32;
    i = 1 as i32;
    while i < crate::src::botlib::be_aas_main::aasworld.numareas {
        if (*crate::src::botlib::be_aas_main::aasworld
            .areasettings
            .offset(i as isize))
        .contents
            & 8 as i32
            != 0
        {
            crate::src::botlib::l_log::Log_Write(
                b"area %d is a forced portal area\r\n\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                i,
            );
            num += 1
        }
        i += 1
        //end if
    }
    botimport.Print.expect("non-null function pointer")(
        1 as i32,
        b"%6d forced portal areas\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        num,
    );
}
//end of the function AAS_CountForcedClusterPortals
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_CreateViewPortals() {
    let mut i: i32 = 0;
    i = 1 as i32;
    while i < crate::src::botlib::be_aas_main::aasworld.numareas {
        if (*crate::src::botlib::be_aas_main::aasworld
            .areasettings
            .offset(i as isize))
        .contents
            & 8 as i32
            != 0
        {
            (*crate::src::botlib::be_aas_main::aasworld
                .areasettings
                .offset(i as isize))
            .contents |= 512 as i32
        }
        i += 1
        //end if
    }
    //end for
}
//
//
//end of the function AAS_CreateViewPortals
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_SetViewPortalsAsClusterPortals() {
    let mut i: i32 = 0;
    i = 1 as i32;
    while i < crate::src::botlib::be_aas_main::aasworld.numareas {
        if (*crate::src::botlib::be_aas_main::aasworld
            .areasettings
            .offset(i as isize))
        .contents
            & 512 as i32
            != 0
        {
            (*crate::src::botlib::be_aas_main::aasworld
                .areasettings
                .offset(i as isize))
            .contents |= 8 as i32
        }
        i += 1
        //end if
    }
    //end for
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
 * name:		be_aas_cluster.h
 *
 * desc:		AAS
 *
 * $Archive: /source/code/botlib/be_aas_cluster.h $
 *
 *****************************************************************************/
/* ****************************************************************************
 * name:		be_aas_cluster.h
 *
 * desc:		AAS
 *
 * $Archive: /source/code/botlib/be_aas_cluster.h $
 *
 *****************************************************************************/
//initialize the AAS clustering
//initialize the AAS clustering
//end of the function AAS_SetViewPortalsAsClusterPortals
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_InitClustering() {
    let mut i: i32 = 0;
    let mut removedPortalAreas: i32 = 0;
    let mut n: i32 = 0;
    let mut total: i32 = 0;
    let mut numreachabilityareas: i32 = 0;
    if crate::src::botlib::be_aas_main::aasworld.loaded == 0 {
        return;
    }
    //if there are clusters
    if crate::src::botlib::be_aas_main::aasworld.numclusters >= 1 as i32 {
        //end if
        //if clustering isn't forced
        if crate::src::botlib::l_libvar::LibVarGetValue(
            b"forceclustering\x00" as *const u8 as *const libc::c_char,
        ) as i32
            == 0
            && crate::src::botlib::l_libvar::LibVarGetValue(
                b"forcereachability\x00" as *const u8 as *const libc::c_char,
            ) as i32
                == 0
        {
            return;
        }
    }
    //set all view portals as cluster portals in case we re-calculate the reachabilities and clusters (with -reach)
    AAS_SetViewPortalsAsClusterPortals();
    //count the number of forced cluster portals
    AAS_CountForcedClusterPortals();
    //remove all area cluster marks
    AAS_RemoveClusterAreas();
    //find possible cluster portals
    AAS_FindPossiblePortals();
    //craete portals to for the bot view
    AAS_CreateViewPortals();
    //remove all portals that are not closing a cluster
    //AAS_RemoveNotClusterClosingPortals();
    //initialize portal memory
    if !crate::src::botlib::be_aas_main::aasworld.portals.is_null() {
        crate::src::botlib::l_memory::FreeMemory(
            crate::src::botlib::be_aas_main::aasworld.portals as *mut libc::c_void,
        );
    }
    crate::src::botlib::be_aas_main::aasworld.portals =
        crate::src::botlib::l_memory::GetClearedMemory(
            (65536 as i32 as libc::c_ulong).wrapping_mul(::std::mem::size_of::<
                aas_portal_t,
            >() as libc::c_ulong),
        ) as *mut aas_portal_t;
    //initialize portal index memory
    if !crate::src::botlib::be_aas_main::aasworld
        .portalindex
        .is_null()
    {
        crate::src::botlib::l_memory::FreeMemory(
            crate::src::botlib::be_aas_main::aasworld.portalindex as *mut libc::c_void,
        );
    }
    crate::src::botlib::be_aas_main::aasworld.portalindex =
        crate::src::botlib::l_memory::GetClearedMemory(
            (65536 as i32 as libc::c_ulong).wrapping_mul(::std::mem::size_of::<
                aas_portalindex_t,
            >() as libc::c_ulong),
        ) as *mut aas_portalindex_t;
    //initialize cluster memory
    if !crate::src::botlib::be_aas_main::aasworld.clusters.is_null() {
        crate::src::botlib::l_memory::FreeMemory(
            crate::src::botlib::be_aas_main::aasworld.clusters as *mut libc::c_void,
        );
    }
    crate::src::botlib::be_aas_main::aasworld.clusters =
        crate::src::botlib::l_memory::GetClearedMemory(
            (65536 as i32 as libc::c_ulong).wrapping_mul(::std::mem::size_of::<
                aas_cluster_t,
            >() as libc::c_ulong),
        ) as *mut aas_cluster_t;
    //
    removedPortalAreas = 0 as i32; //end while
    botimport.Print.expect("non-null function pointer")(
        1 as i32,
        b"\r%6d removed portal areas\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        removedPortalAreas,
    );
    loop {
        botimport.Print.expect("non-null function pointer")(
            1 as i32,
            b"\r%6d\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            removedPortalAreas,
        );
        //initialize the number of portals and clusters
        crate::src::botlib::be_aas_main::aasworld.numportals = 1 as i32; //portal 0 is a dummy
        crate::src::botlib::be_aas_main::aasworld.portalindexsize = 0 as i32; //cluster 0 is a dummy
        crate::src::botlib::be_aas_main::aasworld.numclusters = 1 as i32;
        //create the portals from the portal areas
        AAS_CreatePortals();
        //
        removedPortalAreas += 1;
        //find the clusters
        if AAS_FindClusters() == 0 {
            continue;
        }
        //test the portals
        if !(AAS_TestPortals() == 0) {
            break;
        }
    }
    botimport.Print.expect("non-null function pointer")(
        1 as i32,
        b"\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    //the AAS file should be saved
    crate::src::botlib::be_aas_main::aasworld.savefile =
        qtrue as i32;
    //write the portal areas to the log file
    i = 1 as i32; //end for
    while i < crate::src::botlib::be_aas_main::aasworld.numportals {
        crate::src::botlib::l_log::Log_Write(
            b"portal %d: area %d\r\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            i,
            (*crate::src::botlib::be_aas_main::aasworld
                .portals
                .offset(i as isize))
            .areanum,
        );
        i += 1
    }
    // report cluster info
    botimport.Print.expect("non-null function pointer")(
        1 as i32,
        b"%6d portals created\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        crate::src::botlib::be_aas_main::aasworld.numportals,
    ); //end for
    botimport.Print.expect("non-null function pointer")(
        1 as i32,
        b"%6d clusters created\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        crate::src::botlib::be_aas_main::aasworld.numclusters,
    );
    i = 1 as i32;
    while i < crate::src::botlib::be_aas_main::aasworld.numclusters {
        botimport.Print.expect("non-null function pointer")(
            1 as i32,
            b"cluster %d has %d reachability areas\n\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            i,
            (*crate::src::botlib::be_aas_main::aasworld
                .clusters
                .offset(i as isize))
            .numreachabilityareas,
        );
        i += 1
    }
    // report AAS file efficiency
    numreachabilityareas = 0 as i32;
    total = 0 as i32;
    i = 0 as i32;
    while i < crate::src::botlib::be_aas_main::aasworld.numclusters {
        n = (*crate::src::botlib::be_aas_main::aasworld
            .clusters
            .offset(i as isize))
        .numreachabilityareas;
        numreachabilityareas += n;
        total += n * n;
        i += 1
    }
    total += numreachabilityareas * crate::src::botlib::be_aas_main::aasworld.numportals;
    //
    botimport.Print.expect("non-null function pointer")(
        1 as i32,
        b"%6i total reachability areas\n\x00" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        numreachabilityareas,
    );
    botimport.Print.expect("non-null function pointer")(
        1 as i32,
        b"%6i AAS memory/CPU usage (the lower the better)\n\x00" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        total * 3 as i32,
    );
}
//end of the function AAS_InitClustering

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
pub use crate::aasfile_h::aas_header_s;
pub use crate::aasfile_h::aas_header_t;
pub use crate::aasfile_h::aas_lump_t;
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
pub use crate::be_aas_h::C2RustUnnamed_0;
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
pub use crate::src::qcommon::q_shared::FS_SEEK_CUR;
pub use crate::src::qcommon::q_shared::FS_SEEK_END;
pub use crate::src::qcommon::q_shared::FS_SEEK_SET;
pub use crate::src::qcommon::q_shared::FS_WRITE;

pub use crate::src::botlib::be_aas_file::stdlib_h::atoi;

pub use ::libc::strtol;
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
 * name:		be_aas_file.c
 *
 * desc:		AAS file loading/writing
 *
 * $Archive: /MissionPack/code/botlib/be_aas_file.c $
 *
 *****************************************************************************/
//#define AASFILEDEBUG
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_SwapAASData() {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    //bounding boxes
    i = 0 as i32; //end for
    while i < crate::src::botlib::be_aas_main::aasworld.numbboxes {
        (*crate::src::botlib::be_aas_main::aasworld
            .bboxes
            .offset(i as isize))
        .presencetype = (*crate::src::botlib::be_aas_main::aasworld
            .bboxes
            .offset(i as isize))
        .presencetype;
        (*crate::src::botlib::be_aas_main::aasworld
            .bboxes
            .offset(i as isize))
        .flags = (*crate::src::botlib::be_aas_main::aasworld
            .bboxes
            .offset(i as isize))
        .flags;
        j = 0 as i32;
        while j < 3 as i32 {
            (*crate::src::botlib::be_aas_main::aasworld
                .bboxes
                .offset(i as isize))
            .mins[j as usize] = (*crate::src::botlib::be_aas_main::aasworld
                .bboxes
                .offset(i as isize))
            .mins[j as usize];
            (*crate::src::botlib::be_aas_main::aasworld
                .bboxes
                .offset(i as isize))
            .maxs[j as usize] = (*crate::src::botlib::be_aas_main::aasworld
                .bboxes
                .offset(i as isize))
            .maxs[j as usize];
            j += 1
        }
        i += 1
        //end for
    }
    //vertexes
    i = 0 as i32; //end for
    while i < crate::src::botlib::be_aas_main::aasworld.numvertexes {
        j = 0 as i32;
        while j < 3 as i32 {
            (*crate::src::botlib::be_aas_main::aasworld
                .vertexes
                .offset(i as isize))[j as usize] = (*crate::src::botlib::be_aas_main::aasworld
                .vertexes
                .offset(i as isize))[j as usize];
            j += 1
        }
        i += 1
    }
    //planes
    i = 0 as i32; //end for
    while i < crate::src::botlib::be_aas_main::aasworld.numplanes {
        j = 0 as i32;
        while j < 3 as i32 {
            (*crate::src::botlib::be_aas_main::aasworld
                .planes
                .offset(i as isize))
            .normal[j as usize] = (*crate::src::botlib::be_aas_main::aasworld
                .planes
                .offset(i as isize))
            .normal[j as usize];
            j += 1
        }
        (*crate::src::botlib::be_aas_main::aasworld
            .planes
            .offset(i as isize))
        .dist = (*crate::src::botlib::be_aas_main::aasworld
            .planes
            .offset(i as isize))
        .dist;
        (*crate::src::botlib::be_aas_main::aasworld
            .planes
            .offset(i as isize))
        .type_0 = (*crate::src::botlib::be_aas_main::aasworld
            .planes
            .offset(i as isize))
        .type_0;
        i += 1
    }
    //edges
    i = 0 as i32; //end for
    while i < crate::src::botlib::be_aas_main::aasworld.numedges {
        (*crate::src::botlib::be_aas_main::aasworld
            .edges
            .offset(i as isize))
        .v[0 as i32 as usize] = (*crate::src::botlib::be_aas_main::aasworld
            .edges
            .offset(i as isize))
        .v[0 as i32 as usize];
        (*crate::src::botlib::be_aas_main::aasworld
            .edges
            .offset(i as isize))
        .v[1 as i32 as usize] = (*crate::src::botlib::be_aas_main::aasworld
            .edges
            .offset(i as isize))
        .v[1 as i32 as usize];
        i += 1
    }
    //edgeindex
    i = 0 as i32; //end for
    while i < crate::src::botlib::be_aas_main::aasworld.edgeindexsize {
        *crate::src::botlib::be_aas_main::aasworld
            .edgeindex
            .offset(i as isize) = *crate::src::botlib::be_aas_main::aasworld
            .edgeindex
            .offset(i as isize);
        i += 1
    }
    //faces
    i = 0 as i32; //end for
    while i < crate::src::botlib::be_aas_main::aasworld.numfaces {
        (*crate::src::botlib::be_aas_main::aasworld
            .faces
            .offset(i as isize))
        .planenum = (*crate::src::botlib::be_aas_main::aasworld
            .faces
            .offset(i as isize))
        .planenum;
        (*crate::src::botlib::be_aas_main::aasworld
            .faces
            .offset(i as isize))
        .faceflags = (*crate::src::botlib::be_aas_main::aasworld
            .faces
            .offset(i as isize))
        .faceflags;
        (*crate::src::botlib::be_aas_main::aasworld
            .faces
            .offset(i as isize))
        .numedges = (*crate::src::botlib::be_aas_main::aasworld
            .faces
            .offset(i as isize))
        .numedges;
        (*crate::src::botlib::be_aas_main::aasworld
            .faces
            .offset(i as isize))
        .firstedge = (*crate::src::botlib::be_aas_main::aasworld
            .faces
            .offset(i as isize))
        .firstedge;
        (*crate::src::botlib::be_aas_main::aasworld
            .faces
            .offset(i as isize))
        .frontarea = (*crate::src::botlib::be_aas_main::aasworld
            .faces
            .offset(i as isize))
        .frontarea;
        (*crate::src::botlib::be_aas_main::aasworld
            .faces
            .offset(i as isize))
        .backarea = (*crate::src::botlib::be_aas_main::aasworld
            .faces
            .offset(i as isize))
        .backarea;
        i += 1
    }
    //face index
    i = 0 as i32; //end for
    while i < crate::src::botlib::be_aas_main::aasworld.faceindexsize {
        *crate::src::botlib::be_aas_main::aasworld
            .faceindex
            .offset(i as isize) = *crate::src::botlib::be_aas_main::aasworld
            .faceindex
            .offset(i as isize);
        i += 1
    }
    //convex areas
    i = 0 as i32; //end for
    while i < crate::src::botlib::be_aas_main::aasworld.numareas {
        (*crate::src::botlib::be_aas_main::aasworld
            .areas
            .offset(i as isize))
        .areanum = (*crate::src::botlib::be_aas_main::aasworld
            .areas
            .offset(i as isize))
        .areanum;
        (*crate::src::botlib::be_aas_main::aasworld
            .areas
            .offset(i as isize))
        .numfaces = (*crate::src::botlib::be_aas_main::aasworld
            .areas
            .offset(i as isize))
        .numfaces;
        (*crate::src::botlib::be_aas_main::aasworld
            .areas
            .offset(i as isize))
        .firstface = (*crate::src::botlib::be_aas_main::aasworld
            .areas
            .offset(i as isize))
        .firstface;
        j = 0 as i32;
        while j < 3 as i32 {
            (*crate::src::botlib::be_aas_main::aasworld
                .areas
                .offset(i as isize))
            .mins[j as usize] = (*crate::src::botlib::be_aas_main::aasworld
                .areas
                .offset(i as isize))
            .mins[j as usize];
            (*crate::src::botlib::be_aas_main::aasworld
                .areas
                .offset(i as isize))
            .maxs[j as usize] = (*crate::src::botlib::be_aas_main::aasworld
                .areas
                .offset(i as isize))
            .maxs[j as usize];
            (*crate::src::botlib::be_aas_main::aasworld
                .areas
                .offset(i as isize))
            .center[j as usize] = (*crate::src::botlib::be_aas_main::aasworld
                .areas
                .offset(i as isize))
            .center[j as usize];
            j += 1
        }
        i += 1
        //end for
    }
    //area settings
    i = 0 as i32; //end for
    while i < crate::src::botlib::be_aas_main::aasworld.numareasettings {
        (*crate::src::botlib::be_aas_main::aasworld
            .areasettings
            .offset(i as isize))
        .contents = (*crate::src::botlib::be_aas_main::aasworld
            .areasettings
            .offset(i as isize))
        .contents;
        (*crate::src::botlib::be_aas_main::aasworld
            .areasettings
            .offset(i as isize))
        .areaflags = (*crate::src::botlib::be_aas_main::aasworld
            .areasettings
            .offset(i as isize))
        .areaflags;
        (*crate::src::botlib::be_aas_main::aasworld
            .areasettings
            .offset(i as isize))
        .presencetype = (*crate::src::botlib::be_aas_main::aasworld
            .areasettings
            .offset(i as isize))
        .presencetype;
        (*crate::src::botlib::be_aas_main::aasworld
            .areasettings
            .offset(i as isize))
        .cluster = (*crate::src::botlib::be_aas_main::aasworld
            .areasettings
            .offset(i as isize))
        .cluster;
        (*crate::src::botlib::be_aas_main::aasworld
            .areasettings
            .offset(i as isize))
        .clusterareanum = (*crate::src::botlib::be_aas_main::aasworld
            .areasettings
            .offset(i as isize))
        .clusterareanum;
        (*crate::src::botlib::be_aas_main::aasworld
            .areasettings
            .offset(i as isize))
        .numreachableareas = (*crate::src::botlib::be_aas_main::aasworld
            .areasettings
            .offset(i as isize))
        .numreachableareas;
        (*crate::src::botlib::be_aas_main::aasworld
            .areasettings
            .offset(i as isize))
        .firstreachablearea = (*crate::src::botlib::be_aas_main::aasworld
            .areasettings
            .offset(i as isize))
        .firstreachablearea;
        i += 1
    }
    //area reachability
    i = 0 as i32; //end for
    while i < crate::src::botlib::be_aas_main::aasworld.reachabilitysize {
        (*crate::src::botlib::be_aas_main::aasworld
            .reachability
            .offset(i as isize))
        .areanum = (*crate::src::botlib::be_aas_main::aasworld
            .reachability
            .offset(i as isize))
        .areanum; //end for
        (*crate::src::botlib::be_aas_main::aasworld
            .reachability
            .offset(i as isize))
        .facenum = (*crate::src::botlib::be_aas_main::aasworld
            .reachability
            .offset(i as isize))
        .facenum;
        (*crate::src::botlib::be_aas_main::aasworld
            .reachability
            .offset(i as isize))
        .edgenum = (*crate::src::botlib::be_aas_main::aasworld
            .reachability
            .offset(i as isize))
        .edgenum;
        j = 0 as i32;
        while j < 3 as i32 {
            (*crate::src::botlib::be_aas_main::aasworld
                .reachability
                .offset(i as isize))
            .start[j as usize] = (*crate::src::botlib::be_aas_main::aasworld
                .reachability
                .offset(i as isize))
            .start[j as usize];
            (*crate::src::botlib::be_aas_main::aasworld
                .reachability
                .offset(i as isize))
            .end[j as usize] = (*crate::src::botlib::be_aas_main::aasworld
                .reachability
                .offset(i as isize))
            .end[j as usize];
            j += 1
        }
        (*crate::src::botlib::be_aas_main::aasworld
            .reachability
            .offset(i as isize))
        .traveltype = (*crate::src::botlib::be_aas_main::aasworld
            .reachability
            .offset(i as isize))
        .traveltype;
        (*crate::src::botlib::be_aas_main::aasworld
            .reachability
            .offset(i as isize))
        .traveltime = (*crate::src::botlib::be_aas_main::aasworld
            .reachability
            .offset(i as isize))
        .traveltime;
        i += 1
    }
    //nodes
    i = 0 as i32; //end for
    while i < crate::src::botlib::be_aas_main::aasworld.numnodes {
        (*crate::src::botlib::be_aas_main::aasworld
            .nodes
            .offset(i as isize))
        .planenum = (*crate::src::botlib::be_aas_main::aasworld
            .nodes
            .offset(i as isize))
        .planenum;
        (*crate::src::botlib::be_aas_main::aasworld
            .nodes
            .offset(i as isize))
        .children[0 as i32 as usize] = (*crate::src::botlib::be_aas_main::aasworld
            .nodes
            .offset(i as isize))
        .children[0 as i32 as usize];
        (*crate::src::botlib::be_aas_main::aasworld
            .nodes
            .offset(i as isize))
        .children[1 as i32 as usize] = (*crate::src::botlib::be_aas_main::aasworld
            .nodes
            .offset(i as isize))
        .children[1 as i32 as usize];
        i += 1
    }
    //cluster portals
    i = 0 as i32; //end for
    while i < crate::src::botlib::be_aas_main::aasworld.numportals {
        (*crate::src::botlib::be_aas_main::aasworld
            .portals
            .offset(i as isize))
        .areanum = (*crate::src::botlib::be_aas_main::aasworld
            .portals
            .offset(i as isize))
        .areanum;
        (*crate::src::botlib::be_aas_main::aasworld
            .portals
            .offset(i as isize))
        .frontcluster = (*crate::src::botlib::be_aas_main::aasworld
            .portals
            .offset(i as isize))
        .frontcluster;
        (*crate::src::botlib::be_aas_main::aasworld
            .portals
            .offset(i as isize))
        .backcluster = (*crate::src::botlib::be_aas_main::aasworld
            .portals
            .offset(i as isize))
        .backcluster;
        (*crate::src::botlib::be_aas_main::aasworld
            .portals
            .offset(i as isize))
        .clusterareanum[0 as i32 as usize] = (*crate::src::botlib::be_aas_main::aasworld
            .portals
            .offset(i as isize))
        .clusterareanum[0 as i32 as usize];
        (*crate::src::botlib::be_aas_main::aasworld
            .portals
            .offset(i as isize))
        .clusterareanum[1 as i32 as usize] = (*crate::src::botlib::be_aas_main::aasworld
            .portals
            .offset(i as isize))
        .clusterareanum[1 as i32 as usize];
        i += 1
    }
    //cluster portal index
    i = 0 as i32; //end for
    while i < crate::src::botlib::be_aas_main::aasworld.portalindexsize {
        *crate::src::botlib::be_aas_main::aasworld
            .portalindex
            .offset(i as isize) = *crate::src::botlib::be_aas_main::aasworld
            .portalindex
            .offset(i as isize);
        i += 1
    }
    //cluster
    i = 0 as i32;
    while i < crate::src::botlib::be_aas_main::aasworld.numclusters {
        (*crate::src::botlib::be_aas_main::aasworld
            .clusters
            .offset(i as isize))
        .numareas = (*crate::src::botlib::be_aas_main::aasworld
            .clusters
            .offset(i as isize))
        .numareas;
        (*crate::src::botlib::be_aas_main::aasworld
            .clusters
            .offset(i as isize))
        .numreachabilityareas = (*crate::src::botlib::be_aas_main::aasworld
            .clusters
            .offset(i as isize))
        .numreachabilityareas;
        (*crate::src::botlib::be_aas_main::aasworld
            .clusters
            .offset(i as isize))
        .numportals = (*crate::src::botlib::be_aas_main::aasworld
            .clusters
            .offset(i as isize))
        .numportals;
        (*crate::src::botlib::be_aas_main::aasworld
            .clusters
            .offset(i as isize))
        .firstportal = (*crate::src::botlib::be_aas_main::aasworld
            .clusters
            .offset(i as isize))
        .firstportal;
        i += 1
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
/* ****************************************************************************
 * name:		be_aas_file.h
 *
 * desc:		AAS
 *
 * $Archive: /source/code/botlib/be_aas_file.h $
 *
 *****************************************************************************/
//loads the AAS file with the given name
//writes an AAS file with the given name
//dumps the loaded AAS data
//end of the function AAS_SwapAASData
//===========================================================================
// dump the current loaded aas file
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_DumpAASData() {
    crate::src::botlib::be_aas_main::aasworld.numbboxes = 0 as i32;
    if !crate::src::botlib::be_aas_main::aasworld.bboxes.is_null() {
        crate::src::botlib::l_memory::FreeMemory(
            crate::src::botlib::be_aas_main::aasworld.bboxes as *mut libc::c_void,
        );
    }
    crate::src::botlib::be_aas_main::aasworld.bboxes = 0 as *mut crate::aasfile_h::aas_bbox_t;
    crate::src::botlib::be_aas_main::aasworld.numvertexes = 0 as i32;
    if !crate::src::botlib::be_aas_main::aasworld.vertexes.is_null() {
        crate::src::botlib::l_memory::FreeMemory(
            crate::src::botlib::be_aas_main::aasworld.vertexes as *mut libc::c_void,
        );
    }
    crate::src::botlib::be_aas_main::aasworld.vertexes = 0 as *mut crate::aasfile_h::aas_vertex_t;
    crate::src::botlib::be_aas_main::aasworld.numplanes = 0 as i32;
    if !crate::src::botlib::be_aas_main::aasworld.planes.is_null() {
        crate::src::botlib::l_memory::FreeMemory(
            crate::src::botlib::be_aas_main::aasworld.planes as *mut libc::c_void,
        );
    }
    crate::src::botlib::be_aas_main::aasworld.planes = 0 as *mut crate::aasfile_h::aas_plane_t;
    crate::src::botlib::be_aas_main::aasworld.numedges = 0 as i32;
    if !crate::src::botlib::be_aas_main::aasworld.edges.is_null() {
        crate::src::botlib::l_memory::FreeMemory(
            crate::src::botlib::be_aas_main::aasworld.edges as *mut libc::c_void,
        );
    }
    crate::src::botlib::be_aas_main::aasworld.edges = 0 as *mut crate::aasfile_h::aas_edge_t;
    crate::src::botlib::be_aas_main::aasworld.edgeindexsize = 0 as i32;
    if !crate::src::botlib::be_aas_main::aasworld
        .edgeindex
        .is_null()
    {
        crate::src::botlib::l_memory::FreeMemory(
            crate::src::botlib::be_aas_main::aasworld.edgeindex as *mut libc::c_void,
        );
    }
    crate::src::botlib::be_aas_main::aasworld.edgeindex =
        0 as *mut crate::aasfile_h::aas_edgeindex_t;
    crate::src::botlib::be_aas_main::aasworld.numfaces = 0 as i32;
    if !crate::src::botlib::be_aas_main::aasworld.faces.is_null() {
        crate::src::botlib::l_memory::FreeMemory(
            crate::src::botlib::be_aas_main::aasworld.faces as *mut libc::c_void,
        );
    }
    crate::src::botlib::be_aas_main::aasworld.faces = 0 as *mut crate::aasfile_h::aas_face_t;
    crate::src::botlib::be_aas_main::aasworld.faceindexsize = 0 as i32;
    if !crate::src::botlib::be_aas_main::aasworld
        .faceindex
        .is_null()
    {
        crate::src::botlib::l_memory::FreeMemory(
            crate::src::botlib::be_aas_main::aasworld.faceindex as *mut libc::c_void,
        );
    }
    crate::src::botlib::be_aas_main::aasworld.faceindex =
        0 as *mut crate::aasfile_h::aas_faceindex_t;
    crate::src::botlib::be_aas_main::aasworld.numareas = 0 as i32;
    if !crate::src::botlib::be_aas_main::aasworld.areas.is_null() {
        crate::src::botlib::l_memory::FreeMemory(
            crate::src::botlib::be_aas_main::aasworld.areas as *mut libc::c_void,
        );
    }
    crate::src::botlib::be_aas_main::aasworld.areas = 0 as *mut crate::aasfile_h::aas_area_t;
    crate::src::botlib::be_aas_main::aasworld.numareasettings = 0 as i32;
    if !crate::src::botlib::be_aas_main::aasworld
        .areasettings
        .is_null()
    {
        crate::src::botlib::l_memory::FreeMemory(
            crate::src::botlib::be_aas_main::aasworld.areasettings as *mut libc::c_void,
        );
    }
    crate::src::botlib::be_aas_main::aasworld.areasettings =
        0 as *mut crate::aasfile_h::aas_areasettings_t;
    crate::src::botlib::be_aas_main::aasworld.reachabilitysize = 0 as i32;
    if !crate::src::botlib::be_aas_main::aasworld
        .reachability
        .is_null()
    {
        crate::src::botlib::l_memory::FreeMemory(
            crate::src::botlib::be_aas_main::aasworld.reachability as *mut libc::c_void,
        );
    }
    crate::src::botlib::be_aas_main::aasworld.reachability =
        0 as *mut crate::aasfile_h::aas_reachability_t;
    crate::src::botlib::be_aas_main::aasworld.numnodes = 0 as i32;
    if !crate::src::botlib::be_aas_main::aasworld.nodes.is_null() {
        crate::src::botlib::l_memory::FreeMemory(
            crate::src::botlib::be_aas_main::aasworld.nodes as *mut libc::c_void,
        );
    }
    crate::src::botlib::be_aas_main::aasworld.nodes = 0 as *mut crate::aasfile_h::aas_node_t;
    crate::src::botlib::be_aas_main::aasworld.numportals = 0 as i32;
    if !crate::src::botlib::be_aas_main::aasworld.portals.is_null() {
        crate::src::botlib::l_memory::FreeMemory(
            crate::src::botlib::be_aas_main::aasworld.portals as *mut libc::c_void,
        );
    }
    crate::src::botlib::be_aas_main::aasworld.portals = 0 as *mut crate::aasfile_h::aas_portal_t;
    crate::src::botlib::be_aas_main::aasworld.numportals = 0 as i32;
    if !crate::src::botlib::be_aas_main::aasworld
        .portalindex
        .is_null()
    {
        crate::src::botlib::l_memory::FreeMemory(
            crate::src::botlib::be_aas_main::aasworld.portalindex as *mut libc::c_void,
        );
    }
    crate::src::botlib::be_aas_main::aasworld.portalindex =
        0 as *mut crate::aasfile_h::aas_portalindex_t;
    crate::src::botlib::be_aas_main::aasworld.portalindexsize = 0 as i32;
    if !crate::src::botlib::be_aas_main::aasworld.clusters.is_null() {
        crate::src::botlib::l_memory::FreeMemory(
            crate::src::botlib::be_aas_main::aasworld.clusters as *mut libc::c_void,
        );
    }
    crate::src::botlib::be_aas_main::aasworld.clusters = 0 as *mut crate::aasfile_h::aas_cluster_t;
    crate::src::botlib::be_aas_main::aasworld.numclusters = 0 as i32;
    //
    crate::src::botlib::be_aas_main::aasworld.loaded = crate::src::qcommon::q_shared::qfalse as i32;
    crate::src::botlib::be_aas_main::aasworld.initialized =
        crate::src::qcommon::q_shared::qfalse as i32;
    crate::src::botlib::be_aas_main::aasworld.savefile =
        crate::src::qcommon::q_shared::qfalse as i32;
}
//end of the function AAS_DumpAASData
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
//AASFILEDEBUG
//===========================================================================
// allocate memory and read a lump of an AAS file
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_LoadAASLump(
    mut fp: crate::src::qcommon::q_shared::fileHandle_t,
    mut offset: i32,
    mut length: i32,
    mut lastoffset: *mut i32,
    mut size: i32,
) -> *mut libc::c_char {
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    //
    if length == 0 {
        //end if
        //just alloc a dummy
        return crate::src::botlib::l_memory::GetClearedHunkMemory(
            (size + 1 as i32) as libc::c_ulong,
        ) as *mut libc::c_char;
    }
    //seek to the data
    if offset != *lastoffset {
        crate::src::botlib::be_interface::botimport
            .Print
            .expect("non-null function pointer")(
            2 as i32,
            b"AAS file not sequentially read\n\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        if crate::src::botlib::be_interface::botimport
            .FS_Seek
            .expect("non-null function pointer")(
            fp,
            offset as isize,
            crate::src::qcommon::q_shared::FS_SEEK_SET as i32,
        ) != 0
        {
            crate::src::botlib::be_aas_main::AAS_Error(
                b"can\'t seek to aas lump\n\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            ); //end if
            AAS_DumpAASData();
            crate::src::botlib::be_interface::botimport
                .FS_FCloseFile
                .expect("non-null function pointer")(fp);
            return 0 as *mut libc::c_char;
        }
        //end if
    }
    //allocate memory
    buf = crate::src::botlib::l_memory::GetClearedHunkMemory((length + 1 as i32) as libc::c_ulong)
        as *mut libc::c_char;
    //read the data
    if length != 0 {
        crate::src::botlib::be_interface::botimport
            .FS_Read
            .expect("non-null function pointer")(buf as *mut libc::c_void, length, fp); //end if
        *lastoffset += length
    }
    return buf;
}
//end of the function AAS_LoadAASLump
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_DData(mut data: *mut u8, mut size: i32) {
    let mut i: i32 = 0;
    i = 0 as i32;
    while i < size {
        let ref mut fresh0 = *data.offset(i as isize);
        *fresh0 = (*fresh0 as i32 ^ i as u8 as i32 * 119 as i32) as u8;
        i += 1
    }
    //end for
}
//end of the function AAS_DData
//===========================================================================
// load an aas file
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_LoadAASFile(mut filename: *mut libc::c_char) -> i32 {
    let mut fp: crate::src::qcommon::q_shared::fileHandle_t = 0;
    let mut header: crate::aasfile_h::aas_header_t = crate::aasfile_h::aas_header_t {
        ident: 0,
        version: 0,
        bspchecksum: 0,
        lumps: [crate::aasfile_h::aas_lump_t {
            fileofs: 0,
            filelen: 0,
        }; 14],
    };
    let mut offset: i32 = 0;
    let mut length: i32 = 0;
    let mut lastoffset: i32 = 0;
    crate::src::botlib::be_interface::botimport
        .Print
        .expect("non-null function pointer")(
        1 as i32,
        b"trying to load %s\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        filename,
    );
    //dump current loaded aas file
    AAS_DumpAASData();
    //open the file
    crate::src::botlib::be_interface::botimport
        .FS_FOpenFile
        .expect("non-null function pointer")(
        filename,
        &mut fp,
        crate::src::qcommon::q_shared::FS_READ,
    ); //end if
    if fp == 0 {
        crate::src::botlib::be_aas_main::AAS_Error(
            b"can\'t open %s\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            filename,
        );
        return 4 as i32;
    }
    //read the header
    crate::src::botlib::be_interface::botimport
        .FS_Read
        .expect("non-null function pointer")(
        &mut header as *mut crate::aasfile_h::aas_header_t as *mut libc::c_void,
        ::std::mem::size_of::<crate::aasfile_h::aas_header_t>() as libc::c_ulong as i32,
        fp,
    );
    lastoffset = ::std::mem::size_of::<crate::aasfile_h::aas_header_t>() as libc::c_ulong as i32;
    //check header identification
    header.ident = header.ident; //end if
    if header.ident
        != (('S' as i32) << 24 as i32)
            + (('A' as i32) << 16 as i32)
            + (('A' as i32) << 8 as i32)
            + 'E' as i32
    {
        crate::src::botlib::be_aas_main::AAS_Error(
            b"%s is not an AAS file\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            filename,
        );
        crate::src::botlib::be_interface::botimport
            .FS_FCloseFile
            .expect("non-null function pointer")(fp);
        return 5 as i32;
    }
    //check the version
    header.version = header.version;
    //
    if header.version != 4 as i32 && header.version != 5 as i32 {
        crate::src::botlib::be_aas_main::AAS_Error(
            b"aas file %s is version %i, not %i\n\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            filename,
            header.version,
            5 as i32,
        ); //end if
        crate::src::botlib::be_interface::botimport
            .FS_FCloseFile
            .expect("non-null function pointer")(fp);
        return 6 as i32;
    }
    //
    if header.version == 5 as i32 {
        AAS_DData(
            (&mut header as *mut crate::aasfile_h::aas_header_t as *mut u8)
                .offset(8 as i32 as isize),
            (::std::mem::size_of::<crate::aasfile_h::aas_header_t>() as libc::c_ulong)
                .wrapping_sub(8 as i32 as libc::c_ulong) as i32,
        ); //end if
    }
    //
    crate::src::botlib::be_aas_main::aasworld.bspchecksum =
        atoi(crate::src::botlib::l_libvar::LibVarGetString(
            b"sv_mapChecksum\x00" as *const u8 as *const libc::c_char,
        )); //end if
    if header.bspchecksum != crate::src::botlib::be_aas_main::aasworld.bspchecksum {
        crate::src::botlib::be_aas_main::AAS_Error(
            b"aas file %s is out of date\n\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            filename,
        );
        crate::src::botlib::be_interface::botimport
            .FS_FCloseFile
            .expect("non-null function pointer")(fp);
        return 6 as i32;
    }
    //load the lumps:
    //bounding boxes
    offset = header.lumps[0 as i32 as usize].fileofs;
    length = header.lumps[0 as i32 as usize].filelen;
    crate::src::botlib::be_aas_main::aasworld.bboxes = AAS_LoadAASLump(
        fp,
        offset,
        length,
        &mut lastoffset,
        ::std::mem::size_of::<crate::aasfile_h::aas_bbox_t>() as libc::c_ulong as i32,
    ) as *mut crate::aasfile_h::aas_bbox_t;
    crate::src::botlib::be_aas_main::aasworld.numbboxes = (length as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<crate::aasfile_h::aas_bbox_t>() as libc::c_ulong)
        as i32;
    if crate::src::botlib::be_aas_main::aasworld.numbboxes != 0
        && crate::src::botlib::be_aas_main::aasworld.bboxes.is_null()
    {
        return 7 as i32;
    }
    //vertexes
    offset = header.lumps[1 as i32 as usize].fileofs;
    length = header.lumps[1 as i32 as usize].filelen;
    crate::src::botlib::be_aas_main::aasworld.vertexes = AAS_LoadAASLump(
        fp,
        offset,
        length,
        &mut lastoffset,
        ::std::mem::size_of::<crate::aasfile_h::aas_vertex_t>() as libc::c_ulong as i32,
    )
        as *mut crate::aasfile_h::aas_vertex_t;
    crate::src::botlib::be_aas_main::aasworld.numvertexes = (length as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<crate::aasfile_h::aas_vertex_t>() as libc::c_ulong)
        as i32;
    if crate::src::botlib::be_aas_main::aasworld.numvertexes != 0
        && crate::src::botlib::be_aas_main::aasworld.vertexes.is_null()
    {
        return 7 as i32;
    }
    //planes
    offset = header.lumps[2 as i32 as usize].fileofs;
    length = header.lumps[2 as i32 as usize].filelen;
    crate::src::botlib::be_aas_main::aasworld.planes = AAS_LoadAASLump(
        fp,
        offset,
        length,
        &mut lastoffset,
        ::std::mem::size_of::<crate::aasfile_h::aas_plane_t>() as libc::c_ulong as i32,
    ) as *mut crate::aasfile_h::aas_plane_t;
    crate::src::botlib::be_aas_main::aasworld.numplanes = (length as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<crate::aasfile_h::aas_plane_t>() as libc::c_ulong)
        as i32;
    if crate::src::botlib::be_aas_main::aasworld.numplanes != 0
        && crate::src::botlib::be_aas_main::aasworld.planes.is_null()
    {
        return 7 as i32;
    }
    //edges
    offset = header.lumps[3 as i32 as usize].fileofs;
    length = header.lumps[3 as i32 as usize].filelen;
    crate::src::botlib::be_aas_main::aasworld.edges = AAS_LoadAASLump(
        fp,
        offset,
        length,
        &mut lastoffset,
        ::std::mem::size_of::<crate::aasfile_h::aas_edge_t>() as libc::c_ulong as i32,
    ) as *mut crate::aasfile_h::aas_edge_t;
    crate::src::botlib::be_aas_main::aasworld.numedges = (length as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<crate::aasfile_h::aas_edge_t>() as libc::c_ulong)
        as i32;
    if crate::src::botlib::be_aas_main::aasworld.numedges != 0
        && crate::src::botlib::be_aas_main::aasworld.edges.is_null()
    {
        return 7 as i32;
    }
    //edgeindex
    offset = header.lumps[4 as i32 as usize].fileofs;
    length = header.lumps[4 as i32 as usize].filelen;
    crate::src::botlib::be_aas_main::aasworld.edgeindex = AAS_LoadAASLump(
        fp,
        offset,
        length,
        &mut lastoffset,
        ::std::mem::size_of::<crate::aasfile_h::aas_edgeindex_t>() as libc::c_ulong as i32,
    )
        as *mut crate::aasfile_h::aas_edgeindex_t;
    crate::src::botlib::be_aas_main::aasworld.edgeindexsize = (length as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<crate::aasfile_h::aas_edgeindex_t>() as libc::c_ulong)
        as i32;
    if crate::src::botlib::be_aas_main::aasworld.edgeindexsize != 0
        && crate::src::botlib::be_aas_main::aasworld
            .edgeindex
            .is_null()
    {
        return 7 as i32;
    }
    //faces
    offset = header.lumps[5 as i32 as usize].fileofs;
    length = header.lumps[5 as i32 as usize].filelen;
    crate::src::botlib::be_aas_main::aasworld.faces = AAS_LoadAASLump(
        fp,
        offset,
        length,
        &mut lastoffset,
        ::std::mem::size_of::<crate::aasfile_h::aas_face_t>() as libc::c_ulong as i32,
    ) as *mut crate::aasfile_h::aas_face_t;
    crate::src::botlib::be_aas_main::aasworld.numfaces = (length as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<crate::aasfile_h::aas_face_t>() as libc::c_ulong)
        as i32;
    if crate::src::botlib::be_aas_main::aasworld.numfaces != 0
        && crate::src::botlib::be_aas_main::aasworld.faces.is_null()
    {
        return 7 as i32;
    }
    //faceindex
    offset = header.lumps[6 as i32 as usize].fileofs;
    length = header.lumps[6 as i32 as usize].filelen;
    crate::src::botlib::be_aas_main::aasworld.faceindex = AAS_LoadAASLump(
        fp,
        offset,
        length,
        &mut lastoffset,
        ::std::mem::size_of::<crate::aasfile_h::aas_faceindex_t>() as libc::c_ulong as i32,
    )
        as *mut crate::aasfile_h::aas_faceindex_t;
    crate::src::botlib::be_aas_main::aasworld.faceindexsize = (length as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<crate::aasfile_h::aas_faceindex_t>() as libc::c_ulong)
        as i32;
    if crate::src::botlib::be_aas_main::aasworld.faceindexsize != 0
        && crate::src::botlib::be_aas_main::aasworld
            .faceindex
            .is_null()
    {
        return 7 as i32;
    }
    //convex areas
    offset = header.lumps[7 as i32 as usize].fileofs;
    length = header.lumps[7 as i32 as usize].filelen;
    crate::src::botlib::be_aas_main::aasworld.areas = AAS_LoadAASLump(
        fp,
        offset,
        length,
        &mut lastoffset,
        ::std::mem::size_of::<crate::aasfile_h::aas_area_t>() as libc::c_ulong as i32,
    ) as *mut crate::aasfile_h::aas_area_t;
    crate::src::botlib::be_aas_main::aasworld.numareas = (length as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<crate::aasfile_h::aas_area_t>() as libc::c_ulong)
        as i32;
    if crate::src::botlib::be_aas_main::aasworld.numareas != 0
        && crate::src::botlib::be_aas_main::aasworld.areas.is_null()
    {
        return 7 as i32;
    }
    //area settings
    offset = header.lumps[8 as i32 as usize].fileofs;
    length = header.lumps[8 as i32 as usize].filelen;
    crate::src::botlib::be_aas_main::aasworld.areasettings = AAS_LoadAASLump(
        fp,
        offset,
        length,
        &mut lastoffset,
        ::std::mem::size_of::<crate::aasfile_h::aas_areasettings_t>() as libc::c_ulong as i32,
    )
        as *mut crate::aasfile_h::aas_areasettings_t;
    crate::src::botlib::be_aas_main::aasworld.numareasettings = (length as libc::c_ulong)
        .wrapping_div(
            ::std::mem::size_of::<crate::aasfile_h::aas_areasettings_t>() as libc::c_ulong,
        ) as i32;
    if crate::src::botlib::be_aas_main::aasworld.numareasettings != 0
        && crate::src::botlib::be_aas_main::aasworld
            .areasettings
            .is_null()
    {
        return 7 as i32;
    }
    //reachability list
    offset = header.lumps[9 as i32 as usize].fileofs;
    length = header.lumps[9 as i32 as usize].filelen;
    crate::src::botlib::be_aas_main::aasworld.reachability = AAS_LoadAASLump(
        fp,
        offset,
        length,
        &mut lastoffset,
        ::std::mem::size_of::<crate::aasfile_h::aas_reachability_t>() as libc::c_ulong as i32,
    )
        as *mut crate::aasfile_h::aas_reachability_t;
    crate::src::botlib::be_aas_main::aasworld.reachabilitysize = (length as libc::c_ulong)
        .wrapping_div(
            ::std::mem::size_of::<crate::aasfile_h::aas_reachability_t>() as libc::c_ulong,
        ) as i32;
    if crate::src::botlib::be_aas_main::aasworld.reachabilitysize != 0
        && crate::src::botlib::be_aas_main::aasworld
            .reachability
            .is_null()
    {
        return 7 as i32;
    }
    //nodes
    offset = header.lumps[10 as i32 as usize].fileofs;
    length = header.lumps[10 as i32 as usize].filelen;
    crate::src::botlib::be_aas_main::aasworld.nodes = AAS_LoadAASLump(
        fp,
        offset,
        length,
        &mut lastoffset,
        ::std::mem::size_of::<crate::aasfile_h::aas_node_t>() as libc::c_ulong as i32,
    ) as *mut crate::aasfile_h::aas_node_t;
    crate::src::botlib::be_aas_main::aasworld.numnodes = (length as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<crate::aasfile_h::aas_node_t>() as libc::c_ulong)
        as i32;
    if crate::src::botlib::be_aas_main::aasworld.numnodes != 0
        && crate::src::botlib::be_aas_main::aasworld.nodes.is_null()
    {
        return 7 as i32;
    }
    //cluster portals
    offset = header.lumps[11 as i32 as usize].fileofs;
    length = header.lumps[11 as i32 as usize].filelen;
    crate::src::botlib::be_aas_main::aasworld.portals = AAS_LoadAASLump(
        fp,
        offset,
        length,
        &mut lastoffset,
        ::std::mem::size_of::<crate::aasfile_h::aas_portal_t>() as libc::c_ulong as i32,
    )
        as *mut crate::aasfile_h::aas_portal_t;
    crate::src::botlib::be_aas_main::aasworld.numportals = (length as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<crate::aasfile_h::aas_portal_t>() as libc::c_ulong)
        as i32;
    if crate::src::botlib::be_aas_main::aasworld.numportals != 0
        && crate::src::botlib::be_aas_main::aasworld.portals.is_null()
    {
        return 7 as i32;
    }
    //cluster portal index
    offset = header.lumps[12 as i32 as usize].fileofs;
    length = header.lumps[12 as i32 as usize].filelen;
    crate::src::botlib::be_aas_main::aasworld.portalindex = AAS_LoadAASLump(
        fp,
        offset,
        length,
        &mut lastoffset,
        ::std::mem::size_of::<crate::aasfile_h::aas_portalindex_t>() as libc::c_ulong as i32,
    )
        as *mut crate::aasfile_h::aas_portalindex_t;
    crate::src::botlib::be_aas_main::aasworld.portalindexsize = (length as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<crate::aasfile_h::aas_portalindex_t>() as libc::c_ulong)
        as i32;
    if crate::src::botlib::be_aas_main::aasworld.portalindexsize != 0
        && crate::src::botlib::be_aas_main::aasworld
            .portalindex
            .is_null()
    {
        return 7 as i32;
    }
    //clusters
    offset = header.lumps[13 as i32 as usize].fileofs;
    length = header.lumps[13 as i32 as usize].filelen;
    crate::src::botlib::be_aas_main::aasworld.clusters = AAS_LoadAASLump(
        fp,
        offset,
        length,
        &mut lastoffset,
        ::std::mem::size_of::<crate::aasfile_h::aas_cluster_t>() as libc::c_ulong as i32,
    )
        as *mut crate::aasfile_h::aas_cluster_t;
    crate::src::botlib::be_aas_main::aasworld.numclusters = (length as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<crate::aasfile_h::aas_cluster_t>() as libc::c_ulong)
        as i32;
    if crate::src::botlib::be_aas_main::aasworld.numclusters != 0
        && crate::src::botlib::be_aas_main::aasworld.clusters.is_null()
    {
        return 7 as i32;
    }
    //swap everything
    AAS_SwapAASData();
    //aas file is loaded
    crate::src::botlib::be_aas_main::aasworld.loaded = crate::src::qcommon::q_shared::qtrue as i32;
    //close the file
    crate::src::botlib::be_interface::botimport
        .FS_FCloseFile
        .expect("non-null function pointer")(fp);
    //
    //AASFILEDEBUG
    //
    return 0 as i32;
}
//end of the function AAS_LoadAASFile
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================

static mut AAS_WriteAASLump_offset: i32 = 0;
#[no_mangle]

pub unsafe extern "C" fn AAS_WriteAASLump(
    mut fp: crate::src::qcommon::q_shared::fileHandle_t,
    mut h: *mut crate::aasfile_h::aas_header_t,
    mut lumpnum: i32,
    mut data: *mut libc::c_void,
    mut length: i32,
) -> i32 {
    let mut lump: *mut crate::aasfile_h::aas_lump_t = 0 as *mut crate::aasfile_h::aas_lump_t; //LittleLong(ftell(fp));
    lump =
        &mut *(*h).lumps.as_mut_ptr().offset(lumpnum as isize) as *mut crate::aasfile_h::aas_lump_t; //end if
    (*lump).fileofs = AAS_WriteAASLump_offset;
    (*lump).filelen = length;
    if length > 0 as i32 {
        crate::src::botlib::be_interface::botimport
            .FS_Write
            .expect("non-null function pointer")(data, length, fp);
    }
    AAS_WriteAASLump_offset += length;
    return crate::src::qcommon::q_shared::qtrue as i32;
}
//end of the function AAS_WriteAASLump
//===========================================================================
// aas data is useless after writing to file because it is byte swapped
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_WriteAASFile(
    mut filename: *mut libc::c_char,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut header: crate::aasfile_h::aas_header_t = crate::aasfile_h::aas_header_t {
        ident: 0,
        version: 0,
        bspchecksum: 0,
        lumps: [crate::aasfile_h::aas_lump_t {
            fileofs: 0,
            filelen: 0,
        }; 14],
    };
    let mut fp: crate::src::qcommon::q_shared::fileHandle_t = 0;
    crate::src::botlib::be_interface::botimport
        .Print
        .expect("non-null function pointer")(
        1 as i32,
        b"writing %s\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        filename,
    );
    //swap the aas data
    AAS_SwapAASData();
    //initialize the file header
    crate::stdlib::memset(
        &mut header as *mut crate::aasfile_h::aas_header_t as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<crate::aasfile_h::aas_header_t>() as libc::c_ulong,
    );
    header.ident = (('S' as i32) << 24 as i32)
        + (('A' as i32) << 16 as i32)
        + (('A' as i32) << 8 as i32)
        + 'E' as i32;
    header.version = 5 as i32;
    header.bspchecksum = crate::src::botlib::be_aas_main::aasworld.bspchecksum;
    //open a new file
    crate::src::botlib::be_interface::botimport
        .FS_FOpenFile
        .expect("non-null function pointer")(
        filename,
        &mut fp,
        crate::src::qcommon::q_shared::FS_WRITE,
    ); //end if
    if fp == 0 {
        crate::src::botlib::be_interface::botimport
            .Print
            .expect("non-null function pointer")(
            3 as i32,
            b"error opening %s\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            filename,
        );
        return crate::src::qcommon::q_shared::qfalse;
    }
    //write the header
    crate::src::botlib::be_interface::botimport
        .FS_Write
        .expect("non-null function pointer")(
        &mut header as *mut crate::aasfile_h::aas_header_t as *const libc::c_void,
        ::std::mem::size_of::<crate::aasfile_h::aas_header_t>() as libc::c_ulong as i32,
        fp,
    );
    AAS_WriteAASLump_offset =
        ::std::mem::size_of::<crate::aasfile_h::aas_header_t>() as libc::c_ulong as i32;
    //add the data lumps to the file
    if AAS_WriteAASLump(
        fp,
        &mut header,
        0 as i32,
        crate::src::botlib::be_aas_main::aasworld.bboxes as *mut libc::c_void,
        (crate::src::botlib::be_aas_main::aasworld.numbboxes as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<crate::aasfile_h::aas_bbox_t>() as libc::c_ulong)
            as i32,
    ) == 0
    {
        return crate::src::qcommon::q_shared::qfalse;
    }
    if AAS_WriteAASLump(
        fp,
        &mut header,
        1 as i32,
        crate::src::botlib::be_aas_main::aasworld.vertexes as *mut libc::c_void,
        (crate::src::botlib::be_aas_main::aasworld.numvertexes as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<crate::aasfile_h::aas_vertex_t>() as libc::c_ulong)
            as i32,
    ) == 0
    {
        return crate::src::qcommon::q_shared::qfalse;
    }
    if AAS_WriteAASLump(
        fp,
        &mut header,
        2 as i32,
        crate::src::botlib::be_aas_main::aasworld.planes as *mut libc::c_void,
        (crate::src::botlib::be_aas_main::aasworld.numplanes as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<crate::aasfile_h::aas_plane_t>() as libc::c_ulong)
            as i32,
    ) == 0
    {
        return crate::src::qcommon::q_shared::qfalse;
    }
    if AAS_WriteAASLump(
        fp,
        &mut header,
        3 as i32,
        crate::src::botlib::be_aas_main::aasworld.edges as *mut libc::c_void,
        (crate::src::botlib::be_aas_main::aasworld.numedges as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<crate::aasfile_h::aas_edge_t>() as libc::c_ulong)
            as i32,
    ) == 0
    {
        return crate::src::qcommon::q_shared::qfalse;
    }
    if AAS_WriteAASLump(
        fp,
        &mut header,
        4 as i32,
        crate::src::botlib::be_aas_main::aasworld.edgeindex as *mut libc::c_void,
        (crate::src::botlib::be_aas_main::aasworld.edgeindexsize as libc::c_ulong).wrapping_mul(
            ::std::mem::size_of::<crate::aasfile_h::aas_edgeindex_t>() as libc::c_ulong,
        ) as i32,
    ) == 0
    {
        return crate::src::qcommon::q_shared::qfalse;
    }
    if AAS_WriteAASLump(
        fp,
        &mut header,
        5 as i32,
        crate::src::botlib::be_aas_main::aasworld.faces as *mut libc::c_void,
        (crate::src::botlib::be_aas_main::aasworld.numfaces as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<crate::aasfile_h::aas_face_t>() as libc::c_ulong)
            as i32,
    ) == 0
    {
        return crate::src::qcommon::q_shared::qfalse;
    }
    if AAS_WriteAASLump(
        fp,
        &mut header,
        6 as i32,
        crate::src::botlib::be_aas_main::aasworld.faceindex as *mut libc::c_void,
        (crate::src::botlib::be_aas_main::aasworld.faceindexsize as libc::c_ulong).wrapping_mul(
            ::std::mem::size_of::<crate::aasfile_h::aas_faceindex_t>() as libc::c_ulong,
        ) as i32,
    ) == 0
    {
        return crate::src::qcommon::q_shared::qfalse;
    }
    if AAS_WriteAASLump(
        fp,
        &mut header,
        7 as i32,
        crate::src::botlib::be_aas_main::aasworld.areas as *mut libc::c_void,
        (crate::src::botlib::be_aas_main::aasworld.numareas as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<crate::aasfile_h::aas_area_t>() as libc::c_ulong)
            as i32,
    ) == 0
    {
        return crate::src::qcommon::q_shared::qfalse;
    }
    if AAS_WriteAASLump(
        fp,
        &mut header,
        8 as i32,
        crate::src::botlib::be_aas_main::aasworld.areasettings as *mut libc::c_void,
        (crate::src::botlib::be_aas_main::aasworld.numareasettings as libc::c_ulong).wrapping_mul(
            ::std::mem::size_of::<crate::aasfile_h::aas_areasettings_t>() as libc::c_ulong,
        ) as i32,
    ) == 0
    {
        return crate::src::qcommon::q_shared::qfalse;
    }
    if AAS_WriteAASLump(
        fp,
        &mut header,
        9 as i32,
        crate::src::botlib::be_aas_main::aasworld.reachability as *mut libc::c_void,
        (crate::src::botlib::be_aas_main::aasworld.reachabilitysize as libc::c_ulong).wrapping_mul(
            ::std::mem::size_of::<crate::aasfile_h::aas_reachability_t>() as libc::c_ulong,
        ) as i32,
    ) == 0
    {
        return crate::src::qcommon::q_shared::qfalse;
    }
    if AAS_WriteAASLump(
        fp,
        &mut header,
        10 as i32,
        crate::src::botlib::be_aas_main::aasworld.nodes as *mut libc::c_void,
        (crate::src::botlib::be_aas_main::aasworld.numnodes as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<crate::aasfile_h::aas_node_t>() as libc::c_ulong)
            as i32,
    ) == 0
    {
        return crate::src::qcommon::q_shared::qfalse;
    }
    if AAS_WriteAASLump(
        fp,
        &mut header,
        11 as i32,
        crate::src::botlib::be_aas_main::aasworld.portals as *mut libc::c_void,
        (crate::src::botlib::be_aas_main::aasworld.numportals as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<crate::aasfile_h::aas_portal_t>() as libc::c_ulong)
            as i32,
    ) == 0
    {
        return crate::src::qcommon::q_shared::qfalse;
    }
    if AAS_WriteAASLump(
        fp,
        &mut header,
        12 as i32,
        crate::src::botlib::be_aas_main::aasworld.portalindex as *mut libc::c_void,
        (crate::src::botlib::be_aas_main::aasworld.portalindexsize as libc::c_ulong).wrapping_mul(
            ::std::mem::size_of::<crate::aasfile_h::aas_portalindex_t>() as libc::c_ulong,
        ) as i32,
    ) == 0
    {
        return crate::src::qcommon::q_shared::qfalse;
    }
    if AAS_WriteAASLump(
        fp,
        &mut header,
        13 as i32,
        crate::src::botlib::be_aas_main::aasworld.clusters as *mut libc::c_void,
        (crate::src::botlib::be_aas_main::aasworld.numclusters as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<crate::aasfile_h::aas_cluster_t>() as libc::c_ulong)
            as i32,
    ) == 0
    {
        return crate::src::qcommon::q_shared::qfalse;
    }
    //rewrite the header with the added lumps
    crate::src::botlib::be_interface::botimport
        .FS_Seek
        .expect("non-null function pointer")(
        fp,
        0 as i32 as isize,
        crate::src::qcommon::q_shared::FS_SEEK_SET as i32,
    );
    AAS_DData(
        (&mut header as *mut crate::aasfile_h::aas_header_t as *mut u8).offset(8 as i32 as isize),
        (::std::mem::size_of::<crate::aasfile_h::aas_header_t>() as libc::c_ulong)
            .wrapping_sub(8 as i32 as libc::c_ulong) as i32,
    );
    crate::src::botlib::be_interface::botimport
        .FS_Write
        .expect("non-null function pointer")(
        &mut header as *mut crate::aasfile_h::aas_header_t as *const libc::c_void,
        ::std::mem::size_of::<crate::aasfile_h::aas_header_t>() as libc::c_ulong as i32,
        fp,
    );
    //close the file
    crate::src::botlib::be_interface::botimport
        .FS_FCloseFile
        .expect("non-null function pointer")(fp);
    return crate::src::qcommon::q_shared::qtrue;
}
//end of the function AAS_WriteAASFile

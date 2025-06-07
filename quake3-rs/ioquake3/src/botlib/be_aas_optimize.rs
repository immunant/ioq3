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
 * name:		be_aas_optimize.c
 *
 * desc:		decreases the .aas file size after the reachabilities have
 *				been calculated, just dumps all the faces, edges and vertexes
 *
 * $Archive: /MissionPack/code/botlib/be_aas_optimize.c $
 *
 *****************************************************************************/

pub type optimized_t = optimized_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct optimized_s {
    pub numvertexes: libc::c_int,
    pub vertexes: *mut crate::aasfile_h::aas_vertex_t,
    pub numedges: libc::c_int,
    pub edges: *mut crate::aasfile_h::aas_edge_t,
    pub edgeindexsize: libc::c_int,
    pub edgeindex: *mut crate::aasfile_h::aas_edgeindex_t,
    pub numfaces: libc::c_int,
    pub faces: *mut crate::aasfile_h::aas_face_t,
    pub faceindexsize: libc::c_int,
    pub faceindex: *mut crate::aasfile_h::aas_faceindex_t,
    pub numareas: libc::c_int,
    pub areas: *mut crate::aasfile_h::aas_area_t,
    pub vertexoptimizeindex: *mut libc::c_int,
    pub edgeoptimizeindex: *mut libc::c_int,
    pub faceoptimizeindex: *mut libc::c_int,
}
//vertexes
//edges
//edge index
//faces
//face index
//convex areas
//
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_KeepEdge(mut _edge: *mut crate::aasfile_h::aas_edge_t) -> libc::c_int {
    return 1 as libc::c_int;
}
//end of the function AAS_KeepFace
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_OptimizeEdge(
    mut optimized: *mut optimized_t,
    mut edgenum: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0; //end if
    let mut optedgenum: libc::c_int = 0;
    let mut edge: *mut crate::aasfile_h::aas_edge_t = 0 as *mut crate::aasfile_h::aas_edge_t;
    let mut optedge: *mut crate::aasfile_h::aas_edge_t = 0 as *mut crate::aasfile_h::aas_edge_t;
    edge = &mut *crate::src::botlib::be_aas_main::aasworld
        .edges
        .offset(
            (::libc::abs as unsafe extern "C" fn(_: libc::c_int) -> libc::c_int)(edgenum) as isize,
        ) as *mut crate::aasfile_h::aas_edge_t;
    if AAS_KeepEdge(edge) == 0 {
        return 0 as libc::c_int;
    }
    optedgenum = *(*optimized)
        .edgeoptimizeindex
        .offset(::libc::abs(edgenum) as isize);
    if optedgenum != 0 {
        //keep the edge reversed sign
        if edgenum > 0 as libc::c_int {
            return optedgenum;
        } else {
            return -optedgenum;
        }
    } //end for
    optedge = &mut *(*optimized).edges.offset((*optimized).numedges as isize)
        as *mut crate::aasfile_h::aas_edge_t; //end if
    i = 0 as libc::c_int;
    while i < 2 as libc::c_int {
        if *(*optimized)
            .vertexoptimizeindex
            .offset((*edge).v[i as usize] as isize)
            != 0
        {
            (*optedge).v[i as usize] = *(*optimized)
                .vertexoptimizeindex
                .offset((*edge).v[i as usize] as isize)
        } else {
            (*(*optimized)
                .vertexes
                .offset((*optimized).numvertexes as isize))[0 as libc::c_int as usize] =
                (*crate::src::botlib::be_aas_main::aasworld
                    .vertexes
                    .offset((*edge).v[i as usize] as isize))[0 as libc::c_int as usize];
            (*(*optimized)
                .vertexes
                .offset((*optimized).numvertexes as isize))[1 as libc::c_int as usize] =
                (*crate::src::botlib::be_aas_main::aasworld
                    .vertexes
                    .offset((*edge).v[i as usize] as isize))[1 as libc::c_int as usize];
            (*(*optimized)
                .vertexes
                .offset((*optimized).numvertexes as isize))[2 as libc::c_int as usize] =
                (*crate::src::botlib::be_aas_main::aasworld
                    .vertexes
                    .offset((*edge).v[i as usize] as isize))[2 as libc::c_int as usize];
            (*optedge).v[i as usize] = (*optimized).numvertexes;
            *(*optimized)
                .vertexoptimizeindex
                .offset((*edge).v[i as usize] as isize) = (*optimized).numvertexes;
            (*optimized).numvertexes += 1
        }
        i += 1
        //end else
    }
    *(*optimized)
        .edgeoptimizeindex
        .offset(::libc::abs(edgenum) as isize) = (*optimized).numedges;
    optedgenum = (*optimized).numedges;
    (*optimized).numedges += 1;
    //keep the edge reversed sign
    if edgenum > 0 as libc::c_int {
        return optedgenum;
    } else {
        return -optedgenum;
    };
}
//end of the function AAS_OptimizeEdge
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_KeepFace(mut face: *mut crate::aasfile_h::aas_face_t) -> libc::c_int {
    if (*face).faceflags & 2 as libc::c_int == 0 {
        return 0 as libc::c_int;
    } else {
        return 1 as libc::c_int;
    };
}
//end of the function AAS_KeepFace
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_OptimizeFace(
    mut optimized: *mut optimized_t,
    mut facenum: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0; //end if
    let mut edgenum: libc::c_int = 0;
    let mut optedgenum: libc::c_int = 0;
    let mut optfacenum: libc::c_int = 0;
    let mut face: *mut crate::aasfile_h::aas_face_t = 0 as *mut crate::aasfile_h::aas_face_t;
    let mut optface: *mut crate::aasfile_h::aas_face_t = 0 as *mut crate::aasfile_h::aas_face_t;
    face = &mut *crate::src::botlib::be_aas_main::aasworld
        .faces
        .offset(
            (::libc::abs as unsafe extern "C" fn(_: libc::c_int) -> libc::c_int)(facenum) as isize,
        ) as *mut crate::aasfile_h::aas_face_t;
    if AAS_KeepFace(face) == 0 {
        return 0 as libc::c_int;
    }
    optfacenum = *(*optimized)
        .faceoptimizeindex
        .offset(::libc::abs(facenum) as isize);
    if optfacenum != 0 {
        //keep the face side sign
        if facenum > 0 as libc::c_int {
            return optfacenum;
        } else {
            return -optfacenum;
        }
    } //end for
    optface = &mut *(*optimized).faces.offset((*optimized).numfaces as isize)
        as *mut crate::aasfile_h::aas_face_t;
    crate::stdlib::memcpy(
        optface as *mut libc::c_void,
        face as *const libc::c_void,
        ::std::mem::size_of::<crate::aasfile_h::aas_face_t>() as libc::c_ulong,
    );
    (*optface).numedges = 0 as libc::c_int;
    (*optface).firstedge = (*optimized).edgeindexsize;
    i = 0 as libc::c_int;
    while i < (*face).numedges {
        edgenum = *crate::src::botlib::be_aas_main::aasworld
            .edgeindex
            .offset(((*face).firstedge + i) as isize);
        optedgenum = AAS_OptimizeEdge(optimized, edgenum);
        if optedgenum != 0 {
            *(*optimized)
                .edgeindex
                .offset(((*optface).firstedge + (*optface).numedges) as isize) = optedgenum;
            (*optface).numedges += 1;
            (*optimized).edgeindexsize += 1
        }
        i += 1
        //end if
    }
    *(*optimized)
        .faceoptimizeindex
        .offset(::libc::abs(facenum) as isize) = (*optimized).numfaces;
    optfacenum = (*optimized).numfaces;
    (*optimized).numfaces += 1;
    //keep the face side sign
    if facenum > 0 as libc::c_int {
        return optfacenum;
    } else {
        return -optfacenum;
    };
}
//end of the function AAS_OptimizeFace
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_OptimizeArea(
    mut optimized: *mut optimized_t,
    mut areanum: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut facenum: libc::c_int = 0;
    let mut optfacenum: libc::c_int = 0;
    let mut area: *mut crate::aasfile_h::aas_area_t = 0 as *mut crate::aasfile_h::aas_area_t;
    let mut optarea: *mut crate::aasfile_h::aas_area_t = 0 as *mut crate::aasfile_h::aas_area_t;
    area = &mut *crate::src::botlib::be_aas_main::aasworld
        .areas
        .offset(areanum as isize) as *mut crate::aasfile_h::aas_area_t;
    optarea =
        &mut *(*optimized).areas.offset(areanum as isize) as *mut crate::aasfile_h::aas_area_t;
    crate::stdlib::memcpy(
        optarea as *mut libc::c_void,
        area as *const libc::c_void,
        ::std::mem::size_of::<crate::aasfile_h::aas_area_t>() as libc::c_ulong,
    );
    (*optarea).numfaces = 0 as libc::c_int;
    (*optarea).firstface = (*optimized).faceindexsize;
    i = 0 as libc::c_int;
    while i < (*area).numfaces {
        facenum = *crate::src::botlib::be_aas_main::aasworld
            .faceindex
            .offset(((*area).firstface + i) as isize);
        optfacenum = AAS_OptimizeFace(optimized, facenum);
        if optfacenum != 0 {
            *(*optimized)
                .faceindex
                .offset(((*optarea).firstface + (*optarea).numfaces) as isize) = optfacenum;
            (*optarea).numfaces += 1;
            (*optimized).faceindexsize += 1
        }
        i += 1
        //end if
    }
    //end for
}
//end of the function AAS_OptimizeArea
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_OptimizeAlloc(mut optimized: *mut optimized_t) {
    (*optimized).vertexes = crate::src::botlib::l_memory::GetClearedMemory(
        (crate::src::botlib::be_aas_main::aasworld.numvertexes as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<crate::aasfile_h::aas_vertex_t>() as libc::c_ulong),
    ) as *mut crate::aasfile_h::aas_vertex_t; //edge zero is a dummy
    (*optimized).numvertexes = 0 as libc::c_int; //face zero is a dummy
    (*optimized).edges = crate::src::botlib::l_memory::GetClearedMemory(
        (crate::src::botlib::be_aas_main::aasworld.numedges as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<crate::aasfile_h::aas_edge_t>() as libc::c_ulong),
    ) as *mut crate::aasfile_h::aas_edge_t;
    (*optimized).numedges = 1 as libc::c_int;
    (*optimized).edgeindex = crate::src::botlib::l_memory::GetClearedMemory(
        (crate::src::botlib::be_aas_main::aasworld.edgeindexsize as libc::c_ulong).wrapping_mul(
            ::std::mem::size_of::<crate::aasfile_h::aas_edgeindex_t>() as libc::c_ulong,
        ),
    ) as *mut crate::aasfile_h::aas_edgeindex_t;
    (*optimized).edgeindexsize = 0 as libc::c_int;
    (*optimized).faces = crate::src::botlib::l_memory::GetClearedMemory(
        (crate::src::botlib::be_aas_main::aasworld.numfaces as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<crate::aasfile_h::aas_face_t>() as libc::c_ulong),
    ) as *mut crate::aasfile_h::aas_face_t;
    (*optimized).numfaces = 1 as libc::c_int;
    (*optimized).faceindex = crate::src::botlib::l_memory::GetClearedMemory(
        (crate::src::botlib::be_aas_main::aasworld.faceindexsize as libc::c_ulong).wrapping_mul(
            ::std::mem::size_of::<crate::aasfile_h::aas_faceindex_t>() as libc::c_ulong,
        ),
    ) as *mut crate::aasfile_h::aas_faceindex_t;
    (*optimized).faceindexsize = 0 as libc::c_int;
    (*optimized).areas = crate::src::botlib::l_memory::GetClearedMemory(
        (crate::src::botlib::be_aas_main::aasworld.numareas as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<crate::aasfile_h::aas_area_t>() as libc::c_ulong),
    ) as *mut crate::aasfile_h::aas_area_t;
    (*optimized).numareas = crate::src::botlib::be_aas_main::aasworld.numareas;
    //
    (*optimized).vertexoptimizeindex = crate::src::botlib::l_memory::GetClearedMemory(
        (crate::src::botlib::be_aas_main::aasworld.numvertexes as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    (*optimized).edgeoptimizeindex = crate::src::botlib::l_memory::GetClearedMemory(
        (crate::src::botlib::be_aas_main::aasworld.numedges as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    (*optimized).faceoptimizeindex = crate::src::botlib::l_memory::GetClearedMemory(
        (crate::src::botlib::be_aas_main::aasworld.numfaces as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
}
//end of the function AAS_OptimizeAlloc
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_OptimizeStore(mut optimized: *mut optimized_t) {
    //store the optimized vertexes
    if !crate::src::botlib::be_aas_main::aasworld.vertexes.is_null() {
        crate::src::botlib::l_memory::FreeMemory(
            crate::src::botlib::be_aas_main::aasworld.vertexes as *mut libc::c_void,
        );
    }
    crate::src::botlib::be_aas_main::aasworld.vertexes = (*optimized).vertexes;
    crate::src::botlib::be_aas_main::aasworld.numvertexes = (*optimized).numvertexes;
    //store the optimized edges
    if !crate::src::botlib::be_aas_main::aasworld.edges.is_null() {
        crate::src::botlib::l_memory::FreeMemory(
            crate::src::botlib::be_aas_main::aasworld.edges as *mut libc::c_void,
        );
    }
    crate::src::botlib::be_aas_main::aasworld.edges = (*optimized).edges;
    crate::src::botlib::be_aas_main::aasworld.numedges = (*optimized).numedges;
    //store the optimized edge index
    if !crate::src::botlib::be_aas_main::aasworld
        .edgeindex
        .is_null()
    {
        crate::src::botlib::l_memory::FreeMemory(
            crate::src::botlib::be_aas_main::aasworld.edgeindex as *mut libc::c_void,
        );
    }
    crate::src::botlib::be_aas_main::aasworld.edgeindex = (*optimized).edgeindex;
    crate::src::botlib::be_aas_main::aasworld.edgeindexsize = (*optimized).edgeindexsize;
    //store the optimized faces
    if !crate::src::botlib::be_aas_main::aasworld.faces.is_null() {
        crate::src::botlib::l_memory::FreeMemory(
            crate::src::botlib::be_aas_main::aasworld.faces as *mut libc::c_void,
        );
    }
    crate::src::botlib::be_aas_main::aasworld.faces = (*optimized).faces;
    crate::src::botlib::be_aas_main::aasworld.numfaces = (*optimized).numfaces;
    //store the optimized face index
    if !crate::src::botlib::be_aas_main::aasworld
        .faceindex
        .is_null()
    {
        crate::src::botlib::l_memory::FreeMemory(
            crate::src::botlib::be_aas_main::aasworld.faceindex as *mut libc::c_void,
        );
    }
    crate::src::botlib::be_aas_main::aasworld.faceindex = (*optimized).faceindex;
    crate::src::botlib::be_aas_main::aasworld.faceindexsize = (*optimized).faceindexsize;
    //store the optimized areas
    if !crate::src::botlib::be_aas_main::aasworld.areas.is_null() {
        crate::src::botlib::l_memory::FreeMemory(
            crate::src::botlib::be_aas_main::aasworld.areas as *mut libc::c_void,
        );
    }
    crate::src::botlib::be_aas_main::aasworld.areas = (*optimized).areas;
    crate::src::botlib::be_aas_main::aasworld.numareas = (*optimized).numareas;
    //free optimize indexes
    crate::src::botlib::l_memory::FreeMemory((*optimized).vertexoptimizeindex as *mut libc::c_void);
    crate::src::botlib::l_memory::FreeMemory((*optimized).edgeoptimizeindex as *mut libc::c_void);
    crate::src::botlib::l_memory::FreeMemory((*optimized).faceoptimizeindex as *mut libc::c_void);
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
 * name:		be_aas_optimize.h
 *
 * desc:		AAS
 *
 * $Archive: /source/code/botlib/be_aas_optimize.h $
 *
 *****************************************************************************/
/* ****************************************************************************
 * name:		be_aas_optimize.h
 *
 * desc:		AAS
 *
 * $Archive: /source/code/botlib/be_aas_optimize.h $
 *
 *****************************************************************************/
//end of the function AAS_OptimizeStore
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_Optimize() {
    let mut i: libc::c_int = 0; //end for
    let mut sign: libc::c_int = 0;
    let mut optimized: optimized_t = optimized_t {
        numvertexes: 0,
        vertexes: 0 as *mut crate::aasfile_h::aas_vertex_t,
        numedges: 0,
        edges: 0 as *mut crate::aasfile_h::aas_edge_t,
        edgeindexsize: 0,
        edgeindex: 0 as *mut crate::aasfile_h::aas_edgeindex_t,
        numfaces: 0,
        faces: 0 as *mut crate::aasfile_h::aas_face_t,
        faceindexsize: 0,
        faceindex: 0 as *mut crate::aasfile_h::aas_faceindex_t,
        numareas: 0,
        areas: 0 as *mut crate::aasfile_h::aas_area_t,
        vertexoptimizeindex: 0 as *mut libc::c_int,
        edgeoptimizeindex: 0 as *mut libc::c_int,
        faceoptimizeindex: 0 as *mut libc::c_int,
    };
    AAS_OptimizeAlloc(&mut optimized);
    i = 1 as libc::c_int;
    while i < crate::src::botlib::be_aas_main::aasworld.numareas {
        AAS_OptimizeArea(&mut optimized, i);
        i += 1
    }
    //reset the reachability face pointers
    i = 0 as libc::c_int; //end for
    while i < crate::src::botlib::be_aas_main::aasworld.reachabilitysize {
        //NOTE: for TRAVEL_ELEVATOR the facenum is the model number of
        //		the elevator
        if !((*crate::src::botlib::be_aas_main::aasworld
            .reachability
            .offset(i as isize))
        .traveltype
            & 0xffffff as libc::c_int
            == 11 as libc::c_int)
        {
            //NOTE: for TRAVEL_JUMPPAD the facenum is the Z velocity and the edgenum is the hor velocity
            if !((*crate::src::botlib::be_aas_main::aasworld
                .reachability
                .offset(i as isize))
            .traveltype
                & 0xffffff as libc::c_int
                == 18 as libc::c_int)
            {
                //NOTE: for TRAVEL_FUNCBOB the facenum and edgenum contain other coded information
                if !((*crate::src::botlib::be_aas_main::aasworld
                    .reachability
                    .offset(i as isize))
                .traveltype
                    & 0xffffff as libc::c_int
                    == 19 as libc::c_int)
                {
                    //
                    sign = (*crate::src::botlib::be_aas_main::aasworld
                        .reachability
                        .offset(i as isize))
                    .facenum;
                    (*crate::src::botlib::be_aas_main::aasworld
                        .reachability
                        .offset(i as isize))
                    .facenum = *optimized.faceoptimizeindex.offset(::libc::abs(
                        (*crate::src::botlib::be_aas_main::aasworld
                            .reachability
                            .offset(i as isize))
                        .facenum,
                    ) as isize);
                    if sign < 0 as libc::c_int {
                        (*crate::src::botlib::be_aas_main::aasworld
                            .reachability
                            .offset(i as isize))
                        .facenum = -(*crate::src::botlib::be_aas_main::aasworld
                            .reachability
                            .offset(i as isize))
                        .facenum
                    }
                    sign = (*crate::src::botlib::be_aas_main::aasworld
                        .reachability
                        .offset(i as isize))
                    .edgenum;
                    (*crate::src::botlib::be_aas_main::aasworld
                        .reachability
                        .offset(i as isize))
                    .edgenum = *optimized.edgeoptimizeindex.offset(::libc::abs(
                        (*crate::src::botlib::be_aas_main::aasworld
                            .reachability
                            .offset(i as isize))
                        .edgenum,
                    ) as isize);
                    if sign < 0 as libc::c_int {
                        (*crate::src::botlib::be_aas_main::aasworld
                            .reachability
                            .offset(i as isize))
                        .edgenum = -(*crate::src::botlib::be_aas_main::aasworld
                            .reachability
                            .offset(i as isize))
                        .edgenum
                    }
                }
            }
        }
        i += 1
    }
    //store the optimized AAS data into aasworld
    AAS_OptimizeStore(&mut optimized);
    //print some nice stuff :)
    crate::src::botlib::be_interface::botimport
        .Print
        .expect("non-null function pointer")(
        1 as libc::c_int,
        b"AAS data optimized.\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
//end of the function AAS_Optimize

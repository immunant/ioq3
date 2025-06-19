use ::libc;

pub use crate::internal::__builtin_va_list;
pub use crate::internal::__va_list_tag;
pub use crate::stdarg_h::va_list;

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
pub use crate::src::botlib::l_libvar::libvar_s;
pub use crate::src::botlib::l_libvar::libvar_t;
pub use crate::src::botlib::l_libvar::LibVar;
pub use crate::src::botlib::l_libvar::LibVarGetValue;
pub use crate::src::botlib::l_libvar::LibVarSet;
pub use crate::src::botlib::l_libvar::LibVarValue;
pub use crate::src::qcommon::q_math::VectorNormalize;
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
pub use crate::src::qcommon::q_shared::Com_sprintf;
pub use crate::src::qcommon::q_shared::Q_strncpyz;
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
 * name:		be_aas_main.c
 *
 * desc:		AAS
 *
 * $Archive: /MissionPack/code/botlib/be_aas_main.c $
 *
 *****************************************************************************/
#[no_mangle]

pub static mut aasworld: crate::be_aas_def_h::aas_t = crate::be_aas_def_h::aas_t {
    loaded: 0,
    initialized: 0,
    savefile: 0,
    bspchecksum: 0,
    time: 0.,
    numframes: 0,
    filename: [0; 64],
    mapname: [0; 64],
    numbboxes: 0,
    bboxes: 0 as *const crate::aasfile_h::aas_bbox_t as *mut crate::aasfile_h::aas_bbox_t,
    numvertexes: 0,
    vertexes: 0 as *const crate::aasfile_h::aas_vertex_t as *mut crate::aasfile_h::aas_vertex_t,
    numplanes: 0,
    planes: 0 as *const crate::aasfile_h::aas_plane_t as *mut crate::aasfile_h::aas_plane_t,
    numedges: 0,
    edges: 0 as *const crate::aasfile_h::aas_edge_t as *mut crate::aasfile_h::aas_edge_t,
    edgeindexsize: 0,
    edgeindex: 0 as *const crate::aasfile_h::aas_edgeindex_t
        as *mut crate::aasfile_h::aas_edgeindex_t,
    numfaces: 0,
    faces: 0 as *const crate::aasfile_h::aas_face_t as *mut crate::aasfile_h::aas_face_t,
    faceindexsize: 0,
    faceindex: 0 as *const crate::aasfile_h::aas_faceindex_t
        as *mut crate::aasfile_h::aas_faceindex_t,
    numareas: 0,
    areas: 0 as *const crate::aasfile_h::aas_area_t as *mut crate::aasfile_h::aas_area_t,
    numareasettings: 0,
    areasettings: 0 as *const crate::aasfile_h::aas_areasettings_t
        as *mut crate::aasfile_h::aas_areasettings_t,
    reachabilitysize: 0,
    reachability: 0 as *const crate::aasfile_h::aas_reachability_t
        as *mut crate::aasfile_h::aas_reachability_t,
    numnodes: 0,
    nodes: 0 as *const crate::aasfile_h::aas_node_t as *mut crate::aasfile_h::aas_node_t,
    numportals: 0,
    portals: 0 as *const crate::aasfile_h::aas_portal_t as *mut crate::aasfile_h::aas_portal_t,
    portalindexsize: 0,
    portalindex: 0 as *const crate::aasfile_h::aas_portalindex_t
        as *mut crate::aasfile_h::aas_portalindex_t,
    numclusters: 0,
    clusters: 0 as *const crate::aasfile_h::aas_cluster_t as *mut crate::aasfile_h::aas_cluster_t,
    numreachabilityareas: 0,
    reachabilitytime: 0.,
    linkheap: 0 as *const crate::be_aas_def_h::aas_link_t as *mut crate::be_aas_def_h::aas_link_t,
    linkheapsize: 0,
    freelinks: 0 as *const crate::be_aas_def_h::aas_link_t as *mut crate::be_aas_def_h::aas_link_t,
    arealinkedentities: 0 as *const *mut crate::be_aas_def_h::aas_link_t
        as *mut *mut crate::be_aas_def_h::aas_link_t,
    maxentities: 0,
    maxclients: 0,
    entities: 0 as *const crate::be_aas_def_h::aas_entity_t
        as *mut crate::be_aas_def_h::aas_entity_t,
    travelflagfortype: [0; 32],
    areacontentstravelflags: 0 as *const i32 as *mut i32,
    areaupdate: 0 as *const crate::be_aas_def_h::aas_routingupdate_t
        as *mut crate::be_aas_def_h::aas_routingupdate_t,
    portalupdate: 0 as *const crate::be_aas_def_h::aas_routingupdate_t
        as *mut crate::be_aas_def_h::aas_routingupdate_t,
    frameroutingupdates: 0,
    reversedreachability: 0 as *const crate::be_aas_def_h::aas_reversedreachability_t
        as *mut crate::be_aas_def_h::aas_reversedreachability_t,
    areatraveltimes: 0 as *const *mut *mut u16 as *mut *mut *mut u16,
    clusterareacache: 0 as *const *mut *mut crate::be_aas_def_h::aas_routingcache_t
        as *mut *mut *mut crate::be_aas_def_h::aas_routingcache_t,
    portalcache: 0 as *const *mut crate::be_aas_def_h::aas_routingcache_t
        as *mut *mut crate::be_aas_def_h::aas_routingcache_t,
    oldestcache: 0 as *const crate::be_aas_def_h::aas_routingcache_t
        as *mut crate::be_aas_def_h::aas_routingcache_t,
    newestcache: 0 as *const crate::be_aas_def_h::aas_routingcache_t
        as *mut crate::be_aas_def_h::aas_routingcache_t,
    portalmaxtraveltimes: 0 as *const i32 as *mut i32,
    reachabilityareaindex: 0 as *const i32 as *mut i32,
    reachabilityareas: 0 as *const crate::be_aas_def_h::aas_reachabilityareas_t
        as *mut crate::be_aas_def_h::aas_reachabilityareas_t,
};
#[no_mangle]

pub static mut saveroutingcache: *mut crate::src::botlib::l_libvar::libvar_t = 0
    as *const crate::src::botlib::l_libvar::libvar_t
    as *mut crate::src::botlib::l_libvar::libvar_t;
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_Error(mut fmt: *mut libc::c_char, mut args: ...) {
    let mut str: [libc::c_char; 1024] = [0; 1024];
    let mut arglist: ::std::ffi::VaListImpl;
    arglist = args.clone();
    crate::stdlib::vsnprintf(
        str.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
        fmt,
        arglist.as_va_list(),
    );
    crate::src::botlib::be_interface::botimport
        .Print
        .expect("non-null function pointer")(
        4 as i32,
        b"%s\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        str.as_mut_ptr(),
    );
}
//end of the function AAS_Error
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_Loaded() -> i32 {
    return aasworld.loaded;
}
//end of the function AAS_Loaded
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_Initialized() -> i32 {
    return aasworld.initialized;
}
//end of the function AAS_Initialized
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_SetInitialized() {
    aasworld.initialized = crate::src::qcommon::q_shared::qtrue as i32;
    crate::src::botlib::be_interface::botimport
        .Print
        .expect("non-null function pointer")(
        1 as i32,
        b"AAS initialized.\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
//end of the function AAS_SetInitialized
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_ContinueInit(mut time: f32) {
    //if no AAS file loaded
    if aasworld.loaded == 0 {
        return;
    }
    //if AAS is already initialized
    if aasworld.initialized != 0 {
        return;
    }
    //calculate reachability, if not finished return
    if crate::src::botlib::be_aas_reach::AAS_ContinueInitReachability(time) != 0 {
        return;
    }
    //initialize clustering for the new map
    crate::src::botlib::be_aas_cluster::AAS_InitClustering();
    //if reachability has been calculated and an AAS file should be written
    //or there is a forced data optimization
    if aasworld.savefile != 0
        || crate::src::botlib::l_libvar::LibVarGetValue(
            b"forcewrite\x00" as *const u8 as *const libc::c_char,
        ) as i32
            != 0
    {
        //end if
        //optimize the AAS data
        if crate::src::botlib::l_libvar::LibVarValue(
            b"aasoptimize\x00" as *const u8 as *const libc::c_char,
            b"0\x00" as *const u8 as *const libc::c_char,
        ) as i32
            != 0
        {
            crate::src::botlib::be_aas_optimize::AAS_Optimize();
        }
        //end else
        if crate::src::botlib::be_aas_file::AAS_WriteAASFile(aasworld.filename.as_mut_ptr()) as u64
            != 0
        {
            //save the AAS file
            crate::src::botlib::be_interface::botimport
                .Print
                .expect("non-null function pointer")(
                1 as i32,
                b"%s written successfully\n\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                aasworld.filename.as_mut_ptr(),
            ); //end if
        } else {
            crate::src::botlib::be_interface::botimport
                .Print
                .expect("non-null function pointer")(
                3 as i32,
                b"couldn\'t write %s\n\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                aasworld.filename.as_mut_ptr(),
            );
        }
    }
    //initialize the routing
    crate::src::botlib::be_aas_route::AAS_InitRouting();
    //at this point AAS is initialized
    AAS_SetInitialized();
}
//end of the function AAS_ContinueInit
//===========================================================================
// called at the start of every frame
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_StartFrame(mut time: f32) -> i32 {
    aasworld.time = time;
    //unlink all entities that were not updated last frame
    crate::src::botlib::be_aas_entity::AAS_UnlinkInvalidEntities();
    //invalidate the entities
    crate::src::botlib::be_aas_entity::AAS_InvalidateEntities();
    //initialize AAS
    AAS_ContinueInit(time);
    //
    aasworld.frameroutingupdates = 0 as i32;
    //
    if crate::src::botlib::be_interface::botDeveloper != 0 {
        //end if
        if crate::src::botlib::l_libvar::LibVarGetValue(
            b"showcacheupdates\x00" as *const u8 as *const libc::c_char,
        ) != 0.
        {
            crate::src::botlib::be_aas_route::AAS_RoutingInfo(); //end if
            crate::src::botlib::l_libvar::LibVarSet(
                b"showcacheupdates\x00" as *const u8 as *const libc::c_char,
                b"0\x00" as *const u8 as *const libc::c_char,
            );
        }
        //end if
        if crate::src::botlib::l_libvar::LibVarGetValue(
            b"showmemoryusage\x00" as *const u8 as *const libc::c_char,
        ) != 0.
        {
            crate::src::botlib::l_memory::PrintUsedMemorySize(); //end if
            crate::src::botlib::l_libvar::LibVarSet(
                b"showmemoryusage\x00" as *const u8 as *const libc::c_char,
                b"0\x00" as *const u8 as *const libc::c_char,
            );
        }
        if crate::src::botlib::l_libvar::LibVarGetValue(
            b"memorydump\x00" as *const u8 as *const libc::c_char,
        ) != 0.
        {
            crate::src::botlib::l_memory::PrintMemoryLabels();
            crate::src::botlib::l_libvar::LibVarSet(
                b"memorydump\x00" as *const u8 as *const libc::c_char,
                b"0\x00" as *const u8 as *const libc::c_char,
            );
        }
    }
    //
    if (*saveroutingcache).value != 0. {
        crate::src::botlib::be_aas_route::AAS_WriteRouteCache(); //end if
        crate::src::botlib::l_libvar::LibVarSet(
            b"saveroutingcache\x00" as *const u8 as *const libc::c_char,
            b"0\x00" as *const u8 as *const libc::c_char,
        );
    }
    //
    aasworld.numframes += 1;
    return 0 as i32;
}
//end of the function AAS_StartFrame
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_Time() -> f32 {
    return aasworld.time;
}
//start a new time frame
//AASINTERN
//AASINTERN
//returns true if AAS is initialized
//returns true if AAS is initialized
//returns true if the AAS file is loaded
//returns true if the AAS file is loaded
//returns the current time
//returns the current time
//
//
//end of the function AAS_Time
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_ProjectPointOntoVector(
    mut point: *mut crate::src::qcommon::q_shared::vec_t,
    mut vStart: *mut crate::src::qcommon::q_shared::vec_t,
    mut vEnd: *mut crate::src::qcommon::q_shared::vec_t,
    mut vProj: *mut crate::src::qcommon::q_shared::vec_t,
) {
    let mut pVec: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut vec: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    pVec[0 as i32 as usize] =
        *point.offset(0 as i32 as isize) - *vStart.offset(0 as i32 as isize);
    pVec[1 as i32 as usize] =
        *point.offset(1 as i32 as isize) - *vStart.offset(1 as i32 as isize);
    pVec[2 as i32 as usize] =
        *point.offset(2 as i32 as isize) - *vStart.offset(2 as i32 as isize);
    vec[0 as i32 as usize] =
        *vEnd.offset(0 as i32 as isize) - *vStart.offset(0 as i32 as isize);
    vec[1 as i32 as usize] =
        *vEnd.offset(1 as i32 as isize) - *vStart.offset(1 as i32 as isize);
    vec[2 as i32 as usize] =
        *vEnd.offset(2 as i32 as isize) - *vStart.offset(2 as i32 as isize);
    crate::src::qcommon::q_math::VectorNormalize(vec.as_mut_ptr());
    // project onto the directional vector for this segment
    *vProj.offset(0 as i32 as isize) = *vStart.offset(0 as i32 as isize)
        + vec[0 as i32 as usize]
            * (pVec[0 as i32 as usize] * vec[0 as i32 as usize]
                + pVec[1 as i32 as usize] * vec[1 as i32 as usize]
                + pVec[2 as i32 as usize] * vec[2 as i32 as usize]);
    *vProj.offset(1 as i32 as isize) = *vStart.offset(1 as i32 as isize)
        + vec[1 as i32 as usize]
            * (pVec[0 as i32 as usize] * vec[0 as i32 as usize]
                + pVec[1 as i32 as usize] * vec[1 as i32 as usize]
                + pVec[2 as i32 as usize] * vec[2 as i32 as usize]);
    *vProj.offset(2 as i32 as isize) = *vStart.offset(2 as i32 as isize)
        + vec[2 as i32 as usize]
            * (pVec[0 as i32 as usize] * vec[0 as i32 as usize]
                + pVec[1 as i32 as usize] * vec[1 as i32 as usize]
                + pVec[2 as i32 as usize] * vec[2 as i32 as usize]);
}
//end of the function AAS_ProjectPointOntoVector
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_LoadFiles(mut mapname: *const libc::c_char) -> i32 {
    let mut errnum: i32 = 0;
    let mut aasfile: [libc::c_char; 64] = [0; 64];
    crate::src::qcommon::q_shared::Q_strncpyz(
        aasworld.mapname.as_mut_ptr(),
        mapname,
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as i32,
    );
    //NOTE: first reset the entity links into the AAS areas and BSP leaves
    // the AAS link heap and BSP link heap are reset after respectively the
    // AAS file and BSP file are loaded
    crate::src::botlib::be_aas_entity::AAS_ResetEntityLinks();
    // load bsp info
    crate::src::botlib::be_aas_bspq3::AAS_LoadBSPFile();
    //load the aas file
    crate::src::qcommon::q_shared::Com_sprintf(
        aasfile.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as i32,
        b"maps/%s.aas\x00" as *const u8 as *const libc::c_char,
        mapname,
    );
    errnum = crate::src::botlib::be_aas_file::AAS_LoadAASFile(aasfile.as_mut_ptr());
    if errnum != 0 as i32 {
        return errnum;
    }
    crate::src::botlib::be_interface::botimport
        .Print
        .expect("non-null function pointer")(
        1 as i32,
        b"loaded %s\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        aasfile.as_mut_ptr(),
    );
    crate::src::qcommon::q_shared::Q_strncpyz(
        aasworld.filename.as_mut_ptr(),
        aasfile.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as i32,
    );
    return 0 as i32;
}
//start a new map
//end of the function AAS_LoadFiles
//===========================================================================
// called every time a map changes
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_LoadMap(mut mapname: *const libc::c_char) -> i32 {
    let mut errnum: i32 = 0;
    //if no mapname is provided then the string indexes are updated
    if mapname.is_null() {
        return 0 as i32;
    } //end if
      //
    aasworld.initialized = crate::src::qcommon::q_shared::qfalse as i32;
    //NOTE: free the routing caches before loading a new map because
    // to free the caches the old number of areas, number of clusters
    // and number of areas in a clusters must be available
    crate::src::botlib::be_aas_route::AAS_FreeRoutingCaches();
    //load the map
    errnum = AAS_LoadFiles(mapname); //end if
    if errnum != 0 as i32 {
        aasworld.loaded = crate::src::qcommon::q_shared::qfalse as i32;
        return errnum;
    }
    //
    crate::src::botlib::be_aas_move::AAS_InitSettings();
    //initialize the AAS link heap for the new map
    crate::src::botlib::be_aas_sample::AAS_InitAASLinkHeap();
    //initialize the AAS linked entities for the new map
    crate::src::botlib::be_aas_sample::AAS_InitAASLinkedEntities();
    //initialize reachability for the new map
    crate::src::botlib::be_aas_reach::AAS_InitReachability();
    //initialize the alternative routing
    crate::src::botlib::be_aas_routealt::AAS_InitAlternativeRouting();
    //everything went ok
    return 0 as i32;
}
//end of the function AAS_LoadMap
//===========================================================================
// called when the library is first loaded
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_Setup() -> i32 {
    aasworld.maxclients = crate::src::botlib::l_libvar::LibVarValue(
        b"maxclients\x00" as *const u8 as *const libc::c_char,
        b"128\x00" as *const u8 as *const libc::c_char,
    ) as i32;
    aasworld.maxentities = crate::src::botlib::l_libvar::LibVarValue(
        b"maxentities\x00" as *const u8 as *const libc::c_char,
        b"1024\x00" as *const u8 as *const libc::c_char,
    ) as i32;
    // as soon as it's set to 1 the routing cache will be saved
    saveroutingcache = crate::src::botlib::l_libvar::LibVar(
        b"saveroutingcache\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
    ) as *mut crate::src::botlib::l_libvar::libvar_s;
    //allocate memory for the entities
    if !aasworld.entities.is_null() {
        crate::src::botlib::l_memory::FreeMemory(aasworld.entities as *mut libc::c_void);
    }
    aasworld.entities = crate::src::botlib::l_memory::GetClearedHunkMemory(
        (aasworld.maxentities as libc::c_ulong).wrapping_mul(::std::mem::size_of::<
            crate::be_aas_def_h::aas_entity_t,
        >() as libc::c_ulong),
    ) as *mut crate::be_aas_def_h::aas_entity_t;
    //invalidate all the entities
    crate::src::botlib::be_aas_entity::AAS_InvalidateEntities();
    //force some recalculations
    //LibVarSet("forceclustering", "1");			//force clustering calculation
    //LibVarSet("forcereachability", "1");		//force reachability calculation
    aasworld.numframes = 0 as i32;
    return 0 as i32;
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
 * name:		be_aas_main.h
 *
 * desc:		AAS
 *
 * $Archive: /source/code/botlib/be_aas_main.h $
 *
 *****************************************************************************/
/* ****************************************************************************
 * name:		be_aas_main.h
 *
 * desc:		AAS
 *
 * $Archive: /source/code/botlib/be_aas_main.h $
 *
 *****************************************************************************/
//AAS error message
//set AAS initialized
//setup AAS with the given number of entities and clients
//shutdown AAS
//end of the function AAS_Setup
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_Shutdown() {
    crate::src::botlib::be_aas_routealt::AAS_ShutdownAlternativeRouting();
    //
    crate::src::botlib::be_aas_bspq3::AAS_DumpBSPData();
    //free routing caches
    crate::src::botlib::be_aas_route::AAS_FreeRoutingCaches();
    //free aas link heap
    crate::src::botlib::be_aas_sample::AAS_FreeAASLinkHeap();
    //free aas linked entities
    crate::src::botlib::be_aas_sample::AAS_FreeAASLinkedEntities();
    //free the aas data
    crate::src::botlib::be_aas_file::AAS_DumpAASData();
    //free the entities
    if !aasworld.entities.is_null() {
        crate::src::botlib::l_memory::FreeMemory(aasworld.entities as *mut libc::c_void);
    }
    //clear the aasworld structure
    crate::stdlib::memset(
        &mut aasworld as *mut crate::be_aas_def_h::aas_t as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<crate::be_aas_def_h::aas_t>() as libc::c_ulong,
    );
    //aas has not been initialized
    aasworld.initialized = crate::src::qcommon::q_shared::qfalse as i32;
    //NOTE: as soon as a new .bsp file is loaded the .bsp file memory is
    // freed and reallocated, so there's no need to free that memory here
    //print shutdown
    crate::src::botlib::be_interface::botimport
        .Print
        .expect("non-null function pointer")(
        1 as i32,
        b"AAS shutdown.\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
//end of the function AAS_Shutdown

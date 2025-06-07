use ::libc;

pub mod q_shared_h {

    #[inline]

    pub unsafe extern "C" fn VectorLength(
        mut v: *const crate::src::qcommon::q_shared::vec_t,
    ) -> crate::src::qcommon::q_shared::vec_t {
        return crate::stdlib::sqrt(
            (*v.offset(0 as libc::c_int as isize) * *v.offset(0 as libc::c_int as isize)
                + *v.offset(1 as libc::c_int as isize) * *v.offset(1 as libc::c_int as isize)
                + *v.offset(2 as libc::c_int as isize) * *v.offset(2 as libc::c_int as isize))
                as libc::c_double,
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
pub use crate::be_aas_h::aas_entityinfo_s;
pub use crate::be_aas_h::aas_entityinfo_t;
pub use crate::be_aas_h::aas_predictroute_s;
pub use crate::be_aas_h::aas_trace_s;
pub use crate::be_aas_h::aas_trace_t;
pub use crate::botlib_h::botlib_import_s;
pub use crate::botlib_h::botlib_import_t;
pub use crate::botlib_h::bsp_surface_s;
pub use crate::botlib_h::bsp_surface_t;
pub use crate::botlib_h::bsp_trace_s;
pub use crate::botlib_h::bsp_trace_t;
pub use crate::src::botlib::be_aas_route::q_shared_h::VectorLength;

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
pub use crate::src::qcommon::q_shared::FS_APPEND;
pub use crate::src::qcommon::q_shared::FS_APPEND_SYNC;
pub use crate::src::qcommon::q_shared::FS_READ;
pub use crate::src::qcommon::q_shared::FS_WRITE;

//end of the function AAS_CreateAllRoutingCache
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
//the route cache header
//this header is followed by numportalcache + numareacache aas_routingcache_t
//structures that store routing cache

pub type routecacheheader_t = routecacheheader_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct routecacheheader_s {
    pub ident: libc::c_int,
    pub version: libc::c_int,
    pub numareas: libc::c_int,
    pub numclusters: libc::c_int,
    pub areacrc: libc::c_int,
    pub clustercrc: libc::c_int,
    pub numportalcache: libc::c_int,
    pub numareacache: libc::c_int,
}
//walk speed = 300
//cache refresh time
//15 seconds refresh time
//maximum number of routing updates each frame
/*

  area routing cache:
  stores the distances within one cluster to a specific goal area
  this goal area is in this same cluster and could be a cluster portal
  for every cluster there's a list with routing cache for every area
  in that cluster (including the portals of that cluster)
  area cache stores aasworld.clusters[?].numreachabilityareas travel times

  portal routing cache:
  stores the distances of all portals to a specific goal area
  this goal area could be in any cluster and could also be a cluster portal
  for every area (aasworld.numareas) the portal cache stores
  aasworld.numportals travel times

*/
#[no_mangle]

pub static mut numareacacheupdates: libc::c_int = 0;
#[no_mangle]

pub static mut numportalcacheupdates: libc::c_int = 0;
//ROUTING_DEBUG
#[no_mangle]

pub static mut routingcachesize: libc::c_int = 0;
#[no_mangle]

pub static mut max_routingcachesize: libc::c_int = 0;
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_RoutingInfo() {
    crate::src::botlib::be_interface::botimport
        .Print
        .expect("non-null function pointer")(
        1 as libc::c_int,
        b"%d area cache updates\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        numareacacheupdates,
    );
    crate::src::botlib::be_interface::botimport
        .Print
        .expect("non-null function pointer")(
        1 as libc::c_int,
        b"%d portal cache updates\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        numportalcacheupdates,
    );
    crate::src::botlib::be_interface::botimport
        .Print
        .expect("non-null function pointer")(
        1 as libc::c_int,
        b"%d bytes routing cache\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        routingcachesize,
    );
}
//end of the function AAS_RoutingInfo
//ROUTING_DEBUG
//===========================================================================
// returns the number of the area in the cluster
// assumes the given area is in the given cluster or a portal of the cluster
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[inline]

unsafe extern "C" fn AAS_ClusterAreaNum(
    mut cluster: libc::c_int,
    mut areanum: libc::c_int,
) -> libc::c_int {
    let mut side: libc::c_int = 0;
    let mut areacluster: libc::c_int = 0;
    areacluster = (*crate::src::botlib::be_aas_main::aasworld
        .areasettings
        .offset(areanum as isize))
    .cluster;
    if areacluster > 0 as libc::c_int {
        return (*crate::src::botlib::be_aas_main::aasworld
            .areasettings
            .offset(areanum as isize))
        .clusterareanum;
    } else {
        /*#ifdef ROUTING_DEBUG
                if (aasworld.portals[-areacluster].frontcluster != cluster &&
                        aasworld.portals[-areacluster].backcluster != cluster)
                {
                    botimport.Print(PRT_ERROR, "portal %d: does not belong to cluster %d\n"
                                                    , -areacluster, cluster);
                } //end if
        #endif //ROUTING_DEBUG*/
        side = ((*crate::src::botlib::be_aas_main::aasworld
            .portals
            .offset(-areacluster as isize))
        .frontcluster
            != cluster) as libc::c_int;
        return (*crate::src::botlib::be_aas_main::aasworld
            .portals
            .offset(-areacluster as isize))
        .clusterareanum[side as usize];
    };
    //end else
}
//end of the function AAS_ClusterAreaNum
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_InitTravelFlagFromType() {
    let mut i: libc::c_int = 0; //end for
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        crate::src::botlib::be_aas_main::aasworld.travelflagfortype[i as usize] =
            0x1 as libc::c_int;
        i += 1
    }
    crate::src::botlib::be_aas_main::aasworld.travelflagfortype[1 as libc::c_int as usize] =
        0x1 as libc::c_int;
    crate::src::botlib::be_aas_main::aasworld.travelflagfortype[2 as libc::c_int as usize] =
        0x2 as libc::c_int;
    crate::src::botlib::be_aas_main::aasworld.travelflagfortype[3 as libc::c_int as usize] =
        0x4 as libc::c_int;
    crate::src::botlib::be_aas_main::aasworld.travelflagfortype[4 as libc::c_int as usize] =
        0x8 as libc::c_int;
    crate::src::botlib::be_aas_main::aasworld.travelflagfortype[5 as libc::c_int as usize] =
        0x10 as libc::c_int;
    crate::src::botlib::be_aas_main::aasworld.travelflagfortype[6 as libc::c_int as usize] =
        0x20 as libc::c_int;
    crate::src::botlib::be_aas_main::aasworld.travelflagfortype[7 as libc::c_int as usize] =
        0x80 as libc::c_int;
    crate::src::botlib::be_aas_main::aasworld.travelflagfortype[8 as libc::c_int as usize] =
        0x100 as libc::c_int;
    crate::src::botlib::be_aas_main::aasworld.travelflagfortype[9 as libc::c_int as usize] =
        0x200 as libc::c_int;
    crate::src::botlib::be_aas_main::aasworld.travelflagfortype[10 as libc::c_int as usize] =
        0x400 as libc::c_int;
    crate::src::botlib::be_aas_main::aasworld.travelflagfortype[11 as libc::c_int as usize] =
        0x800 as libc::c_int;
    crate::src::botlib::be_aas_main::aasworld.travelflagfortype[12 as libc::c_int as usize] =
        0x1000 as libc::c_int;
    crate::src::botlib::be_aas_main::aasworld.travelflagfortype[13 as libc::c_int as usize] =
        0x2000 as libc::c_int;
    crate::src::botlib::be_aas_main::aasworld.travelflagfortype[14 as libc::c_int as usize] =
        0x4000 as libc::c_int;
    crate::src::botlib::be_aas_main::aasworld.travelflagfortype[15 as libc::c_int as usize] =
        0x8000 as libc::c_int;
    crate::src::botlib::be_aas_main::aasworld.travelflagfortype[16 as libc::c_int as usize] =
        0x10000 as libc::c_int;
    crate::src::botlib::be_aas_main::aasworld.travelflagfortype[17 as libc::c_int as usize] =
        0x20000 as libc::c_int;
    crate::src::botlib::be_aas_main::aasworld.travelflagfortype[18 as libc::c_int as usize] =
        0x40000 as libc::c_int;
    crate::src::botlib::be_aas_main::aasworld.travelflagfortype[19 as libc::c_int as usize] =
        0x1000000 as libc::c_int;
}
//end of the function AAS_InitTravelFlagFromType
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[inline]

unsafe extern "C" fn AAS_TravelFlagForType_inline(mut traveltype: libc::c_int) -> libc::c_int {
    let mut tfl: libc::c_int = 0 as libc::c_int;
    if traveltype & (1 as libc::c_int) << 24 as libc::c_int != 0 {
        tfl |= 0x8000000 as libc::c_int
    }
    if traveltype & (2 as libc::c_int) << 24 as libc::c_int != 0 {
        tfl |= 0x10000000 as libc::c_int
    }
    traveltype &= 0xffffff as libc::c_int;
    if traveltype < 0 as libc::c_int || traveltype >= 32 as libc::c_int {
        return 0x1 as libc::c_int;
    }
    tfl |= crate::src::botlib::be_aas_main::aasworld.travelflagfortype[traveltype as usize];
    return tfl;
}
//end of the function AAS_TravelFlagForType_inline
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_TravelFlagForType(mut traveltype: libc::c_int) -> libc::c_int {
    return AAS_TravelFlagForType_inline(traveltype);
}
//end of the function AAS_TravelFlagForType_inline
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_UnlinkCache(mut cache: *mut crate::be_aas_def_h::aas_routingcache_t) {
    if !(*cache).time_next.is_null() {
        (*(*cache).time_next).time_prev = (*cache).time_prev
    } else {
        crate::src::botlib::be_aas_main::aasworld.newestcache = (*cache).time_prev
    }
    if !(*cache).time_prev.is_null() {
        (*(*cache).time_prev).time_next = (*cache).time_next
    } else {
        crate::src::botlib::be_aas_main::aasworld.oldestcache = (*cache).time_next
    }
    (*cache).time_next = 0 as *mut crate::be_aas_def_h::aas_routingcache_s;
    (*cache).time_prev = 0 as *mut crate::be_aas_def_h::aas_routingcache_s;
}
//end of the function AAS_UnlinkCache
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_LinkCache(mut cache: *mut crate::be_aas_def_h::aas_routingcache_t) {
    if !crate::src::botlib::be_aas_main::aasworld
        .newestcache
        .is_null()
    {
        //end else
        (*crate::src::botlib::be_aas_main::aasworld.newestcache).time_next = cache; //end if
        (*cache).time_prev = crate::src::botlib::be_aas_main::aasworld.newestcache
    } else {
        crate::src::botlib::be_aas_main::aasworld.oldestcache = cache;
        (*cache).time_prev = 0 as *mut crate::be_aas_def_h::aas_routingcache_s
    }
    (*cache).time_next = 0 as *mut crate::be_aas_def_h::aas_routingcache_s;
    crate::src::botlib::be_aas_main::aasworld.newestcache = cache;
}
//end of the function AAS_LinkCache
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_FreeRoutingCache(
    mut cache: *mut crate::be_aas_def_h::aas_routingcache_t,
) {
    AAS_UnlinkCache(cache);
    routingcachesize -= (*cache).size;
    crate::src::botlib::l_memory::FreeMemory(cache as *mut libc::c_void);
}
//end of the function AAS_FreeRoutingCache
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_RemoveRoutingCacheInCluster(mut clusternum: libc::c_int) {
    let mut i: libc::c_int = 0; //end for
    let mut cache: *mut crate::be_aas_def_h::aas_routingcache_t =
        0 as *mut crate::be_aas_def_h::aas_routingcache_t;
    let mut nextcache: *mut crate::be_aas_def_h::aas_routingcache_t =
        0 as *mut crate::be_aas_def_h::aas_routingcache_t;
    let mut cluster: *mut crate::aasfile_h::aas_cluster_t =
        0 as *mut crate::aasfile_h::aas_cluster_t;
    if crate::src::botlib::be_aas_main::aasworld
        .clusterareacache
        .is_null()
    {
        return;
    }
    cluster = &mut *crate::src::botlib::be_aas_main::aasworld
        .clusters
        .offset(clusternum as isize) as *mut crate::aasfile_h::aas_cluster_t;
    i = 0 as libc::c_int;
    while i < (*cluster).numareas {
        cache = *(*crate::src::botlib::be_aas_main::aasworld
            .clusterareacache
            .offset(clusternum as isize))
        .offset(i as isize);
        while !cache.is_null() {
            nextcache = (*cache).next;
            AAS_FreeRoutingCache(cache);
            cache = nextcache
        }
        let ref mut fresh0 = *(*crate::src::botlib::be_aas_main::aasworld
            .clusterareacache
            .offset(clusternum as isize))
        .offset(i as isize);
        *fresh0 = 0 as *mut crate::be_aas_def_h::aas_routingcache_t;
        i += 1
    }
    //end for
}
//end of the function AAS_RemoveRoutingCacheInCluster
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_RemoveRoutingCacheUsingArea(mut areanum: libc::c_int) {
    let mut i: libc::c_int = 0; //end else
    let mut clusternum: libc::c_int = 0; //end if
    let mut cache: *mut crate::be_aas_def_h::aas_routingcache_t =
        0 as *mut crate::be_aas_def_h::aas_routingcache_t;
    let mut nextcache: *mut crate::be_aas_def_h::aas_routingcache_t =
        0 as *mut crate::be_aas_def_h::aas_routingcache_t;
    clusternum = (*crate::src::botlib::be_aas_main::aasworld
        .areasettings
        .offset(areanum as isize))
    .cluster;
    if clusternum > 0 as libc::c_int {
        //remove all the cache in the cluster the area is in
        AAS_RemoveRoutingCacheInCluster(clusternum);
    } else {
        // if this is a portal remove all cache in both the front and back cluster
        AAS_RemoveRoutingCacheInCluster(
            (*crate::src::botlib::be_aas_main::aasworld
                .portals
                .offset(-clusternum as isize))
            .frontcluster,
        );
        AAS_RemoveRoutingCacheInCluster(
            (*crate::src::botlib::be_aas_main::aasworld
                .portals
                .offset(-clusternum as isize))
            .backcluster,
        );
    }
    // remove all portal cache
    i = 0 as libc::c_int;
    while i < crate::src::botlib::be_aas_main::aasworld.numareas {
        //refresh portal cache
        cache = *crate::src::botlib::be_aas_main::aasworld
            .portalcache
            .offset(i as isize); //end for
        while !cache.is_null() {
            nextcache = (*cache).next;
            AAS_FreeRoutingCache(cache);
            cache = nextcache
        }
        let ref mut fresh1 = *crate::src::botlib::be_aas_main::aasworld
            .portalcache
            .offset(i as isize);
        *fresh1 = 0 as *mut crate::be_aas_def_h::aas_routingcache_t;
        i += 1
    }
    //end for
}
//end of the function AAS_RemoveRoutingCacheUsingArea
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_EnableRoutingArea(
    mut areanum: libc::c_int,
    mut enable: libc::c_int,
) -> libc::c_int {
    let mut flags: libc::c_int = 0; //end if
    if areanum <= 0 as libc::c_int || areanum >= crate::src::botlib::be_aas_main::aasworld.numareas
    {
        if crate::src::botlib::be_interface::botDeveloper != 0 {
            crate::src::botlib::be_interface::botimport
                .Print
                .expect("non-null function pointer")(
                3 as libc::c_int,
                b"AAS_EnableRoutingArea: areanum %d out of range\n\x00" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                areanum,
            ); //end if
        }
        return 0 as libc::c_int;
    }
    flags = (*crate::src::botlib::be_aas_main::aasworld
        .areasettings
        .offset(areanum as isize))
    .areaflags
        & 8 as libc::c_int;
    if enable < 0 as libc::c_int {
        return (flags == 0) as libc::c_int;
    }
    if enable != 0 {
        (*crate::src::botlib::be_aas_main::aasworld
            .areasettings
            .offset(areanum as isize))
        .areaflags &= !(8 as libc::c_int)
    } else {
        (*crate::src::botlib::be_aas_main::aasworld
            .areasettings
            .offset(areanum as isize))
        .areaflags |= 8 as libc::c_int
    }
    // if the status of the area changed
    if flags & 8 as libc::c_int
        != (*crate::src::botlib::be_aas_main::aasworld
            .areasettings
            .offset(areanum as isize))
        .areaflags
            & 8 as libc::c_int
    {
        //end if
        //remove all routing cache involving this area
        AAS_RemoveRoutingCacheUsingArea(areanum);
    }
    return (flags == 0) as libc::c_int;
}
//end of the function AAS_EnableRoutingArea
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[inline]

unsafe extern "C" fn AAS_RoutingTime() -> libc::c_float {
    return crate::src::botlib::be_aas_main::AAS_Time();
}
//end of the function AAS_RoutingTime
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_GetAreaContentsTravelFlags(mut areanum: libc::c_int) -> libc::c_int {
    let mut contents: libc::c_int = 0;
    let mut tfl: libc::c_int = 0;
    contents = (*crate::src::botlib::be_aas_main::aasworld
        .areasettings
        .offset(areanum as isize))
    .contents;
    tfl = 0 as libc::c_int;
    if contents & 1 as libc::c_int != 0 {
        tfl |= 0x100000 as libc::c_int
    } else if contents & 4 as libc::c_int != 0 {
        tfl |= 0x200000 as libc::c_int
    } else if contents & 2 as libc::c_int != 0 {
        tfl |= 0x400000 as libc::c_int
    } else {
        tfl |= 0x80000 as libc::c_int
    }
    if contents & 256 as libc::c_int != 0 {
        tfl |= 0x800000 as libc::c_int
    }
    if contents & 2048 as libc::c_int != 0 {
        tfl |= 0x8000000 as libc::c_int
    }
    if contents & 4096 as libc::c_int != 0 {
        tfl |= 0x10000000 as libc::c_int
    }
    if (*crate::src::botlib::be_aas_main::aasworld
        .areasettings
        .offset(areanum as isize))
    .areaflags
        & 16 as libc::c_int
        != 0
    {
        tfl |= 0x4000000 as libc::c_int
    }
    return tfl;
}
//end of the function AAS_GetAreaContentsTravelFlags
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[inline]

unsafe extern "C" fn AAS_AreaContentsTravelFlags_inline(mut areanum: libc::c_int) -> libc::c_int {
    return *crate::src::botlib::be_aas_main::aasworld
        .areacontentstravelflags
        .offset(areanum as isize);
}
//end of the function AAS_AreaContentsTravelFlags
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_AreaContentsTravelFlags(mut areanum: libc::c_int) -> libc::c_int {
    return *crate::src::botlib::be_aas_main::aasworld
        .areacontentstravelflags
        .offset(areanum as isize);
}
//end of the function AAS_AreaContentsTravelFlags
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_InitAreaContentsTravelFlags() {
    let mut i: libc::c_int = 0;
    if !crate::src::botlib::be_aas_main::aasworld
        .areacontentstravelflags
        .is_null()
    {
        crate::src::botlib::l_memory::FreeMemory(
            crate::src::botlib::be_aas_main::aasworld.areacontentstravelflags as *mut libc::c_void,
        );
    }
    crate::src::botlib::be_aas_main::aasworld.areacontentstravelflags =
        crate::src::botlib::l_memory::GetClearedMemory(
            (crate::src::botlib::be_aas_main::aasworld.numareas as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) as *mut libc::c_int;
    //
    i = 0 as libc::c_int;
    while i < crate::src::botlib::be_aas_main::aasworld.numareas {
        *crate::src::botlib::be_aas_main::aasworld
            .areacontentstravelflags
            .offset(i as isize) = AAS_GetAreaContentsTravelFlags(i);
        i += 1
    }
}
//end of the function AAS_InitAreaContentsTravelFlags
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_CreateReversedReachability() {
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut revlink: *mut crate::be_aas_def_h::aas_reversedlink_t =
        0 as *mut crate::be_aas_def_h::aas_reversedlink_t;
    let mut reach: *mut crate::aasfile_h::aas_reachability_t =
        0 as *mut crate::aasfile_h::aas_reachability_t;
    let mut settings: *mut crate::aasfile_h::aas_areasettings_t =
        0 as *mut crate::aasfile_h::aas_areasettings_t;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    //free reversed links that have already been created
    if !crate::src::botlib::be_aas_main::aasworld
        .reversedreachability
        .is_null()
    {
        crate::src::botlib::l_memory::FreeMemory(
            crate::src::botlib::be_aas_main::aasworld.reversedreachability as *mut libc::c_void,
        );
    }
    //allocate memory for the reversed reachability links
    ptr = crate::src::botlib::l_memory::GetClearedMemory(
        (crate::src::botlib::be_aas_main::aasworld.numareas as libc::c_ulong)
            .wrapping_mul(
                ::std::mem::size_of::<crate::be_aas_def_h::aas_reversedreachability_t>()
                    as libc::c_ulong,
            )
            .wrapping_add(
                (crate::src::botlib::be_aas_main::aasworld.reachabilitysize as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<crate::be_aas_def_h::aas_reversedlink_t>()
                            as libc::c_ulong,
                    ),
            ),
    ) as *mut libc::c_char;
    //
    crate::src::botlib::be_aas_main::aasworld.reversedreachability =
        ptr as *mut crate::be_aas_def_h::aas_reversedreachability_t;
    //pointer to the memory for the reversed links
    ptr = ptr.offset(
        (crate::src::botlib::be_aas_main::aasworld.numareas as libc::c_ulong).wrapping_mul(
            ::std::mem::size_of::<crate::be_aas_def_h::aas_reversedreachability_t>()
                as libc::c_ulong,
        ) as isize,
    );
    //check all reachabilities of all areas
    i = 1 as libc::c_int;
    while i < crate::src::botlib::be_aas_main::aasworld.numareas {
        //settings of the area
        settings = &mut *crate::src::botlib::be_aas_main::aasworld
            .areasettings
            .offset(i as isize) as *mut crate::aasfile_h::aas_areasettings_t;
        //end for
        if (*settings).numreachableareas >= 128 as libc::c_int {
            crate::src::botlib::be_interface::botimport
                .Print
                .expect("non-null function pointer")(
                2 as libc::c_int,
                b"area %d has more than 128 reachabilities\n\x00" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                i,
            );
        }
        n = 0 as libc::c_int;
        while n < (*settings).numreachableareas && n < 128 as libc::c_int {
            //
            //create reversed links for the reachabilities
            //reachability link
            reach = &mut *crate::src::botlib::be_aas_main::aasworld
                .reachability
                .offset(((*settings).firstreachablearea + n) as isize)
                as *mut crate::aasfile_h::aas_reachability_t;
            //
            revlink = ptr as *mut crate::be_aas_def_h::aas_reversedlink_t;
            ptr = ptr.offset(
                ::std::mem::size_of::<crate::be_aas_def_h::aas_reversedlink_t>() as libc::c_ulong
                    as isize,
            );
            //
            (*revlink).areanum = i;
            (*revlink).linknum = (*settings).firstreachablearea + n;
            (*revlink).next = (*crate::src::botlib::be_aas_main::aasworld
                .reversedreachability
                .offset((*reach).areanum as isize))
            .first;
            let ref mut fresh2 = (*crate::src::botlib::be_aas_main::aasworld
                .reversedreachability
                .offset((*reach).areanum as isize))
            .first;
            *fresh2 = revlink;
            let ref mut fresh3 = (*crate::src::botlib::be_aas_main::aasworld
                .reversedreachability
                .offset((*reach).areanum as isize))
            .numlinks;
            *fresh3 += 1;
            n += 1
        }
        i += 1
    }
    //end for
}
//end of the function AAS_CreateReversedReachability
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_AreaTravelTime(
    mut areanum: libc::c_int,
    mut start: *mut crate::src::qcommon::q_shared::vec_t,
    mut end: *mut crate::src::qcommon::q_shared::vec_t,
) -> libc::c_ushort {
    let mut intdist: libc::c_int = 0;
    let mut dist: libc::c_float = 0.;
    let mut dir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    dir[0 as libc::c_int as usize] =
        *start.offset(0 as libc::c_int as isize) - *end.offset(0 as libc::c_int as isize);
    dir[1 as libc::c_int as usize] =
        *start.offset(1 as libc::c_int as isize) - *end.offset(1 as libc::c_int as isize);
    dir[2 as libc::c_int as usize] =
        *start.offset(2 as libc::c_int as isize) - *end.offset(2 as libc::c_int as isize);
    dist = VectorLength(dir.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t);
    //if crouch only area
    if crate::src::botlib::be_aas_reach::AAS_AreaCrouch(areanum) != 0 {
        dist *= 1.3f32
    } else if crate::src::botlib::be_aas_reach::AAS_AreaSwim(areanum) != 0 {
        dist *= 1 as libc::c_int as libc::c_float
    } else {
        //if swim area
        //normal walk area
        dist *= 0.33f32
    }
    //
    intdist = dist as libc::c_int;
    //make sure the distance isn't zero
    if intdist <= 0 as libc::c_int {
        intdist = 1 as libc::c_int
    }
    return intdist as libc::c_ushort;
}
//end of the function AAS_AreaTravelTime
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_CalculateAreaTravelTimes() {
    let mut i: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut end: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut revreach: *mut crate::be_aas_def_h::aas_reversedreachability_t =
        0 as *mut crate::be_aas_def_h::aas_reversedreachability_t;
    let mut revlink: *mut crate::be_aas_def_h::aas_reversedlink_t =
        0 as *mut crate::be_aas_def_h::aas_reversedlink_t;
    let mut reach: *mut crate::aasfile_h::aas_reachability_t =
        0 as *mut crate::aasfile_h::aas_reachability_t;
    let mut settings: *mut crate::aasfile_h::aas_areasettings_t =
        0 as *mut crate::aasfile_h::aas_areasettings_t;
    //if there are still area travel times, free the memory
    if !crate::src::botlib::be_aas_main::aasworld
        .areatraveltimes
        .is_null()
    {
        crate::src::botlib::l_memory::FreeMemory(
            crate::src::botlib::be_aas_main::aasworld.areatraveltimes as *mut libc::c_void,
        );
    }
    //get the total size of all the area travel times
    size = (crate::src::botlib::be_aas_main::aasworld.numareas as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<*mut *mut libc::c_ushort>() as libc::c_ulong)
        as libc::c_int; //end for
    i = 0 as libc::c_int;
    while i < crate::src::botlib::be_aas_main::aasworld.numareas {
        revreach = &mut *crate::src::botlib::be_aas_main::aasworld
            .reversedreachability
            .offset(i as isize)
            as *mut crate::be_aas_def_h::aas_reversedreachability_t;
        //settings of the area
        settings = &mut *crate::src::botlib::be_aas_main::aasworld
            .areasettings
            .offset(i as isize) as *mut crate::aasfile_h::aas_areasettings_t;
        //
        size = (size as libc::c_ulong).wrapping_add(
            ((*settings).numreachableareas as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<*mut libc::c_ushort>() as libc::c_ulong),
        ) as libc::c_int as libc::c_int;
        //
        size = (size as libc::c_ulong).wrapping_add(
            ((*settings).numreachableareas as libc::c_ulong)
                .wrapping_mul(
                    ((*revreach).numlinks as libc::c_ulong)
                        .wrapping_add(::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        & !(::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                )
                .wrapping_mul(::std::mem::size_of::<libc::c_ushort>() as libc::c_ulong),
        ) as libc::c_int as libc::c_int;
        i += 1
    }
    //allocate memory for the area travel times
    ptr =
        crate::src::botlib::l_memory::GetClearedMemory(size as libc::c_ulong) as *mut libc::c_char;
    crate::src::botlib::be_aas_main::aasworld.areatraveltimes =
        ptr as *mut *mut *mut libc::c_ushort;
    ptr = ptr.offset(
        (crate::src::botlib::be_aas_main::aasworld.numareas as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut *mut libc::c_ushort>() as libc::c_ulong)
            as isize,
    );
    //calcluate the travel times for all the areas
    i = 0 as libc::c_int;
    while i < crate::src::botlib::be_aas_main::aasworld.numareas {
        //reversed reachabilities of this area
        revreach = &mut *crate::src::botlib::be_aas_main::aasworld
            .reversedreachability
            .offset(i as isize)
            as *mut crate::be_aas_def_h::aas_reversedreachability_t;
        //end for
        settings = &mut *crate::src::botlib::be_aas_main::aasworld
            .areasettings
            .offset(i as isize) as *mut crate::aasfile_h::aas_areasettings_t;
        let ref mut fresh4 = *crate::src::botlib::be_aas_main::aasworld
            .areatraveltimes
            .offset(i as isize);
        *fresh4 = ptr as *mut *mut libc::c_ushort;
        ptr = ptr.offset(
            ((*settings).numreachableareas as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<*mut libc::c_ushort>() as libc::c_ulong)
                as isize,
        );
        l = 0 as libc::c_int;
        while l < (*settings).numreachableareas {
            let ref mut fresh5 = *(*crate::src::botlib::be_aas_main::aasworld
                .areatraveltimes
                .offset(i as isize))
            .offset(l as isize);
            *fresh5 = ptr as *mut libc::c_ushort;
            ptr = ptr.offset(
                (((*revreach).numlinks as libc::c_ulong)
                    .wrapping_add(::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    & !(::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong))
                .wrapping_mul(::std::mem::size_of::<libc::c_ushort>() as libc::c_ulong)
                    as isize,
            );
            //settings of the area
            //
            //
            //end for
            reach = &mut *crate::src::botlib::be_aas_main::aasworld
                .reachability
                .offset(((*settings).firstreachablearea + l) as isize)
                as *mut crate::aasfile_h::aas_reachability_t;
            n = 0 as libc::c_int;
            revlink = (*revreach).first;
            while !revlink.is_null() {
                end[0 as libc::c_int as usize] = (*crate::src::botlib::be_aas_main::aasworld
                    .reachability
                    .offset((*revlink).linknum as isize))
                .end[0 as libc::c_int as usize];
                end[1 as libc::c_int as usize] = (*crate::src::botlib::be_aas_main::aasworld
                    .reachability
                    .offset((*revlink).linknum as isize))
                .end[1 as libc::c_int as usize];
                end[2 as libc::c_int as usize] = (*crate::src::botlib::be_aas_main::aasworld
                    .reachability
                    .offset((*revlink).linknum as isize))
                .end[2 as libc::c_int as usize];
                //reachability link
                //
                //
                *(*(*crate::src::botlib::be_aas_main::aasworld
                    .areatraveltimes
                    .offset(i as isize))
                .offset(l as isize))
                .offset(n as isize) =
                    AAS_AreaTravelTime(i, end.as_mut_ptr(), (*reach).start.as_mut_ptr());
                revlink = (*revlink).next;
                n += 1
            }
            l += 1
        }
        i += 1
    }
    //end for
}
//end of the function AAS_CalculateAreaTravelTimes
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_PortalMaxTravelTime(mut portalnum: libc::c_int) -> libc::c_int {
    let mut l: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut maxt: libc::c_int = 0;
    let mut portal: *mut crate::aasfile_h::aas_portal_t = 0 as *mut crate::aasfile_h::aas_portal_t;
    let mut revreach: *mut crate::be_aas_def_h::aas_reversedreachability_t =
        0 as *mut crate::be_aas_def_h::aas_reversedreachability_t;
    let mut revlink: *mut crate::be_aas_def_h::aas_reversedlink_t =
        0 as *mut crate::be_aas_def_h::aas_reversedlink_t;
    let mut settings: *mut crate::aasfile_h::aas_areasettings_t =
        0 as *mut crate::aasfile_h::aas_areasettings_t;
    portal = &mut *crate::src::botlib::be_aas_main::aasworld
        .portals
        .offset(portalnum as isize) as *mut crate::aasfile_h::aas_portal_t;
    //reversed reachabilities of this portal area
    revreach = &mut *crate::src::botlib::be_aas_main::aasworld
        .reversedreachability
        .offset((*portal).areanum as isize)
        as *mut crate::be_aas_def_h::aas_reversedreachability_t;
    //settings of the portal area
    settings = &mut *crate::src::botlib::be_aas_main::aasworld
        .areasettings
        .offset((*portal).areanum as isize)
        as *mut crate::aasfile_h::aas_areasettings_t;
    //
    maxt = 0 as libc::c_int; //end for
    l = 0 as libc::c_int;
    while l < (*settings).numreachableareas {
        n = 0 as libc::c_int;
        revlink = (*revreach).first;
        while !revlink.is_null() {
            t = *(*(*crate::src::botlib::be_aas_main::aasworld
                .areatraveltimes
                .offset((*portal).areanum as isize))
            .offset(l as isize))
            .offset(n as isize) as libc::c_int;
            if t > maxt {
                maxt = t
            }
            revlink = (*revlink).next;
            n += 1
            //end if
        }
        l += 1
        //end for
    }
    return maxt;
}
//end of the function AAS_PortalMaxTravelTime
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_InitPortalMaxTravelTimes() {
    let mut i: libc::c_int = 0;
    if !crate::src::botlib::be_aas_main::aasworld
        .portalmaxtraveltimes
        .is_null()
    {
        crate::src::botlib::l_memory::FreeMemory(
            crate::src::botlib::be_aas_main::aasworld.portalmaxtraveltimes as *mut libc::c_void,
        );
    }
    crate::src::botlib::be_aas_main::aasworld.portalmaxtraveltimes =
        crate::src::botlib::l_memory::GetClearedMemory(
            (crate::src::botlib::be_aas_main::aasworld.numportals as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) as *mut libc::c_int;
    i = 0 as libc::c_int;
    while i < crate::src::botlib::be_aas_main::aasworld.numportals {
        *crate::src::botlib::be_aas_main::aasworld
            .portalmaxtraveltimes
            .offset(i as isize) = AAS_PortalMaxTravelTime(i);
        i += 1
        //botimport.Print(PRT_MESSAGE, "portal %d max tt = %d\n", i, aasworld.portalmaxtraveltimes[i]);
    }
    //end for
}
//end of the function AAS_InitPortalMaxTravelTimes
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
/*
int AAS_FreeOldestCache(void)
{
    int i, j, bestcluster, bestarea, freed;
    float besttime;
    aas_routingcache_t *cache, *bestcache;

    freed = qfalse;
    besttime = 999999999;
    bestcache = NULL;
    bestcluster = 0;
    bestarea = 0;
    //refresh cluster cache
    for (i = 0; i < aasworld.numclusters; i++)
    {
        for (j = 0; j < aasworld.clusters[i].numareas; j++)
        {
            for (cache = aasworld.clusterareacache[i][j]; cache; cache = cache->next)
            {
                //never remove cache leading towards a portal
                if (aasworld.areasettings[cache->areanum].cluster < 0) continue;
                //if this cache is older than the cache we found so far
                if (cache->time < besttime)
                {
                    bestcache = cache;
                    bestcluster = i;
                    bestarea = j;
                    besttime = cache->time;
                } //end if
            } //end for
        } //end for
    } //end for
    if (bestcache)
    {
        cache = bestcache;
        if (cache->prev) cache->prev->next = cache->next;
        else aasworld.clusterareacache[bestcluster][bestarea] = cache->next;
        if (cache->next) cache->next->prev = cache->prev;
        AAS_FreeRoutingCache(cache);
        freed = qtrue;
    } //end if
    besttime = 999999999;
    bestcache = NULL;
    bestarea = 0;
    for (i = 0; i < aasworld.numareas; i++)
    {
        //refresh portal cache
        for (cache = aasworld.portalcache[i]; cache; cache = cache->next)
        {
            if (cache->time < besttime)
            {
                bestcache = cache;
                bestarea = i;
                besttime = cache->time;
            } //end if
        } //end for
    } //end for
    if (bestcache)
    {
        cache = bestcache;
        if (cache->prev) cache->prev->next = cache->next;
        else aasworld.portalcache[bestarea] = cache->next;
        if (cache->next) cache->next->prev = cache->prev;
        AAS_FreeRoutingCache(cache);
        freed = qtrue;
    } //end if
    return freed;
} //end of the function AAS_FreeOldestCache
*/
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_FreeOldestCache() -> libc::c_int {
    let mut clusterareanum: libc::c_int = 0;
    let mut cache: *mut crate::be_aas_def_h::aas_routingcache_t =
        0 as *mut crate::be_aas_def_h::aas_routingcache_t;
    cache = crate::src::botlib::be_aas_main::aasworld.oldestcache;
    while !cache.is_null() {
        // never free area cache leading towards a portal
        if !((*cache).type_0 as libc::c_int == 1 as libc::c_int
            && (*crate::src::botlib::be_aas_main::aasworld
                .areasettings
                .offset((*cache).areanum as isize))
            .cluster
                < 0 as libc::c_int)
        {
            break;
        }
        cache = (*cache).time_next
    }
    if !cache.is_null() {
        // unlink the cache
        if (*cache).type_0 as libc::c_int == 1 as libc::c_int {
            //number of the area in the cluster
            clusterareanum = AAS_ClusterAreaNum((*cache).cluster, (*cache).areanum);
            // unlink from cluster area cache
            if !(*cache).prev.is_null() {
                (*(*cache).prev).next = (*cache).next
            } else {
                let ref mut fresh6 = *(*crate::src::botlib::be_aas_main::aasworld
                    .clusterareacache
                    .offset((*cache).cluster as isize))
                .offset(clusterareanum as isize);
                *fresh6 = (*cache).next
            }
            if !(*cache).next.is_null() {
                (*(*cache).next).prev = (*cache).prev
            }
        } else {
            // unlink from portal cache
            if !(*cache).prev.is_null() {
                (*(*cache).prev).next = (*cache).next
            } else {
                let ref mut fresh7 = *crate::src::botlib::be_aas_main::aasworld
                    .portalcache
                    .offset((*cache).areanum as isize);
                *fresh7 = (*cache).next
            }
            if !(*cache).next.is_null() {
                (*(*cache).next).prev = (*cache).prev
            }
        }
        AAS_FreeRoutingCache(cache);
        return crate::src::qcommon::q_shared::qtrue as libc::c_int;
    }
    return crate::src::qcommon::q_shared::qfalse as libc::c_int;
}
//end of the function AAS_FreeOldestCache
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_AllocRoutingCache(
    mut numtraveltimes: libc::c_int,
) -> *mut crate::be_aas_def_h::aas_routingcache_t {
    let mut cache: *mut crate::be_aas_def_h::aas_routingcache_t =
        0 as *mut crate::be_aas_def_h::aas_routingcache_t;
    let mut size: libc::c_int = 0;
    //
    size = (::std::mem::size_of::<crate::be_aas_def_h::aas_routingcache_t>() as libc::c_ulong)
        .wrapping_add(
            (numtraveltimes as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_ushort>() as libc::c_ulong),
        )
        .wrapping_add(
            (numtraveltimes as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong),
        ) as libc::c_int;
    //
    routingcachesize += size;
    //
    cache = crate::src::botlib::l_memory::GetClearedMemory(size as libc::c_ulong)
        as *mut crate::be_aas_def_h::aas_routingcache_t;
    (*cache).reachabilities = (cache as *mut libc::c_uchar)
        .offset(
            ::std::mem::size_of::<crate::be_aas_def_h::aas_routingcache_t>() as libc::c_ulong
                as isize,
        )
        .offset(
            (numtraveltimes as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_ushort>() as libc::c_ulong)
                as isize,
        );
    (*cache).size = size;
    return cache;
}
//end of the function AAS_AllocRoutingCache
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_FreeAllClusterAreaCache() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut cache: *mut crate::be_aas_def_h::aas_routingcache_t =
        0 as *mut crate::be_aas_def_h::aas_routingcache_t;
    let mut nextcache: *mut crate::be_aas_def_h::aas_routingcache_t =
        0 as *mut crate::be_aas_def_h::aas_routingcache_t;
    let mut cluster: *mut crate::aasfile_h::aas_cluster_t =
        0 as *mut crate::aasfile_h::aas_cluster_t;
    //free all cluster cache if existing
    if crate::src::botlib::be_aas_main::aasworld
        .clusterareacache
        .is_null()
    {
        return;
    }
    //free caches
    i = 0 as libc::c_int; //end for
    while i < crate::src::botlib::be_aas_main::aasworld.numclusters {
        cluster = &mut *crate::src::botlib::be_aas_main::aasworld
            .clusters
            .offset(i as isize) as *mut crate::aasfile_h::aas_cluster_t;
        j = 0 as libc::c_int;
        while j < (*cluster).numareas {
            //end for
            cache = *(*crate::src::botlib::be_aas_main::aasworld
                .clusterareacache
                .offset(i as isize))
            .offset(j as isize); //end for
            while !cache.is_null() {
                nextcache = (*cache).next;
                AAS_FreeRoutingCache(cache);
                cache = nextcache
            }
            let ref mut fresh8 = *(*crate::src::botlib::be_aas_main::aasworld
                .clusterareacache
                .offset(i as isize))
            .offset(j as isize);
            *fresh8 = 0 as *mut crate::be_aas_def_h::aas_routingcache_t;
            j += 1
        }
        i += 1
    }
    //free the cluster cache array
    crate::src::botlib::l_memory::FreeMemory(
        crate::src::botlib::be_aas_main::aasworld.clusterareacache as *mut libc::c_void,
    );
    crate::src::botlib::be_aas_main::aasworld.clusterareacache =
        0 as *mut *mut *mut crate::be_aas_def_h::aas_routingcache_t;
}
//end of the function AAS_FreeAllClusterAreaCache
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_InitClusterAreaCache() {
    let mut i: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    //
    size = 0 as libc::c_int; //end for
    i = 0 as libc::c_int;
    while i < crate::src::botlib::be_aas_main::aasworld.numclusters {
        size += (*crate::src::botlib::be_aas_main::aasworld
            .clusters
            .offset(i as isize))
        .numareas;
        i += 1
    }
    //two dimensional array with pointers for every cluster to routing cache
    //for every area in that cluster
    ptr = crate::src::botlib::l_memory::GetClearedMemory(
        (crate::src::botlib::be_aas_main::aasworld.numclusters as libc::c_ulong)
            .wrapping_mul(
                ::std::mem::size_of::<*mut *mut crate::be_aas_def_h::aas_routingcache_t>()
                    as libc::c_ulong,
            )
            .wrapping_add((size as libc::c_ulong).wrapping_mul(::std::mem::size_of::<
                *mut crate::be_aas_def_h::aas_routingcache_t,
            >() as libc::c_ulong)),
    ) as *mut libc::c_char;
    crate::src::botlib::be_aas_main::aasworld.clusterareacache =
        ptr as *mut *mut *mut crate::be_aas_def_h::aas_routingcache_t;
    ptr = ptr.offset(
        (crate::src::botlib::be_aas_main::aasworld.numclusters as libc::c_ulong).wrapping_mul(
            ::std::mem::size_of::<*mut *mut crate::be_aas_def_h::aas_routingcache_t>()
                as libc::c_ulong,
        ) as isize,
    );
    i = 0 as libc::c_int;
    while i < crate::src::botlib::be_aas_main::aasworld.numclusters {
        let ref mut fresh9 = *crate::src::botlib::be_aas_main::aasworld
            .clusterareacache
            .offset(i as isize);
        *fresh9 = ptr as *mut *mut crate::be_aas_def_h::aas_routingcache_t;
        ptr = ptr.offset(
            ((*crate::src::botlib::be_aas_main::aasworld
                .clusters
                .offset(i as isize))
            .numareas as libc::c_ulong)
                .wrapping_mul(
                    ::std::mem::size_of::<*mut crate::be_aas_def_h::aas_routingcache_t>()
                        as libc::c_ulong,
                ) as isize,
        );
        i += 1
    }
    //end for
}
//end of the function AAS_InitClusterAreaCache
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_FreeAllPortalCache() {
    let mut i: libc::c_int = 0;
    let mut cache: *mut crate::be_aas_def_h::aas_routingcache_t =
        0 as *mut crate::be_aas_def_h::aas_routingcache_t;
    let mut nextcache: *mut crate::be_aas_def_h::aas_routingcache_t =
        0 as *mut crate::be_aas_def_h::aas_routingcache_t;
    //free all portal cache if existing
    if crate::src::botlib::be_aas_main::aasworld
        .portalcache
        .is_null()
    {
        return;
    }
    //free portal caches
    i = 0 as libc::c_int; //end for
    while i < crate::src::botlib::be_aas_main::aasworld.numareas {
        cache = *crate::src::botlib::be_aas_main::aasworld
            .portalcache
            .offset(i as isize); //end for
        while !cache.is_null() {
            nextcache = (*cache).next;
            AAS_FreeRoutingCache(cache);
            cache = nextcache
        }
        let ref mut fresh10 = *crate::src::botlib::be_aas_main::aasworld
            .portalcache
            .offset(i as isize);
        *fresh10 = 0 as *mut crate::be_aas_def_h::aas_routingcache_t;
        i += 1
    }
    crate::src::botlib::l_memory::FreeMemory(
        crate::src::botlib::be_aas_main::aasworld.portalcache as *mut libc::c_void,
    );
    crate::src::botlib::be_aas_main::aasworld.portalcache =
        0 as *mut *mut crate::be_aas_def_h::aas_routingcache_t;
}
//end of the function AAS_FreeAllPortalCache
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_InitPortalCache() {
    //
    crate::src::botlib::be_aas_main::aasworld.portalcache =
        crate::src::botlib::l_memory::GetClearedMemory(
            (crate::src::botlib::be_aas_main::aasworld.numareas as libc::c_ulong).wrapping_mul(
                ::std::mem::size_of::<*mut crate::be_aas_def_h::aas_routingcache_t>()
                    as libc::c_ulong,
            ),
        ) as *mut *mut crate::be_aas_def_h::aas_routingcache_t;
}
//end of the function AAS_InitPortalCache
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_InitRoutingUpdate() {
    let mut i: libc::c_int = 0;
    let mut maxreachabilityareas: libc::c_int = 0;
    //free routing update fields if already existing
    if !crate::src::botlib::be_aas_main::aasworld
        .areaupdate
        .is_null()
    {
        crate::src::botlib::l_memory::FreeMemory(
            crate::src::botlib::be_aas_main::aasworld.areaupdate as *mut libc::c_void,
        );
    }
    //
    maxreachabilityareas = 0 as libc::c_int; //end for
    i = 0 as libc::c_int;
    while i < crate::src::botlib::be_aas_main::aasworld.numclusters {
        if (*crate::src::botlib::be_aas_main::aasworld
            .clusters
            .offset(i as isize))
        .numreachabilityareas
            > maxreachabilityareas
        {
            maxreachabilityareas = (*crate::src::botlib::be_aas_main::aasworld
                .clusters
                .offset(i as isize))
            .numreachabilityareas
        }
        i += 1
        //end if
    }
    //allocate memory for the routing update fields
    crate::src::botlib::be_aas_main::aasworld.areaupdate =
        crate::src::botlib::l_memory::GetClearedMemory(
            (maxreachabilityareas as libc::c_ulong).wrapping_mul(::std::mem::size_of::<
                crate::be_aas_def_h::aas_routingupdate_t,
            >() as libc::c_ulong),
        ) as *mut crate::be_aas_def_h::aas_routingupdate_t;
    //
    if !crate::src::botlib::be_aas_main::aasworld
        .portalupdate
        .is_null()
    {
        crate::src::botlib::l_memory::FreeMemory(
            crate::src::botlib::be_aas_main::aasworld.portalupdate as *mut libc::c_void,
        );
    }
    //allocate memory for the portal update fields
    crate::src::botlib::be_aas_main::aasworld.portalupdate =
        crate::src::botlib::l_memory::GetClearedMemory(
            ((crate::src::botlib::be_aas_main::aasworld.numportals + 1 as libc::c_int)
                as libc::c_ulong)
                .wrapping_mul(
                    ::std::mem::size_of::<crate::be_aas_def_h::aas_routingupdate_t>()
                        as libc::c_ulong,
                ),
        ) as *mut crate::be_aas_def_h::aas_routingupdate_t;
}
//end of the function AAS_InitRoutingUpdate
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_CreateAllRoutingCache() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    //int t;
    crate::src::botlib::be_aas_main::aasworld.initialized =
        crate::src::qcommon::q_shared::qtrue as libc::c_int; //end for
    crate::src::botlib::be_interface::botimport
        .Print
        .expect("non-null function pointer")(
        1 as libc::c_int,
        b"AAS_CreateAllRoutingCache\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    i = 1 as libc::c_int;
    while i < crate::src::botlib::be_aas_main::aasworld.numareas {
        if !(crate::src::botlib::be_aas_reach::AAS_AreaReachability(i) == 0) {
            j = 1 as libc::c_int;
            while j < crate::src::botlib::be_aas_main::aasworld.numareas {
                if !(i == j) {
                    if !(crate::src::botlib::be_aas_reach::AAS_AreaReachability(j) == 0) {
                        AAS_AreaTravelTimeToGoalArea(
                            i,
                            (*crate::src::botlib::be_aas_main::aasworld
                                .areas
                                .offset(i as isize))
                            .center
                            .as_mut_ptr(),
                            j,
                            0x2 as libc::c_int
                                | 0x4 as libc::c_int
                                | 0x8 as libc::c_int
                                | 0x10 as libc::c_int
                                | 0x20 as libc::c_int
                                | 0x80 as libc::c_int
                                | 0x100 as libc::c_int
                                | 0x200 as libc::c_int
                                | 0x400 as libc::c_int
                                | 0x800 as libc::c_int
                                | 0x80000 as libc::c_int
                                | 0x100000 as libc::c_int
                                | 0x40000 as libc::c_int
                                | 0x1000000 as libc::c_int,
                        );
                    }
                }
                j += 1
                //t = AAS_AreaTravelTimeToGoalArea(i, aasworld.areas[i].center, j, TFL_DEFAULT);
                //Log_Write("traveltime from %d to %d is %d", i, j, t);
            }
        }
        i += 1
        //end for
    }
    crate::src::botlib::be_aas_main::aasworld.initialized =
        crate::src::qcommon::q_shared::qfalse as libc::c_int;
}
//void AAS_DecompressVis(byte *in, int numareas, byte *decompressed);
//int AAS_CompressVis(byte *vis, int numareas, byte *dest);
#[no_mangle]

pub unsafe extern "C" fn AAS_WriteRouteCache() {
    let mut i: libc::c_int = 0; //end for
    let mut j: libc::c_int = 0;
    let mut numportalcache: libc::c_int = 0;
    let mut numareacache: libc::c_int = 0;
    let mut totalsize: libc::c_int = 0;
    let mut cache: *mut crate::be_aas_def_h::aas_routingcache_t =
        0 as *mut crate::be_aas_def_h::aas_routingcache_t;
    let mut cluster: *mut crate::aasfile_h::aas_cluster_t =
        0 as *mut crate::aasfile_h::aas_cluster_t;
    let mut fp: crate::src::qcommon::q_shared::fileHandle_t = 0;
    let mut filename: [libc::c_char; 64] = [0; 64];
    let mut routecacheheader: routecacheheader_t = routecacheheader_t {
        ident: 0,
        version: 0,
        numareas: 0,
        numclusters: 0,
        areacrc: 0,
        clustercrc: 0,
        numportalcache: 0,
        numareacache: 0,
    };
    numportalcache = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < crate::src::botlib::be_aas_main::aasworld.numareas {
        cache = *crate::src::botlib::be_aas_main::aasworld
            .portalcache
            .offset(i as isize);
        while !cache.is_null() {
            numportalcache += 1;
            cache = (*cache).next
        }
        i += 1
        //end for
    } //end for
    numareacache = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < crate::src::botlib::be_aas_main::aasworld.numclusters {
        cluster = &mut *crate::src::botlib::be_aas_main::aasworld
            .clusters
            .offset(i as isize) as *mut crate::aasfile_h::aas_cluster_t;
        j = 0 as libc::c_int;
        while j < (*cluster).numareas {
            cache = *(*crate::src::botlib::be_aas_main::aasworld
                .clusterareacache
                .offset(i as isize))
            .offset(j as isize);
            while !cache.is_null() {
                numareacache += 1;
                cache = (*cache).next
            }
            j += 1
            //end for
            //end for
        }
        i += 1
    }
    // open the file for writing
    crate::src::qcommon::q_shared::Com_sprintf(
        filename.as_mut_ptr(),
        64 as libc::c_int,
        b"maps/%s.rcd\x00" as *const u8 as *const libc::c_char,
        crate::src::botlib::be_aas_main::aasworld
            .mapname
            .as_mut_ptr(),
    ); //end if
    crate::src::botlib::be_interface::botimport
        .FS_FOpenFile
        .expect("non-null function pointer")(
        filename.as_mut_ptr(),
        &mut fp,
        crate::src::qcommon::q_shared::FS_WRITE,
    );
    if fp == 0 {
        crate::src::botlib::be_aas_main::AAS_Error(
            b"Unable to open file: %s\n\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            filename.as_mut_ptr(),
        );
        return;
    }
    //create the header
    routecacheheader.ident = (('C' as i32) << 24 as libc::c_int)
        + (('R' as i32) << 16 as libc::c_int)
        + (('E' as i32) << 8 as libc::c_int)
        + 'M' as i32;
    routecacheheader.version = 2 as libc::c_int;
    routecacheheader.numareas = crate::src::botlib::be_aas_main::aasworld.numareas;
    routecacheheader.numclusters = crate::src::botlib::be_aas_main::aasworld.numclusters;
    routecacheheader.areacrc = crate::src::botlib::l_crc::CRC_ProcessString(
        crate::src::botlib::be_aas_main::aasworld.areas as *mut libc::c_uchar,
        (::std::mem::size_of::<crate::aasfile_h::aas_area_t>() as libc::c_ulong)
            .wrapping_mul(crate::src::botlib::be_aas_main::aasworld.numareas as libc::c_ulong)
            as libc::c_int,
    ) as libc::c_int;
    routecacheheader.clustercrc = crate::src::botlib::l_crc::CRC_ProcessString(
        crate::src::botlib::be_aas_main::aasworld.clusters as *mut libc::c_uchar,
        (::std::mem::size_of::<crate::aasfile_h::aas_cluster_t>() as libc::c_ulong)
            .wrapping_mul(crate::src::botlib::be_aas_main::aasworld.numclusters as libc::c_ulong)
            as libc::c_int,
    ) as libc::c_int;
    routecacheheader.numportalcache = numportalcache;
    routecacheheader.numareacache = numareacache;
    //write the header
    crate::src::botlib::be_interface::botimport
        .FS_Write
        .expect("non-null function pointer")(
        &mut routecacheheader as *mut routecacheheader_t as *const libc::c_void,
        ::std::mem::size_of::<routecacheheader_t>() as libc::c_ulong as libc::c_int,
        fp,
    );
    //
    totalsize = 0 as libc::c_int;
    //write all the cache
    i = 0 as libc::c_int; //end for
    while i < crate::src::botlib::be_aas_main::aasworld.numareas {
        cache = *crate::src::botlib::be_aas_main::aasworld
            .portalcache
            .offset(i as isize);
        while !cache.is_null() {
            crate::src::botlib::be_interface::botimport
                .FS_Write
                .expect("non-null function pointer")(
                cache as *const libc::c_void,
                (*cache).size,
                fp,
            );
            totalsize += (*cache).size;
            cache = (*cache).next
        }
        i += 1
        //end for
    } //end for
    i = 0 as libc::c_int;
    while i < crate::src::botlib::be_aas_main::aasworld.numclusters {
        cluster = &mut *crate::src::botlib::be_aas_main::aasworld
            .clusters
            .offset(i as isize) as *mut crate::aasfile_h::aas_cluster_t;
        j = 0 as libc::c_int;
        while j < (*cluster).numareas {
            cache = *(*crate::src::botlib::be_aas_main::aasworld
                .clusterareacache
                .offset(i as isize))
            .offset(j as isize);
            while !cache.is_null() {
                crate::src::botlib::be_interface::botimport
                    .FS_Write
                    .expect("non-null function pointer")(
                    cache as *const libc::c_void,
                    (*cache).size,
                    fp,
                );
                totalsize += (*cache).size;
                cache = (*cache).next
            }
            j += 1
            //end for
            //end for
        }
        i += 1
    }
    // write the visareas
    /*
    for (i = 0; i < aasworld.numareas; i++)
    {
        if (!aasworld.areavisibility[i]) {
            size = 0;
            botimport.FS_Write(&size, sizeof(int), fp);
            continue;
        }
        AAS_DecompressVis( aasworld.areavisibility[i], aasworld.numareas, aasworld.decompressedvis );
        size = AAS_CompressVis( aasworld.decompressedvis, aasworld.numareas, aasworld.decompressedvis );
        botimport.FS_Write(&size, sizeof(int), fp);
        botimport.FS_Write(aasworld.decompressedvis, size, fp);
    }
    */
    //
    crate::src::botlib::be_interface::botimport
        .FS_FCloseFile
        .expect("non-null function pointer")(fp);
    crate::src::botlib::be_interface::botimport
        .Print
        .expect("non-null function pointer")(
        1 as libc::c_int,
        b"\nroute cache written to %s\n\x00" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        filename.as_mut_ptr(),
    );
    crate::src::botlib::be_interface::botimport
        .Print
        .expect("non-null function pointer")(
        1 as libc::c_int,
        b"written %d bytes of routing cache\n\x00" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        totalsize,
    );
}
//end of the function AAS_WriteRouteCache
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_ReadCache(
    mut fp: crate::src::qcommon::q_shared::fileHandle_t,
) -> *mut crate::be_aas_def_h::aas_routingcache_t {
    let mut size: libc::c_int = 0;
    let mut cache: *mut crate::be_aas_def_h::aas_routingcache_t =
        0 as *mut crate::be_aas_def_h::aas_routingcache_t;
    crate::src::botlib::be_interface::botimport
        .FS_Read
        .expect("non-null function pointer")(
        &mut size as *mut libc::c_int as *mut libc::c_void,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
        fp,
    );
    cache = crate::src::botlib::l_memory::GetMemory(size as libc::c_ulong)
        as *mut crate::be_aas_def_h::aas_routingcache_t;
    (*cache).size = size;
    crate::src::botlib::be_interface::botimport
        .FS_Read
        .expect("non-null function pointer")(
        (cache as *mut libc::c_uchar)
            .offset(::std::mem::size_of::<libc::c_int>() as libc::c_ulong as isize)
            as *mut libc::c_void,
        (size as libc::c_ulong).wrapping_sub(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            as libc::c_int,
        fp,
    );
    (*cache).reachabilities = (cache as *mut libc::c_uchar)
        .offset(
            ::std::mem::size_of::<crate::be_aas_def_h::aas_routingcache_t>() as libc::c_ulong
                as isize,
        )
        .offset(-(::std::mem::size_of::<libc::c_ushort>() as libc::c_ulong as isize))
        .offset(
            (size as libc::c_ulong)
                .wrapping_sub(
                    ::std::mem::size_of::<crate::be_aas_def_h::aas_routingcache_t>()
                        as libc::c_ulong,
                )
                .wrapping_add(::std::mem::size_of::<libc::c_ushort>() as libc::c_ulong)
                .wrapping_div(3 as libc::c_int as libc::c_ulong)
                .wrapping_mul(2 as libc::c_int as libc::c_ulong) as isize,
        );
    return cache;
}
//end of the function AAS_ReadCache
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_ReadRouteCache() -> libc::c_int {
    let mut i: libc::c_int = 0; //, size;
    let mut clusterareanum: libc::c_int = 0; //end if
    let mut fp: crate::src::qcommon::q_shared::fileHandle_t = 0; //end if
    let mut filename: [libc::c_char; 64] = [0; 64]; //end if
    let mut routecacheheader: routecacheheader_t = routecacheheader_t {
        ident: 0,
        version: 0,
        numareas: 0,
        numclusters: 0,
        areacrc: 0,
        clustercrc: 0,
        numportalcache: 0,
        numareacache: 0,
    }; //end if
    let mut cache: *mut crate::be_aas_def_h::aas_routingcache_t =
        0 as *mut crate::be_aas_def_h::aas_routingcache_t;
    crate::src::qcommon::q_shared::Com_sprintf(
        filename.as_mut_ptr(),
        64 as libc::c_int,
        b"maps/%s.rcd\x00" as *const u8 as *const libc::c_char,
        crate::src::botlib::be_aas_main::aasworld
            .mapname
            .as_mut_ptr(),
    );
    crate::src::botlib::be_interface::botimport
        .FS_FOpenFile
        .expect("non-null function pointer")(
        filename.as_mut_ptr(),
        &mut fp,
        crate::src::qcommon::q_shared::FS_READ,
    );
    if fp == 0 {
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    }
    crate::src::botlib::be_interface::botimport
        .FS_Read
        .expect("non-null function pointer")(
        &mut routecacheheader as *mut routecacheheader_t as *mut libc::c_void,
        ::std::mem::size_of::<routecacheheader_t>() as libc::c_ulong as libc::c_int,
        fp,
    );
    if routecacheheader.ident
        != (('C' as i32) << 24 as libc::c_int)
            + (('R' as i32) << 16 as libc::c_int)
            + (('E' as i32) << 8 as libc::c_int)
            + 'M' as i32
    {
        crate::src::botlib::be_aas_main::AAS_Error(
            b"%s is not a route cache dump\n\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            filename.as_mut_ptr(),
        );
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    }
    if routecacheheader.version != 2 as libc::c_int {
        crate::src::botlib::be_aas_main::AAS_Error(
            b"route cache dump has wrong version %d, should be %d\n\x00" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            routecacheheader.version,
            2 as libc::c_int,
        );
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    }
    if routecacheheader.numareas != crate::src::botlib::be_aas_main::aasworld.numareas {
        //AAS_Error("route cache dump has wrong number of areas\n");
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    } //end if
    if routecacheheader.numclusters != crate::src::botlib::be_aas_main::aasworld.numclusters {
        //AAS_Error("route cache dump has wrong number of clusters\n");
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    } //end if
    if routecacheheader.areacrc
        != crate::src::botlib::l_crc::CRC_ProcessString(
            crate::src::botlib::be_aas_main::aasworld.areas as *mut libc::c_uchar,
            (::std::mem::size_of::<crate::aasfile_h::aas_area_t>() as libc::c_ulong)
                .wrapping_mul(crate::src::botlib::be_aas_main::aasworld.numareas as libc::c_ulong)
                as libc::c_int,
        ) as libc::c_int
    {
        //AAS_Error("route cache dump area CRC incorrect\n");
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    } //end if
    if routecacheheader.clustercrc
        != crate::src::botlib::l_crc::CRC_ProcessString(
            crate::src::botlib::be_aas_main::aasworld.clusters as *mut libc::c_uchar,
            (::std::mem::size_of::<crate::aasfile_h::aas_cluster_t>() as libc::c_ulong)
                .wrapping_mul(
                    crate::src::botlib::be_aas_main::aasworld.numclusters as libc::c_ulong,
                ) as libc::c_int,
        ) as libc::c_int
    {
        //AAS_Error("route cache dump cluster CRC incorrect\n");
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    }
    //read all the portal cache
    i = 0 as libc::c_int; //end for
    while i < routecacheheader.numportalcache {
        cache = AAS_ReadCache(fp);
        (*cache).next = *crate::src::botlib::be_aas_main::aasworld
            .portalcache
            .offset((*cache).areanum as isize);
        (*cache).prev = 0 as *mut crate::be_aas_def_h::aas_routingcache_s;
        if !(*crate::src::botlib::be_aas_main::aasworld
            .portalcache
            .offset((*cache).areanum as isize))
        .is_null()
        {
            let ref mut fresh11 = (**crate::src::botlib::be_aas_main::aasworld
                .portalcache
                .offset((*cache).areanum as isize))
            .prev;
            *fresh11 = cache
        }
        let ref mut fresh12 = *crate::src::botlib::be_aas_main::aasworld
            .portalcache
            .offset((*cache).areanum as isize);
        *fresh12 = cache;
        i += 1
    }
    //read all the cluster area cache
    i = 0 as libc::c_int; //end for
    while i < routecacheheader.numareacache {
        cache = AAS_ReadCache(fp);
        clusterareanum = AAS_ClusterAreaNum((*cache).cluster, (*cache).areanum);
        (*cache).next = *(*crate::src::botlib::be_aas_main::aasworld
            .clusterareacache
            .offset((*cache).cluster as isize))
        .offset(clusterareanum as isize);
        (*cache).prev = 0 as *mut crate::be_aas_def_h::aas_routingcache_s;
        if !(*(*crate::src::botlib::be_aas_main::aasworld
            .clusterareacache
            .offset((*cache).cluster as isize))
        .offset(clusterareanum as isize))
        .is_null()
        {
            let ref mut fresh13 = (**(*crate::src::botlib::be_aas_main::aasworld
                .clusterareacache
                .offset((*cache).cluster as isize))
            .offset(clusterareanum as isize))
            .prev;
            *fresh13 = cache
        }
        let ref mut fresh14 = *(*crate::src::botlib::be_aas_main::aasworld
            .clusterareacache
            .offset((*cache).cluster as isize))
        .offset(clusterareanum as isize);
        *fresh14 = cache;
        i += 1
    }
    // read the visareas
    /*
    aasworld.areavisibility = (byte **) GetClearedMemory(aasworld.numareas * sizeof(byte *));
    aasworld.decompressedvis = (byte *) GetClearedMemory(aasworld.numareas * sizeof(byte));
    for (i = 0; i < aasworld.numareas; i++)
    {
        botimport.FS_Read(&size, sizeof(size), fp );
        if (size) {
            aasworld.areavisibility[i] = (byte *) GetMemory(size);
            botimport.FS_Read(aasworld.areavisibility[i], size, fp );
        }
    }
    */
    //
    crate::src::botlib::be_interface::botimport
        .FS_FCloseFile
        .expect("non-null function pointer")(fp);
    return crate::src::qcommon::q_shared::qtrue as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn AAS_InitReachabilityAreas() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut numareas: libc::c_int = 0;
    let mut areas: [libc::c_int; 32] = [0; 32];
    let mut numreachareas: libc::c_int = 0;
    let mut reach: *mut crate::aasfile_h::aas_reachability_t =
        0 as *mut crate::aasfile_h::aas_reachability_t;
    let mut start: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut end: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    if !crate::src::botlib::be_aas_main::aasworld
        .reachabilityareas
        .is_null()
    {
        crate::src::botlib::l_memory::FreeMemory(
            crate::src::botlib::be_aas_main::aasworld.reachabilityareas as *mut libc::c_void,
        );
    }
    if !crate::src::botlib::be_aas_main::aasworld
        .reachabilityareaindex
        .is_null()
    {
        crate::src::botlib::l_memory::FreeMemory(
            crate::src::botlib::be_aas_main::aasworld.reachabilityareaindex as *mut libc::c_void,
        );
    }
    crate::src::botlib::be_aas_main::aasworld.reachabilityareas =
        crate::src::botlib::l_memory::GetClearedMemory(
            (crate::src::botlib::be_aas_main::aasworld.reachabilitysize as libc::c_ulong)
                .wrapping_mul(
                    ::std::mem::size_of::<crate::be_aas_def_h::aas_reachabilityareas_t>()
                        as libc::c_ulong,
                ),
        ) as *mut crate::be_aas_def_h::aas_reachabilityareas_t;
    crate::src::botlib::be_aas_main::aasworld.reachabilityareaindex =
        crate::src::botlib::l_memory::GetClearedMemory(
            ((crate::src::botlib::be_aas_main::aasworld.reachabilitysize * 32 as libc::c_int)
                as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) as *mut libc::c_int;
    numreachareas = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < crate::src::botlib::be_aas_main::aasworld.reachabilitysize {
        reach = &mut *crate::src::botlib::be_aas_main::aasworld
            .reachability
            .offset(i as isize) as *mut crate::aasfile_h::aas_reachability_t;
        numareas = 0 as libc::c_int;
        //end for
        match (*reach).traveltype & 0xffffff as libc::c_int {
            4 | 9 => {
                //end switch
                //trace areas from start to end
                end[0 as libc::c_int as usize] = (*reach).start[0 as libc::c_int as usize];
                end[1 as libc::c_int as usize] = (*reach).start[1 as libc::c_int as usize];
                end[2 as libc::c_int as usize] = (*reach).start[2 as libc::c_int as usize];
                end[2 as libc::c_int as usize] = (*reach).end[2 as libc::c_int as usize];
                numareas = crate::src::botlib::be_aas_sample::AAS_TraceAreas(
                    (*reach).start.as_mut_ptr(),
                    end.as_mut_ptr(),
                    areas.as_mut_ptr(),
                    0 as *mut crate::src::qcommon::q_shared::vec3_t,
                    32 as libc::c_int,
                )
            }
            7 => {
                start[0 as libc::c_int as usize] = (*reach).end[0 as libc::c_int as usize];
                start[1 as libc::c_int as usize] = (*reach).end[1 as libc::c_int as usize];
                start[2 as libc::c_int as usize] = (*reach).end[2 as libc::c_int as usize];
                start[2 as libc::c_int as usize] = (*reach).start[2 as libc::c_int as usize];
                numareas = crate::src::botlib::be_aas_sample::AAS_TraceAreas(
                    start.as_mut_ptr(),
                    (*reach).end.as_mut_ptr(),
                    areas.as_mut_ptr(),
                    0 as *mut crate::src::qcommon::q_shared::vec3_t,
                    32 as libc::c_int,
                )
            }
            14 => {
                numareas = crate::src::botlib::be_aas_sample::AAS_TraceAreas(
                    (*reach).start.as_mut_ptr(),
                    (*reach).end.as_mut_ptr(),
                    areas.as_mut_ptr(),
                    0 as *mut crate::src::qcommon::q_shared::vec3_t,
                    32 as libc::c_int,
                )
            }
            5 | 12 | 13 | 18 | 11 | 19 | 2 | 3 | 6 | 8 | 10 | _ => {}
        }
        (*crate::src::botlib::be_aas_main::aasworld
            .reachabilityareas
            .offset(i as isize))
        .firstarea = numreachareas;
        (*crate::src::botlib::be_aas_main::aasworld
            .reachabilityareas
            .offset(i as isize))
        .numareas = numareas;
        j = 0 as libc::c_int;
        while j < numareas {
            let fresh15 = numreachareas;
            numreachareas = numreachareas + 1;
            *crate::src::botlib::be_aas_main::aasworld
                .reachabilityareaindex
                .offset(fresh15 as isize) = areas[j as usize];
            j += 1
        }
        i += 1
    }
    //end for
}
//end of the function AAS_InitReachabilityAreas
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_InitRouting() {
    AAS_InitTravelFlagFromType();
    //
    AAS_InitAreaContentsTravelFlags();
    //initialize the routing update fields
    AAS_InitRoutingUpdate();
    //create reversed reachability links used by the routing update algorithm
    AAS_CreateReversedReachability();
    //initialize the cluster cache
    AAS_InitClusterAreaCache();
    //initialize portal cache
    AAS_InitPortalCache();
    //initialize the area travel times
    AAS_CalculateAreaTravelTimes();
    //calculate the maximum travel times through portals
    AAS_InitPortalMaxTravelTimes();
    //get the areas reachabilities go through
    AAS_InitReachabilityAreas();
    //
    numareacacheupdates = 0 as libc::c_int;
    numportalcacheupdates = 0 as libc::c_int;
    //ROUTING_DEBUG
    //
    routingcachesize = 0 as libc::c_int;
    max_routingcachesize = 1024 as libc::c_int
        * crate::src::botlib::l_libvar::LibVarValue(
            b"max_routingcache\x00" as *const u8 as *const libc::c_char,
            b"4096\x00" as *const u8 as *const libc::c_char,
        ) as libc::c_int;
    // read any routing cache if available
    AAS_ReadRouteCache();
}
//end of the function AAS_InitRouting
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_FreeRoutingCaches() {
    // free all the existing cluster area cache
    AAS_FreeAllClusterAreaCache();
    // free all the existing portal cache
    AAS_FreeAllPortalCache();
    // free cached travel times within areas
    if !crate::src::botlib::be_aas_main::aasworld
        .areatraveltimes
        .is_null()
    {
        crate::src::botlib::l_memory::FreeMemory(
            crate::src::botlib::be_aas_main::aasworld.areatraveltimes as *mut libc::c_void,
        );
    }
    crate::src::botlib::be_aas_main::aasworld.areatraveltimes = 0 as *mut *mut *mut libc::c_ushort;
    // free cached maximum travel time through cluster portals
    if !crate::src::botlib::be_aas_main::aasworld
        .portalmaxtraveltimes
        .is_null()
    {
        crate::src::botlib::l_memory::FreeMemory(
            crate::src::botlib::be_aas_main::aasworld.portalmaxtraveltimes as *mut libc::c_void,
        );
    }
    crate::src::botlib::be_aas_main::aasworld.portalmaxtraveltimes = 0 as *mut libc::c_int;
    // free reversed reachability links
    if !crate::src::botlib::be_aas_main::aasworld
        .reversedreachability
        .is_null()
    {
        crate::src::botlib::l_memory::FreeMemory(
            crate::src::botlib::be_aas_main::aasworld.reversedreachability as *mut libc::c_void,
        );
    }
    crate::src::botlib::be_aas_main::aasworld.reversedreachability =
        0 as *mut crate::be_aas_def_h::aas_reversedreachability_t;
    // free routing algorithm memory
    if !crate::src::botlib::be_aas_main::aasworld
        .areaupdate
        .is_null()
    {
        crate::src::botlib::l_memory::FreeMemory(
            crate::src::botlib::be_aas_main::aasworld.areaupdate as *mut libc::c_void,
        );
    }
    crate::src::botlib::be_aas_main::aasworld.areaupdate =
        0 as *mut crate::be_aas_def_h::aas_routingupdate_t;
    if !crate::src::botlib::be_aas_main::aasworld
        .portalupdate
        .is_null()
    {
        crate::src::botlib::l_memory::FreeMemory(
            crate::src::botlib::be_aas_main::aasworld.portalupdate as *mut libc::c_void,
        );
    }
    crate::src::botlib::be_aas_main::aasworld.portalupdate =
        0 as *mut crate::be_aas_def_h::aas_routingupdate_t;
    // free lists with areas the reachabilities go through
    if !crate::src::botlib::be_aas_main::aasworld
        .reachabilityareas
        .is_null()
    {
        crate::src::botlib::l_memory::FreeMemory(
            crate::src::botlib::be_aas_main::aasworld.reachabilityareas as *mut libc::c_void,
        );
    }
    crate::src::botlib::be_aas_main::aasworld.reachabilityareas =
        0 as *mut crate::be_aas_def_h::aas_reachabilityareas_t;
    // free the reachability area index
    if !crate::src::botlib::be_aas_main::aasworld
        .reachabilityareaindex
        .is_null()
    {
        crate::src::botlib::l_memory::FreeMemory(
            crate::src::botlib::be_aas_main::aasworld.reachabilityareaindex as *mut libc::c_void,
        );
    }
    crate::src::botlib::be_aas_main::aasworld.reachabilityareaindex = 0 as *mut libc::c_int;
    // free area contents travel flags look up table
    if !crate::src::botlib::be_aas_main::aasworld
        .areacontentstravelflags
        .is_null()
    {
        crate::src::botlib::l_memory::FreeMemory(
            crate::src::botlib::be_aas_main::aasworld.areacontentstravelflags as *mut libc::c_void,
        );
    }
    crate::src::botlib::be_aas_main::aasworld.areacontentstravelflags = 0 as *mut libc::c_int;
}
//end of the function AAS_FreeRoutingCaches
//===========================================================================
// update the given routing cache
//
// Parameter:			areacache		: routing cache to update
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_UpdateAreaRoutingCache(
    mut areacache: *mut crate::be_aas_def_h::aas_routingcache_t,
) {
    let mut i: libc::c_int = 0; //NOTE: not more than 128 reachabilities per area allowed
    let mut nextareanum: libc::c_int = 0;
    let mut cluster: libc::c_int = 0;
    let mut badtravelflags: libc::c_int = 0;
    let mut clusterareanum: libc::c_int = 0;
    let mut linknum: libc::c_int = 0;
    let mut numreachabilityareas: libc::c_int = 0;
    let mut t: libc::c_ushort = 0;
    let mut startareatraveltimes: [libc::c_ushort; 128] = [0; 128];
    let mut updateliststart: *mut crate::be_aas_def_h::aas_routingupdate_t =
        0 as *mut crate::be_aas_def_h::aas_routingupdate_t;
    let mut updatelistend: *mut crate::be_aas_def_h::aas_routingupdate_t =
        0 as *mut crate::be_aas_def_h::aas_routingupdate_t;
    let mut curupdate: *mut crate::be_aas_def_h::aas_routingupdate_t =
        0 as *mut crate::be_aas_def_h::aas_routingupdate_t;
    let mut nextupdate: *mut crate::be_aas_def_h::aas_routingupdate_t =
        0 as *mut crate::be_aas_def_h::aas_routingupdate_t;
    let mut reach: *mut crate::aasfile_h::aas_reachability_t =
        0 as *mut crate::aasfile_h::aas_reachability_t;
    let mut revreach: *mut crate::be_aas_def_h::aas_reversedreachability_t =
        0 as *mut crate::be_aas_def_h::aas_reversedreachability_t;
    let mut revlink: *mut crate::be_aas_def_h::aas_reversedlink_t =
        0 as *mut crate::be_aas_def_h::aas_reversedlink_t;
    numareacacheupdates += 1;
    //ROUTING_DEBUG
    //number of reachability areas within this cluster
    numreachabilityareas = (*crate::src::botlib::be_aas_main::aasworld
        .clusters
        .offset((*areacache).cluster as isize))
    .numreachabilityareas;
    //
    crate::src::botlib::be_aas_main::aasworld.frameroutingupdates += 1;
    //clear the routing update fields
    //	Com_Memset(aasworld.areaupdate, 0, aasworld.numareas * sizeof(aas_routingupdate_t));
    //
    badtravelflags = !(*areacache).travelflags;
    //
    clusterareanum = AAS_ClusterAreaNum((*areacache).cluster, (*areacache).areanum);
    if clusterareanum >= numreachabilityareas {
        return;
    }
    //
    crate::stdlib::memset(
        startareatraveltimes.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[libc::c_ushort; 128]>() as libc::c_ulong,
    );
    //
    curupdate = &mut *crate::src::botlib::be_aas_main::aasworld
        .areaupdate
        .offset(clusterareanum as isize)
        as *mut crate::be_aas_def_h::aas_routingupdate_t;
    (*curupdate).areanum = (*areacache).areanum;
    //VectorCopy(areacache->origin, curupdate->start);
    (*curupdate).areatraveltimes = startareatraveltimes.as_mut_ptr();
    (*curupdate).tmptraveltime = (*areacache).starttraveltime as libc::c_ushort;
    //
    *(*areacache)
        .traveltimes
        .as_mut_ptr()
        .offset(clusterareanum as isize) = (*areacache).starttraveltime as libc::c_ushort;
    //put the area to start with in the current read list
    (*curupdate).next = 0 as *mut crate::be_aas_def_h::aas_routingupdate_s;
    (*curupdate).prev = 0 as *mut crate::be_aas_def_h::aas_routingupdate_s;
    updateliststart = curupdate;
    updatelistend = curupdate;
    //while there are updates in the current list
    while !updateliststart.is_null() {
        curupdate = updateliststart;
        //end for
        if !(*curupdate).next.is_null() {
            (*(*curupdate).next).prev = 0 as *mut crate::be_aas_def_h::aas_routingupdate_s
        } else {
            updatelistend = 0 as *mut crate::be_aas_def_h::aas_routingupdate_t
        }
        updateliststart = (*curupdate).next;
        (*curupdate).inlist = crate::src::qcommon::q_shared::qfalse;
        revreach = &mut *crate::src::botlib::be_aas_main::aasworld
            .reversedreachability
            .offset((*curupdate).areanum as isize)
            as *mut crate::be_aas_def_h::aas_reversedreachability_t;
        i = 0 as libc::c_int;
        revlink = (*revreach).first;
        while !revlink.is_null() {
            linknum = (*revlink).linknum;
            reach = &mut *crate::src::botlib::be_aas_main::aasworld
                .reachability
                .offset(linknum as isize)
                as *mut crate::aasfile_h::aas_reachability_t;
            //
            //
            //check all reversed reachability links
            //
            //end if
            //if there is used an undesired travel type
            if !(AAS_TravelFlagForType_inline((*reach).traveltype) & badtravelflags != 0) {
                //if not allowed to enter the next area
                if !((*crate::src::botlib::be_aas_main::aasworld
                    .areasettings
                    .offset((*reach).areanum as isize))
                .areaflags
                    & 8 as libc::c_int
                    != 0)
                {
                    //if the next area has a not allowed travel flag
                    if !(AAS_AreaContentsTravelFlags_inline((*reach).areanum) & badtravelflags != 0)
                    {
                        //number of the area the reversed reachability leads to
                        nextareanum = (*revlink).areanum;
                        //get the cluster number of the area
                        cluster = (*crate::src::botlib::be_aas_main::aasworld
                            .areasettings
                            .offset(nextareanum as isize))
                        .cluster;
                        //don't leave the cluster
                        if !(cluster > 0 as libc::c_int && cluster != (*areacache).cluster) {
                            //get the number of the area in the cluster
                            clusterareanum = AAS_ClusterAreaNum((*areacache).cluster, nextareanum);
                            if !(clusterareanum >= numreachabilityareas) {
                                //time already travelled plus the traveltime through
                                //the current area plus the travel time from the reachability
                                t = ((*curupdate).tmptraveltime as libc::c_int
                                    + *(*curupdate).areatraveltimes.offset(i as isize)
                                        as libc::c_int
                                    + (*reach).traveltime as libc::c_int)
                                    as libc::c_ushort;
                                //
                                if *(*areacache)
                                    .traveltimes
                                    .as_mut_ptr()
                                    .offset(clusterareanum as isize)
                                    == 0
                                    || *(*areacache)
                                        .traveltimes
                                        .as_mut_ptr()
                                        .offset(clusterareanum as isize)
                                        as libc::c_int
                                        > t as libc::c_int
                                {
                                    *(*areacache)
                                        .traveltimes
                                        .as_mut_ptr()
                                        .offset(clusterareanum as isize) = t;
                                    *(*areacache).reachabilities.offset(clusterareanum as isize) =
                                        (linknum
                                            - (*crate::src::botlib::be_aas_main::aasworld
                                                .areasettings
                                                .offset(nextareanum as isize))
                                            .firstreachablearea)
                                            as libc::c_uchar;
                                    nextupdate = &mut *crate::src::botlib::be_aas_main::aasworld
                                        .areaupdate
                                        .offset(clusterareanum as isize)
                                        as *mut crate::be_aas_def_h::aas_routingupdate_t;
                                    (*nextupdate).areanum = nextareanum;
                                    (*nextupdate).tmptraveltime = t;
                                    //end if
                                    (*nextupdate).areatraveltimes =
                                        *(*crate::src::botlib::be_aas_main::aasworld
                                            .areatraveltimes
                                            .offset(nextareanum as isize))
                                        .offset(
                                            (linknum
                                                - (*crate::src::botlib::be_aas_main::aasworld
                                                    .areasettings
                                                    .offset(nextareanum as isize))
                                                .firstreachablearea)
                                                as isize,
                                        );
                                    if (*nextupdate).inlist as u64 == 0 {
                                        //VectorCopy(reach->start, nextupdate->start);
                                        // we add the update to the end of the list
                                        // we could also use a B+ tree to have a real sorted list
                                        // on travel time which makes for faster routing updates
                                        (*nextupdate).next =
                                            0 as *mut crate::be_aas_def_h::aas_routingupdate_s;
                                        (*nextupdate).prev = updatelistend;
                                        if !updatelistend.is_null() {
                                            (*updatelistend).next = nextupdate
                                        } else {
                                            updateliststart = nextupdate
                                        }
                                        updatelistend = nextupdate;
                                        (*nextupdate).inlist = crate::src::qcommon::q_shared::qtrue
                                    }
                                }
                            }
                        }
                    }
                }
            }
            revlink = (*revlink).next;
            i += 1
        }
    }
    //end while
}
//end of the function AAS_UpdateAreaRoutingCache
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_GetAreaRoutingCache(
    mut clusternum: libc::c_int,
    mut areanum: libc::c_int,
    mut travelflags: libc::c_int,
) -> *mut crate::be_aas_def_h::aas_routingcache_t {
    let mut clusterareanum: libc::c_int = 0;
    let mut cache: *mut crate::be_aas_def_h::aas_routingcache_t =
        0 as *mut crate::be_aas_def_h::aas_routingcache_t;
    let mut clustercache: *mut crate::be_aas_def_h::aas_routingcache_t =
        0 as *mut crate::be_aas_def_h::aas_routingcache_t;
    //number of the area in the cluster
    clusterareanum = AAS_ClusterAreaNum(clusternum, areanum);
    //pointer to the cache for the area in the cluster
    clustercache = *(*crate::src::botlib::be_aas_main::aasworld
        .clusterareacache
        .offset(clusternum as isize))
    .offset(clusterareanum as isize);
    //find the cache without undesired travel flags
    cache = clustercache; //end for
    while !cache.is_null() {
        //if there aren't used any undesired travel types for the cache
        if (*cache).travelflags == travelflags {
            break;
        }
        cache = (*cache).next
    }
    //if there was no cache
    if cache.is_null() {
        //end else
        cache = AAS_AllocRoutingCache(
            (*crate::src::botlib::be_aas_main::aasworld
                .clusters
                .offset(clusternum as isize))
            .numreachabilityareas,
        ); //end if
        (*cache).cluster = clusternum;
        (*cache).areanum = areanum;
        (*cache).origin[0 as libc::c_int as usize] = (*crate::src::botlib::be_aas_main::aasworld
            .areas
            .offset(areanum as isize))
        .center[0 as libc::c_int as usize];
        (*cache).origin[1 as libc::c_int as usize] = (*crate::src::botlib::be_aas_main::aasworld
            .areas
            .offset(areanum as isize))
        .center[1 as libc::c_int as usize];
        (*cache).origin[2 as libc::c_int as usize] = (*crate::src::botlib::be_aas_main::aasworld
            .areas
            .offset(areanum as isize))
        .center[2 as libc::c_int as usize];
        (*cache).starttraveltime = 1 as libc::c_int as libc::c_float;
        (*cache).travelflags = travelflags;
        (*cache).prev = 0 as *mut crate::be_aas_def_h::aas_routingcache_s;
        (*cache).next = clustercache;
        if !clustercache.is_null() {
            (*clustercache).prev = cache
        }
        let ref mut fresh16 = *(*crate::src::botlib::be_aas_main::aasworld
            .clusterareacache
            .offset(clusternum as isize))
        .offset(clusterareanum as isize);
        *fresh16 = cache;
        AAS_UpdateAreaRoutingCache(cache);
    } else {
        AAS_UnlinkCache(cache);
    }
    //the cache has been accessed
    (*cache).time = AAS_RoutingTime();
    (*cache).type_0 = 1 as libc::c_int as crate::src::qcommon::q_shared::byte;
    AAS_LinkCache(cache);
    return cache;
}
//end of the function AAS_GetAreaRoutingCache
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_UpdatePortalRoutingCache(
    mut portalcache: *mut crate::be_aas_def_h::aas_routingcache_t,
) {
    let mut i: libc::c_int = 0;
    let mut portalnum: libc::c_int = 0;
    let mut clusterareanum: libc::c_int = 0;
    let mut clusternum: libc::c_int = 0;
    let mut t: libc::c_ushort = 0;
    let mut portal: *mut crate::aasfile_h::aas_portal_t = 0 as *mut crate::aasfile_h::aas_portal_t;
    let mut cluster: *mut crate::aasfile_h::aas_cluster_t =
        0 as *mut crate::aasfile_h::aas_cluster_t;
    let mut cache: *mut crate::be_aas_def_h::aas_routingcache_t =
        0 as *mut crate::be_aas_def_h::aas_routingcache_t;
    let mut updateliststart: *mut crate::be_aas_def_h::aas_routingupdate_t =
        0 as *mut crate::be_aas_def_h::aas_routingupdate_t;
    let mut updatelistend: *mut crate::be_aas_def_h::aas_routingupdate_t =
        0 as *mut crate::be_aas_def_h::aas_routingupdate_t;
    let mut curupdate: *mut crate::be_aas_def_h::aas_routingupdate_t =
        0 as *mut crate::be_aas_def_h::aas_routingupdate_t;
    let mut nextupdate: *mut crate::be_aas_def_h::aas_routingupdate_t =
        0 as *mut crate::be_aas_def_h::aas_routingupdate_t;
    numportalcacheupdates += 1;
    //ROUTING_DEBUG
    //clear the routing update fields
    //	Com_Memset(aasworld.portalupdate, 0, (aasworld.numportals+1) * sizeof(aas_routingupdate_t));
    //
    curupdate = &mut *crate::src::botlib::be_aas_main::aasworld
        .portalupdate
        .offset(crate::src::botlib::be_aas_main::aasworld.numportals as isize)
        as *mut crate::be_aas_def_h::aas_routingupdate_t;
    (*curupdate).cluster = (*portalcache).cluster;
    (*curupdate).areanum = (*portalcache).areanum;
    (*curupdate).tmptraveltime = (*portalcache).starttraveltime as libc::c_ushort;
    //if the start area is a cluster portal, store the travel time for that portal
    clusternum = (*crate::src::botlib::be_aas_main::aasworld
        .areasettings
        .offset((*portalcache).areanum as isize))
    .cluster; //end if
    if clusternum < 0 as libc::c_int {
        *(*portalcache)
            .traveltimes
            .as_mut_ptr()
            .offset(-clusternum as isize) = (*portalcache).starttraveltime as libc::c_ushort
    }
    //put the area to start with in the current read list
    (*curupdate).next = 0 as *mut crate::be_aas_def_h::aas_routingupdate_s;
    (*curupdate).prev = 0 as *mut crate::be_aas_def_h::aas_routingupdate_s;
    updateliststart = curupdate;
    updatelistend = curupdate;
    //while there are updates in the current list
    while !updateliststart.is_null() {
        curupdate = updateliststart;
        //end for
        if !(*curupdate).next.is_null() {
            (*(*curupdate).next).prev = 0 as *mut crate::be_aas_def_h::aas_routingupdate_s
        } else {
            updatelistend = 0 as *mut crate::be_aas_def_h::aas_routingupdate_t
        }
        updateliststart = (*curupdate).next;
        (*curupdate).inlist = crate::src::qcommon::q_shared::qfalse;
        cluster = &mut *crate::src::botlib::be_aas_main::aasworld
            .clusters
            .offset((*curupdate).cluster as isize)
            as *mut crate::aasfile_h::aas_cluster_t;
        cache = AAS_GetAreaRoutingCache(
            (*curupdate).cluster,
            (*curupdate).areanum,
            (*portalcache).travelflags,
        );
        i = 0 as libc::c_int;
        while i < (*cluster).numportals {
            portalnum = *crate::src::botlib::be_aas_main::aasworld
                .portalindex
                .offset(((*cluster).firstportal + i) as isize);
            portal = &mut *crate::src::botlib::be_aas_main::aasworld
                .portals
                .offset(portalnum as isize)
                as *mut crate::aasfile_h::aas_portal_t;
            //remove the current update from the list
            //current update is removed from the list
            //
            //
            //take all portals of the cluster
            //end if
            //if this is the portal of the current update continue
            if !((*portal).areanum == (*curupdate).areanum) {
                //
                clusterareanum = AAS_ClusterAreaNum((*curupdate).cluster, (*portal).areanum);
                if !(clusterareanum >= (*cluster).numreachabilityareas) {
                    //
                    t = *(*cache)
                        .traveltimes
                        .as_mut_ptr()
                        .offset(clusterareanum as isize);
                    if !(t == 0) {
                        t = (t as libc::c_int + (*curupdate).tmptraveltime as libc::c_int)
                            as libc::c_ushort;
                        //
                        if *(*portalcache)
                            .traveltimes
                            .as_mut_ptr()
                            .offset(portalnum as isize)
                            == 0
                            || *(*portalcache)
                                .traveltimes
                                .as_mut_ptr()
                                .offset(portalnum as isize)
                                as libc::c_int
                                > t as libc::c_int
                        {
                            *(*portalcache)
                                .traveltimes
                                .as_mut_ptr()
                                .offset(portalnum as isize) = t;
                            nextupdate = &mut *crate::src::botlib::be_aas_main::aasworld
                                .portalupdate
                                .offset(portalnum as isize)
                                as *mut crate::be_aas_def_h::aas_routingupdate_t;
                            //end if
                            if (*portal).frontcluster == (*curupdate).cluster {
                                //end else
                                (*nextupdate).cluster = (*portal).backcluster
                            } else {
                                (*nextupdate).cluster = (*portal).frontcluster
                            } //end if
                            (*nextupdate).areanum = (*portal).areanum;
                            (*nextupdate).tmptraveltime = (t as libc::c_int
                                + *crate::src::botlib::be_aas_main::aasworld
                                    .portalmaxtraveltimes
                                    .offset(portalnum as isize))
                                as libc::c_ushort;
                            if (*nextupdate).inlist as u64 == 0 {
                                //add travel time through the actual portal area for the next update
                                // we add the update to the end of the list
                                // we could also use a B+ tree to have a real sorted list
                                // on travel time which makes for faster routing updates
                                (*nextupdate).next =
                                    0 as *mut crate::be_aas_def_h::aas_routingupdate_s;
                                (*nextupdate).prev = updatelistend;
                                if !updatelistend.is_null() {
                                    (*updatelistend).next = nextupdate
                                } else {
                                    updateliststart = nextupdate
                                }
                                updatelistend = nextupdate;
                                (*nextupdate).inlist = crate::src::qcommon::q_shared::qtrue
                            }
                        }
                    }
                }
            }
            i += 1
        }
    }
    //end while
}
//end of the function AAS_UpdatePortalRoutingCache
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_GetPortalRoutingCache(
    mut clusternum: libc::c_int,
    mut areanum: libc::c_int,
    mut travelflags: libc::c_int,
) -> *mut crate::be_aas_def_h::aas_routingcache_t {
    let mut cache: *mut crate::be_aas_def_h::aas_routingcache_t =
        0 as *mut crate::be_aas_def_h::aas_routingcache_t;
    //find the cached portal routing if existing
    cache = *crate::src::botlib::be_aas_main::aasworld
        .portalcache
        .offset(areanum as isize); //end for
    while !cache.is_null() {
        if (*cache).travelflags == travelflags {
            break;
        }
        cache = (*cache).next
    }
    //if the portal routing isn't cached
    if cache.is_null() {
        //end else
        cache = AAS_AllocRoutingCache(crate::src::botlib::be_aas_main::aasworld.numportals); //end if
        (*cache).cluster = clusternum;
        (*cache).areanum = areanum;
        (*cache).origin[0 as libc::c_int as usize] = (*crate::src::botlib::be_aas_main::aasworld
            .areas
            .offset(areanum as isize))
        .center[0 as libc::c_int as usize];
        (*cache).origin[1 as libc::c_int as usize] = (*crate::src::botlib::be_aas_main::aasworld
            .areas
            .offset(areanum as isize))
        .center[1 as libc::c_int as usize];
        (*cache).origin[2 as libc::c_int as usize] = (*crate::src::botlib::be_aas_main::aasworld
            .areas
            .offset(areanum as isize))
        .center[2 as libc::c_int as usize];
        (*cache).starttraveltime = 1 as libc::c_int as libc::c_float;
        (*cache).travelflags = travelflags;
        //add the cache to the cache list
        (*cache).prev = 0 as *mut crate::be_aas_def_h::aas_routingcache_s;
        (*cache).next = *crate::src::botlib::be_aas_main::aasworld
            .portalcache
            .offset(areanum as isize);
        if !(*crate::src::botlib::be_aas_main::aasworld
            .portalcache
            .offset(areanum as isize))
        .is_null()
        {
            let ref mut fresh17 = (**crate::src::botlib::be_aas_main::aasworld
                .portalcache
                .offset(areanum as isize))
            .prev;
            *fresh17 = cache
        }
        let ref mut fresh18 = *crate::src::botlib::be_aas_main::aasworld
            .portalcache
            .offset(areanum as isize);
        *fresh18 = cache;
        //update the cache
        AAS_UpdatePortalRoutingCache(cache);
    } else {
        AAS_UnlinkCache(cache);
    }
    //the cache has been accessed
    (*cache).time = AAS_RoutingTime();
    (*cache).type_0 = 0 as libc::c_int as crate::src::qcommon::q_shared::byte;
    AAS_LinkCache(cache);
    return cache;
}
//end of the function AAS_GetPortalRoutingCache
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_AreaRouteToGoalArea(
    mut areanum: libc::c_int,
    mut origin: *mut crate::src::qcommon::q_shared::vec_t,
    mut goalareanum: libc::c_int,
    mut travelflags: libc::c_int,
    mut traveltime: *mut libc::c_int,
    mut reachnum: *mut libc::c_int,
) -> libc::c_int {
    let mut clusternum: libc::c_int = 0;
    let mut goalclusternum: libc::c_int = 0;
    let mut portalnum: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut clusterareanum: libc::c_int = 0;
    let mut bestreachnum: libc::c_int = 0;
    let mut t: libc::c_ushort = 0;
    let mut besttime: libc::c_ushort = 0;
    let mut portal: *mut crate::aasfile_h::aas_portal_t = 0 as *mut crate::aasfile_h::aas_portal_t;
    let mut cluster: *mut crate::aasfile_h::aas_cluster_t =
        0 as *mut crate::aasfile_h::aas_cluster_t;
    let mut areacache: *mut crate::be_aas_def_h::aas_routingcache_t =
        0 as *mut crate::be_aas_def_h::aas_routingcache_t;
    let mut portalcache: *mut crate::be_aas_def_h::aas_routingcache_t =
        0 as *mut crate::be_aas_def_h::aas_routingcache_t;
    let mut reach: *mut crate::aasfile_h::aas_reachability_t =
        0 as *mut crate::aasfile_h::aas_reachability_t;
    if crate::src::botlib::be_aas_main::aasworld.initialized == 0 {
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    }
    if areanum == goalareanum {
        *traveltime = 1 as libc::c_int;
        *reachnum = 0 as libc::c_int;
        return crate::src::qcommon::q_shared::qtrue as libc::c_int;
    }
    //check !AAS_AreaReachability(areanum) with custom developer-only debug message
    if areanum <= 0 as libc::c_int || areanum >= crate::src::botlib::be_aas_main::aasworld.numareas
    {
        //end if
        if crate::src::botlib::be_interface::botDeveloper != 0 {
            crate::src::botlib::be_interface::botimport
                .Print
                .expect("non-null function pointer")(
                3 as libc::c_int,
                b"AAS_AreaTravelTimeToGoalArea: areanum %d out of range\n\x00" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                areanum,
            ); //end if
        } //end if
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    } //end if
    if goalareanum <= 0 as libc::c_int
        || goalareanum >= crate::src::botlib::be_aas_main::aasworld.numareas
    {
        if crate::src::botlib::be_interface::botDeveloper != 0 {
            crate::src::botlib::be_interface::botimport
                .Print
                .expect("non-null function pointer")(
                3 as libc::c_int,
                b"AAS_AreaTravelTimeToGoalArea: goalareanum %d out of range\n\x00" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                goalareanum,
            ); //end if
        }
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    }
    if (*crate::src::botlib::be_aas_main::aasworld
        .areasettings
        .offset(areanum as isize))
    .numreachableareas
        == 0
        || (*crate::src::botlib::be_aas_main::aasworld
            .areasettings
            .offset(goalareanum as isize))
        .numreachableareas
            == 0
    {
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    }
    // make sure the routing cache doesn't grow to large
    while crate::src::botlib::l_memory::AvailableMemory()
        < 1 as libc::c_int * 1024 as libc::c_int * 1024 as libc::c_int
    {
        if AAS_FreeOldestCache() == 0 {
            break;
        }
    }
    //
    if crate::src::botlib::be_aas_reach::AAS_AreaDoNotEnter(areanum) != 0
        || crate::src::botlib::be_aas_reach::AAS_AreaDoNotEnter(goalareanum) != 0
    {
        travelflags |= 0x800000 as libc::c_int
    } //end if
      //NOTE: the number of routing updates is limited per frame
      /*
          if (aasworld.frameroutingupdates > MAX_FRAMEROUTINGUPDATES)
          {
      #ifdef DEBUG
              //Log_Write("WARNING: AAS_AreaTravelTimeToGoalArea: frame routing updates overflowed");
      #endif
              return 0;
          } //end if
          */
    //
    clusternum = (*crate::src::botlib::be_aas_main::aasworld
        .areasettings
        .offset(areanum as isize))
    .cluster;
    goalclusternum = (*crate::src::botlib::be_aas_main::aasworld
        .areasettings
        .offset(goalareanum as isize))
    .cluster;
    //check if the area is a portal of the goal area cluster
    if clusternum < 0 as libc::c_int && goalclusternum > 0 as libc::c_int {
        portal = &mut *crate::src::botlib::be_aas_main::aasworld
            .portals
            .offset(-clusternum as isize) as *mut crate::aasfile_h::aas_portal_t;
        if (*portal).frontcluster == goalclusternum || (*portal).backcluster == goalclusternum {
            clusternum = goalclusternum
        } //end if
          //end if
    } else if clusternum > 0 as libc::c_int && goalclusternum < 0 as libc::c_int {
        portal = &mut *crate::src::botlib::be_aas_main::aasworld
            .portals
            .offset(-goalclusternum as isize)
            as *mut crate::aasfile_h::aas_portal_t;
        if (*portal).frontcluster == clusternum || (*portal).backcluster == clusternum {
            goalclusternum = clusternum
        }
        //check if the goalarea is a portal of the area cluster
        //end if
    }
    //if both areas are in the same cluster
    //NOTE: there might be a shorter route via another cluster!!! but we don't care
    if clusternum > 0 as libc::c_int
        && goalclusternum > 0 as libc::c_int
        && clusternum == goalclusternum
    {
        //end if
        //
        areacache = AAS_GetAreaRoutingCache(clusternum, goalareanum, travelflags);
        //end if
        clusterareanum = AAS_ClusterAreaNum(clusternum, areanum);
        cluster = &mut *crate::src::botlib::be_aas_main::aasworld
            .clusters
            .offset(clusternum as isize) as *mut crate::aasfile_h::aas_cluster_t;
        if clusterareanum >= (*cluster).numreachabilityareas {
            return 0 as libc::c_int;
        }
        if *(*areacache)
            .traveltimes
            .as_mut_ptr()
            .offset(clusterareanum as isize) as libc::c_int
            != 0 as libc::c_int
        {
            *reachnum = (*crate::src::botlib::be_aas_main::aasworld
                .areasettings
                .offset(areanum as isize))
            .firstreachablearea
                + *(*areacache).reachabilities.offset(clusterareanum as isize) as libc::c_int;
            if origin.is_null() {
                *traveltime = *(*areacache)
                    .traveltimes
                    .as_mut_ptr()
                    .offset(clusterareanum as isize) as libc::c_int;
                return crate::src::qcommon::q_shared::qtrue as libc::c_int;
            }
            reach = &mut *crate::src::botlib::be_aas_main::aasworld
                .reachability
                .offset(*reachnum as isize)
                as *mut crate::aasfile_h::aas_reachability_t;
            *traveltime = *(*areacache)
                .traveltimes
                .as_mut_ptr()
                .offset(clusterareanum as isize) as libc::c_int
                + AAS_AreaTravelTime(areanum, origin, (*reach).start.as_mut_ptr()) as libc::c_int;
            //the number of the area in the cluster
            //the cluster the area is in
            //if the area is NOT a reachability area
            //if it is possible to travel to the goal area through this cluster
            //
            return crate::src::qcommon::q_shared::qtrue as libc::c_int;
        }
    }
    //
    clusternum = (*crate::src::botlib::be_aas_main::aasworld
        .areasettings
        .offset(areanum as isize))
    .cluster;
    goalclusternum = (*crate::src::botlib::be_aas_main::aasworld
        .areasettings
        .offset(goalareanum as isize))
    .cluster;
    //if the goal area is a portal
    if goalclusternum < 0 as libc::c_int {
        //end if
        //just assume the goal area is part of the front cluster
        portal = &mut *crate::src::botlib::be_aas_main::aasworld
            .portals
            .offset(-goalclusternum as isize)
            as *mut crate::aasfile_h::aas_portal_t;
        goalclusternum = (*portal).frontcluster
    }
    //get the portal routing cache
    portalcache = AAS_GetPortalRoutingCache(goalclusternum, goalareanum, travelflags);
    //if the area is a cluster portal, read directly from the portal cache
    if clusternum < 0 as libc::c_int {
        *traveltime = *(*portalcache)
            .traveltimes
            .as_mut_ptr()
            .offset(-clusternum as isize) as libc::c_int; //end if
        *reachnum = (*crate::src::botlib::be_aas_main::aasworld
            .areasettings
            .offset(areanum as isize))
        .firstreachablearea
            + *(*portalcache).reachabilities.offset(-clusternum as isize) as libc::c_int;
        return crate::src::qcommon::q_shared::qtrue as libc::c_int;
    }
    //
    besttime = 0 as libc::c_int as libc::c_ushort;
    bestreachnum = -(1 as libc::c_int);
    //the cluster the area is in
    cluster = &mut *crate::src::botlib::be_aas_main::aasworld
        .clusters
        .offset(clusternum as isize) as *mut crate::aasfile_h::aas_cluster_t;
    //find the portal of the area cluster leading towards the goal area
    i = 0 as libc::c_int; //end for
    while i < (*cluster).numportals {
        portalnum = *crate::src::botlib::be_aas_main::aasworld
            .portalindex
            .offset(((*cluster).firstportal + i) as isize);
        //end if
        //if the goal area isn't reachable from the portal
        if !(*(*portalcache)
            .traveltimes
            .as_mut_ptr()
            .offset(portalnum as isize)
            == 0)
        {
            //
            portal = &mut *crate::src::botlib::be_aas_main::aasworld
                .portals
                .offset(portalnum as isize)
                as *mut crate::aasfile_h::aas_portal_t;
            //get the cache of the portal area
            areacache = AAS_GetAreaRoutingCache(clusternum, (*portal).areanum, travelflags);
            //current area inside the current cluster
            clusterareanum = AAS_ClusterAreaNum(clusternum, areanum);
            //if the area is NOT a reachability area
            if !(clusterareanum >= (*cluster).numreachabilityareas) {
                //if the portal is NOT reachable from this area
                if !(*(*areacache)
                    .traveltimes
                    .as_mut_ptr()
                    .offset(clusterareanum as isize)
                    == 0)
                {
                    //total travel time is the travel time the portal area is from
                    //the goal area plus the travel time towards the portal area
                    t = (*(*portalcache)
                        .traveltimes
                        .as_mut_ptr()
                        .offset(portalnum as isize) as libc::c_int
                        + *(*areacache)
                            .traveltimes
                            .as_mut_ptr()
                            .offset(clusterareanum as isize)
                            as libc::c_int) as libc::c_ushort;
                    //FIXME: add the exact travel time through the actual portal area
                    //NOTE: for now we just add the largest travel time through the portal area
                    //		because we can't directly calculate the exact travel time
                    //		to be more specific we don't know which reachability was used to travel
                    //		into the portal area
                    t = (t as libc::c_int
                        + *crate::src::botlib::be_aas_main::aasworld
                            .portalmaxtraveltimes
                            .offset(portalnum as isize)) as libc::c_ushort;
                    //
                    if !origin.is_null() {
                        *reachnum = (*crate::src::botlib::be_aas_main::aasworld
                            .areasettings
                            .offset(areanum as isize))
                        .firstreachablearea
                            + *(*areacache).reachabilities.offset(clusterareanum as isize)
                                as libc::c_int; //end if
                        reach = crate::src::botlib::be_aas_main::aasworld
                            .reachability
                            .offset(*reachnum as isize);
                        t = (t as libc::c_int
                            + AAS_AreaTravelTime(areanum, origin, (*reach).start.as_mut_ptr())
                                as libc::c_int) as libc::c_ushort
                    }
                    //if the time is better than the one already found
                    if besttime == 0 || (t as libc::c_int) < besttime as libc::c_int {
                        bestreachnum = *reachnum;
                        besttime = t
                    }
                }
            }
        }
        i += 1
    }
    if bestreachnum < 0 as libc::c_int {
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    }
    *reachnum = bestreachnum;
    *traveltime = besttime as libc::c_int;
    return crate::src::qcommon::q_shared::qtrue as libc::c_int;
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
 * name:		be_aas_route.h
 *
 * desc:		AAS
 *
 * $Archive: /source/code/botlib/be_aas_route.h $
 *
 *****************************************************************************/
/* ****************************************************************************
 * name:		be_aas_route.h
 *
 * desc:		AAS
 *
 * $Archive: /source/code/botlib/be_aas_route.h $
 *
 *****************************************************************************/
//initialize the AAS routing
//free the AAS routing caches
//returns the travel time from start to end in the given area
//
//
//AASINTERN
//AASINTERN
//returns the travel flag for the given travel type
//returns the travel flag for the given travel type
//return the travel flag(s) for traveling through this area
//return the travel flag(s) for traveling through this area
//returns the index of the next reachability for the given area
//returns the index of the next reachability for the given area
//returns the reachability with the given index
//returns the reachability with the given index
//returns a random goal area and goal origin
//returns a random goal area and goal origin
//enable or disable an area for routing
//enable or disable an area for routing
//returns the travel time within the given area from start to end
//returns the travel time within the given area from start to end
//returns the travel time from the area to the goal area using the given travel flags
//returns the travel time from the area to the goal area using the given travel flags
//end of the function AAS_AreaRouteToGoalArea
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_AreaTravelTimeToGoalArea(
    mut areanum: libc::c_int,
    mut origin: *mut crate::src::qcommon::q_shared::vec_t,
    mut goalareanum: libc::c_int,
    mut travelflags: libc::c_int,
) -> libc::c_int {
    let mut traveltime: libc::c_int = 0;
    let mut reachnum: libc::c_int = 0 as libc::c_int;
    if AAS_AreaRouteToGoalArea(
        areanum,
        origin,
        goalareanum,
        travelflags,
        &mut traveltime,
        &mut reachnum,
    ) != 0
    {
        return traveltime;
    }
    return 0 as libc::c_int;
}
//end of the function AAS_AreaTravelTimeToGoalArea
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_AreaReachabilityToGoalArea(
    mut areanum: libc::c_int,
    mut origin: *mut crate::src::qcommon::q_shared::vec_t,
    mut goalareanum: libc::c_int,
    mut travelflags: libc::c_int,
) -> libc::c_int {
    let mut traveltime: libc::c_int = 0;
    let mut reachnum: libc::c_int = 0 as libc::c_int;
    if AAS_AreaRouteToGoalArea(
        areanum,
        origin,
        goalareanum,
        travelflags,
        &mut traveltime,
        &mut reachnum,
    ) != 0
    {
        return reachnum;
    }
    return 0 as libc::c_int;
}
//predict a route up to a stop event
//predict a route up to a stop event
//end of the function AAS_AreaReachabilityToGoalArea
//===========================================================================
// predict the route and stop on one of the stop events
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_PredictRoute(
    mut route: *mut crate::be_aas_h::aas_predictroute_s,
    mut areanum: libc::c_int,
    mut origin: *mut crate::src::qcommon::q_shared::vec_t,
    mut goalareanum: libc::c_int,
    mut travelflags: libc::c_int,
    mut maxareas: libc::c_int,
    mut maxtime: libc::c_int,
    mut stopevent: libc::c_int,
    mut stopcontents: libc::c_int,
    mut stoptfl: libc::c_int,
    mut stopareanum: libc::c_int,
) -> libc::c_int {
    let mut curareanum: libc::c_int = 0;
    let mut reachnum: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut testareanum: libc::c_int = 0;
    let mut curorigin: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut reach: *mut crate::aasfile_h::aas_reachability_t =
        0 as *mut crate::aasfile_h::aas_reachability_t;
    let mut reachareas: *mut crate::be_aas_def_h::aas_reachabilityareas_t =
        0 as *mut crate::be_aas_def_h::aas_reachabilityareas_t;
    //init output
    (*route).stopevent = 0 as libc::c_int; //end while
    (*route).endarea = goalareanum; //end if
    (*route).endcontents = 0 as libc::c_int;
    (*route).endtravelflags = 0 as libc::c_int;
    (*route).endpos[0 as libc::c_int as usize] = *origin.offset(0 as libc::c_int as isize);
    (*route).endpos[1 as libc::c_int as usize] = *origin.offset(1 as libc::c_int as isize);
    (*route).endpos[2 as libc::c_int as usize] = *origin.offset(2 as libc::c_int as isize);
    (*route).time = 0 as libc::c_int;
    curareanum = areanum;
    curorigin[0 as libc::c_int as usize] = *origin.offset(0 as libc::c_int as isize);
    curorigin[1 as libc::c_int as usize] = *origin.offset(1 as libc::c_int as isize);
    curorigin[2 as libc::c_int as usize] = *origin.offset(2 as libc::c_int as isize);
    i = 0 as libc::c_int;
    while curareanum != goalareanum
        && (maxareas == 0 || i < maxareas)
        && i < crate::src::botlib::be_aas_main::aasworld.numareas
    {
        reachnum = AAS_AreaReachabilityToGoalArea(
            curareanum,
            curorigin.as_mut_ptr(),
            goalareanum,
            travelflags,
        );
        if reachnum == 0 {
            (*route).stopevent = 1 as libc::c_int;
            return crate::src::qcommon::q_shared::qfalse as libc::c_int;
        }
        reach = &mut *crate::src::botlib::be_aas_main::aasworld
            .reachability
            .offset(reachnum as isize) as *mut crate::aasfile_h::aas_reachability_t;
        //
        if stopevent & 2 as libc::c_int != 0 {
            //end if
            if AAS_TravelFlagForType_inline((*reach).traveltype) & stoptfl != 0 {
                (*route).stopevent = 2 as libc::c_int; //end if
                (*route).endarea = curareanum;
                (*route).endcontents = (*crate::src::botlib::be_aas_main::aasworld
                    .areasettings
                    .offset(curareanum as isize))
                .contents;
                (*route).endtravelflags = AAS_TravelFlagForType_inline((*reach).traveltype);
                (*route).endpos[0 as libc::c_int as usize] =
                    (*reach).start[0 as libc::c_int as usize];
                (*route).endpos[1 as libc::c_int as usize] =
                    (*reach).start[1 as libc::c_int as usize];
                (*route).endpos[2 as libc::c_int as usize] =
                    (*reach).start[2 as libc::c_int as usize];
                return crate::src::qcommon::q_shared::qtrue as libc::c_int;
            }
            if AAS_AreaContentsTravelFlags_inline((*reach).areanum) & stoptfl != 0 {
                (*route).stopevent = 2 as libc::c_int;
                (*route).endarea = (*reach).areanum;
                (*route).endcontents = (*crate::src::botlib::be_aas_main::aasworld
                    .areasettings
                    .offset((*reach).areanum as isize))
                .contents;
                (*route).endtravelflags = AAS_AreaContentsTravelFlags_inline((*reach).areanum);
                (*route).endpos[0 as libc::c_int as usize] =
                    (*reach).end[0 as libc::c_int as usize];
                (*route).endpos[1 as libc::c_int as usize] =
                    (*reach).end[1 as libc::c_int as usize];
                (*route).endpos[2 as libc::c_int as usize] =
                    (*reach).end[2 as libc::c_int as usize];
                (*route).time +=
                    AAS_AreaTravelTime(areanum, origin, (*reach).start.as_mut_ptr()) as libc::c_int;
                (*route).time += (*reach).traveltime as libc::c_int;
                return crate::src::qcommon::q_shared::qtrue as libc::c_int;
            }
            //end if
        } //end for
        reachareas = &mut *crate::src::botlib::be_aas_main::aasworld
            .reachabilityareas
            .offset(reachnum as isize)
            as *mut crate::be_aas_def_h::aas_reachabilityareas_t;
        j = 0 as libc::c_int;
        while j < (*reachareas).numareas + 1 as libc::c_int {
            if j >= (*reachareas).numareas {
                testareanum = (*reach).areanum
            } else {
                testareanum = *crate::src::botlib::be_aas_main::aasworld
                    .reachabilityareaindex
                    .offset(((*reachareas).firstarea + j) as isize)
            }
            //end if
            if stopevent & 4 as libc::c_int != 0 {
                if (*crate::src::botlib::be_aas_main::aasworld
                    .areasettings
                    .offset(testareanum as isize))
                .contents
                    & stopcontents
                    != 0
                {
                    (*route).stopevent = 4 as libc::c_int; //end if
                    (*route).endarea = testareanum;
                    (*route).endcontents = (*crate::src::botlib::be_aas_main::aasworld
                        .areasettings
                        .offset(testareanum as isize))
                    .contents;
                    (*route).endpos[0 as libc::c_int as usize] =
                        (*reach).end[0 as libc::c_int as usize];
                    (*route).endpos[1 as libc::c_int as usize] =
                        (*reach).end[1 as libc::c_int as usize];
                    (*route).endpos[2 as libc::c_int as usize] =
                        (*reach).end[2 as libc::c_int as usize];
                    (*route).time +=
                        AAS_AreaTravelTime(areanum, origin, (*reach).start.as_mut_ptr())
                            as libc::c_int;
                    (*route).time += (*reach).traveltime as libc::c_int;
                    return crate::src::qcommon::q_shared::qtrue as libc::c_int;
                }
                //end if
            }
            if stopevent & 8 as libc::c_int != 0 {
                if testareanum == stopareanum {
                    (*route).stopevent = 8 as libc::c_int;
                    (*route).endarea = testareanum;
                    (*route).endcontents = (*crate::src::botlib::be_aas_main::aasworld
                        .areasettings
                        .offset(testareanum as isize))
                    .contents;
                    (*route).endpos[0 as libc::c_int as usize] =
                        (*reach).start[0 as libc::c_int as usize];
                    (*route).endpos[1 as libc::c_int as usize] =
                        (*reach).start[1 as libc::c_int as usize];
                    (*route).endpos[2 as libc::c_int as usize] =
                        (*reach).start[2 as libc::c_int as usize];
                    return crate::src::qcommon::q_shared::qtrue as libc::c_int;
                }
                //end if
            }
            j += 1
        }
        (*route).time +=
            AAS_AreaTravelTime(areanum, origin, (*reach).start.as_mut_ptr()) as libc::c_int;
        (*route).time += (*reach).traveltime as libc::c_int;
        (*route).endarea = (*reach).areanum;
        (*route).endcontents = (*crate::src::botlib::be_aas_main::aasworld
            .areasettings
            .offset((*reach).areanum as isize))
        .contents;
        (*route).endtravelflags = AAS_TravelFlagForType_inline((*reach).traveltype);
        (*route).endpos[0 as libc::c_int as usize] = (*reach).end[0 as libc::c_int as usize];
        (*route).endpos[1 as libc::c_int as usize] = (*reach).end[1 as libc::c_int as usize];
        (*route).endpos[2 as libc::c_int as usize] = (*reach).end[2 as libc::c_int as usize];
        //
        curareanum = (*reach).areanum;
        curorigin[0 as libc::c_int as usize] = (*reach).end[0 as libc::c_int as usize];
        curorigin[1 as libc::c_int as usize] = (*reach).end[1 as libc::c_int as usize];
        curorigin[2 as libc::c_int as usize] = (*reach).end[2 as libc::c_int as usize];
        //
        if maxtime != 0 && (*route).time > maxtime {
            break;
        }
        i += 1
    }
    if curareanum != goalareanum {
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    }
    return crate::src::qcommon::q_shared::qtrue as libc::c_int;
}
//end of the function AAS_PredictRoute
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_BridgeWalkable(mut _areanum: libc::c_int) -> libc::c_int {
    return crate::src::qcommon::q_shared::qfalse as libc::c_int;
}
//end of the function AAS_BridgeWalkable
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_ReachabilityFromNum(
    mut num: libc::c_int,
    mut reach: *mut crate::aasfile_h::aas_reachability_s,
) {
    if crate::src::botlib::be_aas_main::aasworld.initialized == 0 {
        crate::stdlib::memset(
            reach as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<crate::aasfile_h::aas_reachability_t>() as libc::c_ulong,
        ); //end if
        return;
    } //end if
    if num < 0 as libc::c_int || num >= crate::src::botlib::be_aas_main::aasworld.reachabilitysize {
        crate::stdlib::memset(
            reach as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<crate::aasfile_h::aas_reachability_t>() as libc::c_ulong,
        );
        return;
    }
    crate::stdlib::memcpy(
        reach as *mut libc::c_void,
        &mut *crate::src::botlib::be_aas_main::aasworld
            .reachability
            .offset(num as isize) as *mut crate::aasfile_h::aas_reachability_t
            as *const libc::c_void,
        ::std::mem::size_of::<crate::aasfile_h::aas_reachability_t>() as libc::c_ulong,
    );
}
//end of the function AAS_ReachabilityFromNum
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_NextAreaReachability(
    mut areanum: libc::c_int,
    mut reachnum: libc::c_int,
) -> libc::c_int {
    let mut settings: *mut crate::aasfile_h::aas_areasettings_t =
        0 as *mut crate::aasfile_h::aas_areasettings_t; //end if
    if crate::src::botlib::be_aas_main::aasworld.initialized == 0 {
        return 0 as libc::c_int;
    } //end if
    if areanum <= 0 as libc::c_int || areanum >= crate::src::botlib::be_aas_main::aasworld.numareas
    {
        crate::src::botlib::be_interface::botimport
            .Print
            .expect("non-null function pointer")(
            3 as libc::c_int,
            b"AAS_NextAreaReachability: areanum %d out of range\n\x00" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            areanum,
        ); //end if
        return 0 as libc::c_int;
    } //end if
    settings = &mut *crate::src::botlib::be_aas_main::aasworld
        .areasettings
        .offset(areanum as isize) as *mut crate::aasfile_h::aas_areasettings_t;
    if reachnum == 0 {
        return (*settings).firstreachablearea;
    }
    if reachnum < (*settings).firstreachablearea {
        crate::src::botlib::be_interface::botimport
            .Print
            .expect("non-null function pointer")(
            4 as libc::c_int,
            b"AAS_NextAreaReachability: reachnum < settings->firstreachableara\x00" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        );
        return 0 as libc::c_int;
    }
    reachnum += 1;
    if reachnum >= (*settings).firstreachablearea + (*settings).numreachableareas {
        return 0 as libc::c_int;
    }
    return reachnum;
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
 * name:		be_aas_reach.h
 *
 * desc:		AAS
 *
 * $Archive: /source/code/botlib/be_aas_reach.h $
 *
 *****************************************************************************/
/* ****************************************************************************
 * name:		be_aas_reach.h
 *
 * desc:		AAS
 *
 * $Archive: /source/code/botlib/be_aas_reach.h $
 *
 *****************************************************************************/
//initialize calculating the reachabilities
//continue calculating the reachabilities
//
//AASINTERN
//AASINTERN
//returns true if the are has reachabilities to other areas
//returns true if the are has reachabilities to other areas
//returns the best reachable area and goal origin for a bounding box at the given origin
//returns the best reachable area and goal origin for a bounding box at the given origin
//returns the best jumppad area from which the bbox at origin is reachable
//returns the best jumppad area from which the bbox at origin is reachable
//returns the next reachability using the given model
//returns the next reachability using the given model
//end of the function AAS_NextAreaReachability
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_NextModelReachability(
    mut num: libc::c_int,
    mut modelnum: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    if num <= 0 as libc::c_int {
        num = 1 as libc::c_int
    } else if num >= crate::src::botlib::be_aas_main::aasworld.reachabilitysize {
        return 0 as libc::c_int;
    } else {
        num += 1
    }
    //
    i = num; //end for
    while i < crate::src::botlib::be_aas_main::aasworld.reachabilitysize {
        if (*crate::src::botlib::be_aas_main::aasworld
            .reachability
            .offset(i as isize))
        .traveltype
            & 0xffffff as libc::c_int
            == 11 as libc::c_int
        {
            if (*crate::src::botlib::be_aas_main::aasworld
                .reachability
                .offset(i as isize))
            .facenum
                == modelnum
            {
                return i;
            }
        } else if (*crate::src::botlib::be_aas_main::aasworld
            .reachability
            .offset(i as isize))
        .traveltype
            & 0xffffff as libc::c_int
            == 19 as libc::c_int
        {
            if (*crate::src::botlib::be_aas_main::aasworld
                .reachability
                .offset(i as isize))
            .facenum
                & 0xffff as libc::c_int
                == modelnum
            {
                return i;
            }
        }
        i += 1 //end if
               //end if
    }
    return 0 as libc::c_int;
}
//end of the function AAS_NextModelReachability
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_RandomGoalArea(
    mut areanum: libc::c_int,
    mut travelflags: libc::c_int,
    mut goalareanum: *mut libc::c_int,
    mut goalorigin: *mut crate::src::qcommon::q_shared::vec_t,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut start: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut end: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut trace: crate::be_aas_h::aas_trace_t = crate::be_aas_h::aas_trace_t {
        startsolid: crate::src::qcommon::q_shared::qfalse,
        fraction: 0.,
        endpos: [0.; 3],
        ent: 0,
        lastarea: 0,
        area: 0,
        planenum: 0,
    };
    //if the area has no reachabilities
    if crate::src::botlib::be_aas_reach::AAS_AreaReachability(areanum) == 0 {
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    }
    //
    n = (crate::src::botlib::be_aas_main::aasworld.numareas as libc::c_float
        * ((::libc::rand() & 0x7fff as libc::c_int) as libc::c_float
            / 0x7fff as libc::c_int as libc::c_float)) as libc::c_int; //end for
    i = 0 as libc::c_int; //end if
    while i < crate::src::botlib::be_aas_main::aasworld.numareas {
        if n <= 0 as libc::c_int {
            n = 1 as libc::c_int
        }
        if n >= crate::src::botlib::be_aas_main::aasworld.numareas {
            n = 1 as libc::c_int
        }
        if crate::src::botlib::be_aas_reach::AAS_AreaReachability(n) != 0 {
            t = AAS_AreaTravelTimeToGoalArea(
                areanum,
                (*crate::src::botlib::be_aas_main::aasworld
                    .areas
                    .offset(areanum as isize))
                .center
                .as_mut_ptr(),
                n,
                travelflags,
            );
            //end if
            if t > 0 as libc::c_int {
                //if the goal is reachable
                if crate::src::botlib::be_aas_reach::AAS_AreaSwim(n) != 0 {
                    *goalareanum = n; //end if
                    *goalorigin.offset(0 as libc::c_int as isize) =
                        (*crate::src::botlib::be_aas_main::aasworld
                            .areas
                            .offset(n as isize))
                        .center[0 as libc::c_int as usize];
                    *goalorigin.offset(1 as libc::c_int as isize) =
                        (*crate::src::botlib::be_aas_main::aasworld
                            .areas
                            .offset(n as isize))
                        .center[1 as libc::c_int as usize];
                    *goalorigin.offset(2 as libc::c_int as isize) =
                        (*crate::src::botlib::be_aas_main::aasworld
                            .areas
                            .offset(n as isize))
                        .center[2 as libc::c_int as usize];
                    //botimport.Print(PRT_MESSAGE, "found random goal area %d\n", *goalareanum);
                    return crate::src::qcommon::q_shared::qtrue as libc::c_int;
                }
                start[0 as libc::c_int as usize] = (*crate::src::botlib::be_aas_main::aasworld
                    .areas
                    .offset(n as isize))
                .center[0 as libc::c_int as usize];
                start[1 as libc::c_int as usize] = (*crate::src::botlib::be_aas_main::aasworld
                    .areas
                    .offset(n as isize))
                .center[1 as libc::c_int as usize];
                start[2 as libc::c_int as usize] = (*crate::src::botlib::be_aas_main::aasworld
                    .areas
                    .offset(n as isize))
                .center[2 as libc::c_int as usize];
                if crate::src::botlib::be_aas_sample::AAS_PointAreaNum(start.as_mut_ptr()) == 0 {
                    crate::src::botlib::l_log::Log_Write(
                        b"area %d center %f %f %f in solid?\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        n,
                        start[0 as libc::c_int as usize] as libc::c_double,
                        start[1 as libc::c_int as usize] as libc::c_double,
                        start[2 as libc::c_int as usize] as libc::c_double,
                    );
                }
                end[0 as libc::c_int as usize] = start[0 as libc::c_int as usize];
                end[1 as libc::c_int as usize] = start[1 as libc::c_int as usize];
                end[2 as libc::c_int as usize] = start[2 as libc::c_int as usize];
                end[2 as libc::c_int as usize] -= 300 as libc::c_int as libc::c_float;
                trace = crate::src::botlib::be_aas_sample::AAS_TraceClientBBox(
                    start.as_mut_ptr(),
                    end.as_mut_ptr(),
                    4 as libc::c_int,
                    -(1 as libc::c_int),
                ) as crate::be_aas_h::aas_trace_s;
                if trace.startsolid as u64 == 0
                    && trace.fraction < 1 as libc::c_int as libc::c_float
                    && crate::src::botlib::be_aas_sample::AAS_PointAreaNum(
                        trace.endpos.as_mut_ptr(),
                    ) == n
                {
                    if crate::src::botlib::be_aas_reach::AAS_AreaGroundFaceArea(n)
                        > 300 as libc::c_int as libc::c_float
                    {
                        *goalareanum = n;
                        *goalorigin.offset(0 as libc::c_int as isize) =
                            trace.endpos[0 as libc::c_int as usize];
                        *goalorigin.offset(1 as libc::c_int as isize) =
                            trace.endpos[1 as libc::c_int as usize];
                        *goalorigin.offset(2 as libc::c_int as isize) =
                            trace.endpos[2 as libc::c_int as usize];
                        //end if
                        //botimport.Print(PRT_MESSAGE, "found random goal area %d\n", *goalareanum);
                        return crate::src::qcommon::q_shared::qtrue as libc::c_int;
                    }
                    //end if
                }
            }
        }
        n += 1;
        i += 1
    }
    return crate::src::qcommon::q_shared::qfalse as libc::c_int;
}
//end of the function AAS_RandomGoalArea
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_AreaVisible(
    mut _srcarea: libc::c_int,
    mut _destarea: libc::c_int,
) -> libc::c_int {
    return crate::src::qcommon::q_shared::qfalse as libc::c_int;
}
//end of the function AAS_AreaVisible
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn DistancePointToLine(
    mut v1: *mut crate::src::qcommon::q_shared::vec_t,
    mut v2: *mut crate::src::qcommon::q_shared::vec_t,
    mut point: *mut crate::src::qcommon::q_shared::vec_t,
) -> libc::c_float {
    let mut vec: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut p2: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    crate::src::botlib::be_aas_main::AAS_ProjectPointOntoVector(point, v1, v2, p2.as_mut_ptr());
    vec[0 as libc::c_int as usize] =
        *point.offset(0 as libc::c_int as isize) - p2[0 as libc::c_int as usize];
    vec[1 as libc::c_int as usize] =
        *point.offset(1 as libc::c_int as isize) - p2[1 as libc::c_int as usize];
    vec[2 as libc::c_int as usize] =
        *point.offset(2 as libc::c_int as isize) - p2[2 as libc::c_int as usize];
    return VectorLength(vec.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t);
}
//end of the function DistancePointToLine
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_NearestHideArea(
    mut _srcnum: libc::c_int,
    mut origin: *mut crate::src::qcommon::q_shared::vec_t,
    mut areanum: libc::c_int,
    mut _enemynum: libc::c_int,
    mut enemyorigin: *mut crate::src::qcommon::q_shared::vec_t,
    mut enemyareanum: libc::c_int,
    mut travelflags: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut nextareanum: libc::c_int = 0;
    let mut badtravelflags: libc::c_int = 0;
    let mut numreach: libc::c_int = 0;
    let mut bestarea: libc::c_int = 0;
    let mut t: libc::c_ushort = 0;
    let mut besttraveltime: libc::c_ushort = 0;
    static mut hidetraveltimes: *mut libc::c_ushort =
        0 as *const libc::c_ushort as *mut libc::c_ushort;
    let mut updateliststart: *mut crate::be_aas_def_h::aas_routingupdate_t =
        0 as *mut crate::be_aas_def_h::aas_routingupdate_t;
    let mut updatelistend: *mut crate::be_aas_def_h::aas_routingupdate_t =
        0 as *mut crate::be_aas_def_h::aas_routingupdate_t;
    let mut curupdate: *mut crate::be_aas_def_h::aas_routingupdate_t =
        0 as *mut crate::be_aas_def_h::aas_routingupdate_t;
    let mut nextupdate: *mut crate::be_aas_def_h::aas_routingupdate_t =
        0 as *mut crate::be_aas_def_h::aas_routingupdate_t;
    let mut reach: *mut crate::aasfile_h::aas_reachability_t =
        0 as *mut crate::aasfile_h::aas_reachability_t;
    let mut dist1: libc::c_float = 0.;
    let mut dist2: libc::c_float = 0.;
    let mut v1: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut v2: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut p: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut startVisible: crate::src::qcommon::q_shared::qboolean =
        crate::src::qcommon::q_shared::qfalse;
    //
    if hidetraveltimes.is_null() {
        //end else
        hidetraveltimes = crate::src::botlib::l_memory::GetClearedMemory(
            (crate::src::botlib::be_aas_main::aasworld.numareas as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_ushort>() as libc::c_ulong),
        ) as *mut libc::c_ushort
    } else {
        crate::stdlib::memset(
            hidetraveltimes as *mut libc::c_void,
            0 as libc::c_int,
            (crate::src::botlib::be_aas_main::aasworld.numareas as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_ushort>() as libc::c_ulong),
        ); //end if
    }
    besttraveltime = 0 as libc::c_int as libc::c_ushort;
    bestarea = 0 as libc::c_int;
    //assume visible
    startVisible = crate::src::qcommon::q_shared::qtrue;
    //
    badtravelflags = !travelflags;
    //
    curupdate = &mut *crate::src::botlib::be_aas_main::aasworld
        .areaupdate
        .offset(areanum as isize) as *mut crate::be_aas_def_h::aas_routingupdate_t;
    (*curupdate).areanum = areanum;
    (*curupdate).start[0 as libc::c_int as usize] = *origin.offset(0 as libc::c_int as isize);
    (*curupdate).start[1 as libc::c_int as usize] = *origin.offset(1 as libc::c_int as isize);
    (*curupdate).start[2 as libc::c_int as usize] = *origin.offset(2 as libc::c_int as isize);
    (*curupdate).areatraveltimes = *(*crate::src::botlib::be_aas_main::aasworld
        .areatraveltimes
        .offset(areanum as isize))
    .offset(0 as libc::c_int as isize);
    (*curupdate).tmptraveltime = 0 as libc::c_int as libc::c_ushort;
    //put the area to start with in the current read list
    (*curupdate).next = 0 as *mut crate::be_aas_def_h::aas_routingupdate_s;
    (*curupdate).prev = 0 as *mut crate::be_aas_def_h::aas_routingupdate_s;
    updateliststart = curupdate;
    updatelistend = curupdate;
    //while there are updates in the list
    while !updateliststart.is_null() {
        curupdate = updateliststart; //end while
                                     //end for
        if !(*curupdate).next.is_null() {
            (*(*curupdate).next).prev = 0 as *mut crate::be_aas_def_h::aas_routingupdate_s
        } else {
            updatelistend = 0 as *mut crate::be_aas_def_h::aas_routingupdate_t
        }
        updateliststart = (*curupdate).next;
        (*curupdate).inlist = crate::src::qcommon::q_shared::qfalse;
        numreach = (*crate::src::botlib::be_aas_main::aasworld
            .areasettings
            .offset((*curupdate).areanum as isize))
        .numreachableareas;
        reach = &mut *crate::src::botlib::be_aas_main::aasworld
            .reachability
            .offset(
                (*crate::src::botlib::be_aas_main::aasworld
                    .areasettings
                    .offset((*curupdate).areanum as isize))
                .firstreachablearea as isize,
            ) as *mut crate::aasfile_h::aas_reachability_t;
        i = 0 as libc::c_int;
        while i < numreach {
            //
            //
            //check all reversed reachability links
            //
            //if an undesired travel type is used
            if !(AAS_TravelFlagForType_inline((*reach).traveltype) & badtravelflags != 0) {
                //
                if !(AAS_AreaContentsTravelFlags_inline((*reach).areanum) & badtravelflags != 0) {
                    //number of the area the reachability leads to
                    nextareanum = (*reach).areanum;
                    // if this moves us into the enemies area, skip it
                    if !(nextareanum == enemyareanum) {
                        //time already travelled plus the traveltime through
                        //the current area plus the travel time from the reachability
                        t = ((*curupdate).tmptraveltime as libc::c_int
                            + AAS_AreaTravelTime(
                                (*curupdate).areanum,
                                (*curupdate).start.as_mut_ptr(),
                                (*reach).start.as_mut_ptr(),
                            ) as libc::c_int
                            + (*reach).traveltime as libc::c_int)
                            as libc::c_ushort;
                        //avoid going near the enemy
                        crate::src::botlib::be_aas_main::AAS_ProjectPointOntoVector(
                            enemyorigin,
                            (*curupdate).start.as_mut_ptr(),
                            (*reach).end.as_mut_ptr(),
                            p.as_mut_ptr(),
                        ); //end else
                        j = 0 as libc::c_int; //end if
                        while j < 3 as libc::c_int {
                            if p[j as usize] > (*curupdate).start[j as usize]
                                && p[j as usize] > (*reach).end[j as usize]
                                || p[j as usize] < (*curupdate).start[j as usize]
                                    && p[j as usize] < (*reach).end[j as usize]
                            {
                                break;
                            }
                            j += 1
                        }
                        if j < 3 as libc::c_int {
                            v2[0 as libc::c_int as usize] = *enemyorigin
                                .offset(0 as libc::c_int as isize)
                                - (*reach).end[0 as libc::c_int as usize];
                            v2[1 as libc::c_int as usize] = *enemyorigin
                                .offset(1 as libc::c_int as isize)
                                - (*reach).end[1 as libc::c_int as usize];
                            v2[2 as libc::c_int as usize] = *enemyorigin
                                .offset(2 as libc::c_int as isize)
                                - (*reach).end[2 as libc::c_int as usize]
                        } else {
                            v2[0 as libc::c_int as usize] = *enemyorigin
                                .offset(0 as libc::c_int as isize)
                                - p[0 as libc::c_int as usize];
                            v2[1 as libc::c_int as usize] = *enemyorigin
                                .offset(1 as libc::c_int as isize)
                                - p[1 as libc::c_int as usize];
                            v2[2 as libc::c_int as usize] = *enemyorigin
                                .offset(2 as libc::c_int as isize)
                                - p[2 as libc::c_int as usize]
                        }
                        dist2 = VectorLength(
                            v2.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t
                        );
                        //never go through the enemy
                        if !(dist2 < 40 as libc::c_int as libc::c_float) {
                            //
                            v1[0 as libc::c_int as usize] = *enemyorigin
                                .offset(0 as libc::c_int as isize)
                                - (*curupdate).start[0 as libc::c_int as usize];
                            v1[1 as libc::c_int as usize] = *enemyorigin
                                .offset(1 as libc::c_int as isize)
                                - (*curupdate).start[1 as libc::c_int as usize];
                            v1[2 as libc::c_int as usize] = *enemyorigin
                                .offset(2 as libc::c_int as isize)
                                - (*curupdate).start[2 as libc::c_int as usize];
                            dist1 = VectorLength(
                                v1.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t
                            );
                            //
                            if dist2 < dist1 {
                                t = (t as libc::c_float
                                    + (dist1 - dist2) * 10 as libc::c_int as libc::c_float)
                                    as libc::c_ushort
                            }
                            // if we weren't visible when starting, make sure we don't move into their view
                            if !(startVisible as u64 == 0
                                && AAS_AreaVisible(enemyareanum, nextareanum) != 0)
                            {
                                //
                                if !(besttraveltime as libc::c_int != 0
                                    && t as libc::c_int >= besttraveltime as libc::c_int)
                                {
                                    //
                                    if *hidetraveltimes.offset(nextareanum as isize) == 0
                                        || *hidetraveltimes.offset(nextareanum as isize)
                                            as libc::c_int
                                            > t as libc::c_int
                                    {
                                        //if the nextarea is not visible from the enemy area
                                        if AAS_AreaVisible(enemyareanum, nextareanum) == 0 {
                                            besttraveltime = t; //end if
                                            bestarea = nextareanum
                                        }
                                        *hidetraveltimes.offset(nextareanum as isize) = t;
                                        nextupdate = &mut *crate::src::botlib::be_aas_main::aasworld
                                            .areaupdate
                                            .offset(nextareanum as isize)
                                            as *mut crate::be_aas_def_h::aas_routingupdate_t;
                                        (*nextupdate).areanum = nextareanum;
                                        (*nextupdate).tmptraveltime = t;
                                        //end if
                                        (*nextupdate).start[0 as libc::c_int as usize] =
                                            (*reach).end[0 as libc::c_int as usize];
                                        (*nextupdate).start[1 as libc::c_int as usize] =
                                            (*reach).end[1 as libc::c_int as usize];
                                        (*nextupdate).start[2 as libc::c_int as usize] =
                                            (*reach).end[2 as libc::c_int as usize];
                                        if (*nextupdate).inlist as u64 == 0 {
                                            //remember where we entered this area
                                            //if this update is not in the list yet
                                            //add the new update to the end of the list
                                            (*nextupdate).next =
                                                0 as *mut crate::be_aas_def_h::aas_routingupdate_s;
                                            (*nextupdate).prev = updatelistend;
                                            if !updatelistend.is_null() {
                                                (*updatelistend).next = nextupdate
                                            } else {
                                                updateliststart = nextupdate
                                            }
                                            updatelistend = nextupdate;
                                            (*nextupdate).inlist =
                                                crate::src::qcommon::q_shared::qtrue
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            i += 1;
            reach = reach.offset(1)
            //end if
        }
    }
    return bestarea;
}
//end of the function AAS_NearestHideArea

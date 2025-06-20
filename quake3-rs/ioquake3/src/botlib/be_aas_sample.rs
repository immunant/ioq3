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

    pub unsafe extern "C" fn VectorInverse(mut v: *mut crate::src::qcommon::q_shared::vec_t) {
        *v.offset(0 as i32 as isize) = -*v.offset(0 as i32 as isize);
        *v.offset(1 as i32 as isize) = -*v.offset(1 as i32 as isize);
        *v.offset(2 as i32 as isize) = -*v.offset(2 as i32 as isize);
    }
    #[inline]

    pub unsafe extern "C" fn CrossProduct(
        mut v1: *const crate::src::qcommon::q_shared::vec_t,
        mut v2: *const crate::src::qcommon::q_shared::vec_t,
        mut cross: *mut crate::src::qcommon::q_shared::vec_t,
    ) {
        *cross.offset(0 as i32 as isize) = *v1.offset(1 as i32 as isize)
            * *v2.offset(2 as i32 as isize)
            - *v1.offset(2 as i32 as isize) * *v2.offset(1 as i32 as isize);
        *cross.offset(1 as i32 as isize) = *v1.offset(2 as i32 as isize)
            * *v2.offset(0 as i32 as isize)
            - *v1.offset(0 as i32 as isize) * *v2.offset(2 as i32 as isize);
        *cross.offset(2 as i32 as isize) = *v1.offset(0 as i32 as isize)
            * *v2.offset(1 as i32 as isize)
            - *v1.offset(1 as i32 as isize) * *v2.offset(0 as i32 as isize);
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
pub use crate::be_aas_h::aas_areainfo_s;
pub use crate::be_aas_h::aas_areainfo_t;
pub use crate::be_aas_h::aas_entityinfo_s;
pub use crate::be_aas_h::aas_entityinfo_t;
pub use crate::be_aas_h::aas_trace_s;
pub use crate::be_aas_h::aas_trace_t;
pub use crate::botlib_h::botlib_import_s;
pub use crate::botlib_h::botlib_import_t;
pub use crate::botlib_h::bsp_surface_s;
pub use crate::botlib_h::bsp_surface_t;
pub use crate::botlib_h::bsp_trace_s;
pub use crate::botlib_h::bsp_trace_t;

pub use crate::src::botlib::be_aas_sample::q_shared_h::CrossProduct;
pub use crate::src::botlib::be_aas_sample::q_shared_h::VectorInverse;
pub use crate::src::botlib::be_aas_sample::q_shared_h::VectorLength;

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
pub use crate::src::qcommon::q_shared::FS_APPEND;
pub use crate::src::qcommon::q_shared::FS_APPEND_SYNC;
pub use crate::src::qcommon::q_shared::FS_READ;
pub use crate::src::qcommon::q_shared::FS_WRITE;

pub type aas_tracestack_t = aas_tracestack_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aas_tracestack_s {
    pub start: vec3_t,
    pub end: vec3_t,
    pub planenum: i32,
    pub nodenum: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aas_linkstack_t {
    pub nodenum: i32,
}
#[no_mangle]

pub static mut numaaslinks: i32 = 0;
//start point of the piece of line to trace
//end point of the piece of line to trace
//last plane used as splitter
//node found after splitting with planenum
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_PresenceTypeBoundingBox(
    mut presencetype: i32,
    mut mins: *mut vec_t,
    mut maxs: *mut vec_t,
) {
    let mut index: i32 = 0;
    //bounding box size for each presence type
    let mut boxmins: [vec3_t; 3] = [
        [0 as i32 as vec_t, 0 as i32 as vec_t, 0 as i32 as vec_t],
        [
            -(15 as i32) as vec_t,
            -(15 as i32) as vec_t,
            -(24 as i32) as vec_t,
        ],
        [
            -(15 as i32) as vec_t,
            -(15 as i32) as vec_t,
            -(24 as i32) as vec_t,
        ],
    ]; //end if
    let mut boxmaxs: [vec3_t; 3] = [
        [0 as i32 as vec_t, 0 as i32 as vec_t, 0 as i32 as vec_t],
        [15 as i32 as vec_t, 15 as i32 as vec_t, 32 as i32 as vec_t],
        [15 as i32 as vec_t, 15 as i32 as vec_t, 8 as i32 as vec_t],
    ];
    if presencetype == 2 as i32 {
        index = 1 as i32
    } else if presencetype == 4 as i32 {
        index = 2 as i32
    } else {
        crate::src::botlib::be_interface::botimport
            .Print
            .expect("non-null function pointer")(
            4 as i32,
            b"AAS_PresenceTypeBoundingBox: unknown presence type\n\x00" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        );
        index = 2 as i32
    }
    *mins.offset(0 as i32 as isize) = boxmins[index as usize][0 as i32 as usize];
    *mins.offset(1 as i32 as isize) = boxmins[index as usize][1 as i32 as usize];
    *mins.offset(2 as i32 as isize) = boxmins[index as usize][2 as i32 as usize];
    *maxs.offset(0 as i32 as isize) = boxmaxs[index as usize][0 as i32 as usize];
    *maxs.offset(1 as i32 as isize) = boxmaxs[index as usize][1 as i32 as usize];
    *maxs.offset(2 as i32 as isize) = boxmaxs[index as usize][2 as i32 as usize];
}
//end of the function AAS_PresenceTypeBoundingBox
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_InitAASLinkHeap() {
    let mut i: i32 = 0;
    let mut max_aaslinks: i32 = 0;
    max_aaslinks = crate::src::botlib::be_aas_main::aasworld.linkheapsize;
    //if there's no link heap present
    if crate::src::botlib::be_aas_main::aasworld.linkheap.is_null() {
        max_aaslinks = crate::src::botlib::l_libvar::LibVarValue(
            b"max_aaslinks\x00" as *const u8 as *const libc::c_char,
            b"6144\x00" as *const u8 as *const libc::c_char,
        ) as i32; //end if
        if max_aaslinks < 0 as i32 {
            max_aaslinks = 0 as i32
        }
        crate::src::botlib::be_aas_main::aasworld.linkheapsize = max_aaslinks;
        crate::src::botlib::be_aas_main::aasworld.linkheap =
            crate::src::botlib::l_memory::GetHunkMemory(
                (max_aaslinks as usize).wrapping_mul(::std::mem::size_of::<aas_link_t>() as usize),
            ) as *mut aas_link_t
    }
    //link the links on the heap
    let ref mut fresh0 = (*crate::src::botlib::be_aas_main::aasworld
        .linkheap
        .offset(0 as i32 as isize))
    .prev_ent; //end for
    *fresh0 = std::ptr::null_mut();
    let ref mut fresh1 = (*crate::src::botlib::be_aas_main::aasworld
        .linkheap
        .offset(0 as i32 as isize))
    .next_ent;
    *fresh1 = &mut *crate::src::botlib::be_aas_main::aasworld
        .linkheap
        .offset(1 as i32 as isize) as *mut aas_link_t;
    i = 1 as i32;
    while i < max_aaslinks - 1 as i32 {
        let ref mut fresh2 = (*crate::src::botlib::be_aas_main::aasworld
            .linkheap
            .offset(i as isize))
        .prev_ent;
        *fresh2 = &mut *crate::src::botlib::be_aas_main::aasworld
            .linkheap
            .offset((i - 1 as i32) as isize) as *mut aas_link_t;
        let ref mut fresh3 = (*crate::src::botlib::be_aas_main::aasworld
            .linkheap
            .offset(i as isize))
        .next_ent;
        *fresh3 = &mut *crate::src::botlib::be_aas_main::aasworld
            .linkheap
            .offset((i + 1 as i32) as isize) as *mut aas_link_t;
        i += 1
    }
    let ref mut fresh4 = (*crate::src::botlib::be_aas_main::aasworld
        .linkheap
        .offset((max_aaslinks - 1 as i32) as isize))
    .prev_ent;
    *fresh4 = &mut *crate::src::botlib::be_aas_main::aasworld
        .linkheap
        .offset((max_aaslinks - 2 as i32) as isize) as *mut aas_link_t;
    let ref mut fresh5 = (*crate::src::botlib::be_aas_main::aasworld
        .linkheap
        .offset((max_aaslinks - 1 as i32) as isize))
    .next_ent;
    *fresh5 = std::ptr::null_mut();
    //pointer to the first free link
    crate::src::botlib::be_aas_main::aasworld.freelinks =
        &mut *crate::src::botlib::be_aas_main::aasworld
            .linkheap
            .offset(0 as i32 as isize) as *mut aas_link_t;
    //
    numaaslinks = max_aaslinks;
}
//end of the function AAS_InitAASLinkHeap
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_FreeAASLinkHeap() {
    if !crate::src::botlib::be_aas_main::aasworld.linkheap.is_null() {
        crate::src::botlib::l_memory::FreeMemory(
            crate::src::botlib::be_aas_main::aasworld.linkheap as *mut libc::c_void,
        );
    }
    crate::src::botlib::be_aas_main::aasworld.linkheap = std::ptr::null_mut();
    crate::src::botlib::be_aas_main::aasworld.linkheapsize = 0 as i32;
}
//end of the function AAS_FreeAASLinkHeap
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_AllocAASLink() -> *mut aas_link_t {
    let mut link: *mut aas_link_t = std::ptr::null_mut(); //end if
    link = crate::src::botlib::be_aas_main::aasworld.freelinks; //end if
    if link.is_null() {
        if crate::src::botlib::be_interface::botDeveloper != 0 {
            crate::src::botlib::be_interface::botimport
                .Print
                .expect("non-null function pointer")(
                4 as i32,
                b"empty aas link heap\n\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
        return std::ptr::null_mut();
    }
    if !crate::src::botlib::be_aas_main::aasworld
        .freelinks
        .is_null()
    {
        crate::src::botlib::be_aas_main::aasworld.freelinks =
            (*crate::src::botlib::be_aas_main::aasworld.freelinks).next_ent
    }
    if !crate::src::botlib::be_aas_main::aasworld
        .freelinks
        .is_null()
    {
        (*crate::src::botlib::be_aas_main::aasworld.freelinks).prev_ent = std::ptr::null_mut()
    }
    numaaslinks -= 1;
    return link;
}
//end of the function AAS_AllocAASLink
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_DeAllocAASLink(mut link: *mut aas_link_t) {
    if !crate::src::botlib::be_aas_main::aasworld
        .freelinks
        .is_null()
    {
        (*crate::src::botlib::be_aas_main::aasworld.freelinks).prev_ent = link
    }
    (*link).prev_ent = std::ptr::null_mut();
    (*link).next_ent = crate::src::botlib::be_aas_main::aasworld.freelinks;
    (*link).prev_area = std::ptr::null_mut();
    (*link).next_area = std::ptr::null_mut();
    crate::src::botlib::be_aas_main::aasworld.freelinks = link;
    numaaslinks += 1;
}
//end of the function AAS_DeAllocAASLink
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_InitAASLinkedEntities() {
    if crate::src::botlib::be_aas_main::aasworld.loaded == 0 {
        return;
    }
    if !crate::src::botlib::be_aas_main::aasworld
        .arealinkedentities
        .is_null()
    {
        crate::src::botlib::l_memory::FreeMemory(
            crate::src::botlib::be_aas_main::aasworld.arealinkedentities as *mut libc::c_void,
        );
    }
    crate::src::botlib::be_aas_main::aasworld.arealinkedentities =
        crate::src::botlib::l_memory::GetClearedHunkMemory(
            (crate::src::botlib::be_aas_main::aasworld.numareas as usize)
                .wrapping_mul(::std::mem::size_of::<*mut aas_link_t>() as usize),
        ) as *mut *mut aas_link_t;
}
//end of the function AAS_InitAASLinkedEntities
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_FreeAASLinkedEntities() {
    if !crate::src::botlib::be_aas_main::aasworld
        .arealinkedentities
        .is_null()
    {
        crate::src::botlib::l_memory::FreeMemory(
            crate::src::botlib::be_aas_main::aasworld.arealinkedentities as *mut libc::c_void,
        );
    }
    crate::src::botlib::be_aas_main::aasworld.arealinkedentities = std::ptr::null_mut();
}
//end of the function AAS_InitAASLinkedEntities
//===========================================================================
// returns the AAS area the point is in
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_PointAreaNum(mut point: *mut vec_t) -> i32 {
    let mut nodenum: i32 = 0; //end if
    let mut dist: vec_t = 0.;
    let mut node: *mut aas_node_t = std::ptr::null_mut();
    let mut plane: *mut aas_plane_t = std::ptr::null_mut();
    if crate::src::botlib::be_aas_main::aasworld.loaded == 0 {
        crate::src::botlib::be_interface::botimport
            .Print
            .expect("non-null function pointer")(
            3 as i32,
            b"AAS_PointAreaNum: aas not loaded\n\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return 0 as i32;
    }
    //start with node 1 because node zero is a dummy used for solid leafs
    nodenum = 1 as i32; //end while
    while nodenum > 0 as i32 {
        //		botimport.Print(PRT_MESSAGE, "[%d]", nodenum);
        //AAS_SAMPLE_DEBUG
        node = &mut *crate::src::botlib::be_aas_main::aasworld
            .nodes
            .offset(nodenum as isize) as *mut aas_node_t;
        //AAS_SAMPLE_DEBUG
        plane = &mut *crate::src::botlib::be_aas_main::aasworld
            .planes
            .offset((*node).planenum as isize) as *mut aas_plane_t; //end if
        dist = *point.offset(0 as i32 as isize) * (*plane).normal[0 as i32 as usize]
            + *point.offset(1 as i32 as isize) * (*plane).normal[1 as i32 as usize]
            + *point.offset(2 as i32 as isize) * (*plane).normal[2 as i32 as usize]
            - (*plane).dist;
        if dist > 0 as i32 as f32 {
            nodenum = (*node).children[0 as i32 as usize]
        } else {
            nodenum = (*node).children[1 as i32 as usize]
        }
    }
    if nodenum == 0 {
        //AAS_SAMPLE_DEBUG
        return 0 as i32;
    }
    return -nodenum;
}
//end of the function AAS_PointAreaNum
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_PointReachabilityAreaIndex(mut origin: *mut vec_t) -> i32 {
    let mut areanum: i32 = 0; //end if
    let mut cluster: i32 = 0; //end for
    let mut i: i32 = 0; //end if
    let mut index: i32 = 0; //end for
    if crate::src::botlib::be_aas_main::aasworld.initialized == 0 {
        return 0 as i32;
    }
    if origin.is_null() {
        index = 0 as i32;
        i = 0 as i32;
        while i < crate::src::botlib::be_aas_main::aasworld.numclusters {
            index += (*crate::src::botlib::be_aas_main::aasworld
                .clusters
                .offset(i as isize))
            .numreachabilityareas;
            i += 1
        }
        return index;
    }
    areanum = AAS_PointAreaNum(origin);
    if areanum == 0 || crate::src::botlib::be_aas_reach::AAS_AreaReachability(areanum) == 0 {
        return 0 as i32;
    }
    cluster = (*crate::src::botlib::be_aas_main::aasworld
        .areasettings
        .offset(areanum as isize))
    .cluster;
    areanum = (*crate::src::botlib::be_aas_main::aasworld
        .areasettings
        .offset(areanum as isize))
    .clusterareanum;
    if cluster < 0 as i32 {
        cluster = (*crate::src::botlib::be_aas_main::aasworld
            .portals
            .offset(-cluster as isize))
        .frontcluster;
        areanum = (*crate::src::botlib::be_aas_main::aasworld
            .portals
            .offset(-cluster as isize))
        .clusterareanum[0 as i32 as usize]
    }
    index = 0 as i32;
    i = 0 as i32;
    while i < cluster {
        index += (*crate::src::botlib::be_aas_main::aasworld
            .clusters
            .offset(i as isize))
        .numreachabilityareas;
        i += 1
    }
    index += areanum;
    return index;
}
//end of the function AAS_PointReachabilityAreaIndex
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_AreaCluster(mut areanum: i32) -> i32 {
    if areanum <= 0 as i32 || areanum >= crate::src::botlib::be_aas_main::aasworld.numareas {
        crate::src::botlib::be_interface::botimport
            .Print
            .expect("non-null function pointer")(
            3 as i32,
            b"AAS_AreaCluster: invalid area number\n\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        ); //end if
        return 0 as i32;
    }
    return (*crate::src::botlib::be_aas_main::aasworld
        .areasettings
        .offset(areanum as isize))
    .cluster;
}
//end of the function AAS_AreaCluster
//===========================================================================
// returns the presence types of the given area
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_AreaPresenceType(mut areanum: i32) -> i32 {
    if crate::src::botlib::be_aas_main::aasworld.loaded == 0 {
        return 0 as i32;
    } //end if
    if areanum <= 0 as i32 || areanum >= crate::src::botlib::be_aas_main::aasworld.numareas {
        crate::src::botlib::be_interface::botimport
            .Print
            .expect("non-null function pointer")(
            3 as i32,
            b"AAS_AreaPresenceType: invalid area number\n\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return 0 as i32;
    }
    return (*crate::src::botlib::be_aas_main::aasworld
        .areasettings
        .offset(areanum as isize))
    .presencetype;
}
//end of the function AAS_AreaPresenceType
//===========================================================================
// returns the presence type at the given point
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_PointPresenceType(mut point: *mut vec_t) -> i32 {
    let mut areanum: i32 = 0;
    if crate::src::botlib::be_aas_main::aasworld.loaded == 0 {
        return 0 as i32;
    }
    areanum = AAS_PointAreaNum(point);
    if areanum == 0 {
        return 1 as i32;
    }
    return (*crate::src::botlib::be_aas_main::aasworld
        .areasettings
        .offset(areanum as isize))
    .presencetype;
}
//end of the function AAS_PointPresenceType
//===========================================================================
// calculates the minimum distance between the origin of the box and the
// given plane when both will collide on the given side of the plane
//
// normal	=	normal vector of plane to calculate distance from
// mins		=	minimums of box relative to origin
// maxs		=	maximums of box relative to origin
// side		=	side of the plane we want to calculate the distance from
//					0 normal vector side
//					1 not normal vector side
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_BoxOriginDistanceFromPlane(
    mut normal: *mut vec_t,
    mut mins: *mut vec_t,
    mut maxs: *mut vec_t,
    mut side: i32,
) -> vec_t {
    let mut v1: vec3_t = [0.; 3];
    let mut v2: vec3_t = [0.; 3];
    let mut i: i32 = 0;
    //swap maxs and mins when on the other side of the plane
    if side != 0 {
        //end else
        //get a point of the box that would be one of the first
        //to collide with the plane
        i = 0 as i32;
        while i < 3 as i32 {
            if *normal.offset(i as isize) as f64 > 0.001f64 {
                v1[i as usize] = *maxs.offset(i as isize)
            } else if (*normal.offset(i as isize) as f64) < -0.001f64 {
                v1[i as usize] = *mins.offset(i as isize)
            } else {
                v1[i as usize] = 0 as i32 as vec_t
            }
            i += 1
        }
    //end for
    } else {
        //get a point of the box that would be one of the first
        //to collide with the plane
        i = 0 as i32;
        while i < 3 as i32 {
            if *normal.offset(i as isize) as f64 > 0.001f64 {
                v1[i as usize] = *mins.offset(i as isize)
            } else if (*normal.offset(i as isize) as f64) < -0.001f64 {
                v1[i as usize] = *maxs.offset(i as isize)
            } else {
                v1[i as usize] = 0 as i32 as vec_t
            }
            i += 1
        }
        //end for
    }
    //
    v2[0 as i32 as usize] = *normal.offset(0 as i32 as isize);
    v2[1 as i32 as usize] = *normal.offset(1 as i32 as isize);
    v2[2 as i32 as usize] = *normal.offset(2 as i32 as isize);
    VectorInverse(v2.as_mut_ptr());
    //	VectorNegate(normal, v2);
    return v1[0 as i32 as usize] * v2[0 as i32 as usize]
        + v1[1 as i32 as usize] * v2[1 as i32 as usize]
        + v1[2 as i32 as usize] * v2[2 as i32 as usize];
}
//end of the function AAS_BoxOriginDistanceFromPlane
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_AreaEntityCollision(
    mut areanum: i32,
    mut start: *mut vec_t,
    mut end: *mut vec_t,
    mut presencetype: i32,
    mut passent: i32,
    mut trace: *mut aas_trace_t,
) -> qboolean {
    let mut collision: i32 = 0; //make compiler happy
    let mut boxmins: vec3_t = [0.; 3];
    let mut boxmaxs: vec3_t = [0.; 3];
    let mut link: *mut aas_link_t = std::ptr::null_mut();
    let mut bsptrace: bsp_trace_t = bsp_trace_t {
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
    AAS_PresenceTypeBoundingBox(presencetype, boxmins.as_mut_ptr(), boxmaxs.as_mut_ptr());
    crate::stdlib::memset(
        &mut bsptrace as *mut bsp_trace_t as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<bsp_trace_t>() as usize,
    );
    //assume no collision
    bsptrace.fraction = 1 as i32 as f32; //end for
    collision = qfalse as i32;
    link = *crate::src::botlib::be_aas_main::aasworld
        .arealinkedentities
        .offset(areanum as isize);
    while !link.is_null() {
        //ignore the pass entity
        if !((*link).entnum == passent) {
            //
            if crate::src::botlib::be_aas_bspq3::AAS_EntityCollision(
                (*link).entnum,
                start,
                boxmins.as_mut_ptr(),
                boxmaxs.as_mut_ptr(),
                end,
                1 as i32 | 0x10000 as i32,
                &mut bsptrace as *mut _ as *mut bsp_trace_s,
            ) as u64
                != 0
            {
                collision = qtrue as i32
            }
        }
        link = (*link).next_ent
        //end if
    } //end if
    if collision != 0 {
        (*trace).startsolid = bsptrace.startsolid;
        (*trace).ent = bsptrace.ent;
        (*trace).endpos[0 as i32 as usize] = bsptrace.endpos[0 as i32 as usize];
        (*trace).endpos[1 as i32 as usize] = bsptrace.endpos[1 as i32 as usize];
        (*trace).endpos[2 as i32 as usize] = bsptrace.endpos[2 as i32 as usize];
        (*trace).area = 0 as i32;
        (*trace).planenum = 0 as i32;
        return qtrue;
    }
    return qfalse;
}
//end of the function AAS_AreaEntityCollision
//===========================================================================
// recursive subdivision of the line by the BSP tree.
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_TraceClientBBox(
    mut start: *mut vec_t,
    mut end: *mut vec_t,
    mut presencetype: i32,
    mut passent: i32,
) -> aas_trace_t {
    let mut side: i32 = 0;
    let mut nodenum: i32 = 0;
    let mut tmpplanenum: i32 = 0;
    let mut front: f32 = 0.;
    let mut back: f32 = 0.;
    let mut frac: f32 = 0.;
    let mut cur_start: vec3_t = [0.; 3];
    let mut cur_end: vec3_t = [0.; 3];
    let mut cur_mid: vec3_t = [0.; 3];
    let mut v1: vec3_t = [0.; 3];
    let mut v2: vec3_t = [0.; 3];
    let mut tracestack: [aas_tracestack_t; 127] = [aas_tracestack_t {
        start: [0.; 3],
        end: [0.; 3],
        planenum: 0,
        nodenum: 0,
    }; 127];
    let mut tstack_p: *mut aas_tracestack_t = std::ptr::null_mut();
    let mut aasnode: *mut aas_node_t = std::ptr::null_mut();
    let mut plane: *mut aas_plane_t = std::ptr::null_mut();
    let mut trace: aas_trace_t = aas_trace_t {
        startsolid: qfalse,
        fraction: 0.,
        endpos: [0.; 3],
        ent: 0,
        lastarea: 0,
        area: 0,
        planenum: 0,
    };
    //clear the trace structure
    crate::stdlib::memset(
        &mut trace as *mut aas_trace_t as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<aas_trace_t>() as usize,
    );
    if crate::src::botlib::be_aas_main::aasworld.loaded == 0 {
        return trace;
    }
    tstack_p = tracestack.as_mut_ptr();
    //we start with the whole line on the stack
    (*tstack_p).start[0 as i32 as usize] = *start.offset(0 as i32 as isize);
    (*tstack_p).start[1 as i32 as usize] = *start.offset(1 as i32 as isize);
    (*tstack_p).start[2 as i32 as usize] = *start.offset(2 as i32 as isize);
    (*tstack_p).end[0 as i32 as usize] = *end.offset(0 as i32 as isize);
    (*tstack_p).end[1 as i32 as usize] = *end.offset(1 as i32 as isize);
    (*tstack_p).end[2 as i32 as usize] = *end.offset(2 as i32 as isize);
    (*tstack_p).planenum = 0 as i32;
    //start with node 1 because node zero is a dummy for a solid leaf
    (*tstack_p).nodenum = 1 as i32; //starting at the root of the tree
    tstack_p = tstack_p.offset(1);
    loop {
        //pop up the stack
        tstack_p = tstack_p.offset(-1);
        //end else
        //if the trace stack is empty (ended up with a piece of the
        //line to be traced in an area)
        if tstack_p < tracestack.as_mut_ptr() {
            tstack_p = tstack_p.offset(1); //end if
                                           //nothing was hit
            trace.startsolid = qfalse;
            trace.fraction = 1.0f64 as f32;
            //endpos is the end of the line
            trace.endpos[0 as i32 as usize] = *end.offset(0 as i32 as isize);
            trace.endpos[1 as i32 as usize] = *end.offset(1 as i32 as isize);
            trace.endpos[2 as i32 as usize] = *end.offset(2 as i32 as isize);
            //nothing hit
            trace.ent = 0 as i32;
            trace.area = 0 as i32;
            trace.planenum = 0 as i32;
            return trace;
        }
        //number of the current node to test the line against
        nodenum = (*tstack_p).nodenum;
        if nodenum < 0 as i32 {
            //if it is an area
            //end if
            //AAS_SAMPLE_DEBUG
            //botimport.Print(PRT_MESSAGE, "areanum = %d, must be %d\n", -nodenum, AAS_PointAreaNum(start));
            //if can't enter the area because it hasn't got the right presence type
            if (*crate::src::botlib::be_aas_main::aasworld
                .areasettings
                .offset(-nodenum as isize))
            .presencetype
                & presencetype
                == 0
            {
                //end else
                if (*tstack_p).start[0 as i32 as usize] == *start.offset(0 as i32 as isize)
                    && (*tstack_p).start[1 as i32 as usize] == *start.offset(1 as i32 as isize)
                    && (*tstack_p).start[2 as i32 as usize] == *start.offset(2 as i32 as isize)
                {
                    //end if
                    //if the start point is still the initial start point
                    //NOTE: no need for epsilons because the points will be
                    //exactly the same when they're both the start point
                    //end else
                    trace.startsolid = qtrue; //end if
                    trace.fraction = 0.0f64 as f32;
                    v1[2 as i32 as usize] = 0 as i32 as vec_t;
                    v1[1 as i32 as usize] = v1[2 as i32 as usize];
                    v1[0 as i32 as usize] = v1[1 as i32 as usize]
                } else {
                    trace.startsolid = qfalse;
                    v1[0 as i32 as usize] =
                        *end.offset(0 as i32 as isize) - *start.offset(0 as i32 as isize);
                    v1[1 as i32 as usize] =
                        *end.offset(1 as i32 as isize) - *start.offset(1 as i32 as isize);
                    v1[2 as i32 as usize] =
                        *end.offset(2 as i32 as isize) - *start.offset(2 as i32 as isize);
                    v2[0 as i32 as usize] =
                        (*tstack_p).start[0 as i32 as usize] - *start.offset(0 as i32 as isize);
                    v2[1 as i32 as usize] =
                        (*tstack_p).start[1 as i32 as usize] - *start.offset(1 as i32 as isize);
                    v2[2 as i32 as usize] =
                        (*tstack_p).start[2 as i32 as usize] - *start.offset(2 as i32 as isize);
                    trace.fraction = VectorLength(v2.as_mut_ptr() as *const vec_t)
                        / VectorNormalize(v1.as_mut_ptr());
                    (*tstack_p).start[0 as i32 as usize] = ((*tstack_p).start[0 as i32 as usize]
                        as f64
                        + v1[0 as i32 as usize] as f64 * -0.125f64)
                        as vec_t;
                    (*tstack_p).start[1 as i32 as usize] = ((*tstack_p).start[1 as i32 as usize]
                        as f64
                        + v1[1 as i32 as usize] as f64 * -0.125f64)
                        as vec_t;
                    (*tstack_p).start[2 as i32 as usize] = ((*tstack_p).start[2 as i32 as usize]
                        as f64
                        + v1[2 as i32 as usize] as f64 * -0.125f64)
                        as vec_t
                }
                trace.endpos[0 as i32 as usize] = (*tstack_p).start[0 as i32 as usize];
                trace.endpos[1 as i32 as usize] = (*tstack_p).start[1 as i32 as usize];
                trace.endpos[2 as i32 as usize] = (*tstack_p).start[2 as i32 as usize];
                trace.ent = 0 as i32;
                trace.area = -nodenum;
                //				VectorSubtract(end, start, v1);
                trace.planenum = (*tstack_p).planenum;
                //always take the plane with normal facing towards the trace start
                plane = &mut *crate::src::botlib::be_aas_main::aasworld
                    .planes
                    .offset(trace.planenum as isize) as *mut aas_plane_t; //end if
                if v1[0 as i32 as usize] * (*plane).normal[0 as i32 as usize]
                    + v1[1 as i32 as usize] * (*plane).normal[1 as i32 as usize]
                    + v1[2 as i32 as usize] * (*plane).normal[2 as i32 as usize]
                    > 0 as i32 as f32
                {
                    trace.planenum ^= 1 as i32
                }
                return trace;
            } else {
                if passent >= 0 as i32 {
                    if AAS_AreaEntityCollision(
                        -nodenum,
                        (*tstack_p).start.as_mut_ptr(),
                        (*tstack_p).end.as_mut_ptr(),
                        presencetype,
                        passent,
                        &mut trace,
                    ) as u64
                        != 0
                    {
                        if trace.startsolid as u64 == 0 {
                            v1[0 as i32 as usize] =
                                *end.offset(0 as i32 as isize) - *start.offset(0 as i32 as isize);
                            v1[1 as i32 as usize] =
                                *end.offset(1 as i32 as isize) - *start.offset(1 as i32 as isize);
                            v1[2 as i32 as usize] =
                                *end.offset(2 as i32 as isize) - *start.offset(2 as i32 as isize);
                            v2[0 as i32 as usize] =
                                trace.endpos[0 as i32 as usize] - *start.offset(0 as i32 as isize);
                            v2[1 as i32 as usize] =
                                trace.endpos[1 as i32 as usize] - *start.offset(1 as i32 as isize);
                            v2[2 as i32 as usize] =
                                trace.endpos[2 as i32 as usize] - *start.offset(2 as i32 as isize);
                            trace.fraction = VectorLength(v2.as_mut_ptr() as *const vec_t)
                                / VectorLength(v1.as_mut_ptr() as *const vec_t)
                        }
                        return trace;
                    }
                    //end if
                }
                //end if
            }
            trace.lastarea = -nodenum
        } else {
            //if it is a solid leaf
            if nodenum == 0 {
                //end if
                //if the start point is still the initial start point
                //NOTE: no need for epsilons because the points will be
                //exactly the same when they're both the start point
                if (*tstack_p).start[0 as i32 as usize] == *start.offset(0 as i32 as isize)
                    && (*tstack_p).start[1 as i32 as usize] == *start.offset(1 as i32 as isize)
                    && (*tstack_p).start[2 as i32 as usize] == *start.offset(2 as i32 as isize)
                {
                    //end else
                    trace.startsolid = qtrue; //end if
                    trace.fraction = 0.0f64 as f32; //hit solid leaf
                    v1[2 as i32 as usize] = 0 as i32 as vec_t;
                    v1[1 as i32 as usize] = v1[2 as i32 as usize];
                    v1[0 as i32 as usize] = v1[1 as i32 as usize]
                } else {
                    trace.startsolid = qfalse;
                    v1[0 as i32 as usize] =
                        *end.offset(0 as i32 as isize) - *start.offset(0 as i32 as isize);
                    v1[1 as i32 as usize] =
                        *end.offset(1 as i32 as isize) - *start.offset(1 as i32 as isize);
                    v1[2 as i32 as usize] =
                        *end.offset(2 as i32 as isize) - *start.offset(2 as i32 as isize);
                    v2[0 as i32 as usize] =
                        (*tstack_p).start[0 as i32 as usize] - *start.offset(0 as i32 as isize);
                    v2[1 as i32 as usize] =
                        (*tstack_p).start[1 as i32 as usize] - *start.offset(1 as i32 as isize);
                    v2[2 as i32 as usize] =
                        (*tstack_p).start[2 as i32 as usize] - *start.offset(2 as i32 as isize);
                    trace.fraction = VectorLength(v2.as_mut_ptr() as *const vec_t)
                        / VectorNormalize(v1.as_mut_ptr());
                    (*tstack_p).start[0 as i32 as usize] = ((*tstack_p).start[0 as i32 as usize]
                        as f64
                        + v1[0 as i32 as usize] as f64 * -0.125f64)
                        as vec_t;
                    (*tstack_p).start[1 as i32 as usize] = ((*tstack_p).start[1 as i32 as usize]
                        as f64
                        + v1[1 as i32 as usize] as f64 * -0.125f64)
                        as vec_t;
                    (*tstack_p).start[2 as i32 as usize] = ((*tstack_p).start[2 as i32 as usize]
                        as f64
                        + v1[2 as i32 as usize] as f64 * -0.125f64)
                        as vec_t
                }
                trace.endpos[0 as i32 as usize] = (*tstack_p).start[0 as i32 as usize];
                trace.endpos[1 as i32 as usize] = (*tstack_p).start[1 as i32 as usize];
                trace.endpos[2 as i32 as usize] = (*tstack_p).start[2 as i32 as usize];
                trace.ent = 0 as i32;
                trace.area = 0 as i32;
                //			VectorSubtract(end, start, v1);
                trace.planenum = (*tstack_p).planenum;
                //always take the plane with normal facing towards the trace start
                plane = &mut *crate::src::botlib::be_aas_main::aasworld
                    .planes
                    .offset(trace.planenum as isize) as *mut aas_plane_t;
                if v1[0 as i32 as usize] * (*plane).normal[0 as i32 as usize]
                    + v1[1 as i32 as usize] * (*plane).normal[1 as i32 as usize]
                    + v1[2 as i32 as usize] * (*plane).normal[2 as i32 as usize]
                    > 0 as i32 as f32
                {
                    trace.planenum ^= 1 as i32
                }
                return trace;
            }
            //AAS_SAMPLE_DEBUG
            //the node to test against
            aasnode = &mut *crate::src::botlib::be_aas_main::aasworld
                .nodes
                .offset(nodenum as isize) as *mut aas_node_t;
            //start point of current line to test against node
            cur_start[0 as i32 as usize] = (*tstack_p).start[0 as i32 as usize];
            cur_start[1 as i32 as usize] = (*tstack_p).start[1 as i32 as usize];
            cur_start[2 as i32 as usize] = (*tstack_p).start[2 as i32 as usize];
            //end point of the current line to test against node
            cur_end[0 as i32 as usize] = (*tstack_p).end[0 as i32 as usize];
            cur_end[1 as i32 as usize] = (*tstack_p).end[1 as i32 as usize];
            cur_end[2 as i32 as usize] = (*tstack_p).end[2 as i32 as usize];
            //the current node plane
            plane = &mut *crate::src::botlib::be_aas_main::aasworld
                .planes
                .offset((*aasnode).planenum as isize) as *mut aas_plane_t;
            //end default
            match (*plane).type_0 {
                _ => {}
            } //end switch
            front = cur_start[0 as i32 as usize] * (*plane).normal[0 as i32 as usize]
                + cur_start[1 as i32 as usize] * (*plane).normal[1 as i32 as usize]
                + cur_start[2 as i32 as usize] * (*plane).normal[2 as i32 as usize]
                - (*plane).dist;
            back = cur_end[0 as i32 as usize] * (*plane).normal[0 as i32 as usize]
                + cur_end[1 as i32 as usize] * (*plane).normal[1 as i32 as usize]
                + cur_end[2 as i32 as usize] * (*plane).normal[2 as i32 as usize]
                - (*plane).dist;
            if front >= -(0 as i32) as f32 && back >= -(0 as i32) as f32 {
                //gee it's not an axial plane
                //keep the current start and end point on the stack
                //and go down the tree with the front child
                (*tstack_p).nodenum = (*aasnode).children[0 as i32 as usize];
                tstack_p = tstack_p.offset(1);
                if tstack_p
                    >= &mut *tracestack.as_mut_ptr().offset(127 as i32 as isize)
                        as *mut aas_tracestack_t
                {
                    crate::src::botlib::be_interface::botimport
                        .Print
                        .expect("non-null function pointer")(
                        3 as i32,
                        b"AAS_TraceBoundingBox: stack overflow\n\x00" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                    );
                    return trace;
                }
            //end if
            } else if front < 0 as i32 as f32 && back < 0 as i32 as f32 {
                //if the whole to be traced line is totally at the back of this node
                //only go down the tree with the back child
                //keep the current start and end point on the stack
                //and go down the tree with the back child
                (*tstack_p).nodenum = (*aasnode).children[1 as i32 as usize];
                tstack_p = tstack_p.offset(1);
                if tstack_p
                    >= &mut *tracestack.as_mut_ptr().offset(127 as i32 as isize)
                        as *mut aas_tracestack_t
                {
                    crate::src::botlib::be_interface::botimport
                        .Print
                        .expect("non-null function pointer")(
                        3 as i32,
                        b"AAS_TraceBoundingBox: stack overflow\n\x00" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                    );
                    return trace;
                }
            //end if
            } else {
                //go down the tree both at the front and back of the node
                tmpplanenum = (*tstack_p).planenum;
                //end if
                if front == back {
                    front -= 0.001f32
                }
                if front < 0 as i32 as f32 {
                    frac = ((front as f64 + 0.125f64) / (front - back) as f64) as f32
                } else {
                    frac = ((front as f64 - 0.125f64) / (front - back) as f64) as f32
                }
                if frac < 0 as i32 as f32 {
                    // bk010221 - new location of divide by zero (see above)
                    // bk0101022 - hack/FPE
                    //calculate the hitpoint with the node (split point of the line)
                    //put the crosspoint TRACEPLANE_EPSILON pixels on the near side
                    // bk010221
                    //
                    //1
                    frac = 0.001f32
                } else if frac > 1 as i32 as f32 {
                    frac = 0.999f32
                } //0
                cur_mid[0 as i32 as usize] = cur_start[0 as i32 as usize]
                    + (cur_end[0 as i32 as usize] - cur_start[0 as i32 as usize]) * frac;
                cur_mid[1 as i32 as usize] = cur_start[1 as i32 as usize]
                    + (cur_end[1 as i32 as usize] - cur_start[1 as i32 as usize]) * frac;
                cur_mid[2 as i32 as usize] = cur_start[2 as i32 as usize]
                    + (cur_end[2 as i32 as usize] - cur_start[2 as i32 as usize]) * frac;
                side = (front < 0 as i32 as f32) as i32;
                (*tstack_p).start[0 as i32 as usize] = cur_mid[0 as i32 as usize];
                (*tstack_p).start[1 as i32 as usize] = cur_mid[1 as i32 as usize];
                (*tstack_p).start[2 as i32 as usize] = cur_mid[2 as i32 as usize];
                (*tstack_p).planenum = (*aasnode).planenum;
                (*tstack_p).nodenum = (*aasnode).children[(side == 0) as i32 as usize];
                tstack_p = tstack_p.offset(1);
                if tstack_p
                    >= &mut *tracestack.as_mut_ptr().offset(127 as i32 as isize)
                        as *mut aas_tracestack_t
                {
                    crate::src::botlib::be_interface::botimport
                        .Print
                        .expect("non-null function pointer")(
                        3 as i32,
                        b"AAS_TraceBoundingBox: stack overflow\n\x00" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                    );
                    return trace;
                }
                (*tstack_p).start[0 as i32 as usize] = cur_start[0 as i32 as usize];
                (*tstack_p).start[1 as i32 as usize] = cur_start[1 as i32 as usize];
                (*tstack_p).start[2 as i32 as usize] = cur_start[2 as i32 as usize];
                (*tstack_p).end[0 as i32 as usize] = cur_mid[0 as i32 as usize];
                (*tstack_p).end[1 as i32 as usize] = cur_mid[1 as i32 as usize];
                (*tstack_p).end[2 as i32 as usize] = cur_mid[2 as i32 as usize];
                (*tstack_p).planenum = tmpplanenum;
                (*tstack_p).nodenum = (*aasnode).children[side as usize];
                tstack_p = tstack_p.offset(1);
                if tstack_p
                    >= &mut *tracestack.as_mut_ptr().offset(127 as i32 as isize)
                        as *mut aas_tracestack_t
                {
                    crate::src::botlib::be_interface::botimport
                        .Print
                        .expect("non-null function pointer")(
                        3 as i32,
                        b"AAS_TraceBoundingBox: stack overflow\n\x00" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                    );
                    return trace;
                }
            }
        }
    }
    //frac = front / (front-back);
    //
    //			AAS_DrawPlaneCross(cur_mid, plane->normal, plane->dist, plane->type, LINECOLOR_RED);
    //side the front part of the line is on
    //first put the end part of the line on the stack (back side)
    //not necessary to store because still on stack
    //VectorCopy(cur_end, tstack_p->end);
    //end if
    //now put the part near the start of the line on the stack so we will
    //continue with thats part first. This way we'll find the first
    //hit of the bbox
    // bk010221 - old location of FPE hack and divide by zero expression
    //if the whole to be traced line is totally at the front of this node
    //only go down the tree with the front child
    //end while
    //	return trace;
}
//end of the function AAS_TraceClientBBox
//===========================================================================
// recursive subdivision of the line by the BSP tree.
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_TraceAreas(
    mut start: *mut vec_t,
    mut end: *mut vec_t,
    mut areas: *mut i32,
    mut points: *mut vec3_t,
    mut maxareas: i32,
) -> i32 {
    let mut side: i32 = 0;
    let mut nodenum: i32 = 0;
    let mut tmpplanenum: i32 = 0;
    let mut numareas: i32 = 0;
    let mut front: f32 = 0.;
    let mut back: f32 = 0.;
    let mut frac: f32 = 0.;
    let mut cur_start: vec3_t = [0.; 3];
    let mut cur_end: vec3_t = [0.; 3];
    let mut cur_mid: vec3_t = [0.; 3];
    let mut tracestack: [aas_tracestack_t; 127] = [aas_tracestack_t {
        start: [0.; 3],
        end: [0.; 3],
        planenum: 0,
        nodenum: 0,
    }; 127];
    let mut tstack_p: *mut aas_tracestack_t = std::ptr::null_mut();
    let mut aasnode: *mut aas_node_t = std::ptr::null_mut();
    let mut plane: *mut aas_plane_t = std::ptr::null_mut();
    numareas = 0 as i32;
    *areas.offset(0 as i32 as isize) = 0 as i32;
    if crate::src::botlib::be_aas_main::aasworld.loaded == 0 {
        return numareas;
    }
    tstack_p = tracestack.as_mut_ptr();
    //we start with the whole line on the stack
    (*tstack_p).start[0 as i32 as usize] = *start.offset(0 as i32 as isize);
    (*tstack_p).start[1 as i32 as usize] = *start.offset(1 as i32 as isize);
    (*tstack_p).start[2 as i32 as usize] = *start.offset(2 as i32 as isize);
    (*tstack_p).end[0 as i32 as usize] = *end.offset(0 as i32 as isize);
    (*tstack_p).end[1 as i32 as usize] = *end.offset(1 as i32 as isize);
    (*tstack_p).end[2 as i32 as usize] = *end.offset(2 as i32 as isize);
    (*tstack_p).planenum = 0 as i32;
    //start with node 1 because node zero is a dummy for a solid leaf
    (*tstack_p).nodenum = 1 as i32; //starting at the root of the tree
    tstack_p = tstack_p.offset(1);
    loop {
        //pop up the stack
        tstack_p = tstack_p.offset(-1);
        //end else
        //if the trace stack is empty (ended up with a piece of the
        //line to be traced in an area)
        if tstack_p < tracestack.as_mut_ptr() {
            return numareas;
        } //end if
          //number of the current node to test the line against
        nodenum = (*tstack_p).nodenum;
        if nodenum < 0 as i32 {
            //if it is an area
            //end if
            //AAS_SAMPLE_DEBUG
            //botimport.Print(PRT_MESSAGE, "areanum = %d, must be %d\n", -nodenum, AAS_PointAreaNum(start));
            *areas.offset(numareas as isize) = -nodenum;
            if !points.is_null() {
                (*points.offset(numareas as isize))[0 as i32 as usize] =
                    (*tstack_p).start[0 as i32 as usize];
                (*points.offset(numareas as isize))[1 as i32 as usize] =
                    (*tstack_p).start[1 as i32 as usize];
                (*points.offset(numareas as isize))[2 as i32 as usize] =
                    (*tstack_p).start[2 as i32 as usize]
            }
            numareas += 1;
            if numareas >= maxareas {
                return numareas;
            }
        } else {
            //if it is a solid leaf
            if nodenum == 0 {
                continue; //end if
            }
            //AAS_SAMPLE_DEBUG
            //the node to test against
            aasnode = &mut *crate::src::botlib::be_aas_main::aasworld
                .nodes
                .offset(nodenum as isize) as *mut aas_node_t;
            //start point of current line to test against node
            cur_start[0 as i32 as usize] = (*tstack_p).start[0 as i32 as usize];
            cur_start[1 as i32 as usize] = (*tstack_p).start[1 as i32 as usize];
            cur_start[2 as i32 as usize] = (*tstack_p).start[2 as i32 as usize];
            //end point of the current line to test against node
            cur_end[0 as i32 as usize] = (*tstack_p).end[0 as i32 as usize];
            cur_end[1 as i32 as usize] = (*tstack_p).end[1 as i32 as usize];
            cur_end[2 as i32 as usize] = (*tstack_p).end[2 as i32 as usize];
            //the current node plane
            plane = &mut *crate::src::botlib::be_aas_main::aasworld
                .planes
                .offset((*aasnode).planenum as isize) as *mut aas_plane_t;
            //end default
            match (*plane).type_0 {
                _ => {}
            } //end switch
            front = cur_start[0 as i32 as usize] * (*plane).normal[0 as i32 as usize]
                + cur_start[1 as i32 as usize] * (*plane).normal[1 as i32 as usize]
                + cur_start[2 as i32 as usize] * (*plane).normal[2 as i32 as usize]
                - (*plane).dist;
            back = cur_end[0 as i32 as usize] * (*plane).normal[0 as i32 as usize]
                + cur_end[1 as i32 as usize] * (*plane).normal[1 as i32 as usize]
                + cur_end[2 as i32 as usize] * (*plane).normal[2 as i32 as usize]
                - (*plane).dist;
            if front > 0 as i32 as f32 && back > 0 as i32 as f32 {
                //gee it's not an axial plane
                //keep the current start and end point on the stack
                //and go down the tree with the front child
                (*tstack_p).nodenum = (*aasnode).children[0 as i32 as usize];
                tstack_p = tstack_p.offset(1);
                if tstack_p
                    >= &mut *tracestack.as_mut_ptr().offset(127 as i32 as isize)
                        as *mut aas_tracestack_t
                {
                    crate::src::botlib::be_interface::botimport
                        .Print
                        .expect("non-null function pointer")(
                        3 as i32,
                        b"AAS_TraceAreas: stack overflow\n\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    );
                    return numareas;
                }
            //end if
            } else if front <= 0 as i32 as f32 && back <= 0 as i32 as f32 {
                //if the whole to be traced line is totally at the back of this node
                //only go down the tree with the back child
                //keep the current start and end point on the stack
                //and go down the tree with the back child
                (*tstack_p).nodenum = (*aasnode).children[1 as i32 as usize];
                tstack_p = tstack_p.offset(1);
                if tstack_p
                    >= &mut *tracestack.as_mut_ptr().offset(127 as i32 as isize)
                        as *mut aas_tracestack_t
                {
                    crate::src::botlib::be_interface::botimport
                        .Print
                        .expect("non-null function pointer")(
                        3 as i32,
                        b"AAS_TraceAreas: stack overflow\n\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    );
                    return numareas;
                }
            //end if
            } else {
                //go down the tree both at the front and back of the node
                tmpplanenum = (*tstack_p).planenum;
                //end if
                if front < 0 as i32 as f32 {
                    frac = front / (front - back)
                } else {
                    frac = front / (front - back)
                }
                if frac < 0 as i32 as f32 {
                    frac = 0 as i32 as f32
                } else if frac > 1 as i32 as f32 {
                    frac = 1 as i32 as f32
                }
                cur_mid[0 as i32 as usize] = cur_start[0 as i32 as usize]
                    + (cur_end[0 as i32 as usize] - cur_start[0 as i32 as usize]) * frac;
                cur_mid[1 as i32 as usize] = cur_start[1 as i32 as usize]
                    + (cur_end[1 as i32 as usize] - cur_start[1 as i32 as usize]) * frac;
                cur_mid[2 as i32 as usize] = cur_start[2 as i32 as usize]
                    + (cur_end[2 as i32 as usize] - cur_start[2 as i32 as usize]) * frac;
                side = (front < 0 as i32 as f32) as i32;
                (*tstack_p).start[0 as i32 as usize] = cur_mid[0 as i32 as usize];
                (*tstack_p).start[1 as i32 as usize] = cur_mid[1 as i32 as usize];
                (*tstack_p).start[2 as i32 as usize] = cur_mid[2 as i32 as usize];
                (*tstack_p).planenum = (*aasnode).planenum;
                (*tstack_p).nodenum = (*aasnode).children[(side == 0) as i32 as usize];
                tstack_p = tstack_p.offset(1);
                if tstack_p
                    >= &mut *tracestack.as_mut_ptr().offset(127 as i32 as isize)
                        as *mut aas_tracestack_t
                {
                    crate::src::botlib::be_interface::botimport
                        .Print
                        .expect("non-null function pointer")(
                        3 as i32,
                        b"AAS_TraceAreas: stack overflow\n\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    );
                    return numareas;
                }
                (*tstack_p).start[0 as i32 as usize] = cur_start[0 as i32 as usize];
                (*tstack_p).start[1 as i32 as usize] = cur_start[1 as i32 as usize];
                (*tstack_p).start[2 as i32 as usize] = cur_start[2 as i32 as usize];
                (*tstack_p).end[0 as i32 as usize] = cur_mid[0 as i32 as usize];
                (*tstack_p).end[1 as i32 as usize] = cur_mid[1 as i32 as usize];
                (*tstack_p).end[2 as i32 as usize] = cur_mid[2 as i32 as usize];
                (*tstack_p).planenum = tmpplanenum;
                (*tstack_p).nodenum = (*aasnode).children[side as usize];
                tstack_p = tstack_p.offset(1);
                if tstack_p
                    >= &mut *tracestack.as_mut_ptr().offset(127 as i32 as isize)
                        as *mut aas_tracestack_t
                {
                    crate::src::botlib::be_interface::botimport
                        .Print
                        .expect("non-null function pointer")(
                        3 as i32,
                        b"AAS_TraceAreas: stack overflow\n\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    );
                    return numareas;
                }
            }
        }
    }
    //calculate the hitpoint with the node (split point of the line)
    //put the crosspoint TRACEPLANE_EPSILON pixels on the near side
    //frac = front / (front-back);
    //
    //			AAS_DrawPlaneCross(cur_mid, plane->normal, plane->dist, plane->type, LINECOLOR_RED);
    //side the front part of the line is on
    //first put the end part of the line on the stack (back side)
    //not necessary to store because still on stack
    //VectorCopy(cur_end, tstack_p->end);
    //end if
    //now put the part near the start of the line on the stack so we will
    //continue with thats part first. This way we'll find the first
    //hit of the bbox
    //if the whole to be traced line is totally at the front of this node
    //only go down the tree with the front child
    //end while
    //	return numareas;
}
//end of the function AAS_TraceAreas
//===========================================================================
// a simple cross product
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
// void AAS_OrthogonalToVectors(vec3_t v1, vec3_t v2, vec3_t res)
//===========================================================================
// tests if the given point is within the face boundaries
//
// Parameter:				face		: face to test if the point is in it
//								pnormal	: normal of the plane to use for the face
//								point		: point to test if inside face boundaries
// Returns:					qtrue if the point is within the face boundaries
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_InsideFace(
    mut face: *mut aas_face_t,
    mut pnormal: *mut vec_t,
    mut point: *mut vec_t,
    mut epsilon: f32,
) -> qboolean {
    let mut i: i32 = 0;
    let mut firstvertex: i32 = 0;
    let mut edgenum: i32 = 0;
    let mut v0: vec3_t = [0.; 3];
    let mut edgevec: vec3_t = [0.; 3];
    let mut pointvec: vec3_t = [0.; 3];
    let mut sepnormal: vec3_t = [0.; 3];
    let mut edge: *mut aas_edge_t = std::ptr::null_mut();
    //AAS_SAMPLE_DEBUG
    if crate::src::botlib::be_aas_main::aasworld.loaded == 0 {
        return qfalse;
    } //end for
    i = 0 as i32;
    while i < (*face).numedges {
        edgenum = *crate::src::botlib::be_aas_main::aasworld
            .edgeindex
            .offset(((*face).firstedge + i) as isize);
        edge = &mut *crate::src::botlib::be_aas_main::aasworld
            .edges
            .offset((libc::abs as unsafe extern "C" fn(_: i32) -> i32)(edgenum) as isize)
            as *mut aas_edge_t;
        //get the first vertex of the edge
        firstvertex = (edgenum < 0 as i32) as i32;
        v0[0 as i32 as usize] = (*crate::src::botlib::be_aas_main::aasworld
            .vertexes
            .offset((*edge).v[firstvertex as usize] as isize))[0 as i32 as usize];
        v0[1 as i32 as usize] = (*crate::src::botlib::be_aas_main::aasworld
            .vertexes
            .offset((*edge).v[firstvertex as usize] as isize))[1 as i32 as usize];
        v0[2 as i32 as usize] = (*crate::src::botlib::be_aas_main::aasworld
            .vertexes
            .offset((*edge).v[firstvertex as usize] as isize))[2 as i32 as usize];
        //edge vector
        edgevec[0 as i32 as usize] = (*crate::src::botlib::be_aas_main::aasworld
            .vertexes
            .offset((*edge).v[(firstvertex == 0) as i32 as usize] as isize))[0 as i32 as usize]
            - v0[0 as i32 as usize];
        edgevec[1 as i32 as usize] = (*crate::src::botlib::be_aas_main::aasworld
            .vertexes
            .offset((*edge).v[(firstvertex == 0) as i32 as usize] as isize))[1 as i32 as usize]
            - v0[1 as i32 as usize];
        edgevec[2 as i32 as usize] = (*crate::src::botlib::be_aas_main::aasworld
            .vertexes
            .offset((*edge).v[(firstvertex == 0) as i32 as usize] as isize))[2 as i32 as usize]
            - v0[2 as i32 as usize];
        //
        //AAS_SAMPLE_DEBUG
        //vector from first edge point to point possible in face
        pointvec[0 as i32 as usize] = *point.offset(0 as i32 as isize) - v0[0 as i32 as usize];
        pointvec[1 as i32 as usize] = *point.offset(1 as i32 as isize) - v0[1 as i32 as usize];
        pointvec[2 as i32 as usize] = *point.offset(2 as i32 as isize) - v0[2 as i32 as usize];
        //get a vector pointing inside the face orthogonal to both the
        //edge vector and the normal vector of the plane the face is in
        //this vector defines a plane through the origin (first vertex of
        //edge) and through both the edge vector and the normal vector
        //of the plane
        sepnormal[0 as i32 as usize] = edgevec[1 as i32 as usize]
            * *pnormal.offset(2 as i32 as isize)
            - edgevec[2 as i32 as usize] * *pnormal.offset(1 as i32 as isize);
        sepnormal[1 as i32 as usize] = edgevec[2 as i32 as usize]
            * *pnormal.offset(0 as i32 as isize)
            - edgevec[0 as i32 as usize] * *pnormal.offset(2 as i32 as isize);
        sepnormal[2 as i32 as usize] = edgevec[0 as i32 as usize]
            * *pnormal.offset(1 as i32 as isize)
            - edgevec[1 as i32 as usize] * *pnormal.offset(0 as i32 as isize);
        //check on which side of the above plane the point is
        //this is done by checking the sign of the dot product of the
        //vector orthogonal vector from above and the vector from the
        //origin (first vertex of edge) to the point
        //if the dotproduct is smaller than zero the point is outside the face
        if pointvec[0 as i32 as usize] * sepnormal[0 as i32 as usize]
            + pointvec[1 as i32 as usize] * sepnormal[1 as i32 as usize]
            + pointvec[2 as i32 as usize] * sepnormal[2 as i32 as usize]
            < -epsilon
        {
            return qfalse;
        }
        i += 1
    }
    return qtrue;
}
//end of the function AAS_InsideFace
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_PointInsideFace(
    mut facenum: i32,
    mut point: *mut vec_t,
    mut epsilon: f32,
) -> qboolean {
    let mut i: i32 = 0;
    let mut firstvertex: i32 = 0;
    let mut edgenum: i32 = 0;
    let mut v1: *mut vec_t = std::ptr::null_mut();
    let mut v2: *mut vec_t = std::ptr::null_mut();
    let mut edgevec: vec3_t = [0.; 3];
    let mut pointvec: vec3_t = [0.; 3];
    let mut sepnormal: vec3_t = [0.; 3];
    let mut edge: *mut aas_edge_t = std::ptr::null_mut();
    let mut plane: *mut aas_plane_t = std::ptr::null_mut();
    let mut face: *mut aas_face_t = std::ptr::null_mut();
    if crate::src::botlib::be_aas_main::aasworld.loaded == 0 {
        return qfalse;
    }
    face = &mut *crate::src::botlib::be_aas_main::aasworld
        .faces
        .offset(facenum as isize) as *mut aas_face_t;
    plane = &mut *crate::src::botlib::be_aas_main::aasworld
        .planes
        .offset((*face).planenum as isize) as *mut aas_plane_t;
    //
    i = 0 as i32; //end for
    while i < (*face).numedges {
        edgenum = *crate::src::botlib::be_aas_main::aasworld
            .edgeindex
            .offset(((*face).firstedge + i) as isize);
        edge = &mut *crate::src::botlib::be_aas_main::aasworld
            .edges
            .offset((libc::abs as unsafe extern "C" fn(_: i32) -> i32)(edgenum) as isize)
            as *mut aas_edge_t;
        //get the first vertex of the edge
        firstvertex = (edgenum < 0 as i32) as i32;
        v1 = (*crate::src::botlib::be_aas_main::aasworld
            .vertexes
            .offset((*edge).v[firstvertex as usize] as isize))
        .as_mut_ptr();
        v2 = (*crate::src::botlib::be_aas_main::aasworld
            .vertexes
            .offset((*edge).v[(firstvertex == 0) as i32 as usize] as isize))
        .as_mut_ptr();
        //edge vector
        edgevec[0 as i32 as usize] = *v2.offset(0 as i32 as isize) - *v1.offset(0 as i32 as isize);
        edgevec[1 as i32 as usize] = *v2.offset(1 as i32 as isize) - *v1.offset(1 as i32 as isize);
        edgevec[2 as i32 as usize] = *v2.offset(2 as i32 as isize) - *v1.offset(2 as i32 as isize);
        //vector from first edge point to point possible in face
        pointvec[0 as i32 as usize] =
            *point.offset(0 as i32 as isize) - *v1.offset(0 as i32 as isize);
        pointvec[1 as i32 as usize] =
            *point.offset(1 as i32 as isize) - *v1.offset(1 as i32 as isize);
        pointvec[2 as i32 as usize] =
            *point.offset(2 as i32 as isize) - *v1.offset(2 as i32 as isize);
        //
        CrossProduct(
            edgevec.as_mut_ptr() as *const vec_t,
            (*plane).normal.as_mut_ptr() as *const vec_t,
            sepnormal.as_mut_ptr(),
        );
        //
        if pointvec[0 as i32 as usize] * sepnormal[0 as i32 as usize]
            + pointvec[1 as i32 as usize] * sepnormal[1 as i32 as usize]
            + pointvec[2 as i32 as usize] * sepnormal[2 as i32 as usize]
            < -epsilon
        {
            return qfalse;
        }
        i += 1
    }
    return qtrue;
}
//end of the function AAS_PointInsideFace
//===========================================================================
// returns the ground face the given point is above in the given area
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_AreaGroundFace(
    mut areanum: i32,
    mut point: *mut vec_t,
) -> *mut aas_face_t {
    let mut i: i32 = 0; //end for
    let mut facenum: i32 = 0;
    let mut up: vec3_t = [0 as i32 as vec_t, 0 as i32 as vec_t, 1 as i32 as vec_t];
    let mut normal: vec3_t = [0.; 3];
    let mut area: *mut aas_area_t = std::ptr::null_mut();
    let mut face: *mut aas_face_t = std::ptr::null_mut();
    if crate::src::botlib::be_aas_main::aasworld.loaded == 0 {
        return std::ptr::null_mut();
    }
    area = &mut *crate::src::botlib::be_aas_main::aasworld
        .areas
        .offset(areanum as isize) as *mut aas_area_t;
    i = 0 as i32;
    while i < (*area).numfaces {
        facenum = *crate::src::botlib::be_aas_main::aasworld
            .faceindex
            .offset(((*area).firstface + i) as isize);
        face = &mut *crate::src::botlib::be_aas_main::aasworld
            .faces
            .offset((libc::abs as unsafe extern "C" fn(_: i32) -> i32)(facenum) as isize)
            as *mut aas_face_t;
        //end if
        if (*face).faceflags & 4 as i32 != 0 {
            //if this is a ground face
            //get the up or down normal
            if (*crate::src::botlib::be_aas_main::aasworld
                .planes
                .offset((*face).planenum as isize))
            .normal[2 as i32 as usize]
                < 0 as i32 as f32
            {
                normal[0 as i32 as usize] = -up[0 as i32 as usize];
                normal[1 as i32 as usize] = -up[1 as i32 as usize];
                normal[2 as i32 as usize] = -up[2 as i32 as usize]
            } else {
                normal[0 as i32 as usize] = up[0 as i32 as usize];
                normal[1 as i32 as usize] = up[1 as i32 as usize];
                normal[2 as i32 as usize] = up[2 as i32 as usize]
            }
            //check if the point is in the face
            if AAS_InsideFace(face, normal.as_mut_ptr(), point, 0.01f32) as u64 != 0 {
                return face;
            }
        }
        i += 1
    }
    return std::ptr::null_mut();
}
//returns the area the point is in
//returns the area the point is in
//
//
//returns the plane the given face is in
//returns the plane the given face is in
//end of the function AAS_AreaGroundFace
//===========================================================================
// returns the face the trace end position is situated in
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_FacePlane(
    mut facenum: i32,
    mut normal: *mut vec_t,
    mut dist: *mut f32,
) {
    let mut plane: *mut aas_plane_t = std::ptr::null_mut();
    plane = &mut *crate::src::botlib::be_aas_main::aasworld.planes.offset(
        (*crate::src::botlib::be_aas_main::aasworld
            .faces
            .offset(facenum as isize))
        .planenum as isize,
    ) as *mut aas_plane_t;
    *normal.offset(0 as i32 as isize) = (*plane).normal[0 as i32 as usize];
    *normal.offset(1 as i32 as isize) = (*plane).normal[1 as i32 as usize];
    *normal.offset(2 as i32 as isize) = (*plane).normal[2 as i32 as usize];
    *dist = (*plane).dist;
}
//end of the function AAS_FacePlane
//===========================================================================
// returns the face the trace end position is situated in
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_TraceEndFace(mut trace: *mut aas_trace_t) -> *mut aas_face_t {
    let mut i: i32 = 0;
    let mut facenum: i32 = 0;
    let mut area: *mut aas_area_t = std::ptr::null_mut();
    let mut face: *mut aas_face_t = std::ptr::null_mut();
    let mut firstface: *mut aas_face_t = std::ptr::null_mut();
    if crate::src::botlib::be_aas_main::aasworld.loaded == 0 {
        return std::ptr::null_mut();
    }
    //if started in solid no face was hit
    if (*trace).startsolid as u64 != 0 {
        return std::ptr::null_mut();
    }
    //trace->lastarea is the last area the trace was in
    area = &mut *crate::src::botlib::be_aas_main::aasworld
        .areas
        .offset((*trace).lastarea as isize) as *mut aas_area_t;
    //check which face the trace.endpos was in
    i = 0 as i32; //end for
    while i < (*area).numfaces {
        facenum = *crate::src::botlib::be_aas_main::aasworld
            .faceindex
            .offset(((*area).firstface + i) as isize);
        face = &mut *crate::src::botlib::be_aas_main::aasworld
            .faces
            .offset((libc::abs as unsafe extern "C" fn(_: i32) -> i32)(facenum) as isize)
            as *mut aas_face_t;
        //end if
        if (*face).planenum & !(1 as i32) == (*trace).planenum & !(1 as i32) {
            //if the face is in the same plane as the trace end point
            //firstface is used for optimization, if theres only one
            //face in the plane then it has to be the good one
            //if there are more faces in the same plane then always
            //check the one with the fewest edges first
            /*			if (firstface)
            {
                if (firstface->numedges < face->numedges)
                {
                    if (AAS_InsideFace(firstface,
                        aasworld.planes[face->planenum].normal, trace->endpos))
                    {
                        return firstface;
                    } //end if
                    firstface = face;
                } //end if
                else
                {
                    if (AAS_InsideFace(face,
                        aasworld.planes[face->planenum].normal, trace->endpos))
                    {
                        return face;
                    } //end if
                } //end else
            } //end if
            else
            {
                firstface = face;
            } //end else*/
            if AAS_InsideFace(
                face,
                (*crate::src::botlib::be_aas_main::aasworld
                    .planes
                    .offset((*face).planenum as isize))
                .normal
                .as_mut_ptr(),
                (*trace).endpos.as_mut_ptr(),
                0.01f32,
            ) as u64
                != 0
            {
                return face;
            }
        }
        i += 1
    }
    return firstface;
}
//end of the function AAS_TraceEndFace
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_BoxOnPlaneSide2(
    mut absmins: *mut vec_t,
    mut absmaxs: *mut vec_t,
    mut p: *mut aas_plane_t,
) -> i32 {
    let mut i: i32 = 0; //end for
    let mut sides: i32 = 0; //end if
    let mut dist1: f32 = 0.;
    let mut dist2: f32 = 0.;
    let mut corners: [vec3_t; 2] = [[0.; 3]; 2];
    i = 0 as i32;
    while i < 3 as i32 {
        if (*p).normal[i as usize] < 0 as i32 as f32 {
            corners[0 as i32 as usize][i as usize] = *absmins.offset(i as isize);
            corners[1 as i32 as usize][i as usize] = *absmaxs.offset(i as isize)
        } else {
            corners[1 as i32 as usize][i as usize] = *absmins.offset(i as isize);
            corners[0 as i32 as usize][i as usize] = *absmaxs.offset(i as isize)
        }
        i += 1
        //end else
    }
    dist1 = (*p).normal[0 as i32 as usize] * corners[0 as i32 as usize][0 as i32 as usize]
        + (*p).normal[1 as i32 as usize] * corners[0 as i32 as usize][1 as i32 as usize]
        + (*p).normal[2 as i32 as usize] * corners[0 as i32 as usize][2 as i32 as usize]
        - (*p).dist;
    dist2 = (*p).normal[0 as i32 as usize] * corners[1 as i32 as usize][0 as i32 as usize]
        + (*p).normal[1 as i32 as usize] * corners[1 as i32 as usize][1 as i32 as usize]
        + (*p).normal[2 as i32 as usize] * corners[1 as i32 as usize][2 as i32 as usize]
        - (*p).dist;
    sides = 0 as i32;
    if dist1 >= 0 as i32 as f32 {
        sides = 1 as i32
    }
    if dist2 < 0 as i32 as f32 {
        sides |= 2 as i32
    }
    return sides;
}
//end of the function AAS_BoxOnPlaneSide2
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
//int AAS_BoxOnPlaneSide(vec3_t absmins, vec3_t absmaxs, aas_plane_t *p)
//end of the function AAS_BoxOnPlaneSide
//===========================================================================
// remove the links to this entity from all areas
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_UnlinkFromAreas(mut areas: *mut aas_link_t) {
    let mut link: *mut aas_link_t = std::ptr::null_mut();
    let mut nextlink: *mut aas_link_t = std::ptr::null_mut();
    link = areas;
    while !link.is_null() {
        //next area the entity is linked in
        nextlink = (*link).next_area;
        //remove the entity from the linked list of this area
        if !(*link).prev_ent.is_null() {
            (*(*link).prev_ent).next_ent = (*link).next_ent
        } else {
            let ref mut fresh6 = *crate::src::botlib::be_aas_main::aasworld
                .arealinkedentities
                .offset((*link).areanum as isize);
            *fresh6 = (*link).next_ent
        }
        if !(*link).next_ent.is_null() {
            (*(*link).next_ent).prev_ent = (*link).prev_ent
        }
        //deallocate the link structure
        AAS_DeAllocAASLink(link);
        link = nextlink
    }
    //end for
}
#[no_mangle]

pub unsafe extern "C" fn AAS_AASLinkEntity(
    mut absmins: *mut vec_t,
    mut absmaxs: *mut vec_t,
    mut entnum: i32,
) -> *mut aas_link_t {
    let mut side: i32 = 0; //end if
    let mut nodenum: i32 = 0;
    let mut linkstack: [aas_linkstack_t; 128] = [aas_linkstack_t { nodenum: 0 }; 128];
    let mut lstack_p: *mut aas_linkstack_t = std::ptr::null_mut();
    let mut aasnode: *mut aas_node_t = std::ptr::null_mut();
    let mut plane: *mut aas_plane_t = std::ptr::null_mut();
    let mut link: *mut aas_link_t = std::ptr::null_mut();
    let mut areas: *mut aas_link_t = std::ptr::null_mut();
    if crate::src::botlib::be_aas_main::aasworld.loaded == 0 {
        crate::src::botlib::be_interface::botimport
            .Print
            .expect("non-null function pointer")(
            3 as i32,
            b"AAS_LinkEntity: aas not loaded\n\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return std::ptr::null_mut();
    }
    areas = std::ptr::null_mut();
    //
    lstack_p = linkstack.as_mut_ptr();
    //we start with the whole line on the stack
    //start with node 1 because node zero is a dummy used for solid leafs
    (*lstack_p).nodenum = 1 as i32; //starting at the root of the tree
    lstack_p = lstack_p.offset(1); //end while
    loop
    //pop up the stack
    {
        lstack_p = lstack_p.offset(-1);
        //if the trace stack is empty (ended up with a piece of the
        //line to be traced in an area)
        if lstack_p < linkstack.as_mut_ptr() {
            break;
        }
        //number of the current node to test the line against
        nodenum = (*lstack_p).nodenum;
        //if it is an area
        if nodenum < 0 as i32 {
            //end if
            //NOTE: the entity might have already been linked into this area
            // because several node children can point to the same area
            link = *crate::src::botlib::be_aas_main::aasworld
                .arealinkedentities
                .offset(-nodenum as isize); //end for
            while !link.is_null() {
                if (*link).entnum == entnum {
                    break;
                }
                link = (*link).next_ent
            }
            if !link.is_null() {
                continue;
            }
            //
            link = AAS_AllocAASLink();
            if link.is_null() {
                return areas;
            }
            (*link).entnum = entnum;
            (*link).areanum = -nodenum;
            //put the link into the double linked area list of the entity
            (*link).prev_area = std::ptr::null_mut();
            (*link).next_area = areas;
            if !areas.is_null() {
                (*areas).prev_area = link
            }
            areas = link;
            //put the link into the double linked entity list of the area
            (*link).prev_ent = std::ptr::null_mut();
            (*link).next_ent = *crate::src::botlib::be_aas_main::aasworld
                .arealinkedentities
                .offset(-nodenum as isize);
            if !(*crate::src::botlib::be_aas_main::aasworld
                .arealinkedentities
                .offset(-nodenum as isize))
            .is_null()
            {
                let ref mut fresh7 = (**crate::src::botlib::be_aas_main::aasworld
                    .arealinkedentities
                    .offset(-nodenum as isize))
                .prev_ent;
                *fresh7 = link
            }
            let ref mut fresh8 = *crate::src::botlib::be_aas_main::aasworld
                .arealinkedentities
                .offset(-nodenum as isize);
            *fresh8 = link
        } else {
            //if solid leaf
            if nodenum == 0 {
                continue;
            }
            //the node to test against
            aasnode = &mut *crate::src::botlib::be_aas_main::aasworld
                .nodes
                .offset(nodenum as isize) as *mut aas_node_t;
            //the current node plane
            plane = &mut *crate::src::botlib::be_aas_main::aasworld
                .planes
                .offset((*aasnode).planenum as isize) as *mut aas_plane_t;
            //get the side(s) the box is situated relative to the plane
            side = AAS_BoxOnPlaneSide2(absmins, absmaxs, plane);
            //if on the front side of the node
            if side & 1 as i32 != 0 {
                (*lstack_p).nodenum = (*aasnode).children[0 as i32 as usize]; //end if
                lstack_p = lstack_p.offset(1)
            } //end if
            if lstack_p
                >= &mut *linkstack.as_mut_ptr().offset(127 as i32 as isize) as *mut aas_linkstack_t
            {
                crate::src::botlib::be_interface::botimport
                    .Print
                    .expect("non-null function pointer")(
                    3 as i32,
                    b"AAS_LinkEntity: stack overflow\n\x00" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
                break;
            } else {
                //if on the back side of the node
                if side & 2 as i32 != 0 {
                    (*lstack_p).nodenum = (*aasnode).children[1 as i32 as usize]; //end if
                    lstack_p = lstack_p.offset(1)
                }
                if !(lstack_p
                    >= &mut *linkstack.as_mut_ptr().offset(127 as i32 as isize)
                        as *mut aas_linkstack_t)
                {
                    continue;
                }
                crate::src::botlib::be_interface::botimport
                    .Print
                    .expect("non-null function pointer")(
                    3 as i32,
                    b"AAS_LinkEntity: stack overflow\n\x00" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
                break;
            }
        }
    }
    return areas;
}
//end of the function AAS_AASLinkEntity
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_LinkEntityClientBBox(
    mut absmins: *mut vec_t,
    mut absmaxs: *mut vec_t,
    mut entnum: i32,
    mut presencetype: i32,
) -> *mut aas_link_t {
    let mut mins: vec3_t = [0.; 3];
    let mut maxs: vec3_t = [0.; 3];
    let mut newabsmins: vec3_t = [0.; 3];
    let mut newabsmaxs: vec3_t = [0.; 3];
    AAS_PresenceTypeBoundingBox(presencetype, mins.as_mut_ptr(), maxs.as_mut_ptr());
    newabsmins[0 as i32 as usize] = *absmins.offset(0 as i32 as isize) - maxs[0 as i32 as usize];
    newabsmins[1 as i32 as usize] = *absmins.offset(1 as i32 as isize) - maxs[1 as i32 as usize];
    newabsmins[2 as i32 as usize] = *absmins.offset(2 as i32 as isize) - maxs[2 as i32 as usize];
    newabsmaxs[0 as i32 as usize] = *absmaxs.offset(0 as i32 as isize) - mins[0 as i32 as usize];
    newabsmaxs[1 as i32 as usize] = *absmaxs.offset(1 as i32 as isize) - mins[1 as i32 as usize];
    newabsmaxs[2 as i32 as usize] = *absmaxs.offset(2 as i32 as isize) - mins[2 as i32 as usize];
    //relink the entity
    return AAS_AASLinkEntity(newabsmins.as_mut_ptr(), newabsmaxs.as_mut_ptr(), entnum);
}
//end of the function AAS_LinkEntityClientBBox
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_BBoxAreas(
    mut absmins: *mut vec_t,
    mut absmaxs: *mut vec_t,
    mut areas: *mut i32,
    mut maxareas: i32,
) -> i32 {
    let mut linkedareas: *mut aas_link_t = std::ptr::null_mut(); //end for
    let mut link: *mut aas_link_t = std::ptr::null_mut();
    let mut num: i32 = 0;
    linkedareas = AAS_AASLinkEntity(absmins, absmaxs, -(1 as i32));
    num = 0 as i32;
    link = linkedareas;
    while !link.is_null() {
        *areas.offset(num as isize) = (*link).areanum;
        num += 1;
        if num >= maxareas {
            break;
        }
        link = (*link).next_area
    }
    AAS_UnlinkFromAreas(linkedareas);
    return num;
}
//AASINTERN
//AASINTERN
//returns the mins and maxs of the bounding box for the given presence type
//returns the mins and maxs of the bounding box for the given presence type
//returns the cluster the area is in (negative portal number if the area is a portal)
//returns the cluster the area is in (negative portal number if the area is a portal)
//returns the presence type(s) of the area
//returns the presence type(s) of the area
//returns the presence type(s) at the given point
//returns the presence type(s) at the given point
//returns the result of the trace of a client bbox
//returns the result of the trace of a client bbox
//stores the areas the trace went through and returns the number of passed areas
//stores the areas the trace went through and returns the number of passed areas
//returns the areas the bounding box is in
//returns the areas the bounding box is in
//return area information
//return area information
//end of the function AAS_BBoxAreas
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_AreaInfo(mut areanum: i32, mut info: *mut aas_areainfo_t) -> i32 {
    let mut settings: *mut aas_areasettings_t = std::ptr::null_mut(); //end if
    if info.is_null() {
        return 0 as i32;
    }
    if areanum <= 0 as i32 || areanum >= crate::src::botlib::be_aas_main::aasworld.numareas {
        crate::src::botlib::be_interface::botimport
            .Print
            .expect("non-null function pointer")(
            3 as i32,
            b"AAS_AreaInfo: areanum %d out of range\n\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            areanum,
        );
        return 0 as i32;
    }
    settings = &mut *crate::src::botlib::be_aas_main::aasworld
        .areasettings
        .offset(areanum as isize) as *mut aas_areasettings_t;
    (*info).cluster = (*settings).cluster;
    (*info).contents = (*settings).contents;
    (*info).flags = (*settings).areaflags;
    (*info).presencetype = (*settings).presencetype;
    (*info).mins[0 as i32 as usize] = (*crate::src::botlib::be_aas_main::aasworld
        .areas
        .offset(areanum as isize))
    .mins[0 as i32 as usize];
    (*info).mins[1 as i32 as usize] = (*crate::src::botlib::be_aas_main::aasworld
        .areas
        .offset(areanum as isize))
    .mins[1 as i32 as usize];
    (*info).mins[2 as i32 as usize] = (*crate::src::botlib::be_aas_main::aasworld
        .areas
        .offset(areanum as isize))
    .mins[2 as i32 as usize];
    (*info).maxs[0 as i32 as usize] = (*crate::src::botlib::be_aas_main::aasworld
        .areas
        .offset(areanum as isize))
    .maxs[0 as i32 as usize];
    (*info).maxs[1 as i32 as usize] = (*crate::src::botlib::be_aas_main::aasworld
        .areas
        .offset(areanum as isize))
    .maxs[1 as i32 as usize];
    (*info).maxs[2 as i32 as usize] = (*crate::src::botlib::be_aas_main::aasworld
        .areas
        .offset(areanum as isize))
    .maxs[2 as i32 as usize];
    (*info).center[0 as i32 as usize] = (*crate::src::botlib::be_aas_main::aasworld
        .areas
        .offset(areanum as isize))
    .center[0 as i32 as usize];
    (*info).center[1 as i32 as usize] = (*crate::src::botlib::be_aas_main::aasworld
        .areas
        .offset(areanum as isize))
    .center[1 as i32 as usize];
    (*info).center[2 as i32 as usize] = (*crate::src::botlib::be_aas_main::aasworld
        .areas
        .offset(areanum as isize))
    .center[2 as i32 as usize];
    return ::std::mem::size_of::<aas_areainfo_t>() as usize as i32;
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
 * name:		be_aas_sample.h
 *
 * desc:		AAS
 *
 * $Archive: /source/code/botlib/be_aas_sample.h $
 *
 *****************************************************************************/
/* ****************************************************************************
 * name:		be_aas_sample.h
 *
 * desc:		AAS
 *
 * $Archive: /source/code/botlib/be_aas_sample.h $
 *
 *****************************************************************************/
//end of the function AAS_AreaInfo
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_PlaneFromNum(mut planenum: i32) -> *mut aas_plane_t {
    if crate::src::botlib::be_aas_main::aasworld.loaded == 0 {
        return std::ptr::null_mut();
    }
    return &mut *crate::src::botlib::be_aas_main::aasworld
        .planes
        .offset(planenum as isize) as *mut aas_plane_t;
}
//end of the function AAS_PlaneFromNum

use ::libc;

pub use crate::be_aas_h::C2RustUnnamed_0;
pub use crate::cm_local_h::cArea_t;
pub use crate::cm_local_h::cLeaf_t;
pub use crate::cm_local_h::cNode_t;
pub use crate::cm_local_h::cPatch_t;
pub use crate::cm_local_h::cbrush_t;
pub use crate::cm_local_h::cbrushside_t;
pub use crate::cm_local_h::clipMap_t;
pub use crate::cm_local_h::cmodel_s;
pub use crate::cm_local_h::cmodel_t;
pub use crate::qfiles_h::dbrush_t;
pub use crate::qfiles_h::dbrushside_t;
pub use crate::qfiles_h::dheader_t;
pub use crate::qfiles_h::dleaf_t;
pub use crate::qfiles_h::dmodel_t;
pub use crate::qfiles_h::dnode_t;
pub use crate::qfiles_h::dplane_t;
pub use crate::qfiles_h::drawVert_t;
pub use crate::qfiles_h::dshader_t;
pub use crate::qfiles_h::dsurface_t;
pub use crate::qfiles_h::lump_t;
pub use crate::qfiles_h::MST_BAD;
pub use crate::qfiles_h::MST_FLARE;
pub use crate::qfiles_h::MST_PATCH;
pub use crate::qfiles_h::MST_PLANAR;
pub use crate::qfiles_h::MST_TRIANGLE_SOUP;
pub use crate::src::qcommon::cm_patch::patchCollide_s;
pub use crate::src::qcommon::cm_patch::CM_ClearLevelPatches;
pub use crate::src::qcommon::cm_patch::CM_GeneratePatchCollide;

pub use crate::src::qcommon::common::Com_Error;
pub use crate::src::qcommon::common::Hunk_Alloc;

pub use crate::src::qcommon::q_math::SetPlaneSignbits;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::clipHandle_t;
pub use crate::src::qcommon::q_shared::cplane_s;
pub use crate::src::qcommon::q_shared::cplane_t;
pub use crate::src::qcommon::q_shared::cvar_s;
pub use crate::src::qcommon::q_shared::cvar_t;
pub use crate::src::qcommon::q_shared::h_dontcare;
pub use crate::src::qcommon::q_shared::h_high;
pub use crate::src::qcommon::q_shared::h_low;
pub use crate::src::qcommon::q_shared::ha_pref;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::vec3_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::qcommon::q_shared::Q_strncpyz;
pub use crate::src::qcommon::q_shared::ERR_DISCONNECT;
pub use crate::src::qcommon::q_shared::ERR_DROP;
pub use crate::src::qcommon::q_shared::ERR_FATAL;
pub use crate::src::qcommon::q_shared::ERR_NEED_CD;
pub use crate::src::qcommon::q_shared::ERR_SERVERDISCONNECT;

extern "C" {
    #[no_mangle]
    pub fn CM_FloodAreaConnections();
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union C2RustUnnamed_111 {
    pub i: *mut i32,
    pub v: *mut libc::c_void,
}
#[no_mangle]

pub static mut cm: clipMap_t = clipMap_t {
    name: [0; 64],
    numShaders: 0,
    shaders: std::ptr::null_mut(),
    numBrushSides: 0,
    brushsides: std::ptr::null_mut(),
    numPlanes: 0,
    planes: std::ptr::null_mut(),
    numNodes: 0,
    nodes: std::ptr::null_mut(),
    numLeafs: 0,
    leafs: std::ptr::null_mut(),
    numLeafBrushes: 0,
    leafbrushes: std::ptr::null_mut(),
    numLeafSurfaces: 0,
    leafsurfaces: std::ptr::null_mut(),
    numSubModels: 0,
    cmodels: std::ptr::null_mut(),
    numBrushes: 0,
    brushes: std::ptr::null_mut(),
    numClusters: 0,
    clusterBytes: 0,
    visibility: std::ptr::null_mut(),
    vised: qfalse,
    numEntityChars: 0,
    entityString: std::ptr::null_mut(),
    numAreas: 0,
    areas: std::ptr::null_mut(),
    areaPortals: std::ptr::null_mut(),
    numSurfaces: 0,
    surfaces: std::ptr::null_mut(),
    floodvalid: 0,
    checkcount: 0,
};
#[no_mangle]

pub static mut c_pointcontents: i32 = 0;
#[no_mangle]

pub static mut c_traces: i32 = 0;
#[no_mangle]

pub static mut c_brush_traces: i32 = 0;
#[no_mangle]

pub static mut c_patch_traces: i32 = 0;
#[no_mangle]

pub static mut cmod_base: *mut byte = std::ptr::null_mut();
#[no_mangle]

pub static mut cm_noAreas: *mut cvar_t = std::ptr::null_mut();
#[no_mangle]

pub static mut cm_noCurves: *mut cvar_t = std::ptr::null_mut();
#[no_mangle]

pub static mut cm_playerCurveClip: *mut cvar_t = std::ptr::null_mut();
#[no_mangle]

pub static mut box_model: cmodel_t = cmodel_t {
    mins: [0.; 3],
    maxs: [0.; 3],
    leaf: cLeaf_t {
        cluster: 0,
        area: 0,
        firstLeafBrush: 0,
        numLeafBrushes: 0,
        firstLeafSurface: 0,
        numLeafSurfaces: 0,
    },
};
#[no_mangle]

pub static mut box_planes: *mut cplane_t = std::ptr::null_mut();
#[no_mangle]

pub static mut box_brush: *mut cbrush_t = std::ptr::null_mut();
/*
===============================================================================

                    MAP LOADING

===============================================================================
*/
/*
=================
CMod_LoadShaders
=================
*/
#[no_mangle]

pub unsafe extern "C" fn CMod_LoadShaders(mut l: *mut lump_t) {
    let mut in_0: *mut dshader_t = std::ptr::null_mut();
    let mut out: *mut dshader_t = std::ptr::null_mut();
    let mut i: i32 = 0;
    let mut count: i32 = 0;
    in_0 = cmod_base.offset((*l).fileofs as isize) as *mut libc::c_void as *mut dshader_t;
    if ((*l).filelen as usize).wrapping_rem(::std::mem::size_of::<dshader_t>() as usize) != 0 {
        Com_Error(
            ERR_DROP as i32,
            b"CMod_LoadShaders: funny lump size\x00" as *const u8 as *const libc::c_char,
        );
    }
    count =
        ((*l).filelen as usize).wrapping_div(::std::mem::size_of::<dshader_t>() as usize) as i32;
    if count < 1 as i32 {
        Com_Error(
            ERR_DROP as i32,
            b"Map with no shaders\x00" as *const u8 as *const libc::c_char,
        );
    }
    cm.shaders = Hunk_Alloc(
        (count as usize).wrapping_mul(::std::mem::size_of::<dshader_t>() as usize) as i32,
        h_high,
    ) as *mut dshader_t;
    cm.numShaders = count;
    crate::stdlib::memcpy(
        cm.shaders as *mut libc::c_void,
        in_0 as *const libc::c_void,
        (count as usize).wrapping_mul(::std::mem::size_of::<dshader_t>() as usize),
    );
    out = cm.shaders;
    i = 0 as i32;
    while i < count {
        (*out).contentFlags = (*out).contentFlags;
        (*out).surfaceFlags = (*out).surfaceFlags;
        i += 1;
        in_0 = in_0.offset(1);
        out = out.offset(1)
    }
}
/*
=================
CMod_LoadSubmodels
=================
*/
#[no_mangle]

pub unsafe extern "C" fn CMod_LoadSubmodels(mut l: *mut lump_t) {
    let mut in_0: *mut dmodel_t = std::ptr::null_mut();
    let mut out: *mut cmodel_t = std::ptr::null_mut();
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut count: i32 = 0;
    let mut indexes: *mut i32 = std::ptr::null_mut();
    in_0 = cmod_base.offset((*l).fileofs as isize) as *mut libc::c_void as *mut dmodel_t;
    if ((*l).filelen as usize).wrapping_rem(::std::mem::size_of::<dmodel_t>() as usize) != 0 {
        Com_Error(
            ERR_DROP as i32,
            b"CMod_LoadSubmodels: funny lump size\x00" as *const u8 as *const libc::c_char,
        );
    }
    count = ((*l).filelen as usize).wrapping_div(::std::mem::size_of::<dmodel_t>() as usize) as i32;
    if count < 1 as i32 {
        Com_Error(
            ERR_DROP as i32,
            b"Map with no models\x00" as *const u8 as *const libc::c_char,
        );
    }
    cm.cmodels = Hunk_Alloc(
        (count as usize).wrapping_mul(::std::mem::size_of::<cmodel_t>() as usize) as i32,
        h_high,
    ) as *mut cmodel_t;
    cm.numSubModels = count;
    if count > 256 as i32 {
        Com_Error(
            ERR_DROP as i32,
            b"MAX_SUBMODELS exceeded\x00" as *const u8 as *const libc::c_char,
        );
    }
    i = 0 as i32;
    while i < count {
        out = &mut *cm.cmodels.offset(i as isize) as *mut cmodel_t;
        j = 0 as i32;
        while j < 3 as i32 {
            // spread the mins / maxs by a pixel
            (*out).mins[j as usize] = (*in_0).mins[j as usize] - 1 as i32 as f32;
            (*out).maxs[j as usize] = (*in_0).maxs[j as usize] + 1 as i32 as f32;
            j += 1
        }
        if !(i == 0 as i32) {
            // make a "leaf" just to hold the model's brushes and surfaces
            (*out).leaf.numLeafBrushes = (*in_0).numBrushes;
            indexes = Hunk_Alloc((*out).leaf.numLeafBrushes * 4 as i32, h_high) as *mut i32;
            (*out).leaf.firstLeafBrush = indexes.offset_from(cm.leafbrushes) as isize as i32;
            j = 0 as i32;
            while j < (*out).leaf.numLeafBrushes {
                *indexes.offset(j as isize) = (*in_0).firstBrush + j;
                j += 1
            }
            (*out).leaf.numLeafSurfaces = (*in_0).numSurfaces;
            indexes = Hunk_Alloc((*out).leaf.numLeafSurfaces * 4 as i32, h_high) as *mut i32;
            (*out).leaf.firstLeafSurface = indexes.offset_from(cm.leafsurfaces) as isize as i32;
            j = 0 as i32;
            while j < (*out).leaf.numLeafSurfaces {
                *indexes.offset(j as isize) = (*in_0).firstSurface + j;
                j += 1
            }
        }
        i += 1;
        in_0 = in_0.offset(1)
        // world model doesn't need other info
    }
}
/*
=================
CMod_LoadNodes

=================
*/
#[no_mangle]

pub unsafe extern "C" fn CMod_LoadNodes(mut l: *mut lump_t) {
    let mut in_0: *mut dnode_t = std::ptr::null_mut();
    let mut child: i32 = 0;
    let mut out: *mut cNode_t = std::ptr::null_mut();
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut count: i32 = 0;
    in_0 = cmod_base.offset((*l).fileofs as isize) as *mut libc::c_void as *mut dnode_t;
    if ((*l).filelen as usize).wrapping_rem(::std::mem::size_of::<dnode_t>() as usize) != 0 {
        Com_Error(
            ERR_DROP as i32,
            b"MOD_LoadBmodel: funny lump size\x00" as *const u8 as *const libc::c_char,
        );
    }
    count = ((*l).filelen as usize).wrapping_div(::std::mem::size_of::<dnode_t>() as usize) as i32;
    if count < 1 as i32 {
        Com_Error(
            ERR_DROP as i32,
            b"Map has no nodes\x00" as *const u8 as *const libc::c_char,
        );
    }
    cm.nodes = Hunk_Alloc(
        (count as usize).wrapping_mul(::std::mem::size_of::<cNode_t>() as usize) as i32,
        h_high,
    ) as *mut cNode_t;
    cm.numNodes = count;
    out = cm.nodes;
    i = 0 as i32;
    while i < count {
        (*out).plane = cm.planes.offset((*in_0).planeNum as isize);
        j = 0 as i32;
        while j < 2 as i32 {
            child = (*in_0).children[j as usize];
            (*out).children[j as usize] = child;
            j += 1
        }
        i += 1;
        out = out.offset(1);
        in_0 = in_0.offset(1)
    }
}
/*
=================
CM_BoundBrush

=================
*/
#[no_mangle]

pub unsafe extern "C" fn CM_BoundBrush(mut b: *mut cbrush_t) {
    (*b).bounds[0 as i32 as usize][0 as i32 as usize] =
        -(*(*(*b).sides.offset(0 as i32 as isize)).plane).dist;
    (*b).bounds[1 as i32 as usize][0 as i32 as usize] =
        (*(*(*b).sides.offset(1 as i32 as isize)).plane).dist;
    (*b).bounds[0 as i32 as usize][1 as i32 as usize] =
        -(*(*(*b).sides.offset(2 as i32 as isize)).plane).dist;
    (*b).bounds[1 as i32 as usize][1 as i32 as usize] =
        (*(*(*b).sides.offset(3 as i32 as isize)).plane).dist;
    (*b).bounds[0 as i32 as usize][2 as i32 as usize] =
        -(*(*(*b).sides.offset(4 as i32 as isize)).plane).dist;
    (*b).bounds[1 as i32 as usize][2 as i32 as usize] =
        (*(*(*b).sides.offset(5 as i32 as isize)).plane).dist;
}
/*
=================
CMod_LoadBrushes

=================
*/
#[no_mangle]

pub unsafe extern "C" fn CMod_LoadBrushes(mut l: *mut lump_t) {
    let mut in_0: *mut dbrush_t = std::ptr::null_mut();
    let mut out: *mut cbrush_t = std::ptr::null_mut();
    let mut i: i32 = 0;
    let mut count: i32 = 0;
    in_0 = cmod_base.offset((*l).fileofs as isize) as *mut libc::c_void as *mut dbrush_t;
    if ((*l).filelen as usize).wrapping_rem(::std::mem::size_of::<dbrush_t>() as usize) != 0 {
        Com_Error(
            ERR_DROP as i32,
            b"MOD_LoadBmodel: funny lump size\x00" as *const u8 as *const libc::c_char,
        );
    }
    count = ((*l).filelen as usize).wrapping_div(::std::mem::size_of::<dbrush_t>() as usize) as i32;
    cm.brushes = Hunk_Alloc(
        ((1 as i32 + count) as usize).wrapping_mul(::std::mem::size_of::<cbrush_t>() as usize)
            as i32,
        h_high,
    ) as *mut cbrush_t;
    cm.numBrushes = count;
    out = cm.brushes;
    i = 0 as i32;
    while i < count {
        (*out).sides = cm.brushsides.offset((*in_0).firstSide as isize);
        (*out).numsides = (*in_0).numSides;
        (*out).shaderNum = (*in_0).shaderNum;
        if (*out).shaderNum < 0 as i32 || (*out).shaderNum >= cm.numShaders {
            Com_Error(
                ERR_DROP as i32,
                b"CMod_LoadBrushes: bad shaderNum: %i\x00" as *const u8 as *const libc::c_char,
                (*out).shaderNum,
            );
        }
        (*out).contents = (*cm.shaders.offset((*out).shaderNum as isize)).contentFlags;
        CM_BoundBrush(out);
        i += 1;
        out = out.offset(1);
        in_0 = in_0.offset(1)
    }
}
/*
=================
CMod_LoadLeafs
=================
*/
#[no_mangle]

pub unsafe extern "C" fn CMod_LoadLeafs(mut l: *mut lump_t) {
    let mut i: i32 = 0;
    let mut out: *mut cLeaf_t = std::ptr::null_mut();
    let mut in_0: *mut dleaf_t = std::ptr::null_mut();
    let mut count: i32 = 0;
    in_0 = cmod_base.offset((*l).fileofs as isize) as *mut libc::c_void as *mut dleaf_t;
    if ((*l).filelen as usize).wrapping_rem(::std::mem::size_of::<dleaf_t>() as usize) != 0 {
        Com_Error(
            ERR_DROP as i32,
            b"MOD_LoadBmodel: funny lump size\x00" as *const u8 as *const libc::c_char,
        );
    }
    count = ((*l).filelen as usize).wrapping_div(::std::mem::size_of::<dleaf_t>() as usize) as i32;
    if count < 1 as i32 {
        Com_Error(
            ERR_DROP as i32,
            b"Map with no leafs\x00" as *const u8 as *const libc::c_char,
        );
    }
    cm.leafs = Hunk_Alloc(
        ((2 as i32 + count) as usize).wrapping_mul(::std::mem::size_of::<cLeaf_t>() as usize)
            as i32,
        h_high,
    ) as *mut cLeaf_t;
    cm.numLeafs = count;
    out = cm.leafs;
    i = 0 as i32;
    while i < count {
        (*out).cluster = (*in_0).cluster;
        (*out).area = (*in_0).area;
        (*out).firstLeafBrush = (*in_0).firstLeafBrush;
        (*out).numLeafBrushes = (*in_0).numLeafBrushes;
        (*out).firstLeafSurface = (*in_0).firstLeafSurface;
        (*out).numLeafSurfaces = (*in_0).numLeafSurfaces;
        if (*out).cluster >= cm.numClusters {
            cm.numClusters = (*out).cluster + 1 as i32
        }
        if (*out).area >= cm.numAreas {
            cm.numAreas = (*out).area + 1 as i32
        }
        i += 1;
        in_0 = in_0.offset(1);
        out = out.offset(1)
    }
    cm.areas = Hunk_Alloc(
        (cm.numAreas as usize).wrapping_mul(::std::mem::size_of::<cArea_t>() as usize) as i32,
        h_high,
    ) as *mut cArea_t;
    cm.areaPortals = Hunk_Alloc(
        ((cm.numAreas * cm.numAreas) as usize).wrapping_mul(::std::mem::size_of::<i32>() as usize)
            as i32,
        h_high,
    ) as *mut i32;
}
/*
=================
CMod_LoadPlanes
=================
*/
#[no_mangle]

pub unsafe extern "C" fn CMod_LoadPlanes(mut l: *mut lump_t) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut out: *mut cplane_t = std::ptr::null_mut();
    let mut in_0: *mut dplane_t = std::ptr::null_mut();
    let mut count: i32 = 0;
    let mut bits: i32 = 0;
    in_0 = cmod_base.offset((*l).fileofs as isize) as *mut libc::c_void as *mut dplane_t;
    if ((*l).filelen as usize).wrapping_rem(::std::mem::size_of::<dplane_t>() as usize) != 0 {
        Com_Error(
            ERR_DROP as i32,
            b"MOD_LoadBmodel: funny lump size\x00" as *const u8 as *const libc::c_char,
        );
    }
    count = ((*l).filelen as usize).wrapping_div(::std::mem::size_of::<dplane_t>() as usize) as i32;
    if count < 1 as i32 {
        Com_Error(
            ERR_DROP as i32,
            b"Map with no planes\x00" as *const u8 as *const libc::c_char,
        );
    }
    cm.planes = Hunk_Alloc(
        ((12 as i32 + count) as usize).wrapping_mul(::std::mem::size_of::<cplane_t>() as usize)
            as i32,
        h_high,
    ) as *mut cplane_t;
    cm.numPlanes = count;
    out = cm.planes;
    i = 0 as i32;
    while i < count {
        bits = 0 as i32;
        j = 0 as i32;
        while j < 3 as i32 {
            (*out).normal[j as usize] = (*in_0).normal[j as usize];
            if (*out).normal[j as usize] < 0 as i32 as f32 {
                bits |= (1 as i32) << j
            }
            j += 1
        }
        (*out).dist = (*in_0).dist;
        (*out).type_0 = if (*out).normal[0 as i32 as usize] as f64 == 1.0f64 {
            0 as i32
        } else if (*out).normal[1 as i32 as usize] as f64 == 1.0f64 {
            1 as i32
        } else if (*out).normal[2 as i32 as usize] as f64 == 1.0f64 {
            2 as i32
        } else {
            3 as i32
        } as byte;
        (*out).signbits = bits as byte;
        i += 1;
        in_0 = in_0.offset(1);
        out = out.offset(1)
    }
}
/*
=================
CMod_LoadLeafBrushes
=================
*/
#[no_mangle]

pub unsafe extern "C" fn CMod_LoadLeafBrushes(mut l: *mut lump_t) {
    let mut i: i32 = 0;
    let mut out: *mut i32 = std::ptr::null_mut();
    let mut in_0: *mut i32 = std::ptr::null_mut();
    let mut count: i32 = 0;
    in_0 = cmod_base.offset((*l).fileofs as isize) as *mut libc::c_void as *mut i32;
    if ((*l).filelen as usize).wrapping_rem(::std::mem::size_of::<i32>() as usize) != 0 {
        Com_Error(
            ERR_DROP as i32,
            b"MOD_LoadBmodel: funny lump size\x00" as *const u8 as *const libc::c_char,
        );
    }
    count = ((*l).filelen as usize).wrapping_div(::std::mem::size_of::<i32>() as usize) as i32;
    cm.leafbrushes = Hunk_Alloc(
        ((count + 1 as i32) as usize).wrapping_mul(::std::mem::size_of::<i32>() as usize) as i32,
        h_high,
    ) as *mut i32;
    cm.numLeafBrushes = count;
    out = cm.leafbrushes;
    i = 0 as i32;
    while i < count {
        *out = *in_0;
        i += 1;
        in_0 = in_0.offset(1);
        out = out.offset(1)
    }
}
/*
=================
CMod_LoadLeafSurfaces
=================
*/
#[no_mangle]

pub unsafe extern "C" fn CMod_LoadLeafSurfaces(mut l: *mut lump_t) {
    let mut i: i32 = 0;
    let mut out: *mut i32 = std::ptr::null_mut();
    let mut in_0: *mut i32 = std::ptr::null_mut();
    let mut count: i32 = 0;
    in_0 = cmod_base.offset((*l).fileofs as isize) as *mut libc::c_void as *mut i32;
    if ((*l).filelen as usize).wrapping_rem(::std::mem::size_of::<i32>() as usize) != 0 {
        Com_Error(
            ERR_DROP as i32,
            b"MOD_LoadBmodel: funny lump size\x00" as *const u8 as *const libc::c_char,
        );
    }
    count = ((*l).filelen as usize).wrapping_div(::std::mem::size_of::<i32>() as usize) as i32;
    cm.leafsurfaces = Hunk_Alloc(
        (count as usize).wrapping_mul(::std::mem::size_of::<i32>() as usize) as i32,
        h_high,
    ) as *mut i32;
    cm.numLeafSurfaces = count;
    out = cm.leafsurfaces;
    i = 0 as i32;
    while i < count {
        *out = *in_0;
        i += 1;
        in_0 = in_0.offset(1);
        out = out.offset(1)
    }
}
/*
=================
CMod_LoadBrushSides
=================
*/
#[no_mangle]

pub unsafe extern "C" fn CMod_LoadBrushSides(mut l: *mut lump_t) {
    let mut i: i32 = 0;
    let mut out: *mut cbrushside_t = std::ptr::null_mut();
    let mut in_0: *mut dbrushside_t = std::ptr::null_mut();
    let mut count: i32 = 0;
    let mut num: i32 = 0;
    in_0 = cmod_base.offset((*l).fileofs as isize) as *mut libc::c_void as *mut dbrushside_t;
    if ((*l).filelen as usize).wrapping_rem(::std::mem::size_of::<dbrushside_t>() as usize) != 0 {
        Com_Error(
            ERR_DROP as i32,
            b"MOD_LoadBmodel: funny lump size\x00" as *const u8 as *const libc::c_char,
        );
    }
    count =
        ((*l).filelen as usize).wrapping_div(::std::mem::size_of::<dbrushside_t>() as usize) as i32;
    cm.brushsides = Hunk_Alloc(
        ((6 as i32 + count) as usize).wrapping_mul(::std::mem::size_of::<cbrushside_t>() as usize)
            as i32,
        h_high,
    ) as *mut cbrushside_t;
    cm.numBrushSides = count;
    out = cm.brushsides;
    i = 0 as i32;
    while i < count {
        num = (*in_0).planeNum;
        (*out).plane = &mut *cm.planes.offset(num as isize) as *mut cplane_t;
        (*out).shaderNum = (*in_0).shaderNum;
        if (*out).shaderNum < 0 as i32 || (*out).shaderNum >= cm.numShaders {
            Com_Error(
                ERR_DROP as i32,
                b"CMod_LoadBrushSides: bad shaderNum: %i\x00" as *const u8 as *const libc::c_char,
                (*out).shaderNum,
            );
        }
        (*out).surfaceFlags = (*cm.shaders.offset((*out).shaderNum as isize)).surfaceFlags;
        i += 1;
        in_0 = in_0.offset(1);
        out = out.offset(1)
    }
}
/*
=================
CMod_LoadEntityString
=================
*/
#[no_mangle]

pub unsafe extern "C" fn CMod_LoadEntityString(mut l: *mut lump_t) {
    cm.entityString = Hunk_Alloc((*l).filelen, h_high) as *mut libc::c_char;
    cm.numEntityChars = (*l).filelen;
    crate::stdlib::memcpy(
        cm.entityString as *mut libc::c_void,
        cmod_base.offset((*l).fileofs as isize) as *const libc::c_void,
        (*l).filelen as usize,
    );
}
#[no_mangle]

pub unsafe extern "C" fn CMod_LoadVisibility(mut l: *mut lump_t) {
    let mut len: i32 = 0;
    let mut buf: *mut byte = std::ptr::null_mut();
    len = (*l).filelen;
    if len == 0 {
        cm.clusterBytes = cm.numClusters + 31 as i32 & !(31 as i32);
        cm.visibility = Hunk_Alloc(cm.clusterBytes, h_high) as *mut byte;
        crate::stdlib::memset(
            cm.visibility as *mut libc::c_void,
            255 as i32,
            cm.clusterBytes as usize,
        );
        return;
    }
    buf = cmod_base.offset((*l).fileofs as isize);
    cm.vised = qtrue;
    cm.visibility = Hunk_Alloc(len, h_high) as *mut byte;
    cm.numClusters = *(buf as *mut i32).offset(0 as i32 as isize);
    cm.clusterBytes = *(buf as *mut i32).offset(1 as i32 as isize);
    crate::stdlib::memcpy(
        cm.visibility as *mut libc::c_void,
        buf.offset(8 as i32 as isize) as *const libc::c_void,
        (len - 8 as i32) as usize,
    );
}
#[no_mangle]

pub unsafe extern "C" fn CMod_LoadPatches(mut surfs: *mut lump_t, mut verts: *mut lump_t) {
    let mut dv: *mut drawVert_t = std::ptr::null_mut();
    let mut dv_p: *mut drawVert_t = std::ptr::null_mut();
    let mut in_0: *mut dsurface_t = std::ptr::null_mut();
    let mut count: i32 = 0;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut c: i32 = 0;
    let mut patch: *mut cPatch_t = std::ptr::null_mut();
    let mut points: [vec3_t; 1024] = [[0.; 3]; 1024];
    let mut width: i32 = 0;
    let mut height: i32 = 0;
    let mut shaderNum: i32 = 0;
    in_0 = cmod_base.offset((*surfs).fileofs as isize) as *mut libc::c_void as *mut dsurface_t;
    if ((*surfs).filelen as usize).wrapping_rem(::std::mem::size_of::<dsurface_t>() as usize) != 0 {
        Com_Error(
            ERR_DROP as i32,
            b"MOD_LoadBmodel: funny lump size\x00" as *const u8 as *const libc::c_char,
        );
    }
    count = ((*surfs).filelen as usize).wrapping_div(::std::mem::size_of::<dsurface_t>() as usize)
        as i32;
    cm.numSurfaces = count;
    cm.surfaces = Hunk_Alloc(
        (cm.numSurfaces as usize).wrapping_mul(::std::mem::size_of::<*mut cPatch_t>() as usize)
            as i32,
        h_high,
    ) as *mut *mut cPatch_t;
    dv = cmod_base.offset((*verts).fileofs as isize) as *mut libc::c_void as *mut drawVert_t;
    if ((*verts).filelen as usize).wrapping_rem(::std::mem::size_of::<drawVert_t>() as usize) != 0 {
        Com_Error(
            ERR_DROP as i32,
            b"MOD_LoadBmodel: funny lump size\x00" as *const u8 as *const libc::c_char,
        );
    }
    // scan through all the surfaces, but only load patches,
    // not planar faces
    i = 0 as i32;
    while i < count {
        if !((*in_0).surfaceType != MST_PATCH as i32) {
            // FIXME: check for non-colliding patches
            patch = Hunk_Alloc(::std::mem::size_of::<cPatch_t>() as usize as i32, h_high)
                as *mut cPatch_t;
            let ref mut fresh0 = *cm.surfaces.offset(i as isize);
            *fresh0 = patch;
            // load the full drawverts onto the stack
            width = (*in_0).patchWidth;
            height = (*in_0).patchHeight;
            c = width * height;
            if c > 1024 as i32 {
                Com_Error(
                    ERR_DROP as i32,
                    b"ParseMesh: MAX_PATCH_VERTS\x00" as *const u8 as *const libc::c_char,
                );
            }
            dv_p = dv.offset((*in_0).firstVert as isize);
            j = 0 as i32;
            while j < c {
                points[j as usize][0 as i32 as usize] = (*dv_p).xyz[0 as i32 as usize];
                points[j as usize][1 as i32 as usize] = (*dv_p).xyz[1 as i32 as usize];
                points[j as usize][2 as i32 as usize] = (*dv_p).xyz[2 as i32 as usize];
                j += 1;
                dv_p = dv_p.offset(1)
            }
            shaderNum = (*in_0).shaderNum;
            (*patch).contents = (*cm.shaders.offset(shaderNum as isize)).contentFlags;
            (*patch).surfaceFlags = (*cm.shaders.offset(shaderNum as isize)).surfaceFlags;
            // create the internal facet structure
            (*patch).pc = CM_GeneratePatchCollide(width, height, points.as_mut_ptr())
        }
        i += 1;
        in_0 = in_0.offset(1)
        // ignore other surfaces
    }
}
//==================================================================
#[no_mangle]

pub unsafe extern "C" fn CM_LumpChecksum(mut lump: *mut lump_t) -> u32 {
    return crate::src::qcommon::md4::Com_BlockChecksum(
        cmod_base.offset((*lump).fileofs as isize) as *const libc::c_void,
        (*lump).filelen,
    );
}
#[no_mangle]

pub unsafe extern "C" fn CM_Checksum(mut header: *mut dheader_t) -> u32 {
    let mut checksums: [u32; 16] = [0; 16];
    checksums[0 as i32 as usize] =
        CM_LumpChecksum(&mut *(*header).lumps.as_mut_ptr().offset(1 as i32 as isize));
    checksums[1 as i32 as usize] =
        CM_LumpChecksum(&mut *(*header).lumps.as_mut_ptr().offset(4 as i32 as isize));
    checksums[2 as i32 as usize] =
        CM_LumpChecksum(&mut *(*header).lumps.as_mut_ptr().offset(6 as i32 as isize));
    checksums[3 as i32 as usize] =
        CM_LumpChecksum(&mut *(*header).lumps.as_mut_ptr().offset(5 as i32 as isize));
    checksums[4 as i32 as usize] =
        CM_LumpChecksum(&mut *(*header).lumps.as_mut_ptr().offset(2 as i32 as isize));
    checksums[5 as i32 as usize] =
        CM_LumpChecksum(&mut *(*header).lumps.as_mut_ptr().offset(9 as i32 as isize));
    checksums[6 as i32 as usize] =
        CM_LumpChecksum(&mut *(*header).lumps.as_mut_ptr().offset(8 as i32 as isize));
    checksums[7 as i32 as usize] =
        CM_LumpChecksum(&mut *(*header).lumps.as_mut_ptr().offset(7 as i32 as isize));
    checksums[8 as i32 as usize] =
        CM_LumpChecksum(&mut *(*header).lumps.as_mut_ptr().offset(3 as i32 as isize));
    checksums[9 as i32 as usize] =
        CM_LumpChecksum(&mut *(*header).lumps.as_mut_ptr().offset(13 as i32 as isize));
    checksums[10 as i32 as usize] =
        CM_LumpChecksum(&mut *(*header).lumps.as_mut_ptr().offset(10 as i32 as isize));
    return crate::src::qcommon::md4::Com_BlockChecksum(
        checksums.as_mut_ptr() as *const libc::c_void,
        11 as i32 * 4 as i32,
    );
}
/*
==================
CM_LoadMap

Loads in the map and all submodels
==================
*/
#[no_mangle]

pub unsafe extern "C" fn CM_LoadMap(
    mut name: *const libc::c_char,
    mut clientload: qboolean,
    mut checksum: *mut i32,
) {
    let mut buf: C2RustUnnamed_111 = C2RustUnnamed_111 {
        i: std::ptr::null_mut()}
    ;
    let mut i: i32 = 0;
    let mut header: dheader_t = dheader_t {
        ident: 0,
        version: 0,
        lumps: [lump_t {
            fileofs: 0,
            filelen: 0,
        }; 17],
    };
    let mut length: i32 = 0;
    static mut last_checksum: u32 = 0;
    if name.is_null() || *name.offset(0 as i32 as isize) == 0 {
        Com_Error(
            ERR_DROP as i32,
            b"CM_LoadMap: NULL name\x00" as *const u8 as *const libc::c_char,
        );
    }
    cm_noAreas = crate::src::qcommon::cvar::Cvar_Get(
        b"cm_noAreas\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x200 as i32,
    ) as *mut cvar_s;
    cm_noCurves = crate::src::qcommon::cvar::Cvar_Get(
        b"cm_noCurves\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x200 as i32,
    ) as *mut cvar_s;
    cm_playerCurveClip = crate::src::qcommon::cvar::Cvar_Get(
        b"cm_playerCurveClip\x00" as *const u8 as *const libc::c_char,
        b"1\x00" as *const u8 as *const libc::c_char,
        0x1 as i32 | 0x200 as i32,
    ) as *mut cvar_s;
    crate::src::qcommon::common::Com_DPrintf(
        b"CM_LoadMap( %s, %i )\n\x00" as *const u8 as *const libc::c_char,
        name,
        clientload as u32,
    );
    if libc::strcmp(cm.name.as_mut_ptr(), name) == 0 && clientload as u32 != 0 {
        *checksum = last_checksum as i32;
        return;
    }
    // free old stuff
    crate::stdlib::memset(
        &mut cm as *mut clipMap_t as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<clipMap_t>() as usize,
    );
    CM_ClearLevelPatches();
    if *name.offset(0 as i32 as isize) == 0 {
        cm.numLeafs = 1 as i32;
        cm.numClusters = 1 as i32;
        cm.numAreas = 1 as i32;
        cm.cmodels =
            Hunk_Alloc(::std::mem::size_of::<cmodel_t>() as usize as i32, h_high) as *mut cmodel_t;
        *checksum = 0 as i32;
        return;
    }
    //
    // load the file
    //
    length = crate::src::qcommon::files::FS_ReadFile(name, &mut buf.v) as i32;
    if buf.i.is_null() {
        Com_Error(
            ERR_DROP as i32,
            b"Couldn\'t load %s\x00" as *const u8 as *const libc::c_char,
            name,
        );
    }
    last_checksum =
        crate::src::qcommon::md4::Com_BlockChecksum(buf.i as *const libc::c_void, length);
    *checksum = last_checksum as i32;
    header = *(buf.i as *mut dheader_t);
    i = 0 as i32;
    while (i as usize)
        < (::std::mem::size_of::<dheader_t>() as usize).wrapping_div(4 as i32 as usize)
    {
        *(&mut header as *mut dheader_t as *mut i32).offset(i as isize) =
            *(&mut header as *mut dheader_t as *mut i32).offset(i as isize);
        i += 1
    }
    if header.version != 46 as i32 {
        Com_Error(
            ERR_DROP as i32,
            b"CM_LoadMap: %s has wrong version number (%i should be %i)\x00" as *const u8
                as *const libc::c_char,
            name,
            header.version,
            46 as i32,
        );
    }
    cmod_base = buf.i as *mut byte;
    // load into heap
    CMod_LoadShaders(&mut *header.lumps.as_mut_ptr().offset(1 as i32 as isize));
    CMod_LoadLeafs(&mut *header.lumps.as_mut_ptr().offset(4 as i32 as isize));
    CMod_LoadLeafBrushes(&mut *header.lumps.as_mut_ptr().offset(6 as i32 as isize));
    CMod_LoadLeafSurfaces(&mut *header.lumps.as_mut_ptr().offset(5 as i32 as isize));
    CMod_LoadPlanes(&mut *header.lumps.as_mut_ptr().offset(2 as i32 as isize));
    CMod_LoadBrushSides(&mut *header.lumps.as_mut_ptr().offset(9 as i32 as isize));
    CMod_LoadBrushes(&mut *header.lumps.as_mut_ptr().offset(8 as i32 as isize));
    CMod_LoadSubmodels(&mut *header.lumps.as_mut_ptr().offset(7 as i32 as isize));
    CMod_LoadNodes(&mut *header.lumps.as_mut_ptr().offset(3 as i32 as isize));
    CMod_LoadEntityString(&mut *header.lumps.as_mut_ptr().offset(0 as i32 as isize));
    CMod_LoadVisibility(&mut *header.lumps.as_mut_ptr().offset(16 as i32 as isize));
    CMod_LoadPatches(
        &mut *header.lumps.as_mut_ptr().offset(13 as i32 as isize),
        &mut *header.lumps.as_mut_ptr().offset(10 as i32 as isize),
    );
    // we are NOT freeing the file, because it is cached for the ref
    crate::src::qcommon::files::FS_FreeFile(buf.v);
    CM_InitBoxHull();
    CM_FloodAreaConnections();
    // allow this to be cached if it is loaded by the server
    if clientload as u64 == 0 {
        Q_strncpyz(
            cm.name.as_mut_ptr(),
            name,
            ::std::mem::size_of::<[libc::c_char; 64]>() as usize as i32,
        );
    };
}
/*
==================
CM_ClearMap
==================
*/
#[no_mangle]

pub unsafe extern "C" fn CM_ClearMap() {
    crate::stdlib::memset(
        &mut cm as *mut clipMap_t as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<clipMap_t>() as usize,
    );
    CM_ClearLevelPatches();
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
// negative numbers are leafs
// submodels don't reference the main tree
// the shader that determined the contents
// to avoid repeated testings
// to avoid repeated testings
// if false, visibility is just a single cluster of ffs
// [ numAreas*numAreas ] reference counts
// non-patches will be NULL
// incremented on each trace
// keep 1/8 unit away to keep the position valid before network snapping
// and to avoid various numeric issues
// cm_test.c
// Used for oriented capsule collision detection
// size of the box being swept through the model
// [signbits][x] = either size[0][x] or size[1][x]
// longest corner length from origin
// greatest of abs(size[0]) and abs(size[1])
// enclosing box of start and end surrounding by size
// origin of the model tracing through
// ored contents of the model tracing through
// optimized case
// returned from trace call
// sphere for oriendted capsule collision
// for overflows where each leaf can't be stored individually
/*
==================
CM_ClipHandleToModel
==================
*/
#[no_mangle]

pub unsafe extern "C" fn CM_ClipHandleToModel(mut handle: clipHandle_t) -> *mut cmodel_t {
    if handle < 0 as i32 {
        Com_Error(
            ERR_DROP as i32,
            b"CM_ClipHandleToModel: bad handle %i\x00" as *const u8 as *const libc::c_char,
            handle,
        );
    }
    if handle < cm.numSubModels {
        return &mut *cm.cmodels.offset(handle as isize) as *mut cmodel_t;
    }
    if handle == 255 as i32 {
        return &mut box_model;
    }
    if handle < 256 as i32 {
        Com_Error(
            ERR_DROP as i32,
            b"CM_ClipHandleToModel: bad handle %i < %i < %i\x00" as *const u8
                as *const libc::c_char,
            cm.numSubModels,
            handle,
            256 as i32,
        );
    }
    Com_Error(
        ERR_DROP as i32,
        b"CM_ClipHandleToModel: bad handle %i\x00" as *const u8 as *const libc::c_char,
        handle + 256 as i32,
    );
}
/*
==================
CM_InlineModel
==================
*/
#[no_mangle]

pub unsafe extern "C" fn CM_InlineModel(mut index: i32) -> clipHandle_t {
    if index < 0 as i32 || index >= cm.numSubModels {
        Com_Error(
            ERR_DROP as i32,
            b"CM_InlineModel: bad number\x00" as *const u8 as *const libc::c_char,
        );
    }
    return index;
}
#[no_mangle]

pub unsafe extern "C" fn CM_NumClusters() -> i32 {
    return cm.numClusters;
}
#[no_mangle]

pub unsafe extern "C" fn CM_NumInlineModels() -> i32 {
    return cm.numSubModels;
}
#[no_mangle]

pub unsafe extern "C" fn CM_EntityString() -> *mut libc::c_char {
    return cm.entityString;
}
#[no_mangle]

pub unsafe extern "C" fn CM_LeafCluster(mut leafnum: i32) -> i32 {
    if leafnum < 0 as i32 || leafnum >= cm.numLeafs {
        Com_Error(
            ERR_DROP as i32,
            b"CM_LeafCluster: bad number\x00" as *const u8 as *const libc::c_char,
        );
    }
    return (*cm.leafs.offset(leafnum as isize)).cluster;
}
// returns an ORed contents mask
// only returns non-solid leafs
// overflow if return listsize and if *lastLeaf != list[listsize-1]
#[no_mangle]

pub unsafe extern "C" fn CM_LeafArea(mut leafnum: i32) -> i32 {
    if leafnum < 0 as i32 || leafnum >= cm.numLeafs {
        Com_Error(
            ERR_DROP as i32,
            b"CM_LeafArea: bad number\x00" as *const u8 as *const libc::c_char,
        );
    }
    return (*cm.leafs.offset(leafnum as isize)).area;
}
//=======================================================================
/*
===================
CM_InitBoxHull

Set up the planes and nodes so that the six floats of a bounding box
can just be stored out and get a proper clipping hull structure.
===================
*/
#[no_mangle]

pub unsafe extern "C" fn CM_InitBoxHull() {
    let mut i: i32 = 0;
    let mut side: i32 = 0;
    let mut p: *mut cplane_t = std::ptr::null_mut();
    let mut s: *mut cbrushside_t = std::ptr::null_mut();
    box_planes = &mut *cm.planes.offset(cm.numPlanes as isize) as *mut cplane_t;
    box_brush = &mut *cm.brushes.offset(cm.numBrushes as isize) as *mut cbrush_t;
    (*box_brush).numsides = 6 as i32;
    (*box_brush).sides = cm.brushsides.offset(cm.numBrushSides as isize);
    (*box_brush).contents = 0x2000000 as i32;
    box_model.leaf.numLeafBrushes = 1 as i32;
    //	box_model.leaf.firstLeafBrush = cm.numBrushes;
    box_model.leaf.firstLeafBrush = cm.numLeafBrushes;
    *cm.leafbrushes.offset(cm.numLeafBrushes as isize) = cm.numBrushes;
    i = 0 as i32;
    while i < 6 as i32 {
        side = i & 1 as i32;
        // brush sides
        s = &mut *cm.brushsides.offset((cm.numBrushSides + i) as isize) as *mut cbrushside_t;
        (*s).plane = cm
            .planes
            .offset((cm.numPlanes + i * 2 as i32 + side) as isize);
        (*s).surfaceFlags = 0 as i32;
        // planes
        p = &mut *box_planes.offset((i * 2 as i32) as isize) as *mut cplane_t;
        (*p).type_0 = (i >> 1 as i32) as byte;
        (*p).signbits = 0 as i32 as byte;
        (*p).normal[2 as i32 as usize] = 0 as i32 as vec_t;
        (*p).normal[1 as i32 as usize] = (*p).normal[2 as i32 as usize];
        (*p).normal[0 as i32 as usize] = (*p).normal[1 as i32 as usize];
        (*p).normal[(i >> 1 as i32) as usize] = 1 as i32 as vec_t;
        p = &mut *box_planes.offset((i * 2 as i32 + 1 as i32) as isize) as *mut cplane_t;
        (*p).type_0 = (3 as i32 + (i >> 1 as i32)) as byte;
        (*p).signbits = 0 as i32 as byte;
        (*p).normal[2 as i32 as usize] = 0 as i32 as vec_t;
        (*p).normal[1 as i32 as usize] = (*p).normal[2 as i32 as usize];
        (*p).normal[0 as i32 as usize] = (*p).normal[1 as i32 as usize];
        (*p).normal[(i >> 1 as i32) as usize] = -(1 as i32) as vec_t;
        SetPlaneSignbits(p as *mut cplane_s);
        i += 1
    }
}
/*
===================
CM_TempBoxModel

To keep everything totally uniform, bounding boxes are turned into small
BSP trees instead of being compared directly.
Capsules are handled differently though.
===================
*/
#[no_mangle]

pub unsafe extern "C" fn CM_TempBoxModel(
    mut mins: *const vec_t,
    mut maxs: *const vec_t,
    mut capsule: i32,
) -> clipHandle_t {
    box_model.mins[0 as i32 as usize] = *mins.offset(0 as i32 as isize);
    box_model.mins[1 as i32 as usize] = *mins.offset(1 as i32 as isize);
    box_model.mins[2 as i32 as usize] = *mins.offset(2 as i32 as isize);
    box_model.maxs[0 as i32 as usize] = *maxs.offset(0 as i32 as isize);
    box_model.maxs[1 as i32 as usize] = *maxs.offset(1 as i32 as isize);
    box_model.maxs[2 as i32 as usize] = *maxs.offset(2 as i32 as isize);
    if capsule != 0 {
        return 254 as i32;
    }
    (*box_planes.offset(0 as i32 as isize)).dist = *maxs.offset(0 as i32 as isize);
    (*box_planes.offset(1 as i32 as isize)).dist = -*maxs.offset(0 as i32 as isize);
    (*box_planes.offset(2 as i32 as isize)).dist = *mins.offset(0 as i32 as isize);
    (*box_planes.offset(3 as i32 as isize)).dist = -*mins.offset(0 as i32 as isize);
    (*box_planes.offset(4 as i32 as isize)).dist = *maxs.offset(1 as i32 as isize);
    (*box_planes.offset(5 as i32 as isize)).dist = -*maxs.offset(1 as i32 as isize);
    (*box_planes.offset(6 as i32 as isize)).dist = *mins.offset(1 as i32 as isize);
    (*box_planes.offset(7 as i32 as isize)).dist = -*mins.offset(1 as i32 as isize);
    (*box_planes.offset(8 as i32 as isize)).dist = *maxs.offset(2 as i32 as isize);
    (*box_planes.offset(9 as i32 as isize)).dist = -*maxs.offset(2 as i32 as isize);
    (*box_planes.offset(10 as i32 as isize)).dist = *mins.offset(2 as i32 as isize);
    (*box_planes.offset(11 as i32 as isize)).dist = -*mins.offset(2 as i32 as isize);
    (*box_brush).bounds[0 as i32 as usize][0 as i32 as usize] = *mins.offset(0 as i32 as isize);
    (*box_brush).bounds[0 as i32 as usize][1 as i32 as usize] = *mins.offset(1 as i32 as isize);
    (*box_brush).bounds[0 as i32 as usize][2 as i32 as usize] = *mins.offset(2 as i32 as isize);
    (*box_brush).bounds[1 as i32 as usize][0 as i32 as usize] = *maxs.offset(0 as i32 as isize);
    (*box_brush).bounds[1 as i32 as usize][1 as i32 as usize] = *maxs.offset(1 as i32 as isize);
    (*box_brush).bounds[1 as i32 as usize][2 as i32 as usize] = *maxs.offset(2 as i32 as isize);
    return 255 as i32;
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
// 0 = world, 1 + are bmodels
/*
===================
CM_ModelBounds
===================
*/
#[no_mangle]

pub unsafe extern "C" fn CM_ModelBounds(
    mut model: clipHandle_t,
    mut mins: *mut vec_t,
    mut maxs: *mut vec_t,
) {
    let mut cmod: *mut cmodel_t = std::ptr::null_mut();
    cmod = CM_ClipHandleToModel(model);
    *mins.offset(0 as i32 as isize) = (*cmod).mins[0 as i32 as usize];
    *mins.offset(1 as i32 as isize) = (*cmod).mins[1 as i32 as usize];
    *mins.offset(2 as i32 as isize) = (*cmod).mins[2 as i32 as usize];
    *maxs.offset(0 as i32 as isize) = (*cmod).maxs[0 as i32 as usize];
    *maxs.offset(1 as i32 as isize) = (*cmod).maxs[1 as i32 as usize];
    *maxs.offset(2 as i32 as isize) = (*cmod).maxs[2 as i32 as usize];
}

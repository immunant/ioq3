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
pub use crate::cm_local_h::leafList_s;
pub use crate::cm_local_h::leafList_t;
pub use crate::qfiles_h::dshader_t;
pub use crate::src::qcommon::cm_load::c_pointcontents;
pub use crate::src::qcommon::cm_load::cm;
pub use crate::src::qcommon::cm_load::cm_noAreas;
pub use crate::src::qcommon::cm_load::CM_ClipHandleToModel;
pub use crate::src::qcommon::cm_patch::patchCollide_s;
pub use crate::src::qcommon::common::Com_Error;
pub use crate::src::qcommon::q_math::AngleVectors;
pub use crate::src::qcommon::q_math::BoxOnPlaneSide;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::clipHandle_t;
pub use crate::src::qcommon::q_shared::cplane_s;
pub use crate::src::qcommon::q_shared::cplane_t;
pub use crate::src::qcommon::q_shared::cvar_s;
pub use crate::src::qcommon::q_shared::cvar_t;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::vec3_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::qcommon::q_shared::ERR_DISCONNECT;
pub use crate::src::qcommon::q_shared::ERR_DROP;
pub use crate::src::qcommon::q_shared::ERR_FATAL;
pub use crate::src::qcommon::q_shared::ERR_NEED_CD;
pub use crate::src::qcommon::q_shared::ERR_SERVERDISCONNECT;

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
==================
CM_PointLeafnum_r

==================
*/
#[no_mangle]

pub unsafe extern "C" fn CM_PointLeafnum_r(
    mut p: *const crate::src::qcommon::q_shared::vec_t,
    mut num: i32,
) -> i32 {
    let mut d: f32 = 0.; // optimize counter
    let mut node: *mut crate::cm_local_h::cNode_t = 0 as *mut crate::cm_local_h::cNode_t;
    let mut plane: *mut crate::src::qcommon::q_shared::cplane_t =
        0 as *mut crate::src::qcommon::q_shared::cplane_t;
    while num >= 0 as i32 {
        node = crate::src::qcommon::cm_load::cm.nodes.offset(num as isize);
        plane = (*node).plane;
        if ((*plane).type_0 as i32) < 3 as i32 {
            d = *p.offset((*plane).type_0 as isize) - (*plane).dist
        } else {
            d = (*plane).normal[0 as i32 as usize] * *p.offset(0 as i32 as isize)
                + (*plane).normal[1 as i32 as usize] * *p.offset(1 as i32 as isize)
                + (*plane).normal[2 as i32 as usize] * *p.offset(2 as i32 as isize)
                - (*plane).dist
        }
        if d < 0 as i32 as f32 {
            num = (*node).children[1 as i32 as usize]
        } else {
            num = (*node).children[0 as i32 as usize]
        }
    }
    crate::src::qcommon::cm_load::c_pointcontents += 1;
    return -(1 as i32) - num;
}
#[no_mangle]

pub unsafe extern "C" fn CM_PointLeafnum(
    mut p: *const crate::src::qcommon::q_shared::vec_t,
) -> i32 {
    if crate::src::qcommon::cm_load::cm.numNodes == 0 {
        // map not loaded
        return 0 as i32;
    }
    return CM_PointLeafnum_r(p, 0 as i32);
}
/*
======================================================================

LEAF LISTING

======================================================================
*/
#[no_mangle]

pub unsafe extern "C" fn CM_StoreLeafs(
    mut ll: *mut crate::cm_local_h::leafList_t,
    mut nodenum: i32,
) {
    let mut leafNum: i32 = 0;
    leafNum = -(1 as i32) - nodenum;
    // store the lastLeaf even if the list is overflowed
    if (*crate::src::qcommon::cm_load::cm
        .leafs
        .offset(leafNum as isize))
    .cluster
        != -(1 as i32)
    {
        (*ll).lastLeaf = leafNum
    }
    if (*ll).count >= (*ll).maxcount {
        (*ll).overflowed = crate::src::qcommon::q_shared::qtrue;
        return;
    }
    let fresh0 = (*ll).count;
    (*ll).count = (*ll).count + 1;
    *(*ll).list.offset(fresh0 as isize) = leafNum;
}
#[no_mangle]

pub unsafe extern "C" fn CM_StoreBrushes(
    mut ll: *mut crate::cm_local_h::leafList_t,
    mut nodenum: i32,
) {
    let mut i: i32 = 0;
    let mut k: i32 = 0;
    let mut leafnum: i32 = 0;
    let mut brushnum: i32 = 0;
    let mut leaf: *mut crate::cm_local_h::cLeaf_t = 0 as *mut crate::cm_local_h::cLeaf_t;
    let mut b: *mut crate::cm_local_h::cbrush_t = 0 as *mut crate::cm_local_h::cbrush_t;
    leafnum = -(1 as i32) - nodenum;
    leaf = &mut *crate::src::qcommon::cm_load::cm
        .leafs
        .offset(leafnum as isize) as *mut crate::cm_local_h::cLeaf_t;
    k = 0 as i32;
    while k < (*leaf).numLeafBrushes {
        brushnum = *crate::src::qcommon::cm_load::cm
            .leafbrushes
            .offset(((*leaf).firstLeafBrush + k) as isize);
        b = &mut *crate::src::qcommon::cm_load::cm
            .brushes
            .offset(brushnum as isize) as *mut crate::cm_local_h::cbrush_t;
        if !((*b).checkcount == crate::src::qcommon::cm_load::cm.checkcount) {
            (*b).checkcount = crate::src::qcommon::cm_load::cm.checkcount;
            i = 0 as i32;
            while i < 3 as i32 {
                if (*b).bounds[0 as i32 as usize][i as usize]
                    >= (*ll).bounds[1 as i32 as usize][i as usize]
                    || (*b).bounds[1 as i32 as usize][i as usize]
                        <= (*ll).bounds[0 as i32 as usize][i as usize]
                {
                    break;
                }
                i += 1
            }
            if !(i != 3 as i32) {
                if (*ll).count >= (*ll).maxcount {
                    (*ll).overflowed = crate::src::qcommon::q_shared::qtrue;
                    return;
                }
                let fresh1 = (*ll).count;
                (*ll).count = (*ll).count + 1;
                let ref mut fresh2 =
                    *((*ll).list as *mut *mut crate::cm_local_h::cbrush_t).offset(fresh1 as isize);
                *fresh2 = b
            }
        }
        k += 1
        // already checked this brush in another leaf
    }
}
/*
=============
CM_BoxLeafnums

Fills in a list of all the leafs touched
=============
*/
#[no_mangle]

pub unsafe extern "C" fn CM_BoxLeafnums_r(
    mut ll: *mut crate::cm_local_h::leafList_t,
    mut nodenum: i32,
) {
    let mut plane: *mut crate::src::qcommon::q_shared::cplane_t =
        0 as *mut crate::src::qcommon::q_shared::cplane_t;
    let mut node: *mut crate::cm_local_h::cNode_t = 0 as *mut crate::cm_local_h::cNode_t;
    let mut s: i32 = 0;
    loop {
        if nodenum < 0 as i32 {
            (*ll).storeLeafs.expect("non-null function pointer")(ll, nodenum);
            return;
        }
        node = &mut *crate::src::qcommon::cm_load::cm
            .nodes
            .offset(nodenum as isize) as *mut crate::cm_local_h::cNode_t;
        plane = (*node).plane;
        s = crate::src::qcommon::q_math::BoxOnPlaneSide(
            (*ll).bounds[0 as i32 as usize].as_mut_ptr(),
            (*ll).bounds[1 as i32 as usize].as_mut_ptr(),
            plane as *mut crate::src::qcommon::q_shared::cplane_s,
        );
        if s == 1 as i32 {
            nodenum = (*node).children[0 as i32 as usize]
        } else if s == 2 as i32 {
            nodenum = (*node).children[1 as i32 as usize]
        } else {
            // go down both
            CM_BoxLeafnums_r(ll, (*node).children[0 as i32 as usize]);
            nodenum = (*node).children[1 as i32 as usize]
        }
    }
}
/*
==================
CM_BoxLeafnums
==================
*/
#[no_mangle]

pub unsafe extern "C" fn CM_BoxLeafnums(
    mut mins: *const crate::src::qcommon::q_shared::vec_t,
    mut maxs: *const crate::src::qcommon::q_shared::vec_t,
    mut list: *mut i32,
    mut listsize: i32,
    mut lastLeaf: *mut i32,
) -> i32 {
    let mut ll: crate::cm_local_h::leafList_t = crate::cm_local_h::leafList_t {
        count: 0,
        maxcount: 0,
        overflowed: crate::src::qcommon::q_shared::qfalse,
        list: 0 as *mut i32,
        bounds: [[0.; 3]; 2],
        lastLeaf: 0,
        storeLeafs: None,
    };
    crate::src::qcommon::cm_load::cm.checkcount += 1;
    ll.bounds[0 as i32 as usize][0 as i32 as usize] =
        *mins.offset(0 as i32 as isize);
    ll.bounds[0 as i32 as usize][1 as i32 as usize] =
        *mins.offset(1 as i32 as isize);
    ll.bounds[0 as i32 as usize][2 as i32 as usize] =
        *mins.offset(2 as i32 as isize);
    ll.bounds[1 as i32 as usize][0 as i32 as usize] =
        *maxs.offset(0 as i32 as isize);
    ll.bounds[1 as i32 as usize][1 as i32 as usize] =
        *maxs.offset(1 as i32 as isize);
    ll.bounds[1 as i32 as usize][2 as i32 as usize] =
        *maxs.offset(2 as i32 as isize);
    ll.count = 0 as i32;
    ll.maxcount = listsize;
    ll.list = list;
    ll.storeLeafs = Some(
        CM_StoreLeafs
            as unsafe extern "C" fn(_: *mut crate::cm_local_h::leafList_t, _: i32) -> (),
    );
    ll.lastLeaf = 0 as i32;
    ll.overflowed = crate::src::qcommon::q_shared::qfalse;
    CM_BoxLeafnums_r(&mut ll, 0 as i32);
    *lastLeaf = ll.lastLeaf;
    return ll.count;
}
/*
==================
CM_BoxBrushes
==================
*/
#[no_mangle]

pub unsafe extern "C" fn CM_BoxBrushes(
    mut mins: *const crate::src::qcommon::q_shared::vec_t,
    mut maxs: *const crate::src::qcommon::q_shared::vec_t,
    mut list: *mut *mut crate::cm_local_h::cbrush_t,
    mut listsize: i32,
) -> i32 {
    let mut ll: crate::cm_local_h::leafList_t = crate::cm_local_h::leafList_t {
        count: 0,
        maxcount: 0,
        overflowed: crate::src::qcommon::q_shared::qfalse,
        list: 0 as *mut i32,
        bounds: [[0.; 3]; 2],
        lastLeaf: 0,
        storeLeafs: None,
    };
    crate::src::qcommon::cm_load::cm.checkcount += 1;
    ll.bounds[0 as i32 as usize][0 as i32 as usize] =
        *mins.offset(0 as i32 as isize);
    ll.bounds[0 as i32 as usize][1 as i32 as usize] =
        *mins.offset(1 as i32 as isize);
    ll.bounds[0 as i32 as usize][2 as i32 as usize] =
        *mins.offset(2 as i32 as isize);
    ll.bounds[1 as i32 as usize][0 as i32 as usize] =
        *maxs.offset(0 as i32 as isize);
    ll.bounds[1 as i32 as usize][1 as i32 as usize] =
        *maxs.offset(1 as i32 as isize);
    ll.bounds[1 as i32 as usize][2 as i32 as usize] =
        *maxs.offset(2 as i32 as isize);
    ll.count = 0 as i32;
    ll.maxcount = listsize;
    ll.list = list as *mut libc::c_void as *mut i32;
    ll.storeLeafs = Some(
        CM_StoreBrushes
            as unsafe extern "C" fn(_: *mut crate::cm_local_h::leafList_t, _: i32) -> (),
    );
    ll.lastLeaf = 0 as i32;
    ll.overflowed = crate::src::qcommon::q_shared::qfalse;
    CM_BoxLeafnums_r(&mut ll, 0 as i32);
    return ll.count;
}
//====================================================================
/*
==================
CM_PointContents

==================
*/
#[no_mangle]

pub unsafe extern "C" fn CM_PointContents(
    mut p: *const crate::src::qcommon::q_shared::vec_t,
    mut model: crate::src::qcommon::q_shared::clipHandle_t,
) -> i32 {
    let mut leafnum: i32 = 0;
    let mut i: i32 = 0;
    let mut k: i32 = 0;
    let mut brushnum: i32 = 0;
    let mut leaf: *mut crate::cm_local_h::cLeaf_t = 0 as *mut crate::cm_local_h::cLeaf_t;
    let mut b: *mut crate::cm_local_h::cbrush_t = 0 as *mut crate::cm_local_h::cbrush_t;
    let mut contents: i32 = 0;
    let mut d: f32 = 0.;
    let mut clipm: *mut crate::cm_local_h::cmodel_t = 0 as *mut crate::cm_local_h::cmodel_t;
    if crate::src::qcommon::cm_load::cm.numNodes == 0 {
        // map not loaded
        return 0 as i32;
    }
    if model != 0 {
        clipm = crate::src::qcommon::cm_load::CM_ClipHandleToModel(model)
            as *mut crate::cm_local_h::cmodel_s;
        leaf = &mut (*clipm).leaf
    } else {
        leafnum = CM_PointLeafnum_r(p, 0 as i32);
        leaf = &mut *crate::src::qcommon::cm_load::cm
            .leafs
            .offset(leafnum as isize) as *mut crate::cm_local_h::cLeaf_t
    }
    contents = 0 as i32;
    k = 0 as i32;
    while k < (*leaf).numLeafBrushes {
        brushnum = *crate::src::qcommon::cm_load::cm
            .leafbrushes
            .offset(((*leaf).firstLeafBrush + k) as isize);
        b = &mut *crate::src::qcommon::cm_load::cm
            .brushes
            .offset(brushnum as isize) as *mut crate::cm_local_h::cbrush_t;
        if !(CM_BoundsIntersectPoint(
            (*b).bounds[0 as i32 as usize].as_mut_ptr()
                as *const crate::src::qcommon::q_shared::vec_t,
            (*b).bounds[1 as i32 as usize].as_mut_ptr()
                as *const crate::src::qcommon::q_shared::vec_t,
            p,
        ) as u64
            == 0)
        {
            // see if the point is in the brush
            i = 0 as i32;
            while i < (*b).numsides {
                d = *p.offset(0 as i32 as isize)
                    * (*(*(*b).sides.offset(i as isize)).plane).normal[0 as i32 as usize]
                    + *p.offset(1 as i32 as isize)
                        * (*(*(*b).sides.offset(i as isize)).plane).normal
                            [1 as i32 as usize]
                    + *p.offset(2 as i32 as isize)
                        * (*(*(*b).sides.offset(i as isize)).plane).normal
                            [2 as i32 as usize];
                // FIXME test for Cash
                //			if ( d >= b->sides[i].plane->dist ) {
                if d > (*(*(*b).sides.offset(i as isize)).plane).dist {
                    break;
                }
                i += 1
            }
            if i == (*b).numsides {
                contents |= (*b).contents
            }
        }
        k += 1
    }
    return contents;
}
/*
==================
CM_TransformedPointContents

Handles offseting and rotation of the end points for moving and
rotating entities
==================
*/
#[no_mangle]

pub unsafe extern "C" fn CM_TransformedPointContents(
    mut p: *const crate::src::qcommon::q_shared::vec_t,
    mut model: crate::src::qcommon::q_shared::clipHandle_t,
    mut origin: *const crate::src::qcommon::q_shared::vec_t,
    mut angles: *const crate::src::qcommon::q_shared::vec_t,
) -> i32 {
    let mut p_l: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut temp: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut forward: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut right: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut up: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    // subtract origin offset
    p_l[0 as i32 as usize] =
        *p.offset(0 as i32 as isize) - *origin.offset(0 as i32 as isize);
    p_l[1 as i32 as usize] =
        *p.offset(1 as i32 as isize) - *origin.offset(1 as i32 as isize);
    p_l[2 as i32 as usize] =
        *p.offset(2 as i32 as isize) - *origin.offset(2 as i32 as isize);
    // rotate start and end into the models frame of reference
    if model != 255 as i32
        && (*angles.offset(0 as i32 as isize) != 0.
            || *angles.offset(1 as i32 as isize) != 0.
            || *angles.offset(2 as i32 as isize) != 0.)
    {
        crate::src::qcommon::q_math::AngleVectors(
            angles,
            forward.as_mut_ptr(),
            right.as_mut_ptr(),
            up.as_mut_ptr(),
        );
        temp[0 as i32 as usize] = p_l[0 as i32 as usize];
        temp[1 as i32 as usize] = p_l[1 as i32 as usize];
        temp[2 as i32 as usize] = p_l[2 as i32 as usize];
        p_l[0 as i32 as usize] = temp[0 as i32 as usize]
            * forward[0 as i32 as usize]
            + temp[1 as i32 as usize] * forward[1 as i32 as usize]
            + temp[2 as i32 as usize] * forward[2 as i32 as usize];
        p_l[1 as i32 as usize] = -(temp[0 as i32 as usize]
            * right[0 as i32 as usize]
            + temp[1 as i32 as usize] * right[1 as i32 as usize]
            + temp[2 as i32 as usize] * right[2 as i32 as usize]);
        p_l[2 as i32 as usize] = temp[0 as i32 as usize]
            * up[0 as i32 as usize]
            + temp[1 as i32 as usize] * up[1 as i32 as usize]
            + temp[2 as i32 as usize] * up[2 as i32 as usize]
    }
    return CM_PointContents(
        p_l.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        model,
    );
}
/*
===============================================================================

PVS

===============================================================================
*/
#[no_mangle]

pub unsafe extern "C" fn CM_ClusterPVS(
    mut cluster: i32,
) -> *mut crate::src::qcommon::q_shared::byte {
    if cluster < 0 as i32
        || cluster >= crate::src::qcommon::cm_load::cm.numClusters
        || crate::src::qcommon::cm_load::cm.vised as u64 == 0
    {
        return crate::src::qcommon::cm_load::cm.visibility;
    }
    return crate::src::qcommon::cm_load::cm
        .visibility
        .offset((cluster * crate::src::qcommon::cm_load::cm.clusterBytes) as isize);
}
/*
===============================================================================

AREAPORTALS

===============================================================================
*/
#[no_mangle]

pub unsafe extern "C" fn CM_FloodArea_r(mut areaNum: i32, mut floodnum: i32) {
    let mut i: i32 = 0;
    let mut area: *mut crate::cm_local_h::cArea_t = 0 as *mut crate::cm_local_h::cArea_t;
    let mut con: *mut i32 = 0 as *mut i32;
    area = &mut *crate::src::qcommon::cm_load::cm
        .areas
        .offset(areaNum as isize) as *mut crate::cm_local_h::cArea_t;
    if (*area).floodvalid == crate::src::qcommon::cm_load::cm.floodvalid {
        if (*area).floodnum == floodnum {
            return;
        }
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as i32,
            b"FloodArea_r: reflooded\x00" as *const u8 as *const libc::c_char,
        );
    }
    (*area).floodnum = floodnum;
    (*area).floodvalid = crate::src::qcommon::cm_load::cm.floodvalid;
    con = crate::src::qcommon::cm_load::cm
        .areaPortals
        .offset((areaNum * crate::src::qcommon::cm_load::cm.numAreas) as isize);
    i = 0 as i32;
    while i < crate::src::qcommon::cm_load::cm.numAreas {
        if *con.offset(i as isize) > 0 as i32 {
            CM_FloodArea_r(i, floodnum);
        }
        i += 1
    }
}
/*
====================
CM_FloodAreaConnections

====================
*/
#[no_mangle]

pub unsafe extern "C" fn CM_FloodAreaConnections() {
    let mut i: i32 = 0;
    let mut area: *mut crate::cm_local_h::cArea_t = 0 as *mut crate::cm_local_h::cArea_t;
    let mut floodnum: i32 = 0;
    // all current floods are now invalid
    crate::src::qcommon::cm_load::cm.floodvalid += 1;
    floodnum = 0 as i32;
    i = 0 as i32;
    while i < crate::src::qcommon::cm_load::cm.numAreas {
        area = &mut *crate::src::qcommon::cm_load::cm.areas.offset(i as isize)
            as *mut crate::cm_local_h::cArea_t;
        if !((*area).floodvalid == crate::src::qcommon::cm_load::cm.floodvalid) {
            floodnum += 1;
            CM_FloodArea_r(i, floodnum);
        }
        i += 1
        // already flooded into
    }
}
/*
====================
CM_AdjustAreaPortalState

====================
*/
#[no_mangle]

pub unsafe extern "C" fn CM_AdjustAreaPortalState(
    mut area1: i32,
    mut area2: i32,
    mut open: crate::src::qcommon::q_shared::qboolean,
) {
    if area1 < 0 as i32 || area2 < 0 as i32 {
        return;
    }
    if area1 >= crate::src::qcommon::cm_load::cm.numAreas
        || area2 >= crate::src::qcommon::cm_load::cm.numAreas
    {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as i32,
            b"CM_ChangeAreaPortalState: bad area number\x00" as *const u8 as *const libc::c_char,
        );
    }
    if open as u64 != 0 {
        let ref mut fresh3 = *crate::src::qcommon::cm_load::cm
            .areaPortals
            .offset((area1 * crate::src::qcommon::cm_load::cm.numAreas + area2) as isize);
        *fresh3 += 1;
        let ref mut fresh4 = *crate::src::qcommon::cm_load::cm
            .areaPortals
            .offset((area2 * crate::src::qcommon::cm_load::cm.numAreas + area1) as isize);
        *fresh4 += 1
    } else {
        let ref mut fresh5 = *crate::src::qcommon::cm_load::cm
            .areaPortals
            .offset((area1 * crate::src::qcommon::cm_load::cm.numAreas + area2) as isize);
        *fresh5 -= 1;
        let ref mut fresh6 = *crate::src::qcommon::cm_load::cm
            .areaPortals
            .offset((area2 * crate::src::qcommon::cm_load::cm.numAreas + area1) as isize);
        *fresh6 -= 1;
        if *crate::src::qcommon::cm_load::cm
            .areaPortals
            .offset((area2 * crate::src::qcommon::cm_load::cm.numAreas + area1) as isize)
            < 0 as i32
        {
            crate::src::qcommon::common::Com_Error(
                crate::src::qcommon::q_shared::ERR_DROP as i32,
                b"CM_AdjustAreaPortalState: negative reference count\x00" as *const u8
                    as *const libc::c_char,
            );
        }
    }
    CM_FloodAreaConnections();
}
/*
====================
CM_AreasConnected

====================
*/
#[no_mangle]

pub unsafe extern "C" fn CM_AreasConnected(
    mut area1: i32,
    mut area2: i32,
) -> crate::src::qcommon::q_shared::qboolean {
    if (*crate::src::qcommon::cm_load::cm_noAreas).integer != 0 {
        return crate::src::qcommon::q_shared::qtrue;
    }
    if area1 < 0 as i32 || area2 < 0 as i32 {
        return crate::src::qcommon::q_shared::qfalse;
    }
    if area1 >= crate::src::qcommon::cm_load::cm.numAreas
        || area2 >= crate::src::qcommon::cm_load::cm.numAreas
    {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as i32,
            b"area >= cm.numAreas\x00" as *const u8 as *const libc::c_char,
        );
    }
    if (*crate::src::qcommon::cm_load::cm
        .areas
        .offset(area1 as isize))
    .floodnum
        == (*crate::src::qcommon::cm_load::cm
            .areas
            .offset(area2 as isize))
        .floodnum
    {
        return crate::src::qcommon::q_shared::qtrue;
    }
    return crate::src::qcommon::q_shared::qfalse;
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
// returns an ORed contents mask
// only returns non-solid leafs
// overflow if return listsize and if *lastLeaf != list[listsize-1]
/*
=================
CM_WriteAreaBits

Writes a bit vector of all the areas
that are in the same flood as the area parameter
Returns the number of bytes needed to hold all the bits.

The bits are OR'd in, so you can CM_WriteAreaBits from multiple
viewpoints and get the union of all visible areas.

This is used to cull non-visible entities from snapshots
=================
*/
#[no_mangle]

pub unsafe extern "C" fn CM_WriteAreaBits(
    mut buffer: *mut crate::src::qcommon::q_shared::byte,
    mut area: i32,
) -> i32 {
    let mut i: i32 = 0;
    let mut floodnum: i32 = 0;
    let mut bytes: i32 = 0;
    bytes = crate::src::qcommon::cm_load::cm.numAreas + 7 as i32 >> 3 as i32;
    if (*crate::src::qcommon::cm_load::cm_noAreas).integer != 0 || area == -(1 as i32) {
        // for debugging, send everything
        crate::stdlib::memset(
            buffer as *mut libc::c_void,
            255 as i32,
            bytes as libc::c_ulong,
        );
    } else {
        floodnum = (*crate::src::qcommon::cm_load::cm.areas.offset(area as isize)).floodnum;
        i = 0 as i32;
        while i < crate::src::qcommon::cm_load::cm.numAreas {
            if (*crate::src::qcommon::cm_load::cm.areas.offset(i as isize)).floodnum == floodnum
                || area == -(1 as i32)
            {
                let ref mut fresh7 = *buffer.offset((i >> 3 as i32) as isize);
                *fresh7 = (*fresh7 as i32 | (1 as i32) << (i & 7 as i32))
                    as crate::src::qcommon::q_shared::byte
            }
            i += 1
        }
    }
    return bytes;
}
/*
====================
CM_BoundsIntersect
====================
*/
#[no_mangle]

pub unsafe extern "C" fn CM_BoundsIntersect(
    mut mins: *const crate::src::qcommon::q_shared::vec_t,
    mut maxs: *const crate::src::qcommon::q_shared::vec_t,
    mut mins2: *const crate::src::qcommon::q_shared::vec_t,
    mut maxs2: *const crate::src::qcommon::q_shared::vec_t,
) -> crate::src::qcommon::q_shared::qboolean {
    if (*maxs.offset(0 as i32 as isize) as f64)
        < *mins2.offset(0 as i32 as isize) as f64 - 0.125f64
        || (*maxs.offset(1 as i32 as isize) as f64)
            < *mins2.offset(1 as i32 as isize) as f64 - 0.125f64
        || (*maxs.offset(2 as i32 as isize) as f64)
            < *mins2.offset(2 as i32 as isize) as f64 - 0.125f64
        || *mins.offset(0 as i32 as isize) as f64
            > *maxs2.offset(0 as i32 as isize) as f64 + 0.125f64
        || *mins.offset(1 as i32 as isize) as f64
            > *maxs2.offset(1 as i32 as isize) as f64 + 0.125f64
        || *mins.offset(2 as i32 as isize) as f64
            > *maxs2.offset(2 as i32 as isize) as f64 + 0.125f64
    {
        return crate::src::qcommon::q_shared::qfalse;
    }
    return crate::src::qcommon::q_shared::qtrue;
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
====================
CM_BoundsIntersectPoint
====================
*/
#[no_mangle]

pub unsafe extern "C" fn CM_BoundsIntersectPoint(
    mut mins: *const crate::src::qcommon::q_shared::vec_t,
    mut maxs: *const crate::src::qcommon::q_shared::vec_t,
    mut point: *const crate::src::qcommon::q_shared::vec_t,
) -> crate::src::qcommon::q_shared::qboolean {
    if (*maxs.offset(0 as i32 as isize) as f64)
        < *point.offset(0 as i32 as isize) as f64 - 0.125f64
        || (*maxs.offset(1 as i32 as isize) as f64)
            < *point.offset(1 as i32 as isize) as f64 - 0.125f64
        || (*maxs.offset(2 as i32 as isize) as f64)
            < *point.offset(2 as i32 as isize) as f64 - 0.125f64
        || *mins.offset(0 as i32 as isize) as f64
            > *point.offset(0 as i32 as isize) as f64 + 0.125f64
        || *mins.offset(1 as i32 as isize) as f64
            > *point.offset(1 as i32 as isize) as f64 + 0.125f64
        || *mins.offset(2 as i32 as isize) as f64
            > *point.offset(2 as i32 as isize) as f64 + 0.125f64
    {
        return crate::src::qcommon::q_shared::qfalse;
    }
    return crate::src::qcommon::q_shared::qtrue;
}

use ::libc;

pub use crate::g_public_h::entityShared_t;
pub use crate::g_public_h::sharedEntity_t;
pub use crate::server_h::serverState_t;
pub use crate::server_h::server_t;
pub use crate::server_h::svEntity_s;
pub use crate::server_h::svEntity_t;
pub use crate::server_h::SS_DEAD;
pub use crate::server_h::SS_GAME;
pub use crate::server_h::SS_LOADING;

pub use crate::src::qcommon::common::Com_Printf;
pub use crate::src::qcommon::q_math::vec3_origin;
pub use crate::src::qcommon::q_math::RadiusFromBounds;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::clipHandle_t;
pub use crate::src::qcommon::q_shared::cplane_s;
pub use crate::src::qcommon::q_shared::cplane_t;
pub use crate::src::qcommon::q_shared::entityState_s;
pub use crate::src::qcommon::q_shared::entityState_t;
pub use crate::src::qcommon::q_shared::playerState_s;
pub use crate::src::qcommon::q_shared::playerState_t;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::trType_t;
pub use crate::src::qcommon::q_shared::trace_t;
pub use crate::src::qcommon::q_shared::trajectory_t;
pub use crate::src::qcommon::q_shared::vec3_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::qcommon::q_shared::TR_GRAVITY;
pub use crate::src::qcommon::q_shared::TR_INTERPOLATE;
pub use crate::src::qcommon::q_shared::TR_LINEAR;
pub use crate::src::qcommon::q_shared::TR_LINEAR_STOP;
pub use crate::src::qcommon::q_shared::TR_SINE;
pub use crate::src::qcommon::q_shared::TR_STATIONARY;
pub use crate::src::server::sv_game::SV_GEntityForSvEntity;
pub use crate::src::server::sv_game::SV_GentityNum;
pub use crate::src::server::sv_game::SV_SvEntityForGentity;
pub use crate::src::server::sv_main::sv;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct worldSector_s {
    pub axis: i32,
    pub dist: f32,
    pub children: [*mut worldSector_s; 2],
    pub entities: *mut crate::server_h::svEntity_t,
}
/*
===============================================================================

ENTITY CHECKING

To avoid linearly searching through lists of entities during environment testing,
the world is carved up with an evenly spaced, axially aligned bsp tree.  Entities
are kept in chains either at the final leafs, or at the first node that splits
them, which prevents having to deal with multiple fragments of a single entity.

===============================================================================
*/

pub type worldSector_t = worldSector_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct areaParms_t {
    pub mins: *const f32,
    pub maxs: *const f32,
    pub list: *mut i32,
    pub count: i32,
    pub maxcount: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct moveclip_t {
    pub boxmins: crate::src::qcommon::q_shared::vec3_t,
    pub boxmaxs: crate::src::qcommon::q_shared::vec3_t,
    pub mins: *const f32,
    pub maxs: *const f32,
    pub start: *const f32,
    pub end: crate::src::qcommon::q_shared::vec3_t,
    pub trace: crate::src::qcommon::q_shared::trace_t,
    pub passEntityNum: i32,
    pub contentmask: i32,
    pub capsule: i32,
}
// -1 = leaf node
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
// world.c -- world query functions
/*
================
SV_ClipHandleForEntity

Returns a headnode that can be used for testing or clipping to a
given entity.  If the entity is a bsp model, the headnode will
be returned, otherwise a custom box tree will be constructed.
================
*/
#[no_mangle]

pub unsafe extern "C" fn SV_ClipHandleForEntity(
    mut ent: *const crate::g_public_h::sharedEntity_t,
) -> crate::src::qcommon::q_shared::clipHandle_t {
    if (*ent).r.bmodel as u64 != 0 {
        // explicit hulls in the BSP model
        return crate::src::qcommon::cm_load::CM_InlineModel((*ent).s.modelindex);
    }
    if (*ent).r.svFlags & 0x400 as i32 != 0 {
        // create a temp capsule from bounding box sizes
        return crate::src::qcommon::cm_load::CM_TempBoxModel(
            (*ent).r.mins.as_ptr(),
            (*ent).r.maxs.as_ptr(),
            crate::src::qcommon::q_shared::qtrue as i32,
        );
    }
    // create a temp tree from bounding box sizes
    return crate::src::qcommon::cm_load::CM_TempBoxModel(
        (*ent).r.mins.as_ptr(),
        (*ent).r.maxs.as_ptr(),
        crate::src::qcommon::q_shared::qfalse as i32,
    );
}
#[no_mangle]

pub static mut sv_worldSectors: [worldSector_t; 64] = [worldSector_t {
    axis: 0,
    dist: 0.,
    children: [0 as *const worldSector_s as *mut worldSector_s; 2],
    entities: 0 as *const crate::server_h::svEntity_t as *mut crate::server_h::svEntity_t,
}; 64];
#[no_mangle]

pub static mut sv_numworldSectors: i32 = 0;
/*
===============
SV_SectorList_f
===============
*/
#[no_mangle]

pub unsafe extern "C" fn SV_SectorList_f() {
    let mut i: i32 = 0;
    let mut c: i32 = 0;
    let mut sec: *mut worldSector_t = 0 as *mut worldSector_t;
    let mut ent: *mut crate::server_h::svEntity_t = 0 as *mut crate::server_h::svEntity_t;
    i = 0 as i32;
    while i < 64 as i32 {
        sec = &mut *sv_worldSectors.as_mut_ptr().offset(i as isize) as *mut worldSector_t;
        c = 0 as i32;
        ent = (*sec).entities;
        while !ent.is_null() {
            c += 1;
            ent = (*ent).nextEntityInWorldSector
        }
        crate::src::qcommon::common::Com_Printf(
            b"sector %i: %i entities\n\x00" as *const u8 as *const libc::c_char,
            i,
            c,
        );
        i += 1
    }
}
/*
===============
SV_CreateworldSector

Builds a uniformly subdivided tree for the given world size
===============
*/

unsafe extern "C" fn SV_CreateworldSector(
    mut depth: i32,
    mut mins: *mut crate::src::qcommon::q_shared::vec_t,
    mut maxs: *mut crate::src::qcommon::q_shared::vec_t,
) -> *mut worldSector_t {
    let mut anode: *mut worldSector_t = 0 as *mut worldSector_t;
    let mut size: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut mins1: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut maxs1: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut mins2: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut maxs2: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    anode = &mut *sv_worldSectors
        .as_mut_ptr()
        .offset(sv_numworldSectors as isize) as *mut worldSector_t;
    sv_numworldSectors += 1;
    if depth == 4 as i32 {
        (*anode).axis = -(1 as i32);
        (*anode).children[1 as i32 as usize] = 0 as *mut worldSector_s;
        (*anode).children[0 as i32 as usize] = (*anode).children[1 as i32 as usize];
        return anode;
    }
    size[0 as i32 as usize] =
        *maxs.offset(0 as i32 as isize) - *mins.offset(0 as i32 as isize);
    size[1 as i32 as usize] =
        *maxs.offset(1 as i32 as isize) - *mins.offset(1 as i32 as isize);
    size[2 as i32 as usize] =
        *maxs.offset(2 as i32 as isize) - *mins.offset(2 as i32 as isize);
    if size[0 as i32 as usize] > size[1 as i32 as usize] {
        (*anode).axis = 0 as i32
    } else {
        (*anode).axis = 1 as i32
    }
    (*anode).dist = (0.5f64
        * (*maxs.offset((*anode).axis as isize) + *mins.offset((*anode).axis as isize))
            as f64) as f32;
    mins1[0 as i32 as usize] = *mins.offset(0 as i32 as isize);
    mins1[1 as i32 as usize] = *mins.offset(1 as i32 as isize);
    mins1[2 as i32 as usize] = *mins.offset(2 as i32 as isize);
    mins2[0 as i32 as usize] = *mins.offset(0 as i32 as isize);
    mins2[1 as i32 as usize] = *mins.offset(1 as i32 as isize);
    mins2[2 as i32 as usize] = *mins.offset(2 as i32 as isize);
    maxs1[0 as i32 as usize] = *maxs.offset(0 as i32 as isize);
    maxs1[1 as i32 as usize] = *maxs.offset(1 as i32 as isize);
    maxs1[2 as i32 as usize] = *maxs.offset(2 as i32 as isize);
    maxs2[0 as i32 as usize] = *maxs.offset(0 as i32 as isize);
    maxs2[1 as i32 as usize] = *maxs.offset(1 as i32 as isize);
    maxs2[2 as i32 as usize] = *maxs.offset(2 as i32 as isize);
    mins2[(*anode).axis as usize] = (*anode).dist;
    maxs1[(*anode).axis as usize] = mins2[(*anode).axis as usize];
    (*anode).children[0 as i32 as usize] = SV_CreateworldSector(
        depth + 1 as i32,
        mins2.as_mut_ptr(),
        maxs2.as_mut_ptr(),
    );
    (*anode).children[1 as i32 as usize] = SV_CreateworldSector(
        depth + 1 as i32,
        mins1.as_mut_ptr(),
        maxs1.as_mut_ptr(),
    );
    return anode;
}
/*
===============
SV_ClearWorld

===============
*/
#[no_mangle]

pub unsafe extern "C" fn SV_ClearWorld() {
    let mut h: crate::src::qcommon::q_shared::clipHandle_t = 0;
    let mut mins: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut maxs: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    crate::stdlib::memset(
        sv_worldSectors.as_mut_ptr() as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<[worldSector_t; 64]>() as libc::c_ulong,
    );
    sv_numworldSectors = 0 as i32;
    // get world map bounds
    h = crate::src::qcommon::cm_load::CM_InlineModel(0 as i32);
    crate::src::qcommon::cm_load::CM_ModelBounds(h, mins.as_mut_ptr(), maxs.as_mut_ptr());
    SV_CreateworldSector(0 as i32, mins.as_mut_ptr(), maxs.as_mut_ptr());
}
/*
===============
SV_UnlinkEntity

===============
*/
#[no_mangle]

pub unsafe extern "C" fn SV_UnlinkEntity(mut gEnt: *mut crate::g_public_h::sharedEntity_t) {
    let mut ent: *mut crate::server_h::svEntity_t = 0 as *mut crate::server_h::svEntity_t;
    let mut scan: *mut crate::server_h::svEntity_t = 0 as *mut crate::server_h::svEntity_t;
    let mut ws: *mut worldSector_t = 0 as *mut worldSector_t;
    ent = crate::src::server::sv_game::SV_SvEntityForGentity(
        gEnt as *mut crate::g_public_h::sharedEntity_t,
    ) as *mut crate::server_h::svEntity_s;
    (*gEnt).r.linked = crate::src::qcommon::q_shared::qfalse;
    ws = (*ent).worldSector;
    if ws.is_null() {
        return;
        // not linked in anywhere
    }
    (*ent).worldSector = 0 as *mut worldSector_s;
    if (*ws).entities == ent {
        (*ws).entities = (*ent).nextEntityInWorldSector;
        return;
    }
    scan = (*ws).entities;
    while !scan.is_null() {
        if (*scan).nextEntityInWorldSector == ent {
            (*scan).nextEntityInWorldSector = (*ent).nextEntityInWorldSector;
            return;
        }
        scan = (*scan).nextEntityInWorldSector
    }
    crate::src::qcommon::common::Com_Printf(
        b"WARNING: SV_UnlinkEntity: not found in worldSector\n\x00" as *const u8
            as *const libc::c_char,
    );
}
#[no_mangle]

pub unsafe extern "C" fn SV_LinkEntity(mut gEnt: *mut crate::g_public_h::sharedEntity_t) {
    let mut node: *mut worldSector_t = 0 as *mut worldSector_t;
    let mut leafs: [i32; 128] = [0; 128];
    let mut cluster: i32 = 0;
    let mut num_leafs: i32 = 0;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut area: i32 = 0;
    let mut lastLeaf: i32 = 0;
    let mut origin: *mut f32 = 0 as *mut f32;
    let mut angles: *mut f32 = 0 as *mut f32;
    let mut ent: *mut crate::server_h::svEntity_t = 0 as *mut crate::server_h::svEntity_t;
    ent = crate::src::server::sv_game::SV_SvEntityForGentity(
        gEnt as *mut crate::g_public_h::sharedEntity_t,
    ) as *mut crate::server_h::svEntity_s;
    if !(*ent).worldSector.is_null() {
        SV_UnlinkEntity(gEnt);
        // unlink from old position
    }
    // encode the size into the entityState_t for client prediction
    if (*gEnt).r.bmodel as u64 != 0 {
        (*gEnt).s.solid = 0xffffff as i32
    // a solid_box will never create this value
    } else if (*gEnt).r.contents & (1 as i32 | 0x2000000 as i32) != 0 {
        // assume that x/y are equal and symetric
        i = (*gEnt).r.maxs[0 as i32 as usize] as i32;
        if i < 1 as i32 {
            i = 1 as i32
        }
        if i > 255 as i32 {
            i = 255 as i32
        }
        // z is not symetric
        j = -(*gEnt).r.mins[2 as i32 as usize] as i32;
        if j < 1 as i32 {
            j = 1 as i32
        }
        if j > 255 as i32 {
            j = 255 as i32
        }
        // and z maxs can be negative...
        k = ((*gEnt).r.maxs[2 as i32 as usize] + 32 as i32 as f32)
            as i32;
        if k < 1 as i32 {
            k = 1 as i32
        }
        if k > 255 as i32 {
            k = 255 as i32
        }
        (*gEnt).s.solid = k << 16 as i32 | j << 8 as i32 | i
    } else {
        (*gEnt).s.solid = 0 as i32
    }
    // get the position
    origin = (*gEnt).r.currentOrigin.as_mut_ptr();
    angles = (*gEnt).r.currentAngles.as_mut_ptr();
    // set the abs box
    if (*gEnt).r.bmodel as u32 != 0
        && (*angles.offset(0 as i32 as isize) != 0.
            || *angles.offset(1 as i32 as isize) != 0.
            || *angles.offset(2 as i32 as isize) != 0.)
    {
        // expand for rotation
        let mut max: f32 = 0.;
        max = crate::src::qcommon::q_math::RadiusFromBounds(
            (*gEnt).r.mins.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            (*gEnt).r.maxs.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        );
        i = 0 as i32;
        while i < 3 as i32 {
            (*gEnt).r.absmin[i as usize] = *origin.offset(i as isize) - max;
            (*gEnt).r.absmax[i as usize] = *origin.offset(i as isize) + max;
            i += 1
        }
    } else {
        // normal
        (*gEnt).r.absmin[0 as i32 as usize] =
            *origin.offset(0 as i32 as isize) + (*gEnt).r.mins[0 as i32 as usize];
        (*gEnt).r.absmin[1 as i32 as usize] =
            *origin.offset(1 as i32 as isize) + (*gEnt).r.mins[1 as i32 as usize];
        (*gEnt).r.absmin[2 as i32 as usize] =
            *origin.offset(2 as i32 as isize) + (*gEnt).r.mins[2 as i32 as usize];
        (*gEnt).r.absmax[0 as i32 as usize] =
            *origin.offset(0 as i32 as isize) + (*gEnt).r.maxs[0 as i32 as usize];
        (*gEnt).r.absmax[1 as i32 as usize] =
            *origin.offset(1 as i32 as isize) + (*gEnt).r.maxs[1 as i32 as usize];
        (*gEnt).r.absmax[2 as i32 as usize] =
            *origin.offset(2 as i32 as isize) + (*gEnt).r.maxs[2 as i32 as usize]
    }
    // because movement is clipped an epsilon away from an actual edge,
    // we must fully check even when bounding boxes don't quite touch
    (*gEnt).r.absmin[0 as i32 as usize] -= 1 as i32 as f32;
    (*gEnt).r.absmin[1 as i32 as usize] -= 1 as i32 as f32;
    (*gEnt).r.absmin[2 as i32 as usize] -= 1 as i32 as f32;
    (*gEnt).r.absmax[0 as i32 as usize] += 1 as i32 as f32;
    (*gEnt).r.absmax[1 as i32 as usize] += 1 as i32 as f32;
    (*gEnt).r.absmax[2 as i32 as usize] += 1 as i32 as f32;
    // link to PVS leafs
    (*ent).numClusters = 0 as i32;
    (*ent).lastCluster = 0 as i32;
    (*ent).areanum = -(1 as i32);
    (*ent).areanum2 = -(1 as i32);
    //get all leafs, including solids
    num_leafs = crate::src::qcommon::cm_test::CM_BoxLeafnums(
        (*gEnt).r.absmin.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        (*gEnt).r.absmax.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        leafs.as_mut_ptr(),
        128 as i32,
        &mut lastLeaf,
    );
    // if none of the leafs were inside the map, the
    // entity is outside the world and can be considered unlinked
    if num_leafs == 0 {
        return;
    }
    // set areas, even from clusters that don't fit in the entity array
    i = 0 as i32;
    while i < num_leafs {
        area = crate::src::qcommon::cm_load::CM_LeafArea(leafs[i as usize]);
        if area != -(1 as i32) {
            // doors may legally straggle two areas,
            // but nothing should evern need more than that
            if (*ent).areanum != -(1 as i32) && (*ent).areanum != area {
                if (*ent).areanum2 != -(1 as i32)
                    && (*ent).areanum2 != area
                    && crate::src::server::sv_main::sv.state as u32
                        == crate::server_h::SS_LOADING as i32 as u32
                {
                    crate::src::qcommon::common::Com_DPrintf(
                        b"Object %i touching 3 areas at %f %f %f\n\x00" as *const u8
                            as *const libc::c_char,
                        (*gEnt).s.number,
                        (*gEnt).r.absmin[0 as i32 as usize] as f64,
                        (*gEnt).r.absmin[1 as i32 as usize] as f64,
                        (*gEnt).r.absmin[2 as i32 as usize] as f64,
                    );
                }
                (*ent).areanum2 = area
            } else {
                (*ent).areanum = area
            }
        }
        i += 1
    }
    // store as many explicit clusters as we can
    (*ent).numClusters = 0 as i32;
    i = 0 as i32;
    while i < num_leafs {
        cluster = crate::src::qcommon::cm_load::CM_LeafCluster(leafs[i as usize]);
        if cluster != -(1 as i32) {
            let fresh0 = (*ent).numClusters;
            (*ent).numClusters = (*ent).numClusters + 1;
            (*ent).clusternums[fresh0 as usize] = cluster;
            if (*ent).numClusters == 16 as i32 {
                break;
            }
        }
        i += 1
    }
    // store off a last cluster if we need to
    if i != num_leafs {
        (*ent).lastCluster = crate::src::qcommon::cm_load::CM_LeafCluster(lastLeaf)
    }
    (*gEnt).r.linkcount += 1;
    // find the first world sector node that the ent's box crosses
    node = sv_worldSectors.as_mut_ptr();
    while !((*node).axis == -(1 as i32)) {
        if (*gEnt).r.absmin[(*node).axis as usize] > (*node).dist {
            node = (*node).children[0 as i32 as usize]
        } else {
            if !((*gEnt).r.absmax[(*node).axis as usize] < (*node).dist) {
                break;
            }
            node = (*node).children[1 as i32 as usize]
        }
        // crosses the node
    }
    // link it in
    (*ent).worldSector = node;
    (*ent).nextEntityInWorldSector = (*node).entities;
    (*node).entities = ent;
    (*gEnt).r.linked = crate::src::qcommon::q_shared::qtrue;
}
/*
====================
SV_AreaEntities_r

====================
*/

unsafe extern "C" fn SV_AreaEntities_r(mut node: *mut worldSector_t, mut ap: *mut areaParms_t) {
    let mut check: *mut crate::server_h::svEntity_t = 0 as *mut crate::server_h::svEntity_t;
    let mut next: *mut crate::server_h::svEntity_t = 0 as *mut crate::server_h::svEntity_t;
    let mut gcheck: *mut crate::g_public_h::sharedEntity_t =
        0 as *mut crate::g_public_h::sharedEntity_t;
    check = (*node).entities;
    while !check.is_null() {
        next = (*check).nextEntityInWorldSector;
        gcheck = crate::src::server::sv_game::SV_GEntityForSvEntity(
            check as *mut crate::server_h::svEntity_s,
        ) as *mut crate::g_public_h::sharedEntity_t;
        if !((*gcheck).r.absmin[0 as i32 as usize]
            > *(*ap).maxs.offset(0 as i32 as isize)
            || (*gcheck).r.absmin[1 as i32 as usize]
                > *(*ap).maxs.offset(1 as i32 as isize)
            || (*gcheck).r.absmin[2 as i32 as usize]
                > *(*ap).maxs.offset(2 as i32 as isize)
            || (*gcheck).r.absmax[0 as i32 as usize]
                < *(*ap).mins.offset(0 as i32 as isize)
            || (*gcheck).r.absmax[1 as i32 as usize]
                < *(*ap).mins.offset(1 as i32 as isize)
            || (*gcheck).r.absmax[2 as i32 as usize]
                < *(*ap).mins.offset(2 as i32 as isize))
        {
            if (*ap).count == (*ap).maxcount {
                crate::src::qcommon::common::Com_Printf(
                    b"SV_AreaEntities: MAXCOUNT\n\x00" as *const u8 as *const libc::c_char,
                );
                return;
            }
            *(*ap).list.offset((*ap).count as isize) =
                check.offset_from(crate::src::server::sv_main::sv.svEntities.as_mut_ptr())
                    as libc::c_long as i32;
            (*ap).count += 1
        }
        check = next
    }
    if (*node).axis == -(1 as i32) {
        return;
        // terminal node
    }
    // recurse down both sides
    if *(*ap).maxs.offset((*node).axis as isize) > (*node).dist {
        SV_AreaEntities_r((*node).children[0 as i32 as usize], ap);
    }
    if *(*ap).mins.offset((*node).axis as isize) < (*node).dist {
        SV_AreaEntities_r((*node).children[1 as i32 as usize], ap);
    };
}
/*
================
SV_AreaEntities
================
*/
#[no_mangle]

pub unsafe extern "C" fn SV_AreaEntities(
    mut mins: *const crate::src::qcommon::q_shared::vec_t,
    mut maxs: *const crate::src::qcommon::q_shared::vec_t,
    mut entityList: *mut i32,
    mut maxcount: i32,
) -> i32 {
    let mut ap: areaParms_t = areaParms_t {
        mins: 0 as *const f32,
        maxs: 0 as *const f32,
        list: 0 as *mut i32,
        count: 0,
        maxcount: 0,
    };
    ap.mins = mins;
    ap.maxs = maxs;
    ap.list = entityList;
    ap.count = 0 as i32;
    ap.maxcount = maxcount;
    SV_AreaEntities_r(sv_worldSectors.as_mut_ptr(), &mut ap);
    return ap.count;
}
// mins and maxs are relative
// if the entire move stays in a solid volume, trace.allsolid will be set,
// trace.startsolid will be set, and trace.fraction will be 0
// if the starting point is in a solid, it will be allowed to move out
// to an open area
// passEntityNum is explicitly excluded from clipping checks (normally ENTITYNUM_NONE)
/*
====================
SV_ClipToEntity

====================
*/
#[no_mangle]

pub unsafe extern "C" fn SV_ClipToEntity(
    mut trace: *mut crate::src::qcommon::q_shared::trace_t,
    mut start: *const crate::src::qcommon::q_shared::vec_t,
    mut mins: *const crate::src::qcommon::q_shared::vec_t,
    mut maxs: *const crate::src::qcommon::q_shared::vec_t,
    mut end: *const crate::src::qcommon::q_shared::vec_t,
    mut entityNum: i32,
    mut contentmask: i32,
    mut capsule: i32,
) {
    let mut touch: *mut crate::g_public_h::sharedEntity_t =
        0 as *mut crate::g_public_h::sharedEntity_t;
    let mut clipHandle: crate::src::qcommon::q_shared::clipHandle_t = 0;
    let mut origin: *mut f32 = 0 as *mut f32;
    let mut angles: *mut f32 = 0 as *mut f32;
    touch = crate::src::server::sv_game::SV_GentityNum(entityNum)
        as *mut crate::g_public_h::sharedEntity_t;
    crate::stdlib::memset(
        trace as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<crate::src::qcommon::q_shared::trace_t>() as libc::c_ulong,
    );
    // if it doesn't have any brushes of a type we
    // are looking for, ignore it
    if contentmask & (*touch).r.contents == 0 {
        (*trace).fraction = 1.0f64 as f32;
        return;
    }
    // might intersect, so do an exact clip
    clipHandle = SV_ClipHandleForEntity(touch);
    origin = (*touch).r.currentOrigin.as_mut_ptr();
    angles = (*touch).r.currentAngles.as_mut_ptr();
    if (*touch).r.bmodel as u64 == 0 {
        angles = crate::src::qcommon::q_math::vec3_origin.as_mut_ptr()
        // boxes don't rotate
    }
    crate::src::qcommon::cm_trace::CM_TransformedBoxTrace(
        trace as *mut crate::src::qcommon::q_shared::trace_t,
        start as *mut f32 as *const crate::src::qcommon::q_shared::vec_t,
        end as *mut f32 as *const crate::src::qcommon::q_shared::vec_t,
        mins as *mut f32,
        maxs as *mut f32,
        clipHandle,
        contentmask,
        origin as *const crate::src::qcommon::q_shared::vec_t,
        angles as *const crate::src::qcommon::q_shared::vec_t,
        capsule,
    );
    if (*trace).fraction < 1 as i32 as f32 {
        (*trace).entityNum = (*touch).s.number
    };
}
/*
====================
SV_ClipMoveToEntities

====================
*/

unsafe extern "C" fn SV_ClipMoveToEntities(mut clip: *mut moveclip_t) {
    let mut i: i32 = 0;
    let mut num: i32 = 0;
    let mut touchlist: [i32; 1024] = [0; 1024];
    let mut touch: *mut crate::g_public_h::sharedEntity_t =
        0 as *mut crate::g_public_h::sharedEntity_t;
    let mut passOwnerNum: i32 = 0;
    let mut trace: crate::src::qcommon::q_shared::trace_t =
        crate::src::qcommon::q_shared::trace_t {
            allsolid: crate::src::qcommon::q_shared::qfalse,
            startsolid: crate::src::qcommon::q_shared::qfalse,
            fraction: 0.,
            endpos: [0.; 3],
            plane: crate::src::qcommon::q_shared::cplane_t {
                normal: [0.; 3],
                dist: 0.,
                type_0: 0,
                signbits: 0,
                pad: [0; 2],
            },
            surfaceFlags: 0,
            contents: 0,
            entityNum: 0,
        };
    let mut clipHandle: crate::src::qcommon::q_shared::clipHandle_t = 0;
    let mut origin: *mut f32 = 0 as *mut f32;
    let mut angles: *mut f32 = 0 as *mut f32;
    num = SV_AreaEntities(
        (*clip).boxmins.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        (*clip).boxmaxs.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        touchlist.as_mut_ptr(),
        (1 as i32) << 10 as i32,
    );
    if (*clip).passEntityNum != ((1 as i32) << 10 as i32) - 1 as i32 {
        passOwnerNum = (*(crate::src::server::sv_game::SV_GentityNum((*clip).passEntityNum)
            as *mut crate::g_public_h::sharedEntity_t))
            .r
            .ownerNum;
        if passOwnerNum == ((1 as i32) << 10 as i32) - 1 as i32 {
            passOwnerNum = -(1 as i32)
        }
    } else {
        passOwnerNum = -(1 as i32)
    }
    let mut current_block_34: u64;
    i = 0 as i32;
    while i < num {
        if (*clip).trace.allsolid as u64 != 0 {
            return;
        }
        touch = crate::src::server::sv_game::SV_GentityNum(touchlist[i as usize])
            as *mut crate::g_public_h::sharedEntity_t;
        // see if we should ignore this entity
        if (*clip).passEntityNum != ((1 as i32) << 10 as i32) - 1 as i32 {
            if touchlist[i as usize] == (*clip).passEntityNum {
                current_block_34 = 13586036798005543211;
            // don't clip against the pass entity
            } else if (*touch).r.ownerNum == (*clip).passEntityNum {
                current_block_34 = 13586036798005543211;
            // don't clip against own missiles
            } else if (*touch).r.ownerNum == passOwnerNum {
                current_block_34 = 13586036798005543211;
            } else {
                current_block_34 = 13472856163611868459;
            }
        } else {
            current_block_34 = 13472856163611868459;
        }
        match current_block_34 {
            13472856163611868459 =>
            // if it doesn't have any brushes of a type we
            // are looking for, ignore it
            {
                if !((*clip).contentmask & (*touch).r.contents == 0) {
                    // might intersect, so do an exact clip
                    clipHandle = SV_ClipHandleForEntity(touch);
                    origin = (*touch).r.currentOrigin.as_mut_ptr();
                    angles = (*touch).r.currentAngles.as_mut_ptr();
                    if (*touch).r.bmodel as u64 == 0 {
                        angles = crate::src::qcommon::q_math::vec3_origin.as_mut_ptr()
                        // boxes don't rotate
                    }
                    crate::src::qcommon::cm_trace::CM_TransformedBoxTrace(
                        &mut trace as *mut _ as *mut crate::src::qcommon::q_shared::trace_t,
                        (*clip).start as *mut f32
                            as *const crate::src::qcommon::q_shared::vec_t,
                        (*clip).end.as_mut_ptr() as *mut f32
                            as *const crate::src::qcommon::q_shared::vec_t,
                        (*clip).mins as *mut f32,
                        (*clip).maxs as *mut f32,
                        clipHandle,
                        (*clip).contentmask,
                        origin as *const crate::src::qcommon::q_shared::vec_t,
                        angles as *const crate::src::qcommon::q_shared::vec_t,
                        (*clip).capsule,
                    );
                    if trace.allsolid as u64 != 0 {
                        (*clip).trace.allsolid = crate::src::qcommon::q_shared::qtrue;
                        trace.entityNum = (*touch).s.number
                    } else if trace.startsolid as u64 != 0 {
                        (*clip).trace.startsolid = crate::src::qcommon::q_shared::qtrue;
                        trace.entityNum = (*touch).s.number
                    }
                    if trace.fraction < (*clip).trace.fraction {
                        let mut oldStart: crate::src::qcommon::q_shared::qboolean =
                            crate::src::qcommon::q_shared::qfalse;
                        // make sure we keep a startsolid from a previous trace
                        oldStart = (*clip).trace.startsolid;
                        trace.entityNum = (*touch).s.number;
                        (*clip).trace = trace;
                        (*clip).trace.startsolid = ::std::mem::transmute::<
                            u32,
                            crate::src::qcommon::q_shared::qboolean,
                        >(
                            (*clip).trace.startsolid as u32 | oldStart as u32,
                        )
                    }
                }
            }
            _ => {}
        }
        i += 1
        // don't clip against other missiles from our owner
    }
}
// returns the CONTENTS_* value from the world and all entities at the given point.
/*
==================
SV_Trace

Moves the given mins/maxs volume through the world from start to end.
passEntityNum and entities owned by passEntityNum are explicitly not checked.
==================
*/
#[no_mangle]

pub unsafe extern "C" fn SV_Trace(
    mut results: *mut crate::src::qcommon::q_shared::trace_t,
    mut start: *const crate::src::qcommon::q_shared::vec_t,
    mut mins: *mut crate::src::qcommon::q_shared::vec_t,
    mut maxs: *mut crate::src::qcommon::q_shared::vec_t,
    mut end: *const crate::src::qcommon::q_shared::vec_t,
    mut passEntityNum: i32,
    mut contentmask: i32,
    mut capsule: i32,
) {
    let mut clip: moveclip_t = moveclip_t {
        boxmins: [0.; 3],
        boxmaxs: [0.; 3],
        mins: 0 as *const f32,
        maxs: 0 as *const f32,
        start: 0 as *const f32,
        end: [0.; 3],
        trace: crate::src::qcommon::q_shared::trace_t {
            allsolid: crate::src::qcommon::q_shared::qfalse,
            startsolid: crate::src::qcommon::q_shared::qfalse,
            fraction: 0.,
            endpos: [0.; 3],
            plane: crate::src::qcommon::q_shared::cplane_t {
                normal: [0.; 3],
                dist: 0.,
                type_0: 0,
                signbits: 0,
                pad: [0; 2],
            },
            surfaceFlags: 0,
            contents: 0,
            entityNum: 0,
        },
        passEntityNum: 0,
        contentmask: 0,
        capsule: 0,
    };
    let mut i: i32 = 0;
    if mins.is_null() {
        mins = crate::src::qcommon::q_math::vec3_origin.as_mut_ptr()
    }
    if maxs.is_null() {
        maxs = crate::src::qcommon::q_math::vec3_origin.as_mut_ptr()
    }
    crate::stdlib::memset(
        &mut clip as *mut moveclip_t as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<moveclip_t>() as libc::c_ulong,
    );
    // clip to world
    crate::src::qcommon::cm_trace::CM_BoxTrace(
        &mut clip.trace as *mut _ as *mut crate::src::qcommon::q_shared::trace_t,
        start,
        end,
        mins,
        maxs,
        0 as i32,
        contentmask,
        capsule,
    );
    clip.trace.entityNum = if clip.trace.fraction as f64 != 1.0f64 {
        ((1 as i32) << 10 as i32) - 2 as i32
    } else {
        ((1 as i32) << 10 as i32) - 1 as i32
    };
    if clip.trace.fraction == 0 as i32 as f32 {
        *results = clip.trace;
        return;
        // blocked immediately by the world
    }
    clip.contentmask = contentmask;
    clip.start = start;
    //	VectorCopy( clip.trace.endpos, clip.end );
    clip.end[0 as i32 as usize] = *end.offset(0 as i32 as isize);
    clip.end[1 as i32 as usize] = *end.offset(1 as i32 as isize);
    clip.end[2 as i32 as usize] = *end.offset(2 as i32 as isize);
    clip.mins = mins as *const f32;
    clip.maxs = maxs as *const f32;
    clip.passEntityNum = passEntityNum;
    clip.capsule = capsule;
    // create the bounding box of the entire move
    // we can limit it to the part of the move not
    // already clipped off by the world, which can be
    // a significant savings for line of sight and shot traces
    i = 0 as i32;
    while i < 3 as i32 {
        if *end.offset(i as isize) > *start.offset(i as isize) {
            clip.boxmins[i as usize] = *clip.start.offset(i as isize)
                + *clip.mins.offset(i as isize)
                - 1 as i32 as f32;
            clip.boxmaxs[i as usize] = clip.end[i as usize]
                + *clip.maxs.offset(i as isize)
                + 1 as i32 as f32
        } else {
            clip.boxmins[i as usize] = clip.end[i as usize] + *clip.mins.offset(i as isize)
                - 1 as i32 as f32;
            clip.boxmaxs[i as usize] = *clip.start.offset(i as isize)
                + *clip.maxs.offset(i as isize)
                + 1 as i32 as f32
        }
        i += 1
    }
    // clip to other solid entities
    SV_ClipMoveToEntities(&mut clip);
    *results = clip.trace;
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
// server.h
//=============================================================================
// !!! MUST NOT CHANGE, SERVER AND
// GAME BOTH REFERENCE !!!
// for delta compression of initial sighting
// if -1, use headnode instead
// if all the clusters don't fit in clusternums
// used to prevent double adding from portal views
// no map loaded
// spawning level entities
// actively running
// if true, send configstring changes during SS_LOADING
// changes each server start
// serverId before a map_restart
// the feed key that we use to compute the pure checksum strings
// https://zerowing.idsoftware.com/bugzilla/show_bug.cgi?id=475
// the serverId associated with the current checksumFeed (always <= serverId)
// incremented for each snapshot built
// <= 1000 / sv_frame->value
// when time > nextFrameTime, process world
// used during game VM init
// the game virtual machine will update these on init and changes
// current number, <= MAX_GENTITIES
// will be > sizeof(playerState_t) due to game private data
// portalarea visibility bits
// into the circular sv_packet_entities[]
// the entities MUST be in increasing state number
// order, otherwise the delta compression will fail
// time the message was transmitted
// time the message was acked
// used to rate drop packets
// can be reused for a new connection
// client has been disconnected, but don't reuse
// connection for a couple seconds
// has been assigned to a client_t, but no gamestate yet
// gamestate has been sent, but client hasn't sent a usercmd
// client is fully in game
// valid command string for SV_Netchan_Encode
// name, etc
// last added reliable message, not necessarily sent or acknowledged yet
// last acknowledged reliable message
// last sent reliable message, not necessarily acknowledged yet
// netchan->outgoingSequence of gamestate
// for delta compression
// reliable client message sequence
// SV_GentityNum(clientnum)
// extracted from userinfo, high bits masked
// downloading
// if not empty string, we are downloading
// file being downloaded
// total bytes (can't use EOF because of paks)
// bytes sent
// last block we sent to the client, awaiting ack
// current block number
// last block we xmited
// the buffers for the download blocks
// We have sent the EOF block
// time we last got an ack from the client
// frame last client usercmd message
// svs.time when another reliable command will be allowed
// svs.time when packet was last received
// svs.time when connection started
// svs.time of last sent snapshot
// true if nextSnapshotTime was set based on rate instead of snapshotMsec
// must timeout a few frames in a row so debugging doesn't break
// updates can be delta'd from here
// bytes / second
// requests a snapshot every snapshotMsec unless rate choked
// TTimo - additional flag to distinguish between a bad pure checksum, and no cp command at all
// TTimo
// queuing outgoing fragmented messages to send them properly, without udp packet bursts
// in case large fragmented messages are stacking up
// buffer them into this queue, and hand them out to netchan as needed
//=============================================================================
// MAX_CHALLENGES is made large to prevent a denial
// of service attack that could cycle all of them
// out before legitimate users connected
// Allow a certain amount of challenges to have the same IP address
// to make it a bit harder to DOS one single IP address from connecting
// while not allowing a single ip to grab all challenge resources
// challenge number coming from the client
// time the last packet was sent to the autherize server
// time the challenge response was sent to client
// time the adr was first used, for authorize timeout checks
// this structure will be cleared only when the game dll changes
// sv_init has completed
// will be strictly increasing across level changes
// ^= SNAPFLAG_SERVERCOUNT every SV_SpawnServer()
// [sv_maxclients->integer];
// sv_maxclients->integer*PACKET_BACKUP*MAX_SNAPSHOT_ENTITIES
// next snapshotEntities to use
// [numSnapshotEntities]
// to prevent invalid IPs from connecting
// for rcon return messages
// authorize server address
// next svs.time that server should do dns lookup for master server
// Structure for managing bans
// For a CIDR-Notation type suffix
//=============================================================================
// persistant server info across maps
// cleared each map
// game virtual machine
//===========================================================
//
// sv_main.c
//
//
// sv_init.c
//
//
// sv_client.c
//
//
// sv_ccmds.c
//
//
// sv_snapshot.c
//
//
// sv_game.c
//
//
// sv_bot.c
//
//============================================================
//
// high level object sorting to reduce interaction tests
//
// called after the world model has been loaded, before linking any entities
// call before removing an entity, and before trying to move one,
// so it doesn't clip against itself
// Needs to be called any time an entity changes origin, mins, maxs,
// or solid.  Automatically unlinks if needed.
// sets ent->r.absmin and ent->r.absmax
// sets ent->leafnums[] for pvs determination even if the entity
// is not solid
// fills in a table of entity numbers with entities that have bounding boxes
// that intersect the given area.  It is possible for a non-axial bmodel
// to be returned that doesn't actually intersect the area on an exact
// test.
// returns the number of pointers filled in
// The world entity is never returned in this list.
/*
=============
SV_PointContents
=============
*/
#[no_mangle]

pub unsafe extern "C" fn SV_PointContents(
    mut p: *const crate::src::qcommon::q_shared::vec_t,
    mut passEntityNum: i32,
) -> i32 {
    let mut touch: [i32; 1024] = [0; 1024];
    let mut hit: *mut crate::g_public_h::sharedEntity_t =
        0 as *mut crate::g_public_h::sharedEntity_t;
    let mut i: i32 = 0;
    let mut num: i32 = 0;
    let mut contents: i32 = 0;
    let mut c2: i32 = 0;
    let mut clipHandle: crate::src::qcommon::q_shared::clipHandle_t = 0;
    let mut angles: *mut f32 = 0 as *mut f32;
    // get base contents from world
    contents = crate::src::qcommon::cm_test::CM_PointContents(p, 0 as i32);
    // or in contents from all the other entities
    num = SV_AreaEntities(
        p,
        p,
        touch.as_mut_ptr(),
        (1 as i32) << 10 as i32,
    );
    i = 0 as i32;
    while i < num {
        if !(touch[i as usize] == passEntityNum) {
            hit = crate::src::server::sv_game::SV_GentityNum(touch[i as usize])
                as *mut crate::g_public_h::sharedEntity_t;
            // might intersect, so do an exact clip
            clipHandle = SV_ClipHandleForEntity(hit);
            angles = (*hit).r.currentAngles.as_mut_ptr();
            if (*hit).r.bmodel as u64 == 0 {
                angles = crate::src::qcommon::q_math::vec3_origin.as_mut_ptr()
                // boxes don't rotate
            }
            c2 = crate::src::qcommon::cm_test::CM_TransformedPointContents(
                p,
                clipHandle,
                (*hit).r.currentOrigin.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
                angles as *const crate::src::qcommon::q_shared::vec_t,
            );
            contents |= c2
        }
        i += 1
    }
    return contents;
}

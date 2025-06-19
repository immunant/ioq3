use ::libc;

pub mod q_shared_h {

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

pub use crate::bg_local_h::pml_t;
pub use crate::bg_public_h::pmove_t;
pub use crate::bg_public_h::C2RustUnnamed_0;
pub use crate::bg_public_h::EV_BULLET;
pub use crate::bg_public_h::EV_BULLET_HIT_FLESH;
pub use crate::bg_public_h::EV_BULLET_HIT_WALL;
pub use crate::bg_public_h::EV_CHANGE_WEAPON;
pub use crate::bg_public_h::EV_DEATH1;
pub use crate::bg_public_h::EV_DEATH2;
pub use crate::bg_public_h::EV_DEATH3;
pub use crate::bg_public_h::EV_DEBUG_LINE;
pub use crate::bg_public_h::EV_FALL_FAR;
pub use crate::bg_public_h::EV_FALL_MEDIUM;
pub use crate::bg_public_h::EV_FALL_SHORT;
pub use crate::bg_public_h::EV_FIRE_WEAPON;
pub use crate::bg_public_h::EV_FOOTSPLASH;
pub use crate::bg_public_h::EV_FOOTSTEP;
pub use crate::bg_public_h::EV_FOOTSTEP_METAL;
pub use crate::bg_public_h::EV_FOOTWADE;
pub use crate::bg_public_h::EV_GENERAL_SOUND;
pub use crate::bg_public_h::EV_GIB_PLAYER;
pub use crate::bg_public_h::EV_GLOBAL_ITEM_PICKUP;
pub use crate::bg_public_h::EV_GLOBAL_SOUND;
pub use crate::bg_public_h::EV_GLOBAL_TEAM_SOUND;
pub use crate::bg_public_h::EV_GRENADE_BOUNCE;
pub use crate::bg_public_h::EV_INVUL_IMPACT;
pub use crate::bg_public_h::EV_ITEM_PICKUP;
pub use crate::bg_public_h::EV_ITEM_POP;
pub use crate::bg_public_h::EV_ITEM_RESPAWN;
pub use crate::bg_public_h::EV_JUICED;
pub use crate::bg_public_h::EV_JUMP;
pub use crate::bg_public_h::EV_JUMP_PAD;
pub use crate::bg_public_h::EV_KAMIKAZE;
pub use crate::bg_public_h::EV_LIGHTNINGBOLT;
pub use crate::bg_public_h::EV_MISSILE_HIT;
pub use crate::bg_public_h::EV_MISSILE_MISS;
pub use crate::bg_public_h::EV_MISSILE_MISS_METAL;
pub use crate::bg_public_h::EV_NOAMMO;
pub use crate::bg_public_h::EV_NONE;
pub use crate::bg_public_h::EV_OBELISKEXPLODE;
pub use crate::bg_public_h::EV_OBELISKPAIN;
pub use crate::bg_public_h::EV_OBITUARY;
pub use crate::bg_public_h::EV_PAIN;
pub use crate::bg_public_h::EV_PLAYER_TELEPORT_IN;
pub use crate::bg_public_h::EV_PLAYER_TELEPORT_OUT;
pub use crate::bg_public_h::EV_POWERUP_BATTLESUIT;
pub use crate::bg_public_h::EV_POWERUP_QUAD;
pub use crate::bg_public_h::EV_POWERUP_REGEN;
pub use crate::bg_public_h::EV_PROXIMITY_MINE_STICK;
pub use crate::bg_public_h::EV_PROXIMITY_MINE_TRIGGER;
pub use crate::bg_public_h::EV_RAILTRAIL;
pub use crate::bg_public_h::EV_SCOREPLUM;
pub use crate::bg_public_h::EV_SHOTGUN;
pub use crate::bg_public_h::EV_STEP_12;
pub use crate::bg_public_h::EV_STEP_16;
pub use crate::bg_public_h::EV_STEP_4;
pub use crate::bg_public_h::EV_STEP_8;
pub use crate::bg_public_h::EV_STOPLOOPINGSOUND;
pub use crate::bg_public_h::EV_SWIM;
pub use crate::bg_public_h::EV_TAUNT;
pub use crate::bg_public_h::EV_TAUNT_FOLLOWME;
pub use crate::bg_public_h::EV_TAUNT_GETFLAG;
pub use crate::bg_public_h::EV_TAUNT_GUARDBASE;
pub use crate::bg_public_h::EV_TAUNT_NO;
pub use crate::bg_public_h::EV_TAUNT_PATROL;
pub use crate::bg_public_h::EV_TAUNT_YES;
pub use crate::bg_public_h::EV_USE_ITEM0;
pub use crate::bg_public_h::EV_USE_ITEM1;
pub use crate::bg_public_h::EV_USE_ITEM10;
pub use crate::bg_public_h::EV_USE_ITEM11;
pub use crate::bg_public_h::EV_USE_ITEM12;
pub use crate::bg_public_h::EV_USE_ITEM13;
pub use crate::bg_public_h::EV_USE_ITEM14;
pub use crate::bg_public_h::EV_USE_ITEM15;
pub use crate::bg_public_h::EV_USE_ITEM2;
pub use crate::bg_public_h::EV_USE_ITEM3;
pub use crate::bg_public_h::EV_USE_ITEM4;
pub use crate::bg_public_h::EV_USE_ITEM5;
pub use crate::bg_public_h::EV_USE_ITEM6;
pub use crate::bg_public_h::EV_USE_ITEM7;
pub use crate::bg_public_h::EV_USE_ITEM8;
pub use crate::bg_public_h::EV_USE_ITEM9;
pub use crate::bg_public_h::EV_WATER_CLEAR;
pub use crate::bg_public_h::EV_WATER_LEAVE;
pub use crate::bg_public_h::EV_WATER_TOUCH;
pub use crate::bg_public_h::EV_WATER_UNDER;
pub use crate::src::game::bg_pmove::c_pmove;
pub use crate::src::game::bg_pmove::pm;
pub use crate::src::game::bg_pmove::pml;
pub use crate::src::game::bg_pmove::PM_AddEvent;
pub use crate::src::game::bg_pmove::PM_AddTouchEnt;
pub use crate::src::game::bg_pmove::PM_ClipVelocity;
pub use crate::src::game::bg_slidemove::q_shared_h::CrossProduct;
pub use crate::src::game::g_main::Com_Printf;
pub use crate::src::qcommon::q_math::VectorNormalize;
pub use crate::src::qcommon::q_math::VectorNormalize2;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::cplane_s;
pub use crate::src::qcommon::q_shared::cplane_t;
pub use crate::src::qcommon::q_shared::playerState_s;
pub use crate::src::qcommon::q_shared::playerState_t;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::trace_t;
pub use crate::src::qcommon::q_shared::usercmd_s;
pub use crate::src::qcommon::q_shared::usercmd_t;
pub use crate::src::qcommon::q_shared::vec3_t;
pub use crate::src::qcommon::q_shared::vec_t;
#[no_mangle]

pub unsafe extern "C" fn PM_SlideMove(
    mut gravity: crate::src::qcommon::q_shared::qboolean,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut bumpcount: i32 = 0;
    let mut numbumps: i32 = 0;
    let mut dir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut d: f32 = 0.;
    let mut numplanes: i32 = 0;
    let mut planes: [crate::src::qcommon::q_shared::vec3_t; 5] = [[0.; 3]; 5];
    let mut primal_velocity: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut clipVelocity: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
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
    let mut end: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut time_left: f32 = 0.;
    let mut into: f32 = 0.;
    let mut endVelocity: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut endClipVelocity: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    numbumps = 4 as i32;
    primal_velocity[0 as i32 as usize] =
        (*(*crate::src::game::bg_pmove::pm).ps).velocity[0 as i32 as usize];
    primal_velocity[1 as i32 as usize] =
        (*(*crate::src::game::bg_pmove::pm).ps).velocity[1 as i32 as usize];
    primal_velocity[2 as i32 as usize] =
        (*(*crate::src::game::bg_pmove::pm).ps).velocity[2 as i32 as usize];
    if gravity as u64 != 0 {
        endVelocity[0 as i32 as usize] =
            (*(*crate::src::game::bg_pmove::pm).ps).velocity[0 as i32 as usize];
        endVelocity[1 as i32 as usize] =
            (*(*crate::src::game::bg_pmove::pm).ps).velocity[1 as i32 as usize];
        endVelocity[2 as i32 as usize] =
            (*(*crate::src::game::bg_pmove::pm).ps).velocity[2 as i32 as usize];
        endVelocity[2 as i32 as usize] -= (*(*crate::src::game::bg_pmove::pm).ps).gravity as f32
            * crate::src::game::bg_pmove::pml.frametime;
        (*(*crate::src::game::bg_pmove::pm).ps).velocity[2 as i32 as usize] =
            (((*(*crate::src::game::bg_pmove::pm).ps).velocity[2 as i32 as usize]
                + endVelocity[2 as i32 as usize]) as f64
                * 0.5f64) as crate::src::qcommon::q_shared::vec_t;
        primal_velocity[2 as i32 as usize] = endVelocity[2 as i32 as usize];
        if crate::src::game::bg_pmove::pml.groundPlane as u64 != 0 {
            // slide along the ground plane
            crate::src::game::bg_pmove::PM_ClipVelocity(
                (*(*crate::src::game::bg_pmove::pm).ps)
                    .velocity
                    .as_mut_ptr(),
                crate::src::game::bg_pmove::pml
                    .groundTrace
                    .plane
                    .normal
                    .as_mut_ptr(),
                (*(*crate::src::game::bg_pmove::pm).ps)
                    .velocity
                    .as_mut_ptr(),
                1.001f32,
            );
        }
    }
    time_left = crate::src::game::bg_pmove::pml.frametime;
    // never turn against the ground plane
    if crate::src::game::bg_pmove::pml.groundPlane as u64 != 0 {
        numplanes = 1 as i32;
        planes[0 as i32 as usize][0 as i32 as usize] =
            crate::src::game::bg_pmove::pml.groundTrace.plane.normal[0 as i32 as usize];
        planes[0 as i32 as usize][1 as i32 as usize] =
            crate::src::game::bg_pmove::pml.groundTrace.plane.normal[1 as i32 as usize];
        planes[0 as i32 as usize][2 as i32 as usize] =
            crate::src::game::bg_pmove::pml.groundTrace.plane.normal[2 as i32 as usize]
    } else {
        numplanes = 0 as i32
    }
    // never turn against original velocity
    crate::src::qcommon::q_math::VectorNormalize2(
        (*(*crate::src::game::bg_pmove::pm).ps)
            .velocity
            .as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        planes[numplanes as usize].as_mut_ptr(),
    );
    numplanes += 1;
    bumpcount = 0 as i32;
    while bumpcount < numbumps {
        // calculate position we are trying to move to
        end[0 as i32 as usize] = (*(*crate::src::game::bg_pmove::pm).ps).origin[0 as i32 as usize]
            + (*(*crate::src::game::bg_pmove::pm).ps).velocity[0 as i32 as usize] * time_left;
        end[1 as i32 as usize] = (*(*crate::src::game::bg_pmove::pm).ps).origin[1 as i32 as usize]
            + (*(*crate::src::game::bg_pmove::pm).ps).velocity[1 as i32 as usize] * time_left;
        end[2 as i32 as usize] = (*(*crate::src::game::bg_pmove::pm).ps).origin[2 as i32 as usize]
            + (*(*crate::src::game::bg_pmove::pm).ps).velocity[2 as i32 as usize] * time_left;
        // see if we can make it there
        (*crate::src::game::bg_pmove::pm)
            .trace
            .expect("non-null function pointer")(
            &mut trace,
            (*(*crate::src::game::bg_pmove::pm).ps).origin.as_mut_ptr()
                as *const crate::src::qcommon::q_shared::vec_t,
            (*crate::src::game::bg_pmove::pm).mins.as_mut_ptr()
                as *const crate::src::qcommon::q_shared::vec_t,
            (*crate::src::game::bg_pmove::pm).maxs.as_mut_ptr()
                as *const crate::src::qcommon::q_shared::vec_t,
            end.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            (*(*crate::src::game::bg_pmove::pm).ps).clientNum,
            (*crate::src::game::bg_pmove::pm).tracemask,
        );
        if trace.allsolid as u64 != 0 {
            // entity is completely trapped in another solid
            (*(*crate::src::game::bg_pmove::pm).ps).velocity[2 as i32 as usize] =
                0 as i32 as crate::src::qcommon::q_shared::vec_t; // don't build up falling damage, but allow sideways acceleration
            return crate::src::qcommon::q_shared::qtrue;
        }
        if trace.fraction > 0 as i32 as f32 {
            // actually covered some distance
            (*(*crate::src::game::bg_pmove::pm).ps).origin[0 as i32 as usize] =
                trace.endpos[0 as i32 as usize];
            (*(*crate::src::game::bg_pmove::pm).ps).origin[1 as i32 as usize] =
                trace.endpos[1 as i32 as usize];
            (*(*crate::src::game::bg_pmove::pm).ps).origin[2 as i32 as usize] =
                trace.endpos[2 as i32 as usize]
        }
        if trace.fraction == 1 as i32 as f32 {
            break;
        }
        // save entity for contact
        crate::src::game::bg_pmove::PM_AddTouchEnt(trace.entityNum);
        time_left -= time_left * trace.fraction;
        if numplanes >= 5 as i32 {
            // this shouldn't really happen
            (*(*crate::src::game::bg_pmove::pm).ps).velocity[2 as i32 as usize] =
                0 as i32 as crate::src::qcommon::q_shared::vec_t;
            (*(*crate::src::game::bg_pmove::pm).ps).velocity[1 as i32 as usize] =
                (*(*crate::src::game::bg_pmove::pm).ps).velocity[2 as i32 as usize];
            (*(*crate::src::game::bg_pmove::pm).ps).velocity[0 as i32 as usize] =
                (*(*crate::src::game::bg_pmove::pm).ps).velocity[1 as i32 as usize];
            return crate::src::qcommon::q_shared::qtrue;
        }
        //
        // if this is the same plane we hit before, nudge velocity
        // out along it, which fixes some epsilon issues with
        // non-axial planes
        //
        i = 0 as i32;
        while i < numplanes {
            if (trace.plane.normal[0 as i32 as usize] * planes[i as usize][0 as i32 as usize]
                + trace.plane.normal[1 as i32 as usize] * planes[i as usize][1 as i32 as usize]
                + trace.plane.normal[2 as i32 as usize] * planes[i as usize][2 as i32 as usize])
                as f64
                > 0.99f64
            {
                (*(*crate::src::game::bg_pmove::pm).ps).velocity[0 as i32 as usize] =
                    trace.plane.normal[0 as i32 as usize]
                        + (*(*crate::src::game::bg_pmove::pm).ps).velocity[0 as i32 as usize];
                (*(*crate::src::game::bg_pmove::pm).ps).velocity[1 as i32 as usize] =
                    trace.plane.normal[1 as i32 as usize]
                        + (*(*crate::src::game::bg_pmove::pm).ps).velocity[1 as i32 as usize];
                (*(*crate::src::game::bg_pmove::pm).ps).velocity[2 as i32 as usize] =
                    trace.plane.normal[2 as i32 as usize]
                        + (*(*crate::src::game::bg_pmove::pm).ps).velocity[2 as i32 as usize];
                break;
            } else {
                i += 1
            }
        }
        if !(i < numplanes) {
            planes[numplanes as usize][0 as i32 as usize] = trace.plane.normal[0 as i32 as usize];
            planes[numplanes as usize][1 as i32 as usize] = trace.plane.normal[1 as i32 as usize];
            planes[numplanes as usize][2 as i32 as usize] = trace.plane.normal[2 as i32 as usize];
            numplanes += 1;
            //
            // modify velocity so it parallels all of the clip planes
            //
            // find a plane that it enters
            i = 0 as i32;
            while i < numplanes {
                into = (*(*crate::src::game::bg_pmove::pm).ps).velocity[0 as i32 as usize]
                    * planes[i as usize][0 as i32 as usize]
                    + (*(*crate::src::game::bg_pmove::pm).ps).velocity[1 as i32 as usize]
                        * planes[i as usize][1 as i32 as usize]
                    + (*(*crate::src::game::bg_pmove::pm).ps).velocity[2 as i32 as usize]
                        * planes[i as usize][2 as i32 as usize];
                if into as f64 >= 0.1f64 {
                    i += 1
                // move doesn't interact with the plane
                } else {
                    // see how hard we are hitting things
                    if -into > crate::src::game::bg_pmove::pml.impactSpeed {
                        crate::src::game::bg_pmove::pml.impactSpeed = -into
                    }
                    // slide along the plane
                    crate::src::game::bg_pmove::PM_ClipVelocity(
                        (*(*crate::src::game::bg_pmove::pm).ps)
                            .velocity
                            .as_mut_ptr(),
                        planes[i as usize].as_mut_ptr(),
                        clipVelocity.as_mut_ptr(),
                        1.001f32,
                    );
                    if gravity as u64 != 0 {
                        // slide along the plane
                        crate::src::game::bg_pmove::PM_ClipVelocity(
                            endVelocity.as_mut_ptr(),
                            planes[i as usize].as_mut_ptr(),
                            endClipVelocity.as_mut_ptr(),
                            1.001f32,
                        );
                    }
                    // see if there is a second plane that the new move enters
                    j = 0 as i32;
                    while j < numplanes {
                        if !(j == i) {
                            if !((clipVelocity[0 as i32 as usize]
                                * planes[j as usize][0 as i32 as usize]
                                + clipVelocity[1 as i32 as usize]
                                    * planes[j as usize][1 as i32 as usize]
                                + clipVelocity[2 as i32 as usize]
                                    * planes[j as usize][2 as i32 as usize])
                                as f64
                                >= 0.1f64)
                            {
                                // try clipping the move to the plane
                                crate::src::game::bg_pmove::PM_ClipVelocity(
                                    clipVelocity.as_mut_ptr(),
                                    planes[j as usize].as_mut_ptr(),
                                    clipVelocity.as_mut_ptr(),
                                    1.001f32,
                                );
                                if gravity as u64 != 0 {
                                    crate::src::game::bg_pmove::PM_ClipVelocity(
                                        endClipVelocity.as_mut_ptr(),
                                        planes[j as usize].as_mut_ptr(),
                                        endClipVelocity.as_mut_ptr(),
                                        1.001f32,
                                    );
                                }
                                // see if it goes back into the first clip plane
                                if !(clipVelocity[0 as i32 as usize]
                                    * planes[i as usize][0 as i32 as usize]
                                    + clipVelocity[1 as i32 as usize]
                                        * planes[i as usize][1 as i32 as usize]
                                    + clipVelocity[2 as i32 as usize]
                                        * planes[i as usize][2 as i32 as usize]
                                    >= 0 as i32 as f32)
                                {
                                    // slide the original velocity along the crease
                                    CrossProduct(
                                        planes[i as usize].as_mut_ptr()
                                            as *const crate::src::qcommon::q_shared::vec_t,
                                        planes[j as usize].as_mut_ptr()
                                            as *const crate::src::qcommon::q_shared::vec_t,
                                        dir.as_mut_ptr(),
                                    );
                                    crate::src::qcommon::q_math::VectorNormalize(dir.as_mut_ptr());
                                    d = dir[0 as i32 as usize]
                                        * (*(*crate::src::game::bg_pmove::pm).ps).velocity
                                            [0 as i32 as usize]
                                        + dir[1 as i32 as usize]
                                            * (*(*crate::src::game::bg_pmove::pm).ps).velocity
                                                [1 as i32 as usize]
                                        + dir[2 as i32 as usize]
                                            * (*(*crate::src::game::bg_pmove::pm).ps).velocity
                                                [2 as i32 as usize];
                                    clipVelocity[0 as i32 as usize] = dir[0 as i32 as usize] * d;
                                    clipVelocity[1 as i32 as usize] = dir[1 as i32 as usize] * d;
                                    clipVelocity[2 as i32 as usize] = dir[2 as i32 as usize] * d;
                                    if gravity as u64 != 0 {
                                        CrossProduct(
                                            planes[i as usize].as_mut_ptr()
                                                as *const crate::src::qcommon::q_shared::vec_t,
                                            planes[j as usize].as_mut_ptr()
                                                as *const crate::src::qcommon::q_shared::vec_t,
                                            dir.as_mut_ptr(),
                                        );
                                        crate::src::qcommon::q_math::VectorNormalize(
                                            dir.as_mut_ptr(),
                                        );
                                        d = dir[0 as i32 as usize] * endVelocity[0 as i32 as usize]
                                            + dir[1 as i32 as usize]
                                                * endVelocity[1 as i32 as usize]
                                            + dir[2 as i32 as usize]
                                                * endVelocity[2 as i32 as usize];
                                        endClipVelocity[0 as i32 as usize] =
                                            dir[0 as i32 as usize] * d;
                                        endClipVelocity[1 as i32 as usize] =
                                            dir[1 as i32 as usize] * d;
                                        endClipVelocity[2 as i32 as usize] =
                                            dir[2 as i32 as usize] * d
                                    }
                                    // see if there is a third plane the the new move enters
                                    k = 0 as i32;
                                    while k < numplanes {
                                        if !(k == i || k == j) {
                                            if !((clipVelocity[0 as i32 as usize]
                                                * planes[k as usize][0 as i32 as usize]
                                                + clipVelocity[1 as i32 as usize]
                                                    * planes[k as usize][1 as i32 as usize]
                                                + clipVelocity[2 as i32 as usize]
                                                    * planes[k as usize][2 as i32 as usize])
                                                as f64
                                                >= 0.1f64)
                                            {
                                                // stop dead at a tripple plane interaction
                                                (*(*crate::src::game::bg_pmove::pm).ps).velocity
                                                    [2 as i32 as usize] = 0 as i32
                                                    as crate::src::qcommon::q_shared::vec_t;
                                                (*(*crate::src::game::bg_pmove::pm).ps).velocity
                                                    [1 as i32 as usize] =
                                                    (*(*crate::src::game::bg_pmove::pm).ps)
                                                        .velocity
                                                        [2 as i32 as usize];
                                                (*(*crate::src::game::bg_pmove::pm).ps).velocity
                                                    [0 as i32 as usize] =
                                                    (*(*crate::src::game::bg_pmove::pm).ps)
                                                        .velocity
                                                        [1 as i32 as usize];
                                                return crate::src::qcommon::q_shared::qtrue;
                                            }
                                        }
                                        k += 1
                                        // move doesn't interact with the plane
                                    }
                                }
                            }
                        }
                        j += 1
                        // move doesn't interact with the plane
                    }
                    // if we have fixed all interactions, try another move
                    (*(*crate::src::game::bg_pmove::pm).ps).velocity[0 as i32 as usize] =
                        clipVelocity[0 as i32 as usize];
                    (*(*crate::src::game::bg_pmove::pm).ps).velocity[1 as i32 as usize] =
                        clipVelocity[1 as i32 as usize];
                    (*(*crate::src::game::bg_pmove::pm).ps).velocity[2 as i32 as usize] =
                        clipVelocity[2 as i32 as usize];
                    if gravity as u64 != 0 {
                        endVelocity[0 as i32 as usize] = endClipVelocity[0 as i32 as usize];
                        endVelocity[1 as i32 as usize] = endClipVelocity[1 as i32 as usize];
                        endVelocity[2 as i32 as usize] = endClipVelocity[2 as i32 as usize]
                    }
                    break;
                }
            }
        }
        bumpcount += 1
    }
    if gravity as u64 != 0 {
        (*(*crate::src::game::bg_pmove::pm).ps).velocity[0 as i32 as usize] =
            endVelocity[0 as i32 as usize];
        (*(*crate::src::game::bg_pmove::pm).ps).velocity[1 as i32 as usize] =
            endVelocity[1 as i32 as usize];
        (*(*crate::src::game::bg_pmove::pm).ps).velocity[2 as i32 as usize] =
            endVelocity[2 as i32 as usize]
    }
    // don't change velocity if in a timer (FIXME: is this correct?)
    if (*(*crate::src::game::bg_pmove::pm).ps).pm_time != 0 {
        (*(*crate::src::game::bg_pmove::pm).ps).velocity[0 as i32 as usize] =
            primal_velocity[0 as i32 as usize];
        (*(*crate::src::game::bg_pmove::pm).ps).velocity[1 as i32 as usize] =
            primal_velocity[1 as i32 as usize];
        (*(*crate::src::game::bg_pmove::pm).ps).velocity[2 as i32 as usize] =
            primal_velocity[2 as i32 as usize]
    }
    return (bumpcount != 0 as i32) as i32 as crate::src::qcommon::q_shared::qboolean;
}
/*
==================
PM_StepSlideMove

==================
*/
#[no_mangle]

pub unsafe extern "C" fn PM_StepSlideMove(mut gravity: crate::src::qcommon::q_shared::qboolean) {
    let mut start_o: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut start_v: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    //	vec3_t		down_o, down_v;
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
    //	float		down_dist, up_dist;
    //	vec3_t		delta, delta2;
    let mut up: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut down: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut stepSize: f32 = 0.;
    start_o[0 as i32 as usize] = (*(*crate::src::game::bg_pmove::pm).ps).origin[0 as i32 as usize];
    start_o[1 as i32 as usize] = (*(*crate::src::game::bg_pmove::pm).ps).origin[1 as i32 as usize];
    start_o[2 as i32 as usize] = (*(*crate::src::game::bg_pmove::pm).ps).origin[2 as i32 as usize];
    start_v[0 as i32 as usize] =
        (*(*crate::src::game::bg_pmove::pm).ps).velocity[0 as i32 as usize];
    start_v[1 as i32 as usize] =
        (*(*crate::src::game::bg_pmove::pm).ps).velocity[1 as i32 as usize];
    start_v[2 as i32 as usize] =
        (*(*crate::src::game::bg_pmove::pm).ps).velocity[2 as i32 as usize];
    if PM_SlideMove(gravity) as u32 == 0 as i32 as u32 {
        return;
        // we got exactly where we wanted to go first try
    }
    down[0 as i32 as usize] = start_o[0 as i32 as usize];
    down[1 as i32 as usize] = start_o[1 as i32 as usize];
    down[2 as i32 as usize] = start_o[2 as i32 as usize];
    down[2 as i32 as usize] -= 18 as i32 as f32;
    (*crate::src::game::bg_pmove::pm)
        .trace
        .expect("non-null function pointer")(
        &mut trace,
        start_o.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        (*crate::src::game::bg_pmove::pm).mins.as_mut_ptr()
            as *const crate::src::qcommon::q_shared::vec_t,
        (*crate::src::game::bg_pmove::pm).maxs.as_mut_ptr()
            as *const crate::src::qcommon::q_shared::vec_t,
        down.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        (*(*crate::src::game::bg_pmove::pm).ps).clientNum,
        (*crate::src::game::bg_pmove::pm).tracemask,
    );
    up[0 as i32 as usize] = 0 as i32 as crate::src::qcommon::q_shared::vec_t;
    up[1 as i32 as usize] = 0 as i32 as crate::src::qcommon::q_shared::vec_t;
    up[2 as i32 as usize] = 1 as i32 as crate::src::qcommon::q_shared::vec_t;
    // never step up when you still have up velocity
    if (*(*crate::src::game::bg_pmove::pm).ps).velocity[2 as i32 as usize] > 0 as i32 as f32
        && (trace.fraction as f64 == 1.0f64
            || ((trace.plane.normal[0 as i32 as usize] * up[0 as i32 as usize]
                + trace.plane.normal[1 as i32 as usize] * up[1 as i32 as usize]
                + trace.plane.normal[2 as i32 as usize] * up[2 as i32 as usize])
                as f64)
                < 0.7f64)
    {
        return;
    }
    //VectorCopy (pm->ps->origin, down_o);
    //VectorCopy (pm->ps->velocity, down_v);
    up[0 as i32 as usize] = start_o[0 as i32 as usize];
    up[1 as i32 as usize] = start_o[1 as i32 as usize];
    up[2 as i32 as usize] = start_o[2 as i32 as usize];
    up[2 as i32 as usize] += 18 as i32 as f32;
    // test the player position if they were a stepheight higher
    (*crate::src::game::bg_pmove::pm)
        .trace
        .expect("non-null function pointer")(
        &mut trace,
        start_o.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        (*crate::src::game::bg_pmove::pm).mins.as_mut_ptr()
            as *const crate::src::qcommon::q_shared::vec_t,
        (*crate::src::game::bg_pmove::pm).maxs.as_mut_ptr()
            as *const crate::src::qcommon::q_shared::vec_t,
        up.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        (*(*crate::src::game::bg_pmove::pm).ps).clientNum,
        (*crate::src::game::bg_pmove::pm).tracemask,
    );
    if trace.allsolid as u64 != 0 {
        if (*crate::src::game::bg_pmove::pm).debugLevel != 0 {
            crate::src::game::g_main::Com_Printf(
                b"%i:bend can\'t step\n\x00" as *const u8 as *const libc::c_char,
                crate::src::game::bg_pmove::c_pmove,
            );
        }
        return;
        // can't step up
    }
    stepSize = trace.endpos[2 as i32 as usize] - start_o[2 as i32 as usize];
    // try slidemove from this position
    (*(*crate::src::game::bg_pmove::pm).ps).origin[0 as i32 as usize] =
        trace.endpos[0 as i32 as usize];
    (*(*crate::src::game::bg_pmove::pm).ps).origin[1 as i32 as usize] =
        trace.endpos[1 as i32 as usize];
    (*(*crate::src::game::bg_pmove::pm).ps).origin[2 as i32 as usize] =
        trace.endpos[2 as i32 as usize];
    (*(*crate::src::game::bg_pmove::pm).ps).velocity[0 as i32 as usize] =
        start_v[0 as i32 as usize];
    (*(*crate::src::game::bg_pmove::pm).ps).velocity[1 as i32 as usize] =
        start_v[1 as i32 as usize];
    (*(*crate::src::game::bg_pmove::pm).ps).velocity[2 as i32 as usize] =
        start_v[2 as i32 as usize];
    PM_SlideMove(gravity);
    // push down the final amount
    down[0 as i32 as usize] = (*(*crate::src::game::bg_pmove::pm).ps).origin[0 as i32 as usize];
    down[1 as i32 as usize] = (*(*crate::src::game::bg_pmove::pm).ps).origin[1 as i32 as usize];
    down[2 as i32 as usize] = (*(*crate::src::game::bg_pmove::pm).ps).origin[2 as i32 as usize];
    down[2 as i32 as usize] -= stepSize;
    (*crate::src::game::bg_pmove::pm)
        .trace
        .expect("non-null function pointer")(
        &mut trace,
        (*(*crate::src::game::bg_pmove::pm).ps).origin.as_mut_ptr()
            as *const crate::src::qcommon::q_shared::vec_t,
        (*crate::src::game::bg_pmove::pm).mins.as_mut_ptr()
            as *const crate::src::qcommon::q_shared::vec_t,
        (*crate::src::game::bg_pmove::pm).maxs.as_mut_ptr()
            as *const crate::src::qcommon::q_shared::vec_t,
        down.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        (*(*crate::src::game::bg_pmove::pm).ps).clientNum,
        (*crate::src::game::bg_pmove::pm).tracemask,
    );
    if trace.allsolid as u64 == 0 {
        (*(*crate::src::game::bg_pmove::pm).ps).origin[0 as i32 as usize] =
            trace.endpos[0 as i32 as usize];
        (*(*crate::src::game::bg_pmove::pm).ps).origin[1 as i32 as usize] =
            trace.endpos[1 as i32 as usize];
        (*(*crate::src::game::bg_pmove::pm).ps).origin[2 as i32 as usize] =
            trace.endpos[2 as i32 as usize]
    }
    if (trace.fraction as f64) < 1.0f64 {
        crate::src::game::bg_pmove::PM_ClipVelocity(
            (*(*crate::src::game::bg_pmove::pm).ps)
                .velocity
                .as_mut_ptr(),
            trace.plane.normal.as_mut_ptr(),
            (*(*crate::src::game::bg_pmove::pm).ps)
                .velocity
                .as_mut_ptr(),
            1.001f32,
        );
    }
    // use the step move
    let mut delta: f32 = 0.;
    delta = (*(*crate::src::game::bg_pmove::pm).ps).origin[2 as i32 as usize]
        - start_o[2 as i32 as usize];
    if delta > 2 as i32 as f32 {
        if delta < 7 as i32 as f32 {
            crate::src::game::bg_pmove::PM_AddEvent(crate::bg_public_h::EV_STEP_4 as i32);
        } else if delta < 11 as i32 as f32 {
            crate::src::game::bg_pmove::PM_AddEvent(crate::bg_public_h::EV_STEP_8 as i32);
        } else if delta < 15 as i32 as f32 {
            crate::src::game::bg_pmove::PM_AddEvent(crate::bg_public_h::EV_STEP_12 as i32);
        } else {
            crate::src::game::bg_pmove::PM_AddEvent(crate::bg_public_h::EV_STEP_16 as i32);
        }
    }
    if (*crate::src::game::bg_pmove::pm).debugLevel != 0 {
        crate::src::game::g_main::Com_Printf(
            b"%i:stepped\n\x00" as *const u8 as *const libc::c_char,
            crate::src::game::bg_pmove::c_pmove,
        );
    };
}

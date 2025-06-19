use ::libc;

pub mod q_shared_h {

    #[inline]

    pub unsafe extern "C" fn VectorCompare(
        mut v1: *const crate::src::qcommon::q_shared::vec_t,
        mut v2: *const crate::src::qcommon::q_shared::vec_t,
    ) -> i32 {
        if *v1.offset(0 as i32 as isize) != *v2.offset(0 as i32 as isize)
            || *v1.offset(1 as i32 as isize) != *v2.offset(1 as i32 as isize)
            || *v1.offset(2 as i32 as isize) != *v2.offset(2 as i32 as isize)
        {
            return 0 as i32;
        }
        return 1 as i32;
    }
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

    pub unsafe extern "C" fn VectorLengthSquared(
        mut v: *const crate::src::qcommon::q_shared::vec_t,
    ) -> crate::src::qcommon::q_shared::vec_t {
        return *v.offset(0 as i32 as isize) * *v.offset(0 as i32 as isize)
            + *v.offset(1 as i32 as isize) * *v.offset(1 as i32 as isize)
            + *v.offset(2 as i32 as isize) * *v.offset(2 as i32 as isize);
    }

    // __Q_SHARED_H
}

pub use crate::be_aas_h::aas_entityinfo_s;
pub use crate::be_aas_h::aas_entityinfo_t;
pub use crate::be_ai_goal_h::bot_goal_s;
pub use crate::be_ai_goal_h::bot_goal_t;
pub use crate::be_ai_move_h::bot_moveresult_s;
pub use crate::be_ai_move_h::bot_moveresult_t;
pub use crate::bg_public_h::C2RustUnnamed_0;
pub use crate::bg_public_h::GT_1FCTF;
pub use crate::bg_public_h::GT_CTF;
pub use crate::bg_public_h::GT_FFA;
pub use crate::bg_public_h::GT_HARVESTER;
pub use crate::bg_public_h::GT_MAX_GAME_TYPE;
pub use crate::bg_public_h::GT_OBELISK;
pub use crate::bg_public_h::GT_SINGLE_PLAYER;
pub use crate::bg_public_h::GT_TEAM;
pub use crate::bg_public_h::GT_TOURNAMENT;
pub use crate::bg_public_h::PERS_ASSIST_COUNT;
pub use crate::bg_public_h::PERS_ATTACKEE_ARMOR;
pub use crate::bg_public_h::PERS_ATTACKER;
pub use crate::bg_public_h::PERS_CAPTURES;
pub use crate::bg_public_h::PERS_DEFEND_COUNT;
pub use crate::bg_public_h::PERS_EXCELLENT_COUNT;
pub use crate::bg_public_h::PERS_GAUNTLET_FRAG_COUNT;
pub use crate::bg_public_h::PERS_HITS;
pub use crate::bg_public_h::PERS_IMPRESSIVE_COUNT;
pub use crate::bg_public_h::PERS_KILLED;
pub use crate::bg_public_h::PERS_PLAYEREVENTS;
pub use crate::bg_public_h::PERS_RANK;
pub use crate::bg_public_h::PERS_SCORE;
pub use crate::bg_public_h::PERS_SPAWN_COUNT;
pub use crate::bg_public_h::PERS_TEAM;
pub use crate::bg_public_h::TEAM_BLUE;
pub use crate::bg_public_h::TEAM_FREE;
pub use crate::bg_public_h::TEAM_NUM_TEAMS;
pub use crate::bg_public_h::TEAM_RED;
pub use crate::bg_public_h::TEAM_SPECTATOR;
pub use crate::botlib_h::bsp_surface_s;
pub use crate::botlib_h::bsp_surface_t;
pub use crate::botlib_h::bsp_trace_s;
pub use crate::botlib_h::bsp_trace_t;
pub use crate::g_local_h::bot_settings_s;
pub use crate::g_local_h::bot_settings_t;
pub use crate::src::game::ai_dmnet::q_shared_h::VectorCompare;
pub use crate::src::game::ai_dmnet::q_shared_h::VectorLength;
pub use crate::src::game::ai_dmnet::q_shared_h::VectorLengthSquared;
pub use crate::src::game::ai_main::bot_activategoal_s;
pub use crate::src::game::ai_main::bot_activategoal_t;
pub use crate::src::game::ai_main::bot_state_s;
pub use crate::src::game::ai_main::bot_state_t;
pub use crate::src::game::ai_main::bot_waypoint_s;
pub use crate::src::game::ai_main::bot_waypoint_t;
pub use crate::src::game::ai_main::floattime;
pub use crate::src::game::ai_main::BotAI_BotInitialChat;
pub use crate::src::game::ai_main::BotAI_GetEntityState;
pub use crate::src::game::ai_main::BotAI_Print;
pub use crate::src::game::ai_main::BotAI_Trace;
pub use crate::src::game::ai_main::BotEntityInfo;
pub use crate::src::game::ai_main::BotResetState;
pub use crate::src::game::g_syscalls::trap_AAS_AreaReachability;
pub use crate::src::game::g_syscalls::trap_AAS_AreaTravelTimeToGoalArea;
pub use crate::src::game::g_syscalls::trap_AAS_PointContents;
pub use crate::src::game::g_syscalls::trap_AAS_Swimming;
pub use crate::src::game::g_syscalls::trap_BotChooseLTGItem;
pub use crate::src::game::g_syscalls::trap_BotChooseNBGItem;
pub use crate::src::game::g_syscalls::trap_BotEmptyGoalStack;
pub use crate::src::game::g_syscalls::trap_BotEnterChat;
pub use crate::src::game::g_syscalls::trap_BotGetSecondGoal;
pub use crate::src::game::g_syscalls::trap_BotGetTopGoal;
pub use crate::src::game::g_syscalls::trap_BotGoalName;
pub use crate::src::game::g_syscalls::trap_BotItemGoalInVisButNotVisible;
pub use crate::src::game::g_syscalls::trap_BotMoveInDirection;
pub use crate::src::game::g_syscalls::trap_BotMoveToGoal;
pub use crate::src::game::g_syscalls::trap_BotMovementViewTarget;
pub use crate::src::game::g_syscalls::trap_BotPopGoal;
pub use crate::src::game::g_syscalls::trap_BotPushGoal;
pub use crate::src::game::g_syscalls::trap_BotResetAvoidGoals;
pub use crate::src::game::g_syscalls::trap_BotResetAvoidReach;
pub use crate::src::game::g_syscalls::trap_BotResetGoalState;
pub use crate::src::game::g_syscalls::trap_BotResetLastAvoidReach;
pub use crate::src::game::g_syscalls::trap_BotResetMoveState;
pub use crate::src::game::g_syscalls::trap_BotSetAvoidGoalTime;
pub use crate::src::game::g_syscalls::trap_BotTouchingGoal;
pub use crate::src::game::g_syscalls::trap_Characteristic_BFloat;
pub use crate::src::game::g_syscalls::trap_EA_Action;
pub use crate::src::game::g_syscalls::trap_EA_Attack;
pub use crate::src::game::g_syscalls::trap_EA_Crouch;
pub use crate::src::game::g_syscalls::trap_EA_Gesture;
pub use crate::src::game::g_syscalls::trap_EA_Respawn;
pub use crate::src::game::g_syscalls::trap_EA_Talk;
pub use crate::src::game::g_syscalls::trap_PointContents;
pub use crate::src::qcommon::q_math::vectoangles;
pub use crate::src::qcommon::q_math::AngleVectors;
pub use crate::src::qcommon::q_math::VectorNormalize;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::cplane_s;
pub use crate::src::qcommon::q_shared::cplane_t;
pub use crate::src::qcommon::q_shared::cvarHandle_t;
pub use crate::src::qcommon::q_shared::entityState_s;
pub use crate::src::qcommon::q_shared::entityState_t;
pub use crate::src::qcommon::q_shared::playerState_s;
pub use crate::src::qcommon::q_shared::playerState_t;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::trType_t;
pub use crate::src::qcommon::q_shared::trajectory_t;
pub use crate::src::qcommon::q_shared::usercmd_s;
pub use crate::src::qcommon::q_shared::usercmd_t;
pub use crate::src::qcommon::q_shared::vec3_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::qcommon::q_shared::vmCvar_t;
pub use crate::src::qcommon::q_shared::Com_sprintf;
pub use crate::src::qcommon::q_shared::TR_GRAVITY;
pub use crate::src::qcommon::q_shared::TR_INTERPOLATE;
pub use crate::src::qcommon::q_shared::TR_LINEAR;
pub use crate::src::qcommon::q_shared::TR_LINEAR_STOP;
pub use crate::src::qcommon::q_shared::TR_SINE;
pub use crate::src::qcommon::q_shared::TR_STATIONARY;

#[no_mangle]

pub static mut numnodeswitches: i32 = 0;
#[no_mangle]

pub static mut nodeswitch: [[libc::c_char; 144]; 51] = [[0; 144]; 51];
/*
==================
BotResetNodeSwitches
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotResetNodeSwitches() {
    numnodeswitches = 0 as i32;
}
/*
==================
BotDumpNodeSwitches
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotDumpNodeSwitches(mut bs: *mut crate::src::game::ai_main::bot_state_t) {
    let mut i: i32 = 0;
    let mut netname: [libc::c_char; 36] = [0; 36];
    crate::src::game::ai_dmq3::ClientName(
        (*bs).client,
        netname.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as i32,
    );
    crate::src::game::ai_main::BotAI_Print(
        1 as i32,
        b"%s at %1.1f switched more than %d AI nodes\n\x00" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        netname.as_mut_ptr(),
        crate::src::game::ai_main::floattime as f64,
        50 as i32,
    );
    i = 0 as i32;
    while i < numnodeswitches {
        crate::src::game::ai_main::BotAI_Print(
            1 as i32,
            b"%s\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            nodeswitch[i as usize].as_mut_ptr(),
        );
        i += 1
    }
    crate::src::game::ai_main::BotAI_Print(
        4 as i32,
        b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
/*
==================
BotRecordNodeSwitch
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotRecordNodeSwitch(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut node: *mut libc::c_char,
    mut str: *mut libc::c_char,
    mut s: *mut libc::c_char,
) {
    let mut netname: [libc::c_char; 36] = [0; 36];
    crate::src::game::ai_dmq3::ClientName(
        (*bs).client,
        netname.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as i32,
    );
    crate::src::qcommon::q_shared::Com_sprintf(
        nodeswitch[numnodeswitches as usize].as_mut_ptr(),
        144 as i32,
        b"%s at %2.1f entered %s: %s from %s\n\x00" as *const u8 as *const libc::c_char,
        netname.as_mut_ptr(),
        crate::src::game::ai_main::floattime as f64,
        node,
        str,
        s,
    );
    //DEBUG
    numnodeswitches += 1;
}
/*
==================
BotGetAirGoal
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotGetAirGoal(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut goal: *mut crate::be_ai_goal_h::bot_goal_t,
) -> i32 {
    let mut bsptrace: crate::botlib_h::bsp_trace_t = crate::botlib_h::bsp_trace_t {
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
        exp_dist: 0.,
        sidenum: 0,
        surface: crate::botlib_h::bsp_surface_t {
            name: [0; 16],
            flags: 0,
            value: 0,
        },
        contents: 0,
        ent: 0,
    };
    let mut end: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut mins: crate::src::qcommon::q_shared::vec3_t = [
        -(15 as i32) as crate::src::qcommon::q_shared::vec_t,
        -(15 as i32) as crate::src::qcommon::q_shared::vec_t,
        -(2 as i32) as crate::src::qcommon::q_shared::vec_t,
    ];
    let mut maxs: crate::src::qcommon::q_shared::vec3_t = [
        15 as i32 as crate::src::qcommon::q_shared::vec_t,
        15 as i32 as crate::src::qcommon::q_shared::vec_t,
        2 as i32 as crate::src::qcommon::q_shared::vec_t,
    ];
    let mut areanum: i32 = 0;
    //trace up until we hit solid
    end[0 as i32 as usize] = (*bs).origin[0 as i32 as usize];
    end[1 as i32 as usize] = (*bs).origin[1 as i32 as usize];
    end[2 as i32 as usize] = (*bs).origin[2 as i32 as usize];
    end[2 as i32 as usize] += 1000 as i32 as f32;
    crate::src::game::ai_main::BotAI_Trace(
        &mut bsptrace as *mut _ as *mut crate::botlib_h::bsp_trace_s,
        (*bs).origin.as_mut_ptr(),
        mins.as_mut_ptr(),
        maxs.as_mut_ptr(),
        end.as_mut_ptr(),
        (*bs).entitynum,
        1 as i32 | 0x10000 as i32,
    );
    //trace down until we hit water
    end[0 as i32 as usize] = bsptrace.endpos[0 as i32 as usize];
    end[1 as i32 as usize] = bsptrace.endpos[1 as i32 as usize];
    end[2 as i32 as usize] = bsptrace.endpos[2 as i32 as usize];
    crate::src::game::ai_main::BotAI_Trace(
        &mut bsptrace as *mut _ as *mut crate::botlib_h::bsp_trace_s,
        end.as_mut_ptr(),
        mins.as_mut_ptr(),
        maxs.as_mut_ptr(),
        (*bs).origin.as_mut_ptr(),
        (*bs).entitynum,
        32 as i32 | 16 as i32 | 8 as i32,
    );
    //if we found the water surface
    if bsptrace.fraction > 0 as i32 as f32 {
        areanum = crate::src::game::ai_dmq3::BotPointAreaNum(bsptrace.endpos.as_mut_ptr());
        if areanum != 0 {
            (*goal).origin[0 as i32 as usize] = bsptrace.endpos[0 as i32 as usize];
            (*goal).origin[1 as i32 as usize] = bsptrace.endpos[1 as i32 as usize];
            (*goal).origin[2 as i32 as usize] = bsptrace.endpos[2 as i32 as usize];
            (*goal).origin[2 as i32 as usize] -= 2 as i32 as f32;
            (*goal).areanum = areanum;
            (*goal).mins[0 as i32 as usize] = -(15 as i32) as crate::src::qcommon::q_shared::vec_t;
            (*goal).mins[1 as i32 as usize] = -(15 as i32) as crate::src::qcommon::q_shared::vec_t;
            (*goal).mins[2 as i32 as usize] = -(1 as i32) as crate::src::qcommon::q_shared::vec_t;
            (*goal).maxs[0 as i32 as usize] = 15 as i32 as crate::src::qcommon::q_shared::vec_t;
            (*goal).maxs[1 as i32 as usize] = 15 as i32 as crate::src::qcommon::q_shared::vec_t;
            (*goal).maxs[2 as i32 as usize] = 1 as i32 as crate::src::qcommon::q_shared::vec_t;
            (*goal).flags = 128 as i32;
            (*goal).number = 0 as i32;
            (*goal).iteminfo = 0 as i32;
            (*goal).entitynum = 0 as i32;
            return crate::src::qcommon::q_shared::qtrue as i32;
        }
    }
    return crate::src::qcommon::q_shared::qfalse as i32;
}
/*
==================
BotGoForAir
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotGoForAir(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut tfl: i32,
    mut ltg: *mut crate::be_ai_goal_h::bot_goal_t,
    mut range: f32,
) -> i32 {
    let mut goal: crate::be_ai_goal_h::bot_goal_t = crate::be_ai_goal_h::bot_goal_t {
        origin: [0.; 3],
        areanum: 0,
        mins: [0.; 3],
        maxs: [0.; 3],
        entitynum: 0,
        number: 0,
        flags: 0,
        iteminfo: 0,
    };
    //if the bot needs air
    if (*bs).lastair_time < crate::src::game::ai_main::floattime - 6 as i32 as f32 {
        //
        //DEBUG
        //if we can find an air goal
        if BotGetAirGoal(bs, &mut goal) != 0 {
            crate::src::game::g_syscalls::trap_BotPushGoal(
                (*bs).gs,
                &mut goal as *mut crate::be_ai_goal_h::bot_goal_t as *mut libc::c_void,
            );
            return crate::src::qcommon::q_shared::qtrue as i32;
        } else {
            //get a nearby goal outside the water
            while crate::src::game::g_syscalls::trap_BotChooseNBGItem(
                (*bs).gs,
                (*bs).origin.as_mut_ptr(),
                (*bs).inventory.as_mut_ptr(),
                tfl,
                ltg as *mut libc::c_void,
                range,
            ) != 0
            {
                crate::src::game::g_syscalls::trap_BotGetTopGoal(
                    (*bs).gs,
                    &mut goal as *mut crate::be_ai_goal_h::bot_goal_t as *mut libc::c_void,
                );
                //if the goal is not in water
                if crate::src::game::g_syscalls::trap_AAS_PointContents(goal.origin.as_mut_ptr())
                    & (32 as i32 | 16 as i32 | 8 as i32)
                    == 0
                {
                    return crate::src::qcommon::q_shared::qtrue as i32;
                }
                crate::src::game::g_syscalls::trap_BotPopGoal((*bs).gs);
            }
            crate::src::game::g_syscalls::trap_BotResetAvoidGoals((*bs).gs);
        }
    }
    return crate::src::qcommon::q_shared::qfalse as i32;
}
/*
==================
BotNearbyGoal
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotNearbyGoal(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut tfl: i32,
    mut ltg: *mut crate::be_ai_goal_h::bot_goal_t,
    mut range: f32,
) -> i32 {
    let mut ret: i32 = 0;
    //check if the bot should go for air
    if BotGoForAir(bs, tfl, ltg, range) != 0 {
        return crate::src::qcommon::q_shared::qtrue as i32;
    }
    // if the bot is carrying a flag or cubes
    if crate::src::game::ai_dmq3::BotCTFCarryingFlag(
        bs as *mut crate::src::game::ai_main::bot_state_s,
    ) != 0
    {
        //if the bot is just a few secs away from the base
        if crate::src::game::g_syscalls::trap_AAS_AreaTravelTimeToGoalArea(
            (*bs).areanum,
            (*bs).origin.as_mut_ptr(),
            (*bs).teamgoal.areanum,
            0x2 as i32
                | 0x4 as i32
                | 0x8 as i32
                | 0x10 as i32
                | 0x20 as i32
                | 0x80 as i32
                | 0x100 as i32
                | 0x200 as i32
                | 0x400 as i32
                | 0x800 as i32
                | 0x80000 as i32
                | 0x100000 as i32
                | 0x40000 as i32
                | 0x1000000 as i32,
        ) < 300 as i32
        {
            //make the range really small
            range = 50 as i32 as f32
        }
    }
    //
    ret = crate::src::game::g_syscalls::trap_BotChooseNBGItem(
        (*bs).gs,
        (*bs).origin.as_mut_ptr(),
        (*bs).inventory.as_mut_ptr(),
        tfl,
        ltg as *mut libc::c_void,
        range,
    );
    /*
    if (ret)
    {
        char buf[128];
        //get the goal at the top of the stack
        trap_BotGetTopGoal(bs->gs, &goal);
        trap_BotGoalName(goal.number, buf, sizeof(buf));
        BotAI_Print(PRT_MESSAGE, "%1.1f: new nearby goal %s\n", FloatTime(), buf);
    }
    */
    return ret;
}
/*
==================
BotReachedGoal
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotReachedGoal(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut goal: *mut crate::be_ai_goal_h::bot_goal_t,
) -> i32 {
    if (*goal).flags & 1 as i32 != 0 {
        //if touching the goal
        if crate::src::game::g_syscalls::trap_BotTouchingGoal(
            (*bs).origin.as_mut_ptr(),
            goal as *mut libc::c_void,
        ) != 0
        {
            if (*goal).flags & 4 as i32 == 0 {
                crate::src::game::g_syscalls::trap_BotSetAvoidGoalTime(
                    (*bs).gs,
                    (*goal).number,
                    -(1 as i32) as f32,
                );
            }
            return crate::src::qcommon::q_shared::qtrue as i32;
        }
        //if the goal isn't there
        if crate::src::game::g_syscalls::trap_BotItemGoalInVisButNotVisible(
            (*bs).entitynum,
            (*bs).eye.as_mut_ptr(),
            (*bs).viewangles.as_mut_ptr(),
            goal as *mut libc::c_void,
        ) != 0
        {
            /*
            float avoidtime;
            int t;

            avoidtime = trap_BotAvoidGoalTime(bs->gs, goal->number);
            if (avoidtime > 0) {
                t = trap_AAS_AreaTravelTimeToGoalArea(bs->areanum, bs->origin, goal->areanum, bs->tfl);
                if ((float) t * 0.009 < avoidtime)
                    return qtrue;
            }
            */
            return crate::src::qcommon::q_shared::qtrue as i32;
        }
        //if in the goal area and below or above the goal and not swimming
        if (*bs).areanum == (*goal).areanum {
            if (*bs).origin[0 as i32 as usize]
                > (*goal).origin[0 as i32 as usize] + (*goal).mins[0 as i32 as usize]
                && (*bs).origin[0 as i32 as usize]
                    < (*goal).origin[0 as i32 as usize] + (*goal).maxs[0 as i32 as usize]
            {
                if (*bs).origin[1 as i32 as usize]
                    > (*goal).origin[1 as i32 as usize] + (*goal).mins[1 as i32 as usize]
                    && (*bs).origin[1 as i32 as usize]
                        < (*goal).origin[1 as i32 as usize] + (*goal).maxs[1 as i32 as usize]
                {
                    if crate::src::game::g_syscalls::trap_AAS_Swimming((*bs).origin.as_mut_ptr())
                        == 0
                    {
                        return crate::src::qcommon::q_shared::qtrue as i32;
                    }
                }
            }
        }
    } else if (*goal).flags & 128 as i32 != 0 {
        //if touching the goal
        if crate::src::game::g_syscalls::trap_BotTouchingGoal(
            (*bs).origin.as_mut_ptr(),
            goal as *mut libc::c_void,
        ) != 0
        {
            return crate::src::qcommon::q_shared::qtrue as i32;
        }
        //if the bot got air
        if (*bs).lastair_time > crate::src::game::ai_main::floattime - 1 as i32 as f32 {
            return crate::src::qcommon::q_shared::qtrue as i32;
        }
    } else if crate::src::game::g_syscalls::trap_BotTouchingGoal(
        (*bs).origin.as_mut_ptr(),
        goal as *mut libc::c_void,
    ) != 0
    {
        return crate::src::qcommon::q_shared::qtrue as i32;
    }
    return crate::src::qcommon::q_shared::qfalse as i32;
}
//if touching the goal
/*
==================
BotGetItemLongTermGoal
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotGetItemLongTermGoal(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut tfl: i32,
    mut goal: *mut crate::be_ai_goal_h::bot_goal_t,
) -> i32 {
    //if the bot has no goal
    if crate::src::game::g_syscalls::trap_BotGetTopGoal((*bs).gs, goal as *mut libc::c_void) == 0 {
        //BotAI_Print(PRT_MESSAGE, "no ltg on stack\n");
        (*bs).ltg_time = 0 as i32 as f32
    } else if BotReachedGoal(bs, goal) != 0 {
        crate::src::game::ai_dmq3::BotChooseWeapon(
            bs as *mut crate::src::game::ai_main::bot_state_s,
        );
        (*bs).ltg_time = 0 as i32 as f32
    }
    //if the bot touches the current goal
    //if it is time to find a new long term goal
    if (*bs).ltg_time < crate::src::game::ai_main::floattime {
        //pop the current goal from the stack
        crate::src::game::g_syscalls::trap_BotPopGoal((*bs).gs);
        //BotAI_Print(PRT_MESSAGE, "%s: choosing new ltg\n", ClientName(bs->client, netname, sizeof(netname)));
        //choose a new goal
        //BotAI_Print(PRT_MESSAGE, "%6.1f client %d: BotChooseLTGItem\n", FloatTime(), bs->client);
        if crate::src::game::g_syscalls::trap_BotChooseLTGItem(
            (*bs).gs,
            (*bs).origin.as_mut_ptr(),
            (*bs).inventory.as_mut_ptr(),
            tfl,
        ) != 0
        {
            /*
            char buf[128];
            //get the goal at the top of the stack
            trap_BotGetTopGoal(bs->gs, goal);
            trap_BotGoalName(goal->number, buf, sizeof(buf));
            BotAI_Print(PRT_MESSAGE, "%1.1f: new long term goal %s\n", FloatTime(), buf);
            */
            (*bs).ltg_time = crate::src::game::ai_main::floattime + 20 as i32 as f32
        } else {
            //the bot gets sorta stuck with all the avoid timings, shouldn't happen though
            //
            //trap_BotDumpAvoidGoals(bs->gs);
            //reset the avoid goals and the avoid reach
            crate::src::game::g_syscalls::trap_BotResetAvoidGoals((*bs).gs);
            crate::src::game::g_syscalls::trap_BotResetAvoidReach((*bs).ms);
        }
        //get the goal at the top of the stack
        return crate::src::game::g_syscalls::trap_BotGetTopGoal(
            (*bs).gs,
            goal as *mut libc::c_void,
        );
    }
    return crate::src::qcommon::q_shared::qtrue as i32;
}
/*
==================
BotGetLongTermGoal

we could also create a separate AI node for every long term goal type
however this saves us a lot of code
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotGetLongTermGoal(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut tfl: i32,
    mut retreat: i32,
    mut goal: *mut crate::be_ai_goal_h::bot_goal_t,
) -> i32 {
    let mut target: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut dir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut dir2: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut netname: [libc::c_char; 36] = [0; 36];
    let mut buf: [libc::c_char; 256] = [0; 256];
    let mut areanum: i32 = 0;
    let mut croucher: f32 = 0.;
    let mut entinfo: crate::be_aas_h::aas_entityinfo_t = crate::be_aas_h::aas_entityinfo_t {
        valid: 0,
        type_0: 0,
        flags: 0,
        ltime: 0.,
        update_time: 0.,
        number: 0,
        origin: [0.; 3],
        angles: [0.; 3],
        old_origin: [0.; 3],
        lastvisorigin: [0.; 3],
        mins: [0.; 3],
        maxs: [0.; 3],
        groundent: 0,
        solid: 0,
        modelindex: 0,
        modelindex2: 0,
        frame: 0,
        event: 0,
        eventParm: 0,
        powerups: 0,
        weapon: 0,
        legsAnim: 0,
        torsoAnim: 0,
    };
    let mut botinfo: crate::be_aas_h::aas_entityinfo_t = crate::be_aas_h::aas_entityinfo_t {
        valid: 0,
        type_0: 0,
        flags: 0,
        ltime: 0.,
        update_time: 0.,
        number: 0,
        origin: [0.; 3],
        angles: [0.; 3],
        old_origin: [0.; 3],
        lastvisorigin: [0.; 3],
        mins: [0.; 3],
        maxs: [0.; 3],
        groundent: 0,
        solid: 0,
        modelindex: 0,
        modelindex2: 0,
        frame: 0,
        event: 0,
        eventParm: 0,
        powerups: 0,
        weapon: 0,
        legsAnim: 0,
        torsoAnim: 0,
    };
    let mut wp: *mut crate::src::game::ai_main::bot_waypoint_t =
        0 as *mut crate::src::game::ai_main::bot_waypoint_t;
    if (*bs).ltgtype == 1 as i32 && retreat == 0 {
        //check for bot typing status message
        if (*bs).teammessage_time != 0.
            && (*bs).teammessage_time < crate::src::game::ai_main::floattime
        {
            crate::src::game::ai_main::BotAI_BotInitialChat(
                bs as *mut crate::src::game::ai_main::bot_state_s,
                b"help_start\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                crate::src::game::ai_dmq3::EasyClientName(
                    (*bs).teammate,
                    netname.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as i32,
                ),
                0 as *mut libc::c_void,
            );
            crate::src::game::g_syscalls::trap_BotEnterChat(
                (*bs).cs,
                (*bs).decisionmaker,
                2 as i32,
            );
            crate::src::game::ai_team::BotVoiceChatOnly(
                bs as *mut crate::src::game::ai_main::bot_state_s,
                (*bs).decisionmaker,
                b"yes\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            crate::src::game::g_syscalls::trap_EA_Action((*bs).client, 0x100000 as i32);
            (*bs).teammessage_time = 0 as i32 as f32
        }
        //if trying to help the team mate for more than a minute
        if (*bs).teamgoal_time < crate::src::game::ai_main::floattime {
            (*bs).ltgtype = 0 as i32
        }
        //if the team mate IS visible for quite some time
        if (*bs).teammatevisible_time < crate::src::game::ai_main::floattime - 10 as i32 as f32 {
            (*bs).ltgtype = 0 as i32
        }
        //get entity information of the companion
        crate::src::game::ai_main::BotEntityInfo(
            (*bs).teammate,
            &mut entinfo as *mut _ as *mut crate::be_aas_h::aas_entityinfo_s,
        );
        //if the team mate is visible
        if crate::src::game::ai_dmq3::BotEntityVisible(
            (*bs).entitynum,
            (*bs).eye.as_mut_ptr(),
            (*bs).viewangles.as_mut_ptr(),
            360 as i32 as f32,
            (*bs).teammate,
        ) != 0.
        {
            //if close just stand still there
            dir[0 as i32 as usize] =
                entinfo.origin[0 as i32 as usize] - (*bs).origin[0 as i32 as usize];
            dir[1 as i32 as usize] =
                entinfo.origin[1 as i32 as usize] - (*bs).origin[1 as i32 as usize];
            dir[2 as i32 as usize] =
                entinfo.origin[2 as i32 as usize] - (*bs).origin[2 as i32 as usize];
            if VectorLengthSquared(dir.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t)
                < (100 as i32 * 100 as i32) as f32
            {
                crate::src::game::g_syscalls::trap_BotResetAvoidReach((*bs).ms);
                return crate::src::qcommon::q_shared::qfalse as i32;
            }
        } else {
            //last time the bot was NOT visible
            (*bs).teammatevisible_time = crate::src::game::ai_main::floattime
        }
        //if the entity information is valid (entity in PVS)
        if entinfo.valid != 0 {
            areanum = crate::src::game::ai_dmq3::BotPointAreaNum(entinfo.origin.as_mut_ptr());
            if areanum != 0 && crate::src::game::g_syscalls::trap_AAS_AreaReachability(areanum) != 0
            {
                //update team goal
                (*bs).teamgoal.entitynum = (*bs).teammate;
                (*bs).teamgoal.areanum = areanum;
                (*bs).teamgoal.origin[0 as i32 as usize] = entinfo.origin[0 as i32 as usize];
                (*bs).teamgoal.origin[1 as i32 as usize] = entinfo.origin[1 as i32 as usize];
                (*bs).teamgoal.origin[2 as i32 as usize] = entinfo.origin[2 as i32 as usize];
                (*bs).teamgoal.mins[0 as i32 as usize] =
                    -(8 as i32) as crate::src::qcommon::q_shared::vec_t;
                (*bs).teamgoal.mins[1 as i32 as usize] =
                    -(8 as i32) as crate::src::qcommon::q_shared::vec_t;
                (*bs).teamgoal.mins[2 as i32 as usize] =
                    -(8 as i32) as crate::src::qcommon::q_shared::vec_t;
                (*bs).teamgoal.maxs[0 as i32 as usize] =
                    8 as i32 as crate::src::qcommon::q_shared::vec_t;
                (*bs).teamgoal.maxs[1 as i32 as usize] =
                    8 as i32 as crate::src::qcommon::q_shared::vec_t;
                (*bs).teamgoal.maxs[2 as i32 as usize] =
                    8 as i32 as crate::src::qcommon::q_shared::vec_t
            }
        }
        crate::stdlib::memcpy(
            goal as *mut libc::c_void,
            &mut (*bs).teamgoal as *mut crate::be_ai_goal_h::bot_goal_t as *const libc::c_void,
            ::std::mem::size_of::<crate::be_ai_goal_h::bot_goal_t>() as libc::c_ulong,
        );
        return crate::src::qcommon::q_shared::qtrue as i32;
    }
    //if the bot accompanies someone
    if (*bs).ltgtype == 2 as i32 && retreat == 0 {
        //check for bot typing status message
        if (*bs).teammessage_time != 0.
            && (*bs).teammessage_time < crate::src::game::ai_main::floattime
        {
            crate::src::game::ai_main::BotAI_BotInitialChat(
                bs as *mut crate::src::game::ai_main::bot_state_s,
                b"accompany_start\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                crate::src::game::ai_dmq3::EasyClientName(
                    (*bs).teammate,
                    netname.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as i32,
                ),
                0 as *mut libc::c_void,
            );
            crate::src::game::g_syscalls::trap_BotEnterChat(
                (*bs).cs,
                (*bs).decisionmaker,
                2 as i32,
            );
            crate::src::game::ai_team::BotVoiceChatOnly(
                bs as *mut crate::src::game::ai_main::bot_state_s,
                (*bs).decisionmaker,
                b"yes\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            crate::src::game::g_syscalls::trap_EA_Action((*bs).client, 0x100000 as i32);
            (*bs).teammessage_time = 0 as i32 as f32
        }
        //if accompanying the companion for 3 minutes
        if (*bs).teamgoal_time < crate::src::game::ai_main::floattime {
            crate::src::game::ai_main::BotAI_BotInitialChat(
                bs as *mut crate::src::game::ai_main::bot_state_s,
                b"accompany_stop\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                crate::src::game::ai_dmq3::EasyClientName(
                    (*bs).teammate,
                    netname.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as i32,
                ),
                0 as *mut libc::c_void,
            );
            crate::src::game::g_syscalls::trap_BotEnterChat((*bs).cs, (*bs).teammate, 2 as i32);
            (*bs).ltgtype = 0 as i32
        }
        //get entity information of the companion
        crate::src::game::ai_main::BotEntityInfo(
            (*bs).teammate,
            &mut entinfo as *mut _ as *mut crate::be_aas_h::aas_entityinfo_s,
        );
        //if the companion is visible
        if crate::src::game::ai_dmq3::BotEntityVisible(
            (*bs).entitynum,
            (*bs).eye.as_mut_ptr(),
            (*bs).viewangles.as_mut_ptr(),
            360 as i32 as f32,
            (*bs).teammate,
        ) != 0.
        {
            //update visible time
            (*bs).teammatevisible_time = crate::src::game::ai_main::floattime;
            dir[0 as i32 as usize] =
                entinfo.origin[0 as i32 as usize] - (*bs).origin[0 as i32 as usize];
            dir[1 as i32 as usize] =
                entinfo.origin[1 as i32 as usize] - (*bs).origin[1 as i32 as usize];
            dir[2 as i32 as usize] =
                entinfo.origin[2 as i32 as usize] - (*bs).origin[2 as i32 as usize];
            if VectorLengthSquared(dir.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t)
                < (*bs).formation_dist * (*bs).formation_dist
            {
                //
                // if the client being followed bumps into this bot then
                // the bot should back up
                crate::src::game::ai_main::BotEntityInfo(
                    (*bs).entitynum,
                    &mut botinfo as *mut _ as *mut crate::be_aas_h::aas_entityinfo_s,
                );
                // if the followed client is not standing ontop of the bot
                if botinfo.origin[2 as i32 as usize] + botinfo.maxs[2 as i32 as usize]
                    > entinfo.origin[2 as i32 as usize] + entinfo.mins[2 as i32 as usize]
                {
                    // if the bounding boxes touch each other
                    if botinfo.origin[0 as i32 as usize] + botinfo.maxs[0 as i32 as usize]
                        > entinfo.origin[0 as i32 as usize] + entinfo.mins[0 as i32 as usize]
                            - 4 as i32 as f32
                        && botinfo.origin[0 as i32 as usize] + botinfo.mins[0 as i32 as usize]
                            < entinfo.origin[0 as i32 as usize]
                                + entinfo.maxs[0 as i32 as usize]
                                + 4 as i32 as f32
                    {
                        if botinfo.origin[1 as i32 as usize] + botinfo.maxs[1 as i32 as usize]
                            > entinfo.origin[1 as i32 as usize] + entinfo.mins[1 as i32 as usize]
                                - 4 as i32 as f32
                            && botinfo.origin[1 as i32 as usize] + botinfo.mins[1 as i32 as usize]
                                < entinfo.origin[1 as i32 as usize]
                                    + entinfo.maxs[1 as i32 as usize]
                                    + 4 as i32 as f32
                        {
                            if botinfo.origin[2 as i32 as usize] + botinfo.maxs[2 as i32 as usize]
                                > entinfo.origin[2 as i32 as usize]
                                    + entinfo.mins[2 as i32 as usize]
                                    - 4 as i32 as f32
                                && botinfo.origin[2 as i32 as usize]
                                    + botinfo.mins[2 as i32 as usize]
                                    < entinfo.origin[2 as i32 as usize]
                                        + entinfo.maxs[2 as i32 as usize]
                                        + 4 as i32 as f32
                            {
                                // if the followed client looks in the direction of this bot
                                crate::src::qcommon::q_math::AngleVectors(
                                    entinfo.angles.as_mut_ptr()
                                        as *const crate::src::qcommon::q_shared::vec_t,
                                    dir.as_mut_ptr(),
                                    0 as *mut crate::src::qcommon::q_shared::vec_t,
                                    0 as *mut crate::src::qcommon::q_shared::vec_t,
                                );
                                dir[2 as i32 as usize] =
                                    0 as i32 as crate::src::qcommon::q_shared::vec_t;
                                crate::src::qcommon::q_math::VectorNormalize(dir.as_mut_ptr());
                                //VectorSubtract(entinfo.origin, entinfo.lastvisorigin, dir);
                                dir2[0 as i32 as usize] = (*bs).origin[0 as i32 as usize]
                                    - entinfo.origin[0 as i32 as usize];
                                dir2[1 as i32 as usize] = (*bs).origin[1 as i32 as usize]
                                    - entinfo.origin[1 as i32 as usize];
                                dir2[2 as i32 as usize] = (*bs).origin[2 as i32 as usize]
                                    - entinfo.origin[2 as i32 as usize];
                                crate::src::qcommon::q_math::VectorNormalize(dir2.as_mut_ptr());
                                if (dir[0 as i32 as usize] * dir2[0 as i32 as usize]
                                    + dir[1 as i32 as usize] * dir2[1 as i32 as usize]
                                    + dir[2 as i32 as usize] * dir2[2 as i32 as usize])
                                    as f64
                                    > 0.7f64
                                {
                                    // back up
                                    crate::src::game::ai_dmq3::BotSetupForMovement(
                                        bs as *mut crate::src::game::ai_main::bot_state_s,
                                    );
                                    crate::src::game::g_syscalls::trap_BotMoveInDirection(
                                        (*bs).ms,
                                        dir2.as_mut_ptr(),
                                        400 as i32 as f32,
                                        1 as i32,
                                    );
                                }
                            }
                        }
                    }
                }
                //check if the bot wants to crouch
                //don't crouch if crouched less than 5 seconds ago
                if (*bs).attackcrouch_time < crate::src::game::ai_main::floattime - 5 as i32 as f32
                {
                    croucher = crate::src::game::g_syscalls::trap_Characteristic_BFloat(
                        (*bs).character,
                        36 as i32,
                        0 as i32 as f32,
                        1 as i32 as f32,
                    );
                    if ((::libc::rand() & 0x7fff as i32) as f32 / 0x7fff as i32 as f32)
                        < (*bs).thinktime * croucher
                    {
                        (*bs).attackcrouch_time = crate::src::game::ai_main::floattime
                            + 5 as i32 as f32
                            + croucher * 15 as i32 as f32
                    }
                }
                //don't crouch when swimming
                if crate::src::game::g_syscalls::trap_AAS_Swimming((*bs).origin.as_mut_ptr()) != 0 {
                    (*bs).attackcrouch_time = crate::src::game::ai_main::floattime - 1 as i32 as f32
                }
                //if not arrived yet or arived some time ago
                if (*bs).arrive_time < crate::src::game::ai_main::floattime - 2 as i32 as f32 {
                    //if not arrived yet
                    if (*bs).arrive_time == 0. {
                        crate::src::game::g_syscalls::trap_EA_Gesture((*bs).client);
                        crate::src::game::ai_main::BotAI_BotInitialChat(
                            bs as *mut crate::src::game::ai_main::bot_state_s,
                            b"accompany_arrive\x00" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                            crate::src::game::ai_dmq3::EasyClientName(
                                (*bs).teammate,
                                netname.as_mut_ptr(),
                                ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as i32,
                            ),
                            0 as *mut libc::c_void,
                        );
                        crate::src::game::g_syscalls::trap_BotEnterChat(
                            (*bs).cs,
                            (*bs).teammate,
                            2 as i32,
                        );
                        (*bs).arrive_time = crate::src::game::ai_main::floattime
                    } else if (*bs).attackcrouch_time > crate::src::game::ai_main::floattime {
                        crate::src::game::g_syscalls::trap_EA_Crouch((*bs).client);
                    } else if (((::libc::rand() & 0x7fff as i32) as f32 / 0x7fff as i32 as f32)
                        as f64)
                        < (*bs).thinktime as f64 * 0.05f64
                    {
                        //if the bot wants to crouch
                        //else do some model taunts
                        //do a gesture :)
                        crate::src::game::g_syscalls::trap_EA_Gesture((*bs).client);
                    }
                }
                //if just arrived look at the companion
                if (*bs).arrive_time > crate::src::game::ai_main::floattime - 2 as i32 as f32 {
                    dir[0 as i32 as usize] =
                        entinfo.origin[0 as i32 as usize] - (*bs).origin[0 as i32 as usize];
                    dir[1 as i32 as usize] =
                        entinfo.origin[1 as i32 as usize] - (*bs).origin[1 as i32 as usize];
                    dir[2 as i32 as usize] =
                        entinfo.origin[2 as i32 as usize] - (*bs).origin[2 as i32 as usize];
                    crate::src::qcommon::q_math::vectoangles(
                        dir.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
                        (*bs).ideal_viewangles.as_mut_ptr(),
                    );
                    (*bs).ideal_viewangles[2 as i32 as usize] =
                        ((*bs).ideal_viewangles[2 as i32 as usize] as f64 * 0.5f64)
                            as crate::src::qcommon::q_shared::vec_t
                } else if (((::libc::rand() & 0x7fff as i32) as f32 / 0x7fff as i32 as f32) as f64)
                    < (*bs).thinktime as f64 * 0.8f64
                {
                    crate::src::game::ai_dmq3::BotRoamGoal(
                        bs as *mut crate::src::game::ai_main::bot_state_s,
                        target.as_mut_ptr(),
                    );
                    dir[0 as i32 as usize] =
                        target[0 as i32 as usize] - (*bs).origin[0 as i32 as usize];
                    dir[1 as i32 as usize] =
                        target[1 as i32 as usize] - (*bs).origin[1 as i32 as usize];
                    dir[2 as i32 as usize] =
                        target[2 as i32 as usize] - (*bs).origin[2 as i32 as usize];
                    crate::src::qcommon::q_math::vectoangles(
                        dir.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
                        (*bs).ideal_viewangles.as_mut_ptr(),
                    );
                    (*bs).ideal_viewangles[2 as i32 as usize] =
                        ((*bs).ideal_viewangles[2 as i32 as usize] as f64 * 0.5f64)
                            as crate::src::qcommon::q_shared::vec_t
                }
                //else look strategically around for enemies
                //check if the bot wants to go for air
                if BotGoForAir(bs, (*bs).tfl, &mut (*bs).teamgoal, 400 as i32 as f32) != 0 {
                    crate::src::game::g_syscalls::trap_BotResetLastAvoidReach((*bs).ms);
                    //get the goal at the top of the stack
                    //trap_BotGetTopGoal(bs->gs, &tmpgoal);
                    //trap_BotGoalName(tmpgoal.number, buf, 144);
                    //BotAI_Print(PRT_MESSAGE, "new nearby goal %s\n", buf);
                    //time the bot gets to pick up the nearby goal item
                    (*bs).nbg_time = crate::src::game::ai_main::floattime + 8 as i32 as f32;
                    AIEnter_Seek_NBG(
                        bs,
                        b"BotLongTermGoal: go for air\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    );
                    return crate::src::qcommon::q_shared::qfalse as i32;
                }
                //
                crate::src::game::g_syscalls::trap_BotResetAvoidReach((*bs).ms);
                return crate::src::qcommon::q_shared::qfalse as i32;
            }
        }
        //if the entity information is valid (entity in PVS)
        if entinfo.valid != 0 {
            areanum = crate::src::game::ai_dmq3::BotPointAreaNum(entinfo.origin.as_mut_ptr());
            if areanum != 0 && crate::src::game::g_syscalls::trap_AAS_AreaReachability(areanum) != 0
            {
                //update team goal
                (*bs).teamgoal.entitynum = (*bs).teammate;
                (*bs).teamgoal.areanum = areanum;
                (*bs).teamgoal.origin[0 as i32 as usize] = entinfo.origin[0 as i32 as usize];
                (*bs).teamgoal.origin[1 as i32 as usize] = entinfo.origin[1 as i32 as usize];
                (*bs).teamgoal.origin[2 as i32 as usize] = entinfo.origin[2 as i32 as usize];
                (*bs).teamgoal.mins[0 as i32 as usize] =
                    -(8 as i32) as crate::src::qcommon::q_shared::vec_t;
                (*bs).teamgoal.mins[1 as i32 as usize] =
                    -(8 as i32) as crate::src::qcommon::q_shared::vec_t;
                (*bs).teamgoal.mins[2 as i32 as usize] =
                    -(8 as i32) as crate::src::qcommon::q_shared::vec_t;
                (*bs).teamgoal.maxs[0 as i32 as usize] =
                    8 as i32 as crate::src::qcommon::q_shared::vec_t;
                (*bs).teamgoal.maxs[1 as i32 as usize] =
                    8 as i32 as crate::src::qcommon::q_shared::vec_t;
                (*bs).teamgoal.maxs[2 as i32 as usize] =
                    8 as i32 as crate::src::qcommon::q_shared::vec_t
            }
        }
        //the goal the bot should go for
        crate::stdlib::memcpy(
            goal as *mut libc::c_void,
            &mut (*bs).teamgoal as *mut crate::be_ai_goal_h::bot_goal_t as *const libc::c_void,
            ::std::mem::size_of::<crate::be_ai_goal_h::bot_goal_t>() as libc::c_ulong,
        );
        //if the companion is NOT visible for too long
        if (*bs).teammatevisible_time < crate::src::game::ai_main::floattime - 60 as i32 as f32 {
            crate::src::game::ai_main::BotAI_BotInitialChat(
                bs as *mut crate::src::game::ai_main::bot_state_s,
                b"accompany_cannotfind\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                crate::src::game::ai_dmq3::EasyClientName(
                    (*bs).teammate,
                    netname.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as i32,
                ),
                0 as *mut libc::c_void,
            );
            crate::src::game::g_syscalls::trap_BotEnterChat((*bs).cs, (*bs).teammate, 2 as i32);
            (*bs).ltgtype = 0 as i32;
            // just to make sure the bot won't spam this message
            (*bs).teammatevisible_time = crate::src::game::ai_main::floattime
        }
        return crate::src::qcommon::q_shared::qtrue as i32;
    }
    //
    if (*bs).ltgtype == 3 as i32 {
        if crate::src::game::g_syscalls::trap_AAS_AreaTravelTimeToGoalArea(
            (*bs).areanum,
            (*bs).origin.as_mut_ptr(),
            (*bs).teamgoal.areanum,
            0x2 as i32
                | 0x4 as i32
                | 0x8 as i32
                | 0x10 as i32
                | 0x20 as i32
                | 0x80 as i32
                | 0x100 as i32
                | 0x200 as i32
                | 0x400 as i32
                | 0x800 as i32
                | 0x80000 as i32
                | 0x100000 as i32
                | 0x40000 as i32
                | 0x1000000 as i32,
        ) as f32
            > (*bs).defendaway_range
        {
            (*bs).defendaway_time = 0 as i32 as f32
        }
    }
    //if defending a key area
    if (*bs).ltgtype == 3 as i32
        && retreat == 0
        && (*bs).defendaway_time < crate::src::game::ai_main::floattime
    {
        //check for bot typing status message
        if (*bs).teammessage_time != 0.
            && (*bs).teammessage_time < crate::src::game::ai_main::floattime
        {
            crate::src::game::g_syscalls::trap_BotGoalName(
                (*bs).teamgoal.number,
                buf.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as i32,
            );
            crate::src::game::ai_main::BotAI_BotInitialChat(
                bs as *mut crate::src::game::ai_main::bot_state_s,
                b"defend_start\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                buf.as_mut_ptr(),
                0 as *mut libc::c_void,
            );
            crate::src::game::g_syscalls::trap_BotEnterChat((*bs).cs, 0 as i32, 1 as i32);
            crate::src::game::ai_team::BotVoiceChatOnly(
                bs as *mut crate::src::game::ai_main::bot_state_s,
                -(1 as i32),
                b"ondefense\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            (*bs).teammessage_time = 0 as i32 as f32
        }
        //set the bot goal
        crate::stdlib::memcpy(
            goal as *mut libc::c_void,
            &mut (*bs).teamgoal as *mut crate::be_ai_goal_h::bot_goal_t as *const libc::c_void,
            ::std::mem::size_of::<crate::be_ai_goal_h::bot_goal_t>() as libc::c_ulong,
        );
        //stop after 2 minutes
        if (*bs).teamgoal_time < crate::src::game::ai_main::floattime {
            crate::src::game::g_syscalls::trap_BotGoalName(
                (*bs).teamgoal.number,
                buf.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as i32,
            );
            crate::src::game::ai_main::BotAI_BotInitialChat(
                bs as *mut crate::src::game::ai_main::bot_state_s,
                b"defend_stop\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                buf.as_mut_ptr(),
                0 as *mut libc::c_void,
            );
            crate::src::game::g_syscalls::trap_BotEnterChat((*bs).cs, 0 as i32, 1 as i32);
            (*bs).ltgtype = 0 as i32
        }
        //if very close... go away for some time
        dir[0 as i32 as usize] =
            (*goal).origin[0 as i32 as usize] - (*bs).origin[0 as i32 as usize];
        dir[1 as i32 as usize] =
            (*goal).origin[1 as i32 as usize] - (*bs).origin[1 as i32 as usize];
        dir[2 as i32 as usize] =
            (*goal).origin[2 as i32 as usize] - (*bs).origin[2 as i32 as usize];
        if VectorLengthSquared(dir.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t)
            < (70 as i32 * 70 as i32) as f32
        {
            crate::src::game::g_syscalls::trap_BotResetAvoidReach((*bs).ms);
            (*bs).defendaway_time = crate::src::game::ai_main::floattime
                + 3 as i32 as f32
                + 3 as i32 as f32
                    * ((::libc::rand() & 0x7fff as i32) as f32 / 0x7fff as i32 as f32);
            if crate::src::game::ai_dmq3::BotHasPersistantPowerupAndWeapon(
                bs as *mut crate::src::game::ai_main::bot_state_s,
            ) != 0
            {
                (*bs).defendaway_range = 100 as i32 as f32
            } else {
                (*bs).defendaway_range = 350 as i32 as f32
            }
        }
        return crate::src::qcommon::q_shared::qtrue as i32;
    }
    //going to kill someone
    if (*bs).ltgtype == 11 as i32 && retreat == 0 {
        //check for bot typing status message
        if (*bs).teammessage_time != 0.
            && (*bs).teammessage_time < crate::src::game::ai_main::floattime
        {
            crate::src::game::ai_dmq3::EasyClientName(
                (*bs).teamgoal.entitynum,
                buf.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as i32,
            );
            crate::src::game::ai_main::BotAI_BotInitialChat(
                bs as *mut crate::src::game::ai_main::bot_state_s,
                b"kill_start\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                buf.as_mut_ptr(),
                0 as *mut libc::c_void,
            );
            crate::src::game::g_syscalls::trap_BotEnterChat(
                (*bs).cs,
                (*bs).decisionmaker,
                2 as i32,
            );
            (*bs).teammessage_time = 0 as i32 as f32
        }
        //
        if (*bs).killedenemy_time > (*bs).teamgoal_time - 180 as i32 as f32
            && (*bs).lastkilledplayer == (*bs).teamgoal.entitynum
        {
            crate::src::game::ai_dmq3::EasyClientName(
                (*bs).teamgoal.entitynum,
                buf.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as i32,
            );
            crate::src::game::ai_main::BotAI_BotInitialChat(
                bs as *mut crate::src::game::ai_main::bot_state_s,
                b"kill_done\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                buf.as_mut_ptr(),
                0 as *mut libc::c_void,
            );
            crate::src::game::g_syscalls::trap_BotEnterChat(
                (*bs).cs,
                (*bs).decisionmaker,
                2 as i32,
            );
            (*bs).ltgtype = 0 as i32
        }
        //
        if (*bs).teamgoal_time < crate::src::game::ai_main::floattime {
            (*bs).ltgtype = 0 as i32
        }
        //just roam around
        return BotGetItemLongTermGoal(bs, tfl, goal);
    }
    //get an item
    if (*bs).ltgtype == 10 as i32 && retreat == 0 {
        //check for bot typing status message
        if (*bs).teammessage_time != 0.
            && (*bs).teammessage_time < crate::src::game::ai_main::floattime
        {
            crate::src::game::g_syscalls::trap_BotGoalName(
                (*bs).teamgoal.number,
                buf.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as i32,
            );
            crate::src::game::ai_main::BotAI_BotInitialChat(
                bs as *mut crate::src::game::ai_main::bot_state_s,
                b"getitem_start\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                buf.as_mut_ptr(),
                0 as *mut libc::c_void,
            );
            crate::src::game::g_syscalls::trap_BotEnterChat(
                (*bs).cs,
                (*bs).decisionmaker,
                2 as i32,
            );
            crate::src::game::ai_team::BotVoiceChatOnly(
                bs as *mut crate::src::game::ai_main::bot_state_s,
                (*bs).decisionmaker,
                b"yes\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            crate::src::game::g_syscalls::trap_EA_Action((*bs).client, 0x100000 as i32);
            (*bs).teammessage_time = 0 as i32 as f32
        }
        //set the bot goal
        crate::stdlib::memcpy(
            goal as *mut libc::c_void,
            &mut (*bs).teamgoal as *mut crate::be_ai_goal_h::bot_goal_t as *const libc::c_void,
            ::std::mem::size_of::<crate::be_ai_goal_h::bot_goal_t>() as libc::c_ulong,
        );
        //stop after some time
        if (*bs).teamgoal_time < crate::src::game::ai_main::floattime {
            (*bs).ltgtype = 0 as i32
        }
        //
        if crate::src::game::g_syscalls::trap_BotItemGoalInVisButNotVisible(
            (*bs).entitynum,
            (*bs).eye.as_mut_ptr(),
            (*bs).viewangles.as_mut_ptr(),
            goal as *mut libc::c_void,
        ) != 0
        {
            crate::src::game::g_syscalls::trap_BotGoalName(
                (*bs).teamgoal.number,
                buf.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as i32,
            );
            crate::src::game::ai_main::BotAI_BotInitialChat(
                bs as *mut crate::src::game::ai_main::bot_state_s,
                b"getitem_notthere\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                buf.as_mut_ptr(),
                0 as *mut libc::c_void,
            );
            crate::src::game::g_syscalls::trap_BotEnterChat(
                (*bs).cs,
                (*bs).decisionmaker,
                2 as i32,
            );
            (*bs).ltgtype = 0 as i32
        } else if BotReachedGoal(bs, goal) != 0 {
            crate::src::game::g_syscalls::trap_BotGoalName(
                (*bs).teamgoal.number,
                buf.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as i32,
            );
            crate::src::game::ai_main::BotAI_BotInitialChat(
                bs as *mut crate::src::game::ai_main::bot_state_s,
                b"getitem_gotit\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                buf.as_mut_ptr(),
                0 as *mut libc::c_void,
            );
            crate::src::game::g_syscalls::trap_BotEnterChat(
                (*bs).cs,
                (*bs).decisionmaker,
                2 as i32,
            );
            (*bs).ltgtype = 0 as i32
        }
        return crate::src::qcommon::q_shared::qtrue as i32;
    }
    //if camping somewhere
    if ((*bs).ltgtype == 7 as i32 || (*bs).ltgtype == 8 as i32) && retreat == 0 {
        //check for bot typing status message
        if (*bs).teammessage_time != 0.
            && (*bs).teammessage_time < crate::src::game::ai_main::floattime
        {
            if (*bs).ltgtype == 8 as i32 {
                crate::src::game::ai_main::BotAI_BotInitialChat(
                    bs as *mut crate::src::game::ai_main::bot_state_s,
                    b"camp_start\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    crate::src::game::ai_dmq3::EasyClientName(
                        (*bs).teammate,
                        netname.as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as i32,
                    ),
                    0 as *mut libc::c_void,
                );
                crate::src::game::g_syscalls::trap_BotEnterChat(
                    (*bs).cs,
                    (*bs).decisionmaker,
                    2 as i32,
                );
                crate::src::game::ai_team::BotVoiceChatOnly(
                    bs as *mut crate::src::game::ai_main::bot_state_s,
                    (*bs).decisionmaker,
                    b"yes\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
                crate::src::game::g_syscalls::trap_EA_Action((*bs).client, 0x100000 as i32);
            }
            (*bs).teammessage_time = 0 as i32 as f32
        }
        //set the bot goal
        crate::stdlib::memcpy(
            goal as *mut libc::c_void,
            &mut (*bs).teamgoal as *mut crate::be_ai_goal_h::bot_goal_t as *const libc::c_void,
            ::std::mem::size_of::<crate::be_ai_goal_h::bot_goal_t>() as libc::c_ulong,
        );
        //
        if (*bs).teamgoal_time < crate::src::game::ai_main::floattime {
            if (*bs).ltgtype == 8 as i32 {
                crate::src::game::ai_main::BotAI_BotInitialChat(
                    bs as *mut crate::src::game::ai_main::bot_state_s,
                    b"camp_stop\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    0 as *mut libc::c_void,
                );
                crate::src::game::g_syscalls::trap_BotEnterChat(
                    (*bs).cs,
                    (*bs).decisionmaker,
                    2 as i32,
                );
            }
            (*bs).ltgtype = 0 as i32
        }
        //if really near the camp spot
        dir[0 as i32 as usize] =
            (*goal).origin[0 as i32 as usize] - (*bs).origin[0 as i32 as usize];
        dir[1 as i32 as usize] =
            (*goal).origin[1 as i32 as usize] - (*bs).origin[1 as i32 as usize];
        dir[2 as i32 as usize] =
            (*goal).origin[2 as i32 as usize] - (*bs).origin[2 as i32 as usize];
        if VectorLengthSquared(dir.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t)
            < (60 as i32 * 60 as i32) as f32
        {
            //if not arrived yet
            if (*bs).arrive_time == 0. {
                if (*bs).ltgtype == 8 as i32 {
                    crate::src::game::ai_main::BotAI_BotInitialChat(
                        bs as *mut crate::src::game::ai_main::bot_state_s,
                        b"camp_arrive\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        crate::src::game::ai_dmq3::EasyClientName(
                            (*bs).teammate,
                            netname.as_mut_ptr(),
                            ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as i32,
                        ),
                        0 as *mut libc::c_void,
                    );
                    crate::src::game::g_syscalls::trap_BotEnterChat(
                        (*bs).cs,
                        (*bs).decisionmaker,
                        2 as i32,
                    );
                    crate::src::game::ai_team::BotVoiceChatOnly(
                        bs as *mut crate::src::game::ai_main::bot_state_s,
                        (*bs).decisionmaker,
                        b"inposition\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    );
                }
                (*bs).arrive_time = crate::src::game::ai_main::floattime
            }
            //look strategically around for enemies
            if (((::libc::rand() & 0x7fff as i32) as f32 / 0x7fff as i32 as f32) as f64)
                < (*bs).thinktime as f64 * 0.8f64
            {
                crate::src::game::ai_dmq3::BotRoamGoal(
                    bs as *mut crate::src::game::ai_main::bot_state_s,
                    target.as_mut_ptr(),
                );
                dir[0 as i32 as usize] =
                    target[0 as i32 as usize] - (*bs).origin[0 as i32 as usize];
                dir[1 as i32 as usize] =
                    target[1 as i32 as usize] - (*bs).origin[1 as i32 as usize];
                dir[2 as i32 as usize] =
                    target[2 as i32 as usize] - (*bs).origin[2 as i32 as usize];
                crate::src::qcommon::q_math::vectoangles(
                    dir.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
                    (*bs).ideal_viewangles.as_mut_ptr(),
                );
                (*bs).ideal_viewangles[2 as i32 as usize] =
                    ((*bs).ideal_viewangles[2 as i32 as usize] as f64 * 0.5f64)
                        as crate::src::qcommon::q_shared::vec_t
            }
            //check if the bot wants to crouch
            //don't crouch if crouched less than 5 seconds ago
            if (*bs).attackcrouch_time < crate::src::game::ai_main::floattime - 5 as i32 as f32 {
                croucher = crate::src::game::g_syscalls::trap_Characteristic_BFloat(
                    (*bs).character,
                    36 as i32,
                    0 as i32 as f32,
                    1 as i32 as f32,
                );
                if ((::libc::rand() & 0x7fff as i32) as f32 / 0x7fff as i32 as f32)
                    < (*bs).thinktime * croucher
                {
                    (*bs).attackcrouch_time = crate::src::game::ai_main::floattime
                        + 5 as i32 as f32
                        + croucher * 15 as i32 as f32
                }
            }
            //if the bot wants to crouch
            if (*bs).attackcrouch_time > crate::src::game::ai_main::floattime {
                crate::src::game::g_syscalls::trap_EA_Crouch((*bs).client);
            }
            //don't crouch when swimming
            if crate::src::game::g_syscalls::trap_AAS_Swimming((*bs).origin.as_mut_ptr()) != 0 {
                (*bs).attackcrouch_time = crate::src::game::ai_main::floattime - 1 as i32 as f32
            }
            //make sure the bot is not gonna drown
            if crate::src::game::g_syscalls::trap_PointContents(
                (*bs).eye.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
                (*bs).entitynum,
            ) & (32 as i32 | 16 as i32 | 8 as i32)
                != 0
            {
                if (*bs).ltgtype == 8 as i32 {
                    crate::src::game::ai_main::BotAI_BotInitialChat(
                        bs as *mut crate::src::game::ai_main::bot_state_s,
                        b"camp_stop\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        0 as *mut libc::c_void,
                    );
                    crate::src::game::g_syscalls::trap_BotEnterChat(
                        (*bs).cs,
                        (*bs).decisionmaker,
                        2 as i32,
                    );
                    //
                    if (*bs).lastgoal_ltgtype == 8 as i32 {
                        (*bs).lastgoal_ltgtype = 0 as i32
                    }
                }
                (*bs).ltgtype = 0 as i32
            }
            //
            //FIXME: move around a bit
            //
            crate::src::game::g_syscalls::trap_BotResetAvoidReach((*bs).ms);
            return crate::src::qcommon::q_shared::qfalse as i32;
        }
        return crate::src::qcommon::q_shared::qtrue as i32;
    }
    //patrolling along several waypoints
    if (*bs).ltgtype == 9 as i32 && retreat == 0 {
        //check for bot typing status message
        if (*bs).teammessage_time != 0.
            && (*bs).teammessage_time < crate::src::game::ai_main::floattime
        {
            ::libc::strcpy(
                buf.as_mut_ptr(),
                b"\x00" as *const u8 as *const libc::c_char,
            );
            wp = (*bs).patrolpoints;
            while !wp.is_null() {
                ::libc::strcat(buf.as_mut_ptr(), (*wp).name.as_mut_ptr());
                if !(*wp).next.is_null() {
                    ::libc::strcat(
                        buf.as_mut_ptr(),
                        b" to \x00" as *const u8 as *const libc::c_char,
                    );
                }
                wp = (*wp).next
            }
            crate::src::game::ai_main::BotAI_BotInitialChat(
                bs as *mut crate::src::game::ai_main::bot_state_s,
                b"patrol_start\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                buf.as_mut_ptr(),
                0 as *mut libc::c_void,
            );
            crate::src::game::g_syscalls::trap_BotEnterChat(
                (*bs).cs,
                (*bs).decisionmaker,
                2 as i32,
            );
            crate::src::game::ai_team::BotVoiceChatOnly(
                bs as *mut crate::src::game::ai_main::bot_state_s,
                (*bs).decisionmaker,
                b"yes\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            crate::src::game::g_syscalls::trap_EA_Action((*bs).client, 0x100000 as i32);
            (*bs).teammessage_time = 0 as i32 as f32
        }
        //
        if (*bs).curpatrolpoint.is_null() {
            (*bs).ltgtype = 0 as i32;
            return crate::src::qcommon::q_shared::qfalse as i32;
        }
        //if the bot touches the current goal
        if crate::src::game::g_syscalls::trap_BotTouchingGoal(
            (*bs).origin.as_mut_ptr(),
            &mut (*(*bs).curpatrolpoint).goal as *mut crate::be_ai_goal_h::bot_goal_t
                as *mut libc::c_void,
        ) != 0
        {
            if (*bs).patrolflags & 4 as i32 != 0 {
                if !(*(*bs).curpatrolpoint).prev.is_null() {
                    (*bs).curpatrolpoint = (*(*bs).curpatrolpoint).prev
                } else {
                    (*bs).curpatrolpoint = (*(*bs).curpatrolpoint).next;
                    (*bs).patrolflags &= !(4 as i32)
                }
            } else if !(*(*bs).curpatrolpoint).next.is_null() {
                (*bs).curpatrolpoint = (*(*bs).curpatrolpoint).next
            } else {
                (*bs).curpatrolpoint = (*(*bs).curpatrolpoint).prev;
                (*bs).patrolflags |= 4 as i32
            }
        }
        //stop after 5 minutes
        if (*bs).teamgoal_time < crate::src::game::ai_main::floattime {
            crate::src::game::ai_main::BotAI_BotInitialChat(
                bs as *mut crate::src::game::ai_main::bot_state_s,
                b"patrol_stop\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                0 as *mut libc::c_void,
            );
            crate::src::game::g_syscalls::trap_BotEnterChat(
                (*bs).cs,
                (*bs).decisionmaker,
                2 as i32,
            );
            (*bs).ltgtype = 0 as i32
        }
        if (*bs).curpatrolpoint.is_null() {
            (*bs).ltgtype = 0 as i32;
            return crate::src::qcommon::q_shared::qfalse as i32;
        }
        crate::stdlib::memcpy(
            goal as *mut libc::c_void,
            &mut (*(*bs).curpatrolpoint).goal as *mut crate::be_ai_goal_h::bot_goal_t
                as *const libc::c_void,
            ::std::mem::size_of::<crate::be_ai_goal_h::bot_goal_t>() as libc::c_ulong,
        );
        return crate::src::qcommon::q_shared::qtrue as i32;
    }
    if crate::src::game::ai_dmq3::gametype == crate::bg_public_h::GT_CTF as i32 {
        //if going for enemy flag
        if (*bs).ltgtype == 4 as i32 {
            //check for bot typing status message
            if (*bs).teammessage_time != 0.
                && (*bs).teammessage_time < crate::src::game::ai_main::floattime
            {
                crate::src::game::ai_main::BotAI_BotInitialChat(
                    bs as *mut crate::src::game::ai_main::bot_state_s,
                    b"captureflag_start\x00" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    0 as *mut libc::c_void,
                );
                crate::src::game::g_syscalls::trap_BotEnterChat((*bs).cs, 0 as i32, 1 as i32);
                crate::src::game::ai_team::BotVoiceChatOnly(
                    bs as *mut crate::src::game::ai_main::bot_state_s,
                    -(1 as i32),
                    b"ongetflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
                (*bs).teammessage_time = 0 as i32 as f32
            }
            //
            match crate::src::game::ai_dmq3::BotTeam(
                bs as *mut crate::src::game::ai_main::bot_state_s,
            ) {
                1 => {
                    crate::stdlib::memcpy(
                        goal as *mut libc::c_void,
                        &mut crate::src::game::ai_dmq3::ctf_blueflag
                            as *mut crate::be_ai_goal_h::bot_goal_t
                            as *const libc::c_void,
                        ::std::mem::size_of::<crate::be_ai_goal_h::bot_goal_t>() as libc::c_ulong,
                    );
                }
                2 => {
                    crate::stdlib::memcpy(
                        goal as *mut libc::c_void,
                        &mut crate::src::game::ai_dmq3::ctf_redflag
                            as *mut crate::be_ai_goal_h::bot_goal_t
                            as *const libc::c_void,
                        ::std::mem::size_of::<crate::be_ai_goal_h::bot_goal_t>() as libc::c_ulong,
                    );
                }
                _ => {
                    (*bs).ltgtype = 0 as i32;
                    return crate::src::qcommon::q_shared::qfalse as i32;
                }
            }
            //if touching the flag
            if crate::src::game::g_syscalls::trap_BotTouchingGoal(
                (*bs).origin.as_mut_ptr(),
                goal as *mut libc::c_void,
            ) != 0
            {
                // make sure the bot knows the flag isn't there anymore
                match crate::src::game::ai_dmq3::BotTeam(
                    bs as *mut crate::src::game::ai_main::bot_state_s,
                ) {
                    1 => (*bs).blueflagstatus = 1 as i32,
                    2 => (*bs).redflagstatus = 1 as i32,
                    _ => {}
                }
                (*bs).ltgtype = 0 as i32
            }
            //stop after 3 minutes
            if (*bs).teamgoal_time < crate::src::game::ai_main::floattime {
                (*bs).ltgtype = 0 as i32
            }

            crate::src::game::ai_dmq3::BotAlternateRoute(
                bs as *mut crate::src::game::ai_main::bot_state_s,
                goal as *mut crate::be_ai_goal_h::bot_goal_s,
            ) as *mut crate::be_ai_goal_h::bot_goal_s;
            return crate::src::qcommon::q_shared::qtrue as i32;
        }
        //if rushing to the base
        if (*bs).ltgtype == 5 as i32
            && (*bs).rushbaseaway_time < crate::src::game::ai_main::floattime
        {
            match crate::src::game::ai_dmq3::BotTeam(
                bs as *mut crate::src::game::ai_main::bot_state_s,
            ) {
                1 => {
                    crate::stdlib::memcpy(
                        goal as *mut libc::c_void,
                        &mut crate::src::game::ai_dmq3::ctf_redflag
                            as *mut crate::be_ai_goal_h::bot_goal_t
                            as *const libc::c_void,
                        ::std::mem::size_of::<crate::be_ai_goal_h::bot_goal_t>() as libc::c_ulong,
                    );
                }
                2 => {
                    crate::stdlib::memcpy(
                        goal as *mut libc::c_void,
                        &mut crate::src::game::ai_dmq3::ctf_blueflag
                            as *mut crate::be_ai_goal_h::bot_goal_t
                            as *const libc::c_void,
                        ::std::mem::size_of::<crate::be_ai_goal_h::bot_goal_t>() as libc::c_ulong,
                    );
                }
                _ => {
                    (*bs).ltgtype = 0 as i32;
                    return crate::src::qcommon::q_shared::qfalse as i32;
                }
            }
            //if not carrying the flag anymore
            if crate::src::game::ai_dmq3::BotCTFCarryingFlag(
                bs as *mut crate::src::game::ai_main::bot_state_s,
            ) == 0
            {
                (*bs).ltgtype = 0 as i32
            }
            //quit rushing after 2 minutes
            if (*bs).teamgoal_time < crate::src::game::ai_main::floattime {
                (*bs).ltgtype = 0 as i32
            }
            //if touching the base flag the bot should loose the enemy flag
            if crate::src::game::g_syscalls::trap_BotTouchingGoal(
                (*bs).origin.as_mut_ptr(),
                goal as *mut libc::c_void,
            ) != 0
            {
                //if the bot is still carrying the enemy flag then the
                //base flag is gone, now just walk near the base a bit
                if crate::src::game::ai_dmq3::BotCTFCarryingFlag(
                    bs as *mut crate::src::game::ai_main::bot_state_s,
                ) != 0
                {
                    crate::src::game::g_syscalls::trap_BotResetAvoidReach((*bs).ms);
                    (*bs).rushbaseaway_time = crate::src::game::ai_main::floattime
                        + 5 as i32 as f32
                        + 10 as i32 as f32
                            * ((::libc::rand() & 0x7fff as i32) as f32 / 0x7fff as i32 as f32)
                //FIXME: add chat to tell the others to get back the flag
                } else {
                    (*bs).ltgtype = 0 as i32
                }
            }

            crate::src::game::ai_dmq3::BotAlternateRoute(
                bs as *mut crate::src::game::ai_main::bot_state_s,
                goal as *mut crate::be_ai_goal_h::bot_goal_s,
            ) as *mut crate::be_ai_goal_h::bot_goal_s;
            return crate::src::qcommon::q_shared::qtrue as i32;
        }
        //returning flag
        if (*bs).ltgtype == 6 as i32 {
            //check for bot typing status message
            if (*bs).teammessage_time != 0.
                && (*bs).teammessage_time < crate::src::game::ai_main::floattime
            {
                crate::src::game::ai_main::BotAI_BotInitialChat(
                    bs as *mut crate::src::game::ai_main::bot_state_s,
                    b"returnflag_start\x00" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    0 as *mut libc::c_void,
                );
                crate::src::game::g_syscalls::trap_BotEnterChat((*bs).cs, 0 as i32, 1 as i32);
                crate::src::game::ai_team::BotVoiceChatOnly(
                    bs as *mut crate::src::game::ai_main::bot_state_s,
                    -(1 as i32),
                    b"onreturnflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
                (*bs).teammessage_time = 0 as i32 as f32
            }
            //
            match crate::src::game::ai_dmq3::BotTeam(
                bs as *mut crate::src::game::ai_main::bot_state_s,
            ) {
                1 => {
                    crate::stdlib::memcpy(
                        goal as *mut libc::c_void,
                        &mut crate::src::game::ai_dmq3::ctf_blueflag
                            as *mut crate::be_ai_goal_h::bot_goal_t
                            as *const libc::c_void,
                        ::std::mem::size_of::<crate::be_ai_goal_h::bot_goal_t>() as libc::c_ulong,
                    );
                }
                2 => {
                    crate::stdlib::memcpy(
                        goal as *mut libc::c_void,
                        &mut crate::src::game::ai_dmq3::ctf_redflag
                            as *mut crate::be_ai_goal_h::bot_goal_t
                            as *const libc::c_void,
                        ::std::mem::size_of::<crate::be_ai_goal_h::bot_goal_t>() as libc::c_ulong,
                    );
                }
                _ => {
                    (*bs).ltgtype = 0 as i32;
                    return crate::src::qcommon::q_shared::qfalse as i32;
                }
            }
            //if touching the flag
            if crate::src::game::g_syscalls::trap_BotTouchingGoal(
                (*bs).origin.as_mut_ptr(),
                goal as *mut libc::c_void,
            ) != 0
            {
                (*bs).ltgtype = 0 as i32
            }
            //stop after 3 minutes
            if (*bs).teamgoal_time < crate::src::game::ai_main::floattime {
                (*bs).ltgtype = 0 as i32
            }

            crate::src::game::ai_dmq3::BotAlternateRoute(
                bs as *mut crate::src::game::ai_main::bot_state_s,
                goal as *mut crate::be_ai_goal_h::bot_goal_s,
            ) as *mut crate::be_ai_goal_h::bot_goal_s;
            return crate::src::qcommon::q_shared::qtrue as i32;
        }
    }
    //CTF
    //normal goal stuff
    return BotGetItemLongTermGoal(bs, tfl, goal);
}
/*
==================
BotLongTermGoal
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotLongTermGoal(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut tfl: i32,
    mut retreat: i32,
    mut goal: *mut crate::be_ai_goal_h::bot_goal_t,
) -> i32 {
    let mut entinfo: crate::be_aas_h::aas_entityinfo_t = crate::be_aas_h::aas_entityinfo_t {
        valid: 0,
        type_0: 0,
        flags: 0,
        ltime: 0.,
        update_time: 0.,
        number: 0,
        origin: [0.; 3],
        angles: [0.; 3],
        old_origin: [0.; 3],
        lastvisorigin: [0.; 3],
        mins: [0.; 3],
        maxs: [0.; 3],
        groundent: 0,
        solid: 0,
        modelindex: 0,
        modelindex2: 0,
        frame: 0,
        event: 0,
        eventParm: 0,
        powerups: 0,
        weapon: 0,
        legsAnim: 0,
        torsoAnim: 0,
    };
    let mut teammate: [libc::c_char; 256] = [0; 256];
    let mut squaredist: f32 = 0.;
    let mut areanum: i32 = 0;
    let mut dir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    //FIXME: also have air long term goals?
    //
    //if the bot is leading someone and not retreating
    if (*bs).lead_time > 0 as i32 as f32 && retreat == 0 {
        if (*bs).lead_time < crate::src::game::ai_main::floattime {
            crate::src::game::ai_main::BotAI_BotInitialChat(
                bs as *mut crate::src::game::ai_main::bot_state_s,
                b"lead_stop\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                crate::src::game::ai_dmq3::EasyClientName(
                    (*bs).lead_teammate,
                    teammate.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as i32,
                ),
                0 as *mut libc::c_void,
            );
            crate::src::game::g_syscalls::trap_BotEnterChat((*bs).cs, (*bs).teammate, 2 as i32);
            (*bs).lead_time = 0 as i32 as f32;
            return BotGetLongTermGoal(bs, tfl, retreat, goal);
        }
        //
        if (*bs).leadmessage_time < 0 as i32 as f32
            && -(*bs).leadmessage_time < crate::src::game::ai_main::floattime
        {
            crate::src::game::ai_main::BotAI_BotInitialChat(
                bs as *mut crate::src::game::ai_main::bot_state_s,
                b"followme\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                crate::src::game::ai_dmq3::EasyClientName(
                    (*bs).lead_teammate,
                    teammate.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as i32,
                ),
                0 as *mut libc::c_void,
            );
            crate::src::game::g_syscalls::trap_BotEnterChat((*bs).cs, (*bs).teammate, 2 as i32);
            (*bs).leadmessage_time = crate::src::game::ai_main::floattime
        }
        //get entity information of the companion
        crate::src::game::ai_main::BotEntityInfo(
            (*bs).lead_teammate,
            &mut entinfo as *mut _ as *mut crate::be_aas_h::aas_entityinfo_s,
        );
        //
        if entinfo.valid != 0 {
            areanum = crate::src::game::ai_dmq3::BotPointAreaNum(entinfo.origin.as_mut_ptr());
            if areanum != 0 && crate::src::game::g_syscalls::trap_AAS_AreaReachability(areanum) != 0
            {
                //update team goal
                (*bs).lead_teamgoal.entitynum = (*bs).lead_teammate;
                (*bs).lead_teamgoal.areanum = areanum;
                (*bs).lead_teamgoal.origin[0 as i32 as usize] = entinfo.origin[0 as i32 as usize];
                (*bs).lead_teamgoal.origin[1 as i32 as usize] = entinfo.origin[1 as i32 as usize];
                (*bs).lead_teamgoal.origin[2 as i32 as usize] = entinfo.origin[2 as i32 as usize];
                (*bs).lead_teamgoal.mins[0 as i32 as usize] =
                    -(8 as i32) as crate::src::qcommon::q_shared::vec_t;
                (*bs).lead_teamgoal.mins[1 as i32 as usize] =
                    -(8 as i32) as crate::src::qcommon::q_shared::vec_t;
                (*bs).lead_teamgoal.mins[2 as i32 as usize] =
                    -(8 as i32) as crate::src::qcommon::q_shared::vec_t;
                (*bs).lead_teamgoal.maxs[0 as i32 as usize] =
                    8 as i32 as crate::src::qcommon::q_shared::vec_t;
                (*bs).lead_teamgoal.maxs[1 as i32 as usize] =
                    8 as i32 as crate::src::qcommon::q_shared::vec_t;
                (*bs).lead_teamgoal.maxs[2 as i32 as usize] =
                    8 as i32 as crate::src::qcommon::q_shared::vec_t
            }
        }
        //if the team mate is visible
        if crate::src::game::ai_dmq3::BotEntityVisible(
            (*bs).entitynum,
            (*bs).eye.as_mut_ptr(),
            (*bs).viewangles.as_mut_ptr(),
            360 as i32 as f32,
            (*bs).lead_teammate,
        ) != 0.
        {
            (*bs).leadvisible_time = crate::src::game::ai_main::floattime
        }
        //if the team mate is not visible for 1 seconds
        if (*bs).leadvisible_time < crate::src::game::ai_main::floattime - 1 as i32 as f32 {
            (*bs).leadbackup_time = crate::src::game::ai_main::floattime + 2 as i32 as f32
        }
        //distance towards the team mate
        dir[0 as i32 as usize] =
            (*bs).origin[0 as i32 as usize] - (*bs).lead_teamgoal.origin[0 as i32 as usize];
        dir[1 as i32 as usize] =
            (*bs).origin[1 as i32 as usize] - (*bs).lead_teamgoal.origin[1 as i32 as usize];
        dir[2 as i32 as usize] =
            (*bs).origin[2 as i32 as usize] - (*bs).lead_teamgoal.origin[2 as i32 as usize];
        squaredist =
            VectorLengthSquared(dir.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t);
        //if backing up towards the team mate
        if (*bs).leadbackup_time > crate::src::game::ai_main::floattime {
            if (*bs).leadmessage_time < crate::src::game::ai_main::floattime - 20 as i32 as f32 {
                crate::src::game::ai_main::BotAI_BotInitialChat(
                    bs as *mut crate::src::game::ai_main::bot_state_s,
                    b"followme\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    crate::src::game::ai_dmq3::EasyClientName(
                        (*bs).lead_teammate,
                        teammate.as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as i32,
                    ),
                    0 as *mut libc::c_void,
                );
                crate::src::game::g_syscalls::trap_BotEnterChat((*bs).cs, (*bs).teammate, 2 as i32);
                (*bs).leadmessage_time = crate::src::game::ai_main::floattime
            }
            //if very close to the team mate
            if squaredist < (100 as i32 * 100 as i32) as f32 {
                (*bs).leadbackup_time = 0 as i32 as f32
            }
            //the bot should go back to the team mate
            crate::stdlib::memcpy(
                goal as *mut libc::c_void,
                &mut (*bs).lead_teamgoal as *mut crate::be_ai_goal_h::bot_goal_t
                    as *const libc::c_void,
                ::std::mem::size_of::<crate::be_ai_goal_h::bot_goal_t>() as libc::c_ulong,
            );
            return crate::src::qcommon::q_shared::qtrue as i32;
        } else {
            //if quite distant from the team mate
            if squaredist > (500 as i32 * 500 as i32) as f32 {
                if (*bs).leadmessage_time < crate::src::game::ai_main::floattime - 20 as i32 as f32
                {
                    crate::src::game::ai_main::BotAI_BotInitialChat(
                        bs as *mut crate::src::game::ai_main::bot_state_s,
                        b"followme\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        crate::src::game::ai_dmq3::EasyClientName(
                            (*bs).lead_teammate,
                            teammate.as_mut_ptr(),
                            ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as i32,
                        ),
                        0 as *mut libc::c_void,
                    );
                    crate::src::game::g_syscalls::trap_BotEnterChat(
                        (*bs).cs,
                        (*bs).teammate,
                        2 as i32,
                    );
                    (*bs).leadmessage_time = crate::src::game::ai_main::floattime
                }
                //look at the team mate
                dir[0 as i32 as usize] =
                    entinfo.origin[0 as i32 as usize] - (*bs).origin[0 as i32 as usize];
                dir[1 as i32 as usize] =
                    entinfo.origin[1 as i32 as usize] - (*bs).origin[1 as i32 as usize];
                dir[2 as i32 as usize] =
                    entinfo.origin[2 as i32 as usize] - (*bs).origin[2 as i32 as usize];
                crate::src::qcommon::q_math::vectoangles(
                    dir.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
                    (*bs).ideal_viewangles.as_mut_ptr(),
                );
                (*bs).ideal_viewangles[2 as i32 as usize] =
                    ((*bs).ideal_viewangles[2 as i32 as usize] as f64 * 0.5f64)
                        as crate::src::qcommon::q_shared::vec_t;
                //just wait for the team mate
                return crate::src::qcommon::q_shared::qfalse as i32;
            }
        }
    }
    return BotGetLongTermGoal(bs, tfl, retreat, goal);
}
/*
==================
AIEnter_Intermission
==================
*/
#[no_mangle]

pub unsafe extern "C" fn AIEnter_Intermission(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut s: *mut libc::c_char,
) {
    BotRecordNodeSwitch(
        bs,
        b"intermission\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        s,
    );
    //reset the bot state
    crate::src::game::ai_main::BotResetState(bs as *mut crate::src::game::ai_main::bot_state_s);
    //check for end level chat
    if crate::src::game::ai_chat::BotChat_EndLevel(
        bs as *mut crate::src::game::ai_main::bot_state_s,
    ) != 0
    {
        crate::src::game::g_syscalls::trap_BotEnterChat((*bs).cs, 0 as i32, (*bs).chatto);
    }
    (*bs).ainode = Some(
        AINode_Intermission
            as unsafe extern "C" fn(_: *mut crate::src::game::ai_main::bot_state_t) -> i32,
    );
}
/*
==================
AINode_Intermission
==================
*/
#[no_mangle]

pub unsafe extern "C" fn AINode_Intermission(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
) -> i32 {
    //if the intermission ended
    if crate::src::game::ai_dmq3::BotIntermission(bs as *mut crate::src::game::ai_main::bot_state_s)
        as u64
        == 0
    {
        if crate::src::game::ai_chat::BotChat_StartLevel(
            bs as *mut crate::src::game::ai_main::bot_state_s,
        ) != 0
        {
            (*bs).stand_time = crate::src::game::ai_main::floattime
                + crate::src::game::ai_chat::BotChatTime(
                    bs as *mut crate::src::game::ai_main::bot_state_s,
                )
        } else {
            (*bs).stand_time = crate::src::game::ai_main::floattime + 2 as i32 as f32
        }
        AIEnter_Stand(
            bs,
            b"intermission: chat\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    return crate::src::qcommon::q_shared::qtrue as i32;
}
/*
==================
AIEnter_Observer
==================
*/
#[no_mangle]

pub unsafe extern "C" fn AIEnter_Observer(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut s: *mut libc::c_char,
) {
    BotRecordNodeSwitch(
        bs,
        b"observer\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        s,
    );
    //reset the bot state
    crate::src::game::ai_main::BotResetState(bs as *mut crate::src::game::ai_main::bot_state_s);
    (*bs).ainode = Some(
        AINode_Observer
            as unsafe extern "C" fn(_: *mut crate::src::game::ai_main::bot_state_t) -> i32,
    );
}
/*
==================
AINode_Observer
==================
*/
#[no_mangle]

pub unsafe extern "C" fn AINode_Observer(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
) -> i32 {
    //if the bot left observer mode
    if crate::src::game::ai_dmq3::BotIsObserver(bs as *mut crate::src::game::ai_main::bot_state_s)
        as u64
        == 0
    {
        AIEnter_Stand(
            bs,
            b"observer: left observer\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    return crate::src::qcommon::q_shared::qtrue as i32;
}
/*
==================
AIEnter_Stand
==================
*/
#[no_mangle]

pub unsafe extern "C" fn AIEnter_Stand(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut s: *mut libc::c_char,
) {
    BotRecordNodeSwitch(
        bs,
        b"stand\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        s,
    );
    (*bs).standfindenemy_time = crate::src::game::ai_main::floattime + 1 as i32 as f32;
    (*bs).ainode = Some(
        AINode_Stand as unsafe extern "C" fn(_: *mut crate::src::game::ai_main::bot_state_t) -> i32,
    );
}
/*
==================
AINode_Stand
==================
*/
#[no_mangle]

pub unsafe extern "C" fn AINode_Stand(mut bs: *mut crate::src::game::ai_main::bot_state_t) -> i32 {
    //if the bot's health decreased
    if (*bs).lastframe_health > (*bs).inventory[29 as i32 as usize] {
        if crate::src::game::ai_chat::BotChat_HitTalking(
            bs as *mut crate::src::game::ai_main::bot_state_s,
        ) != 0
        {
            (*bs).standfindenemy_time = ((crate::src::game::ai_main::floattime
                + crate::src::game::ai_chat::BotChatTime(
                    bs as *mut crate::src::game::ai_main::bot_state_s,
                )) as f64
                + 0.1f64) as f32;
            (*bs).stand_time = ((crate::src::game::ai_main::floattime
                + crate::src::game::ai_chat::BotChatTime(
                    bs as *mut crate::src::game::ai_main::bot_state_s,
                )) as f64
                + 0.1f64) as f32
        }
    }
    if (*bs).standfindenemy_time < crate::src::game::ai_main::floattime {
        if crate::src::game::ai_dmq3::BotFindEnemy(
            bs as *mut crate::src::game::ai_main::bot_state_s,
            -(1 as i32),
        ) != 0
        {
            AIEnter_Battle_Fight(
                bs,
                b"stand: found enemy\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            return crate::src::qcommon::q_shared::qfalse as i32;
        }
        (*bs).standfindenemy_time = crate::src::game::ai_main::floattime + 1 as i32 as f32
    }
    // put up chat icon
    crate::src::game::g_syscalls::trap_EA_Talk((*bs).client);
    // when done standing
    if (*bs).stand_time < crate::src::game::ai_main::floattime {
        crate::src::game::g_syscalls::trap_BotEnterChat((*bs).cs, 0 as i32, (*bs).chatto);
        AIEnter_Seek_LTG(
            bs,
            b"stand: time out\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    //
    return crate::src::qcommon::q_shared::qtrue as i32;
}
/*
==================
AIEnter_Respawn
==================
*/
#[no_mangle]

pub unsafe extern "C" fn AIEnter_Respawn(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut s: *mut libc::c_char,
) {
    BotRecordNodeSwitch(
        bs,
        b"respawn\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        s,
    );
    //reset some states
    crate::src::game::g_syscalls::trap_BotResetMoveState((*bs).ms);
    crate::src::game::g_syscalls::trap_BotResetGoalState((*bs).gs);
    crate::src::game::g_syscalls::trap_BotResetAvoidGoals((*bs).gs);
    crate::src::game::g_syscalls::trap_BotResetAvoidReach((*bs).ms);
    //if the bot wants to chat
    if crate::src::game::ai_chat::BotChat_Death(bs as *mut crate::src::game::ai_main::bot_state_s)
        != 0
    {
        (*bs).respawn_time = crate::src::game::ai_main::floattime
            + crate::src::game::ai_chat::BotChatTime(
                bs as *mut crate::src::game::ai_main::bot_state_s,
            );
        (*bs).respawnchat_time = crate::src::game::ai_main::floattime
    } else {
        (*bs).respawn_time = crate::src::game::ai_main::floattime
            + 1 as i32 as f32
            + (::libc::rand() & 0x7fff as i32) as f32 / 0x7fff as i32 as f32;
        (*bs).respawnchat_time = 0 as i32 as f32
    }
    //set respawn state
    (*bs).respawn_wait = crate::src::qcommon::q_shared::qfalse as i32;
    (*bs).ainode = Some(
        AINode_Respawn
            as unsafe extern "C" fn(_: *mut crate::src::game::ai_main::bot_state_t) -> i32,
    );
}
/*
==================
AINode_Respawn
==================
*/
#[no_mangle]

pub unsafe extern "C" fn AINode_Respawn(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
) -> i32 {
    // if waiting for the actual respawn
    if (*bs).respawn_wait != 0 {
        if crate::src::game::ai_dmq3::BotIsDead(bs as *mut crate::src::game::ai_main::bot_state_s)
            as u64
            == 0
        {
            AIEnter_Seek_LTG(
                bs,
                b"respawn: respawned\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        } else {
            crate::src::game::g_syscalls::trap_EA_Respawn((*bs).client);
        }
    } else if (*bs).respawn_time < crate::src::game::ai_main::floattime {
        // wait until respawned
        (*bs).respawn_wait = crate::src::qcommon::q_shared::qtrue as i32;
        // elementary action respawn
        crate::src::game::g_syscalls::trap_EA_Respawn((*bs).client);
        //
        if (*bs).respawnchat_time != 0. {
            crate::src::game::g_syscalls::trap_BotEnterChat((*bs).cs, 0 as i32, (*bs).chatto);
            (*bs).enemy = -(1 as i32)
        }
    }
    if (*bs).respawnchat_time != 0.
        && ((*bs).respawnchat_time as f64) < crate::src::game::ai_main::floattime as f64 - 0.5f64
    {
        crate::src::game::g_syscalls::trap_EA_Talk((*bs).client);
    }
    //
    return crate::src::qcommon::q_shared::qtrue as i32;
}
/*
==================
BotSelectActivateWeapon
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotSelectActivateWeapon(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
) -> i32 {
    //
    if (*bs).inventory[6 as i32 as usize] > 0 as i32
        && (*bs).inventory[19 as i32 as usize] > 0 as i32
    {
        return 2 as i32;
    } else if (*bs).inventory[5 as i32 as usize] > 0 as i32
        && (*bs).inventory[18 as i32 as usize] > 0 as i32
    {
        return 3 as i32;
    } else if (*bs).inventory[11 as i32 as usize] > 0 as i32
        && (*bs).inventory[21 as i32 as usize] > 0 as i32
    {
        return 8 as i32;
    } else if (*bs).inventory[9 as i32 as usize] > 0 as i32
        && (*bs).inventory[22 as i32 as usize] > 0 as i32
    {
        return 6 as i32;
    } else if (*bs).inventory[7 as i32 as usize] > 0 as i32
        && (*bs).inventory[20 as i32 as usize] > 0 as i32
    {
        return 4 as i32;
    } else if (*bs).inventory[10 as i32 as usize] > 0 as i32
        && (*bs).inventory[24 as i32 as usize] > 0 as i32
    {
        return 7 as i32;
    } else if (*bs).inventory[8 as i32 as usize] > 0 as i32
        && (*bs).inventory[23 as i32 as usize] > 0 as i32
    {
        return 5 as i32;
    } else if (*bs).inventory[13 as i32 as usize] > 0 as i32
        && (*bs).inventory[25 as i32 as usize] > 0 as i32
    {
        return 9 as i32;
    } else {
        return -(1 as i32);
    };
}
/*
==================
BotClearPath

 try to deactivate obstacles like proximity mines on the bot's path
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotClearPath(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut moveresult: *mut crate::be_ai_move_h::bot_moveresult_t,
) {
    let mut i: i32 = 0;
    let mut bestmine: i32 = 0;
    let mut dist: f32 = 0.;
    let mut bestdist: f32 = 0.;
    let mut target: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut dir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut bsptrace: crate::botlib_h::bsp_trace_t = crate::botlib_h::bsp_trace_t {
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
        exp_dist: 0.,
        sidenum: 0,
        surface: crate::botlib_h::bsp_surface_t {
            name: [0; 16],
            flags: 0,
            value: 0,
        },
        contents: 0,
        ent: 0,
    };
    let mut state: crate::src::qcommon::q_shared::entityState_t =
        crate::src::qcommon::q_shared::entityState_t {
            number: 0,
            eType: 0,
            eFlags: 0,
            pos: crate::src::qcommon::q_shared::trajectory_t {
                trType: crate::src::qcommon::q_shared::TR_STATIONARY,
                trTime: 0,
                trDuration: 0,
                trBase: [0.; 3],
                trDelta: [0.; 3],
            },
            apos: crate::src::qcommon::q_shared::trajectory_t {
                trType: crate::src::qcommon::q_shared::TR_STATIONARY,
                trTime: 0,
                trDuration: 0,
                trBase: [0.; 3],
                trDelta: [0.; 3],
            },
            time: 0,
            time2: 0,
            origin: [0.; 3],
            origin2: [0.; 3],
            angles: [0.; 3],
            angles2: [0.; 3],
            otherEntityNum: 0,
            otherEntityNum2: 0,
            groundEntityNum: 0,
            constantLight: 0,
            loopSound: 0,
            modelindex: 0,
            modelindex2: 0,
            clientNum: 0,
            frame: 0,
            solid: 0,
            event: 0,
            eventParm: 0,
            powerups: 0,
            weapon: 0,
            legsAnim: 0,
            torsoAnim: 0,
            generic1: 0,
        };
    // if there is a dead body wearing kamikze nearby
    if (*bs).kamikazebody != 0 {
        // if the bot's view angles and weapon are not used for movement
        if (*moveresult).flags & (1 as i32 | 16 as i32) == 0 {
            //
            crate::src::game::ai_main::BotAI_GetEntityState(
                (*bs).kamikazebody,
                &mut state as *mut _ as *mut crate::src::qcommon::q_shared::entityState_s,
            );
            target[0 as i32 as usize] = state.pos.trBase[0 as i32 as usize];
            target[1 as i32 as usize] = state.pos.trBase[1 as i32 as usize];
            target[2 as i32 as usize] = state.pos.trBase[2 as i32 as usize];
            target[2 as i32 as usize] += 8 as i32 as f32;
            dir[0 as i32 as usize] = target[0 as i32 as usize] - (*bs).eye[0 as i32 as usize];
            dir[1 as i32 as usize] = target[1 as i32 as usize] - (*bs).eye[1 as i32 as usize];
            dir[2 as i32 as usize] = target[2 as i32 as usize] - (*bs).eye[2 as i32 as usize];
            crate::src::qcommon::q_math::vectoangles(
                dir.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
                (*moveresult).ideal_viewangles.as_mut_ptr(),
            );
            //
            (*moveresult).weapon = BotSelectActivateWeapon(bs);
            if (*moveresult).weapon == -(1 as i32) {
                // FIXME: run away!
                (*moveresult).weapon = 0 as i32
            }
            if (*moveresult).weapon != 0 {
                //
                (*moveresult).flags |= 16 as i32 | 1 as i32;
                // if holding the right weapon
                if (*bs).cur_ps.weapon == (*moveresult).weapon {
                    // if the bot is pretty close with its aim
                    if crate::src::game::ai_dmq3::InFieldOfVision(
                        (*bs).viewangles.as_mut_ptr(),
                        20 as i32 as f32,
                        (*moveresult).ideal_viewangles.as_mut_ptr(),
                    ) as u64
                        != 0
                    {
                        //
                        crate::src::game::ai_main::BotAI_Trace(
                            &mut bsptrace as *mut _ as *mut crate::botlib_h::bsp_trace_s,
                            (*bs).eye.as_mut_ptr(),
                            0 as *mut crate::src::qcommon::q_shared::vec_t,
                            0 as *mut crate::src::qcommon::q_shared::vec_t,
                            target.as_mut_ptr(),
                            (*bs).entitynum,
                            1 as i32 | 0x2000000 as i32 | 0x4000000 as i32,
                        );
                        // if the mine is visible from the current position
                        if bsptrace.fraction as f64 >= 1.0f64 || bsptrace.ent == state.number {
                            // shoot at the mine
                            crate::src::game::g_syscalls::trap_EA_Attack((*bs).client);
                        }
                    }
                }
            }
        }
    }
    if (*moveresult).flags & 256 as i32 != 0 {
        (*bs).blockedbyavoidspot_time = crate::src::game::ai_main::floattime + 5 as i32 as f32
    }
    // if blocked by an avoid spot and the view angles and weapon are used for movement
    if (*bs).blockedbyavoidspot_time > crate::src::game::ai_main::floattime
        && (*moveresult).flags & (1 as i32 | 16 as i32) == 0
    {
        bestdist = 300 as i32 as f32;
        bestmine = -(1 as i32);
        i = 0 as i32;
        while i < (*bs).numproxmines {
            crate::src::game::ai_main::BotAI_GetEntityState(
                (*bs).proxmines[i as usize],
                &mut state as *mut _ as *mut crate::src::qcommon::q_shared::entityState_s,
            );
            dir[0 as i32 as usize] =
                state.pos.trBase[0 as i32 as usize] - (*bs).origin[0 as i32 as usize];
            dir[1 as i32 as usize] =
                state.pos.trBase[1 as i32 as usize] - (*bs).origin[1 as i32 as usize];
            dir[2 as i32 as usize] =
                state.pos.trBase[2 as i32 as usize] - (*bs).origin[2 as i32 as usize];
            dist = VectorLength(dir.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t);
            if dist < bestdist {
                bestdist = dist;
                bestmine = i
            }
            i += 1
        }
        if bestmine != -(1 as i32) {
            //
            // state->generic1 == TEAM_RED || state->generic1 == TEAM_BLUE
            //
            // deactivate prox mines in the bot's path by shooting
            // rockets or plasma cells etc. at them
            crate::src::game::ai_main::BotAI_GetEntityState(
                (*bs).proxmines[bestmine as usize],
                &mut state as *mut _ as *mut crate::src::qcommon::q_shared::entityState_s,
            );
            target[0 as i32 as usize] = state.pos.trBase[0 as i32 as usize];
            target[1 as i32 as usize] = state.pos.trBase[1 as i32 as usize];
            target[2 as i32 as usize] = state.pos.trBase[2 as i32 as usize];
            target[2 as i32 as usize] += 2 as i32 as f32;
            dir[0 as i32 as usize] = target[0 as i32 as usize] - (*bs).eye[0 as i32 as usize];
            dir[1 as i32 as usize] = target[1 as i32 as usize] - (*bs).eye[1 as i32 as usize];
            dir[2 as i32 as usize] = target[2 as i32 as usize] - (*bs).eye[2 as i32 as usize];
            crate::src::qcommon::q_math::vectoangles(
                dir.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
                (*moveresult).ideal_viewangles.as_mut_ptr(),
            );
            // if the bot has a weapon that does splash damage
            if (*bs).inventory[11 as i32 as usize] > 0 as i32
                && (*bs).inventory[21 as i32 as usize] > 0 as i32
            {
                (*moveresult).weapon = 8 as i32
            } else if (*bs).inventory[8 as i32 as usize] > 0 as i32
                && (*bs).inventory[23 as i32 as usize] > 0 as i32
            {
                (*moveresult).weapon = 5 as i32
            } else if (*bs).inventory[13 as i32 as usize] > 0 as i32
                && (*bs).inventory[25 as i32 as usize] > 0 as i32
            {
                (*moveresult).weapon = 9 as i32
            } else {
                (*moveresult).weapon = 0 as i32
            }
            if (*moveresult).weapon != 0 {
                //
                (*moveresult).flags |= 16 as i32 | 1 as i32;
                // if holding the right weapon
                if (*bs).cur_ps.weapon == (*moveresult).weapon {
                    // if the bot is pretty close with its aim
                    if crate::src::game::ai_dmq3::InFieldOfVision(
                        (*bs).viewangles.as_mut_ptr(),
                        20 as i32 as f32,
                        (*moveresult).ideal_viewangles.as_mut_ptr(),
                    ) as u64
                        != 0
                    {
                        //
                        crate::src::game::ai_main::BotAI_Trace(
                            &mut bsptrace as *mut _ as *mut crate::botlib_h::bsp_trace_s,
                            (*bs).eye.as_mut_ptr(),
                            0 as *mut crate::src::qcommon::q_shared::vec_t,
                            0 as *mut crate::src::qcommon::q_shared::vec_t,
                            target.as_mut_ptr(),
                            (*bs).entitynum,
                            1 as i32 | 0x2000000 as i32 | 0x4000000 as i32,
                        );
                        // if the mine is visible from the current position
                        if bsptrace.fraction as f64 >= 1.0f64 || bsptrace.ent == state.number {
                            // shoot at the mine
                            crate::src::game::g_syscalls::trap_EA_Attack((*bs).client);
                        }
                    }
                }
            }
        }
    };
}
/*
==================
AIEnter_Seek_ActivateEntity
==================
*/
#[no_mangle]

pub unsafe extern "C" fn AIEnter_Seek_ActivateEntity(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut s: *mut libc::c_char,
) {
    BotRecordNodeSwitch(
        bs,
        b"activate entity\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        s,
    );
    (*bs).ainode = Some(
        AINode_Seek_ActivateEntity
            as unsafe extern "C" fn(_: *mut crate::src::game::ai_main::bot_state_t) -> i32,
    );
}
/*
==================
AINode_Seek_Activate_Entity
==================
*/
#[no_mangle]

pub unsafe extern "C" fn AINode_Seek_ActivateEntity(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
) -> i32 {
    let mut goal: *mut crate::be_ai_goal_h::bot_goal_t = 0 as *mut crate::be_ai_goal_h::bot_goal_t;
    let mut target: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut dir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut ideal_viewangles: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut moveresult: crate::be_ai_move_h::bot_moveresult_t =
        crate::be_ai_move_h::bot_moveresult_t {
            failure: 0,
            type_0: 0,
            blocked: 0,
            blockentity: 0,
            traveltype: 0,
            flags: 0,
            weapon: 0,
            movedir: [0.; 3],
            ideal_viewangles: [0.; 3],
        };
    let mut targetvisible: i32 = 0;
    let mut bsptrace: crate::botlib_h::bsp_trace_t = crate::botlib_h::bsp_trace_t {
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
        exp_dist: 0.,
        sidenum: 0,
        surface: crate::botlib_h::bsp_surface_t {
            name: [0; 16],
            flags: 0,
            value: 0,
        },
        contents: 0,
        ent: 0,
    };
    let mut entinfo: crate::be_aas_h::aas_entityinfo_t = crate::be_aas_h::aas_entityinfo_t {
        valid: 0,
        type_0: 0,
        flags: 0,
        ltime: 0.,
        update_time: 0.,
        number: 0,
        origin: [0.; 3],
        angles: [0.; 3],
        old_origin: [0.; 3],
        lastvisorigin: [0.; 3],
        mins: [0.; 3],
        maxs: [0.; 3],
        groundent: 0,
        solid: 0,
        modelindex: 0,
        modelindex2: 0,
        frame: 0,
        event: 0,
        eventParm: 0,
        powerups: 0,
        weapon: 0,
        legsAnim: 0,
        torsoAnim: 0,
    };
    if crate::src::game::ai_dmq3::BotIsObserver(bs as *mut crate::src::game::ai_main::bot_state_s)
        as u64
        != 0
    {
        crate::src::game::ai_dmq3::BotClearActivateGoalStack(
            bs as *mut crate::src::game::ai_main::bot_state_s,
        );
        AIEnter_Observer(
            bs,
            b"active entity: observer\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    //if in the intermission
    if crate::src::game::ai_dmq3::BotIntermission(bs as *mut crate::src::game::ai_main::bot_state_s)
        as u64
        != 0
    {
        crate::src::game::ai_dmq3::BotClearActivateGoalStack(
            bs as *mut crate::src::game::ai_main::bot_state_s,
        );
        AIEnter_Intermission(
            bs,
            b"activate entity: intermission\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    //respawn if dead
    if crate::src::game::ai_dmq3::BotIsDead(bs as *mut crate::src::game::ai_main::bot_state_s)
        as u64
        != 0
    {
        crate::src::game::ai_dmq3::BotClearActivateGoalStack(
            bs as *mut crate::src::game::ai_main::bot_state_s,
        );
        AIEnter_Respawn(
            bs,
            b"activate entity: bot dead\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    //
    (*bs).tfl = 0x2 as i32
        | 0x4 as i32
        | 0x8 as i32
        | 0x10 as i32
        | 0x20 as i32
        | 0x80 as i32
        | 0x100 as i32
        | 0x200 as i32
        | 0x400 as i32
        | 0x800 as i32
        | 0x80000 as i32
        | 0x100000 as i32
        | 0x40000 as i32
        | 0x1000000 as i32;
    if crate::src::game::ai_dmq3::bot_grapple.integer != 0 {
        (*bs).tfl |= 0x4000 as i32
    }
    // if in lava or slime the bot should be able to get out
    if crate::src::game::ai_dmq3::BotInLavaOrSlime(
        bs as *mut crate::src::game::ai_main::bot_state_s,
    ) as u64
        != 0
    {
        (*bs).tfl |= 0x400000 as i32 | 0x200000 as i32
    }
    // map specific code
    crate::src::game::ai_dmq3::BotMapScripts(bs as *mut crate::src::game::ai_main::bot_state_s);
    // no enemy
    (*bs).enemy = -(1 as i32);
    // if the bot has no activate goal
    if (*bs).activatestack.is_null() {
        crate::src::game::ai_dmq3::BotClearActivateGoalStack(
            bs as *mut crate::src::game::ai_main::bot_state_s,
        );
        AIEnter_Seek_NBG(
            bs,
            b"activate entity: no goal\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    //
    goal = &mut (*(*bs).activatestack).goal;
    // initialize target being visible to false
    targetvisible = crate::src::qcommon::q_shared::qfalse as i32;
    // if the bot has to shoot at a target to activate something
    if (*(*bs).activatestack).shoot != 0 {
        //
        crate::src::game::ai_main::BotAI_Trace(
            &mut bsptrace as *mut _ as *mut crate::botlib_h::bsp_trace_s,
            (*bs).eye.as_mut_ptr(),
            0 as *mut crate::src::qcommon::q_shared::vec_t,
            0 as *mut crate::src::qcommon::q_shared::vec_t,
            (*(*bs).activatestack).target.as_mut_ptr(),
            (*bs).entitynum,
            1 as i32 | 0x2000000 as i32 | 0x4000000 as i32,
        );
        // if the shootable entity is visible from the current position
        if bsptrace.fraction as f64 >= 1.0f64 || bsptrace.ent == (*goal).entitynum {
            targetvisible = crate::src::qcommon::q_shared::qtrue as i32;
            // if holding the right weapon
            if (*bs).cur_ps.weapon == (*(*bs).activatestack).weapon {
                dir[0 as i32 as usize] =
                    (*(*bs).activatestack).target[0 as i32 as usize] - (*bs).eye[0 as i32 as usize];
                dir[1 as i32 as usize] =
                    (*(*bs).activatestack).target[1 as i32 as usize] - (*bs).eye[1 as i32 as usize];
                dir[2 as i32 as usize] =
                    (*(*bs).activatestack).target[2 as i32 as usize] - (*bs).eye[2 as i32 as usize];
                crate::src::qcommon::q_math::vectoangles(
                    dir.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
                    ideal_viewangles.as_mut_ptr(),
                );
                // if the bot is pretty close with its aim
                if crate::src::game::ai_dmq3::InFieldOfVision(
                    (*bs).viewangles.as_mut_ptr(),
                    20 as i32 as f32,
                    ideal_viewangles.as_mut_ptr(),
                ) as u64
                    != 0
                {
                    crate::src::game::g_syscalls::trap_EA_Attack((*bs).client);
                }
            }
        }
    }
    // if the shoot target is visible
    if targetvisible != 0 {
        // get the entity info of the entity the bot is shooting at
        crate::src::game::ai_main::BotEntityInfo(
            (*goal).entitynum,
            &mut entinfo as *mut _ as *mut crate::be_aas_h::aas_entityinfo_s,
        );
        // if the entity the bot shoots at moved
        if VectorCompare(
            (*(*bs).activatestack).origin.as_mut_ptr()
                as *const crate::src::qcommon::q_shared::vec_t,
            entinfo.origin.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        ) == 0
        {
            //DEBUG
            (*(*bs).activatestack).time = 0 as i32 as f32
        }
        // if the activate goal has been activated or the bot takes too long
        if (*(*bs).activatestack).time < crate::src::game::ai_main::floattime {
            crate::src::game::ai_dmq3::BotPopFromActivateGoalStack(
                bs as *mut crate::src::game::ai_main::bot_state_s,
            );
            // if there are more activate goals on the stack
            if !(*bs).activatestack.is_null() {
                (*(*bs).activatestack).time =
                    crate::src::game::ai_main::floattime + 10 as i32 as f32;
                return crate::src::qcommon::q_shared::qfalse as i32;
            }
            AIEnter_Seek_NBG(
                bs,
                b"activate entity: time out\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            return crate::src::qcommon::q_shared::qfalse as i32;
        }
        crate::stdlib::memset(
            &mut moveresult as *mut crate::be_ai_move_h::bot_moveresult_t as *mut libc::c_void,
            0 as i32,
            ::std::mem::size_of::<crate::be_ai_move_h::bot_moveresult_t>() as libc::c_ulong,
        );
    } else {
        // if the bot has no goal
        if goal.is_null() {
            (*(*bs).activatestack).time = 0 as i32 as f32
        } else if (*(*bs).activatestack).shoot == 0 {
            // if the bot does not have a shoot goal
            //if the bot touches the current goal
            if crate::src::game::g_syscalls::trap_BotTouchingGoal(
                (*bs).origin.as_mut_ptr(),
                goal as *mut libc::c_void,
            ) != 0
            {
                //DEBUG
                (*(*bs).activatestack).time = 0 as i32 as f32
            }
        }
        // if the activate goal has been activated or the bot takes too long
        if (*(*bs).activatestack).time < crate::src::game::ai_main::floattime {
            crate::src::game::ai_dmq3::BotPopFromActivateGoalStack(
                bs as *mut crate::src::game::ai_main::bot_state_s,
            );
            // if there are more activate goals on the stack
            if !(*bs).activatestack.is_null() {
                (*(*bs).activatestack).time =
                    crate::src::game::ai_main::floattime + 10 as i32 as f32;
                return crate::src::qcommon::q_shared::qfalse as i32;
            }
            AIEnter_Seek_NBG(
                bs,
                b"activate entity: activated\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            return crate::src::qcommon::q_shared::qfalse as i32;
        }
        //predict obstacles
        if crate::src::game::ai_dmq3::BotAIPredictObstacles(
            bs as *mut crate::src::game::ai_main::bot_state_s,
            goal as *mut crate::be_ai_goal_h::bot_goal_s,
        ) != 0
        {
            return crate::src::qcommon::q_shared::qfalse as i32;
        }
        //initialize the movement state
        crate::src::game::ai_dmq3::BotSetupForMovement(
            bs as *mut crate::src::game::ai_main::bot_state_s,
        );
        //move towards the goal
        crate::src::game::g_syscalls::trap_BotMoveToGoal(
            &mut moveresult as *mut crate::be_ai_move_h::bot_moveresult_t as *mut libc::c_void,
            (*bs).ms,
            goal as *mut libc::c_void,
            (*bs).tfl,
        );
        //if the movement failed
        if moveresult.failure != 0 {
            //reset the avoid reach, otherwise bot is stuck in current area
            crate::src::game::g_syscalls::trap_BotResetAvoidReach((*bs).ms);
            //
            (*(*bs).activatestack).time = 0 as i32 as f32
        }
        //check if the bot is blocked
        crate::src::game::ai_dmq3::BotAIBlocked(
            bs as *mut crate::src::game::ai_main::bot_state_s,
            &mut moveresult as *mut _ as *mut crate::be_ai_move_h::bot_moveresult_s,
            crate::src::qcommon::q_shared::qtrue as i32,
        );
    }
    //
    BotClearPath(bs, &mut moveresult);
    // if the bot has to shoot to activate
    if (*(*bs).activatestack).shoot != 0 {
        // if the view angles aren't yet used for the movement
        if moveresult.flags & 1 as i32 == 0 {
            dir[0 as i32 as usize] =
                (*(*bs).activatestack).target[0 as i32 as usize] - (*bs).eye[0 as i32 as usize];
            dir[1 as i32 as usize] =
                (*(*bs).activatestack).target[1 as i32 as usize] - (*bs).eye[1 as i32 as usize];
            dir[2 as i32 as usize] =
                (*(*bs).activatestack).target[2 as i32 as usize] - (*bs).eye[2 as i32 as usize];
            crate::src::qcommon::q_math::vectoangles(
                dir.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
                moveresult.ideal_viewangles.as_mut_ptr(),
            );
            moveresult.flags |= 1 as i32
        }
        // if there's no weapon yet used for the movement
        if moveresult.flags & 16 as i32 == 0 {
            moveresult.flags |= 16 as i32;
            //
            (*(*bs).activatestack).weapon = BotSelectActivateWeapon(bs);
            if (*(*bs).activatestack).weapon == -(1 as i32) {
                //FIXME: find a decent weapon first
                (*(*bs).activatestack).weapon = 0 as i32
            }
            moveresult.weapon = (*(*bs).activatestack).weapon
        }
    }
    // if the ideal view angles are set for movement
    if moveresult.flags & (8 as i32 | 1 as i32 | 2 as i32) != 0 {
        (*bs).ideal_viewangles[0 as i32 as usize] = moveresult.ideal_viewangles[0 as i32 as usize];
        (*bs).ideal_viewangles[1 as i32 as usize] = moveresult.ideal_viewangles[1 as i32 as usize];
        (*bs).ideal_viewangles[2 as i32 as usize] = moveresult.ideal_viewangles[2 as i32 as usize]
    } else if moveresult.flags & 4 as i32 != 0 {
        if (((::libc::rand() & 0x7fff as i32) as f32 / 0x7fff as i32 as f32) as f64)
            < (*bs).thinktime as f64 * 0.8f64
        {
            crate::src::game::ai_dmq3::BotRoamGoal(
                bs as *mut crate::src::game::ai_main::bot_state_s,
                target.as_mut_ptr(),
            );
            dir[0 as i32 as usize] = target[0 as i32 as usize] - (*bs).origin[0 as i32 as usize];
            dir[1 as i32 as usize] = target[1 as i32 as usize] - (*bs).origin[1 as i32 as usize];
            dir[2 as i32 as usize] = target[2 as i32 as usize] - (*bs).origin[2 as i32 as usize];
            crate::src::qcommon::q_math::vectoangles(
                dir.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
                (*bs).ideal_viewangles.as_mut_ptr(),
            );
            (*bs).ideal_viewangles[2 as i32 as usize] =
                ((*bs).ideal_viewangles[2 as i32 as usize] as f64 * 0.5f64)
                    as crate::src::qcommon::q_shared::vec_t
        }
    } else if (*bs).flags & 32 as i32 == 0 {
        if crate::src::game::g_syscalls::trap_BotMovementViewTarget(
            (*bs).ms,
            goal as *mut libc::c_void,
            (*bs).tfl,
            300 as i32 as f32,
            target.as_mut_ptr(),
        ) != 0
        {
            dir[0 as i32 as usize] = target[0 as i32 as usize] - (*bs).origin[0 as i32 as usize];
            dir[1 as i32 as usize] = target[1 as i32 as usize] - (*bs).origin[1 as i32 as usize];
            dir[2 as i32 as usize] = target[2 as i32 as usize] - (*bs).origin[2 as i32 as usize];
            crate::src::qcommon::q_math::vectoangles(
                dir.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
                (*bs).ideal_viewangles.as_mut_ptr(),
            );
        } else {
            crate::src::qcommon::q_math::vectoangles(
                moveresult.movedir.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
                (*bs).ideal_viewangles.as_mut_ptr(),
            );
        }
        (*bs).ideal_viewangles[2 as i32 as usize] =
            ((*bs).ideal_viewangles[2 as i32 as usize] as f64 * 0.5f64)
                as crate::src::qcommon::q_shared::vec_t
    }
    // if waiting for something
    // if the weapon is used for the bot movement
    if moveresult.flags & 16 as i32 != 0 {
        (*bs).weaponnum = moveresult.weapon
    }
    // if there is an enemy
    if crate::src::game::ai_dmq3::BotFindEnemy(
        bs as *mut crate::src::game::ai_main::bot_state_s,
        -(1 as i32),
    ) != 0
    {
        if crate::src::game::ai_dmq3::BotWantsToRetreat(
            bs as *mut crate::src::game::ai_main::bot_state_s,
        ) != 0
        {
            //keep the current long term goal and retreat
            AIEnter_Battle_NBG(
                bs,
                b"activate entity: found enemy\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        } else {
            crate::src::game::g_syscalls::trap_BotResetLastAvoidReach((*bs).ms);
            //empty the goal stack
            crate::src::game::g_syscalls::trap_BotEmptyGoalStack((*bs).gs);
            //go fight
            AIEnter_Battle_Fight(
                bs,
                b"activate entity: found enemy\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
        crate::src::game::ai_dmq3::BotClearActivateGoalStack(
            bs as *mut crate::src::game::ai_main::bot_state_s,
        );
    }
    return crate::src::qcommon::q_shared::qtrue as i32;
}
/*
==================
AIEnter_Seek_NBG
==================
*/
#[no_mangle]

pub unsafe extern "C" fn AIEnter_Seek_NBG(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut s: *mut libc::c_char,
) {
    let mut goal: crate::be_ai_goal_h::bot_goal_t = crate::be_ai_goal_h::bot_goal_t {
        origin: [0.; 3],
        areanum: 0,
        mins: [0.; 3],
        maxs: [0.; 3],
        entitynum: 0,
        number: 0,
        flags: 0,
        iteminfo: 0,
    };
    let mut buf: [libc::c_char; 144] = [0; 144];
    if crate::src::game::g_syscalls::trap_BotGetTopGoal(
        (*bs).gs,
        &mut goal as *mut crate::be_ai_goal_h::bot_goal_t as *mut libc::c_void,
    ) != 0
    {
        crate::src::game::g_syscalls::trap_BotGoalName(goal.number, buf.as_mut_ptr(), 144 as i32);
        BotRecordNodeSwitch(
            bs,
            b"seek NBG\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            buf.as_mut_ptr(),
            s,
        );
    } else {
        BotRecordNodeSwitch(
            bs,
            b"seek NBG\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"no goal\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            s,
        );
    }
    (*bs).ainode = Some(
        AINode_Seek_NBG
            as unsafe extern "C" fn(_: *mut crate::src::game::ai_main::bot_state_t) -> i32,
    );
}
/*
==================
AINode_Seek_NBG
==================
*/
#[no_mangle]

pub unsafe extern "C" fn AINode_Seek_NBG(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
) -> i32 {
    let mut goal: crate::be_ai_goal_h::bot_goal_t = crate::be_ai_goal_h::bot_goal_t {
        origin: [0.; 3],
        areanum: 0,
        mins: [0.; 3],
        maxs: [0.; 3],
        entitynum: 0,
        number: 0,
        flags: 0,
        iteminfo: 0,
    };
    let mut target: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut dir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut moveresult: crate::be_ai_move_h::bot_moveresult_t =
        crate::be_ai_move_h::bot_moveresult_t {
            failure: 0,
            type_0: 0,
            blocked: 0,
            blockentity: 0,
            traveltype: 0,
            flags: 0,
            weapon: 0,
            movedir: [0.; 3],
            ideal_viewangles: [0.; 3],
        };
    if crate::src::game::ai_dmq3::BotIsObserver(bs as *mut crate::src::game::ai_main::bot_state_s)
        as u64
        != 0
    {
        AIEnter_Observer(
            bs,
            b"seek nbg: observer\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    //if in the intermission
    if crate::src::game::ai_dmq3::BotIntermission(bs as *mut crate::src::game::ai_main::bot_state_s)
        as u64
        != 0
    {
        AIEnter_Intermission(
            bs,
            b"seek nbg: intermision\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    //respawn if dead
    if crate::src::game::ai_dmq3::BotIsDead(bs as *mut crate::src::game::ai_main::bot_state_s)
        as u64
        != 0
    {
        AIEnter_Respawn(
            bs,
            b"seek nbg: bot dead\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    //
    (*bs).tfl = 0x2 as i32
        | 0x4 as i32
        | 0x8 as i32
        | 0x10 as i32
        | 0x20 as i32
        | 0x80 as i32
        | 0x100 as i32
        | 0x200 as i32
        | 0x400 as i32
        | 0x800 as i32
        | 0x80000 as i32
        | 0x100000 as i32
        | 0x40000 as i32
        | 0x1000000 as i32;
    if crate::src::game::ai_dmq3::bot_grapple.integer != 0 {
        (*bs).tfl |= 0x4000 as i32
    }
    //if in lava or slime the bot should be able to get out
    if crate::src::game::ai_dmq3::BotInLavaOrSlime(
        bs as *mut crate::src::game::ai_main::bot_state_s,
    ) as u64
        != 0
    {
        (*bs).tfl |= 0x400000 as i32 | 0x200000 as i32
    }
    //
    if crate::src::game::ai_dmq3::BotCanAndWantsToRocketJump(
        bs as *mut crate::src::game::ai_main::bot_state_s,
    ) != 0
    {
        (*bs).tfl |= 0x1000 as i32
    }
    //map specific code
    crate::src::game::ai_dmq3::BotMapScripts(bs as *mut crate::src::game::ai_main::bot_state_s);
    //no enemy
    (*bs).enemy = -(1 as i32);
    //if the bot has no goal
    if crate::src::game::g_syscalls::trap_BotGetTopGoal(
        (*bs).gs,
        &mut goal as *mut crate::be_ai_goal_h::bot_goal_t as *mut libc::c_void,
    ) == 0
    {
        (*bs).nbg_time = 0 as i32 as f32
    } else if BotReachedGoal(bs, &mut goal) != 0 {
        crate::src::game::ai_dmq3::BotChooseWeapon(
            bs as *mut crate::src::game::ai_main::bot_state_s,
        );
        (*bs).nbg_time = 0 as i32 as f32
    }
    //if the bot touches the current goal
    //
    if (*bs).nbg_time < crate::src::game::ai_main::floattime {
        //pop the current goal from the stack
        crate::src::game::g_syscalls::trap_BotPopGoal((*bs).gs);
        //check for new nearby items right away
        //NOTE: we canNOT reset the check_time to zero because it would create an endless loop of node switches
        (*bs).check_time = (crate::src::game::ai_main::floattime as f64 + 0.05f64) as f32;
        //go back to seek ltg
        AIEnter_Seek_LTG(
            bs,
            b"seek nbg: time out\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    //predict obstacles
    if crate::src::game::ai_dmq3::BotAIPredictObstacles(
        bs as *mut crate::src::game::ai_main::bot_state_s,
        &mut goal as *mut _ as *mut crate::be_ai_goal_h::bot_goal_s,
    ) != 0
    {
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    //initialize the movement state
    crate::src::game::ai_dmq3::BotSetupForMovement(
        bs as *mut crate::src::game::ai_main::bot_state_s,
    );
    //move towards the goal
    crate::src::game::g_syscalls::trap_BotMoveToGoal(
        &mut moveresult as *mut crate::be_ai_move_h::bot_moveresult_t as *mut libc::c_void,
        (*bs).ms,
        &mut goal as *mut crate::be_ai_goal_h::bot_goal_t as *mut libc::c_void,
        (*bs).tfl,
    );
    //if the movement failed
    if moveresult.failure != 0 {
        //reset the avoid reach, otherwise bot is stuck in current area
        crate::src::game::g_syscalls::trap_BotResetAvoidReach((*bs).ms);
        (*bs).nbg_time = 0 as i32 as f32
    }
    //check if the bot is blocked
    crate::src::game::ai_dmq3::BotAIBlocked(
        bs as *mut crate::src::game::ai_main::bot_state_s,
        &mut moveresult as *mut _ as *mut crate::be_ai_move_h::bot_moveresult_s,
        crate::src::qcommon::q_shared::qtrue as i32,
    );
    //
    BotClearPath(bs, &mut moveresult);
    //if the viewangles are used for the movement
    if moveresult.flags & (8 as i32 | 1 as i32 | 2 as i32) != 0 {
        (*bs).ideal_viewangles[0 as i32 as usize] = moveresult.ideal_viewangles[0 as i32 as usize];
        (*bs).ideal_viewangles[1 as i32 as usize] = moveresult.ideal_viewangles[1 as i32 as usize];
        (*bs).ideal_viewangles[2 as i32 as usize] = moveresult.ideal_viewangles[2 as i32 as usize]
    } else if moveresult.flags & 4 as i32 != 0 {
        if (((::libc::rand() & 0x7fff as i32) as f32 / 0x7fff as i32 as f32) as f64)
            < (*bs).thinktime as f64 * 0.8f64
        {
            crate::src::game::ai_dmq3::BotRoamGoal(
                bs as *mut crate::src::game::ai_main::bot_state_s,
                target.as_mut_ptr(),
            );
            dir[0 as i32 as usize] = target[0 as i32 as usize] - (*bs).origin[0 as i32 as usize];
            dir[1 as i32 as usize] = target[1 as i32 as usize] - (*bs).origin[1 as i32 as usize];
            dir[2 as i32 as usize] = target[2 as i32 as usize] - (*bs).origin[2 as i32 as usize];
            crate::src::qcommon::q_math::vectoangles(
                dir.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
                (*bs).ideal_viewangles.as_mut_ptr(),
            );
            (*bs).ideal_viewangles[2 as i32 as usize] =
                ((*bs).ideal_viewangles[2 as i32 as usize] as f64 * 0.5f64)
                    as crate::src::qcommon::q_shared::vec_t
        }
    } else if (*bs).flags & 32 as i32 == 0 {
        if crate::src::game::g_syscalls::trap_BotGetSecondGoal(
            (*bs).gs,
            &mut goal as *mut crate::be_ai_goal_h::bot_goal_t as *mut libc::c_void,
        ) == 0
        {
            crate::src::game::g_syscalls::trap_BotGetTopGoal(
                (*bs).gs,
                &mut goal as *mut crate::be_ai_goal_h::bot_goal_t as *mut libc::c_void,
            );
        }
        if crate::src::game::g_syscalls::trap_BotMovementViewTarget(
            (*bs).ms,
            &mut goal as *mut crate::be_ai_goal_h::bot_goal_t as *mut libc::c_void,
            (*bs).tfl,
            300 as i32 as f32,
            target.as_mut_ptr(),
        ) != 0
        {
            dir[0 as i32 as usize] = target[0 as i32 as usize] - (*bs).origin[0 as i32 as usize];
            dir[1 as i32 as usize] = target[1 as i32 as usize] - (*bs).origin[1 as i32 as usize];
            dir[2 as i32 as usize] = target[2 as i32 as usize] - (*bs).origin[2 as i32 as usize];
            crate::src::qcommon::q_math::vectoangles(
                dir.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
                (*bs).ideal_viewangles.as_mut_ptr(),
            );
        } else {
            //if waiting for something
            //FIXME: look at cluster portals?
            crate::src::qcommon::q_math::vectoangles(
                moveresult.movedir.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
                (*bs).ideal_viewangles.as_mut_ptr(),
            );
        }
        (*bs).ideal_viewangles[2 as i32 as usize] =
            ((*bs).ideal_viewangles[2 as i32 as usize] as f64 * 0.5f64)
                as crate::src::qcommon::q_shared::vec_t
    }
    //if the weapon is used for the bot movement
    if moveresult.flags & 16 as i32 != 0 {
        (*bs).weaponnum = moveresult.weapon
    }
    //if there is an enemy
    if crate::src::game::ai_dmq3::BotFindEnemy(
        bs as *mut crate::src::game::ai_main::bot_state_s,
        -(1 as i32),
    ) != 0
    {
        if crate::src::game::ai_dmq3::BotWantsToRetreat(
            bs as *mut crate::src::game::ai_main::bot_state_s,
        ) != 0
        {
            //keep the current long term goal and retreat
            AIEnter_Battle_NBG(
                bs,
                b"seek nbg: found enemy\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        } else {
            crate::src::game::g_syscalls::trap_BotResetLastAvoidReach((*bs).ms);
            //empty the goal stack
            crate::src::game::g_syscalls::trap_BotEmptyGoalStack((*bs).gs);
            //go fight
            AIEnter_Battle_Fight(
                bs,
                b"seek nbg: found enemy\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
    }
    return crate::src::qcommon::q_shared::qtrue as i32;
}
/*
==================
AIEnter_Seek_LTG
==================
*/
#[no_mangle]

pub unsafe extern "C" fn AIEnter_Seek_LTG(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut s: *mut libc::c_char,
) {
    let mut goal: crate::be_ai_goal_h::bot_goal_t = crate::be_ai_goal_h::bot_goal_t {
        origin: [0.; 3],
        areanum: 0,
        mins: [0.; 3],
        maxs: [0.; 3],
        entitynum: 0,
        number: 0,
        flags: 0,
        iteminfo: 0,
    };
    let mut buf: [libc::c_char; 144] = [0; 144];
    if crate::src::game::g_syscalls::trap_BotGetTopGoal(
        (*bs).gs,
        &mut goal as *mut crate::be_ai_goal_h::bot_goal_t as *mut libc::c_void,
    ) != 0
    {
        crate::src::game::g_syscalls::trap_BotGoalName(goal.number, buf.as_mut_ptr(), 144 as i32);
        BotRecordNodeSwitch(
            bs,
            b"seek LTG\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            buf.as_mut_ptr(),
            s,
        );
    } else {
        BotRecordNodeSwitch(
            bs,
            b"seek LTG\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"no goal\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            s,
        );
    }
    (*bs).ainode = Some(
        AINode_Seek_LTG
            as unsafe extern "C" fn(_: *mut crate::src::game::ai_main::bot_state_t) -> i32,
    );
}
/*
==================
AINode_Seek_LTG
==================
*/
#[no_mangle]

pub unsafe extern "C" fn AINode_Seek_LTG(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
) -> i32 {
    let mut goal: crate::be_ai_goal_h::bot_goal_t = crate::be_ai_goal_h::bot_goal_t {
        origin: [0.; 3],
        areanum: 0,
        mins: [0.; 3],
        maxs: [0.; 3],
        entitynum: 0,
        number: 0,
        flags: 0,
        iteminfo: 0,
    };
    let mut target: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut dir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut moveresult: crate::be_ai_move_h::bot_moveresult_t =
        crate::be_ai_move_h::bot_moveresult_t {
            failure: 0,
            type_0: 0,
            blocked: 0,
            blockentity: 0,
            traveltype: 0,
            flags: 0,
            weapon: 0,
            movedir: [0.; 3],
            ideal_viewangles: [0.; 3],
        };
    let mut range: i32 = 0;
    //char buf[128];
    //bot_goal_t tmpgoal;
    if crate::src::game::ai_dmq3::BotIsObserver(bs as *mut crate::src::game::ai_main::bot_state_s)
        as u64
        != 0
    {
        AIEnter_Observer(
            bs,
            b"seek ltg: observer\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    //if in the intermission
    if crate::src::game::ai_dmq3::BotIntermission(bs as *mut crate::src::game::ai_main::bot_state_s)
        as u64
        != 0
    {
        AIEnter_Intermission(
            bs,
            b"seek ltg: intermission\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    //respawn if dead
    if crate::src::game::ai_dmq3::BotIsDead(bs as *mut crate::src::game::ai_main::bot_state_s)
        as u64
        != 0
    {
        AIEnter_Respawn(
            bs,
            b"seek ltg: bot dead\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    //
    if crate::src::game::ai_chat::BotChat_Random(bs as *mut crate::src::game::ai_main::bot_state_s)
        != 0
    {
        (*bs).stand_time = crate::src::game::ai_main::floattime
            + crate::src::game::ai_chat::BotChatTime(
                bs as *mut crate::src::game::ai_main::bot_state_s,
            );
        AIEnter_Stand(
            bs,
            b"seek ltg: random chat\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    //
    (*bs).tfl = 0x2 as i32
        | 0x4 as i32
        | 0x8 as i32
        | 0x10 as i32
        | 0x20 as i32
        | 0x80 as i32
        | 0x100 as i32
        | 0x200 as i32
        | 0x400 as i32
        | 0x800 as i32
        | 0x80000 as i32
        | 0x100000 as i32
        | 0x40000 as i32
        | 0x1000000 as i32;
    if crate::src::game::ai_dmq3::bot_grapple.integer != 0 {
        (*bs).tfl |= 0x4000 as i32
    }
    //if in lava or slime the bot should be able to get out
    if crate::src::game::ai_dmq3::BotInLavaOrSlime(
        bs as *mut crate::src::game::ai_main::bot_state_s,
    ) as u64
        != 0
    {
        (*bs).tfl |= 0x400000 as i32 | 0x200000 as i32
    }
    //
    if crate::src::game::ai_dmq3::BotCanAndWantsToRocketJump(
        bs as *mut crate::src::game::ai_main::bot_state_s,
    ) != 0
    {
        (*bs).tfl |= 0x1000 as i32
    }
    //map specific code
    crate::src::game::ai_dmq3::BotMapScripts(bs as *mut crate::src::game::ai_main::bot_state_s);
    //no enemy
    (*bs).enemy = -(1 as i32);
    //
    if (*bs).killedenemy_time > crate::src::game::ai_main::floattime - 2 as i32 as f32 {
        if ((::libc::rand() & 0x7fff as i32) as f32 / 0x7fff as i32 as f32)
            < (*bs).thinktime * 1 as i32 as f32
        {
            crate::src::game::g_syscalls::trap_EA_Gesture((*bs).client);
        }
    }
    //if there is an enemy
    if crate::src::game::ai_dmq3::BotFindEnemy(
        bs as *mut crate::src::game::ai_main::bot_state_s,
        -(1 as i32),
    ) != 0
    {
        if crate::src::game::ai_dmq3::BotWantsToRetreat(
            bs as *mut crate::src::game::ai_main::bot_state_s,
        ) != 0
        {
            //keep the current long term goal and retreat
            AIEnter_Battle_Retreat(
                bs,
                b"seek ltg: found enemy\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            return crate::src::qcommon::q_shared::qfalse as i32;
        } else {
            crate::src::game::g_syscalls::trap_BotResetLastAvoidReach((*bs).ms);
            //empty the goal stack
            crate::src::game::g_syscalls::trap_BotEmptyGoalStack((*bs).gs);
            //go fight
            AIEnter_Battle_Fight(
                bs,
                b"seek ltg: found enemy\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            return crate::src::qcommon::q_shared::qfalse as i32;
        }
    }
    //
    crate::src::game::ai_dmq3::BotTeamGoals(
        bs as *mut crate::src::game::ai_main::bot_state_s,
        crate::src::qcommon::q_shared::qfalse as i32,
    );
    //get the current long term goal
    if BotLongTermGoal(
        bs,
        (*bs).tfl,
        crate::src::qcommon::q_shared::qfalse as i32,
        &mut goal,
    ) == 0
    {
        return crate::src::qcommon::q_shared::qtrue as i32;
    }
    //check for nearby goals periodicly
    if (*bs).check_time < crate::src::game::ai_main::floattime {
        (*bs).check_time = (crate::src::game::ai_main::floattime as f64 + 0.5f64) as f32;
        //check if the bot wants to camp
        crate::src::game::ai_dmq3::BotWantsToCamp(
            bs as *mut crate::src::game::ai_main::bot_state_s,
        );
        //
        if (*bs).ltgtype == 3 as i32 {
            range = 400 as i32
        } else {
            range = 150 as i32
        }
        //
        if crate::src::game::ai_dmq3::gametype == crate::bg_public_h::GT_CTF as i32 {
            //if carrying a flag the bot shouldn't be distracted too much
            if crate::src::game::ai_dmq3::BotCTFCarryingFlag(
                bs as *mut crate::src::game::ai_main::bot_state_s,
            ) != 0
            {
                range = 50 as i32
            }
        }
        //CTF
        //
        if BotNearbyGoal(bs, (*bs).tfl, &mut goal, range as f32) != 0 {
            crate::src::game::g_syscalls::trap_BotResetLastAvoidReach((*bs).ms);
            //get the goal at the top of the stack
            //trap_BotGetTopGoal(bs->gs, &tmpgoal);
            //trap_BotGoalName(tmpgoal.number, buf, 144);
            //BotAI_Print(PRT_MESSAGE, "new nearby goal %s\n", buf);
            //time the bot gets to pick up the nearby goal item
            (*bs).nbg_time = ((crate::src::game::ai_main::floattime + 4 as i32 as f32) as f64
                + range as f64 * 0.01f64) as f32;
            AIEnter_Seek_NBG(
                bs,
                b"ltg seek: nbg\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            return crate::src::qcommon::q_shared::qfalse as i32;
        }
    }
    //predict obstacles
    if crate::src::game::ai_dmq3::BotAIPredictObstacles(
        bs as *mut crate::src::game::ai_main::bot_state_s,
        &mut goal as *mut _ as *mut crate::be_ai_goal_h::bot_goal_s,
    ) != 0
    {
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    //initialize the movement state
    crate::src::game::ai_dmq3::BotSetupForMovement(
        bs as *mut crate::src::game::ai_main::bot_state_s,
    );
    //move towards the goal
    crate::src::game::g_syscalls::trap_BotMoveToGoal(
        &mut moveresult as *mut crate::be_ai_move_h::bot_moveresult_t as *mut libc::c_void,
        (*bs).ms,
        &mut goal as *mut crate::be_ai_goal_h::bot_goal_t as *mut libc::c_void,
        (*bs).tfl,
    );
    //if the movement failed
    if moveresult.failure != 0 {
        //reset the avoid reach, otherwise bot is stuck in current area
        crate::src::game::g_syscalls::trap_BotResetAvoidReach((*bs).ms);
        //BotAI_Print(PRT_MESSAGE, "movement failure %d\n", moveresult.traveltype);
        (*bs).ltg_time = 0 as i32 as f32
    }
    //
    crate::src::game::ai_dmq3::BotAIBlocked(
        bs as *mut crate::src::game::ai_main::bot_state_s,
        &mut moveresult as *mut _ as *mut crate::be_ai_move_h::bot_moveresult_s,
        crate::src::qcommon::q_shared::qtrue as i32,
    );
    //
    BotClearPath(bs, &mut moveresult);
    //if the viewangles are used for the movement
    if moveresult.flags & (8 as i32 | 1 as i32 | 2 as i32) != 0 {
        (*bs).ideal_viewangles[0 as i32 as usize] = moveresult.ideal_viewangles[0 as i32 as usize];
        (*bs).ideal_viewangles[1 as i32 as usize] = moveresult.ideal_viewangles[1 as i32 as usize];
        (*bs).ideal_viewangles[2 as i32 as usize] = moveresult.ideal_viewangles[2 as i32 as usize]
    } else if moveresult.flags & 4 as i32 != 0 {
        if (((::libc::rand() & 0x7fff as i32) as f32 / 0x7fff as i32 as f32) as f64)
            < (*bs).thinktime as f64 * 0.8f64
        {
            crate::src::game::ai_dmq3::BotRoamGoal(
                bs as *mut crate::src::game::ai_main::bot_state_s,
                target.as_mut_ptr(),
            );
            dir[0 as i32 as usize] = target[0 as i32 as usize] - (*bs).origin[0 as i32 as usize];
            dir[1 as i32 as usize] = target[1 as i32 as usize] - (*bs).origin[1 as i32 as usize];
            dir[2 as i32 as usize] = target[2 as i32 as usize] - (*bs).origin[2 as i32 as usize];
            crate::src::qcommon::q_math::vectoangles(
                dir.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
                (*bs).ideal_viewangles.as_mut_ptr(),
            );
            (*bs).ideal_viewangles[2 as i32 as usize] =
                ((*bs).ideal_viewangles[2 as i32 as usize] as f64 * 0.5f64)
                    as crate::src::qcommon::q_shared::vec_t
        }
    } else if (*bs).flags & 32 as i32 == 0 {
        if crate::src::game::g_syscalls::trap_BotMovementViewTarget(
            (*bs).ms,
            &mut goal as *mut crate::be_ai_goal_h::bot_goal_t as *mut libc::c_void,
            (*bs).tfl,
            300 as i32 as f32,
            target.as_mut_ptr(),
        ) != 0
        {
            dir[0 as i32 as usize] = target[0 as i32 as usize] - (*bs).origin[0 as i32 as usize];
            dir[1 as i32 as usize] = target[1 as i32 as usize] - (*bs).origin[1 as i32 as usize];
            dir[2 as i32 as usize] = target[2 as i32 as usize] - (*bs).origin[2 as i32 as usize];
            crate::src::qcommon::q_math::vectoangles(
                dir.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
                (*bs).ideal_viewangles.as_mut_ptr(),
            );
        } else if VectorLengthSquared(
            moveresult.movedir.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t
        ) != 0.
        {
            crate::src::qcommon::q_math::vectoangles(
                moveresult.movedir.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
                (*bs).ideal_viewangles.as_mut_ptr(),
            );
        } else if (((::libc::rand() & 0x7fff as i32) as f32 / 0x7fff as i32 as f32) as f64)
            < (*bs).thinktime as f64 * 0.8f64
        {
            crate::src::game::ai_dmq3::BotRoamGoal(
                bs as *mut crate::src::game::ai_main::bot_state_s,
                target.as_mut_ptr(),
            );
            dir[0 as i32 as usize] = target[0 as i32 as usize] - (*bs).origin[0 as i32 as usize];
            dir[1 as i32 as usize] = target[1 as i32 as usize] - (*bs).origin[1 as i32 as usize];
            dir[2 as i32 as usize] = target[2 as i32 as usize] - (*bs).origin[2 as i32 as usize];
            crate::src::qcommon::q_math::vectoangles(
                dir.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
                (*bs).ideal_viewangles.as_mut_ptr(),
            );
            (*bs).ideal_viewangles[2 as i32 as usize] =
                ((*bs).ideal_viewangles[2 as i32 as usize] as f64 * 0.5f64)
                    as crate::src::qcommon::q_shared::vec_t
        }
        (*bs).ideal_viewangles[2 as i32 as usize] =
            ((*bs).ideal_viewangles[2 as i32 as usize] as f64 * 0.5f64)
                as crate::src::qcommon::q_shared::vec_t
    }
    //if waiting for something
    //FIXME: look at cluster portals?
    //if the weapon is used for the bot movement
    if moveresult.flags & 16 as i32 != 0 {
        (*bs).weaponnum = moveresult.weapon
    }
    //
    return crate::src::qcommon::q_shared::qtrue as i32;
}
/*
==================
AIEnter_Battle_Fight
==================
*/
#[no_mangle]

pub unsafe extern "C" fn AIEnter_Battle_Fight(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut s: *mut libc::c_char,
) {
    BotRecordNodeSwitch(
        bs,
        b"battle fight\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        s,
    );
    crate::src::game::g_syscalls::trap_BotResetLastAvoidReach((*bs).ms);
    (*bs).ainode = Some(
        AINode_Battle_Fight
            as unsafe extern "C" fn(_: *mut crate::src::game::ai_main::bot_state_t) -> i32,
    );
    (*bs).flags &= !(64 as i32);
}
/*
==================
AIEnter_Battle_SuicidalFight
==================
*/
#[no_mangle]

pub unsafe extern "C" fn AIEnter_Battle_SuicidalFight(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut s: *mut libc::c_char,
) {
    BotRecordNodeSwitch(
        bs,
        b"battle fight\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        s,
    );
    crate::src::game::g_syscalls::trap_BotResetLastAvoidReach((*bs).ms);
    (*bs).ainode = Some(
        AINode_Battle_Fight
            as unsafe extern "C" fn(_: *mut crate::src::game::ai_main::bot_state_t) -> i32,
    );
    (*bs).flags |= 64 as i32;
}
/*
==================
AINode_Battle_Fight
==================
*/
#[no_mangle]

pub unsafe extern "C" fn AINode_Battle_Fight(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
) -> i32 {
    let mut areanum: i32 = 0;
    let mut target: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut entinfo: crate::be_aas_h::aas_entityinfo_t = crate::be_aas_h::aas_entityinfo_t {
        valid: 0,
        type_0: 0,
        flags: 0,
        ltime: 0.,
        update_time: 0.,
        number: 0,
        origin: [0.; 3],
        angles: [0.; 3],
        old_origin: [0.; 3],
        lastvisorigin: [0.; 3],
        mins: [0.; 3],
        maxs: [0.; 3],
        groundent: 0,
        solid: 0,
        modelindex: 0,
        modelindex2: 0,
        frame: 0,
        event: 0,
        eventParm: 0,
        powerups: 0,
        weapon: 0,
        legsAnim: 0,
        torsoAnim: 0,
    };
    let mut moveresult: crate::be_ai_move_h::bot_moveresult_t =
        crate::be_ai_move_h::bot_moveresult_t {
            failure: 0,
            type_0: 0,
            blocked: 0,
            blockentity: 0,
            traveltype: 0,
            flags: 0,
            weapon: 0,
            movedir: [0.; 3],
            ideal_viewangles: [0.; 3],
        };
    if crate::src::game::ai_dmq3::BotIsObserver(bs as *mut crate::src::game::ai_main::bot_state_s)
        as u64
        != 0
    {
        AIEnter_Observer(
            bs,
            b"battle fight: observer\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    //if in the intermission
    if crate::src::game::ai_dmq3::BotIntermission(bs as *mut crate::src::game::ai_main::bot_state_s)
        as u64
        != 0
    {
        AIEnter_Intermission(
            bs,
            b"battle fight: intermission\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    //respawn if dead
    if crate::src::game::ai_dmq3::BotIsDead(bs as *mut crate::src::game::ai_main::bot_state_s)
        as u64
        != 0
    {
        AIEnter_Respawn(
            bs,
            b"battle fight: bot dead\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    //if there is another better enemy
    crate::src::game::ai_dmq3::BotFindEnemy(
        bs as *mut crate::src::game::ai_main::bot_state_s,
        (*bs).enemy,
    );
    //if no enemy
    if (*bs).enemy < 0 as i32 {
        AIEnter_Seek_LTG(
            bs,
            b"battle fight: no enemy\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    //
    crate::src::game::ai_main::BotEntityInfo(
        (*bs).enemy,
        &mut entinfo as *mut _ as *mut crate::be_aas_h::aas_entityinfo_s,
    );
    //if the enemy is dead
    if (*bs).enemydeath_time != 0. {
        if ((*bs).enemydeath_time as f64) < crate::src::game::ai_main::floattime as f64 - 1.0f64 {
            (*bs).enemydeath_time = 0 as i32 as f32;
            if (*bs).enemysuicide != 0 {
                crate::src::game::ai_chat::BotChat_EnemySuicide(
                    bs as *mut crate::src::game::ai_main::bot_state_s,
                );
            }
            if (*bs).lastkilledplayer == (*bs).enemy
                && crate::src::game::ai_chat::BotChat_Kill(
                    bs as *mut crate::src::game::ai_main::bot_state_s,
                ) != 0
            {
                (*bs).stand_time = crate::src::game::ai_main::floattime
                    + crate::src::game::ai_chat::BotChatTime(
                        bs as *mut crate::src::game::ai_main::bot_state_s,
                    );
                AIEnter_Stand(
                    bs,
                    b"battle fight: enemy dead\x00" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
            } else {
                (*bs).ltg_time = 0 as i32 as f32;
                AIEnter_Seek_LTG(
                    bs,
                    b"battle fight: enemy dead\x00" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
            }
            return crate::src::qcommon::q_shared::qfalse as i32;
        }
    } else if crate::src::game::ai_dmq3::EntityIsDead(
        &mut entinfo as *mut _ as *mut crate::be_aas_h::aas_entityinfo_s,
    ) as u64
        != 0
    {
        (*bs).enemydeath_time = crate::src::game::ai_main::floattime
    }
    //if the enemy is invisible and not shooting the bot looses track easily
    if crate::src::game::ai_dmq3::EntityIsInvisible(
        &mut entinfo as *mut _ as *mut crate::be_aas_h::aas_entityinfo_s,
    ) as u32
        != 0
        && crate::src::game::ai_dmq3::EntityIsShooting(
            &mut entinfo as *mut _ as *mut crate::be_aas_h::aas_entityinfo_s,
        ) as u64
            == 0
    {
        if (((::libc::rand() & 0x7fff as i32) as f32 / 0x7fff as i32 as f32) as f64) < 0.2f64 {
            AIEnter_Seek_LTG(
                bs,
                b"battle fight: invisible\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            return crate::src::qcommon::q_shared::qfalse as i32;
        }
    }
    //
    target[0 as i32 as usize] = entinfo.origin[0 as i32 as usize];
    target[1 as i32 as usize] = entinfo.origin[1 as i32 as usize];
    target[2 as i32 as usize] = entinfo.origin[2 as i32 as usize];
    // if not a player enemy
    // ignore non-player enemies
    //update the reachability area and origin if possible
    areanum = crate::src::game::ai_dmq3::BotPointAreaNum(target.as_mut_ptr());
    if areanum != 0 && crate::src::game::g_syscalls::trap_AAS_AreaReachability(areanum) != 0 {
        (*bs).lastenemyorigin[0 as i32 as usize] = target[0 as i32 as usize];
        (*bs).lastenemyorigin[1 as i32 as usize] = target[1 as i32 as usize];
        (*bs).lastenemyorigin[2 as i32 as usize] = target[2 as i32 as usize];
        (*bs).lastenemyareanum = areanum
    }
    //update the attack inventory values
    crate::src::game::ai_dmq3::BotUpdateBattleInventory(
        bs as *mut crate::src::game::ai_main::bot_state_s,
        (*bs).enemy,
    );
    //if the bot's health decreased
    if (*bs).lastframe_health > (*bs).inventory[29 as i32 as usize] {
        if crate::src::game::ai_chat::BotChat_HitNoDeath(
            bs as *mut crate::src::game::ai_main::bot_state_s,
        ) != 0
        {
            (*bs).stand_time = crate::src::game::ai_main::floattime
                + crate::src::game::ai_chat::BotChatTime(
                    bs as *mut crate::src::game::ai_main::bot_state_s,
                );
            AIEnter_Stand(
                bs,
                b"battle fight: chat health decreased\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            return crate::src::qcommon::q_shared::qfalse as i32;
        }
    }
    //if the bot hit someone
    if (*bs).cur_ps.persistant[crate::bg_public_h::PERS_HITS as i32 as usize] > (*bs).lasthitcount {
        if crate::src::game::ai_chat::BotChat_HitNoKill(
            bs as *mut crate::src::game::ai_main::bot_state_s,
        ) != 0
        {
            (*bs).stand_time = crate::src::game::ai_main::floattime
                + crate::src::game::ai_chat::BotChatTime(
                    bs as *mut crate::src::game::ai_main::bot_state_s,
                );
            AIEnter_Stand(
                bs,
                b"battle fight: chat hit someone\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            return crate::src::qcommon::q_shared::qfalse as i32;
        }
    }
    //if the enemy is not visible
    if crate::src::game::ai_dmq3::BotEntityVisible(
        (*bs).entitynum,
        (*bs).eye.as_mut_ptr(),
        (*bs).viewangles.as_mut_ptr(),
        360 as i32 as f32,
        (*bs).enemy,
    ) == 0.
    {
        if crate::src::game::ai_dmq3::BotWantsToChase(
            bs as *mut crate::src::game::ai_main::bot_state_s,
        ) != 0
        {
            AIEnter_Battle_Chase(
                bs,
                b"battle fight: enemy out of sight\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            return crate::src::qcommon::q_shared::qfalse as i32;
        } else {
            AIEnter_Seek_LTG(
                bs,
                b"battle fight: enemy out of sight\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            return crate::src::qcommon::q_shared::qfalse as i32;
        }
    }
    //use holdable items
    crate::src::game::ai_dmq3::BotBattleUseItems(bs as *mut crate::src::game::ai_main::bot_state_s);
    //
    (*bs).tfl = 0x2 as i32
        | 0x4 as i32
        | 0x8 as i32
        | 0x10 as i32
        | 0x20 as i32
        | 0x80 as i32
        | 0x100 as i32
        | 0x200 as i32
        | 0x400 as i32
        | 0x800 as i32
        | 0x80000 as i32
        | 0x100000 as i32
        | 0x40000 as i32
        | 0x1000000 as i32;
    if crate::src::game::ai_dmq3::bot_grapple.integer != 0 {
        (*bs).tfl |= 0x4000 as i32
    }
    //if in lava or slime the bot should be able to get out
    if crate::src::game::ai_dmq3::BotInLavaOrSlime(
        bs as *mut crate::src::game::ai_main::bot_state_s,
    ) as u64
        != 0
    {
        (*bs).tfl |= 0x400000 as i32 | 0x200000 as i32
    }
    //
    if crate::src::game::ai_dmq3::BotCanAndWantsToRocketJump(
        bs as *mut crate::src::game::ai_main::bot_state_s,
    ) != 0
    {
        (*bs).tfl |= 0x1000 as i32
    }
    //choose the best weapon to fight with
    crate::src::game::ai_dmq3::BotChooseWeapon(bs as *mut crate::src::game::ai_main::bot_state_s);
    //do attack movements
    moveresult = crate::src::game::ai_dmq3::BotAttackMove(
        bs as *mut crate::src::game::ai_main::bot_state_s,
        (*bs).tfl,
    ) as crate::be_ai_move_h::bot_moveresult_s;
    //if the movement failed
    if moveresult.failure != 0 {
        //reset the avoid reach, otherwise bot is stuck in current area
        crate::src::game::g_syscalls::trap_BotResetAvoidReach((*bs).ms);
        //BotAI_Print(PRT_MESSAGE, "movement failure %d\n", moveresult.traveltype);
        (*bs).ltg_time = 0 as i32 as f32
    }
    //
    crate::src::game::ai_dmq3::BotAIBlocked(
        bs as *mut crate::src::game::ai_main::bot_state_s,
        &mut moveresult as *mut _ as *mut crate::be_ai_move_h::bot_moveresult_s,
        crate::src::qcommon::q_shared::qfalse as i32,
    );
    //aim at the enemy
    crate::src::game::ai_dmq3::BotAimAtEnemy(bs as *mut crate::src::game::ai_main::bot_state_s);
    //attack the enemy if possible
    crate::src::game::ai_dmq3::BotCheckAttack(bs as *mut crate::src::game::ai_main::bot_state_s);
    //if the bot wants to retreat
    if (*bs).flags & 64 as i32 == 0 {
        if crate::src::game::ai_dmq3::BotWantsToRetreat(
            bs as *mut crate::src::game::ai_main::bot_state_s,
        ) != 0
        {
            AIEnter_Battle_Retreat(
                bs,
                b"battle fight: wants to retreat\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            return crate::src::qcommon::q_shared::qtrue as i32;
        }
    }
    return crate::src::qcommon::q_shared::qtrue as i32;
}
/*
==================
AIEnter_Battle_Chase
==================
*/
#[no_mangle]

pub unsafe extern "C" fn AIEnter_Battle_Chase(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut s: *mut libc::c_char,
) {
    BotRecordNodeSwitch(
        bs,
        b"battle chase\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        s,
    );
    (*bs).chase_time = crate::src::game::ai_main::floattime;
    (*bs).ainode = Some(
        AINode_Battle_Chase
            as unsafe extern "C" fn(_: *mut crate::src::game::ai_main::bot_state_t) -> i32,
    );
}
/*
==================
AINode_Battle_Chase
==================
*/
#[no_mangle]

pub unsafe extern "C" fn AINode_Battle_Chase(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
) -> i32 {
    let mut goal: crate::be_ai_goal_h::bot_goal_t = crate::be_ai_goal_h::bot_goal_t {
        origin: [0.; 3],
        areanum: 0,
        mins: [0.; 3],
        maxs: [0.; 3],
        entitynum: 0,
        number: 0,
        flags: 0,
        iteminfo: 0,
    };
    let mut target: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut dir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut moveresult: crate::be_ai_move_h::bot_moveresult_t =
        crate::be_ai_move_h::bot_moveresult_t {
            failure: 0,
            type_0: 0,
            blocked: 0,
            blockentity: 0,
            traveltype: 0,
            flags: 0,
            weapon: 0,
            movedir: [0.; 3],
            ideal_viewangles: [0.; 3],
        };
    let mut range: f32 = 0.;
    if crate::src::game::ai_dmq3::BotIsObserver(bs as *mut crate::src::game::ai_main::bot_state_s)
        as u64
        != 0
    {
        AIEnter_Observer(
            bs,
            b"battle chase: observer\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    //if in the intermission
    if crate::src::game::ai_dmq3::BotIntermission(bs as *mut crate::src::game::ai_main::bot_state_s)
        as u64
        != 0
    {
        AIEnter_Intermission(
            bs,
            b"battle chase: intermission\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    //respawn if dead
    if crate::src::game::ai_dmq3::BotIsDead(bs as *mut crate::src::game::ai_main::bot_state_s)
        as u64
        != 0
    {
        AIEnter_Respawn(
            bs,
            b"battle chase: bot dead\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    //if no enemy
    if (*bs).enemy < 0 as i32 {
        AIEnter_Seek_LTG(
            bs,
            b"battle chase: no enemy\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    //if the enemy is visible
    if crate::src::game::ai_dmq3::BotEntityVisible(
        (*bs).entitynum,
        (*bs).eye.as_mut_ptr(),
        (*bs).viewangles.as_mut_ptr(),
        360 as i32 as f32,
        (*bs).enemy,
    ) != 0.
    {
        AIEnter_Battle_Fight(
            bs,
            b"battle chase\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    //if there is another enemy
    if crate::src::game::ai_dmq3::BotFindEnemy(
        bs as *mut crate::src::game::ai_main::bot_state_s,
        -(1 as i32),
    ) != 0
    {
        AIEnter_Battle_Fight(
            bs,
            b"battle chase: better enemy\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    //there is no last enemy area
    if (*bs).lastenemyareanum == 0 {
        AIEnter_Seek_LTG(
            bs,
            b"battle chase: no enemy area\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    //
    (*bs).tfl = 0x2 as i32
        | 0x4 as i32
        | 0x8 as i32
        | 0x10 as i32
        | 0x20 as i32
        | 0x80 as i32
        | 0x100 as i32
        | 0x200 as i32
        | 0x400 as i32
        | 0x800 as i32
        | 0x80000 as i32
        | 0x100000 as i32
        | 0x40000 as i32
        | 0x1000000 as i32;
    if crate::src::game::ai_dmq3::bot_grapple.integer != 0 {
        (*bs).tfl |= 0x4000 as i32
    }
    //if in lava or slime the bot should be able to get out
    if crate::src::game::ai_dmq3::BotInLavaOrSlime(
        bs as *mut crate::src::game::ai_main::bot_state_s,
    ) as u64
        != 0
    {
        (*bs).tfl |= 0x400000 as i32 | 0x200000 as i32
    }
    //
    if crate::src::game::ai_dmq3::BotCanAndWantsToRocketJump(
        bs as *mut crate::src::game::ai_main::bot_state_s,
    ) != 0
    {
        (*bs).tfl |= 0x1000 as i32
    }
    //map specific code
    crate::src::game::ai_dmq3::BotMapScripts(bs as *mut crate::src::game::ai_main::bot_state_s);
    //create the chase goal
    goal.entitynum = (*bs).enemy;
    goal.areanum = (*bs).lastenemyareanum;
    goal.origin[0 as i32 as usize] = (*bs).lastenemyorigin[0 as i32 as usize];
    goal.origin[1 as i32 as usize] = (*bs).lastenemyorigin[1 as i32 as usize];
    goal.origin[2 as i32 as usize] = (*bs).lastenemyorigin[2 as i32 as usize];
    goal.mins[0 as i32 as usize] = -(8 as i32) as crate::src::qcommon::q_shared::vec_t;
    goal.mins[1 as i32 as usize] = -(8 as i32) as crate::src::qcommon::q_shared::vec_t;
    goal.mins[2 as i32 as usize] = -(8 as i32) as crate::src::qcommon::q_shared::vec_t;
    goal.maxs[0 as i32 as usize] = 8 as i32 as crate::src::qcommon::q_shared::vec_t;
    goal.maxs[1 as i32 as usize] = 8 as i32 as crate::src::qcommon::q_shared::vec_t;
    goal.maxs[2 as i32 as usize] = 8 as i32 as crate::src::qcommon::q_shared::vec_t;
    //if the last seen enemy spot is reached the enemy could not be found
    if crate::src::game::g_syscalls::trap_BotTouchingGoal(
        (*bs).origin.as_mut_ptr(),
        &mut goal as *mut crate::be_ai_goal_h::bot_goal_t as *mut libc::c_void,
    ) != 0
    {
        (*bs).chase_time = 0 as i32 as f32
    }
    //if there's no chase time left
    if (*bs).chase_time == 0.
        || (*bs).chase_time < crate::src::game::ai_main::floattime - 10 as i32 as f32
    {
        AIEnter_Seek_LTG(
            bs,
            b"battle chase: time out\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    //check for nearby goals periodicly
    if (*bs).check_time < crate::src::game::ai_main::floattime {
        (*bs).check_time = crate::src::game::ai_main::floattime + 1 as i32 as f32;
        range = 150 as i32 as f32;
        //
        if BotNearbyGoal(bs, (*bs).tfl, &mut goal, range) != 0 {
            //the bot gets 5 seconds to pick up the nearby goal item
            (*bs).nbg_time = (crate::src::game::ai_main::floattime as f64
                + 0.1f64 * range as f64
                + 1 as i32 as f64) as f32;
            crate::src::game::g_syscalls::trap_BotResetLastAvoidReach((*bs).ms);
            AIEnter_Battle_NBG(
                bs,
                b"battle chase: nbg\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            return crate::src::qcommon::q_shared::qfalse as i32;
        }
    }
    //
    crate::src::game::ai_dmq3::BotUpdateBattleInventory(
        bs as *mut crate::src::game::ai_main::bot_state_s,
        (*bs).enemy,
    );
    //initialize the movement state
    crate::src::game::ai_dmq3::BotSetupForMovement(
        bs as *mut crate::src::game::ai_main::bot_state_s,
    );
    //move towards the goal
    crate::src::game::g_syscalls::trap_BotMoveToGoal(
        &mut moveresult as *mut crate::be_ai_move_h::bot_moveresult_t as *mut libc::c_void,
        (*bs).ms,
        &mut goal as *mut crate::be_ai_goal_h::bot_goal_t as *mut libc::c_void,
        (*bs).tfl,
    );
    //if the movement failed
    if moveresult.failure != 0 {
        //reset the avoid reach, otherwise bot is stuck in current area
        crate::src::game::g_syscalls::trap_BotResetAvoidReach((*bs).ms);
        //BotAI_Print(PRT_MESSAGE, "movement failure %d\n", moveresult.traveltype);
        (*bs).ltg_time = 0 as i32 as f32
    }
    //
    crate::src::game::ai_dmq3::BotAIBlocked(
        bs as *mut crate::src::game::ai_main::bot_state_s,
        &mut moveresult as *mut _ as *mut crate::be_ai_move_h::bot_moveresult_s,
        crate::src::qcommon::q_shared::qfalse as i32,
    );
    //
    if moveresult.flags & (8 as i32 | 1 as i32 | 2 as i32) != 0 {
        (*bs).ideal_viewangles[0 as i32 as usize] = moveresult.ideal_viewangles[0 as i32 as usize];
        (*bs).ideal_viewangles[1 as i32 as usize] = moveresult.ideal_viewangles[1 as i32 as usize];
        (*bs).ideal_viewangles[2 as i32 as usize] = moveresult.ideal_viewangles[2 as i32 as usize]
    } else if (*bs).flags & 32 as i32 == 0 {
        if (*bs).chase_time > crate::src::game::ai_main::floattime - 2 as i32 as f32 {
            crate::src::game::ai_dmq3::BotAimAtEnemy(
                bs as *mut crate::src::game::ai_main::bot_state_s,
            );
        } else if crate::src::game::g_syscalls::trap_BotMovementViewTarget(
            (*bs).ms,
            &mut goal as *mut crate::be_ai_goal_h::bot_goal_t as *mut libc::c_void,
            (*bs).tfl,
            300 as i32 as f32,
            target.as_mut_ptr(),
        ) != 0
        {
            dir[0 as i32 as usize] = target[0 as i32 as usize] - (*bs).origin[0 as i32 as usize];
            dir[1 as i32 as usize] = target[1 as i32 as usize] - (*bs).origin[1 as i32 as usize];
            dir[2 as i32 as usize] = target[2 as i32 as usize] - (*bs).origin[2 as i32 as usize];
            crate::src::qcommon::q_math::vectoangles(
                dir.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
                (*bs).ideal_viewangles.as_mut_ptr(),
            );
        } else {
            crate::src::qcommon::q_math::vectoangles(
                moveresult.movedir.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
                (*bs).ideal_viewangles.as_mut_ptr(),
            );
        }
        (*bs).ideal_viewangles[2 as i32 as usize] =
            ((*bs).ideal_viewangles[2 as i32 as usize] as f64 * 0.5f64)
                as crate::src::qcommon::q_shared::vec_t
    }
    //if the weapon is used for the bot movement
    if moveresult.flags & 16 as i32 != 0 {
        (*bs).weaponnum = moveresult.weapon
    }
    //if the bot is in the area the enemy was last seen in
    if (*bs).areanum == (*bs).lastenemyareanum {
        (*bs).chase_time = 0 as i32 as f32
    }
    //if the bot wants to retreat (the bot could have been damage during the chase)
    if crate::src::game::ai_dmq3::BotWantsToRetreat(
        bs as *mut crate::src::game::ai_main::bot_state_s,
    ) != 0
    {
        AIEnter_Battle_Retreat(
            bs,
            b"battle chase: wants to retreat\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return crate::src::qcommon::q_shared::qtrue as i32;
    }
    return crate::src::qcommon::q_shared::qtrue as i32;
}
/*
==================
AIEnter_Battle_Retreat
==================
*/
#[no_mangle]

pub unsafe extern "C" fn AIEnter_Battle_Retreat(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut s: *mut libc::c_char,
) {
    BotRecordNodeSwitch(
        bs,
        b"battle retreat\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        s,
    );
    (*bs).ainode = Some(
        AINode_Battle_Retreat
            as unsafe extern "C" fn(_: *mut crate::src::game::ai_main::bot_state_t) -> i32,
    );
}
/*
==================
AINode_Battle_Retreat
==================
*/
#[no_mangle]

pub unsafe extern "C" fn AINode_Battle_Retreat(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
) -> i32 {
    let mut goal: crate::be_ai_goal_h::bot_goal_t = crate::be_ai_goal_h::bot_goal_t {
        origin: [0.; 3],
        areanum: 0,
        mins: [0.; 3],
        maxs: [0.; 3],
        entitynum: 0,
        number: 0,
        flags: 0,
        iteminfo: 0,
    };
    let mut entinfo: crate::be_aas_h::aas_entityinfo_t = crate::be_aas_h::aas_entityinfo_t {
        valid: 0,
        type_0: 0,
        flags: 0,
        ltime: 0.,
        update_time: 0.,
        number: 0,
        origin: [0.; 3],
        angles: [0.; 3],
        old_origin: [0.; 3],
        lastvisorigin: [0.; 3],
        mins: [0.; 3],
        maxs: [0.; 3],
        groundent: 0,
        solid: 0,
        modelindex: 0,
        modelindex2: 0,
        frame: 0,
        event: 0,
        eventParm: 0,
        powerups: 0,
        weapon: 0,
        legsAnim: 0,
        torsoAnim: 0,
    };
    let mut moveresult: crate::be_ai_move_h::bot_moveresult_t =
        crate::be_ai_move_h::bot_moveresult_t {
            failure: 0,
            type_0: 0,
            blocked: 0,
            blockentity: 0,
            traveltype: 0,
            flags: 0,
            weapon: 0,
            movedir: [0.; 3],
            ideal_viewangles: [0.; 3],
        };
    let mut target: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut dir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut attack_skill: f32 = 0.;
    let mut range: f32 = 0.;
    let mut areanum: i32 = 0;
    if crate::src::game::ai_dmq3::BotIsObserver(bs as *mut crate::src::game::ai_main::bot_state_s)
        as u64
        != 0
    {
        AIEnter_Observer(
            bs,
            b"battle retreat: observer\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    //if in the intermission
    if crate::src::game::ai_dmq3::BotIntermission(bs as *mut crate::src::game::ai_main::bot_state_s)
        as u64
        != 0
    {
        AIEnter_Intermission(
            bs,
            b"battle retreat: intermission\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    //respawn if dead
    if crate::src::game::ai_dmq3::BotIsDead(bs as *mut crate::src::game::ai_main::bot_state_s)
        as u64
        != 0
    {
        AIEnter_Respawn(
            bs,
            b"battle retreat: bot dead\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    //if no enemy
    if (*bs).enemy < 0 as i32 {
        AIEnter_Seek_LTG(
            bs,
            b"battle retreat: no enemy\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    //
    crate::src::game::ai_main::BotEntityInfo(
        (*bs).enemy,
        &mut entinfo as *mut _ as *mut crate::be_aas_h::aas_entityinfo_s,
    );
    if crate::src::game::ai_dmq3::EntityIsDead(
        &mut entinfo as *mut _ as *mut crate::be_aas_h::aas_entityinfo_s,
    ) as u64
        != 0
    {
        AIEnter_Seek_LTG(
            bs,
            b"battle retreat: enemy dead\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    //if there is another better enemy
    crate::src::game::ai_dmq3::BotFindEnemy(
        bs as *mut crate::src::game::ai_main::bot_state_s,
        (*bs).enemy,
    );
    //
    (*bs).tfl = 0x2 as i32
        | 0x4 as i32
        | 0x8 as i32
        | 0x10 as i32
        | 0x20 as i32
        | 0x80 as i32
        | 0x100 as i32
        | 0x200 as i32
        | 0x400 as i32
        | 0x800 as i32
        | 0x80000 as i32
        | 0x100000 as i32
        | 0x40000 as i32
        | 0x1000000 as i32;
    if crate::src::game::ai_dmq3::bot_grapple.integer != 0 {
        (*bs).tfl |= 0x4000 as i32
    }
    //if in lava or slime the bot should be able to get out
    if crate::src::game::ai_dmq3::BotInLavaOrSlime(
        bs as *mut crate::src::game::ai_main::bot_state_s,
    ) as u64
        != 0
    {
        (*bs).tfl |= 0x400000 as i32 | 0x200000 as i32
    }
    //map specific code
    crate::src::game::ai_dmq3::BotMapScripts(bs as *mut crate::src::game::ai_main::bot_state_s);
    //update the attack inventory values
    crate::src::game::ai_dmq3::BotUpdateBattleInventory(
        bs as *mut crate::src::game::ai_main::bot_state_s,
        (*bs).enemy,
    );
    //if the bot doesn't want to retreat anymore... probably picked up some nice items
    if crate::src::game::ai_dmq3::BotWantsToChase(bs as *mut crate::src::game::ai_main::bot_state_s)
        != 0
    {
        //empty the goal stack, when chasing, only the enemy is the goal
        crate::src::game::g_syscalls::trap_BotEmptyGoalStack((*bs).gs);
        //go chase the enemy
        AIEnter_Battle_Chase(
            bs,
            b"battle retreat: wants to chase\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    //update the last time the enemy was visible
    if crate::src::game::ai_dmq3::BotEntityVisible(
        (*bs).entitynum,
        (*bs).eye.as_mut_ptr(),
        (*bs).viewangles.as_mut_ptr(),
        360 as i32 as f32,
        (*bs).enemy,
    ) != 0.
    {
        (*bs).enemyvisible_time = crate::src::game::ai_main::floattime;
        target[0 as i32 as usize] = entinfo.origin[0 as i32 as usize];
        target[1 as i32 as usize] = entinfo.origin[1 as i32 as usize];
        target[2 as i32 as usize] = entinfo.origin[2 as i32 as usize];
        // if not a player enemy
        // ignore non-player enemies
        //update the reachability area and origin if possible
        areanum = crate::src::game::ai_dmq3::BotPointAreaNum(target.as_mut_ptr());
        if areanum != 0 && crate::src::game::g_syscalls::trap_AAS_AreaReachability(areanum) != 0 {
            (*bs).lastenemyorigin[0 as i32 as usize] = target[0 as i32 as usize];
            (*bs).lastenemyorigin[1 as i32 as usize] = target[1 as i32 as usize];
            (*bs).lastenemyorigin[2 as i32 as usize] = target[2 as i32 as usize];
            (*bs).lastenemyareanum = areanum
        }
    }
    //if the enemy is NOT visible for 4 seconds
    if (*bs).enemyvisible_time < crate::src::game::ai_main::floattime - 4 as i32 as f32 {
        AIEnter_Seek_LTG(
            bs,
            b"battle retreat: lost enemy\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return crate::src::qcommon::q_shared::qfalse as i32;
    } else {
        //else if the enemy is NOT visible
        if (*bs).enemyvisible_time < crate::src::game::ai_main::floattime {
            //if there is another enemy
            if crate::src::game::ai_dmq3::BotFindEnemy(
                bs as *mut crate::src::game::ai_main::bot_state_s,
                -(1 as i32),
            ) != 0
            {
                AIEnter_Battle_Fight(
                    bs,
                    b"battle retreat: another enemy\x00" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
                return crate::src::qcommon::q_shared::qfalse as i32;
            }
        }
    }
    //
    crate::src::game::ai_dmq3::BotTeamGoals(
        bs as *mut crate::src::game::ai_main::bot_state_s,
        crate::src::qcommon::q_shared::qtrue as i32,
    );
    //use holdable items
    crate::src::game::ai_dmq3::BotBattleUseItems(bs as *mut crate::src::game::ai_main::bot_state_s);
    //get the current long term goal while retreating
    if BotLongTermGoal(
        bs,
        (*bs).tfl,
        crate::src::qcommon::q_shared::qtrue as i32,
        &mut goal,
    ) == 0
    {
        AIEnter_Battle_SuicidalFight(
            bs,
            b"battle retreat: no way out\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    //check for nearby goals periodicly
    if (*bs).check_time < crate::src::game::ai_main::floattime {
        (*bs).check_time = crate::src::game::ai_main::floattime + 1 as i32 as f32;
        range = 150 as i32 as f32;
        if crate::src::game::ai_dmq3::gametype == crate::bg_public_h::GT_CTF as i32 {
            //if carrying a flag the bot shouldn't be distracted too much
            if crate::src::game::ai_dmq3::BotCTFCarryingFlag(
                bs as *mut crate::src::game::ai_main::bot_state_s,
            ) != 0
            {
                range = 50 as i32 as f32
            }
        }
        //CTF
        //
        if BotNearbyGoal(bs, (*bs).tfl, &mut goal, range) != 0 {
            crate::src::game::g_syscalls::trap_BotResetLastAvoidReach((*bs).ms);
            //time the bot gets to pick up the nearby goal item
            (*bs).nbg_time =
                crate::src::game::ai_main::floattime + range / 100 as i32 as f32 + 1 as i32 as f32;
            AIEnter_Battle_NBG(
                bs,
                b"battle retreat: nbg\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            return crate::src::qcommon::q_shared::qfalse as i32;
        }
    }
    //initialize the movement state
    crate::src::game::ai_dmq3::BotSetupForMovement(
        bs as *mut crate::src::game::ai_main::bot_state_s,
    );
    //move towards the goal
    crate::src::game::g_syscalls::trap_BotMoveToGoal(
        &mut moveresult as *mut crate::be_ai_move_h::bot_moveresult_t as *mut libc::c_void,
        (*bs).ms,
        &mut goal as *mut crate::be_ai_goal_h::bot_goal_t as *mut libc::c_void,
        (*bs).tfl,
    );
    //if the movement failed
    if moveresult.failure != 0 {
        //reset the avoid reach, otherwise bot is stuck in current area
        crate::src::game::g_syscalls::trap_BotResetAvoidReach((*bs).ms);
        //BotAI_Print(PRT_MESSAGE, "movement failure %d\n", moveresult.traveltype);
        (*bs).ltg_time = 0 as i32 as f32
    }
    //
    crate::src::game::ai_dmq3::BotAIBlocked(
        bs as *mut crate::src::game::ai_main::bot_state_s,
        &mut moveresult as *mut _ as *mut crate::be_ai_move_h::bot_moveresult_s,
        crate::src::qcommon::q_shared::qfalse as i32,
    );
    //choose the best weapon to fight with
    crate::src::game::ai_dmq3::BotChooseWeapon(bs as *mut crate::src::game::ai_main::bot_state_s);
    //if the view is fixed for the movement
    if moveresult.flags & (1 as i32 | 2 as i32) != 0 {
        (*bs).ideal_viewangles[0 as i32 as usize] = moveresult.ideal_viewangles[0 as i32 as usize];
        (*bs).ideal_viewangles[1 as i32 as usize] = moveresult.ideal_viewangles[1 as i32 as usize];
        (*bs).ideal_viewangles[2 as i32 as usize] = moveresult.ideal_viewangles[2 as i32 as usize]
    } else if moveresult.flags & 8 as i32 == 0 && (*bs).flags & 32 as i32 == 0 {
        attack_skill = crate::src::game::g_syscalls::trap_Characteristic_BFloat(
            (*bs).character,
            2 as i32,
            0 as i32 as f32,
            1 as i32 as f32,
        );
        //if the bot is skilled enough
        if attack_skill as f64 > 0.3f64 {
            crate::src::game::ai_dmq3::BotAimAtEnemy(
                bs as *mut crate::src::game::ai_main::bot_state_s,
            );
        } else {
            if crate::src::game::g_syscalls::trap_BotMovementViewTarget(
                (*bs).ms,
                &mut goal as *mut crate::be_ai_goal_h::bot_goal_t as *mut libc::c_void,
                (*bs).tfl,
                300 as i32 as f32,
                target.as_mut_ptr(),
            ) != 0
            {
                dir[0 as i32 as usize] =
                    target[0 as i32 as usize] - (*bs).origin[0 as i32 as usize];
                dir[1 as i32 as usize] =
                    target[1 as i32 as usize] - (*bs).origin[1 as i32 as usize];
                dir[2 as i32 as usize] =
                    target[2 as i32 as usize] - (*bs).origin[2 as i32 as usize];
                crate::src::qcommon::q_math::vectoangles(
                    dir.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
                    (*bs).ideal_viewangles.as_mut_ptr(),
                );
            } else {
                crate::src::qcommon::q_math::vectoangles(
                    moveresult.movedir.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
                    (*bs).ideal_viewangles.as_mut_ptr(),
                );
            }
            (*bs).ideal_viewangles[2 as i32 as usize] =
                ((*bs).ideal_viewangles[2 as i32 as usize] as f64 * 0.5f64)
                    as crate::src::qcommon::q_shared::vec_t
        }
    }
    //if the weapon is used for the bot movement
    if moveresult.flags & 16 as i32 != 0 {
        (*bs).weaponnum = moveresult.weapon
    }
    //attack the enemy if possible
    crate::src::game::ai_dmq3::BotCheckAttack(bs as *mut crate::src::game::ai_main::bot_state_s);
    //
    return crate::src::qcommon::q_shared::qtrue as i32;
}
/*
==================
AIEnter_Battle_NBG
==================
*/
#[no_mangle]

pub unsafe extern "C" fn AIEnter_Battle_NBG(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut s: *mut libc::c_char,
) {
    BotRecordNodeSwitch(
        bs,
        b"battle NBG\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        s,
    );
    (*bs).ainode = Some(
        AINode_Battle_NBG
            as unsafe extern "C" fn(_: *mut crate::src::game::ai_main::bot_state_t) -> i32,
    );
}
/*
==================
AINode_Battle_NBG
==================
*/
#[no_mangle]

pub unsafe extern "C" fn AINode_Battle_NBG(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
) -> i32 {
    let mut areanum: i32 = 0;
    let mut goal: crate::be_ai_goal_h::bot_goal_t = crate::be_ai_goal_h::bot_goal_t {
        origin: [0.; 3],
        areanum: 0,
        mins: [0.; 3],
        maxs: [0.; 3],
        entitynum: 0,
        number: 0,
        flags: 0,
        iteminfo: 0,
    };
    let mut entinfo: crate::be_aas_h::aas_entityinfo_t = crate::be_aas_h::aas_entityinfo_t {
        valid: 0,
        type_0: 0,
        flags: 0,
        ltime: 0.,
        update_time: 0.,
        number: 0,
        origin: [0.; 3],
        angles: [0.; 3],
        old_origin: [0.; 3],
        lastvisorigin: [0.; 3],
        mins: [0.; 3],
        maxs: [0.; 3],
        groundent: 0,
        solid: 0,
        modelindex: 0,
        modelindex2: 0,
        frame: 0,
        event: 0,
        eventParm: 0,
        powerups: 0,
        weapon: 0,
        legsAnim: 0,
        torsoAnim: 0,
    };
    let mut moveresult: crate::be_ai_move_h::bot_moveresult_t =
        crate::be_ai_move_h::bot_moveresult_t {
            failure: 0,
            type_0: 0,
            blocked: 0,
            blockentity: 0,
            traveltype: 0,
            flags: 0,
            weapon: 0,
            movedir: [0.; 3],
            ideal_viewangles: [0.; 3],
        };
    let mut attack_skill: f32 = 0.;
    let mut target: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut dir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    if crate::src::game::ai_dmq3::BotIsObserver(bs as *mut crate::src::game::ai_main::bot_state_s)
        as u64
        != 0
    {
        AIEnter_Observer(
            bs,
            b"battle nbg: observer\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    //if in the intermission
    if crate::src::game::ai_dmq3::BotIntermission(bs as *mut crate::src::game::ai_main::bot_state_s)
        as u64
        != 0
    {
        AIEnter_Intermission(
            bs,
            b"battle nbg: intermission\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    //respawn if dead
    if crate::src::game::ai_dmq3::BotIsDead(bs as *mut crate::src::game::ai_main::bot_state_s)
        as u64
        != 0
    {
        AIEnter_Respawn(
            bs,
            b"battle nbg: bot dead\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    //if no enemy
    if (*bs).enemy < 0 as i32 {
        AIEnter_Seek_NBG(
            bs,
            b"battle nbg: no enemy\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    //
    crate::src::game::ai_main::BotEntityInfo(
        (*bs).enemy,
        &mut entinfo as *mut _ as *mut crate::be_aas_h::aas_entityinfo_s,
    );
    if crate::src::game::ai_dmq3::EntityIsDead(
        &mut entinfo as *mut _ as *mut crate::be_aas_h::aas_entityinfo_s,
    ) as u64
        != 0
    {
        AIEnter_Seek_NBG(
            bs,
            b"battle nbg: enemy dead\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    //
    (*bs).tfl = 0x2 as i32
        | 0x4 as i32
        | 0x8 as i32
        | 0x10 as i32
        | 0x20 as i32
        | 0x80 as i32
        | 0x100 as i32
        | 0x200 as i32
        | 0x400 as i32
        | 0x800 as i32
        | 0x80000 as i32
        | 0x100000 as i32
        | 0x40000 as i32
        | 0x1000000 as i32;
    if crate::src::game::ai_dmq3::bot_grapple.integer != 0 {
        (*bs).tfl |= 0x4000 as i32
    }
    //if in lava or slime the bot should be able to get out
    if crate::src::game::ai_dmq3::BotInLavaOrSlime(
        bs as *mut crate::src::game::ai_main::bot_state_s,
    ) as u64
        != 0
    {
        (*bs).tfl |= 0x400000 as i32 | 0x200000 as i32
    }
    //
    if crate::src::game::ai_dmq3::BotCanAndWantsToRocketJump(
        bs as *mut crate::src::game::ai_main::bot_state_s,
    ) != 0
    {
        (*bs).tfl |= 0x1000 as i32
    }
    //map specific code
    crate::src::game::ai_dmq3::BotMapScripts(bs as *mut crate::src::game::ai_main::bot_state_s);
    //update the last time the enemy was visible
    if crate::src::game::ai_dmq3::BotEntityVisible(
        (*bs).entitynum,
        (*bs).eye.as_mut_ptr(),
        (*bs).viewangles.as_mut_ptr(),
        360 as i32 as f32,
        (*bs).enemy,
    ) != 0.
    {
        (*bs).enemyvisible_time = crate::src::game::ai_main::floattime;
        target[0 as i32 as usize] = entinfo.origin[0 as i32 as usize];
        target[1 as i32 as usize] = entinfo.origin[1 as i32 as usize];
        target[2 as i32 as usize] = entinfo.origin[2 as i32 as usize];
        // if not a player enemy
        // ignore non-player enemies
        //update the reachability area and origin if possible
        areanum = crate::src::game::ai_dmq3::BotPointAreaNum(target.as_mut_ptr());
        if areanum != 0 && crate::src::game::g_syscalls::trap_AAS_AreaReachability(areanum) != 0 {
            (*bs).lastenemyorigin[0 as i32 as usize] = target[0 as i32 as usize];
            (*bs).lastenemyorigin[1 as i32 as usize] = target[1 as i32 as usize];
            (*bs).lastenemyorigin[2 as i32 as usize] = target[2 as i32 as usize];
            (*bs).lastenemyareanum = areanum
        }
    }
    //if the bot has no goal or touches the current goal
    if crate::src::game::g_syscalls::trap_BotGetTopGoal(
        (*bs).gs,
        &mut goal as *mut crate::be_ai_goal_h::bot_goal_t as *mut libc::c_void,
    ) == 0
    {
        (*bs).nbg_time = 0 as i32 as f32
    } else if BotReachedGoal(bs, &mut goal) != 0 {
        (*bs).nbg_time = 0 as i32 as f32
    }
    //
    if (*bs).nbg_time < crate::src::game::ai_main::floattime {
        //pop the current goal from the stack
        crate::src::game::g_syscalls::trap_BotPopGoal((*bs).gs);
        //if the bot still has a goal
        if crate::src::game::g_syscalls::trap_BotGetTopGoal(
            (*bs).gs,
            &mut goal as *mut crate::be_ai_goal_h::bot_goal_t as *mut libc::c_void,
        ) != 0
        {
            AIEnter_Battle_Retreat(
                bs,
                b"battle nbg: time out\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        } else {
            AIEnter_Battle_Fight(
                bs,
                b"battle nbg: time out\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
        //
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    //initialize the movement state
    crate::src::game::ai_dmq3::BotSetupForMovement(
        bs as *mut crate::src::game::ai_main::bot_state_s,
    );
    //move towards the goal
    crate::src::game::g_syscalls::trap_BotMoveToGoal(
        &mut moveresult as *mut crate::be_ai_move_h::bot_moveresult_t as *mut libc::c_void,
        (*bs).ms,
        &mut goal as *mut crate::be_ai_goal_h::bot_goal_t as *mut libc::c_void,
        (*bs).tfl,
    );
    //if the movement failed
    if moveresult.failure != 0 {
        //reset the avoid reach, otherwise bot is stuck in current area
        crate::src::game::g_syscalls::trap_BotResetAvoidReach((*bs).ms);
        //BotAI_Print(PRT_MESSAGE, "movement failure %d\n", moveresult.traveltype);
        (*bs).nbg_time = 0 as i32 as f32
    }
    //
    crate::src::game::ai_dmq3::BotAIBlocked(
        bs as *mut crate::src::game::ai_main::bot_state_s,
        &mut moveresult as *mut _ as *mut crate::be_ai_move_h::bot_moveresult_s,
        crate::src::qcommon::q_shared::qfalse as i32,
    );
    //update the attack inventory values
    crate::src::game::ai_dmq3::BotUpdateBattleInventory(
        bs as *mut crate::src::game::ai_main::bot_state_s,
        (*bs).enemy,
    );
    //choose the best weapon to fight with
    crate::src::game::ai_dmq3::BotChooseWeapon(bs as *mut crate::src::game::ai_main::bot_state_s);
    //if the view is fixed for the movement
    if moveresult.flags & (1 as i32 | 2 as i32) != 0 {
        (*bs).ideal_viewangles[0 as i32 as usize] = moveresult.ideal_viewangles[0 as i32 as usize];
        (*bs).ideal_viewangles[1 as i32 as usize] = moveresult.ideal_viewangles[1 as i32 as usize];
        (*bs).ideal_viewangles[2 as i32 as usize] = moveresult.ideal_viewangles[2 as i32 as usize]
    } else if moveresult.flags & 8 as i32 == 0 && (*bs).flags & 32 as i32 == 0 {
        attack_skill = crate::src::game::g_syscalls::trap_Characteristic_BFloat(
            (*bs).character,
            2 as i32,
            0 as i32 as f32,
            1 as i32 as f32,
        );
        //if the bot is skilled enough and the enemy is visible
        if attack_skill as f64 > 0.3f64 {
            //&& BotEntityVisible(bs->entitynum, bs->eye, bs->viewangles, 360, bs->enemy)
            crate::src::game::ai_dmq3::BotAimAtEnemy(
                bs as *mut crate::src::game::ai_main::bot_state_s,
            );
        } else {
            if crate::src::game::g_syscalls::trap_BotMovementViewTarget(
                (*bs).ms,
                &mut goal as *mut crate::be_ai_goal_h::bot_goal_t as *mut libc::c_void,
                (*bs).tfl,
                300 as i32 as f32,
                target.as_mut_ptr(),
            ) != 0
            {
                dir[0 as i32 as usize] =
                    target[0 as i32 as usize] - (*bs).origin[0 as i32 as usize];
                dir[1 as i32 as usize] =
                    target[1 as i32 as usize] - (*bs).origin[1 as i32 as usize];
                dir[2 as i32 as usize] =
                    target[2 as i32 as usize] - (*bs).origin[2 as i32 as usize];
                crate::src::qcommon::q_math::vectoangles(
                    dir.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
                    (*bs).ideal_viewangles.as_mut_ptr(),
                );
            } else {
                crate::src::qcommon::q_math::vectoangles(
                    moveresult.movedir.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
                    (*bs).ideal_viewangles.as_mut_ptr(),
                );
            }
            (*bs).ideal_viewangles[2 as i32 as usize] =
                ((*bs).ideal_viewangles[2 as i32 as usize] as f64 * 0.5f64)
                    as crate::src::qcommon::q_shared::vec_t
        }
    }
    //if the weapon is used for the bot movement
    if moveresult.flags & 16 as i32 != 0 {
        (*bs).weaponnum = moveresult.weapon
    }
    //attack the enemy if possible
    crate::src::game::ai_dmq3::BotCheckAttack(bs as *mut crate::src::game::ai_main::bot_state_s);
    //
    return crate::src::qcommon::q_shared::qtrue as i32;
}

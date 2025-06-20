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

    // __Q_SHARED_H
}

pub mod stdlib_float_h {
    #[inline]

    pub unsafe extern "C" fn atof(mut __nptr: *const libc::c_char) -> f64 {
        return libc::strtod(__nptr, std::ptr::null_mut() as *mut *mut libc::c_char);
    }
}

pub use crate::bg_public_h::gitem_s;
pub use crate::bg_public_h::gitem_t;
pub use crate::bg_public_h::itemType_t;
pub use crate::bg_public_h::team_t;
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
pub use crate::bg_public_h::IT_AMMO;
pub use crate::bg_public_h::IT_ARMOR;
pub use crate::bg_public_h::IT_BAD;
pub use crate::bg_public_h::IT_HEALTH;
pub use crate::bg_public_h::IT_HOLDABLE;
pub use crate::bg_public_h::IT_PERSISTANT_POWERUP;
pub use crate::bg_public_h::IT_POWERUP;
pub use crate::bg_public_h::IT_TEAM;
pub use crate::bg_public_h::IT_WEAPON;
pub use crate::bg_public_h::TEAM_BLUE;
pub use crate::bg_public_h::TEAM_FREE;
pub use crate::bg_public_h::TEAM_NUM_TEAMS;
pub use crate::bg_public_h::TEAM_RED;
pub use crate::bg_public_h::TEAM_SPECTATOR;
pub use crate::g_public_h::entityShared_t;
pub use crate::src::game::ai_cmd::q_shared_h::VectorLength;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::cplane_s;
pub use crate::src::qcommon::q_shared::cplane_t;
pub use crate::src::qcommon::q_shared::entityState_s;
pub use crate::src::qcommon::q_shared::entityState_t;
pub use crate::src::qcommon::q_shared::fileHandle_t;
pub use crate::src::qcommon::q_shared::playerState_s;
pub use crate::src::qcommon::q_shared::playerState_t;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::trType_t;
pub use crate::src::qcommon::q_shared::trace_t;
pub use crate::src::qcommon::q_shared::trajectory_t;
pub use crate::src::qcommon::q_shared::usercmd_s;
pub use crate::src::qcommon::q_shared::usercmd_t;
pub use crate::src::qcommon::q_shared::vec3_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::qcommon::q_shared::Com_sprintf;
pub use crate::src::qcommon::q_shared::Q_stricmp;
pub use crate::src::qcommon::q_shared::Q_strncpyz;
pub use crate::src::qcommon::q_shared::TR_GRAVITY;
pub use crate::src::qcommon::q_shared::TR_INTERPOLATE;
pub use crate::src::qcommon::q_shared::TR_LINEAR;
pub use crate::src::qcommon::q_shared::TR_LINEAR_STOP;
pub use crate::src::qcommon::q_shared::TR_SINE;
pub use crate::src::qcommon::q_shared::TR_STATIONARY;

pub use crate::be_aas_h::aas_entityinfo_s;
pub use crate::be_aas_h::aas_entityinfo_t;
pub use crate::be_ai_chat_h::bot_match_s;
pub use crate::be_ai_chat_h::bot_match_t;
pub use crate::be_ai_chat_h::bot_matchvariable_s;
pub use crate::be_ai_chat_h::bot_matchvariable_t;
pub use crate::be_ai_goal_h::bot_goal_s;
pub use crate::be_ai_goal_h::bot_goal_t;
pub use crate::botlib_h::bsp_surface_s;
pub use crate::botlib_h::bsp_surface_t;
pub use crate::botlib_h::bsp_trace_s;
pub use crate::botlib_h::bsp_trace_t;
pub use crate::g_local_h::bot_settings_s;
pub use crate::g_local_h::bot_settings_t;
pub use crate::g_local_h::clientConnected_t;
pub use crate::g_local_h::clientPersistant_t;
pub use crate::g_local_h::clientSession_t;
pub use crate::g_local_h::gclient_s;
pub use crate::g_local_h::gentity_s;
pub use crate::g_local_h::gentity_t;
pub use crate::g_local_h::level_locals_t;
pub use crate::g_local_h::moverState_t;
pub use crate::g_local_h::playerTeamStateState_t;
pub use crate::g_local_h::playerTeamState_t;
pub use crate::g_local_h::spectatorState_t;
pub use crate::g_local_h::CON_CONNECTED;
pub use crate::g_local_h::CON_CONNECTING;
pub use crate::g_local_h::CON_DISCONNECTED;
pub use crate::g_local_h::MOVER_1TO2;
pub use crate::g_local_h::MOVER_2TO1;
pub use crate::g_local_h::MOVER_POS1;
pub use crate::g_local_h::MOVER_POS2;
pub use crate::g_local_h::SPECTATOR_FOLLOW;
pub use crate::g_local_h::SPECTATOR_FREE;
pub use crate::g_local_h::SPECTATOR_NOT;
pub use crate::g_local_h::SPECTATOR_SCOREBOARD;
pub use crate::g_local_h::TEAM_ACTIVE;
pub use crate::g_local_h::TEAM_BEGIN;
pub use crate::src::game::ai_cmd::stdlib_float_h::atof;
pub use crate::src::game::ai_main::bot_activategoal_s;
pub use crate::src::game::ai_main::bot_activategoal_t;
pub use crate::src::game::ai_main::bot_state_s;
pub use crate::src::game::ai_main::bot_state_t;
pub use crate::src::game::ai_main::bot_waypoint_s;
pub use crate::src::game::ai_main::bot_waypoint_t;
pub use crate::src::game::ai_main::floattime;
pub use crate::src::game::ai_main::BotAI_BotInitialChat;
pub use crate::src::game::ai_main::BotAI_Print;
pub use crate::src::game::ai_main::BotAI_Trace;
pub use crate::src::game::ai_main::BotEntityInfo;
pub use crate::src::game::g_main::level;
pub use crate::src::game::g_syscalls::trap_AAS_AreaTravelTimeToGoalArea;
pub use crate::src::game::g_syscalls::trap_BotEnterChat;
pub use crate::src::game::g_syscalls::trap_BotFindMatch;
pub use crate::src::game::g_syscalls::trap_BotGetLevelItemGoal;
pub use crate::src::game::g_syscalls::trap_BotGoalName;
pub use crate::src::game::g_syscalls::trap_BotMatchVariable;
pub use crate::src::game::g_syscalls::trap_EA_Action;
pub use crate::src::game::g_syscalls::trap_EA_Command;
pub use crate::src::game::g_syscalls::trap_EA_SayTeam;
pub use crate::src::game::g_syscalls::trap_GetConfigstring;

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
//
/* ****************************************************************************
 * name:		ai_cmd.c
 *
 * desc:		Quake3 bot AI
 *
 * $Archive: /MissionPack/code/game/ai_cmd.c $
 *
 *****************************************************************************/
//
//
// for the voice chats
#[no_mangle]

pub static mut notleader: [i32; 64] = [0; 64];
//DEBUG
/*
==================
BotGetItemTeamGoal

FIXME: add stuff like "upper rocket launcher"
"the rl near the railgun", "lower grenade launcher" etc.
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotGetItemTeamGoal(
    mut goalname: *mut libc::c_char,
    mut goal: *mut bot_goal_t,
) -> i32 {
    let mut i: i32 = 0;
    if crate::stdlib::strlen(goalname) == 0 {
        return qfalse as i32;
    }
    i = -(1 as i32);
    loop {
        i = trap_BotGetLevelItemGoal(i, goalname, goal as *mut libc::c_void);
        if i > 0 as i32 {
            //do NOT defend dropped items
            if !((*goal).flags & 4 as i32 != 0) {
                return qtrue as i32;
            }
        }
        if !(i > 0 as i32) {
            break;
        }
    }
    return qfalse as i32;
}
/*
==================
BotGetMessageTeamGoal
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotGetMessageTeamGoal(
    mut bs: *mut bot_state_t,
    mut goalname: *mut libc::c_char,
    mut goal: *mut bot_goal_t,
) -> i32 {
    let mut cp: *mut bot_waypoint_t = std::ptr::null_mut();
    if BotGetItemTeamGoal(goalname, goal) != 0 {
        return qtrue as i32;
    }
    cp = crate::src::game::ai_dmq3::BotFindWayPoint(
        (*bs).checkpoints as *mut bot_waypoint_s,
        goalname,
    ) as *mut bot_waypoint_s;
    if !cp.is_null() {
        crate::stdlib::memcpy(
            goal as *mut libc::c_void,
            &mut (*cp).goal as *mut bot_goal_t as *const libc::c_void,
            ::std::mem::size_of::<bot_goal_t>() as usize,
        );
        return qtrue as i32;
    }
    return qfalse as i32;
}
/*
==================
BotGetTime
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotGetTime(mut match_0: *mut bot_match_t) -> f32 {
    let mut timematch: bot_match_t = bot_match_t {
        string: [0; 256],
        type_0: 0,
        subtype: 0,
        variables: [bot_matchvariable_t {
            offset: 0,
            length: 0,
        }; 8],
    };
    let mut timestring: [libc::c_char; 256] = [0; 256];
    let mut t: f32 = 0.;
    //if the matched string has a time
    if (*match_0).subtype & 16 as i32 != 0 {
        //get the time string
        trap_BotMatchVariable(
            match_0 as *mut libc::c_void,
            6 as i32,
            timestring.as_mut_ptr(),
            256 as i32,
        );
        //match it to find out if the time is in seconds or minutes
        if trap_BotFindMatch(
            timestring.as_mut_ptr(),
            &mut timematch as *mut bot_match_t as *mut libc::c_void,
            8 as i32 as usize,
        ) != 0
        {
            if timematch.type_0 == 107 as i32 {
                t = 99999999.0f32
            } else if timematch.type_0 == 109 as i32 {
                t = (10 as i32 * 60 as i32) as f32
            // 10 minutes
            } else if timematch.type_0 == 108 as i32 {
                t = (30 as i32 * 60 as i32) as f32
            // 30 minutes
            } else {
                trap_BotMatchVariable(
                    &mut timematch as *mut bot_match_t as *mut libc::c_void,
                    6 as i32,
                    timestring.as_mut_ptr(),
                    256 as i32,
                );
                if timematch.type_0 == 105 as i32 {
                    t = (atof(timestring.as_mut_ptr()) * 60 as i32 as f64) as f32
                } else if timematch.type_0 == 106 as i32 {
                    t = atof(timestring.as_mut_ptr()) as f32
                } else {
                    t = 0 as i32 as f32
                }
            }
            //if there's a valid time
            if t > 0 as i32 as f32 {
                return floattime + t;
            }
        }
    }
    return 0 as i32 as f32;
}
/*
==================
FindClientByName
==================
*/
#[no_mangle]

pub unsafe extern "C" fn FindClientByName(mut name: *mut libc::c_char) -> i32 {
    let mut i: i32 = 0;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    i = 0 as i32;
    while i < level.maxclients {
        crate::src::game::ai_dmq3::ClientName(
            i,
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as usize as i32,
        );
        if Q_stricmp(buf.as_mut_ptr(), name) == 0 {
            return i;
        }
        i += 1
    }
    i = 0 as i32;
    while i < level.maxclients {
        crate::src::game::ai_dmq3::ClientName(
            i,
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as usize as i32,
        );
        if !crate::src::game::ai_dmq3::stristr(buf.as_mut_ptr(), name).is_null() {
            return i;
        }
        i += 1
    }
    return -(1 as i32);
}
/*
==================
FindEnemyByName
==================
*/
#[no_mangle]

pub unsafe extern "C" fn FindEnemyByName(
    mut bs: *mut bot_state_t,
    mut name: *mut libc::c_char,
) -> i32 {
    let mut i: i32 = 0;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    i = 0 as i32;
    while i < level.maxclients {
        if !(crate::src::game::ai_dmq3::BotSameTeam(bs as *mut bot_state_s, i) != 0) {
            crate::src::game::ai_dmq3::ClientName(
                i,
                buf.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 1024]>() as usize as i32,
            );
            if Q_stricmp(buf.as_mut_ptr(), name) == 0 {
                return i;
            }
        }
        i += 1
    }
    i = 0 as i32;
    while i < level.maxclients {
        if !(crate::src::game::ai_dmq3::BotSameTeam(bs as *mut bot_state_s, i) != 0) {
            crate::src::game::ai_dmq3::ClientName(
                i,
                buf.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 1024]>() as usize as i32,
            );
            if !crate::src::game::ai_dmq3::stristr(buf.as_mut_ptr(), name).is_null() {
                return i;
            }
        }
        i += 1
    }
    return -(1 as i32);
}
/*
==================
NumPlayersOnSameTeam
==================
*/
#[no_mangle]

pub unsafe extern "C" fn NumPlayersOnSameTeam(mut bs: *mut bot_state_t) -> i32 {
    let mut i: i32 = 0;
    let mut num: i32 = 0;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    num = 0 as i32;
    i = 0 as i32;
    while i < level.maxclients {
        trap_GetConfigstring(
            32 as i32 + 256 as i32 + 256 as i32 + i,
            buf.as_mut_ptr(),
            1024 as i32,
        );
        if crate::stdlib::strlen(buf.as_mut_ptr()) != 0 {
            if crate::src::game::ai_dmq3::BotSameTeam(bs as *mut bot_state_s, i + 1 as i32) != 0 {
                num += 1
            }
        }
        i += 1
    }
    return num;
}
/*
==================
TeamPlayIsOn
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotGetPatrolWaypoints(
    mut bs: *mut bot_state_t,
    mut match_0: *mut bot_match_t,
) -> i32 {
    let mut keyarea: [libc::c_char; 256] = [0; 256];
    let mut patrolflags: i32 = 0;
    let mut wp: *mut bot_waypoint_t = std::ptr::null_mut();
    let mut newwp: *mut bot_waypoint_t = std::ptr::null_mut();
    let mut newpatrolpoints: *mut bot_waypoint_t = std::ptr::null_mut();
    let mut keyareamatch: bot_match_t = bot_match_t {
        string: [0; 256],
        type_0: 0,
        subtype: 0,
        variables: [bot_matchvariable_t {
            offset: 0,
            length: 0,
        }; 8],
    };
    let mut goal: bot_goal_t = bot_goal_t {
        origin: [0.; 3],
        areanum: 0,
        mins: [0.; 3],
        maxs: [0.; 3],
        entitynum: 0,
        number: 0,
        flags: 0,
        iteminfo: 0,
    };
    newpatrolpoints = std::ptr::null_mut();
    patrolflags = 0 as i32;
    //
    trap_BotMatchVariable(
        match_0 as *mut libc::c_void,
        5 as i32,
        keyarea.as_mut_ptr(),
        256 as i32,
    );
    loop
    //
    {
        if trap_BotFindMatch(
            keyarea.as_mut_ptr(),
            &mut keyareamatch as *mut bot_match_t as *mut libc::c_void,
            64 as i32 as usize,
        ) == 0
        {
            trap_EA_SayTeam(
                (*bs).client,
                b"what do you say?\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            crate::src::game::ai_dmq3::BotFreeWaypoints(newpatrolpoints as *mut bot_waypoint_s);
            (*bs).patrolpoints = std::ptr::null_mut();
            return qfalse as i32;
        }
        trap_BotMatchVariable(
            &mut keyareamatch as *mut bot_match_t as *mut libc::c_void,
            5 as i32,
            keyarea.as_mut_ptr(),
            256 as i32,
        );
        if BotGetMessageTeamGoal(bs, keyarea.as_mut_ptr(), &mut goal) == 0 {
            //BotAI_BotInitialChat(bs, "cannotfind", keyarea, NULL);
            //trap_BotEnterChat(bs->cs, 0, CHAT_TEAM);
            crate::src::game::ai_dmq3::BotFreeWaypoints(newpatrolpoints as *mut bot_waypoint_s);
            (*bs).patrolpoints = std::ptr::null_mut();
            return qfalse as i32;
        }
        //create a new waypoint
        newwp = crate::src::game::ai_dmq3::BotCreateWayPoint(
            keyarea.as_mut_ptr(),
            goal.origin.as_mut_ptr(),
            goal.areanum,
        ) as *mut bot_waypoint_s;
        if newwp.is_null() {
            break;
        }
        //add the waypoint to the patrol points
        (*newwp).next = std::ptr::null_mut();
        wp = newpatrolpoints;
        while !wp.is_null() && !(*wp).next.is_null() {
            wp = (*wp).next
        }
        if wp.is_null() {
            newpatrolpoints = newwp;
            (*newwp).prev = std::ptr::null_mut()
        } else {
            (*wp).next = newwp;
            (*newwp).prev = wp
        }
        //
        if keyareamatch.subtype & 512 as i32 != 0 {
            patrolflags = 1 as i32;
            break;
        } else if keyareamatch.subtype & 1024 as i32 != 0 {
            patrolflags = 2 as i32;
            break;
        } else {
            if !(keyareamatch.subtype & 256 as i32 != 0) {
                break;
            }
            trap_BotMatchVariable(
                &mut keyareamatch as *mut bot_match_t as *mut libc::c_void,
                6 as i32,
                keyarea.as_mut_ptr(),
                256 as i32,
            );
        }
    }
    //
    if newpatrolpoints.is_null() || (*newpatrolpoints).next.is_null() {
        trap_EA_SayTeam(
            (*bs).client,
            b"I need more key points to patrol\n\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        crate::src::game::ai_dmq3::BotFreeWaypoints(newpatrolpoints as *mut bot_waypoint_s);
        newpatrolpoints = std::ptr::null_mut();
        return qfalse as i32;
    }
    //
    crate::src::game::ai_dmq3::BotFreeWaypoints((*bs).patrolpoints as *mut bot_waypoint_s);
    (*bs).patrolpoints = newpatrolpoints;
    //
    (*bs).curpatrolpoint = (*bs).patrolpoints;
    (*bs).patrolflags = patrolflags;
    //
    return qtrue as i32;
}
/*
==================
BotAddressedToBot
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotAddressedToBot(
    mut bs: *mut bot_state_t,
    mut match_0: *mut bot_match_t,
) -> i32 {
    let mut addressedto: [libc::c_char; 256] = [0; 256];
    let mut netname: [libc::c_char; 256] = [0; 256];
    let mut name: [libc::c_char; 256] = [0; 256];
    let mut botname: [libc::c_char; 128] = [0; 128];
    let mut client: i32 = 0;
    let mut addresseematch: bot_match_t = bot_match_t {
        string: [0; 256],
        type_0: 0,
        subtype: 0,
        variables: [bot_matchvariable_t {
            offset: 0,
            length: 0,
        }; 8],
    };
    trap_BotMatchVariable(
        match_0 as *mut libc::c_void,
        0 as i32,
        netname.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 256]>() as usize as i32,
    );
    client = crate::src::game::ai_dmq3::ClientOnSameTeamFromName(
        bs as *mut bot_state_s,
        netname.as_mut_ptr(),
    );
    if client < 0 as i32 {
        return qfalse as i32;
    }
    //if the message is addressed to someone
    if (*match_0).subtype & 2 as i32 != 0 {
        trap_BotMatchVariable(
            match_0 as *mut libc::c_void,
            2 as i32,
            addressedto.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 256]>() as usize as i32,
        );
        //the name of this bot
        crate::src::game::ai_dmq3::ClientName((*bs).client, botname.as_mut_ptr(), 128 as i32);
        //
        while trap_BotFindMatch(
            addressedto.as_mut_ptr(),
            &mut addresseematch as *mut bot_match_t as *mut libc::c_void,
            32 as i32 as usize,
        ) != 0
        {
            if addresseematch.type_0 == 101 as i32 {
                return qtrue as i32;
            } else if addresseematch.type_0 == 102 as i32 {
                trap_BotMatchVariable(
                    &mut addresseematch as *mut bot_match_t as *mut libc::c_void,
                    4 as i32,
                    name.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 256]>() as usize as i32,
                );
                if crate::stdlib::strlen(name.as_mut_ptr()) != 0 {
                    if !crate::src::game::ai_dmq3::stristr(botname.as_mut_ptr(), name.as_mut_ptr())
                        .is_null()
                    {
                        return qtrue as i32;
                    }
                    if !crate::src::game::ai_dmq3::stristr(
                        (*bs).subteam.as_mut_ptr(),
                        name.as_mut_ptr(),
                    )
                    .is_null()
                    {
                        return qtrue as i32;
                    }
                }
                trap_BotMatchVariable(
                    &mut addresseematch as *mut bot_match_t as *mut libc::c_void,
                    6 as i32,
                    addressedto.as_mut_ptr(),
                    256 as i32,
                );
            } else {
                trap_BotMatchVariable(
                    &mut addresseematch as *mut bot_match_t as *mut libc::c_void,
                    4 as i32,
                    name.as_mut_ptr(),
                    256 as i32,
                );
                if crate::stdlib::strlen(name.as_mut_ptr()) != 0 {
                    if !crate::src::game::ai_dmq3::stristr(botname.as_mut_ptr(), name.as_mut_ptr())
                        .is_null()
                    {
                        return qtrue as i32;
                    }
                    if !crate::src::game::ai_dmq3::stristr(
                        (*bs).subteam.as_mut_ptr(),
                        name.as_mut_ptr(),
                    )
                    .is_null()
                    {
                        return qtrue as i32;
                    }
                }
                break;
            }
        }
        //Com_sprintf(buf, sizeof(buf), "not addressed to me but %s", addressedto);
        //trap_EA_Say(bs->client, buf);
        return qfalse as i32;
    } else {
        let mut tellmatch: bot_match_t = bot_match_t {
            string: [0; 256],
            type_0: 0,
            subtype: 0,
            variables: [bot_matchvariable_t {
                offset: 0,
                length: 0,
            }; 8],
        };
        tellmatch.type_0 = 0 as i32;
        //if this message wasn't directed solely to this bot
        if trap_BotFindMatch(
            (*match_0).string.as_mut_ptr(),
            &mut tellmatch as *mut bot_match_t as *mut libc::c_void,
            128 as i32 as usize,
        ) == 0
            || tellmatch.type_0 != 202 as i32
        {
            //make sure not everyone reacts to this message
            if (libc::rand() & 0x7fff as i32) as f32 / 0x7fff as i32 as f32
                > 1.0f64 as f32 / (NumPlayersOnSameTeam(bs) - 1 as i32) as f32
            {
                return qfalse as i32;
            }
        }
    }
    return qtrue as i32;
}
/*
==================
BotGPSToPosition
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotGPSToPosition(
    mut buf: *mut libc::c_char,
    mut position: *mut vec_t,
) -> i32 {
    let mut i: i32 = 0;
    let mut j: i32 = 0 as i32;
    let mut num: i32 = 0;
    let mut sign: i32 = 0;
    i = 0 as i32;
    while i < 3 as i32 {
        num = 0 as i32;
        while *buf.offset(j as isize) as i32 == ' ' as i32 {
            j += 1
        }
        if *buf.offset(j as isize) as i32 == '-' as i32 {
            j += 1;
            sign = -(1 as i32)
        } else {
            sign = 1 as i32
        }
        while *buf.offset(j as isize) != 0 {
            if *buf.offset(j as isize) as i32 >= '0' as i32
                && *buf.offset(j as isize) as i32 <= '9' as i32
            {
                num = num * 10 as i32 + *buf.offset(j as isize) as i32 - '0' as i32;
                j += 1
            } else {
                j += 1;
                break;
            }
        }
        BotAI_Print(
            1 as i32,
            b"%d\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            sign * num,
        );
        *position.offset(i as isize) = sign as f32 * num as f32;
        i += 1
    }
    return qtrue as i32;
}
/*
==================
BotMatch_HelpAccompany
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotMatch_HelpAccompany(
    mut bs: *mut bot_state_t,
    mut match_0: *mut bot_match_t,
) {
    let mut client: i32 = 0;
    let mut other: i32 = 0;
    let mut areanum: i32 = 0;
    let mut teammate: [libc::c_char; 256] = [0; 256];
    let mut netname: [libc::c_char; 256] = [0; 256];
    let mut itemname: [libc::c_char; 256] = [0; 256];
    let mut teammatematch: bot_match_t = bot_match_t {
        string: [0; 256],
        type_0: 0,
        subtype: 0,
        variables: [bot_matchvariable_t {
            offset: 0,
            length: 0,
        }; 8],
    };
    let mut entinfo: aas_entityinfo_t = aas_entityinfo_t {
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
    if crate::src::game::ai_dmq3::TeamPlayIsOn() == 0 {
        return;
    }
    //if not addressed to this bot
    if BotAddressedToBot(bs, match_0) == 0 {
        return;
    }
    //get the team mate name
    trap_BotMatchVariable(
        match_0 as *mut libc::c_void,
        4 as i32,
        teammate.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 256]>() as usize as i32,
    );
    //get the client to help
    if trap_BotFindMatch(
        teammate.as_mut_ptr(),
        &mut teammatematch as *mut bot_match_t as *mut libc::c_void,
        16 as i32 as usize,
    ) != 0
        && teammatematch.type_0 == 100 as i32
    {
        //get the netname
        trap_BotMatchVariable(
            match_0 as *mut libc::c_void,
            0 as i32,
            netname.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 256]>() as usize as i32,
        );
        client = crate::src::game::ai_dmq3::ClientFromName(netname.as_mut_ptr());
        other = qfalse as i32
    } else {
        //asked for someone else
        client = FindClientByName(teammate.as_mut_ptr());
        //if this is the bot self
        if client == (*bs).client {
            other = qfalse as i32
        } else if crate::src::game::ai_dmq3::BotSameTeam(bs as *mut bot_state_s, client) == 0 {
            //FIXME: say "I don't help the enemy"
            return;
        } else {
            other = qtrue as i32
        }
    }
    //if the bot doesn't know who to help (FindClientByName returned -1)
    if client < 0 as i32 {
        if other != 0 {
            BotAI_BotInitialChat(
                bs as *mut bot_state_s,
                b"whois\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                teammate.as_mut_ptr(),
                std::ptr::null_mut::<libc::c_char>(),
            );
        } else {
            BotAI_BotInitialChat(
                bs as *mut bot_state_s,
                b"whois\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                netname.as_mut_ptr(),
                std::ptr::null_mut::<libc::c_char>(),
            );
        }
        client = crate::src::game::ai_dmq3::ClientFromName(netname.as_mut_ptr());
        trap_BotEnterChat((*bs).cs, client, 2 as i32);
        return;
    }
    //don't help or accompany yourself
    if client == (*bs).client {
        return;
    }
    //
    (*bs).teamgoal.entitynum = -(1 as i32);
    BotEntityInfo(client, &mut entinfo as *mut _ as *mut aas_entityinfo_s);
    //if info is valid (in PVS)
    if entinfo.valid != 0 {
        areanum = crate::src::game::ai_dmq3::BotPointAreaNum(entinfo.origin.as_mut_ptr());
        if areanum != 0 {
            // && trap_AAS_AreaReachability(areanum)) {
            (*bs).teamgoal.entitynum = client;
            (*bs).teamgoal.areanum = areanum;
            (*bs).teamgoal.origin[0 as i32 as usize] = entinfo.origin[0 as i32 as usize];
            (*bs).teamgoal.origin[1 as i32 as usize] = entinfo.origin[1 as i32 as usize];
            (*bs).teamgoal.origin[2 as i32 as usize] = entinfo.origin[2 as i32 as usize];
            (*bs).teamgoal.mins[0 as i32 as usize] = -(8 as i32) as vec_t;
            (*bs).teamgoal.mins[1 as i32 as usize] = -(8 as i32) as vec_t;
            (*bs).teamgoal.mins[2 as i32 as usize] = -(8 as i32) as vec_t;
            (*bs).teamgoal.maxs[0 as i32 as usize] = 8 as i32 as vec_t;
            (*bs).teamgoal.maxs[1 as i32 as usize] = 8 as i32 as vec_t;
            (*bs).teamgoal.maxs[2 as i32 as usize] = 8 as i32 as vec_t
        }
    }
    //if no teamgoal yet
    if (*bs).teamgoal.entitynum < 0 as i32 {
        //if near an item
        if (*match_0).subtype & 1 as i32 != 0 {
            //get the match variable
            trap_BotMatchVariable(
                match_0 as *mut libc::c_void,
                3 as i32,
                itemname.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 256]>() as usize as i32,
            );
            //
            if BotGetMessageTeamGoal(bs, itemname.as_mut_ptr(), &mut (*bs).teamgoal) == 0 {
                //BotAI_BotInitialChat(bs, "cannotfind", itemname, NULL);
                //trap_BotEnterChat(bs->cs, bs->client, CHAT_TEAM);
                return;
            }
        }
    }
    //
    if (*bs).teamgoal.entitynum < 0 as i32 {
        if other != 0 {
            BotAI_BotInitialChat(
                bs as *mut bot_state_s,
                b"whereis\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                teammate.as_mut_ptr(),
                std::ptr::null_mut::<libc::c_char>(),
            );
        } else {
            BotAI_BotInitialChat(
                bs as *mut bot_state_s,
                b"whereareyou\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                netname.as_mut_ptr(),
                std::ptr::null_mut::<libc::c_char>(),
            );
        }
        client = crate::src::game::ai_dmq3::ClientFromName(netname.as_mut_ptr());
        trap_BotEnterChat((*bs).cs, client, 1 as i32);
        return;
    }
    //the team mate
    (*bs).teammate = client;
    //
    trap_BotMatchVariable(
        match_0 as *mut libc::c_void,
        0 as i32,
        netname.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 256]>() as usize as i32,
    );
    //
    client = crate::src::game::ai_dmq3::ClientFromName(netname.as_mut_ptr());
    //the team mate who ordered
    (*bs).decisionmaker = client;
    (*bs).ordered = qtrue as i32;
    (*bs).order_time = floattime;
    //last time the team mate was assumed visible
    (*bs).teammatevisible_time = floattime;
    //set the time to send a message to the team mates
    (*bs).teammessage_time = floattime
        + 2 as i32 as f32 * ((libc::rand() & 0x7fff as i32) as f32 / 0x7fff as i32 as f32);
    //get the team goal time
    (*bs).teamgoal_time = BotGetTime(match_0);
    //set the ltg type
    if (*match_0).type_0 == 3 as i32 {
        (*bs).ltgtype = 1 as i32; //3.5 meter
        if (*bs).teamgoal_time == 0. {
            (*bs).teamgoal_time = floattime + 60 as i32 as f32
        }
    } else {
        (*bs).ltgtype = 2 as i32;
        if (*bs).teamgoal_time == 0. {
            (*bs).teamgoal_time = floattime + 600 as i32 as f32
        }
        (*bs).formation_dist = (3.5f64 * 32 as i32 as f64) as f32;
        (*bs).arrive_time = 0 as i32 as f32;
        //
        crate::src::game::ai_dmq3::BotSetTeamStatus(bs as *mut bot_state_s);
        // remember last ordered task
        crate::src::game::ai_dmq3::BotRememberLastOrderedTask(bs as *mut bot_state_s);
    };
    //DEBUG
}
/*
==================
BotMatch_DefendKeyArea
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotMatch_DefendKeyArea(
    mut bs: *mut bot_state_t,
    mut match_0: *mut bot_match_t,
) {
    let mut itemname: [libc::c_char; 256] = [0; 256];
    let mut netname: [libc::c_char; 256] = [0; 256];
    let mut client: i32 = 0;
    if crate::src::game::ai_dmq3::TeamPlayIsOn() == 0 {
        return;
    }
    //if not addressed to this bot
    if BotAddressedToBot(bs, match_0) == 0 {
        return;
    }
    //get the match variable
    trap_BotMatchVariable(
        match_0 as *mut libc::c_void,
        5 as i32,
        itemname.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 256]>() as usize as i32,
    );
    //
    if BotGetMessageTeamGoal(bs, itemname.as_mut_ptr(), &mut (*bs).teamgoal) == 0 {
        //BotAI_BotInitialChat(bs, "cannotfind", itemname, NULL);
        //trap_BotEnterChat(bs->cs, bs->client, CHAT_TEAM);
        return;
    }
    //
    trap_BotMatchVariable(
        match_0 as *mut libc::c_void,
        0 as i32,
        netname.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 256]>() as usize as i32,
    );
    //
    client = crate::src::game::ai_dmq3::ClientFromName(netname.as_mut_ptr());
    //the team mate who ordered
    (*bs).decisionmaker = client;
    (*bs).ordered = qtrue as i32;
    (*bs).order_time = floattime;
    //set the time to send a message to the team mates
    (*bs).teammessage_time = floattime
        + 2 as i32 as f32 * ((libc::rand() & 0x7fff as i32) as f32 / 0x7fff as i32 as f32);
    //set the ltg type
    (*bs).ltgtype = 3 as i32;
    //get the team goal time
    (*bs).teamgoal_time = BotGetTime(match_0);
    //set the team goal time
    if (*bs).teamgoal_time == 0. {
        (*bs).teamgoal_time = floattime + 600 as i32 as f32
    }
    //away from defending
    (*bs).defendaway_time = 0 as i32 as f32;
    //
    crate::src::game::ai_dmq3::BotSetTeamStatus(bs as *mut bot_state_s);
    // remember last ordered task
    crate::src::game::ai_dmq3::BotRememberLastOrderedTask(bs as *mut bot_state_s);
    //DEBUG
}
/*
==================
BotMatch_GetItem
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotMatch_GetItem(mut bs: *mut bot_state_t, mut match_0: *mut bot_match_t) {
    let mut itemname: [libc::c_char; 256] = [0; 256];
    let mut netname: [libc::c_char; 256] = [0; 256];
    let mut client: i32 = 0;
    if crate::src::game::ai_dmq3::TeamPlayIsOn() == 0 {
        return;
    }
    //if not addressed to this bot
    if BotAddressedToBot(bs, match_0) == 0 {
        return;
    }
    //get the match variable
    trap_BotMatchVariable(
        match_0 as *mut libc::c_void,
        3 as i32,
        itemname.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 256]>() as usize as i32,
    );
    //
    if BotGetMessageTeamGoal(bs, itemname.as_mut_ptr(), &mut (*bs).teamgoal) == 0 {
        //BotAI_BotInitialChat(bs, "cannotfind", itemname, NULL);
        //trap_BotEnterChat(bs->cs, bs->client, CHAT_TEAM);
        return;
    }
    trap_BotMatchVariable(
        match_0 as *mut libc::c_void,
        0 as i32,
        netname.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 256]>() as usize as i32,
    );
    client = crate::src::game::ai_dmq3::ClientOnSameTeamFromName(
        bs as *mut bot_state_s,
        netname.as_mut_ptr(),
    );
    //
    (*bs).decisionmaker = client;
    (*bs).ordered = qtrue as i32;
    (*bs).order_time = floattime;
    //set the time to send a message to the team mates
    (*bs).teammessage_time = floattime
        + 2 as i32 as f32 * ((libc::rand() & 0x7fff as i32) as f32 / 0x7fff as i32 as f32);
    //set the ltg type
    (*bs).ltgtype = 10 as i32;
    //set the team goal time
    (*bs).teamgoal_time = floattime + 60 as i32 as f32;
    //
    crate::src::game::ai_dmq3::BotSetTeamStatus(bs as *mut bot_state_s);
    //DEBUG
}
/*
==================
BotMatch_Camp
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotMatch_Camp(mut bs: *mut bot_state_t, mut match_0: *mut bot_match_t) {
    let mut client: i32 = 0;
    let mut areanum: i32 = 0;
    let mut netname: [libc::c_char; 256] = [0; 256];
    let mut itemname: [libc::c_char; 256] = [0; 256];
    let mut entinfo: aas_entityinfo_t = aas_entityinfo_t {
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
    if crate::src::game::ai_dmq3::TeamPlayIsOn() == 0 {
        return;
    }
    //if not addressed to this bot
    if BotAddressedToBot(bs, match_0) == 0 {
        return;
    }
    //
    trap_BotMatchVariable(
        match_0 as *mut libc::c_void,
        0 as i32,
        netname.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 256]>() as usize as i32,
    );
    //asked for someone else
    client = FindClientByName(netname.as_mut_ptr());
    //if there's no valid client with this name
    if client < 0 as i32 {
        BotAI_BotInitialChat(
            bs as *mut bot_state_s,
            b"whois\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            netname.as_mut_ptr(),
            std::ptr::null_mut::<libc::c_char>(),
        );
        trap_BotEnterChat((*bs).cs, (*bs).client, 1 as i32);
        return;
    }
    //get the match variable
    trap_BotMatchVariable(
        match_0 as *mut libc::c_void,
        5 as i32,
        itemname.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 256]>() as usize as i32,
    );
    //in CTF it could be the base
    if (*match_0).subtype & 64 as i32 != 0 {
        //camp at the spot the bot is currently standing
        (*bs).teamgoal.entitynum = (*bs).entitynum;
        (*bs).teamgoal.areanum = (*bs).areanum;
        (*bs).teamgoal.origin[0 as i32 as usize] = (*bs).origin[0 as i32 as usize];
        (*bs).teamgoal.origin[1 as i32 as usize] = (*bs).origin[1 as i32 as usize];
        (*bs).teamgoal.origin[2 as i32 as usize] = (*bs).origin[2 as i32 as usize];
        (*bs).teamgoal.mins[0 as i32 as usize] = -(8 as i32) as vec_t;
        (*bs).teamgoal.mins[1 as i32 as usize] = -(8 as i32) as vec_t;
        (*bs).teamgoal.mins[2 as i32 as usize] = -(8 as i32) as vec_t;
        (*bs).teamgoal.maxs[0 as i32 as usize] = 8 as i32 as vec_t;
        (*bs).teamgoal.maxs[1 as i32 as usize] = 8 as i32 as vec_t;
        (*bs).teamgoal.maxs[2 as i32 as usize] = 8 as i32 as vec_t
    } else if (*match_0).subtype & 32 as i32 != 0 {
        //if this is the bot self
        if client == (*bs).client {
            return;
        }
        //
        (*bs).teamgoal.entitynum = -(1 as i32);
        BotEntityInfo(client, &mut entinfo as *mut _ as *mut aas_entityinfo_s);
        //if info is valid (in PVS)
        if entinfo.valid != 0 {
            areanum = crate::src::game::ai_dmq3::BotPointAreaNum(entinfo.origin.as_mut_ptr());
            if areanum != 0 {
                // && trap_AAS_AreaReachability(areanum)) {
                //NOTE: just assume the bot knows where the person is
                //if (BotEntityVisible(bs->entitynum, bs->eye, bs->viewangles, 360, client)) {
                (*bs).teamgoal.entitynum = client;
                (*bs).teamgoal.areanum = areanum;
                (*bs).teamgoal.origin[0 as i32 as usize] = entinfo.origin[0 as i32 as usize];
                (*bs).teamgoal.origin[1 as i32 as usize] = entinfo.origin[1 as i32 as usize];
                (*bs).teamgoal.origin[2 as i32 as usize] = entinfo.origin[2 as i32 as usize];
                (*bs).teamgoal.mins[0 as i32 as usize] = -(8 as i32) as vec_t;
                (*bs).teamgoal.mins[1 as i32 as usize] = -(8 as i32) as vec_t;
                (*bs).teamgoal.mins[2 as i32 as usize] = -(8 as i32) as vec_t;
                (*bs).teamgoal.maxs[0 as i32 as usize] = 8 as i32 as vec_t;
                (*bs).teamgoal.maxs[1 as i32 as usize] = 8 as i32 as vec_t;
                (*bs).teamgoal.maxs[2 as i32 as usize] = 8 as i32 as vec_t
                //}
            }
        }
        //if the other is not visible
        if (*bs).teamgoal.entitynum < 0 as i32 {
            BotAI_BotInitialChat(
                bs as *mut bot_state_s,
                b"whereareyou\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                netname.as_mut_ptr(),
                std::ptr::null_mut::<libc::c_char>(),
            );
            client = crate::src::game::ai_dmq3::ClientFromName(netname.as_mut_ptr());
            trap_BotEnterChat((*bs).cs, client, 2 as i32);
            return;
        }
    } else if BotGetMessageTeamGoal(bs, itemname.as_mut_ptr(), &mut (*bs).teamgoal) == 0 {
        //BotAI_BotInitialChat(bs, "cannotfind", itemname, NULL);
        //client = ClientFromName(netname);
        //trap_BotEnterChat(bs->cs, client, CHAT_TELL);
        return;
    }
    //
    (*bs).decisionmaker = client;
    (*bs).ordered = qtrue as i32;
    (*bs).order_time = floattime;
    //set the time to send a message to the team mates
    (*bs).teammessage_time = floattime
        + 2 as i32 as f32 * ((libc::rand() & 0x7fff as i32) as f32 / 0x7fff as i32 as f32);
    //set the ltg type
    (*bs).ltgtype = 8 as i32;
    //get the team goal time
    (*bs).teamgoal_time = BotGetTime(match_0);
    //set the team goal time
    if (*bs).teamgoal_time == 0. {
        (*bs).teamgoal_time = floattime + 600 as i32 as f32
    }
    //not arrived yet
    (*bs).arrive_time = 0 as i32 as f32;
    //
    crate::src::game::ai_dmq3::BotSetTeamStatus(bs as *mut bot_state_s);
    // remember last ordered task
    crate::src::game::ai_dmq3::BotRememberLastOrderedTask(bs as *mut bot_state_s);
    //DEBUG
}
/*
==================
BotMatch_Patrol
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotMatch_Patrol(mut bs: *mut bot_state_t, mut match_0: *mut bot_match_t) {
    let mut netname: [libc::c_char; 256] = [0; 256];
    let mut client: i32 = 0;
    if crate::src::game::ai_dmq3::TeamPlayIsOn() == 0 {
        return;
    }
    //if not addressed to this bot
    if BotAddressedToBot(bs, match_0) == 0 {
        return;
    }
    //get the patrol waypoints
    if BotGetPatrolWaypoints(bs, match_0) == 0 {
        return;
    }
    //
    trap_BotMatchVariable(
        match_0 as *mut libc::c_void,
        0 as i32,
        netname.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 256]>() as usize as i32,
    );
    //
    client = FindClientByName(netname.as_mut_ptr());
    //
    (*bs).decisionmaker = client;
    (*bs).ordered = qtrue as i32;
    (*bs).order_time = floattime;
    //set the time to send a message to the team mates
    (*bs).teammessage_time = floattime
        + 2 as i32 as f32 * ((libc::rand() & 0x7fff as i32) as f32 / 0x7fff as i32 as f32);
    //set the ltg type
    (*bs).ltgtype = 9 as i32;
    //get the team goal time
    (*bs).teamgoal_time = BotGetTime(match_0);
    //set the team goal time if not set already
    if (*bs).teamgoal_time == 0. {
        (*bs).teamgoal_time = floattime + 600 as i32 as f32
    }
    //
    crate::src::game::ai_dmq3::BotSetTeamStatus(bs as *mut bot_state_s);
    // remember last ordered task
    crate::src::game::ai_dmq3::BotRememberLastOrderedTask(bs as *mut bot_state_s);
    //DEBUG
}
/*
==================
BotMatch_GetFlag
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotMatch_GetFlag(mut bs: *mut bot_state_t, mut match_0: *mut bot_match_t) {
    let mut netname: [libc::c_char; 256] = [0; 256];
    let mut client: i32 = 0;
    if crate::src::game::ai_dmq3::gametype == GT_CTF as i32 {
        if crate::src::game::ai_dmq3::ctf_redflag.areanum == 0
            || crate::src::game::ai_dmq3::ctf_blueflag.areanum == 0
        {
            return;
        }
    } else {
        return;
    }
    //if not addressed to this bot
    if BotAddressedToBot(bs, match_0) == 0 {
        return;
    }
    //
    trap_BotMatchVariable(
        match_0 as *mut libc::c_void,
        0 as i32,
        netname.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 256]>() as usize as i32,
    );
    //
    client = FindClientByName(netname.as_mut_ptr());
    //
    (*bs).decisionmaker = client;
    (*bs).ordered = qtrue as i32;
    (*bs).order_time = floattime;
    //set the time to send a message to the team mates
    (*bs).teammessage_time = floattime
        + 2 as i32 as f32 * ((libc::rand() & 0x7fff as i32) as f32 / 0x7fff as i32 as f32);
    //set the ltg type
    (*bs).ltgtype = 4 as i32;
    //set the team goal time
    (*bs).teamgoal_time = floattime + 600 as i32 as f32;
    // get an alternate route in ctf
    if crate::src::game::ai_dmq3::gametype == GT_CTF as i32 {
        //get an alternative route goal towards the enemy base
        crate::src::game::ai_dmq3::BotGetAlternateRouteGoal(
            bs as *mut bot_state_s,
            crate::src::game::ai_dmq3::BotOppositeTeam(bs as *mut bot_state_s),
        );
    }
    //
    crate::src::game::ai_dmq3::BotSetTeamStatus(bs as *mut bot_state_s);
    // remember last ordered task
    crate::src::game::ai_dmq3::BotRememberLastOrderedTask(bs as *mut bot_state_s);
    //DEBUG
}
/*
==================
BotMatch_AttackEnemyBase
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotMatch_AttackEnemyBase(
    mut bs: *mut bot_state_t,
    mut match_0: *mut bot_match_t,
) {
    let mut netname: [libc::c_char; 256] = [0; 256];
    let mut client: i32 = 0;
    if crate::src::game::ai_dmq3::gametype == GT_CTF as i32 {
        BotMatch_GetFlag(bs, match_0);
    } else {
        return;
    }
    //if not addressed to this bot
    if BotAddressedToBot(bs, match_0) == 0 {
        return;
    }
    //
    trap_BotMatchVariable(
        match_0 as *mut libc::c_void,
        0 as i32,
        netname.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 256]>() as usize as i32,
    );
    //
    client = FindClientByName(netname.as_mut_ptr());
    //
    (*bs).decisionmaker = client;
    (*bs).ordered = qtrue as i32;
    (*bs).order_time = floattime;
    //set the time to send a message to the team mates
    (*bs).teammessage_time = floattime
        + 2 as i32 as f32 * ((libc::rand() & 0x7fff as i32) as f32 / 0x7fff as i32 as f32);
    //set the ltg type
    (*bs).ltgtype = 13 as i32;
    //set the team goal time
    (*bs).teamgoal_time = floattime + 600 as i32 as f32;
    (*bs).attackaway_time = 0 as i32 as f32;
    //
    crate::src::game::ai_dmq3::BotSetTeamStatus(bs as *mut bot_state_s);
    // remember last ordered task
    crate::src::game::ai_dmq3::BotRememberLastOrderedTask(bs as *mut bot_state_s);
    //DEBUG
}
/*
==================
BotMatch_RushBase
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotMatch_RushBase(
    mut bs: *mut bot_state_t,
    mut match_0: *mut bot_match_t,
) {
    let mut netname: [libc::c_char; 256] = [0; 256];
    let mut client: i32 = 0;
    if crate::src::game::ai_dmq3::gametype == GT_CTF as i32 {
        if crate::src::game::ai_dmq3::ctf_redflag.areanum == 0
            || crate::src::game::ai_dmq3::ctf_blueflag.areanum == 0
        {
            return;
        }
    } else {
        return;
    }
    //if not addressed to this bot
    if BotAddressedToBot(bs, match_0) == 0 {
        return;
    }
    //
    trap_BotMatchVariable(
        match_0 as *mut libc::c_void,
        0 as i32,
        netname.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 256]>() as usize as i32,
    );
    //
    client = FindClientByName(netname.as_mut_ptr());
    //
    (*bs).decisionmaker = client;
    (*bs).ordered = qtrue as i32;
    (*bs).order_time = floattime;
    //set the time to send a message to the team mates
    (*bs).teammessage_time = floattime
        + 2 as i32 as f32 * ((libc::rand() & 0x7fff as i32) as f32 / 0x7fff as i32 as f32);
    //set the ltg type
    (*bs).ltgtype = 5 as i32;
    //set the team goal time
    (*bs).teamgoal_time = floattime + 120 as i32 as f32;
    (*bs).rushbaseaway_time = 0 as i32 as f32;
    //
    crate::src::game::ai_dmq3::BotSetTeamStatus(bs as *mut bot_state_s);
    //DEBUG
}
/*
==================
BotMatch_TaskPreference
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotMatch_TaskPreference(
    mut bs: *mut bot_state_t,
    mut match_0: *mut bot_match_t,
) {
    let mut netname: [libc::c_char; 36] = [0; 36];
    let mut teammatename: [libc::c_char; 256] = [0; 256];
    let mut teammate: i32 = 0;
    let mut preference: i32 = 0;
    crate::src::game::ai_dmq3::ClientName(
        (*bs).client,
        netname.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 36]>() as usize as i32,
    );
    if Q_stricmp(netname.as_mut_ptr(), (*bs).teamleader.as_mut_ptr()) != 0 as i32 {
        return;
    }
    trap_BotMatchVariable(
        match_0 as *mut libc::c_void,
        0 as i32,
        teammatename.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 256]>() as usize as i32,
    );
    teammate = crate::src::game::ai_dmq3::ClientFromName(teammatename.as_mut_ptr());
    if teammate < 0 as i32 {
        return;
    }
    preference =
        crate::src::game::ai_team::BotGetTeamMateTaskPreference(bs as *mut bot_state_s, teammate);
    match (*match_0).subtype {
        1 => {
            preference &= !(2 as i32);
            preference |= 1 as i32
        }
        2 => {
            preference &= !(1 as i32);
            preference |= 2 as i32
        }
        4 => preference &= !(2 as i32 | 1 as i32),
        _ => {}
    }
    crate::src::game::ai_team::BotSetTeamMateTaskPreference(
        bs as *mut bot_state_s,
        teammate,
        preference,
    );
    //
    crate::src::game::ai_dmq3::EasyClientName(
        teammate,
        teammatename.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 256]>() as usize as i32,
    );
    BotAI_BotInitialChat(
        bs as *mut bot_state_s,
        b"keepinmind\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        teammatename.as_mut_ptr(),
        std::ptr::null_mut::<libc::c_char>(),
    );
    trap_BotEnterChat((*bs).cs, teammate, 2 as i32);
    crate::src::game::ai_team::BotVoiceChatOnly(
        bs as *mut bot_state_s,
        teammate,
        b"yes\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    trap_EA_Action((*bs).client, 0x100000 as i32);
}
/*
==================
BotMatch_ReturnFlag
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotMatch_ReturnFlag(
    mut bs: *mut bot_state_t,
    mut match_0: *mut bot_match_t,
) {
    let mut netname: [libc::c_char; 256] = [0; 256];
    let mut client: i32 = 0;
    //if not in CTF mode
    if crate::src::game::ai_dmq3::gametype != GT_CTF as i32 {
        return;
    }
    //if not addressed to this bot
    if BotAddressedToBot(bs, match_0) == 0 {
        return;
    }
    //
    trap_BotMatchVariable(
        match_0 as *mut libc::c_void,
        0 as i32,
        netname.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 256]>() as usize as i32,
    );
    //
    client = FindClientByName(netname.as_mut_ptr());
    //
    (*bs).decisionmaker = client;
    (*bs).ordered = qtrue as i32;
    (*bs).order_time = floattime;
    //set the time to send a message to the team mates
    (*bs).teammessage_time = floattime
        + 2 as i32 as f32 * ((libc::rand() & 0x7fff as i32) as f32 / 0x7fff as i32 as f32);
    //set the ltg type
    (*bs).ltgtype = 6 as i32;
    //set the team goal time
    (*bs).teamgoal_time = floattime + 180 as i32 as f32;
    (*bs).rushbaseaway_time = 0 as i32 as f32;
    //
    crate::src::game::ai_dmq3::BotSetTeamStatus(bs as *mut bot_state_s);
    //DEBUG
}
/*
==================
BotMatch_JoinSubteam
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotMatch_JoinSubteam(
    mut bs: *mut bot_state_t,
    mut match_0: *mut bot_match_t,
) {
    let mut teammate: [libc::c_char; 256] = [0; 256];
    let mut netname: [libc::c_char; 256] = [0; 256];
    let mut client: i32 = 0;
    if crate::src::game::ai_dmq3::TeamPlayIsOn() == 0 {
        return;
    }
    //if not addressed to this bot
    if BotAddressedToBot(bs, match_0) == 0 {
        return;
    }
    //get the sub team name
    trap_BotMatchVariable(
        match_0 as *mut libc::c_void,
        4 as i32,
        teammate.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 256]>() as usize as i32,
    );
    //set the sub team name
    crate::stdlib::strncpy(
        (*bs).subteam.as_mut_ptr(),
        teammate.as_mut_ptr(),
        32 as i32 as usize,
    );
    (*bs).subteam[31 as i32 as usize] = '\u{0}' as i32 as libc::c_char;
    //
    trap_BotMatchVariable(
        match_0 as *mut libc::c_void,
        0 as i32,
        netname.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 256]>() as usize as i32,
    );
    BotAI_BotInitialChat(
        bs as *mut bot_state_s,
        b"joinedteam\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        teammate.as_mut_ptr(),
        std::ptr::null_mut::<libc::c_char>(),
    );
    client = crate::src::game::ai_dmq3::ClientFromName(netname.as_mut_ptr());
    trap_BotEnterChat((*bs).cs, client, 2 as i32);
}
/*
==================
BotMatch_LeaveSubteam
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotMatch_LeaveSubteam(
    mut bs: *mut bot_state_t,
    mut match_0: *mut bot_match_t,
) {
    let mut netname: [libc::c_char; 256] = [0; 256];
    let mut client: i32 = 0;
    if crate::src::game::ai_dmq3::TeamPlayIsOn() == 0 {
        return;
    }
    //if not addressed to this bot
    if BotAddressedToBot(bs, match_0) == 0 {
        return;
    }
    //
    if crate::stdlib::strlen((*bs).subteam.as_mut_ptr()) != 0 {
        BotAI_BotInitialChat(
            bs as *mut bot_state_s,
            b"leftteam\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            (*bs).subteam.as_mut_ptr(),
            std::ptr::null_mut::<libc::c_char>(),
        ); //end if
        trap_BotMatchVariable(
            match_0 as *mut libc::c_void,
            0 as i32,
            netname.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 256]>() as usize as i32,
        );
        client = crate::src::game::ai_dmq3::ClientFromName(netname.as_mut_ptr());
        trap_BotEnterChat((*bs).cs, client, 2 as i32);
    }
    libc::strcpy(
        (*bs).subteam.as_mut_ptr(),
        b"\x00" as *const u8 as *const libc::c_char,
    );
}
/*
==================
BotMatch_LeaveSubteam
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotMatch_WhichTeam(
    mut bs: *mut bot_state_t,
    mut match_0: *mut bot_match_t,
) {
    if crate::src::game::ai_dmq3::TeamPlayIsOn() == 0 {
        return;
    }
    //if not addressed to this bot
    if BotAddressedToBot(bs, match_0) == 0 {
        return;
    }
    //
    if crate::stdlib::strlen((*bs).subteam.as_mut_ptr()) != 0 {
        BotAI_BotInitialChat(
            bs as *mut bot_state_s,
            b"inteam\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            (*bs).subteam.as_mut_ptr(),
            std::ptr::null_mut::<libc::c_char>(),
        );
    } else {
        BotAI_BotInitialChat(
            bs as *mut bot_state_s,
            b"noteam\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            std::ptr::null_mut::<libc::c_char>(),
        );
    }
    trap_BotEnterChat((*bs).cs, (*bs).client, 1 as i32);
}
/*
==================
BotMatch_CheckPoint
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotMatch_CheckPoint(
    mut bs: *mut bot_state_t,
    mut match_0: *mut bot_match_t,
) {
    let mut areanum: i32 = 0;
    let mut client: i32 = 0;
    let mut buf: [libc::c_char; 256] = [0; 256];
    let mut netname: [libc::c_char; 256] = [0; 256];
    let mut position: vec3_t = [0.; 3];
    let mut cp: *mut bot_waypoint_t = std::ptr::null_mut();
    if crate::src::game::ai_dmq3::TeamPlayIsOn() == 0 {
        return;
    }
    //
    trap_BotMatchVariable(
        match_0 as *mut libc::c_void,
        5 as i32,
        buf.as_mut_ptr(),
        256 as i32,
    );
    position[2 as i32 as usize] = 0 as i32 as vec_t;
    position[1 as i32 as usize] = position[2 as i32 as usize];
    position[0 as i32 as usize] = position[1 as i32 as usize];
    //
    trap_BotMatchVariable(
        match_0 as *mut libc::c_void,
        0 as i32,
        netname.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 256]>() as usize as i32,
    );
    client = crate::src::game::ai_dmq3::ClientFromName(netname.as_mut_ptr());
    //BotGPSToPosition(buf, position);
    libc::sscanf(
        buf.as_mut_ptr(),
        b"%f %f %f\x00" as *const u8 as *const libc::c_char,
        &mut *position.as_mut_ptr().offset(0 as i32 as isize) as *mut vec_t,
        &mut *position.as_mut_ptr().offset(1 as i32 as isize) as *mut vec_t,
        &mut *position.as_mut_ptr().offset(2 as i32 as isize) as *mut vec_t,
    );
    position[2 as i32 as usize] = (position[2 as i32 as usize] as f64 + 0.5f64) as vec_t;
    areanum = crate::src::game::ai_dmq3::BotPointAreaNum(position.as_mut_ptr());
    if areanum == 0 {
        if BotAddressedToBot(bs, match_0) != 0 {
            BotAI_BotInitialChat(
                bs as *mut bot_state_s,
                b"checkpoint_invalid\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                std::ptr::null_mut(),
            );
            trap_BotEnterChat((*bs).cs, client, 2 as i32);
        }
        return;
    }
    //
    trap_BotMatchVariable(
        match_0 as *mut libc::c_void,
        6 as i32,
        buf.as_mut_ptr(),
        256 as i32,
    );
    //check if there already exists a checkpoint with this name
    cp = crate::src::game::ai_dmq3::BotFindWayPoint(
        (*bs).checkpoints as *mut bot_waypoint_s,
        buf.as_mut_ptr(),
    ) as *mut bot_waypoint_s;
    if !cp.is_null() {
        if !(*cp).next.is_null() {
            (*(*cp).next).prev = (*cp).prev
        }
        if !(*cp).prev.is_null() {
            (*(*cp).prev).next = (*cp).next
        } else {
            (*bs).checkpoints = (*cp).next
        }
        (*cp).inuse = qfalse as i32
    }
    //create a new check point
    cp = crate::src::game::ai_dmq3::BotCreateWayPoint(
        buf.as_mut_ptr(),
        position.as_mut_ptr(),
        areanum,
    ) as *mut bot_waypoint_s;
    //add the check point to the bot's known chech points
    (*cp).next = (*bs).checkpoints;
    if !(*bs).checkpoints.is_null() {
        (*(*bs).checkpoints).prev = cp
    }
    (*bs).checkpoints = cp;
    //
    if BotAddressedToBot(bs, match_0) != 0 {
        Com_sprintf(
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 256]>() as usize as i32,
            b"%1.0f %1.0f %1.0f\x00" as *const u8 as *const libc::c_char,
            (*cp).goal.origin[0 as i32 as usize] as f64,
            (*cp).goal.origin[1 as i32 as usize] as f64,
            (*cp).goal.origin[2 as i32 as usize] as f64,
        );
        BotAI_BotInitialChat(
            bs as *mut bot_state_s,
            b"checkpoint_confirm\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            (*cp).name.as_mut_ptr(),
            buf.as_mut_ptr(),
            std::ptr::null_mut::<libc::c_char>(),
        );
        trap_BotEnterChat((*bs).cs, client, 2 as i32);
    };
}
/*
==================
BotMatch_FormationSpace
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotMatch_FormationSpace(
    mut bs: *mut bot_state_t,
    mut match_0: *mut bot_match_t,
) {
    let mut buf: [libc::c_char; 256] = [0; 256];
    let mut space: f32 = 0.;
    if crate::src::game::ai_dmq3::TeamPlayIsOn() == 0 {
        return;
    }
    //if not addressed to this bot
    if BotAddressedToBot(bs, match_0) == 0 {
        return;
    }
    //
    trap_BotMatchVariable(
        match_0 as *mut libc::c_void,
        5 as i32,
        buf.as_mut_ptr(),
        256 as i32,
    );
    //if it's the distance in feet
    if (*match_0).subtype & 8 as i32 != 0 {
        space = (0.3048f64 * 32 as i32 as f64 * atof(buf.as_mut_ptr())) as f32
    } else {
        //else it's in meters
        space = (32 as i32 as f64 * atof(buf.as_mut_ptr())) as f32
    }
    //check if the formation intervening space is valid
    if space < 48 as i32 as f32 || space > 500 as i32 as f32 {
        space = 100 as i32 as f32
    }
    (*bs).formation_dist = space;
}
/*
==================
BotMatch_Dismiss
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotMatch_Dismiss(mut bs: *mut bot_state_t, mut match_0: *mut bot_match_t) {
    let mut netname: [libc::c_char; 256] = [0; 256];
    let mut client: i32 = 0;
    if crate::src::game::ai_dmq3::TeamPlayIsOn() == 0 {
        return;
    }
    //if not addressed to this bot
    if BotAddressedToBot(bs, match_0) == 0 {
        return;
    }
    trap_BotMatchVariable(
        match_0 as *mut libc::c_void,
        0 as i32,
        netname.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 256]>() as usize as i32,
    );
    client = crate::src::game::ai_dmq3::ClientFromName(netname.as_mut_ptr());
    //
    (*bs).decisionmaker = client;
    //
    (*bs).ltgtype = 0 as i32;
    (*bs).lead_time = 0 as i32 as f32;
    (*bs).lastgoal_ltgtype = 0 as i32;
    //
    BotAI_BotInitialChat(
        bs as *mut bot_state_s,
        b"dismissed\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        std::ptr::null_mut::<libc::c_char>(),
    );
    trap_BotEnterChat((*bs).cs, client, 2 as i32);
}
/*
==================
BotMatch_Suicide
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotMatch_Suicide(mut bs: *mut bot_state_t, mut match_0: *mut bot_match_t) {
    let mut netname: [libc::c_char; 256] = [0; 256];
    let mut client: i32 = 0;
    if crate::src::game::ai_dmq3::TeamPlayIsOn() == 0 {
        return;
    }
    //if not addressed to this bot
    if BotAddressedToBot(bs, match_0) == 0 {
        return;
    }
    //
    trap_EA_Command(
        (*bs).client,
        b"kill\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    //
    trap_BotMatchVariable(
        match_0 as *mut libc::c_void,
        0 as i32,
        netname.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 256]>() as usize as i32,
    );
    client = crate::src::game::ai_dmq3::ClientFromName(netname.as_mut_ptr());
    //
    crate::src::game::ai_team::BotVoiceChat(
        bs as *mut bot_state_s,
        client,
        b"taunt\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    trap_EA_Action((*bs).client, 0x100000 as i32);
}
/*
==================
BotMatch_StartTeamLeaderShip
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotMatch_StartTeamLeaderShip(
    mut bs: *mut bot_state_t,
    mut match_0: *mut bot_match_t,
) {
    let mut client: i32 = 0;
    let mut teammate: [libc::c_char; 256] = [0; 256];
    if crate::src::game::ai_dmq3::TeamPlayIsOn() == 0 {
        return;
    }
    //if chats for him or herself
    if (*match_0).subtype & 128 as i32 != 0 {
        //get the team mate that will be the team leader
        trap_BotMatchVariable(
            match_0 as *mut libc::c_void,
            0 as i32,
            teammate.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 256]>() as usize as i32,
        );
        crate::stdlib::strncpy(
            (*bs).teamleader.as_mut_ptr(),
            teammate.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 36]>() as usize,
        );
        (*bs).teamleader[(::std::mem::size_of::<[libc::c_char; 36]>() as usize)
            .wrapping_sub(1 as i32 as usize) as usize] = '\u{0}' as i32 as libc::c_char
    } else {
        //chats for someone else
        //get the team mate that will be the team leader
        trap_BotMatchVariable(
            match_0 as *mut libc::c_void,
            4 as i32,
            teammate.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 256]>() as usize as i32,
        );
        client = FindClientByName(teammate.as_mut_ptr());
        if client >= 0 as i32 {
            crate::src::game::ai_dmq3::ClientName(
                client,
                (*bs).teamleader.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 36]>() as usize as i32,
            );
        }
    };
}
/*
==================
BotMatch_StopTeamLeaderShip
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotMatch_StopTeamLeaderShip(
    mut bs: *mut bot_state_t,
    mut match_0: *mut bot_match_t,
) {
    let mut client: i32 = 0;
    let mut teammate: [libc::c_char; 256] = [0; 256];
    let mut netname: [libc::c_char; 256] = [0; 256];
    if crate::src::game::ai_dmq3::TeamPlayIsOn() == 0 {
        return;
    }
    //get the team mate that stops being the team leader
    trap_BotMatchVariable(
        match_0 as *mut libc::c_void,
        4 as i32,
        teammate.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 256]>() as usize as i32,
    );
    //if chats for him or herself
    if (*match_0).subtype & 128 as i32 != 0 {
        trap_BotMatchVariable(
            match_0 as *mut libc::c_void,
            0 as i32,
            netname.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 256]>() as usize as i32,
        ); //end else
        client = FindClientByName(netname.as_mut_ptr())
    } else {
        //chats for someone else
        client = FindClientByName(teammate.as_mut_ptr())
    }
    if client >= 0 as i32 {
        if Q_stricmp(
            (*bs).teamleader.as_mut_ptr(),
            crate::src::game::ai_dmq3::ClientName(
                client,
                netname.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 256]>() as usize as i32,
            ),
        ) == 0
        {
            (*bs).teamleader[0 as i32 as usize] = '\u{0}' as i32 as libc::c_char;
            notleader[client as usize] = qtrue as i32
        }
    };
}
/*
==================
BotMatch_WhoIsTeamLeader
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotMatch_WhoIsTeamLeader(
    mut bs: *mut bot_state_t,
    mut _match_0: *mut bot_match_t,
) {
    let mut netname: [libc::c_char; 256] = [0; 256];
    if crate::src::game::ai_dmq3::TeamPlayIsOn() == 0 {
        return;
    }
    crate::src::game::ai_dmq3::ClientName(
        (*bs).client,
        netname.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 256]>() as usize as i32,
    );
    //if this bot IS the team leader
    if Q_stricmp(netname.as_mut_ptr(), (*bs).teamleader.as_mut_ptr()) == 0 {
        trap_EA_SayTeam(
            (*bs).client,
            b"I\'m the team leader\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    };
}
/*
==================
BotMatch_WhatAreYouDoing
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotMatch_WhatAreYouDoing(
    mut bs: *mut bot_state_t,
    mut match_0: *mut bot_match_t,
) {
    let mut netname: [libc::c_char; 256] = [0; 256];
    let mut goalname: [libc::c_char; 256] = [0; 256];
    let mut client: i32 = 0;
    //if not addressed to this bot
    if BotAddressedToBot(bs, match_0) == 0 {
        return;
    }
    //
    match (*bs).ltgtype {
        1 => {
            crate::src::game::ai_dmq3::EasyClientName(
                (*bs).teammate,
                netname.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 256]>() as usize as i32,
            );
            BotAI_BotInitialChat(
                bs as *mut bot_state_s,
                b"helping\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                netname.as_mut_ptr(),
                std::ptr::null_mut(),
            );
        }
        2 => {
            crate::src::game::ai_dmq3::EasyClientName(
                (*bs).teammate,
                netname.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 256]>() as usize as i32,
            );
            BotAI_BotInitialChat(
                bs as *mut bot_state_s,
                b"accompanying\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                netname.as_mut_ptr(),
                std::ptr::null_mut(),
            );
        }
        3 => {
            trap_BotGoalName(
                (*bs).teamgoal.number,
                goalname.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 256]>() as usize as i32,
            );
            BotAI_BotInitialChat(
                bs as *mut bot_state_s,
                b"defending\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                goalname.as_mut_ptr(),
                std::ptr::null_mut(),
            );
        }
        10 => {
            trap_BotGoalName(
                (*bs).teamgoal.number,
                goalname.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 256]>() as usize as i32,
            );
            BotAI_BotInitialChat(
                bs as *mut bot_state_s,
                b"gettingitem\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                goalname.as_mut_ptr(),
                std::ptr::null_mut(),
            );
        }
        11 => {
            crate::src::game::ai_dmq3::ClientName(
                (*bs).teamgoal.entitynum,
                netname.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 256]>() as usize as i32,
            );
            BotAI_BotInitialChat(
                bs as *mut bot_state_s,
                b"killing\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                netname.as_mut_ptr(),
                std::ptr::null_mut(),
            );
        }
        7 | 8 => {
            BotAI_BotInitialChat(
                bs as *mut bot_state_s,
                b"camping\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                std::ptr::null_mut(),
            );
        }
        9 => {
            BotAI_BotInitialChat(
                bs as *mut bot_state_s,
                b"patrolling\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                std::ptr::null_mut(),
            );
        }
        4 => {
            BotAI_BotInitialChat(
                bs as *mut bot_state_s,
                b"capturingflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                std::ptr::null_mut(),
            );
        }
        5 => {
            BotAI_BotInitialChat(
                bs as *mut bot_state_s,
                b"rushingbase\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                std::ptr::null_mut(),
            );
        }
        6 => {
            BotAI_BotInitialChat(
                bs as *mut bot_state_s,
                b"returningflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                std::ptr::null_mut(),
            );
        }
        _ => {
            BotAI_BotInitialChat(
                bs as *mut bot_state_s,
                b"roaming\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                std::ptr::null_mut(),
            );
        }
    }
    //chat what the bot is doing
    trap_BotMatchVariable(
        match_0 as *mut libc::c_void,
        0 as i32,
        netname.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 256]>() as usize as i32,
    );
    client = crate::src::game::ai_dmq3::ClientFromName(netname.as_mut_ptr());
    trap_BotEnterChat((*bs).cs, client, 2 as i32);
}
/*
==================
BotMatch_WhatIsMyCommand
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotMatch_WhatIsMyCommand(
    mut bs: *mut bot_state_t,
    mut _match_0: *mut bot_match_t,
) {
    let mut netname: [libc::c_char; 36] = [0; 36];
    crate::src::game::ai_dmq3::ClientName(
        (*bs).client,
        netname.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 36]>() as usize as i32,
    );
    if Q_stricmp(netname.as_mut_ptr(), (*bs).teamleader.as_mut_ptr()) != 0 as i32 {
        return;
    }
    (*bs).forceorders = qtrue as i32;
}
/*
==================
BotNearestVisibleItem
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotNearestVisibleItem(
    mut bs: *mut bot_state_t,
    mut itemname: *mut libc::c_char,
    mut goal: *mut bot_goal_t,
) -> f32 {
    let mut i: i32 = 0;
    let mut name: [libc::c_char; 64] = [0; 64];
    let mut tmpgoal: bot_goal_t = bot_goal_t {
        origin: [0.; 3],
        areanum: 0,
        mins: [0.; 3],
        maxs: [0.; 3],
        entitynum: 0,
        number: 0,
        flags: 0,
        iteminfo: 0,
    };
    let mut dist: f32 = 0.;
    let mut bestdist: f32 = 0.;
    let mut dir: vec3_t = [0.; 3];
    let mut trace: bsp_trace_t = bsp_trace_t {
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
    bestdist = 999999 as i32 as f32;
    i = -(1 as i32);
    loop {
        i = trap_BotGetLevelItemGoal(
            i,
            itemname,
            &mut tmpgoal as *mut bot_goal_t as *mut libc::c_void,
        );
        trap_BotGoalName(
            tmpgoal.number,
            name.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 64]>() as usize as i32,
        );
        if !(Q_stricmp(itemname, name.as_mut_ptr()) != 0 as i32) {
            dir[0 as i32 as usize] =
                tmpgoal.origin[0 as i32 as usize] - (*bs).origin[0 as i32 as usize];
            dir[1 as i32 as usize] =
                tmpgoal.origin[1 as i32 as usize] - (*bs).origin[1 as i32 as usize];
            dir[2 as i32 as usize] =
                tmpgoal.origin[2 as i32 as usize] - (*bs).origin[2 as i32 as usize];
            dist = VectorLength(dir.as_mut_ptr() as *const vec_t);
            if dist < bestdist {
                //trace from start to end
                BotAI_Trace(
                    &mut trace as *mut _ as *mut bsp_trace_s,
                    (*bs).eye.as_mut_ptr(),
                    std::ptr::null_mut::<vec_t>(),
                    std::ptr::null_mut::<vec_t>(),
                    tmpgoal.origin.as_mut_ptr(),
                    (*bs).client,
                    1 as i32 | 0x10000 as i32,
                );
                if trace.fraction as f64 >= 1.0f64 {
                    bestdist = dist;
                    crate::stdlib::memcpy(
                        goal as *mut libc::c_void,
                        &mut tmpgoal as *mut bot_goal_t as *const libc::c_void,
                        ::std::mem::size_of::<bot_goal_t>() as usize,
                    );
                }
            }
        }
        if !(i > 0 as i32) {
            break;
        }
    }
    return bestdist;
}
/*
==================
BotMatch_WhereAreYou
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotMatch_WhereAreYou(
    mut bs: *mut bot_state_t,
    mut match_0: *mut bot_match_t,
) {
    let mut dist: f32 = 0.;
    let mut bestdist: f32 = 0.;
    let mut i: i32 = 0;
    let mut bestitem: i32 = 0;
    let mut redtt: i32 = 0;
    let mut bluett: i32 = 0;
    let mut client: i32 = 0;
    let mut goal: bot_goal_t = bot_goal_t {
        origin: [0.; 3],
        areanum: 0,
        mins: [0.; 3],
        maxs: [0.; 3],
        entitynum: 0,
        number: 0,
        flags: 0,
        iteminfo: 0,
    };
    let mut netname: [libc::c_char; 256] = [0; 256];
    let mut nearbyitems: [*mut libc::c_char; 18] = [
        b"Shotgun\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"Grenade Launcher\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"Rocket Launcher\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"Plasmagun\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"Railgun\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"Lightning Gun\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"BFG10K\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"Quad Damage\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"Regeneration\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"Battle Suit\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"Speed\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"Invisibility\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"Flight\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"Armor\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"Heavy Armor\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"Red Flag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"Blue Flag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        std::ptr::null_mut(),
    ];
    //
    if crate::src::game::ai_dmq3::TeamPlayIsOn() == 0 {
        return;
    }
    //if not addressed to this bot
    if BotAddressedToBot(bs, match_0) == 0 {
        return;
    }
    bestitem = -(1 as i32);
    bestdist = 999999 as i32 as f32;
    i = 0 as i32;
    while !nearbyitems[i as usize].is_null() {
        dist = BotNearestVisibleItem(bs, nearbyitems[i as usize], &mut goal);
        if dist < bestdist {
            bestdist = dist;
            bestitem = i
        }
        i += 1
    }
    if bestitem != -(1 as i32) {
        if crate::src::game::ai_dmq3::gametype == GT_CTF as i32 {
            redtt = trap_AAS_AreaTravelTimeToGoalArea(
                (*bs).areanum,
                (*bs).origin.as_mut_ptr(),
                crate::src::game::ai_dmq3::ctf_redflag.areanum,
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
            );
            bluett = trap_AAS_AreaTravelTimeToGoalArea(
                (*bs).areanum,
                (*bs).origin.as_mut_ptr(),
                crate::src::game::ai_dmq3::ctf_blueflag.areanum,
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
            );
            if (redtt as f64) < (redtt + bluett) as f64 * 0.4f64 {
                BotAI_BotInitialChat(
                    bs as *mut bot_state_s,
                    b"teamlocation\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    nearbyitems[bestitem as usize],
                    b"red\x00" as *const u8 as *const libc::c_char,
                    std::ptr::null_mut(),
                );
            } else if (bluett as f64) < (redtt + bluett) as f64 * 0.4f64 {
                BotAI_BotInitialChat(
                    bs as *mut bot_state_s,
                    b"teamlocation\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    nearbyitems[bestitem as usize],
                    b"blue\x00" as *const u8 as *const libc::c_char,
                    std::ptr::null_mut(),
                );
            } else {
                BotAI_BotInitialChat(
                    bs as *mut bot_state_s,
                    b"location\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    nearbyitems[bestitem as usize],
                    std::ptr::null_mut(),
                );
            }
        } else {
            BotAI_BotInitialChat(
                bs as *mut bot_state_s,
                b"location\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                nearbyitems[bestitem as usize],
                std::ptr::null_mut(),
            );
        }
        trap_BotMatchVariable(
            match_0 as *mut libc::c_void,
            0 as i32,
            netname.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 256]>() as usize as i32,
        );
        client = crate::src::game::ai_dmq3::ClientFromName(netname.as_mut_ptr());
        trap_BotEnterChat((*bs).cs, client, 2 as i32);
    };
}
/*
==================
BotMatch_LeadTheWay
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotMatch_LeadTheWay(
    mut bs: *mut bot_state_t,
    mut match_0: *mut bot_match_t,
) {
    let mut entinfo: aas_entityinfo_t = aas_entityinfo_t {
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
    let mut netname: [libc::c_char; 256] = [0; 256];
    let mut teammate: [libc::c_char; 256] = [0; 256];
    let mut client: i32 = 0;
    let mut areanum: i32 = 0;
    let mut other: i32 = 0;
    if crate::src::game::ai_dmq3::TeamPlayIsOn() == 0 {
        return;
    }
    //if not addressed to this bot
    if BotAddressedToBot(bs, match_0) == 0 {
        return;
    }
    //if someone asks for someone else
    if (*match_0).subtype & 2048 as i32 != 0 {
        //get the team mate name
        trap_BotMatchVariable(
            match_0 as *mut libc::c_void,
            4 as i32,
            teammate.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 256]>() as usize as i32,
        );
        client = FindClientByName(teammate.as_mut_ptr());
        //if this is the bot self
        if client == (*bs).client {
            other = qfalse as i32
        } else if crate::src::game::ai_dmq3::BotSameTeam(bs as *mut bot_state_s, client) == 0 {
            //FIXME: say "I don't help the enemy"
            return;
        } else {
            other = qtrue as i32
        }
    } else {
        //get the netname
        trap_BotMatchVariable(
            match_0 as *mut libc::c_void,
            0 as i32,
            netname.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 256]>() as usize as i32,
        );
        client = crate::src::game::ai_dmq3::ClientFromName(netname.as_mut_ptr());
        other = qfalse as i32
    }
    //if the bot doesn't know who to help (FindClientByName returned -1)
    if client < 0 as i32 {
        BotAI_BotInitialChat(
            bs as *mut bot_state_s,
            b"whois\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            netname.as_mut_ptr(),
            std::ptr::null_mut(),
        );
        trap_BotEnterChat((*bs).cs, (*bs).client, 1 as i32);
        return;
    }
    //
    (*bs).lead_teamgoal.entitynum = -(1 as i32);
    BotEntityInfo(client, &mut entinfo as *mut _ as *mut aas_entityinfo_s);
    //if info is valid (in PVS)
    if entinfo.valid != 0 {
        areanum = crate::src::game::ai_dmq3::BotPointAreaNum(entinfo.origin.as_mut_ptr());
        if areanum != 0 {
            // && trap_AAS_AreaReachability(areanum)) {
            (*bs).lead_teamgoal.entitynum = client;
            (*bs).lead_teamgoal.areanum = areanum;
            (*bs).lead_teamgoal.origin[0 as i32 as usize] = entinfo.origin[0 as i32 as usize];
            (*bs).lead_teamgoal.origin[1 as i32 as usize] = entinfo.origin[1 as i32 as usize];
            (*bs).lead_teamgoal.origin[2 as i32 as usize] = entinfo.origin[2 as i32 as usize];
            (*bs).lead_teamgoal.mins[0 as i32 as usize] = -(8 as i32) as vec_t;
            (*bs).lead_teamgoal.mins[1 as i32 as usize] = -(8 as i32) as vec_t;
            (*bs).lead_teamgoal.mins[2 as i32 as usize] = -(8 as i32) as vec_t;
            (*bs).lead_teamgoal.maxs[0 as i32 as usize] = 8 as i32 as vec_t;
            (*bs).lead_teamgoal.maxs[1 as i32 as usize] = 8 as i32 as vec_t;
            (*bs).lead_teamgoal.maxs[2 as i32 as usize] = 8 as i32 as vec_t
        }
    }
    if (*bs).teamgoal.entitynum < 0 as i32 {
        if other != 0 {
            BotAI_BotInitialChat(
                bs as *mut bot_state_s,
                b"whereis\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                teammate.as_mut_ptr(),
                std::ptr::null_mut(),
            );
        } else {
            BotAI_BotInitialChat(
                bs as *mut bot_state_s,
                b"whereareyou\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                netname.as_mut_ptr(),
                std::ptr::null_mut(),
            );
        }
        trap_BotEnterChat((*bs).cs, (*bs).client, 1 as i32);
        return;
    }
    (*bs).lead_teammate = client;
    (*bs).lead_time = floattime + 600 as i32 as f32;
    (*bs).leadvisible_time = 0 as i32 as f32;
    (*bs).leadmessage_time = -(floattime
        + 2 as i32 as f32 * ((libc::rand() & 0x7fff as i32) as f32 / 0x7fff as i32 as f32));
}
/*
==================
BotMatch_Kill
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotMatch_Kill(mut bs: *mut bot_state_t, mut match_0: *mut bot_match_t) {
    let mut enemy: [libc::c_char; 256] = [0; 256];
    let mut netname: [libc::c_char; 256] = [0; 256];
    let mut client: i32 = 0;
    if crate::src::game::ai_dmq3::TeamPlayIsOn() == 0 {
        return;
    }
    //if not addressed to this bot
    if BotAddressedToBot(bs, match_0) == 0 {
        return;
    }
    trap_BotMatchVariable(
        match_0 as *mut libc::c_void,
        4 as i32,
        enemy.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 256]>() as usize as i32,
    );
    //
    client = FindEnemyByName(bs, enemy.as_mut_ptr());
    if client < 0 as i32 {
        BotAI_BotInitialChat(
            bs as *mut bot_state_s,
            b"whois\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            enemy.as_mut_ptr(),
            std::ptr::null_mut(),
        );
        trap_BotMatchVariable(
            match_0 as *mut libc::c_void,
            0 as i32,
            netname.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 256]>() as usize as i32,
        );
        client = crate::src::game::ai_dmq3::ClientFromName(netname.as_mut_ptr());
        trap_BotEnterChat((*bs).cs, client, 2 as i32);
        return;
    }
    (*bs).teamgoal.entitynum = client;
    //set the time to send a message to the team mates
    (*bs).teammessage_time = floattime
        + 2 as i32 as f32 * ((libc::rand() & 0x7fff as i32) as f32 / 0x7fff as i32 as f32);
    //set the ltg type
    (*bs).ltgtype = 11 as i32;
    //set the team goal time
    (*bs).teamgoal_time = floattime + 180 as i32 as f32;
    //
    crate::src::game::ai_dmq3::BotSetTeamStatus(bs as *mut bot_state_s);
    //DEBUG
}
/*
==================
BotMatch_CTF
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotMatch_CTF(mut bs: *mut bot_state_t, mut match_0: *mut bot_match_t) {
    let mut flag: [libc::c_char; 128] = [0; 128];
    let mut netname: [libc::c_char; 36] = [0; 36];
    if crate::src::game::ai_dmq3::gametype == GT_CTF as i32 {
        trap_BotMatchVariable(
            match_0 as *mut libc::c_void,
            1 as i32,
            flag.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 128]>() as usize as i32,
        );
        if (*match_0).subtype & 4096 as i32 != 0 {
            if Q_stricmp(
                flag.as_mut_ptr(),
                b"red\x00" as *const u8 as *const libc::c_char,
            ) == 0
            {
                (*bs).redflagstatus = 1 as i32;
                if crate::src::game::ai_dmq3::BotTeam(bs as *mut bot_state_s) == TEAM_BLUE as i32 {
                    trap_BotMatchVariable(
                        match_0 as *mut libc::c_void,
                        0 as i32,
                        netname.as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 36]>() as usize as i32,
                    );
                    (*bs).flagcarrier =
                        crate::src::game::ai_dmq3::ClientFromName(netname.as_mut_ptr())
                }
            } else {
                (*bs).blueflagstatus = 1 as i32;
                if crate::src::game::ai_dmq3::BotTeam(bs as *mut bot_state_s) == TEAM_RED as i32 {
                    trap_BotMatchVariable(
                        match_0 as *mut libc::c_void,
                        0 as i32,
                        netname.as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 36]>() as usize as i32,
                    );
                    (*bs).flagcarrier =
                        crate::src::game::ai_dmq3::ClientFromName(netname.as_mut_ptr())
                }
            }
            (*bs).flagstatuschanged = 1 as i32;
            (*bs).lastflagcapture_time = floattime
        } else if (*match_0).subtype & 8192 as i32 != 0 {
            (*bs).redflagstatus = 0 as i32;
            (*bs).blueflagstatus = 0 as i32;
            (*bs).flagcarrier = 0 as i32;
            (*bs).flagstatuschanged = 1 as i32
        } else if (*match_0).subtype & 16384 as i32 != 0 {
            if Q_stricmp(
                flag.as_mut_ptr(),
                b"red\x00" as *const u8 as *const libc::c_char,
            ) == 0
            {
                (*bs).redflagstatus = 0 as i32
            } else {
                (*bs).blueflagstatus = 0 as i32
            }
            (*bs).flagstatuschanged = 1 as i32
        }
    };
}
#[no_mangle]

pub unsafe extern "C" fn BotMatch_EnterGame(
    mut _bs: *mut bot_state_t,
    mut match_0: *mut bot_match_t,
) {
    let mut client: i32 = 0;
    let mut netname: [libc::c_char; 36] = [0; 36];
    trap_BotMatchVariable(
        match_0 as *mut libc::c_void,
        0 as i32,
        netname.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 36]>() as usize as i32,
    );
    client = FindClientByName(netname.as_mut_ptr());
    if client >= 0 as i32 {
        notleader[client as usize] = qfalse as i32
    };
    //NOTE: eliza chats will catch this
    //Com_sprintf(buf, sizeof(buf), "heya %s", netname);
    //EA_Say(bs->client, buf);
}
#[no_mangle]

pub unsafe extern "C" fn BotMatch_NewLeader(
    mut bs: *mut bot_state_t,
    mut match_0: *mut bot_match_t,
) {
    let mut client: i32 = 0;
    let mut netname: [libc::c_char; 36] = [0; 36];
    trap_BotMatchVariable(
        match_0 as *mut libc::c_void,
        0 as i32,
        netname.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 36]>() as usize as i32,
    );
    client = FindClientByName(netname.as_mut_ptr());
    if crate::src::game::ai_dmq3::BotSameTeam(bs as *mut bot_state_s, client) == 0 {
        return;
    }
    Q_strncpyz(
        (*bs).teamleader.as_mut_ptr(),
        netname.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 36]>() as usize as i32,
    );
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
//
/* ****************************************************************************
 * name:		ai_cmd.h
 *
 * desc:		Quake3 bot AI
 *
 * $Archive: /source/code/botai/ai_chat.c $
 *
 *****************************************************************************/
/*
==================
BotMatchMessage
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotMatchMessage(
    mut bs: *mut bot_state_t,
    mut message: *mut libc::c_char,
) -> i32 {
    let mut match_0: bot_match_t = bot_match_t {
        string: [0; 256],
        type_0: 0,
        subtype: 0,
        variables: [bot_matchvariable_t {
            offset: 0,
            length: 0,
        }; 8],
    };
    match_0.type_0 = 0 as i32;
    //if it is an unknown message
    if trap_BotFindMatch(
        message,
        &mut match_0 as *mut bot_match_t as *mut libc::c_void,
        (2 as i32 | 4 as i32 | 256 as i32) as usize,
    ) == 0
    {
        return qfalse as i32;
    }
    let mut current_block_33: u64;
    //react to the found message
    match match_0.type_0 {
        3 => {
            current_block_33 = 7502529970979898288;
        }
        4 => {
            //someone calling for company
            current_block_33 = 7502529970979898288;
        }
        5 => {
            //teamplay defend a key area
            BotMatch_DefendKeyArea(bs, &mut match_0);
            current_block_33 = 2631791190359682872;
        }
        20 => {
            //camp somewhere
            BotMatch_Camp(bs, &mut match_0);
            current_block_33 = 2631791190359682872;
        }
        22 => {
            //patrol between several key areas
            BotMatch_Patrol(bs, &mut match_0);
            current_block_33 = 2631791190359682872;
        }
        7 => {
            //ctf get the enemy flag
            BotMatch_GetFlag(bs, &mut match_0);
            current_block_33 = 2631791190359682872;
        }
        6 => {
            //ctf rush to the base
            BotMatch_RushBase(bs, &mut match_0);
            current_block_33 = 2631791190359682872;
        }
        27 => {
            //CTF & 1FCTF
            BotMatch_ReturnFlag(bs, &mut match_0);
            current_block_33 = 2631791190359682872;
        }
        30 => {
            //CTF & 1FCTF & Obelisk & Harvester
            BotMatch_TaskPreference(bs, &mut match_0);
            current_block_33 = 2631791190359682872;
        }
        300 => {
            //CTF & 1FCTF
            BotMatch_CTF(bs, &mut match_0);
            current_block_33 = 2631791190359682872;
        }
        24 => {
            BotMatch_GetItem(bs, &mut match_0);
            current_block_33 = 2631791190359682872;
        }
        13 => {
            //join a sub team
            BotMatch_JoinSubteam(bs, &mut match_0);
            current_block_33 = 2631791190359682872;
        }
        14 => {
            //leave a sub team
            BotMatch_LeaveSubteam(bs, &mut match_0);
            current_block_33 = 2631791190359682872;
        }
        29 => {
            BotMatch_WhichTeam(bs, &mut match_0);
            current_block_33 = 2631791190359682872;
        }
        21 => {
            //remember a check point
            BotMatch_CheckPoint(bs, &mut match_0);
            current_block_33 = 2631791190359682872;
        }
        15 => {
            //start the creation of a new formation
            trap_EA_SayTeam(
                (*bs).client,
                b"the part of my brain to create formations has been damaged\x00" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            );
            current_block_33 = 2631791190359682872;
        }
        16 => {
            //tell someone his/her position in the formation
            trap_EA_SayTeam(
                (*bs).client,
                b"the part of my brain to create formations has been damaged\x00" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            );
            current_block_33 = 2631791190359682872;
        }
        17 => {
            //set the formation space
            BotMatch_FormationSpace(bs, &mut match_0);
            current_block_33 = 2631791190359682872;
        }
        19 => {
            //dismiss someone
            BotMatch_Dismiss(bs, &mut match_0);
            current_block_33 = 2631791190359682872;
        }
        8 => {
            //someone will become the team leader
            BotMatch_StartTeamLeaderShip(bs, &mut match_0);
            current_block_33 = 2631791190359682872;
        }
        9 => {
            //someone will stop being the team leader
            BotMatch_StopTeamLeaderShip(bs, &mut match_0);
            current_block_33 = 2631791190359682872;
        }
        10 => {
            BotMatch_WhoIsTeamLeader(bs, &mut match_0);
            current_block_33 = 2631791190359682872;
        }
        12 => {
            //ask a bot what he/she is doing
            BotMatch_WhatAreYouDoing(bs, &mut match_0);
            current_block_33 = 2631791190359682872;
        }
        28 => {
            BotMatch_WhatIsMyCommand(bs, &mut match_0);
            current_block_33 = 2631791190359682872;
        }
        26 => {
            BotMatch_WhereAreYou(bs, &mut match_0);
            current_block_33 = 2631791190359682872;
        }
        23 => {
            BotMatch_LeadTheWay(bs, &mut match_0);
            current_block_33 = 2631791190359682872;
        }
        25 => {
            BotMatch_Kill(bs, &mut match_0);
            current_block_33 = 2631791190359682872;
        }
        2 => {
            //someone entered the game
            BotMatch_EnterGame(bs, &mut match_0);
            current_block_33 = 2631791190359682872;
        }
        1 => {
            BotMatch_NewLeader(bs, &mut match_0);
            current_block_33 = 2631791190359682872;
        }
        18 | 11 => {
            current_block_33 = 2631791190359682872;
        }
        33 => {
            BotMatch_Suicide(bs, &mut match_0);
            current_block_33 = 2631791190359682872;
        }
        _ => {
            BotAI_Print(
                1 as i32,
                b"unknown match type\n\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            current_block_33 = 2631791190359682872;
        }
    }
    match current_block_33 {
        7502529970979898288 =>
        //someone calling for help
        {
            BotMatch_HelpAccompany(bs, &mut match_0);
        }
        _ => {}
    }
    return qtrue as i32;
}

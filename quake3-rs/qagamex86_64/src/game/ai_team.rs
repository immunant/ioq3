use ::libc;

pub mod stdlib_h {
    #[inline]

    pub unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
        return ::libc::strtol(
            __nptr,
            0 as *mut libc::c_void as *mut *mut libc::c_char,
            10 as libc::c_int,
        ) as libc::c_int;
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
pub use crate::src::qcommon::q_shared::Info_ValueForKey;
pub use crate::src::qcommon::q_shared::Q_stricmp;
pub use crate::src::qcommon::q_shared::TR_GRAVITY;
pub use crate::src::qcommon::q_shared::TR_INTERPOLATE;
pub use crate::src::qcommon::q_shared::TR_LINEAR;
pub use crate::src::qcommon::q_shared::TR_LINEAR_STOP;
pub use crate::src::qcommon::q_shared::TR_SINE;
pub use crate::src::qcommon::q_shared::TR_STATIONARY;

pub use crate::be_ai_goal_h::bot_goal_s;
pub use crate::be_ai_goal_h::bot_goal_t;
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

pub use crate::src::game::ai_main::bot_activategoal_s;
pub use crate::src::game::ai_main::bot_activategoal_t;
pub use crate::src::game::ai_main::bot_state_s;
pub use crate::src::game::ai_main::bot_state_t;
pub use crate::src::game::ai_main::bot_waypoint_s;
pub use crate::src::game::ai_main::bot_waypoint_t;
pub use crate::src::game::ai_main::floattime;
pub use crate::src::game::ai_main::BotAI_BotInitialChat;
pub use crate::src::game::ai_main::BotAI_GetClientState;
pub use crate::src::game::ai_team::stdlib_h::atoi;

pub use crate::src::game::g_main::g_entities;
pub use crate::src::game::g_main::level;
pub use crate::src::game::g_syscalls::trap_AAS_AreaTravelTimeToGoalArea;
pub use crate::src::game::g_syscalls::trap_BotEnterChat;
pub use crate::src::game::g_syscalls::trap_BotGetChatMessage;
pub use crate::src::game::g_syscalls::trap_BotQueueConsoleMessage;
pub use crate::src::game::g_syscalls::trap_GetConfigstring;

pub use ::libc::rand;

pub use ::libc::strtol;
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
 * name:		ai_team.c
 *
 * desc:		Quake3 bot AI
 *
 * $Archive: /MissionPack/code/game/ai_team.c $
 *
 *****************************************************************************/
//
// for the voice chats
//ctf task preferences for a client

pub type bot_ctftaskpreference_t = bot_ctftaskpreference_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bot_ctftaskpreference_s {
    pub name: [libc::c_char; 36],
    pub preference: libc::c_int,
}
#[no_mangle]

pub static mut ctftaskpreferences: [bot_ctftaskpreference_t; 64] = [bot_ctftaskpreference_t {
    name: [0; 36],
    preference: 0,
}; 64];
/*
==================
BotValidTeamLeader
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotValidTeamLeader(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
) -> libc::c_int {
    if crate::stdlib::strlen((*bs).teamleader.as_mut_ptr()) == 0 {
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    }
    if crate::src::game::ai_dmq3::ClientFromName((*bs).teamleader.as_mut_ptr())
        == -(1 as libc::c_int)
    {
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    }
    return crate::src::qcommon::q_shared::qtrue as libc::c_int;
}
/*
==================
BotNumTeamMates
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotNumTeamMates(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut numplayers: libc::c_int = 0;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    numplayers = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < crate::src::game::g_main::level.maxclients {
        crate::src::game::g_syscalls::trap_GetConfigstring(
            32 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int + i,
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
        );
        //if no config string or no name
        if !(crate::stdlib::strlen(buf.as_mut_ptr()) == 0
            || crate::stdlib::strlen(crate::src::qcommon::q_shared::Info_ValueForKey(
                buf.as_mut_ptr(),
                b"n\x00" as *const u8 as *const libc::c_char,
            )) == 0)
        {
            //skip spectators
            if !(atoi(crate::src::qcommon::q_shared::Info_ValueForKey(
                buf.as_mut_ptr(),
                b"t\x00" as *const u8 as *const libc::c_char,
            )) == crate::bg_public_h::TEAM_SPECTATOR as libc::c_int)
            {
                //
                if crate::src::game::ai_dmq3::BotSameTeam(
                    bs as *mut crate::src::game::ai_main::bot_state_s,
                    i,
                ) != 0
                {
                    numplayers += 1
                }
            }
        }
        i += 1
    }
    return numplayers;
}
/*
==================
BotClientTravelTimeToGoal
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotClientTravelTimeToGoal(
    mut client: libc::c_int,
    mut goal: *mut crate::be_ai_goal_h::bot_goal_t,
) -> libc::c_int {
    let mut ps: crate::src::qcommon::q_shared::playerState_t =
        crate::src::qcommon::q_shared::playerState_t {
            commandTime: 0,
            pm_type: 0,
            bobCycle: 0,
            pm_flags: 0,
            pm_time: 0,
            origin: [0.; 3],
            velocity: [0.; 3],
            weaponTime: 0,
            gravity: 0,
            speed: 0,
            delta_angles: [0; 3],
            groundEntityNum: 0,
            legsTimer: 0,
            legsAnim: 0,
            torsoTimer: 0,
            torsoAnim: 0,
            movementDir: 0,
            grapplePoint: [0.; 3],
            eFlags: 0,
            eventSequence: 0,
            events: [0; 2],
            eventParms: [0; 2],
            externalEvent: 0,
            externalEventParm: 0,
            externalEventTime: 0,
            clientNum: 0,
            weapon: 0,
            weaponstate: 0,
            viewangles: [0.; 3],
            viewheight: 0,
            damageEvent: 0,
            damageYaw: 0,
            damagePitch: 0,
            damageCount: 0,
            stats: [0; 16],
            persistant: [0; 16],
            powerups: [0; 16],
            ammo: [0; 16],
            generic1: 0,
            loopSound: 0,
            jumppad_ent: 0,
            ping: 0,
            pmove_framecount: 0,
            jumppad_frame: 0,
            entityEventSequence: 0,
        };
    let mut areanum: libc::c_int = 0;
    if crate::src::game::ai_main::BotAI_GetClientState(
        client,
        &mut ps as *mut _ as *mut crate::src::qcommon::q_shared::playerState_s,
    ) != 0
    {
        areanum = crate::src::game::ai_dmq3::BotPointAreaNum(ps.origin.as_mut_ptr())
    } else {
        areanum = 0 as libc::c_int
    }
    if areanum == 0 {
        return 1 as libc::c_int;
    }
    return crate::src::game::g_syscalls::trap_AAS_AreaTravelTimeToGoalArea(
        areanum,
        ps.origin.as_mut_ptr(),
        (*goal).areanum,
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
/*
==================
BotSortTeamMatesByBaseTravelTime
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotSortTeamMatesByBaseTravelTime(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut teammates: *mut libc::c_int,
    mut maxteammates: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut numteammates: libc::c_int = 0;
    let mut traveltime: libc::c_int = 0;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut traveltimes: [libc::c_int; 64] = [0; 64];
    let mut goal: *mut crate::be_ai_goal_h::bot_goal_t = 0 as *mut crate::be_ai_goal_h::bot_goal_t;
    if crate::src::game::ai_dmq3::gametype == crate::bg_public_h::GT_CTF as libc::c_int {
        if crate::src::game::ai_dmq3::BotTeam(bs as *mut crate::src::game::ai_main::bot_state_s)
            == crate::bg_public_h::TEAM_RED as libc::c_int
        {
            goal = &mut crate::src::game::ai_dmq3::ctf_redflag
        } else {
            goal = &mut crate::src::game::ai_dmq3::ctf_blueflag
        }
    }
    numteammates = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < crate::src::game::g_main::level.maxclients {
        crate::src::game::g_syscalls::trap_GetConfigstring(
            32 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int + i,
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
        );
        //if no config string or no name
        if !(crate::stdlib::strlen(buf.as_mut_ptr()) == 0
            || crate::stdlib::strlen(crate::src::qcommon::q_shared::Info_ValueForKey(
                buf.as_mut_ptr(),
                b"n\x00" as *const u8 as *const libc::c_char,
            )) == 0)
        {
            //skip spectators
            if !(atoi(crate::src::qcommon::q_shared::Info_ValueForKey(
                buf.as_mut_ptr(),
                b"t\x00" as *const u8 as *const libc::c_char,
            )) == crate::bg_public_h::TEAM_SPECTATOR as libc::c_int)
            {
                //
                if crate::src::game::ai_dmq3::BotSameTeam(
                    bs as *mut crate::src::game::ai_main::bot_state_s,
                    i,
                ) != 0
                    && !goal.is_null()
                {
                    //
                    traveltime = BotClientTravelTimeToGoal(i, goal);
                    //
                    j = 0 as libc::c_int;
                    while j < numteammates {
                        if traveltime < traveltimes[j as usize] {
                            k = numteammates;
                            while k > j {
                                traveltimes[k as usize] =
                                    traveltimes[(k - 1 as libc::c_int) as usize];
                                *teammates.offset(k as isize) =
                                    *teammates.offset((k - 1 as libc::c_int) as isize);
                                k -= 1
                            }
                            break;
                        } else {
                            j += 1
                        }
                    }
                    traveltimes[j as usize] = traveltime;
                    *teammates.offset(j as isize) = i;
                    numteammates += 1;
                    if numteammates >= maxteammates {
                        break;
                    }
                }
            }
        }
        i += 1
    }
    return numteammates;
}
/*
==================
BotSetTeamMateTaskPreference
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotSetTeamMateTaskPreference(
    mut _bs: *mut crate::src::game::ai_main::bot_state_t,
    mut teammate: libc::c_int,
    mut preference: libc::c_int,
) {
    let mut teammatename: [libc::c_char; 36] = [0; 36];
    ctftaskpreferences[teammate as usize].preference = preference;
    crate::src::game::ai_dmq3::ClientName(
        teammate,
        teammatename.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
    );
    ::libc::strcpy(
        ctftaskpreferences[teammate as usize].name.as_mut_ptr(),
        teammatename.as_mut_ptr(),
    );
}
/*
==================
BotGetTeamMateTaskPreference
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotGetTeamMateTaskPreference(
    mut _bs: *mut crate::src::game::ai_main::bot_state_t,
    mut teammate: libc::c_int,
) -> libc::c_int {
    let mut teammatename: [libc::c_char; 36] = [0; 36];
    if ctftaskpreferences[teammate as usize].preference == 0 {
        return 0 as libc::c_int;
    }
    crate::src::game::ai_dmq3::ClientName(
        teammate,
        teammatename.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
    );
    if crate::src::qcommon::q_shared::Q_stricmp(
        teammatename.as_mut_ptr(),
        ctftaskpreferences[teammate as usize].name.as_mut_ptr(),
    ) != 0
    {
        return 0 as libc::c_int;
    }
    return ctftaskpreferences[teammate as usize].preference;
}
/*
==================
BotSortTeamMatesByTaskPreference
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotSortTeamMatesByTaskPreference(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut teammates: *mut libc::c_int,
    mut numteammates: libc::c_int,
) -> libc::c_int {
    let mut defenders: [libc::c_int; 64] = [0; 64];
    let mut numdefenders: libc::c_int = 0;
    let mut attackers: [libc::c_int; 64] = [0; 64];
    let mut numattackers: libc::c_int = 0;
    let mut roamers: [libc::c_int; 64] = [0; 64];
    let mut numroamers: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut preference: libc::c_int = 0;
    numroamers = 0 as libc::c_int;
    numattackers = numroamers;
    numdefenders = numattackers;
    i = 0 as libc::c_int;
    while i < numteammates {
        preference = BotGetTeamMateTaskPreference(bs, *teammates.offset(i as isize));
        if preference & 1 as libc::c_int != 0 {
            let fresh0 = numdefenders;
            numdefenders = numdefenders + 1;
            defenders[fresh0 as usize] = *teammates.offset(i as isize)
        } else if preference & 2 as libc::c_int != 0 {
            let fresh1 = numattackers;
            numattackers = numattackers + 1;
            attackers[fresh1 as usize] = *teammates.offset(i as isize)
        } else {
            let fresh2 = numroamers;
            numroamers = numroamers + 1;
            roamers[fresh2 as usize] = *teammates.offset(i as isize)
        }
        i += 1
    }
    numteammates = 0 as libc::c_int;
    //defenders at the front of the list
    crate::stdlib::memcpy(
        &mut *teammates.offset(numteammates as isize) as *mut libc::c_int as *mut libc::c_void,
        defenders.as_mut_ptr() as *const libc::c_void,
        (numdefenders as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    numteammates += numdefenders;
    //roamers in the middle
    crate::stdlib::memcpy(
        &mut *teammates.offset(numteammates as isize) as *mut libc::c_int as *mut libc::c_void,
        roamers.as_mut_ptr() as *const libc::c_void,
        (numroamers as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    numteammates += numroamers;
    //attacker in the back of the list
    crate::stdlib::memcpy(
        &mut *teammates.offset(numteammates as isize) as *mut libc::c_int as *mut libc::c_void,
        attackers.as_mut_ptr() as *const libc::c_void,
        (numattackers as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    numteammates += numattackers;
    return numteammates;
}
/*
==================
BotSayTeamOrders
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotSayTeamOrderAlways(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut toclient: libc::c_int,
) {
    let mut teamchat: [libc::c_char; 256] = [0; 256];
    let mut buf: [libc::c_char; 256] = [0; 256];
    let mut name: [libc::c_char; 36] = [0; 36];
    //if the bot is talking to itself
    if (*bs).client == toclient {
        //don't show the message just put it in the console message queue
        crate::src::game::g_syscalls::trap_BotGetChatMessage(
            (*bs).cs,
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
        );
        crate::src::game::ai_dmq3::ClientName(
            (*bs).client,
            name.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
        );
        crate::src::qcommon::q_shared::Com_sprintf(
            teamchat.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
            b"\x19(%s\x19)\x19: %s\x00" as *const u8 as *const libc::c_char,
            name.as_mut_ptr(),
            buf.as_mut_ptr(),
        );
        crate::src::game::g_syscalls::trap_BotQueueConsoleMessage(
            (*bs).cs,
            1 as libc::c_int,
            teamchat.as_mut_ptr(),
        );
    } else {
        crate::src::game::g_syscalls::trap_BotEnterChat((*bs).cs, toclient, 2 as libc::c_int);
    };
}
/*
==================
BotSayTeamOrders
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotSayTeamOrder(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut toclient: libc::c_int,
) {
    BotSayTeamOrderAlways(bs, toclient);
}
/*
==================
BotVoiceChat
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotVoiceChat(
    mut _bs: *mut crate::src::game::ai_main::bot_state_t,
    mut _toclient: libc::c_int,
    mut _voicechat: *mut libc::c_char,
) {
}
/*
==================
BotVoiceChatOnly
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotVoiceChatOnly(
    mut _bs: *mut crate::src::game::ai_main::bot_state_t,
    mut _toclient: libc::c_int,
    mut _voicechat: *mut libc::c_char,
) {
}
/*
==================
BotSayVoiceTeamOrder
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotSayVoiceTeamOrder(
    mut _bs: *mut crate::src::game::ai_main::bot_state_t,
    mut _toclient: libc::c_int,
    mut _voicechat: *mut libc::c_char,
) {
}
/*
==================
BotCTFOrders
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotCTFOrders_BothFlagsNotAtBase(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
) {
    let mut numteammates: libc::c_int = 0;
    let mut defenders: libc::c_int = 0;
    let mut attackers: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut other: libc::c_int = 0;
    let mut teammates: [libc::c_int; 64] = [
        0 as libc::c_int,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    let mut name: [libc::c_char; 36] = [0; 36];
    let mut carriername: [libc::c_char; 36] = [0; 36];
    numteammates = BotSortTeamMatesByBaseTravelTime(
        bs,
        teammates.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_int; 64]>() as libc::c_ulong as libc::c_int,
    );
    BotSortTeamMatesByTaskPreference(bs, teammates.as_mut_ptr(), numteammates);
    //different orders based on the number of team mates
    match (*bs).numteammates {
        1 => {}
        2 => {
            //tell the one not carrying the flag to attack the enemy base
            if teammates[0 as libc::c_int as usize] != (*bs).flagcarrier {
                other = teammates[0 as libc::c_int as usize]
            } else {
                other = teammates[1 as libc::c_int as usize]
            }
            crate::src::game::ai_dmq3::ClientName(
                other,
                name.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
            );
            crate::src::game::ai_main::BotAI_BotInitialChat(
                bs as *mut crate::src::game::ai_main::bot_state_s,
                b"cmd_getflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                name.as_mut_ptr(),
                0 as *mut libc::c_void,
            );
            BotSayTeamOrder(bs, other);
            BotSayVoiceTeamOrder(
                bs,
                other,
                b"getflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        }
        3 => {
            //tell the one closest to the base not carrying the flag to accompany the flag carrier
            if teammates[0 as libc::c_int as usize] != (*bs).flagcarrier {
                other = teammates[0 as libc::c_int as usize]
            } else {
                other = teammates[1 as libc::c_int as usize]
            }
            crate::src::game::ai_dmq3::ClientName(
                other,
                name.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
            );
            if (*bs).flagcarrier != -(1 as libc::c_int) {
                crate::src::game::ai_dmq3::ClientName(
                    (*bs).flagcarrier,
                    carriername.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
                );
                if (*bs).flagcarrier == (*bs).client {
                    crate::src::game::ai_main::BotAI_BotInitialChat(
                        bs as *mut crate::src::game::ai_main::bot_state_s,
                        b"cmd_accompanyme\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        name.as_mut_ptr(),
                        0 as *mut libc::c_void,
                    );
                    BotSayVoiceTeamOrder(
                        bs,
                        other,
                        b"followme\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    );
                } else {
                    crate::src::game::ai_main::BotAI_BotInitialChat(
                        bs as *mut crate::src::game::ai_main::bot_state_s,
                        b"cmd_accompany\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        name.as_mut_ptr(),
                        carriername.as_mut_ptr(),
                        0 as *mut libc::c_void,
                    );
                    BotSayVoiceTeamOrder(
                        bs,
                        other,
                        b"followflagcarrier\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    );
                }
            } else {
                //
                crate::src::game::ai_main::BotAI_BotInitialChat(
                    bs as *mut crate::src::game::ai_main::bot_state_s,
                    b"cmd_getflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    name.as_mut_ptr(),
                    0 as *mut libc::c_void,
                );
                BotSayVoiceTeamOrder(
                    bs,
                    other,
                    b"getflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
            }
            BotSayTeamOrder(bs, other);
            //tell the one furthest from the the base not carrying the flag to get the enemy flag
            if teammates[2 as libc::c_int as usize] != (*bs).flagcarrier {
                other = teammates[2 as libc::c_int as usize]
            } else {
                other = teammates[1 as libc::c_int as usize]
            }
            crate::src::game::ai_dmq3::ClientName(
                other,
                name.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
            );
            crate::src::game::ai_main::BotAI_BotInitialChat(
                bs as *mut crate::src::game::ai_main::bot_state_s,
                b"cmd_getflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                name.as_mut_ptr(),
                0 as *mut libc::c_void,
            );
            BotSayTeamOrder(bs, other);
            BotSayVoiceTeamOrder(
                bs,
                other,
                b"returnflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        }
        _ => {
            defenders = (numteammates as libc::c_float as libc::c_int as libc::c_double * 0.4f64
                + 0.5f64) as libc::c_int;
            if defenders > 4 as libc::c_int {
                defenders = 4 as libc::c_int
            }
            attackers = (numteammates as libc::c_float as libc::c_int as libc::c_double * 0.5f64
                + 0.5f64) as libc::c_int;
            if attackers > 5 as libc::c_int {
                attackers = 5 as libc::c_int
            }
            if (*bs).flagcarrier != -(1 as libc::c_int) {
                crate::src::game::ai_dmq3::ClientName(
                    (*bs).flagcarrier,
                    carriername.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
                );
                i = 0 as libc::c_int;
                while i < defenders {
                    //
                    if !(teammates[i as usize] == (*bs).flagcarrier) {
                        //
                        crate::src::game::ai_dmq3::ClientName(
                            teammates[i as usize],
                            name.as_mut_ptr(),
                            ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong
                                as libc::c_int,
                        );
                        if (*bs).flagcarrier == (*bs).client {
                            crate::src::game::ai_main::BotAI_BotInitialChat(
                                bs as *mut crate::src::game::ai_main::bot_state_s,
                                b"cmd_accompanyme\x00" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                                name.as_mut_ptr(),
                                0 as *mut libc::c_void,
                            );
                            BotSayVoiceTeamOrder(
                                bs,
                                teammates[i as usize],
                                b"followme\x00" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                            );
                        } else {
                            crate::src::game::ai_main::BotAI_BotInitialChat(
                                bs as *mut crate::src::game::ai_main::bot_state_s,
                                b"cmd_accompany\x00" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                                name.as_mut_ptr(),
                                carriername.as_mut_ptr(),
                                0 as *mut libc::c_void,
                            );
                            BotSayVoiceTeamOrder(
                                bs,
                                teammates[i as usize],
                                b"followflagcarrier\x00" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                            );
                        }
                        BotSayTeamOrder(bs, teammates[i as usize]);
                    }
                    i += 1
                }
            } else {
                i = 0 as libc::c_int;
                while i < defenders {
                    //
                    if !(teammates[i as usize] == (*bs).flagcarrier) {
                        //
                        crate::src::game::ai_dmq3::ClientName(
                            teammates[i as usize],
                            name.as_mut_ptr(),
                            ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong
                                as libc::c_int,
                        );
                        crate::src::game::ai_main::BotAI_BotInitialChat(
                            bs as *mut crate::src::game::ai_main::bot_state_s,
                            b"cmd_getflag\x00" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                            name.as_mut_ptr(),
                            0 as *mut libc::c_void,
                        );
                        BotSayVoiceTeamOrder(
                            bs,
                            teammates[i as usize],
                            b"getflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        );
                        BotSayTeamOrder(bs, teammates[i as usize]);
                    }
                    i += 1
                }
            }
            i = 0 as libc::c_int;
            while i < attackers {
                //
                if !(teammates[(numteammates - i - 1 as libc::c_int) as usize] == (*bs).flagcarrier)
                {
                    //
                    crate::src::game::ai_dmq3::ClientName(
                        teammates[(numteammates - i - 1 as libc::c_int) as usize],
                        name.as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
                    );
                    crate::src::game::ai_main::BotAI_BotInitialChat(
                        bs as *mut crate::src::game::ai_main::bot_state_s,
                        b"cmd_getflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        name.as_mut_ptr(),
                        0 as *mut libc::c_void,
                    );
                    BotSayTeamOrder(
                        bs,
                        teammates[(numteammates - i - 1 as libc::c_int) as usize],
                    );
                    BotSayVoiceTeamOrder(
                        bs,
                        teammates[(numteammates - i - 1 as libc::c_int) as usize],
                        b"returnflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    );
                }
                i += 1
            }
        }
    };
}
/*
==================
BotCTFOrders
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotCTFOrders_FlagNotAtBase(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
) {
    let mut numteammates: libc::c_int = 0;
    let mut defenders: libc::c_int = 0;
    let mut attackers: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut teammates: [libc::c_int; 64] = [0; 64];
    let mut name: [libc::c_char; 36] = [0; 36];
    numteammates = BotSortTeamMatesByBaseTravelTime(
        bs,
        teammates.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_int; 64]>() as libc::c_ulong as libc::c_int,
    );
    BotSortTeamMatesByTaskPreference(bs, teammates.as_mut_ptr(), numteammates);
    //passive strategy
    if (*bs).ctfstrategy & 1 as libc::c_int == 0 {
        //different orders based on the number of team mates
        match (*bs).numteammates {
            1 => {}
            2 => {
                // keep one near the base for when the flag is returned
                crate::src::game::ai_dmq3::ClientName(
                    teammates[0 as libc::c_int as usize],
                    name.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
                );
                crate::src::game::ai_main::BotAI_BotInitialChat(
                    bs as *mut crate::src::game::ai_main::bot_state_s,
                    b"cmd_defendbase\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    name.as_mut_ptr(),
                    0 as *mut libc::c_void,
                );
                BotSayTeamOrder(bs, teammates[0 as libc::c_int as usize]);
                BotSayVoiceTeamOrder(
                    bs,
                    teammates[0 as libc::c_int as usize],
                    b"defend\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
                //
                crate::src::game::ai_dmq3::ClientName(
                    teammates[1 as libc::c_int as usize],
                    name.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
                );
                crate::src::game::ai_main::BotAI_BotInitialChat(
                    bs as *mut crate::src::game::ai_main::bot_state_s,
                    b"cmd_getflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    name.as_mut_ptr(),
                    0 as *mut libc::c_void,
                );
                BotSayTeamOrder(bs, teammates[1 as libc::c_int as usize]);
                BotSayVoiceTeamOrder(
                    bs,
                    teammates[1 as libc::c_int as usize],
                    b"getflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
            }
            3 => {
                //keep one near the base for when the flag is returned
                crate::src::game::ai_dmq3::ClientName(
                    teammates[0 as libc::c_int as usize],
                    name.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
                );
                crate::src::game::ai_main::BotAI_BotInitialChat(
                    bs as *mut crate::src::game::ai_main::bot_state_s,
                    b"cmd_defendbase\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    name.as_mut_ptr(),
                    0 as *mut libc::c_void,
                );
                BotSayTeamOrder(bs, teammates[0 as libc::c_int as usize]);
                BotSayVoiceTeamOrder(
                    bs,
                    teammates[0 as libc::c_int as usize],
                    b"defend\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
                //the other two get the flag
                crate::src::game::ai_dmq3::ClientName(
                    teammates[1 as libc::c_int as usize],
                    name.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
                );
                crate::src::game::ai_main::BotAI_BotInitialChat(
                    bs as *mut crate::src::game::ai_main::bot_state_s,
                    b"cmd_getflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    name.as_mut_ptr(),
                    0 as *mut libc::c_void,
                );
                BotSayTeamOrder(bs, teammates[1 as libc::c_int as usize]);
                BotSayVoiceTeamOrder(
                    bs,
                    teammates[1 as libc::c_int as usize],
                    b"getflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
                //
                crate::src::game::ai_dmq3::ClientName(
                    teammates[2 as libc::c_int as usize],
                    name.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
                );
                crate::src::game::ai_main::BotAI_BotInitialChat(
                    bs as *mut crate::src::game::ai_main::bot_state_s,
                    b"cmd_getflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    name.as_mut_ptr(),
                    0 as *mut libc::c_void,
                );
                BotSayTeamOrder(bs, teammates[2 as libc::c_int as usize]);
                BotSayVoiceTeamOrder(
                    bs,
                    teammates[2 as libc::c_int as usize],
                    b"getflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
            }
            _ => {
                //keep some people near the base for when the flag is returned
                defenders = (numteammates as libc::c_float as libc::c_int as libc::c_double
                    * 0.3f64
                    + 0.5f64) as libc::c_int;
                if defenders > 3 as libc::c_int {
                    defenders = 3 as libc::c_int
                }
                attackers = (numteammates as libc::c_float as libc::c_int as libc::c_double
                    * 0.6f64
                    + 0.5f64) as libc::c_int;
                if attackers > 6 as libc::c_int {
                    attackers = 6 as libc::c_int
                }
                i = 0 as libc::c_int;
                while i < defenders {
                    //
                    crate::src::game::ai_dmq3::ClientName(
                        teammates[i as usize],
                        name.as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
                    );
                    crate::src::game::ai_main::BotAI_BotInitialChat(
                        bs as *mut crate::src::game::ai_main::bot_state_s,
                        b"cmd_defendbase\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        name.as_mut_ptr(),
                        0 as *mut libc::c_void,
                    );
                    BotSayTeamOrder(bs, teammates[i as usize]);
                    BotSayVoiceTeamOrder(
                        bs,
                        teammates[i as usize],
                        b"defend\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    );
                    i += 1
                }
                i = 0 as libc::c_int;
                while i < attackers {
                    //
                    crate::src::game::ai_dmq3::ClientName(
                        teammates[(numteammates - i - 1 as libc::c_int) as usize],
                        name.as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
                    );
                    crate::src::game::ai_main::BotAI_BotInitialChat(
                        bs as *mut crate::src::game::ai_main::bot_state_s,
                        b"cmd_getflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        name.as_mut_ptr(),
                        0 as *mut libc::c_void,
                    );
                    BotSayTeamOrder(
                        bs,
                        teammates[(numteammates - i - 1 as libc::c_int) as usize],
                    );
                    BotSayVoiceTeamOrder(
                        bs,
                        teammates[0 as libc::c_int as usize],
                        b"getflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    );
                    i += 1
                }
            }
        }
    } else {
        //different orders based on the number of team mates
        match (*bs).numteammates {
            1 => {}
            2 => {
                //both will go for the enemy flag
                crate::src::game::ai_dmq3::ClientName(
                    teammates[0 as libc::c_int as usize],
                    name.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
                );
                crate::src::game::ai_main::BotAI_BotInitialChat(
                    bs as *mut crate::src::game::ai_main::bot_state_s,
                    b"cmd_getflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    name.as_mut_ptr(),
                    0 as *mut libc::c_void,
                );
                BotSayTeamOrder(bs, teammates[0 as libc::c_int as usize]);
                BotSayVoiceTeamOrder(
                    bs,
                    teammates[0 as libc::c_int as usize],
                    b"getflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
                //
                crate::src::game::ai_dmq3::ClientName(
                    teammates[1 as libc::c_int as usize],
                    name.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
                );
                crate::src::game::ai_main::BotAI_BotInitialChat(
                    bs as *mut crate::src::game::ai_main::bot_state_s,
                    b"cmd_getflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    name.as_mut_ptr(),
                    0 as *mut libc::c_void,
                );
                BotSayTeamOrder(bs, teammates[1 as libc::c_int as usize]);
                BotSayVoiceTeamOrder(
                    bs,
                    teammates[1 as libc::c_int as usize],
                    b"getflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
            }
            3 => {
                //everyone go for the flag
                crate::src::game::ai_dmq3::ClientName(
                    teammates[0 as libc::c_int as usize],
                    name.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
                );
                crate::src::game::ai_main::BotAI_BotInitialChat(
                    bs as *mut crate::src::game::ai_main::bot_state_s,
                    b"cmd_getflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    name.as_mut_ptr(),
                    0 as *mut libc::c_void,
                );
                BotSayTeamOrder(bs, teammates[0 as libc::c_int as usize]);
                BotSayVoiceTeamOrder(
                    bs,
                    teammates[0 as libc::c_int as usize],
                    b"getflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
                //
                crate::src::game::ai_dmq3::ClientName(
                    teammates[1 as libc::c_int as usize],
                    name.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
                );
                crate::src::game::ai_main::BotAI_BotInitialChat(
                    bs as *mut crate::src::game::ai_main::bot_state_s,
                    b"cmd_getflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    name.as_mut_ptr(),
                    0 as *mut libc::c_void,
                );
                BotSayTeamOrder(bs, teammates[1 as libc::c_int as usize]);
                BotSayVoiceTeamOrder(
                    bs,
                    teammates[1 as libc::c_int as usize],
                    b"getflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
                //
                crate::src::game::ai_dmq3::ClientName(
                    teammates[2 as libc::c_int as usize],
                    name.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
                );
                crate::src::game::ai_main::BotAI_BotInitialChat(
                    bs as *mut crate::src::game::ai_main::bot_state_s,
                    b"cmd_getflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    name.as_mut_ptr(),
                    0 as *mut libc::c_void,
                );
                BotSayTeamOrder(bs, teammates[2 as libc::c_int as usize]);
                BotSayVoiceTeamOrder(
                    bs,
                    teammates[2 as libc::c_int as usize],
                    b"getflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
            }
            _ => {
                //keep some people near the base for when the flag is returned
                defenders = (numteammates as libc::c_float as libc::c_int as libc::c_double
                    * 0.2f64
                    + 0.5f64) as libc::c_int;
                if defenders > 2 as libc::c_int {
                    defenders = 2 as libc::c_int
                }
                attackers = (numteammates as libc::c_float as libc::c_int as libc::c_double
                    * 0.7f64
                    + 0.5f64) as libc::c_int;
                if attackers > 7 as libc::c_int {
                    attackers = 7 as libc::c_int
                }
                i = 0 as libc::c_int;
                while i < defenders {
                    //
                    crate::src::game::ai_dmq3::ClientName(
                        teammates[i as usize],
                        name.as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
                    );
                    crate::src::game::ai_main::BotAI_BotInitialChat(
                        bs as *mut crate::src::game::ai_main::bot_state_s,
                        b"cmd_defendbase\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        name.as_mut_ptr(),
                        0 as *mut libc::c_void,
                    );
                    BotSayTeamOrder(bs, teammates[i as usize]);
                    BotSayVoiceTeamOrder(
                        bs,
                        teammates[i as usize],
                        b"defend\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    );
                    i += 1
                }
                i = 0 as libc::c_int;
                while i < attackers {
                    //
                    crate::src::game::ai_dmq3::ClientName(
                        teammates[(numteammates - i - 1 as libc::c_int) as usize],
                        name.as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
                    );
                    crate::src::game::ai_main::BotAI_BotInitialChat(
                        bs as *mut crate::src::game::ai_main::bot_state_s,
                        b"cmd_getflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        name.as_mut_ptr(),
                        0 as *mut libc::c_void,
                    );
                    BotSayTeamOrder(
                        bs,
                        teammates[(numteammates - i - 1 as libc::c_int) as usize],
                    );
                    BotSayVoiceTeamOrder(
                        bs,
                        teammates[(numteammates - i - 1 as libc::c_int) as usize],
                        b"getflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    );
                    i += 1
                }
            }
        }
    };
}
/*
==================
BotCTFOrders
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotCTFOrders_EnemyFlagNotAtBase(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
) {
    let mut numteammates: libc::c_int = 0;
    let mut defenders: libc::c_int = 0;
    let mut attackers: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut other: libc::c_int = 0;
    let mut teammates: [libc::c_int; 64] = [0; 64];
    let mut name: [libc::c_char; 36] = [0; 36];
    let mut carriername: [libc::c_char; 36] = [0; 36];
    numteammates = BotSortTeamMatesByBaseTravelTime(
        bs,
        teammates.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_int; 64]>() as libc::c_ulong as libc::c_int,
    );
    BotSortTeamMatesByTaskPreference(bs, teammates.as_mut_ptr(), numteammates);
    //different orders based on the number of team mates
    match numteammates {
        1 => {}
        2 => {
            //tell the one not carrying the flag to defend the base
            if teammates[0 as libc::c_int as usize] == (*bs).flagcarrier {
                other = teammates[1 as libc::c_int as usize]
            } else {
                other = teammates[0 as libc::c_int as usize]
            }
            crate::src::game::ai_dmq3::ClientName(
                other,
                name.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
            );
            crate::src::game::ai_main::BotAI_BotInitialChat(
                bs as *mut crate::src::game::ai_main::bot_state_s,
                b"cmd_defendbase\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                name.as_mut_ptr(),
                0 as *mut libc::c_void,
            );
            BotSayTeamOrder(bs, other);
            BotSayVoiceTeamOrder(
                bs,
                other,
                b"defend\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        }
        3 => {
            //tell the one closest to the base not carrying the flag to defend the base
            if teammates[0 as libc::c_int as usize] != (*bs).flagcarrier {
                other = teammates[0 as libc::c_int as usize]
            } else {
                other = teammates[1 as libc::c_int as usize]
            }
            crate::src::game::ai_dmq3::ClientName(
                other,
                name.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
            );
            crate::src::game::ai_main::BotAI_BotInitialChat(
                bs as *mut crate::src::game::ai_main::bot_state_s,
                b"cmd_defendbase\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                name.as_mut_ptr(),
                0 as *mut libc::c_void,
            );
            BotSayTeamOrder(bs, other);
            BotSayVoiceTeamOrder(
                bs,
                other,
                b"defend\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            //tell the other also to defend the base
            if teammates[2 as libc::c_int as usize] != (*bs).flagcarrier {
                other = teammates[2 as libc::c_int as usize]
            } else {
                other = teammates[1 as libc::c_int as usize]
            }
            crate::src::game::ai_dmq3::ClientName(
                other,
                name.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
            );
            crate::src::game::ai_main::BotAI_BotInitialChat(
                bs as *mut crate::src::game::ai_main::bot_state_s,
                b"cmd_defendbase\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                name.as_mut_ptr(),
                0 as *mut libc::c_void,
            );
            BotSayTeamOrder(bs, other);
            BotSayVoiceTeamOrder(
                bs,
                other,
                b"defend\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        }
        _ => {
            //60% will defend the base
            defenders = (numteammates as libc::c_float as libc::c_int as libc::c_double * 0.6f64
                + 0.5f64) as libc::c_int;
            if defenders > 6 as libc::c_int {
                defenders = 6 as libc::c_int
            }
            //30% accompanies the flag carrier
            attackers = (numteammates as libc::c_float as libc::c_int as libc::c_double * 0.3f64
                + 0.5f64) as libc::c_int;
            if attackers > 3 as libc::c_int {
                attackers = 3 as libc::c_int
            }
            i = 0 as libc::c_int;
            while i < defenders {
                //
                if !(teammates[i as usize] == (*bs).flagcarrier) {
                    crate::src::game::ai_dmq3::ClientName(
                        teammates[i as usize],
                        name.as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
                    );
                    crate::src::game::ai_main::BotAI_BotInitialChat(
                        bs as *mut crate::src::game::ai_main::bot_state_s,
                        b"cmd_defendbase\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        name.as_mut_ptr(),
                        0 as *mut libc::c_void,
                    );
                    BotSayTeamOrder(bs, teammates[i as usize]);
                    BotSayVoiceTeamOrder(
                        bs,
                        teammates[i as usize],
                        b"defend\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    );
                }
                i += 1
            }
            // if we have a flag carrier
            if (*bs).flagcarrier != -(1 as libc::c_int) {
                crate::src::game::ai_dmq3::ClientName(
                    (*bs).flagcarrier,
                    carriername.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
                );
                i = 0 as libc::c_int;
                while i < attackers {
                    //
                    if !(teammates[(numteammates - i - 1 as libc::c_int) as usize]
                        == (*bs).flagcarrier)
                    {
                        //
                        crate::src::game::ai_dmq3::ClientName(
                            teammates[(numteammates - i - 1 as libc::c_int) as usize],
                            name.as_mut_ptr(),
                            ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong
                                as libc::c_int,
                        );
                        if (*bs).flagcarrier == (*bs).client {
                            crate::src::game::ai_main::BotAI_BotInitialChat(
                                bs as *mut crate::src::game::ai_main::bot_state_s,
                                b"cmd_accompanyme\x00" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                                name.as_mut_ptr(),
                                0 as *mut libc::c_void,
                            );
                            BotSayVoiceTeamOrder(
                                bs,
                                teammates[(numteammates - i - 1 as libc::c_int) as usize],
                                b"followme\x00" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                            );
                        } else {
                            crate::src::game::ai_main::BotAI_BotInitialChat(
                                bs as *mut crate::src::game::ai_main::bot_state_s,
                                b"cmd_accompany\x00" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                                name.as_mut_ptr(),
                                carriername.as_mut_ptr(),
                                0 as *mut libc::c_void,
                            );
                            BotSayVoiceTeamOrder(
                                bs,
                                teammates[(numteammates - i - 1 as libc::c_int) as usize],
                                b"followflagcarrier\x00" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                            );
                        }
                        BotSayTeamOrder(
                            bs,
                            teammates[(numteammates - i - 1 as libc::c_int) as usize],
                        );
                    }
                    i += 1
                }
            } else {
                i = 0 as libc::c_int;
                while i < attackers {
                    //
                    if !(teammates[(numteammates - i - 1 as libc::c_int) as usize]
                        == (*bs).flagcarrier)
                    {
                        //
                        crate::src::game::ai_dmq3::ClientName(
                            teammates[(numteammates - i - 1 as libc::c_int) as usize],
                            name.as_mut_ptr(),
                            ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong
                                as libc::c_int,
                        );
                        crate::src::game::ai_main::BotAI_BotInitialChat(
                            bs as *mut crate::src::game::ai_main::bot_state_s,
                            b"cmd_getflag\x00" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                            name.as_mut_ptr(),
                            0 as *mut libc::c_void,
                        );
                        BotSayVoiceTeamOrder(
                            bs,
                            teammates[(numteammates - i - 1 as libc::c_int) as usize],
                            b"getflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        );
                        BotSayTeamOrder(
                            bs,
                            teammates[(numteammates - i - 1 as libc::c_int) as usize],
                        );
                    }
                    i += 1
                }
            }
        }
    };
}
/*
==================
BotCTFOrders
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotCTFOrders_BothFlagsAtBase(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
) {
    let mut numteammates: libc::c_int = 0;
    let mut defenders: libc::c_int = 0;
    let mut attackers: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut teammates: [libc::c_int; 64] = [
        0 as libc::c_int,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    let mut name: [libc::c_char; 36] = [0; 36];
    //sort team mates by travel time to base
    numteammates = BotSortTeamMatesByBaseTravelTime(
        bs,
        teammates.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_int; 64]>() as libc::c_ulong as libc::c_int,
    );
    //sort team mates by CTF preference
    BotSortTeamMatesByTaskPreference(bs, teammates.as_mut_ptr(), numteammates);
    //passive strategy
    if (*bs).ctfstrategy & 1 as libc::c_int == 0 {
        //different orders based on the number of team mates
        match numteammates {
            1 => {}
            2 => {
                //the one closest to the base will defend the base
                crate::src::game::ai_dmq3::ClientName(
                    teammates[0 as libc::c_int as usize],
                    name.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
                );
                crate::src::game::ai_main::BotAI_BotInitialChat(
                    bs as *mut crate::src::game::ai_main::bot_state_s,
                    b"cmd_defendbase\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    name.as_mut_ptr(),
                    0 as *mut libc::c_void,
                );
                BotSayTeamOrder(bs, teammates[0 as libc::c_int as usize]);
                BotSayVoiceTeamOrder(
                    bs,
                    teammates[0 as libc::c_int as usize],
                    b"defend\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
                //the other will get the flag
                crate::src::game::ai_dmq3::ClientName(
                    teammates[1 as libc::c_int as usize],
                    name.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
                );
                crate::src::game::ai_main::BotAI_BotInitialChat(
                    bs as *mut crate::src::game::ai_main::bot_state_s,
                    b"cmd_getflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    name.as_mut_ptr(),
                    0 as *mut libc::c_void,
                );
                BotSayTeamOrder(bs, teammates[1 as libc::c_int as usize]);
                BotSayVoiceTeamOrder(
                    bs,
                    teammates[1 as libc::c_int as usize],
                    b"getflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
            }
            3 => {
                //the one closest to the base will defend the base
                crate::src::game::ai_dmq3::ClientName(
                    teammates[0 as libc::c_int as usize],
                    name.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
                );
                crate::src::game::ai_main::BotAI_BotInitialChat(
                    bs as *mut crate::src::game::ai_main::bot_state_s,
                    b"cmd_defendbase\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    name.as_mut_ptr(),
                    0 as *mut libc::c_void,
                );
                BotSayTeamOrder(bs, teammates[0 as libc::c_int as usize]);
                BotSayVoiceTeamOrder(
                    bs,
                    teammates[0 as libc::c_int as usize],
                    b"defend\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
                //the second one closest to the base will defend the base
                crate::src::game::ai_dmq3::ClientName(
                    teammates[1 as libc::c_int as usize],
                    name.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
                );
                crate::src::game::ai_main::BotAI_BotInitialChat(
                    bs as *mut crate::src::game::ai_main::bot_state_s,
                    b"cmd_defendbase\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    name.as_mut_ptr(),
                    0 as *mut libc::c_void,
                );
                BotSayTeamOrder(bs, teammates[1 as libc::c_int as usize]);
                BotSayVoiceTeamOrder(
                    bs,
                    teammates[1 as libc::c_int as usize],
                    b"defend\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
                //the other will get the flag
                crate::src::game::ai_dmq3::ClientName(
                    teammates[2 as libc::c_int as usize],
                    name.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
                );
                crate::src::game::ai_main::BotAI_BotInitialChat(
                    bs as *mut crate::src::game::ai_main::bot_state_s,
                    b"cmd_getflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    name.as_mut_ptr(),
                    0 as *mut libc::c_void,
                );
                BotSayTeamOrder(bs, teammates[2 as libc::c_int as usize]);
                BotSayVoiceTeamOrder(
                    bs,
                    teammates[2 as libc::c_int as usize],
                    b"getflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
            }
            _ => {
                defenders = (numteammates as libc::c_float as libc::c_int as libc::c_double
                    * 0.5f64
                    + 0.5f64) as libc::c_int;
                if defenders > 5 as libc::c_int {
                    defenders = 5 as libc::c_int
                }
                attackers = (numteammates as libc::c_float as libc::c_int as libc::c_double
                    * 0.4f64
                    + 0.5f64) as libc::c_int;
                if attackers > 4 as libc::c_int {
                    attackers = 4 as libc::c_int
                }
                i = 0 as libc::c_int;
                while i < defenders {
                    //
                    crate::src::game::ai_dmq3::ClientName(
                        teammates[i as usize],
                        name.as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
                    );
                    crate::src::game::ai_main::BotAI_BotInitialChat(
                        bs as *mut crate::src::game::ai_main::bot_state_s,
                        b"cmd_defendbase\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        name.as_mut_ptr(),
                        0 as *mut libc::c_void,
                    );
                    BotSayTeamOrder(bs, teammates[i as usize]);
                    BotSayVoiceTeamOrder(
                        bs,
                        teammates[i as usize],
                        b"defend\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    );
                    i += 1
                }
                i = 0 as libc::c_int;
                while i < attackers {
                    //
                    crate::src::game::ai_dmq3::ClientName(
                        teammates[(numteammates - i - 1 as libc::c_int) as usize],
                        name.as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
                    );
                    crate::src::game::ai_main::BotAI_BotInitialChat(
                        bs as *mut crate::src::game::ai_main::bot_state_s,
                        b"cmd_getflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        name.as_mut_ptr(),
                        0 as *mut libc::c_void,
                    );
                    BotSayTeamOrder(
                        bs,
                        teammates[(numteammates - i - 1 as libc::c_int) as usize],
                    );
                    BotSayVoiceTeamOrder(
                        bs,
                        teammates[(numteammates - i - 1 as libc::c_int) as usize],
                        b"getflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    );
                    i += 1
                }
            }
        }
    } else {
        //different orders based on the number of team mates
        match numteammates {
            1 => {}
            2 => {
                //the one closest to the base will defend the base
                crate::src::game::ai_dmq3::ClientName(
                    teammates[0 as libc::c_int as usize],
                    name.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
                );
                crate::src::game::ai_main::BotAI_BotInitialChat(
                    bs as *mut crate::src::game::ai_main::bot_state_s,
                    b"cmd_defendbase\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    name.as_mut_ptr(),
                    0 as *mut libc::c_void,
                );
                BotSayTeamOrder(bs, teammates[0 as libc::c_int as usize]);
                BotSayVoiceTeamOrder(
                    bs,
                    teammates[0 as libc::c_int as usize],
                    b"defend\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
                //the other will get the flag
                crate::src::game::ai_dmq3::ClientName(
                    teammates[1 as libc::c_int as usize],
                    name.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
                );
                crate::src::game::ai_main::BotAI_BotInitialChat(
                    bs as *mut crate::src::game::ai_main::bot_state_s,
                    b"cmd_getflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    name.as_mut_ptr(),
                    0 as *mut libc::c_void,
                );
                BotSayTeamOrder(bs, teammates[1 as libc::c_int as usize]);
                BotSayVoiceTeamOrder(
                    bs,
                    teammates[1 as libc::c_int as usize],
                    b"getflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
            }
            3 => {
                //the one closest to the base will defend the base
                crate::src::game::ai_dmq3::ClientName(
                    teammates[0 as libc::c_int as usize],
                    name.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
                );
                crate::src::game::ai_main::BotAI_BotInitialChat(
                    bs as *mut crate::src::game::ai_main::bot_state_s,
                    b"cmd_defendbase\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    name.as_mut_ptr(),
                    0 as *mut libc::c_void,
                );
                BotSayTeamOrder(bs, teammates[0 as libc::c_int as usize]);
                BotSayVoiceTeamOrder(
                    bs,
                    teammates[0 as libc::c_int as usize],
                    b"defend\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
                //the others should go for the enemy flag
                crate::src::game::ai_dmq3::ClientName(
                    teammates[1 as libc::c_int as usize],
                    name.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
                );
                crate::src::game::ai_main::BotAI_BotInitialChat(
                    bs as *mut crate::src::game::ai_main::bot_state_s,
                    b"cmd_getflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    name.as_mut_ptr(),
                    0 as *mut libc::c_void,
                );
                BotSayTeamOrder(bs, teammates[1 as libc::c_int as usize]);
                BotSayVoiceTeamOrder(
                    bs,
                    teammates[1 as libc::c_int as usize],
                    b"getflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
                //
                crate::src::game::ai_dmq3::ClientName(
                    teammates[2 as libc::c_int as usize],
                    name.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
                );
                crate::src::game::ai_main::BotAI_BotInitialChat(
                    bs as *mut crate::src::game::ai_main::bot_state_s,
                    b"cmd_getflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    name.as_mut_ptr(),
                    0 as *mut libc::c_void,
                );
                BotSayTeamOrder(bs, teammates[2 as libc::c_int as usize]);
                BotSayVoiceTeamOrder(
                    bs,
                    teammates[2 as libc::c_int as usize],
                    b"getflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
            }
            _ => {
                defenders = (numteammates as libc::c_float as libc::c_int as libc::c_double
                    * 0.4f64
                    + 0.5f64) as libc::c_int;
                if defenders > 4 as libc::c_int {
                    defenders = 4 as libc::c_int
                }
                attackers = (numteammates as libc::c_float as libc::c_int as libc::c_double
                    * 0.5f64
                    + 0.5f64) as libc::c_int;
                if attackers > 5 as libc::c_int {
                    attackers = 5 as libc::c_int
                }
                i = 0 as libc::c_int;
                while i < defenders {
                    //
                    crate::src::game::ai_dmq3::ClientName(
                        teammates[i as usize],
                        name.as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
                    );
                    crate::src::game::ai_main::BotAI_BotInitialChat(
                        bs as *mut crate::src::game::ai_main::bot_state_s,
                        b"cmd_defendbase\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        name.as_mut_ptr(),
                        0 as *mut libc::c_void,
                    );
                    BotSayTeamOrder(bs, teammates[i as usize]);
                    BotSayVoiceTeamOrder(
                        bs,
                        teammates[i as usize],
                        b"defend\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    );
                    i += 1
                }
                i = 0 as libc::c_int;
                while i < attackers {
                    //
                    crate::src::game::ai_dmq3::ClientName(
                        teammates[(numteammates - i - 1 as libc::c_int) as usize],
                        name.as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
                    );
                    crate::src::game::ai_main::BotAI_BotInitialChat(
                        bs as *mut crate::src::game::ai_main::bot_state_s,
                        b"cmd_getflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        name.as_mut_ptr(),
                        0 as *mut libc::c_void,
                    );
                    BotSayTeamOrder(
                        bs,
                        teammates[(numteammates - i - 1 as libc::c_int) as usize],
                    );
                    BotSayVoiceTeamOrder(
                        bs,
                        teammates[(numteammates - i - 1 as libc::c_int) as usize],
                        b"getflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    );
                    i += 1
                }
            }
        }
    };
}
/*
==================
BotCTFOrders
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotCTFOrders(mut bs: *mut crate::src::game::ai_main::bot_state_t) {
    let mut flagstatus: libc::c_int = 0;
    //
    if crate::src::game::ai_dmq3::BotTeam(bs as *mut crate::src::game::ai_main::bot_state_s)
        == crate::bg_public_h::TEAM_RED as libc::c_int
    {
        flagstatus = (*bs).redflagstatus * 2 as libc::c_int + (*bs).blueflagstatus
    } else {
        flagstatus = (*bs).blueflagstatus * 2 as libc::c_int + (*bs).redflagstatus
    }
    //
    match flagstatus {
        0 => {
            BotCTFOrders_BothFlagsAtBase(bs);
        }
        1 => {
            BotCTFOrders_EnemyFlagNotAtBase(bs);
        }
        2 => {
            BotCTFOrders_FlagNotAtBase(bs);
        }
        3 => {
            BotCTFOrders_BothFlagsNotAtBase(bs);
        }
        _ => {}
    };
}
/*
==================
BotCreateGroup
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotCreateGroup(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut teammates: *mut libc::c_int,
    mut groupsize: libc::c_int,
) {
    let mut name: [libc::c_char; 36] = [0; 36];
    let mut leadername: [libc::c_char; 36] = [0; 36];
    let mut i: libc::c_int = 0;
    // the others in the group will follow the teammates[0]
    crate::src::game::ai_dmq3::ClientName(
        *teammates.offset(0 as libc::c_int as isize),
        leadername.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
    );
    i = 1 as libc::c_int;
    while i < groupsize {
        crate::src::game::ai_dmq3::ClientName(
            *teammates.offset(i as isize),
            name.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
        );
        if *teammates.offset(0 as libc::c_int as isize) == (*bs).client {
            crate::src::game::ai_main::BotAI_BotInitialChat(
                bs as *mut crate::src::game::ai_main::bot_state_s,
                b"cmd_accompanyme\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                name.as_mut_ptr(),
                0 as *mut libc::c_void,
            );
        } else {
            crate::src::game::ai_main::BotAI_BotInitialChat(
                bs as *mut crate::src::game::ai_main::bot_state_s,
                b"cmd_accompany\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                name.as_mut_ptr(),
                leadername.as_mut_ptr(),
                0 as *mut libc::c_void,
            );
        }
        BotSayTeamOrderAlways(bs, *teammates.offset(i as isize));
        i += 1
    }
}
/*
==================
BotTeamOrders

  FIXME: defend key areas?
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotTeamOrders(mut bs: *mut crate::src::game::ai_main::bot_state_t) {
    let mut teammates: [libc::c_int; 64] = [0; 64];
    let mut numteammates: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    numteammates = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < crate::src::game::g_main::level.maxclients {
        crate::src::game::g_syscalls::trap_GetConfigstring(
            32 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int + i,
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
        );
        //if no config string or no name
        if !(crate::stdlib::strlen(buf.as_mut_ptr()) == 0
            || crate::stdlib::strlen(crate::src::qcommon::q_shared::Info_ValueForKey(
                buf.as_mut_ptr(),
                b"n\x00" as *const u8 as *const libc::c_char,
            )) == 0)
        {
            //skip spectators
            if !(atoi(crate::src::qcommon::q_shared::Info_ValueForKey(
                buf.as_mut_ptr(),
                b"t\x00" as *const u8 as *const libc::c_char,
            )) == crate::bg_public_h::TEAM_SPECTATOR as libc::c_int)
            {
                //
                if crate::src::game::ai_dmq3::BotSameTeam(
                    bs as *mut crate::src::game::ai_main::bot_state_s,
                    i,
                ) != 0
                {
                    teammates[numteammates as usize] = i;
                    numteammates += 1
                }
            }
        }
        i += 1
    }
    //
    match numteammates {
        1 | 2 => {}
        3 => {
            //have one follow another and one free roaming
            BotCreateGroup(bs, teammates.as_mut_ptr(), 2 as libc::c_int); //a group of 2
        }
        4 => {
            BotCreateGroup(bs, teammates.as_mut_ptr(), 2 as libc::c_int); //a group of 2
            BotCreateGroup(
                bs,
                &mut *teammates.as_mut_ptr().offset(2 as libc::c_int as isize),
                2 as libc::c_int,
            ); //a group of 2
        }
        5 => {
            BotCreateGroup(bs, teammates.as_mut_ptr(), 2 as libc::c_int); //a group of 3
            BotCreateGroup(
                bs,
                &mut *teammates.as_mut_ptr().offset(2 as libc::c_int as isize),
                3 as libc::c_int,
            );
        }
        _ => {
            if numteammates <= 10 as libc::c_int {
                i = 0 as libc::c_int;
                while i < numteammates / 2 as libc::c_int {
                    BotCreateGroup(
                        bs,
                        &mut *teammates
                            .as_mut_ptr()
                            .offset((i * 2 as libc::c_int) as isize),
                        2 as libc::c_int,
                    );
                    i += 1
                    //groups of 2
                }
            }
        }
    };
}
/*
==================
FindHumanTeamLeader
==================
*/
#[no_mangle]

pub unsafe extern "C" fn FindHumanTeamLeader(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 64 as libc::c_int {
        if crate::src::game::g_main::g_entities[i as usize].inuse as u64 != 0 {
            // if this player is not a bot
            if crate::src::game::g_main::g_entities[i as usize].r.svFlags & 0x8 as libc::c_int == 0
            {
                // if this player is ok with being the leader
                if crate::src::game::ai_cmd::notleader[i as usize] == 0 {
                    // if this player is on the same team
                    if crate::src::game::ai_dmq3::BotSameTeam(
                        bs as *mut crate::src::game::ai_main::bot_state_s,
                        i,
                    ) != 0
                    {
                        crate::src::game::ai_dmq3::ClientName(
                            i,
                            (*bs).teamleader.as_mut_ptr(),
                            ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong
                                as libc::c_int,
                        );
                        // if not yet ordered to do anything
                        if crate::src::game::ai_dmq3::BotSetLastOrderedTask(
                            bs as *mut crate::src::game::ai_main::bot_state_s,
                        ) == 0
                        {
                            // go on defense by default
                            crate::src::game::ai_vcmd::BotVoiceChat_Defend(
                                bs as *mut crate::src::game::ai_main::bot_state_s,
                                i,
                                2 as libc::c_int,
                            );
                        }
                        return crate::src::qcommon::q_shared::qtrue as libc::c_int;
                    }
                }
            }
        }
        i += 1
    }
    return crate::src::qcommon::q_shared::qfalse as libc::c_int;
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
 * name:		ai_team.h
 *
 * desc:		Quake3 bot AI
 *
 * $Archive: /source/code/botai/ai_chat.c $
 *
 *****************************************************************************/
/*
==================
BotTeamAI
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotTeamAI(mut bs: *mut crate::src::game::ai_main::bot_state_t) {
    let mut numteammates: libc::c_int = 0;
    let mut netname: [libc::c_char; 36] = [0; 36];
    //
    if crate::src::game::ai_dmq3::gametype < crate::bg_public_h::GT_TEAM as libc::c_int {
        return;
    }
    // make sure we've got a valid team leader
    if BotValidTeamLeader(bs) == 0 {
        //
        if FindHumanTeamLeader(bs) == 0 {
            //
            if (*bs).askteamleader_time == 0. && (*bs).becometeamleader_time == 0. {
                if (*bs).entergame_time + 10 as libc::c_int as libc::c_float
                    > crate::src::game::ai_main::floattime
                {
                    (*bs).askteamleader_time = crate::src::game::ai_main::floattime
                        + 5 as libc::c_int as libc::c_float
                        + (::libc::rand() & 0x7fff as libc::c_int) as libc::c_float
                            / 0x7fff as libc::c_int as libc::c_float
                            * 10 as libc::c_int as libc::c_float
                } else {
                    (*bs).becometeamleader_time = crate::src::game::ai_main::floattime
                        + 5 as libc::c_int as libc::c_float
                        + (::libc::rand() & 0x7fff as libc::c_int) as libc::c_float
                            / 0x7fff as libc::c_int as libc::c_float
                            * 10 as libc::c_int as libc::c_float
                }
            }
            if (*bs).askteamleader_time != 0.
                && (*bs).askteamleader_time < crate::src::game::ai_main::floattime
            {
                // if asked for a team leader and no response
                crate::src::game::ai_main::BotAI_BotInitialChat(
                    bs as *mut crate::src::game::ai_main::bot_state_s,
                    b"whoisteamleader\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    0 as *mut libc::c_void,
                );
                crate::src::game::g_syscalls::trap_BotEnterChat(
                    (*bs).cs,
                    0 as libc::c_int,
                    1 as libc::c_int,
                );
                (*bs).askteamleader_time = 0 as libc::c_int as libc::c_float;
                (*bs).becometeamleader_time = crate::src::game::ai_main::floattime
                    + 8 as libc::c_int as libc::c_float
                    + (::libc::rand() & 0x7fff as libc::c_int) as libc::c_float
                        / 0x7fff as libc::c_int as libc::c_float
                        * 10 as libc::c_int as libc::c_float
            }
            if (*bs).becometeamleader_time != 0.
                && (*bs).becometeamleader_time < crate::src::game::ai_main::floattime
            {
                crate::src::game::ai_main::BotAI_BotInitialChat(
                    bs as *mut crate::src::game::ai_main::bot_state_s,
                    b"iamteamleader\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    0 as *mut libc::c_void,
                );
                crate::src::game::g_syscalls::trap_BotEnterChat(
                    (*bs).cs,
                    0 as libc::c_int,
                    1 as libc::c_int,
                );
                BotSayVoiceTeamOrder(
                    bs,
                    -(1 as libc::c_int),
                    b"startleader\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
                crate::src::game::ai_dmq3::ClientName(
                    (*bs).client,
                    netname.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
                );
                crate::stdlib::strncpy(
                    (*bs).teamleader.as_mut_ptr(),
                    netname.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong,
                );
                (*bs).teamleader[(::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    as usize] = '\u{0}' as i32 as libc::c_char;
                (*bs).becometeamleader_time = 0 as libc::c_int as libc::c_float
            }
            return;
        }
    }
    (*bs).askteamleader_time = 0 as libc::c_int as libc::c_float;
    (*bs).becometeamleader_time = 0 as libc::c_int as libc::c_float;
    //return if this bot is NOT the team leader
    crate::src::game::ai_dmq3::ClientName(
        (*bs).client,
        netname.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
    );
    if crate::src::qcommon::q_shared::Q_stricmp(netname.as_mut_ptr(), (*bs).teamleader.as_mut_ptr())
        != 0 as libc::c_int
    {
        return;
    }
    //
    numteammates = BotNumTeamMates(bs);
    //give orders
    match crate::src::game::ai_dmq3::gametype {
        3 => {
            if (*bs).numteammates != numteammates || (*bs).forceorders != 0 {
                (*bs).teamgiveorders_time = crate::src::game::ai_main::floattime;
                (*bs).numteammates = numteammates;
                (*bs).forceorders = crate::src::qcommon::q_shared::qfalse as libc::c_int
            }
            //if it's time to give orders
            if (*bs).teamgiveorders_time != 0.
                && (*bs).teamgiveorders_time
                    < crate::src::game::ai_main::floattime - 5 as libc::c_int as libc::c_float
            {
                BotTeamOrders(bs);
                //give orders again after 120 seconds
                (*bs).teamgiveorders_time =
                    crate::src::game::ai_main::floattime + 120 as libc::c_int as libc::c_float
            }
        }
        4 => {
            //if the number of team mates changed or the flag status changed
            //or someone wants to know what to do
            if (*bs).numteammates != numteammates
                || (*bs).flagstatuschanged != 0
                || (*bs).forceorders != 0
            {
                (*bs).teamgiveorders_time = crate::src::game::ai_main::floattime;
                (*bs).numteammates = numteammates;
                (*bs).flagstatuschanged = crate::src::qcommon::q_shared::qfalse as libc::c_int;
                (*bs).forceorders = crate::src::qcommon::q_shared::qfalse as libc::c_int
            }
            //if there were no flag captures the last 3 minutes
            if (*bs).lastflagcapture_time
                < crate::src::game::ai_main::floattime - 240 as libc::c_int as libc::c_float
            {
                (*bs).lastflagcapture_time = crate::src::game::ai_main::floattime;
                //randomly change the CTF strategy
                if (((::libc::rand() & 0x7fff as libc::c_int) as libc::c_float
                    / 0x7fff as libc::c_int as libc::c_float) as libc::c_double)
                    < 0.4f64
                {
                    (*bs).ctfstrategy ^= 1 as libc::c_int;
                    (*bs).teamgiveorders_time = crate::src::game::ai_main::floattime
                }
            }
            //if it's time to give orders
            if (*bs).teamgiveorders_time != 0.
                && (*bs).teamgiveorders_time
                    < crate::src::game::ai_main::floattime - 3 as libc::c_int as libc::c_float
            {
                BotCTFOrders(bs);
                //
                (*bs).teamgiveorders_time = 0 as libc::c_int as libc::c_float
            }
        }
        _ => {}
    };
}

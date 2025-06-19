use ::libc;

pub mod stdlib_h {
    #[inline]

    pub unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> i32 {
        return ::libc::strtol(
            __nptr,
            0 as *mut libc::c_void as *mut *mut libc::c_char,
            10 as i32,
        ) as i32;
    }
}

pub use crate::be_aas_h::aas_entityinfo_s;
pub use crate::be_aas_h::aas_entityinfo_t;
pub use crate::be_ai_goal_h::bot_goal_s;
pub use crate::be_ai_goal_h::bot_goal_t;
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
pub use crate::bg_public_h::TEAM_BLUE;
pub use crate::bg_public_h::TEAM_FREE;
pub use crate::bg_public_h::TEAM_NUM_TEAMS;
pub use crate::bg_public_h::TEAM_RED;
pub use crate::bg_public_h::TEAM_SPECTATOR;
pub use crate::g_local_h::bot_settings_s;
pub use crate::g_local_h::bot_settings_t;
pub use crate::src::game::ai_main::bot_activategoal_s;
pub use crate::src::game::ai_main::bot_activategoal_t;
pub use crate::src::game::ai_main::bot_state_s;
pub use crate::src::game::ai_main::bot_state_t;
pub use crate::src::game::ai_main::bot_waypoint_s;
pub use crate::src::game::ai_main::bot_waypoint_t;
pub use crate::src::game::ai_main::floattime;
pub use crate::src::game::ai_main::BotAI_BotInitialChat;
pub use crate::src::game::ai_main::BotEntityInfo;
pub use crate::src::game::g_syscalls::trap_BotEnterChat;
pub use crate::src::game::g_syscalls::trap_EA_Action;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::playerState_s;
pub use crate::src::qcommon::q_shared::playerState_t;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::usercmd_s;
pub use crate::src::qcommon::q_shared::usercmd_t;
pub use crate::src::qcommon::q_shared::vec3_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::qcommon::q_shared::Q_stricmp;
pub use crate::src::qcommon::q_shared::Q_strncpyz;

pub use crate::src::game::ai_vcmd::stdlib_h::atoi;
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
 * name:		ai_vcmd.c
 *
 * desc:		Quake3 bot AI
 *
 * $Archive: /MissionPack/code/game/ai_vcmd.c $
 *
 *****************************************************************************/
//
//
// for the voice chats

pub type voiceCommand_t = voiceCommand_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct voiceCommand_s {
    pub cmd: *mut libc::c_char,
    pub func: Option<
        unsafe extern "C" fn(
            _: *mut crate::src::game::ai_main::bot_state_t,
            _: i32,
            _: i32,
        ) -> (),
    >,
}
/*
==================
BotVoiceChat_GetFlag
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotVoiceChat_GetFlag(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut client: i32,
    mut _mode: i32,
) {
    //
    if crate::src::game::ai_dmq3::gametype == crate::bg_public_h::GT_CTF as i32 {
        if crate::src::game::ai_dmq3::ctf_redflag.areanum == 0
            || crate::src::game::ai_dmq3::ctf_blueflag.areanum == 0
        {
            return;
        }
    } else {
        return;
    }
    //
    (*bs).decisionmaker = client;
    (*bs).ordered = crate::src::qcommon::q_shared::qtrue as i32;
    (*bs).order_time = crate::src::game::ai_main::floattime;
    //set the time to send a message to the team mates
    (*bs).teammessage_time = crate::src::game::ai_main::floattime
        + 2 as i32 as f32
            * ((::libc::rand() & 0x7fff as i32) as f32
                / 0x7fff as i32 as f32);
    //set the ltg type
    (*bs).ltgtype = 4 as i32;
    //set the team goal time
    (*bs).teamgoal_time =
        crate::src::game::ai_main::floattime + 600 as i32 as f32;
    // get an alternate route in ctf
    if crate::src::game::ai_dmq3::gametype == crate::bg_public_h::GT_CTF as i32 {
        //get an alternative route goal towards the enemy base
        crate::src::game::ai_dmq3::BotGetAlternateRouteGoal(
            bs as *mut crate::src::game::ai_main::bot_state_s,
            crate::src::game::ai_dmq3::BotOppositeTeam(
                bs as *mut crate::src::game::ai_main::bot_state_s,
            ),
        );
    }
    //
    crate::src::game::ai_dmq3::BotSetTeamStatus(bs as *mut crate::src::game::ai_main::bot_state_s);
    // remember last ordered task
    crate::src::game::ai_dmq3::BotRememberLastOrderedTask(
        bs as *mut crate::src::game::ai_main::bot_state_s,
    );
    //DEBUG
}
/*
==================
BotVoiceChat_Offense
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotVoiceChat_Offense(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut client: i32,
    mut mode: i32,
) {
    if crate::src::game::ai_dmq3::gametype == crate::bg_public_h::GT_CTF as i32 {
        BotVoiceChat_GetFlag(bs, client, mode);
        return;
    }
    //
    (*bs).decisionmaker = client;
    (*bs).ordered = crate::src::qcommon::q_shared::qtrue as i32;
    (*bs).order_time = crate::src::game::ai_main::floattime;
    //set the time to send a message to the team mates
    (*bs).teammessage_time = crate::src::game::ai_main::floattime
        + 2 as i32 as f32
            * ((::libc::rand() & 0x7fff as i32) as f32
                / 0x7fff as i32 as f32);
    //set the ltg type
    (*bs).ltgtype = 13 as i32;
    //set the team goal time
    (*bs).teamgoal_time =
        crate::src::game::ai_main::floattime + 600 as i32 as f32;
    (*bs).attackaway_time = 0 as i32 as f32;
    //
    crate::src::game::ai_dmq3::BotSetTeamStatus(bs as *mut crate::src::game::ai_main::bot_state_s);
    // remember last ordered task
    crate::src::game::ai_dmq3::BotRememberLastOrderedTask(
        bs as *mut crate::src::game::ai_main::bot_state_s,
    );
    //DEBUG
}
/*
==================
BotVoiceChat_Defend
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotVoiceChat_Defend(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut client: i32,
    mut _mode: i32,
) {
    if crate::src::game::ai_dmq3::gametype == crate::bg_public_h::GT_CTF as i32 {
        //
        match crate::src::game::ai_dmq3::BotTeam(bs as *mut crate::src::game::ai_main::bot_state_s)
        {
            1 => {
                crate::stdlib::memcpy(
                    &mut (*bs).teamgoal as *mut crate::be_ai_goal_h::bot_goal_t
                        as *mut libc::c_void,
                    &mut crate::src::game::ai_dmq3::ctf_redflag
                        as *mut crate::be_ai_goal_h::bot_goal_t
                        as *const libc::c_void,
                    ::std::mem::size_of::<crate::be_ai_goal_h::bot_goal_t>() as libc::c_ulong,
                );
            }
            2 => {
                crate::stdlib::memcpy(
                    &mut (*bs).teamgoal as *mut crate::be_ai_goal_h::bot_goal_t
                        as *mut libc::c_void,
                    &mut crate::src::game::ai_dmq3::ctf_blueflag
                        as *mut crate::be_ai_goal_h::bot_goal_t
                        as *const libc::c_void,
                    ::std::mem::size_of::<crate::be_ai_goal_h::bot_goal_t>() as libc::c_ulong,
                );
            }
            _ => return,
        }
    } else {
        return;
    }
    //
    (*bs).decisionmaker = client;
    (*bs).ordered = crate::src::qcommon::q_shared::qtrue as i32;
    (*bs).order_time = crate::src::game::ai_main::floattime;
    //set the time to send a message to the team mates
    (*bs).teammessage_time = crate::src::game::ai_main::floattime
        + 2 as i32 as f32
            * ((::libc::rand() & 0x7fff as i32) as f32
                / 0x7fff as i32 as f32);
    //set the ltg type
    (*bs).ltgtype = 3 as i32;
    //get the team goal time
    (*bs).teamgoal_time =
        crate::src::game::ai_main::floattime + 600 as i32 as f32;
    //away from defending
    (*bs).defendaway_time = 0 as i32 as f32;
    //
    crate::src::game::ai_dmq3::BotSetTeamStatus(bs as *mut crate::src::game::ai_main::bot_state_s);
    // remember last ordered task
    crate::src::game::ai_dmq3::BotRememberLastOrderedTask(
        bs as *mut crate::src::game::ai_main::bot_state_s,
    );
    //DEBUG
}
/*
==================
BotVoiceChat_DefendFlag
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotVoiceChat_DefendFlag(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut client: i32,
    mut mode: i32,
) {
    BotVoiceChat_Defend(bs, client, mode);
}
/*
==================
BotVoiceChat_Patrol
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotVoiceChat_Patrol(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut client: i32,
    mut _mode: i32,
) {
    //
    (*bs).decisionmaker = client;
    //
    (*bs).ltgtype = 0 as i32;
    (*bs).lead_time = 0 as i32 as f32;
    (*bs).lastgoal_ltgtype = 0 as i32;
    //
    crate::src::game::ai_main::BotAI_BotInitialChat(
        bs as *mut crate::src::game::ai_main::bot_state_s,
        b"dismissed\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *mut libc::c_void,
    );
    crate::src::game::g_syscalls::trap_BotEnterChat((*bs).cs, client, 2 as i32);
    crate::src::game::ai_team::BotVoiceChatOnly(
        bs as *mut crate::src::game::ai_main::bot_state_s,
        -(1 as i32),
        b"onpatrol\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    //
    crate::src::game::ai_dmq3::BotSetTeamStatus(bs as *mut crate::src::game::ai_main::bot_state_s);
    //DEBUG
}
/*
==================
BotVoiceChat_Camp
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotVoiceChat_Camp(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut client: i32,
    mut _mode: i32,
) {
    let mut areanum: i32 = 0;
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
    let mut netname: [libc::c_char; 36] = [0; 36];
    //
    (*bs).teamgoal.entitynum = -(1 as i32);
    crate::src::game::ai_main::BotEntityInfo(
        client,
        &mut entinfo as *mut _ as *mut crate::be_aas_h::aas_entityinfo_s,
    );
    //if info is valid (in PVS)
    if entinfo.valid != 0 {
        areanum = crate::src::game::ai_dmq3::BotPointAreaNum(entinfo.origin.as_mut_ptr());
        if areanum != 0 {
            // && trap_AAS_AreaReachability(areanum)) {
            //NOTE: just assume the bot knows where the person is
            //if (BotEntityVisible(bs->entitynum, bs->eye, bs->viewangles, 360, client)) {
            (*bs).teamgoal.entitynum = client;
            (*bs).teamgoal.areanum = areanum;
            (*bs).teamgoal.origin[0 as i32 as usize] =
                entinfo.origin[0 as i32 as usize];
            (*bs).teamgoal.origin[1 as i32 as usize] =
                entinfo.origin[1 as i32 as usize];
            (*bs).teamgoal.origin[2 as i32 as usize] =
                entinfo.origin[2 as i32 as usize];
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
            //}
        }
    }
    //if the other is not visible
    if (*bs).teamgoal.entitynum < 0 as i32 {
        crate::src::game::ai_main::BotAI_BotInitialChat(
            bs as *mut crate::src::game::ai_main::bot_state_s,
            b"whereareyou\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            crate::src::game::ai_dmq3::EasyClientName(
                client,
                netname.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as i32,
            ),
            0 as *mut libc::c_void,
        );
        crate::src::game::g_syscalls::trap_BotEnterChat((*bs).cs, client, 2 as i32);
        return;
    }
    //
    (*bs).decisionmaker = client;
    (*bs).ordered = crate::src::qcommon::q_shared::qtrue as i32;
    (*bs).order_time = crate::src::game::ai_main::floattime;
    //set the time to send a message to the team mates
    (*bs).teammessage_time = crate::src::game::ai_main::floattime
        + 2 as i32 as f32
            * ((::libc::rand() & 0x7fff as i32) as f32
                / 0x7fff as i32 as f32);
    //set the ltg type
    (*bs).ltgtype = 8 as i32;
    //get the team goal time
    (*bs).teamgoal_time =
        crate::src::game::ai_main::floattime + 600 as i32 as f32;
    //the teammate that requested the camping
    (*bs).teammate = client;
    //not arrived yet
    (*bs).arrive_time = 0 as i32 as f32;
    //
    crate::src::game::ai_dmq3::BotSetTeamStatus(bs as *mut crate::src::game::ai_main::bot_state_s);
    // remember last ordered task
    crate::src::game::ai_dmq3::BotRememberLastOrderedTask(
        bs as *mut crate::src::game::ai_main::bot_state_s,
    );
    //DEBUG
}
/*
==================
BotVoiceChat_FollowMe
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotVoiceChat_FollowMe(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut client: i32,
    mut _mode: i32,
) {
    let mut areanum: i32 = 0;
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
    let mut netname: [libc::c_char; 36] = [0; 36];
    (*bs).teamgoal.entitynum = -(1 as i32);
    crate::src::game::ai_main::BotEntityInfo(
        client,
        &mut entinfo as *mut _ as *mut crate::be_aas_h::aas_entityinfo_s,
    );
    //if info is valid (in PVS)
    if entinfo.valid != 0 {
        areanum = crate::src::game::ai_dmq3::BotPointAreaNum(entinfo.origin.as_mut_ptr());
        if areanum != 0 {
            // && trap_AAS_AreaReachability(areanum)) {
            (*bs).teamgoal.entitynum = client;
            (*bs).teamgoal.areanum = areanum;
            (*bs).teamgoal.origin[0 as i32 as usize] =
                entinfo.origin[0 as i32 as usize];
            (*bs).teamgoal.origin[1 as i32 as usize] =
                entinfo.origin[1 as i32 as usize];
            (*bs).teamgoal.origin[2 as i32 as usize] =
                entinfo.origin[2 as i32 as usize];
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
    //if the other is not visible
    if (*bs).teamgoal.entitynum < 0 as i32 {
        crate::src::game::ai_main::BotAI_BotInitialChat(
            bs as *mut crate::src::game::ai_main::bot_state_s,
            b"whereareyou\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            crate::src::game::ai_dmq3::EasyClientName(
                client,
                netname.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as i32,
            ),
            0 as *mut libc::c_void,
        );
        crate::src::game::g_syscalls::trap_BotEnterChat((*bs).cs, client, 2 as i32);
        return;
    }
    //
    (*bs).decisionmaker = client;
    (*bs).ordered = crate::src::qcommon::q_shared::qtrue as i32;
    (*bs).order_time = crate::src::game::ai_main::floattime;
    //the team mate
    (*bs).teammate = client;
    //last time the team mate was assumed visible
    (*bs).teammatevisible_time = crate::src::game::ai_main::floattime;
    //set the time to send a message to the team mates
    (*bs).teammessage_time = crate::src::game::ai_main::floattime
        + 2 as i32 as f32
            * ((::libc::rand() & 0x7fff as i32) as f32
                / 0x7fff as i32 as f32);
    //get the team goal time
    (*bs).teamgoal_time =
        crate::src::game::ai_main::floattime + 600 as i32 as f32;
    //set the ltg type
    (*bs).ltgtype = 2 as i32; //3.5 meter
    (*bs).formation_dist = (3.5f64 * 32 as i32 as f64) as f32;
    (*bs).arrive_time = 0 as i32 as f32;
    //
    crate::src::game::ai_dmq3::BotSetTeamStatus(bs as *mut crate::src::game::ai_main::bot_state_s);
    // remember last ordered task
    crate::src::game::ai_dmq3::BotRememberLastOrderedTask(
        bs as *mut crate::src::game::ai_main::bot_state_s,
    );
    //DEBUG
}
/*
==================
BotVoiceChat_FollowFlagCarrier
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotVoiceChat_FollowFlagCarrier(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut _client: i32,
    mut mode: i32,
) {
    let mut carrier: i32 = 0;
    carrier = crate::src::game::ai_dmq3::BotTeamFlagCarrier(
        bs as *mut crate::src::game::ai_main::bot_state_s,
    );
    if carrier >= 0 as i32 {
        BotVoiceChat_FollowMe(bs, carrier, mode);
    };
    //DEBUG
}
/*
==================
BotVoiceChat_ReturnFlag
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotVoiceChat_ReturnFlag(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut client: i32,
    mut _mode: i32,
) {
    //if not in CTF mode
    if crate::src::game::ai_dmq3::gametype != crate::bg_public_h::GT_CTF as i32 {
        return;
    }
    //
    (*bs).decisionmaker = client;
    (*bs).ordered = crate::src::qcommon::q_shared::qtrue as i32;
    (*bs).order_time = crate::src::game::ai_main::floattime;
    //set the time to send a message to the team mates
    (*bs).teammessage_time = crate::src::game::ai_main::floattime
        + 2 as i32 as f32
            * ((::libc::rand() & 0x7fff as i32) as f32
                / 0x7fff as i32 as f32);
    //set the ltg type
    (*bs).ltgtype = 6 as i32;
    //set the team goal time
    (*bs).teamgoal_time =
        crate::src::game::ai_main::floattime + 180 as i32 as f32;
    (*bs).rushbaseaway_time = 0 as i32 as f32;
    crate::src::game::ai_dmq3::BotSetTeamStatus(bs as *mut crate::src::game::ai_main::bot_state_s);
    //DEBUG
}
/*
==================
BotVoiceChat_StartLeader
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotVoiceChat_StartLeader(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut client: i32,
    mut _mode: i32,
) {
    crate::src::game::ai_dmq3::ClientName(
        client,
        (*bs).teamleader.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as i32,
    );
}
/*
==================
BotVoiceChat_StopLeader
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotVoiceChat_StopLeader(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut client: i32,
    mut _mode: i32,
) {
    let mut netname: [libc::c_char; 256] = [0; 256];
    if crate::src::qcommon::q_shared::Q_stricmp(
        (*bs).teamleader.as_mut_ptr(),
        crate::src::game::ai_dmq3::ClientName(
            client,
            netname.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as i32,
        ),
    ) == 0
    {
        (*bs).teamleader[0 as i32 as usize] = '\u{0}' as i32 as libc::c_char;
        crate::src::game::ai_cmd::notleader[client as usize] =
            crate::src::qcommon::q_shared::qtrue as i32
    };
}
/*
==================
BotVoiceChat_WhoIsLeader
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotVoiceChat_WhoIsLeader(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut _client: i32,
    mut _mode: i32,
) {
    let mut netname: [libc::c_char; 256] = [0; 256];
    if crate::src::game::ai_dmq3::TeamPlayIsOn() == 0 {
        return;
    }
    crate::src::game::ai_dmq3::ClientName(
        (*bs).client,
        netname.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as i32,
    );
    //if this bot IS the team leader
    if crate::src::qcommon::q_shared::Q_stricmp(netname.as_mut_ptr(), (*bs).teamleader.as_mut_ptr())
        == 0
    {
        crate::src::game::ai_main::BotAI_BotInitialChat(
            bs as *mut crate::src::game::ai_main::bot_state_s,
            b"iamteamleader\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as *mut libc::c_void,
        );
        crate::src::game::g_syscalls::trap_BotEnterChat(
            (*bs).cs,
            0 as i32,
            1 as i32,
        );
        crate::src::game::ai_team::BotVoiceChatOnly(
            bs as *mut crate::src::game::ai_main::bot_state_s,
            -(1 as i32),
            b"startleader\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    };
}
/*
==================
BotVoiceChat_WantOnDefense
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotVoiceChat_WantOnDefense(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut client: i32,
    mut _mode: i32,
) {
    let mut netname: [libc::c_char; 36] = [0; 36];
    let mut preference: i32 = 0;
    preference = crate::src::game::ai_team::BotGetTeamMateTaskPreference(
        bs as *mut crate::src::game::ai_main::bot_state_s,
        client,
    );
    preference &= !(2 as i32);
    preference |= 1 as i32;
    crate::src::game::ai_team::BotSetTeamMateTaskPreference(
        bs as *mut crate::src::game::ai_main::bot_state_s,
        client,
        preference,
    );
    //
    crate::src::game::ai_dmq3::EasyClientName(
        client,
        netname.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as i32,
    );
    crate::src::game::ai_main::BotAI_BotInitialChat(
        bs as *mut crate::src::game::ai_main::bot_state_s,
        b"keepinmind\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        netname.as_mut_ptr(),
        0 as *mut libc::c_void,
    );
    crate::src::game::g_syscalls::trap_BotEnterChat((*bs).cs, client, 2 as i32);
    crate::src::game::ai_team::BotVoiceChatOnly(
        bs as *mut crate::src::game::ai_main::bot_state_s,
        client,
        b"yes\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    crate::src::game::g_syscalls::trap_EA_Action((*bs).client, 0x100000 as i32);
}
/*
==================
BotVoiceChat_WantOnOffense
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotVoiceChat_WantOnOffense(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut client: i32,
    mut _mode: i32,
) {
    let mut netname: [libc::c_char; 36] = [0; 36];
    let mut preference: i32 = 0;
    preference = crate::src::game::ai_team::BotGetTeamMateTaskPreference(
        bs as *mut crate::src::game::ai_main::bot_state_s,
        client,
    );
    preference &= !(1 as i32);
    preference |= 2 as i32;
    crate::src::game::ai_team::BotSetTeamMateTaskPreference(
        bs as *mut crate::src::game::ai_main::bot_state_s,
        client,
        preference,
    );
    //
    crate::src::game::ai_dmq3::EasyClientName(
        client,
        netname.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as i32,
    );
    crate::src::game::ai_main::BotAI_BotInitialChat(
        bs as *mut crate::src::game::ai_main::bot_state_s,
        b"keepinmind\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        netname.as_mut_ptr(),
        0 as *mut libc::c_void,
    );
    crate::src::game::g_syscalls::trap_BotEnterChat((*bs).cs, client, 2 as i32);
    crate::src::game::ai_team::BotVoiceChatOnly(
        bs as *mut crate::src::game::ai_main::bot_state_s,
        client,
        b"yes\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    crate::src::game::g_syscalls::trap_EA_Action((*bs).client, 0x100000 as i32);
}
#[no_mangle]

pub unsafe extern "C" fn BotVoiceChat_Dummy(
    mut _bs: *mut crate::src::game::ai_main::bot_state_t,
    mut _client: i32,
    mut _mode: i32,
) {
}
#[no_mangle]

pub static mut voiceCommands: [voiceCommand_t; 15] = {
    [
        {
            let mut init = voiceCommand_s {
                cmd: b"getflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    BotVoiceChat_GetFlag
                        as unsafe extern "C" fn(
                            _: *mut crate::src::game::ai_main::bot_state_t,
                            _: i32,
                            _: i32,
                        ) -> (),
                ),
            };
            init
        },
        {
            let mut init = voiceCommand_s {
                cmd: b"offense\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    BotVoiceChat_Offense
                        as unsafe extern "C" fn(
                            _: *mut crate::src::game::ai_main::bot_state_t,
                            _: i32,
                            _: i32,
                        ) -> (),
                ),
            };
            init
        },
        {
            let mut init = voiceCommand_s {
                cmd: b"defend\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    BotVoiceChat_Defend
                        as unsafe extern "C" fn(
                            _: *mut crate::src::game::ai_main::bot_state_t,
                            _: i32,
                            _: i32,
                        ) -> (),
                ),
            };
            init
        },
        {
            let mut init = voiceCommand_s {
                cmd: b"defendflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    BotVoiceChat_DefendFlag
                        as unsafe extern "C" fn(
                            _: *mut crate::src::game::ai_main::bot_state_t,
                            _: i32,
                            _: i32,
                        ) -> (),
                ),
            };
            init
        },
        {
            let mut init = voiceCommand_s {
                cmd: b"patrol\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    BotVoiceChat_Patrol
                        as unsafe extern "C" fn(
                            _: *mut crate::src::game::ai_main::bot_state_t,
                            _: i32,
                            _: i32,
                        ) -> (),
                ),
            };
            init
        },
        {
            let mut init = voiceCommand_s {
                cmd: b"camp\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    BotVoiceChat_Camp
                        as unsafe extern "C" fn(
                            _: *mut crate::src::game::ai_main::bot_state_t,
                            _: i32,
                            _: i32,
                        ) -> (),
                ),
            };
            init
        },
        {
            let mut init = voiceCommand_s {
                cmd: b"followme\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    BotVoiceChat_FollowMe
                        as unsafe extern "C" fn(
                            _: *mut crate::src::game::ai_main::bot_state_t,
                            _: i32,
                            _: i32,
                        ) -> (),
                ),
            };
            init
        },
        {
            let mut init = voiceCommand_s {
                cmd: b"followflagcarrier\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                func: Some(
                    BotVoiceChat_FollowFlagCarrier
                        as unsafe extern "C" fn(
                            _: *mut crate::src::game::ai_main::bot_state_t,
                            _: i32,
                            _: i32,
                        ) -> (),
                ),
            };
            init
        },
        {
            let mut init = voiceCommand_s {
                cmd: b"returnflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    BotVoiceChat_ReturnFlag
                        as unsafe extern "C" fn(
                            _: *mut crate::src::game::ai_main::bot_state_t,
                            _: i32,
                            _: i32,
                        ) -> (),
                ),
            };
            init
        },
        {
            let mut init = voiceCommand_s {
                cmd: b"startleader\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    BotVoiceChat_StartLeader
                        as unsafe extern "C" fn(
                            _: *mut crate::src::game::ai_main::bot_state_t,
                            _: i32,
                            _: i32,
                        ) -> (),
                ),
            };
            init
        },
        {
            let mut init = voiceCommand_s {
                cmd: b"stopleader\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    BotVoiceChat_StopLeader
                        as unsafe extern "C" fn(
                            _: *mut crate::src::game::ai_main::bot_state_t,
                            _: i32,
                            _: i32,
                        ) -> (),
                ),
            };
            init
        },
        {
            let mut init = voiceCommand_s {
                cmd: b"whoisleader\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    BotVoiceChat_WhoIsLeader
                        as unsafe extern "C" fn(
                            _: *mut crate::src::game::ai_main::bot_state_t,
                            _: i32,
                            _: i32,
                        ) -> (),
                ),
            };
            init
        },
        {
            let mut init = voiceCommand_s {
                cmd: b"wantondefense\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    BotVoiceChat_WantOnDefense
                        as unsafe extern "C" fn(
                            _: *mut crate::src::game::ai_main::bot_state_t,
                            _: i32,
                            _: i32,
                        ) -> (),
                ),
            };
            init
        },
        {
            let mut init = voiceCommand_s {
                cmd: b"wantonoffense\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    BotVoiceChat_WantOnOffense
                        as unsafe extern "C" fn(
                            _: *mut crate::src::game::ai_main::bot_state_t,
                            _: i32,
                            _: i32,
                        ) -> (),
                ),
            };
            init
        },
        {
            let mut init = voiceCommand_s {
                cmd: 0 as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    BotVoiceChat_Dummy
                        as unsafe extern "C" fn(
                            _: *mut crate::src::game::ai_main::bot_state_t,
                            _: i32,
                            _: i32,
                        ) -> (),
                ),
            };
            init
        },
    ]
};
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
 * name:		ai_vcmd.h
 *
 * desc:		Quake3 bot AI
 *
 * $Archive: /source/code/botai/ai_vcmd.c $
 *
 *****************************************************************************/
#[no_mangle]

pub unsafe extern "C" fn BotVoiceChatCommand(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut mode: i32,
    mut voiceChat: *mut libc::c_char,
) -> i32 {
    let mut i: i32 = 0;
    let mut clientNum: i32 = 0;
    //int voiceOnly, color;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buf: [libc::c_char; 256] = [0; 256];
    let mut cmd: *mut libc::c_char = 0 as *mut libc::c_char;
    if crate::src::game::ai_dmq3::TeamPlayIsOn() == 0 {
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    if mode == 0 as i32 {
        return crate::src::qcommon::q_shared::qfalse as i32;
        // don't do anything with voice chats to everyone
    }
    crate::src::qcommon::q_shared::Q_strncpyz(
        buf.as_mut_ptr(),
        voiceChat,
        ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as i32,
    );
    cmd = buf.as_mut_ptr();
    while *cmd as i32 != 0 && *cmd as i32 > ' ' as i32 {
        cmd = cmd.offset(1)
    }
    while *cmd as i32 != 0 && *cmd as i32 <= ' ' as i32 {
        let fresh0 = cmd;
        cmd = cmd.offset(1);
        *fresh0 = '\u{0}' as i32 as libc::c_char
    }
    //voiceOnly = atoi(ptr);
    ptr = cmd;
    while *cmd as i32 != 0 && *cmd as i32 > ' ' as i32 {
        cmd = cmd.offset(1)
    }
    while *cmd as i32 != 0 && *cmd as i32 <= ' ' as i32 {
        let fresh1 = cmd;
        cmd = cmd.offset(1);
        *fresh1 = '\u{0}' as i32 as libc::c_char
    }
    clientNum = atoi(ptr);
    while *cmd as i32 != 0 && *cmd as i32 > ' ' as i32 {
        cmd = cmd.offset(1)
    }
    while *cmd as i32 != 0 && *cmd as i32 <= ' ' as i32 {
        let fresh2 = cmd;
        cmd = cmd.offset(1);
        *fresh2 = '\u{0}' as i32 as libc::c_char
    }
    //color = atoi(ptr);
    if crate::src::game::ai_dmq3::BotSameTeam(
        bs as *mut crate::src::game::ai_main::bot_state_s,
        clientNum,
    ) == 0
    {
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    i = 0 as i32;
    while !voiceCommands[i as usize].cmd.is_null() {
        if crate::src::qcommon::q_shared::Q_stricmp(cmd, voiceCommands[i as usize].cmd) == 0 {
            voiceCommands[i as usize]
                .func
                .expect("non-null function pointer")(bs, clientNum, mode);
            return crate::src::qcommon::q_shared::qtrue as i32;
        }
        i += 1
    }
    return crate::src::qcommon::q_shared::qfalse as i32;
}

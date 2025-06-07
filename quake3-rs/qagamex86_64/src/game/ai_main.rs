// =============== BEGIN ai_main_h ================

//

//check points
pub type bot_waypoint_t = crate::src::game::ai_main::bot_waypoint_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bot_waypoint_s {
    pub inuse: libc::c_int,
    pub name: [libc::c_char; 32],
    pub goal: crate::be_ai_goal_h::bot_goal_t,
    pub next: *mut crate::src::game::ai_main::bot_waypoint_s,
    pub prev: *mut crate::src::game::ai_main::bot_waypoint_s,
}

pub type bot_activategoal_t = crate::src::game::ai_main::bot_activategoal_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bot_activategoal_s {
    pub inuse: libc::c_int,
    pub goal: crate::be_ai_goal_h::bot_goal_t,
    pub time: libc::c_float,
    pub start_time: libc::c_float,
    pub justused_time: libc::c_float,
    pub shoot: libc::c_int,
    pub weapon: libc::c_int,
    pub target: crate::src::qcommon::q_shared::vec3_t,
    pub origin: crate::src::qcommon::q_shared::vec3_t,
    pub areas: [libc::c_int; 32],
    pub numareas: libc::c_int,
    pub areasdisabled: libc::c_int,
    pub next: *mut crate::src::game::ai_main::bot_activategoal_s,
}
//goal to activate (buttons etc.)
//time to activate something
//time starting to activate something
//time the goal was used
//true if bot has to shoot to activate
//weapon to be used for activation
//target to shoot at to activate something
//origin of the blocking entity to activate
//routing areas disabled by blocking entity
//number of disabled routing areas
//true if the areas are disabled for the routing
//next activate goal on stack
//bot state

pub type bot_state_t = crate::src::game::ai_main::bot_state_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bot_state_s {
    pub inuse: libc::c_int,
    pub botthink_residual: libc::c_int,
    pub client: libc::c_int,
    pub entitynum: libc::c_int,
    pub cur_ps: crate::src::qcommon::q_shared::playerState_t,
    pub last_eFlags: libc::c_int,
    pub lastucmd: crate::src::qcommon::q_shared::usercmd_t,
    pub entityeventTime: [libc::c_int; 1024],
    pub settings: crate::g_local_h::bot_settings_t,
    pub ainode:
        Option<unsafe extern "C" fn(_: *mut crate::src::game::ai_main::bot_state_s) -> libc::c_int>,
    pub thinktime: libc::c_float,
    pub origin: crate::src::qcommon::q_shared::vec3_t,
    pub velocity: crate::src::qcommon::q_shared::vec3_t,
    pub presencetype: libc::c_int,
    pub eye: crate::src::qcommon::q_shared::vec3_t,
    pub areanum: libc::c_int,
    pub inventory: [libc::c_int; 256],
    pub tfl: libc::c_int,
    pub flags: libc::c_int,
    pub respawn_wait: libc::c_int,
    pub lasthealth: libc::c_int,
    pub lastkilledplayer: libc::c_int,
    pub lastkilledby: libc::c_int,
    pub botdeathtype: libc::c_int,
    pub enemydeathtype: libc::c_int,
    pub botsuicide: libc::c_int,
    pub enemysuicide: libc::c_int,
    pub setupcount: libc::c_int,
    pub map_restart: libc::c_int,
    pub entergamechat: libc::c_int,
    pub num_deaths: libc::c_int,
    pub num_kills: libc::c_int,
    pub revenge_enemy: libc::c_int,
    pub revenge_kills: libc::c_int,
    pub lastframe_health: libc::c_int,
    pub lasthitcount: libc::c_int,
    pub chatto: libc::c_int,
    pub walker: libc::c_float,
    pub ltime: libc::c_float,
    pub entergame_time: libc::c_float,
    pub ltg_time: libc::c_float,
    pub nbg_time: libc::c_float,
    pub respawn_time: libc::c_float,
    pub respawnchat_time: libc::c_float,
    pub chase_time: libc::c_float,
    pub enemyvisible_time: libc::c_float,
    pub check_time: libc::c_float,
    pub stand_time: libc::c_float,
    pub lastchat_time: libc::c_float,
    pub kamikaze_time: libc::c_float,
    pub invulnerability_time: libc::c_float,
    pub standfindenemy_time: libc::c_float,
    pub attackstrafe_time: libc::c_float,
    pub attackcrouch_time: libc::c_float,
    pub attackchase_time: libc::c_float,
    pub attackjump_time: libc::c_float,
    pub enemysight_time: libc::c_float,
    pub enemydeath_time: libc::c_float,
    pub enemyposition_time: libc::c_float,
    pub defendaway_time: libc::c_float,
    pub defendaway_range: libc::c_float,
    pub rushbaseaway_time: libc::c_float,
    pub attackaway_time: libc::c_float,
    pub harvestaway_time: libc::c_float,
    pub ctfroam_time: libc::c_float,
    pub killedenemy_time: libc::c_float,
    pub arrive_time: libc::c_float,
    pub lastair_time: libc::c_float,
    pub teleport_time: libc::c_float,
    pub camp_time: libc::c_float,
    pub weaponchange_time: libc::c_float,
    pub firethrottlewait_time: libc::c_float,
    pub firethrottleshoot_time: libc::c_float,
    pub notblocked_time: libc::c_float,
    pub blockedbyavoidspot_time: libc::c_float,
    pub predictobstacles_time: libc::c_float,
    pub predictobstacles_goalareanum: libc::c_int,
    pub aimtarget: crate::src::qcommon::q_shared::vec3_t,
    pub enemyvelocity: crate::src::qcommon::q_shared::vec3_t,
    pub enemyorigin: crate::src::qcommon::q_shared::vec3_t,
    pub kamikazebody: libc::c_int,
    pub proxmines: [libc::c_int; 64],
    pub numproxmines: libc::c_int,
    pub character: libc::c_int,
    pub ms: libc::c_int,
    pub gs: libc::c_int,
    pub cs: libc::c_int,
    pub ws: libc::c_int,
    pub enemy: libc::c_int,
    pub lastenemyareanum: libc::c_int,
    pub lastenemyorigin: crate::src::qcommon::q_shared::vec3_t,
    pub weaponnum: libc::c_int,
    pub viewangles: crate::src::qcommon::q_shared::vec3_t,
    pub ideal_viewangles: crate::src::qcommon::q_shared::vec3_t,
    pub viewanglespeed: crate::src::qcommon::q_shared::vec3_t,
    pub ltgtype: libc::c_int,
    pub teammate: libc::c_int,
    pub decisionmaker: libc::c_int,
    pub ordered: libc::c_int,
    pub order_time: libc::c_float,
    pub owndecision_time: libc::c_int,
    pub teamgoal: crate::be_ai_goal_h::bot_goal_t,
    pub altroutegoal: crate::be_ai_goal_h::bot_goal_t,
    pub reachedaltroutegoal_time: libc::c_float,
    pub teammessage_time: libc::c_float,
    pub teamgoal_time: libc::c_float,
    pub teammatevisible_time: libc::c_float,
    pub teamtaskpreference: libc::c_int,
    pub lastgoal_decisionmaker: libc::c_int,
    pub lastgoal_ltgtype: libc::c_int,
    pub lastgoal_teammate: libc::c_int,
    pub lastgoal_teamgoal: crate::be_ai_goal_h::bot_goal_t,
    pub lead_teammate: libc::c_int,
    pub lead_teamgoal: crate::be_ai_goal_h::bot_goal_t,
    pub lead_time: libc::c_float,
    pub leadvisible_time: libc::c_float,
    pub leadmessage_time: libc::c_float,
    pub leadbackup_time: libc::c_float,
    pub teamleader: [libc::c_char; 36],
    pub askteamleader_time: libc::c_float,
    pub becometeamleader_time: libc::c_float,
    pub teamgiveorders_time: libc::c_float,
    pub lastflagcapture_time: libc::c_float,
    pub numteammates: libc::c_int,
    pub redflagstatus: libc::c_int,
    pub blueflagstatus: libc::c_int,
    pub neutralflagstatus: libc::c_int,
    pub flagstatuschanged: libc::c_int,
    pub forceorders: libc::c_int,
    pub flagcarrier: libc::c_int,
    pub ctfstrategy: libc::c_int,
    pub subteam: [libc::c_char; 32],
    pub formation_dist: libc::c_float,
    pub activatestack: *mut crate::src::game::ai_main::bot_activategoal_t,
    pub activategoalheap: [crate::src::game::ai_main::bot_activategoal_t; 8],
    pub checkpoints: *mut crate::src::game::ai_main::bot_waypoint_t,
    pub patrolpoints: *mut crate::src::game::ai_main::bot_waypoint_t,
    pub curpatrolpoint: *mut crate::src::game::ai_main::bot_waypoint_t,
    pub patrolflags: libc::c_int,
}
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

pub use crate::internal::__builtin_va_list;
pub use crate::internal::__va_list_tag;
pub use crate::stdarg_h::va_list;

pub use crate::bg_public_h::gitem_s;
pub use crate::bg_public_h::gitem_t;
pub use crate::bg_public_h::itemType_t;
pub use crate::bg_public_h::team_t;
pub use crate::bg_public_h::C2RustUnnamed_0;
pub use crate::bg_public_h::ET_BEAM;
pub use crate::bg_public_h::ET_EVENTS;
pub use crate::bg_public_h::ET_GENERAL;
pub use crate::bg_public_h::ET_GRAPPLE;
pub use crate::bg_public_h::ET_INVISIBLE;
pub use crate::bg_public_h::ET_ITEM;
pub use crate::bg_public_h::ET_MISSILE;
pub use crate::bg_public_h::ET_MOVER;
pub use crate::bg_public_h::ET_PLAYER;
pub use crate::bg_public_h::ET_PORTAL;
pub use crate::bg_public_h::ET_PUSH_TRIGGER;
pub use crate::bg_public_h::ET_SPEAKER;
pub use crate::bg_public_h::ET_TEAM;
pub use crate::bg_public_h::ET_TELEPORT_TRIGGER;
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
pub use crate::bg_public_h::WP_BFG;
pub use crate::bg_public_h::WP_GAUNTLET;
pub use crate::bg_public_h::WP_GRAPPLING_HOOK;
pub use crate::bg_public_h::WP_GRENADE_LAUNCHER;
pub use crate::bg_public_h::WP_LIGHTNING;
pub use crate::bg_public_h::WP_MACHINEGUN;
pub use crate::bg_public_h::WP_NONE;
pub use crate::bg_public_h::WP_NUM_WEAPONS;
pub use crate::bg_public_h::WP_PLASMAGUN;
pub use crate::bg_public_h::WP_RAILGUN;
pub use crate::bg_public_h::WP_ROCKET_LAUNCHER;
pub use crate::bg_public_h::WP_SHOTGUN;
pub use crate::g_public_h::entityShared_t;
pub use crate::src::qcommon::q_math::AngleMod;
pub use crate::src::qcommon::q_math::AngleVectors;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::cplane_s;
pub use crate::src::qcommon::q_shared::cplane_t;
pub use crate::src::qcommon::q_shared::cvarHandle_t;
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
pub use crate::src::qcommon::q_shared::va;
pub use crate::src::qcommon::q_shared::vec3_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::qcommon::q_shared::vmCvar_t;
pub use crate::src::qcommon::q_shared::Com_sprintf;
pub use crate::src::qcommon::q_shared::Info_ValueForKey;
pub use crate::src::qcommon::q_shared::Q_IsColorString;
pub use crate::src::qcommon::q_shared::Q_stricmp;
pub use crate::src::qcommon::q_shared::EXEC_APPEND;
pub use crate::src::qcommon::q_shared::EXEC_INSERT;
pub use crate::src::qcommon::q_shared::EXEC_NOW;
pub use crate::src::qcommon::q_shared::TR_GRAVITY;
pub use crate::src::qcommon::q_shared::TR_INTERPOLATE;
pub use crate::src::qcommon::q_shared::TR_LINEAR;
pub use crate::src::qcommon::q_shared::TR_LINEAR_STOP;
pub use crate::src::qcommon::q_shared::TR_SINE;
pub use crate::src::qcommon::q_shared::TR_STATIONARY;

pub use crate::be_aas_h::aas_areainfo_s;
pub use crate::be_aas_h::aas_areainfo_t;
pub use crate::be_aas_h::aas_entityinfo_s;
pub use crate::be_aas_h::aas_entityinfo_t;
pub use crate::be_aas_h::SOLID_BBOX;
pub use crate::be_aas_h::SOLID_BSP;
pub use crate::be_aas_h::SOLID_NOT;
pub use crate::be_aas_h::SOLID_TRIGGER;
pub use crate::be_ai_goal_h::bot_goal_s;
pub use crate::be_ai_goal_h::bot_goal_t;
pub use crate::botlib_h::bot_entitystate_s;
pub use crate::botlib_h::bot_entitystate_t;
pub use crate::botlib_h::bot_input_s;
pub use crate::botlib_h::bot_input_t;
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
pub use crate::src::game::g_bot::G_CheckBotSpawn;
pub use crate::src::game::g_main::g_entities;
pub use crate::src::game::g_main::level;
pub use crate::src::game::g_main::G_Error;
pub use crate::src::game::g_main::G_Printf;
pub use crate::src::game::g_mem::G_Alloc;
pub use crate::src::game::g_syscalls::trap_AAS_AreaInfo;
pub use crate::src::game::g_syscalls::trap_AAS_EntityInfo;
pub use crate::src::game::g_syscalls::trap_AAS_Initialized;
pub use crate::src::game::g_syscalls::trap_AAS_Time;
pub use crate::src::game::g_syscalls::trap_BotAllocChatState;
pub use crate::src::game::g_syscalls::trap_BotAllocGoalState;
pub use crate::src::game::g_syscalls::trap_BotAllocMoveState;
pub use crate::src::game::g_syscalls::trap_BotAllocWeaponState;
pub use crate::src::game::g_syscalls::trap_BotEnterChat;
pub use crate::src::game::g_syscalls::trap_BotFreeCharacter;
pub use crate::src::game::g_syscalls::trap_BotFreeChatState;
pub use crate::src::game::g_syscalls::trap_BotFreeGoalState;
pub use crate::src::game::g_syscalls::trap_BotFreeMoveState;
pub use crate::src::game::g_syscalls::trap_BotFreeWeaponState;
pub use crate::src::game::g_syscalls::trap_BotGetServerCommand;
pub use crate::src::game::g_syscalls::trap_BotGetSnapshotEntity;
pub use crate::src::game::g_syscalls::trap_BotGetTopGoal;
pub use crate::src::game::g_syscalls::trap_BotGoalName;
pub use crate::src::game::g_syscalls::trap_BotInitialChat;
pub use crate::src::game::g_syscalls::trap_BotInterbreedGoalFuzzyLogic;
pub use crate::src::game::g_syscalls::trap_BotLibLoadMap;
pub use crate::src::game::g_syscalls::trap_BotLibSetup;
pub use crate::src::game::g_syscalls::trap_BotLibShutdown;
pub use crate::src::game::g_syscalls::trap_BotLibStartFrame;
pub use crate::src::game::g_syscalls::trap_BotLibUpdateEntity;
pub use crate::src::game::g_syscalls::trap_BotLibVarSet;
pub use crate::src::game::g_syscalls::trap_BotLoadCharacter;
pub use crate::src::game::g_syscalls::trap_BotLoadChatFile;
pub use crate::src::game::g_syscalls::trap_BotLoadItemWeights;
pub use crate::src::game::g_syscalls::trap_BotLoadWeaponWeights;
pub use crate::src::game::g_syscalls::trap_BotMutateGoalFuzzyLogic;
pub use crate::src::game::g_syscalls::trap_BotQueueConsoleMessage;
pub use crate::src::game::g_syscalls::trap_BotResetAvoidGoals;
pub use crate::src::game::g_syscalls::trap_BotResetAvoidReach;
pub use crate::src::game::g_syscalls::trap_BotResetGoalState;
pub use crate::src::game::g_syscalls::trap_BotResetMoveState;
pub use crate::src::game::g_syscalls::trap_BotResetWeaponState;
pub use crate::src::game::g_syscalls::trap_BotSaveGoalFuzzyLogic;
pub use crate::src::game::g_syscalls::trap_BotSetChatGender;
pub use crate::src::game::g_syscalls::trap_BotUpdateEntityItems;
pub use crate::src::game::g_syscalls::trap_BotUserCommand;
pub use crate::src::game::g_syscalls::trap_Characteristic_BFloat;
pub use crate::src::game::g_syscalls::trap_Characteristic_String;
pub use crate::src::game::g_syscalls::trap_Cvar_Register;
pub use crate::src::game::g_syscalls::trap_Cvar_Set;
pub use crate::src::game::g_syscalls::trap_Cvar_Update;
pub use crate::src::game::g_syscalls::trap_Cvar_VariableIntegerValue;
pub use crate::src::game::g_syscalls::trap_Cvar_VariableStringBuffer;
pub use crate::src::game::g_syscalls::trap_EA_GetInput;
pub use crate::src::game::g_syscalls::trap_EA_ResetInput;
pub use crate::src::game::g_syscalls::trap_EA_SelectWeapon;
pub use crate::src::game::g_syscalls::trap_EA_View;
pub use crate::src::game::g_syscalls::trap_GeneticParentsAndChildSelection;
pub use crate::src::game::g_syscalls::trap_GetConfigstring;
pub use crate::src::game::g_syscalls::trap_SendConsoleCommand;
pub use crate::src::game::g_syscalls::trap_SetConfigstring;
pub use crate::src::game::g_syscalls::trap_Trace;

pub use crate::src::game::ai_main::stdlib_h::atoi;

pub use ::libc::strtol;
extern "C" {
    #[no_mangle]
    pub fn ExitLevel();
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
 * name:		ai_main.c
 *
 * desc:		Quake3 bot AI
 *
 * $Archive: /MissionPack/code/game/ai_main.c $
 *
 *****************************************************************************/
//
//
//bot states
#[no_mangle]

pub static mut botstates: [*mut crate::src::game::ai_main::bot_state_t; 64] =
    [0 as *const crate::src::game::ai_main::bot_state_t
        as *mut crate::src::game::ai_main::bot_state_t; 64];
//number of bots
#[no_mangle]

pub static mut numbots: libc::c_int = 0;
//floating point time
#[no_mangle]

pub static mut floattime: libc::c_float = 0.;
//time to do a regular update
#[no_mangle]

pub static mut regularupdate_time: libc::c_float = 0.;
//
#[no_mangle]

pub static mut bot_interbreed: libc::c_int = 0;
#[no_mangle]

pub static mut bot_interbreedmatchcount: libc::c_int = 0;
//
#[no_mangle]

pub static mut bot_thinktime: crate::src::qcommon::q_shared::vmCvar_t =
    crate::src::qcommon::q_shared::vmCvar_t {
        handle: 0,
        modificationCount: 0,
        value: 0.,
        integer: 0,
        string: [0; 256],
    };
#[no_mangle]

pub static mut bot_memorydump: crate::src::qcommon::q_shared::vmCvar_t =
    crate::src::qcommon::q_shared::vmCvar_t {
        handle: 0,
        modificationCount: 0,
        value: 0.,
        integer: 0,
        string: [0; 256],
    };
#[no_mangle]

pub static mut bot_saveroutingcache: crate::src::qcommon::q_shared::vmCvar_t =
    crate::src::qcommon::q_shared::vmCvar_t {
        handle: 0,
        modificationCount: 0,
        value: 0.,
        integer: 0,
        string: [0; 256],
    };
#[no_mangle]

pub static mut bot_pause: crate::src::qcommon::q_shared::vmCvar_t =
    crate::src::qcommon::q_shared::vmCvar_t {
        handle: 0,
        modificationCount: 0,
        value: 0.,
        integer: 0,
        string: [0; 256],
    };
#[no_mangle]

pub static mut bot_report: crate::src::qcommon::q_shared::vmCvar_t =
    crate::src::qcommon::q_shared::vmCvar_t {
        handle: 0,
        modificationCount: 0,
        value: 0.,
        integer: 0,
        string: [0; 256],
    };
#[no_mangle]

pub static mut bot_testsolid: crate::src::qcommon::q_shared::vmCvar_t =
    crate::src::qcommon::q_shared::vmCvar_t {
        handle: 0,
        modificationCount: 0,
        value: 0.,
        integer: 0,
        string: [0; 256],
    };
#[no_mangle]

pub static mut bot_testclusters: crate::src::qcommon::q_shared::vmCvar_t =
    crate::src::qcommon::q_shared::vmCvar_t {
        handle: 0,
        modificationCount: 0,
        value: 0.,
        integer: 0,
        string: [0; 256],
    };
#[no_mangle]

pub static mut bot_developer: crate::src::qcommon::q_shared::vmCvar_t =
    crate::src::qcommon::q_shared::vmCvar_t {
        handle: 0,
        modificationCount: 0,
        value: 0.,
        integer: 0,
        string: [0; 256],
    };
#[no_mangle]

pub static mut bot_interbreedchar: crate::src::qcommon::q_shared::vmCvar_t =
    crate::src::qcommon::q_shared::vmCvar_t {
        handle: 0,
        modificationCount: 0,
        value: 0.,
        integer: 0,
        string: [0; 256],
    };
#[no_mangle]

pub static mut bot_interbreedbots: crate::src::qcommon::q_shared::vmCvar_t =
    crate::src::qcommon::q_shared::vmCvar_t {
        handle: 0,
        modificationCount: 0,
        value: 0.,
        integer: 0,
        string: [0; 256],
    };
#[no_mangle]

pub static mut bot_interbreedcycle: crate::src::qcommon::q_shared::vmCvar_t =
    crate::src::qcommon::q_shared::vmCvar_t {
        handle: 0,
        modificationCount: 0,
        value: 0.,
        integer: 0,
        string: [0; 256],
    };
#[no_mangle]

pub static mut bot_interbreedwrite: crate::src::qcommon::q_shared::vmCvar_t =
    crate::src::qcommon::q_shared::vmCvar_t {
        handle: 0,
        modificationCount: 0,
        value: 0.,
        integer: 0,
        string: [0; 256],
    };
// from the game source
/*
==================
BotAI_Print
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotAI_Print(
    mut type_0: libc::c_int,
    mut fmt: *mut libc::c_char,
    mut args: ...
) {
    let mut str: [libc::c_char; 2048] = [0; 2048];
    let mut ap: ::std::ffi::VaListImpl;
    ap = args.clone();
    crate::stdlib::vsnprintf(
        str.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 2048]>() as libc::c_ulong,
        fmt,
        ap.as_va_list(),
    );
    match type_0 {
        1 => {
            crate::src::game::g_main::G_Printf(
                b"%s\x00" as *const u8 as *const libc::c_char,
                str.as_mut_ptr(),
            );
        }
        2 => {
            crate::src::game::g_main::G_Printf(
                b"^3Warning: %s\x00" as *const u8 as *const libc::c_char,
                str.as_mut_ptr(),
            );
        }
        3 => {
            crate::src::game::g_main::G_Printf(
                b"^1Error: %s\x00" as *const u8 as *const libc::c_char,
                str.as_mut_ptr(),
            );
        }
        4 => {
            crate::src::game::g_main::G_Printf(
                b"^1Fatal: %s\x00" as *const u8 as *const libc::c_char,
                str.as_mut_ptr(),
            );
        }
        5 => {
            crate::src::game::g_main::G_Error(
                b"^1Exit: %s\x00" as *const u8 as *const libc::c_char,
                str.as_mut_ptr(),
            );
        }
        _ => {
            crate::src::game::g_main::G_Printf(
                b"unknown print type\n\x00" as *const u8 as *const libc::c_char,
            );
        }
    };
}
/*
==================
BotAI_Trace
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotAI_Trace(
    mut bsptrace: *mut crate::botlib_h::bsp_trace_t,
    mut start: *mut crate::src::qcommon::q_shared::vec_t,
    mut mins: *mut crate::src::qcommon::q_shared::vec_t,
    mut maxs: *mut crate::src::qcommon::q_shared::vec_t,
    mut end: *mut crate::src::qcommon::q_shared::vec_t,
    mut passent: libc::c_int,
    mut contentmask: libc::c_int,
) {
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
    crate::src::game::g_syscalls::trap_Trace(
        &mut trace as *mut _ as *mut crate::src::qcommon::q_shared::trace_t,
        start as *const crate::src::qcommon::q_shared::vec_t,
        mins as *const crate::src::qcommon::q_shared::vec_t,
        maxs as *const crate::src::qcommon::q_shared::vec_t,
        end as *const crate::src::qcommon::q_shared::vec_t,
        passent,
        contentmask,
    );
    //copy the trace information
    (*bsptrace).allsolid = trace.allsolid;
    (*bsptrace).startsolid = trace.startsolid;
    (*bsptrace).fraction = trace.fraction;
    (*bsptrace).endpos[0 as libc::c_int as usize] = trace.endpos[0 as libc::c_int as usize];
    (*bsptrace).endpos[1 as libc::c_int as usize] = trace.endpos[1 as libc::c_int as usize];
    (*bsptrace).endpos[2 as libc::c_int as usize] = trace.endpos[2 as libc::c_int as usize];
    (*bsptrace).plane.dist = trace.plane.dist;
    (*bsptrace).plane.normal[0 as libc::c_int as usize] =
        trace.plane.normal[0 as libc::c_int as usize];
    (*bsptrace).plane.normal[1 as libc::c_int as usize] =
        trace.plane.normal[1 as libc::c_int as usize];
    (*bsptrace).plane.normal[2 as libc::c_int as usize] =
        trace.plane.normal[2 as libc::c_int as usize];
    (*bsptrace).plane.signbits = trace.plane.signbits;
    (*bsptrace).plane.type_0 = trace.plane.type_0;
    (*bsptrace).surface.value = 0 as libc::c_int;
    (*bsptrace).surface.flags = trace.surfaceFlags;
    (*bsptrace).ent = trace.entityNum;
    (*bsptrace).exp_dist = 0 as libc::c_int as libc::c_float;
    (*bsptrace).sidenum = 0 as libc::c_int;
    (*bsptrace).contents = 0 as libc::c_int;
}
/*
==================
BotAI_GetClientState
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotAI_GetClientState(
    mut clientNum: libc::c_int,
    mut state: *mut crate::src::qcommon::q_shared::playerState_t,
) -> libc::c_int {
    let mut ent: *mut crate::g_local_h::gentity_t = 0 as *mut crate::g_local_h::gentity_t;
    ent = &mut *crate::src::game::g_main::g_entities
        .as_mut_ptr()
        .offset(clientNum as isize) as *mut crate::g_local_h::gentity_t;
    if (*ent).inuse as u64 == 0 {
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    }
    if (*ent).client.is_null() {
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    }
    crate::stdlib::memcpy(
        state as *mut libc::c_void,
        &mut (*(*ent).client).ps as *mut crate::src::qcommon::q_shared::playerState_t
            as *const libc::c_void,
        ::std::mem::size_of::<crate::src::qcommon::q_shared::playerState_t>() as libc::c_ulong,
    );
    return crate::src::qcommon::q_shared::qtrue as libc::c_int;
}
/*
==================
BotAI_GetEntityState
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotAI_GetEntityState(
    mut entityNum: libc::c_int,
    mut state: *mut crate::src::qcommon::q_shared::entityState_t,
) -> libc::c_int {
    let mut ent: *mut crate::g_local_h::gentity_t = 0 as *mut crate::g_local_h::gentity_t;
    ent = &mut *crate::src::game::g_main::g_entities
        .as_mut_ptr()
        .offset(entityNum as isize) as *mut crate::g_local_h::gentity_t;
    crate::stdlib::memset(
        state as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<crate::src::qcommon::q_shared::entityState_t>() as libc::c_ulong,
    );
    if (*ent).inuse as u64 == 0 {
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    }
    if (*ent).r.linked as u64 == 0 {
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    }
    if (*ent).r.svFlags & 0x1 as libc::c_int != 0 {
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    }
    crate::stdlib::memcpy(
        state as *mut libc::c_void,
        &mut (*ent).s as *mut crate::src::qcommon::q_shared::entityState_t as *const libc::c_void,
        ::std::mem::size_of::<crate::src::qcommon::q_shared::entityState_t>() as libc::c_ulong,
    );
    return crate::src::qcommon::q_shared::qtrue as libc::c_int;
}
/*
==================
BotAI_GetSnapshotEntity
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotAI_GetSnapshotEntity(
    mut clientNum: libc::c_int,
    mut sequence: libc::c_int,
    mut state: *mut crate::src::qcommon::q_shared::entityState_t,
) -> libc::c_int {
    let mut entNum: libc::c_int = 0;
    entNum = crate::src::game::g_syscalls::trap_BotGetSnapshotEntity(clientNum, sequence);
    if entNum == -(1 as libc::c_int) {
        crate::stdlib::memset(
            state as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<crate::src::qcommon::q_shared::entityState_t>() as libc::c_ulong,
        );
        return -(1 as libc::c_int);
    }
    BotAI_GetEntityState(entNum, state);
    return sequence + 1 as libc::c_int;
}
/*
==================
BotAI_BotInitialChat
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotAI_BotInitialChat(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut type_0: *mut libc::c_char,
    mut args: ...
) {
    let mut i: libc::c_int = 0;
    let mut mcontext: libc::c_int = 0;
    let mut ap: ::std::ffi::VaListImpl;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut vars: [*mut libc::c_char; 8] = [0 as *mut libc::c_char; 8];
    crate::stdlib::memset(
        vars.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[*mut libc::c_char; 8]>() as libc::c_ulong,
    );
    ap = args.clone();
    p = ap.as_va_list().arg::<*mut libc::c_char>();
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        if p.is_null() {
            break;
        }
        vars[i as usize] = p;
        p = ap.as_va_list().arg::<*mut libc::c_char>();
        i += 1
    }
    mcontext = crate::src::game::ai_dmq3::BotSynonymContext(
        bs as *mut crate::src::game::ai_main::bot_state_s,
    );
    crate::src::game::g_syscalls::trap_BotInitialChat(
        (*bs).cs,
        type_0,
        mcontext,
        vars[0 as libc::c_int as usize],
        vars[1 as libc::c_int as usize],
        vars[2 as libc::c_int as usize],
        vars[3 as libc::c_int as usize],
        vars[4 as libc::c_int as usize],
        vars[5 as libc::c_int as usize],
        vars[6 as libc::c_int as usize],
        vars[7 as libc::c_int as usize],
    );
}
/*
==================
BotTestAAS
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotTestAAS(mut origin: *mut crate::src::qcommon::q_shared::vec_t) {
    let mut areanum: libc::c_int = 0;
    let mut info: crate::be_aas_h::aas_areainfo_t = crate::be_aas_h::aas_areainfo_t {
        contents: 0,
        flags: 0,
        presencetype: 0,
        cluster: 0,
        mins: [0.; 3],
        maxs: [0.; 3],
        center: [0.; 3],
    };
    crate::src::game::g_syscalls::trap_Cvar_Update(
        &mut bot_testsolid as *mut _ as *mut crate::src::qcommon::q_shared::vmCvar_t,
    );
    crate::src::game::g_syscalls::trap_Cvar_Update(
        &mut bot_testclusters as *mut _ as *mut crate::src::qcommon::q_shared::vmCvar_t,
    );
    if bot_testsolid.integer != 0 {
        if crate::src::game::g_syscalls::trap_AAS_Initialized() == 0 {
            return;
        }
        areanum = crate::src::game::ai_dmq3::BotPointAreaNum(origin);
        if areanum != 0 {
            BotAI_Print(
                1 as libc::c_int,
                b"\rempty area\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        } else {
            BotAI_Print(
                1 as libc::c_int,
                b"\r^1SOLID area\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        }
    } else if bot_testclusters.integer != 0 {
        if crate::src::game::g_syscalls::trap_AAS_Initialized() == 0 {
            return;
        }
        areanum = crate::src::game::ai_dmq3::BotPointAreaNum(origin);
        if areanum == 0 {
            BotAI_Print(
                1 as libc::c_int,
                b"\r^1Solid!                              \x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        } else {
            crate::src::game::g_syscalls::trap_AAS_AreaInfo(
                areanum,
                &mut info as *mut crate::be_aas_h::aas_areainfo_t as *mut libc::c_void,
            );
            BotAI_Print(
                1 as libc::c_int,
                b"\rarea %d, cluster %d       \x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                areanum,
                info.cluster,
            );
        }
    };
}
/*
==================
BotReportStatus
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotReportStatus(mut bs: *mut crate::src::game::ai_main::bot_state_t) {
    let mut goalname: [libc::c_char; 256] = [0; 256];
    let mut netname: [libc::c_char; 256] = [0; 256];
    let mut leader: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut flagstatus: [libc::c_char; 32] = [0; 32];
    //
    crate::src::game::ai_dmq3::ClientName(
        (*bs).client,
        netname.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
    );
    if crate::src::qcommon::q_shared::Q_stricmp(netname.as_mut_ptr(), (*bs).teamleader.as_mut_ptr())
        == 0 as libc::c_int
    {
        leader = b"L\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    } else {
        leader = b" \x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    }
    ::libc::strcpy(
        flagstatus.as_mut_ptr(),
        b"  \x00" as *const u8 as *const libc::c_char,
    );
    if crate::src::game::ai_dmq3::gametype == crate::bg_public_h::GT_CTF as libc::c_int {
        if crate::src::game::ai_dmq3::BotCTFCarryingFlag(
            bs as *mut crate::src::game::ai_main::bot_state_s,
        ) != 0
        {
            if crate::src::game::ai_dmq3::BotTeam(bs as *mut crate::src::game::ai_main::bot_state_s)
                == crate::bg_public_h::TEAM_RED as libc::c_int
            {
                ::libc::strcpy(
                    flagstatus.as_mut_ptr(),
                    b"^1F \x00" as *const u8 as *const libc::c_char,
                );
            } else {
                ::libc::strcpy(
                    flagstatus.as_mut_ptr(),
                    b"^4F \x00" as *const u8 as *const libc::c_char,
                );
            }
        }
    }
    match (*bs).ltgtype {
        1 => {
            crate::src::game::ai_dmq3::EasyClientName(
                (*bs).teammate,
                goalname.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
            );
            BotAI_Print(
                1 as libc::c_int,
                b"%-20s%s%s: helping %s\n\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                netname.as_mut_ptr(),
                leader,
                flagstatus.as_mut_ptr(),
                goalname.as_mut_ptr(),
            );
        }
        2 => {
            crate::src::game::ai_dmq3::EasyClientName(
                (*bs).teammate,
                goalname.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
            );
            BotAI_Print(
                1 as libc::c_int,
                b"%-20s%s%s: accompanying %s\n\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                netname.as_mut_ptr(),
                leader,
                flagstatus.as_mut_ptr(),
                goalname.as_mut_ptr(),
            );
        }
        3 => {
            crate::src::game::g_syscalls::trap_BotGoalName(
                (*bs).teamgoal.number,
                goalname.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
            );
            BotAI_Print(
                1 as libc::c_int,
                b"%-20s%s%s: defending %s\n\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                netname.as_mut_ptr(),
                leader,
                flagstatus.as_mut_ptr(),
                goalname.as_mut_ptr(),
            );
        }
        10 => {
            crate::src::game::g_syscalls::trap_BotGoalName(
                (*bs).teamgoal.number,
                goalname.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
            );
            BotAI_Print(
                1 as libc::c_int,
                b"%-20s%s%s: getting item %s\n\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                netname.as_mut_ptr(),
                leader,
                flagstatus.as_mut_ptr(),
                goalname.as_mut_ptr(),
            );
        }
        11 => {
            crate::src::game::ai_dmq3::ClientName(
                (*bs).teamgoal.entitynum,
                goalname.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
            );
            BotAI_Print(
                1 as libc::c_int,
                b"%-20s%s%s: killing %s\n\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                netname.as_mut_ptr(),
                leader,
                flagstatus.as_mut_ptr(),
                goalname.as_mut_ptr(),
            );
        }
        7 | 8 => {
            BotAI_Print(
                1 as libc::c_int,
                b"%-20s%s%s: camping\n\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                netname.as_mut_ptr(),
                leader,
                flagstatus.as_mut_ptr(),
            );
        }
        9 => {
            BotAI_Print(
                1 as libc::c_int,
                b"%-20s%s%s: patrolling\n\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                netname.as_mut_ptr(),
                leader,
                flagstatus.as_mut_ptr(),
            );
        }
        4 => {
            BotAI_Print(
                1 as libc::c_int,
                b"%-20s%s%s: capturing flag\n\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                netname.as_mut_ptr(),
                leader,
                flagstatus.as_mut_ptr(),
            );
        }
        5 => {
            BotAI_Print(
                1 as libc::c_int,
                b"%-20s%s%s: rushing base\n\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                netname.as_mut_ptr(),
                leader,
                flagstatus.as_mut_ptr(),
            );
        }
        6 => {
            BotAI_Print(
                1 as libc::c_int,
                b"%-20s%s%s: returning flag\n\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                netname.as_mut_ptr(),
                leader,
                flagstatus.as_mut_ptr(),
            );
        }
        13 => {
            BotAI_Print(
                1 as libc::c_int,
                b"%-20s%s%s: attacking the enemy base\n\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                netname.as_mut_ptr(),
                leader,
                flagstatus.as_mut_ptr(),
            );
        }
        12 => {
            BotAI_Print(
                1 as libc::c_int,
                b"%-20s%s%s: harvesting\n\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                netname.as_mut_ptr(),
                leader,
                flagstatus.as_mut_ptr(),
            );
        }
        _ => {
            BotAI_Print(
                1 as libc::c_int,
                b"%-20s%s%s: roaming\n\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                netname.as_mut_ptr(),
                leader,
                flagstatus.as_mut_ptr(),
            );
        }
    };
}
/*
==================
BotTeamplayReport
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotTeamplayReport() {
    let mut i: libc::c_int = 0;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    BotAI_Print(
        1 as libc::c_int,
        b"^1RED\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    i = 0 as libc::c_int;
    while i < crate::src::game::g_main::level.maxclients {
        //
        if !(botstates[i as usize].is_null() || (*botstates[i as usize]).inuse == 0) {
            //
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
                if atoi(crate::src::qcommon::q_shared::Info_ValueForKey(
                    buf.as_mut_ptr(),
                    b"t\x00" as *const u8 as *const libc::c_char,
                )) == crate::bg_public_h::TEAM_RED as libc::c_int
                {
                    BotReportStatus(botstates[i as usize]);
                }
            }
        }
        i += 1
    }
    BotAI_Print(
        1 as libc::c_int,
        b"^4BLUE\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    i = 0 as libc::c_int;
    while i < crate::src::game::g_main::level.maxclients {
        //
        if !(botstates[i as usize].is_null() || (*botstates[i as usize]).inuse == 0) {
            //
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
                if atoi(crate::src::qcommon::q_shared::Info_ValueForKey(
                    buf.as_mut_ptr(),
                    b"t\x00" as *const u8 as *const libc::c_char,
                )) == crate::bg_public_h::TEAM_BLUE as libc::c_int
                {
                    BotReportStatus(botstates[i as usize]);
                }
            }
        }
        i += 1
    }
}
/*
==================
BotSetInfoConfigString
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotSetInfoConfigString(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
) {
    let mut goalname: [libc::c_char; 256] = [0; 256];
    let mut netname: [libc::c_char; 256] = [0; 256];
    let mut action: [libc::c_char; 256] = [0; 256];
    let mut leader: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut carrying: [libc::c_char; 32] = [0; 32];
    let mut cs: *mut libc::c_char = 0 as *mut libc::c_char;
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
    //
    crate::src::game::ai_dmq3::ClientName(
        (*bs).client,
        netname.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
    );
    if crate::src::qcommon::q_shared::Q_stricmp(netname.as_mut_ptr(), (*bs).teamleader.as_mut_ptr())
        == 0 as libc::c_int
    {
        leader = b"L\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    } else {
        leader = b" \x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    }
    ::libc::strcpy(
        carrying.as_mut_ptr(),
        b"  \x00" as *const u8 as *const libc::c_char,
    );
    if crate::src::game::ai_dmq3::gametype == crate::bg_public_h::GT_CTF as libc::c_int {
        if crate::src::game::ai_dmq3::BotCTFCarryingFlag(
            bs as *mut crate::src::game::ai_main::bot_state_s,
        ) != 0
        {
            ::libc::strcpy(
                carrying.as_mut_ptr(),
                b"F \x00" as *const u8 as *const libc::c_char,
            );
        }
    }
    match (*bs).ltgtype {
        1 => {
            crate::src::game::ai_dmq3::EasyClientName(
                (*bs).teammate,
                goalname.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
            );
            crate::src::qcommon::q_shared::Com_sprintf(
                action.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
                b"helping %s\x00" as *const u8 as *const libc::c_char,
                goalname.as_mut_ptr(),
            );
        }
        2 => {
            crate::src::game::ai_dmq3::EasyClientName(
                (*bs).teammate,
                goalname.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
            );
            crate::src::qcommon::q_shared::Com_sprintf(
                action.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
                b"accompanying %s\x00" as *const u8 as *const libc::c_char,
                goalname.as_mut_ptr(),
            );
        }
        3 => {
            crate::src::game::g_syscalls::trap_BotGoalName(
                (*bs).teamgoal.number,
                goalname.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
            );
            crate::src::qcommon::q_shared::Com_sprintf(
                action.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
                b"defending %s\x00" as *const u8 as *const libc::c_char,
                goalname.as_mut_ptr(),
            );
        }
        10 => {
            crate::src::game::g_syscalls::trap_BotGoalName(
                (*bs).teamgoal.number,
                goalname.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
            );
            crate::src::qcommon::q_shared::Com_sprintf(
                action.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
                b"getting item %s\x00" as *const u8 as *const libc::c_char,
                goalname.as_mut_ptr(),
            );
        }
        11 => {
            crate::src::game::ai_dmq3::ClientName(
                (*bs).teamgoal.entitynum,
                goalname.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
            );
            crate::src::qcommon::q_shared::Com_sprintf(
                action.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
                b"killing %s\x00" as *const u8 as *const libc::c_char,
                goalname.as_mut_ptr(),
            );
        }
        7 | 8 => {
            crate::src::qcommon::q_shared::Com_sprintf(
                action.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
                b"camping\x00" as *const u8 as *const libc::c_char,
            );
        }
        9 => {
            crate::src::qcommon::q_shared::Com_sprintf(
                action.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
                b"patrolling\x00" as *const u8 as *const libc::c_char,
            );
        }
        4 => {
            crate::src::qcommon::q_shared::Com_sprintf(
                action.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
                b"capturing flag\x00" as *const u8 as *const libc::c_char,
            );
        }
        5 => {
            crate::src::qcommon::q_shared::Com_sprintf(
                action.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
                b"rushing base\x00" as *const u8 as *const libc::c_char,
            );
        }
        6 => {
            crate::src::qcommon::q_shared::Com_sprintf(
                action.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
                b"returning flag\x00" as *const u8 as *const libc::c_char,
            );
        }
        13 => {
            crate::src::qcommon::q_shared::Com_sprintf(
                action.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
                b"attacking the enemy base\x00" as *const u8 as *const libc::c_char,
            );
        }
        12 => {
            crate::src::qcommon::q_shared::Com_sprintf(
                action.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
                b"harvesting\x00" as *const u8 as *const libc::c_char,
            );
        }
        _ => {
            crate::src::game::g_syscalls::trap_BotGetTopGoal(
                (*bs).gs,
                &mut goal as *mut crate::be_ai_goal_h::bot_goal_t as *mut libc::c_void,
            );
            crate::src::game::g_syscalls::trap_BotGoalName(
                goal.number,
                goalname.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
            );
            crate::src::qcommon::q_shared::Com_sprintf(
                action.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
                b"roaming %s\x00" as *const u8 as *const libc::c_char,
                goalname.as_mut_ptr(),
            );
        }
    }
    cs = crate::src::qcommon::q_shared::va(
        b"l\\%s\\c\\%s\\a\\%s\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        leader,
        carrying.as_mut_ptr(),
        action.as_mut_ptr(),
    );
    crate::src::game::g_syscalls::trap_SetConfigstring(25 as libc::c_int + (*bs).client, cs);
}
/*
==============
BotUpdateInfoConfigStrings
==============
*/
#[no_mangle]

pub unsafe extern "C" fn BotUpdateInfoConfigStrings() {
    let mut i: libc::c_int = 0;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    i = 0 as libc::c_int;
    while i < crate::src::game::g_main::level.maxclients {
        //
        if !(botstates[i as usize].is_null() || (*botstates[i as usize]).inuse == 0) {
            //
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
                BotSetInfoConfigString(botstates[i as usize]);
            }
        }
        i += 1
    }
}
/*
==============
BotInterbreedBots
==============
*/
#[no_mangle]

pub unsafe extern "C" fn BotInterbreedBots() {
    let mut ranks: [libc::c_float; 64] = [0.; 64];
    let mut parent1: libc::c_int = 0;
    let mut parent2: libc::c_int = 0;
    let mut child: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    // get rankings for all the bots
    i = 0 as libc::c_int;
    while i < 64 as libc::c_int {
        if !botstates[i as usize].is_null() && (*botstates[i as usize]).inuse != 0 {
            ranks[i as usize] = ((*botstates[i as usize]).num_kills * 2 as libc::c_int
                - (*botstates[i as usize]).num_deaths)
                as libc::c_float
        } else {
            ranks[i as usize] = -(1 as libc::c_int) as libc::c_float
        }
        i += 1
    }
    if crate::src::game::g_syscalls::trap_GeneticParentsAndChildSelection(
        64 as libc::c_int,
        ranks.as_mut_ptr(),
        &mut parent1,
        &mut parent2,
        &mut child,
    ) != 0
    {
        crate::src::game::g_syscalls::trap_BotInterbreedGoalFuzzyLogic(
            (*botstates[parent1 as usize]).gs,
            (*botstates[parent2 as usize]).gs,
            (*botstates[child as usize]).gs,
        );
        crate::src::game::g_syscalls::trap_BotMutateGoalFuzzyLogic(
            (*botstates[child as usize]).gs,
            1 as libc::c_int as libc::c_float,
        );
    }
    // reset the kills and deaths
    i = 0 as libc::c_int;
    while i < 64 as libc::c_int {
        if !botstates[i as usize].is_null() && (*botstates[i as usize]).inuse != 0 {
            (*botstates[i as usize]).num_kills = 0 as libc::c_int;
            (*botstates[i as usize]).num_deaths = 0 as libc::c_int
        }
        i += 1
    }
}
/*
==============
BotWriteInterbreeded
==============
*/
#[no_mangle]

pub unsafe extern "C" fn BotWriteInterbreeded(mut filename: *mut libc::c_char) {
    let mut rank: libc::c_float = 0.;
    let mut bestrank: libc::c_float = 0.;
    let mut i: libc::c_int = 0;
    let mut bestbot: libc::c_int = 0;
    bestrank = 0 as libc::c_int as libc::c_float;
    bestbot = -(1 as libc::c_int);
    // get the best bot
    i = 0 as libc::c_int;
    while i < 64 as libc::c_int {
        if !botstates[i as usize].is_null() && (*botstates[i as usize]).inuse != 0 {
            rank = ((*botstates[i as usize]).num_kills * 2 as libc::c_int
                - (*botstates[i as usize]).num_deaths) as libc::c_float
        } else {
            rank = -(1 as libc::c_int) as libc::c_float
        }
        if rank > bestrank {
            bestrank = rank;
            bestbot = i
        }
        i += 1
    }
    if bestbot >= 0 as libc::c_int {
        //write out the new goal fuzzy logic
        crate::src::game::g_syscalls::trap_BotSaveGoalFuzzyLogic(
            (*botstates[bestbot as usize]).gs,
            filename,
        );
    };
}
/*
==============
BotInterbreedEndMatch

add link back into ExitLevel?
==============
*/
#[no_mangle]

pub unsafe extern "C" fn BotInterbreedEndMatch() {
    if bot_interbreed == 0 {
        return;
    }
    bot_interbreedmatchcount += 1;
    if bot_interbreedmatchcount >= bot_interbreedcycle.integer {
        bot_interbreedmatchcount = 0 as libc::c_int;
        //
        crate::src::game::g_syscalls::trap_Cvar_Update(
            &mut bot_interbreedwrite as *mut _ as *mut crate::src::qcommon::q_shared::vmCvar_t,
        );
        if crate::stdlib::strlen(bot_interbreedwrite.string.as_mut_ptr()) != 0 {
            BotWriteInterbreeded(bot_interbreedwrite.string.as_mut_ptr());
            crate::src::game::g_syscalls::trap_Cvar_Set(
                b"bot_interbreedwrite\x00" as *const u8 as *const libc::c_char,
                b"\x00" as *const u8 as *const libc::c_char,
            );
        }
        BotInterbreedBots();
    };
}
/*
==============
BotInterbreeding
==============
*/
#[no_mangle]

pub unsafe extern "C" fn BotInterbreeding() {
    let mut i: libc::c_int = 0;
    crate::src::game::g_syscalls::trap_Cvar_Update(
        &mut bot_interbreedchar as *mut _ as *mut crate::src::qcommon::q_shared::vmCvar_t,
    );
    if crate::stdlib::strlen(bot_interbreedchar.string.as_mut_ptr()) == 0 {
        return;
    }
    //make sure we are in tournament mode
    if crate::src::game::ai_dmq3::gametype != crate::bg_public_h::GT_TOURNAMENT as libc::c_int {
        crate::src::game::g_syscalls::trap_Cvar_Set(
            b"g_gametype\x00" as *const u8 as *const libc::c_char,
            crate::src::qcommon::q_shared::va(
                b"%d\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                crate::bg_public_h::GT_TOURNAMENT as libc::c_int,
            ),
        );
        ExitLevel();
        return;
    }
    //shutdown all the bots
    i = 0 as libc::c_int;
    while i < 64 as libc::c_int {
        if !botstates[i as usize].is_null() && (*botstates[i as usize]).inuse != 0 {
            BotAIShutdownClient(
                (*botstates[i as usize]).client,
                crate::src::qcommon::q_shared::qfalse,
            );
        }
        i += 1
    }
    //make sure all item weight configs are reloaded and Not shared
    crate::src::game::g_syscalls::trap_BotLibVarSet(
        b"bot_reloadcharacters\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    //add a number of bots using the desired bot character
    i = 0 as libc::c_int;
    while i < bot_interbreedbots.integer {
        crate::src::game::g_syscalls::trap_SendConsoleCommand(
            crate::src::qcommon::q_shared::EXEC_INSERT as libc::c_int,
            crate::src::qcommon::q_shared::va(
                b"addbot %s 4 free %i %s%d\n\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                bot_interbreedchar.string.as_mut_ptr(),
                i * 50 as libc::c_int,
                bot_interbreedchar.string.as_mut_ptr(),
                i,
            ),
        );
        i += 1
    }
    //
    crate::src::game::g_syscalls::trap_Cvar_Set(
        b"bot_interbreedchar\x00" as *const u8 as *const libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char,
    );
    bot_interbreed = crate::src::qcommon::q_shared::qtrue as libc::c_int;
}
//returns info about the entity
/*
==============
BotEntityInfo
==============
*/
#[no_mangle]

pub unsafe extern "C" fn BotEntityInfo(
    mut entnum: libc::c_int,
    mut info: *mut crate::be_aas_h::aas_entityinfo_t,
) {
    crate::src::game::g_syscalls::trap_AAS_EntityInfo(entnum, info as *mut libc::c_void);
}
//returns the number of bots in the game
/*
==============
NumBots
==============
*/
#[no_mangle]

pub unsafe extern "C" fn NumBots() -> libc::c_int {
    return numbots;
}
/*
==============
BotTeamLeader
==============
*/
#[no_mangle]

pub unsafe extern "C" fn BotTeamLeader(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
) -> libc::c_int {
    let mut leader: libc::c_int = 0;
    leader = crate::src::game::ai_dmq3::ClientFromName((*bs).teamleader.as_mut_ptr());
    if leader < 0 as libc::c_int {
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    }
    if botstates[leader as usize].is_null() || (*botstates[leader as usize]).inuse == 0 {
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    }
    return crate::src::qcommon::q_shared::qtrue as libc::c_int;
}
/*
==============
AngleDifference
==============
*/
#[no_mangle]

pub unsafe extern "C" fn AngleDifference(
    mut ang1: libc::c_float,
    mut ang2: libc::c_float,
) -> libc::c_float {
    let mut diff: libc::c_float = 0.;
    diff = ang1 - ang2;
    if ang1 > ang2 {
        if diff as libc::c_double > 180.0f64 {
            diff = (diff as libc::c_double - 360.0f64) as libc::c_float
        }
    } else if (diff as libc::c_double) < -180.0f64 {
        diff = (diff as libc::c_double + 360.0f64) as libc::c_float
    }
    return diff;
}
/*
==============
BotChangeViewAngle
==============
*/
#[no_mangle]

pub unsafe extern "C" fn BotChangeViewAngle(
    mut angle: libc::c_float,
    mut ideal_angle: libc::c_float,
    mut speed: libc::c_float,
) -> libc::c_float {
    let mut move_0: libc::c_float = 0.;
    angle = crate::src::qcommon::q_math::AngleMod(angle);
    ideal_angle = crate::src::qcommon::q_math::AngleMod(ideal_angle);
    if angle == ideal_angle {
        return angle;
    }
    move_0 = ideal_angle - angle;
    if ideal_angle > angle {
        if move_0 as libc::c_double > 180.0f64 {
            move_0 = (move_0 as libc::c_double - 360.0f64) as libc::c_float
        }
    } else if (move_0 as libc::c_double) < -180.0f64 {
        move_0 = (move_0 as libc::c_double + 360.0f64) as libc::c_float
    }
    if move_0 > 0 as libc::c_int as libc::c_float {
        if move_0 > speed {
            move_0 = speed
        }
    } else if move_0 < -speed {
        move_0 = -speed
    }
    return crate::src::qcommon::q_math::AngleMod(angle + move_0);
}
/*
==============
BotChangeViewAngles
==============
*/
#[no_mangle]

pub unsafe extern "C" fn BotChangeViewAngles(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut thinktime: libc::c_float,
) {
    let mut diff: libc::c_float = 0.;
    let mut factor: libc::c_float = 0.;
    let mut maxchange: libc::c_float = 0.;
    let mut anglespeed: libc::c_float = 0.;
    let mut disired_speed: libc::c_float = 0.;
    let mut i: libc::c_int = 0;
    if (*bs).ideal_viewangles[0 as libc::c_int as usize] > 180 as libc::c_int as libc::c_float {
        (*bs).ideal_viewangles[0 as libc::c_int as usize] -= 360 as libc::c_int as libc::c_float
    }
    //
    if (*bs).enemy >= 0 as libc::c_int {
        factor = crate::src::game::g_syscalls::trap_Characteristic_BFloat(
            (*bs).character,
            4 as libc::c_int,
            0.01f32,
            1 as libc::c_int as libc::c_float,
        );
        maxchange = crate::src::game::g_syscalls::trap_Characteristic_BFloat(
            (*bs).character,
            5 as libc::c_int,
            1 as libc::c_int as libc::c_float,
            1800 as libc::c_int as libc::c_float,
        )
    } else {
        factor = 0.05f32;
        maxchange = 360 as libc::c_int as libc::c_float
    }
    if maxchange < 240 as libc::c_int as libc::c_float {
        maxchange = 240 as libc::c_int as libc::c_float
    }
    maxchange *= thinktime;
    i = 0 as libc::c_int;
    while i < 2 as libc::c_int {
        //
        if crate::src::game::ai_dmq3::bot_challenge.integer != 0 {
            //smooth slowdown view model
            diff = crate::stdlib::fabs(AngleDifference(
                (*bs).viewangles[i as usize],
                (*bs).ideal_viewangles[i as usize],
            ) as libc::c_double) as libc::c_float;
            anglespeed = diff * factor;
            if anglespeed > maxchange {
                anglespeed = maxchange
            }
            (*bs).viewangles[i as usize] = BotChangeViewAngle(
                (*bs).viewangles[i as usize],
                (*bs).ideal_viewangles[i as usize],
                anglespeed,
            )
        } else {
            //over reaction view model
            (*bs).viewangles[i as usize] =
                crate::src::qcommon::q_math::AngleMod((*bs).viewangles[i as usize]);
            (*bs).ideal_viewangles[i as usize] =
                crate::src::qcommon::q_math::AngleMod((*bs).ideal_viewangles[i as usize]);
            diff = AngleDifference(
                (*bs).viewangles[i as usize],
                (*bs).ideal_viewangles[i as usize],
            );
            disired_speed = diff * factor;
            (*bs).viewanglespeed[i as usize] += (*bs).viewanglespeed[i as usize] - disired_speed;
            if (*bs).viewanglespeed[i as usize] > 180 as libc::c_int as libc::c_float {
                (*bs).viewanglespeed[i as usize] = maxchange
            }
            if (*bs).viewanglespeed[i as usize] < -(180 as libc::c_int) as libc::c_float {
                (*bs).viewanglespeed[i as usize] = -maxchange
            }
            anglespeed = (*bs).viewanglespeed[i as usize];
            if anglespeed > maxchange {
                anglespeed = maxchange
            }
            if anglespeed < -maxchange {
                anglespeed = -maxchange
            }
            (*bs).viewangles[i as usize] += anglespeed;
            (*bs).viewangles[i as usize] =
                crate::src::qcommon::q_math::AngleMod((*bs).viewangles[i as usize]);
            //demping
            (*bs).viewanglespeed[i as usize] = ((*bs).viewanglespeed[i as usize] as libc::c_double
                * (0.45f64 * (1 as libc::c_int as libc::c_float - factor) as libc::c_double))
                as crate::src::qcommon::q_shared::vec_t
        }
        i += 1
        //BotAI_Print(PRT_MESSAGE, "ideal_angles %f %f\n", bs->ideal_viewangles[0], bs->ideal_viewangles[1], bs->ideal_viewangles[2]);`
        //bs->viewangles[i] = bs->ideal_viewangles[i];
    }
    //bs->viewangles[PITCH] = 0;
    if (*bs).viewangles[0 as libc::c_int as usize] > 180 as libc::c_int as libc::c_float {
        (*bs).viewangles[0 as libc::c_int as usize] -= 360 as libc::c_int as libc::c_float
    }
    //elementary action: view
    crate::src::game::g_syscalls::trap_EA_View((*bs).client, (*bs).viewangles.as_mut_ptr());
}
/*
==============
BotInputToUserCommand
==============
*/
#[no_mangle]

pub unsafe extern "C" fn BotInputToUserCommand(
    mut bi: *mut crate::botlib_h::bot_input_t,
    mut ucmd: *mut crate::src::qcommon::q_shared::usercmd_t,
    mut delta_angles: *mut libc::c_int,
    mut time: libc::c_int,
) {
    let mut angles: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut forward: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut right: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut temp: libc::c_short = 0;
    let mut j: libc::c_int = 0;
    let mut f: libc::c_float = 0.;
    let mut r: libc::c_float = 0.;
    let mut u: libc::c_float = 0.;
    let mut m: libc::c_float = 0.;
    //clear the whole structure
    crate::stdlib::memset(
        ucmd as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<crate::src::qcommon::q_shared::usercmd_t>() as libc::c_ulong,
    );
    //the duration for the user command in milli seconds
    (*ucmd).serverTime = time;
    //
    if (*bi).actionflags & 0x8000 as libc::c_int != 0 {
        (*bi).actionflags |= 0x10 as libc::c_int;
        (*bi).actionflags &= !(0x8000 as libc::c_int)
    }
    //set the buttons
    if (*bi).actionflags & 0x8 as libc::c_int != 0 {
        (*ucmd).buttons = 1 as libc::c_int
    }
    if (*bi).actionflags & 0x1 as libc::c_int != 0 {
        (*ucmd).buttons |= 1 as libc::c_int
    }
    if (*bi).actionflags & 0x10000 as libc::c_int != 0 {
        (*ucmd).buttons |= 2 as libc::c_int
    }
    if (*bi).actionflags & 0x20000 as libc::c_int != 0 {
        (*ucmd).buttons |= 8 as libc::c_int
    }
    if (*bi).actionflags & 0x2 as libc::c_int != 0 {
        (*ucmd).buttons |= 4 as libc::c_int
    }
    if (*bi).actionflags & 0x80000 as libc::c_int != 0 {
        (*ucmd).buttons |= 16 as libc::c_int
    }
    if (*bi).actionflags & 0x100000 as libc::c_int != 0 {
        (*ucmd).buttons |= 32 as libc::c_int
    }
    if (*bi).actionflags & 0x200000 as libc::c_int != 0 {
        (*ucmd).buttons |= 64 as libc::c_int
    }
    if (*bi).actionflags & 0x800000 as libc::c_int != 0 {
        (*ucmd).buttons |= 128 as libc::c_int
    }
    if (*bi).actionflags & 0x1000000 as libc::c_int != 0 {
        (*ucmd).buttons |= 256 as libc::c_int
    }
    if (*bi).actionflags & 0x2000000 as libc::c_int != 0 {
        (*ucmd).buttons |= 512 as libc::c_int
    }
    if (*bi).actionflags & 0x8000000 as libc::c_int != 0 {
        (*ucmd).buttons |= 1024 as libc::c_int
    }
    //
    (*ucmd).weapon = (*bi).weapon as crate::src::qcommon::q_shared::byte;
    //set the view angles
    //NOTE: the ucmd->angles are the angles WITHOUT the delta angles
    (*ucmd).angles[0 as libc::c_int as usize] =
        ((*bi).viewangles[0 as libc::c_int as usize] * 65536 as libc::c_int as libc::c_float
            / 360 as libc::c_int as libc::c_float) as libc::c_int
            & 65535 as libc::c_int;
    (*ucmd).angles[1 as libc::c_int as usize] =
        ((*bi).viewangles[1 as libc::c_int as usize] * 65536 as libc::c_int as libc::c_float
            / 360 as libc::c_int as libc::c_float) as libc::c_int
            & 65535 as libc::c_int;
    (*ucmd).angles[2 as libc::c_int as usize] =
        ((*bi).viewangles[2 as libc::c_int as usize] * 65536 as libc::c_int as libc::c_float
            / 360 as libc::c_int as libc::c_float) as libc::c_int
            & 65535 as libc::c_int;
    //subtract the delta angles
    j = 0 as libc::c_int;
    while j < 3 as libc::c_int {
        temp = ((*ucmd).angles[j as usize] - *delta_angles.offset(j as isize)) as libc::c_short;
        /*NOTE: disabled because temp should be mod first
        if ( j == PITCH ) {
            // don't let the player look up or down more than 90 degrees
            if ( temp > 16000 ) temp = 16000;
            else if ( temp < -16000 ) temp = -16000;
        }
        */
        (*ucmd).angles[j as usize] = temp as libc::c_int;
        j += 1
    }
    //NOTE: movement is relative to the REAL view angles
    //get the horizontal forward and right vector
    //get the pitch in the range [-180, 180]
    if (*bi).dir[2 as libc::c_int as usize] != 0. {
        angles[0 as libc::c_int as usize] = (*bi).viewangles[0 as libc::c_int as usize]
    } else {
        angles[0 as libc::c_int as usize] = 0 as libc::c_int as crate::src::qcommon::q_shared::vec_t
    }
    angles[1 as libc::c_int as usize] = (*bi).viewangles[1 as libc::c_int as usize];
    angles[2 as libc::c_int as usize] = 0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
    crate::src::qcommon::q_math::AngleVectors(
        angles.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        forward.as_mut_ptr(),
        right.as_mut_ptr(),
        0 as *mut crate::src::qcommon::q_shared::vec_t,
    );
    //bot input speed is in the range [0, 400]
    (*bi).speed =
        (*bi).speed * 127 as libc::c_int as libc::c_float / 400 as libc::c_int as libc::c_float;
    //set the view independent movement
    f = forward[0 as libc::c_int as usize] * (*bi).dir[0 as libc::c_int as usize]
        + forward[1 as libc::c_int as usize] * (*bi).dir[1 as libc::c_int as usize]
        + forward[2 as libc::c_int as usize] * (*bi).dir[2 as libc::c_int as usize];
    r = right[0 as libc::c_int as usize] * (*bi).dir[0 as libc::c_int as usize]
        + right[1 as libc::c_int as usize] * (*bi).dir[1 as libc::c_int as usize]
        + right[2 as libc::c_int as usize] * (*bi).dir[2 as libc::c_int as usize];
    u = (crate::stdlib::fabs(forward[2 as libc::c_int as usize] as libc::c_double)
        * (*bi).dir[2 as libc::c_int as usize] as libc::c_double) as libc::c_float;
    m = crate::stdlib::fabs(f as libc::c_double) as libc::c_float;
    if crate::stdlib::fabs(r as libc::c_double) > m as libc::c_double {
        m = crate::stdlib::fabs(r as libc::c_double) as libc::c_float
    }
    if crate::stdlib::fabs(u as libc::c_double) > m as libc::c_double {
        m = crate::stdlib::fabs(u as libc::c_double) as libc::c_float
    }
    if m > 0 as libc::c_int as libc::c_float {
        f *= (*bi).speed / m;
        r *= (*bi).speed / m;
        u *= (*bi).speed / m
    }
    (*ucmd).forwardmove = f as libc::c_schar;
    (*ucmd).rightmove = r as libc::c_schar;
    (*ucmd).upmove = u as libc::c_schar;
    if (*bi).actionflags & 0x200 as libc::c_int != 0 {
        (*ucmd).forwardmove = 127 as libc::c_int as libc::c_schar
    }
    if (*bi).actionflags & 0x800 as libc::c_int != 0 {
        (*ucmd).forwardmove = -(127 as libc::c_int) as libc::c_schar
    }
    if (*bi).actionflags & 0x1000 as libc::c_int != 0 {
        (*ucmd).rightmove = -(127 as libc::c_int) as libc::c_schar
    }
    if (*bi).actionflags & 0x2000 as libc::c_int != 0 {
        (*ucmd).rightmove = 127 as libc::c_int as libc::c_schar
    }
    //jump/moveup
    if (*bi).actionflags & 0x10 as libc::c_int != 0 {
        (*ucmd).upmove = 127 as libc::c_int as libc::c_schar
    }
    //crouch/movedown
    if (*bi).actionflags & 0x80 as libc::c_int != 0 {
        (*ucmd).upmove = -(127 as libc::c_int) as libc::c_schar
    };
}
/*
==============
BotUpdateInput
==============
*/
#[no_mangle]

pub unsafe extern "C" fn BotUpdateInput(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut time: libc::c_int,
    mut elapsed_time: libc::c_int,
) {
    let mut bi: crate::botlib_h::bot_input_t = crate::botlib_h::bot_input_t {
        thinktime: 0.,
        dir: [0.; 3],
        speed: 0.,
        viewangles: [0.; 3],
        actionflags: 0,
        weapon: 0,
    };
    let mut j: libc::c_int = 0;
    //add the delta angles to the bot's current view angles
    j = 0 as libc::c_int;
    while j < 3 as libc::c_int {
        (*bs).viewangles[j as usize] = crate::src::qcommon::q_math::AngleMod(
            ((*bs).viewangles[j as usize] as libc::c_double
                + (*bs).cur_ps.delta_angles[j as usize] as libc::c_double
                    * (360.0f64 / 65536 as libc::c_int as libc::c_double))
                as libc::c_float,
        );
        j += 1
    }
    //change the bot view angles
    BotChangeViewAngles(
        bs,
        elapsed_time as libc::c_float / 1000 as libc::c_int as libc::c_float,
    );
    //retrieve the bot input
    crate::src::game::g_syscalls::trap_EA_GetInput(
        (*bs).client,
        time as libc::c_float / 1000 as libc::c_int as libc::c_float,
        &mut bi as *mut crate::botlib_h::bot_input_t as *mut libc::c_void,
    );
    //respawn hack
    if bi.actionflags & 0x8 as libc::c_int != 0 {
        if (*bs).lastucmd.buttons & 1 as libc::c_int != 0 {
            bi.actionflags &= !(0x8 as libc::c_int | 0x1 as libc::c_int)
        }
    }
    //convert the bot input to a usercmd
    BotInputToUserCommand(
        &mut bi,
        &mut (*bs).lastucmd,
        (*bs).cur_ps.delta_angles.as_mut_ptr(),
        time,
    );
    //subtract the delta angles
    j = 0 as libc::c_int;
    while j < 3 as libc::c_int {
        (*bs).viewangles[j as usize] = crate::src::qcommon::q_math::AngleMod(
            ((*bs).viewangles[j as usize] as libc::c_double
                - (*bs).cur_ps.delta_angles[j as usize] as libc::c_double
                    * (360.0f64 / 65536 as libc::c_int as libc::c_double))
                as libc::c_float,
        );
        j += 1
    }
}
/*
==============
BotAIRegularUpdate
==============
*/
#[no_mangle]

pub unsafe extern "C" fn BotAIRegularUpdate() {
    if regularupdate_time < floattime {
        crate::src::game::g_syscalls::trap_BotUpdateEntityItems();
        regularupdate_time = (floattime as libc::c_double + 0.3f64) as libc::c_float
    };
}
/*
==============
RemoveColorEscapeSequences
==============
*/
#[no_mangle]

pub unsafe extern "C" fn RemoveColorEscapeSequences(mut text: *mut libc::c_char) {
    let mut i: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    l = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while *text.offset(i as isize) != 0 {
        if crate::src::qcommon::q_shared::Q_IsColorString(&mut *text.offset(i as isize)) as u64 != 0
        {
            i += 1
        } else if !(*text.offset(i as isize) as libc::c_int > 0x7e as libc::c_int) {
            let fresh0 = l;
            l = l + 1;
            *text.offset(fresh0 as isize) = *text.offset(i as isize)
        }
        i += 1
    }
    *text.offset(l as isize) = '\u{0}' as i32 as libc::c_char;
}
/*
==============
BotAI
==============
*/
#[no_mangle]

pub unsafe extern "C" fn BotAI(
    mut client: libc::c_int,
    mut thinktime: libc::c_float,
) -> libc::c_int {
    let mut bs: *mut crate::src::game::ai_main::bot_state_t =
        0 as *mut crate::src::game::ai_main::bot_state_t;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut args: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut j: libc::c_int = 0;
    crate::src::game::g_syscalls::trap_EA_ResetInput(client);
    //
    bs = botstates[client as usize];
    if bs.is_null() || (*bs).inuse == 0 {
        BotAI_Print(
            4 as libc::c_int,
            b"BotAI: client %d is not setup\n\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            client,
        );
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    }
    //retrieve the current client state
    if BotAI_GetClientState(client, &mut (*bs).cur_ps) == 0 {
        BotAI_Print(
            4 as libc::c_int,
            b"BotAI: failed to get player state for player %d\n\x00" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            client,
        );
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    }
    //retrieve any waiting server commands
    while crate::src::game::g_syscalls::trap_BotGetServerCommand(
        client,
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
    ) != 0
    {
        //have buf point to the command and args to the command arguments
        args = ::libc::strchr(buf.as_mut_ptr(), ' ' as i32);
        if args.is_null() {
            continue;
        }
        let fresh1 = args;
        args = args.offset(1);
        *fresh1 = '\u{0}' as i32 as libc::c_char;
        //remove color espace sequences from the arguments
        RemoveColorEscapeSequences(args);
        if !(crate::src::qcommon::q_shared::Q_stricmp(
            buf.as_mut_ptr(),
            b"cp \x00" as *const u8 as *const libc::c_char,
        ) == 0)
        {
            if !(crate::src::qcommon::q_shared::Q_stricmp(
                buf.as_mut_ptr(),
                b"cs\x00" as *const u8 as *const libc::c_char,
            ) == 0)
            {
                if crate::src::qcommon::q_shared::Q_stricmp(
                    buf.as_mut_ptr(),
                    b"print\x00" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    //remove first and last quote from the chat message
                    crate::stdlib::memmove(
                        args as *mut libc::c_void,
                        args.offset(1 as libc::c_int as isize) as *const libc::c_void,
                        crate::stdlib::strlen(args),
                    );
                    *args.offset(
                        crate::stdlib::strlen(args).wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            as isize,
                    ) = '\u{0}' as i32 as libc::c_char;
                    crate::src::game::g_syscalls::trap_BotQueueConsoleMessage(
                        (*bs).cs,
                        0 as libc::c_int,
                        args,
                    );
                } else if crate::src::qcommon::q_shared::Q_stricmp(
                    buf.as_mut_ptr(),
                    b"chat\x00" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    //remove first and last quote from the chat message
                    crate::stdlib::memmove(
                        args as *mut libc::c_void,
                        args.offset(1 as libc::c_int as isize) as *const libc::c_void,
                        crate::stdlib::strlen(args),
                    );
                    *args.offset(
                        crate::stdlib::strlen(args).wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            as isize,
                    ) = '\u{0}' as i32 as libc::c_char;
                    crate::src::game::g_syscalls::trap_BotQueueConsoleMessage(
                        (*bs).cs,
                        1 as libc::c_int,
                        args,
                    );
                } else if crate::src::qcommon::q_shared::Q_stricmp(
                    buf.as_mut_ptr(),
                    b"tchat\x00" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    //remove first and last quote from the chat message
                    crate::stdlib::memmove(
                        args as *mut libc::c_void,
                        args.offset(1 as libc::c_int as isize) as *const libc::c_void,
                        crate::stdlib::strlen(args),
                    );
                    *args.offset(
                        crate::stdlib::strlen(args).wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            as isize,
                    ) = '\u{0}' as i32 as libc::c_char;
                    crate::src::game::g_syscalls::trap_BotQueueConsoleMessage(
                        (*bs).cs,
                        1 as libc::c_int,
                        args,
                    );
                } else if !(crate::src::qcommon::q_shared::Q_stricmp(
                    buf.as_mut_ptr(),
                    b"scores\x00" as *const u8 as *const libc::c_char,
                ) == 0)
                {
                    crate::src::qcommon::q_shared::Q_stricmp(
                        buf.as_mut_ptr(),
                        b"clientLevelShot\x00" as *const u8 as *const libc::c_char,
                    );
                }
            }
        }
    }
    //add the delta angles to the bot's current view angles
    j = 0 as libc::c_int;
    while j < 3 as libc::c_int {
        (*bs).viewangles[j as usize] = crate::src::qcommon::q_math::AngleMod(
            ((*bs).viewangles[j as usize] as libc::c_double
                + (*bs).cur_ps.delta_angles[j as usize] as libc::c_double
                    * (360.0f64 / 65536 as libc::c_int as libc::c_double))
                as libc::c_float,
        );
        j += 1
    }
    //increase the local time of the bot
    (*bs).ltime += thinktime;
    //
    (*bs).thinktime = thinktime;
    //origin of the bot
    (*bs).origin[0 as libc::c_int as usize] = (*bs).cur_ps.origin[0 as libc::c_int as usize];
    (*bs).origin[1 as libc::c_int as usize] = (*bs).cur_ps.origin[1 as libc::c_int as usize];
    (*bs).origin[2 as libc::c_int as usize] = (*bs).cur_ps.origin[2 as libc::c_int as usize];
    //eye coordinates of the bot
    (*bs).eye[0 as libc::c_int as usize] = (*bs).cur_ps.origin[0 as libc::c_int as usize];
    (*bs).eye[1 as libc::c_int as usize] = (*bs).cur_ps.origin[1 as libc::c_int as usize];
    (*bs).eye[2 as libc::c_int as usize] = (*bs).cur_ps.origin[2 as libc::c_int as usize];
    (*bs).eye[2 as libc::c_int as usize] += (*bs).cur_ps.viewheight as libc::c_float;
    //get the area the bot is in
    (*bs).areanum = crate::src::game::ai_dmq3::BotPointAreaNum((*bs).origin.as_mut_ptr());
    //the real AI
    crate::src::game::ai_dmq3::BotDeathmatchAI(
        bs as *mut crate::src::game::ai_main::bot_state_s,
        thinktime,
    );
    //set the weapon selection every AI frame
    crate::src::game::g_syscalls::trap_EA_SelectWeapon((*bs).client, (*bs).weaponnum);
    //subtract the delta angles
    j = 0 as libc::c_int;
    while j < 3 as libc::c_int {
        (*bs).viewangles[j as usize] = crate::src::qcommon::q_math::AngleMod(
            ((*bs).viewangles[j as usize] as libc::c_double
                - (*bs).cur_ps.delta_angles[j as usize] as libc::c_double
                    * (360.0f64 / 65536 as libc::c_int as libc::c_double))
                as libc::c_float,
        );
        j += 1
    }
    //everything was ok
    return crate::src::qcommon::q_shared::qtrue as libc::c_int;
}
/*
==================
BotScheduleBotThink
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotScheduleBotThink() {
    let mut i: libc::c_int = 0;
    let mut botnum: libc::c_int = 0;
    botnum = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 64 as libc::c_int {
        if !(botstates[i as usize].is_null() || (*botstates[i as usize]).inuse == 0) {
            //initialize the bot think residual time
            (*botstates[i as usize]).botthink_residual = bot_thinktime.integer * botnum / numbots;
            botnum += 1
        }
        i += 1
    }
}
/*
==============
BotWriteSessionData
==============
*/
#[no_mangle]

pub unsafe extern "C" fn BotWriteSessionData(mut bs: *mut crate::src::game::ai_main::bot_state_t) {
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut var: *const libc::c_char = 0 as *const libc::c_char;
    s = crate::src::qcommon::q_shared::va(
        b"%i %i %i %i %i %i %i %i %f %f %f %f %f %f %f %f %f %f\x00" as *const u8
            as *const libc::c_char as *mut libc::c_char,
        (*bs).lastgoal_decisionmaker,
        (*bs).lastgoal_ltgtype,
        (*bs).lastgoal_teammate,
        (*bs).lastgoal_teamgoal.areanum,
        (*bs).lastgoal_teamgoal.entitynum,
        (*bs).lastgoal_teamgoal.flags,
        (*bs).lastgoal_teamgoal.iteminfo,
        (*bs).lastgoal_teamgoal.number,
        (*bs).lastgoal_teamgoal.origin[0 as libc::c_int as usize] as libc::c_double,
        (*bs).lastgoal_teamgoal.origin[1 as libc::c_int as usize] as libc::c_double,
        (*bs).lastgoal_teamgoal.origin[2 as libc::c_int as usize] as libc::c_double,
        (*bs).lastgoal_teamgoal.mins[0 as libc::c_int as usize] as libc::c_double,
        (*bs).lastgoal_teamgoal.mins[1 as libc::c_int as usize] as libc::c_double,
        (*bs).lastgoal_teamgoal.mins[2 as libc::c_int as usize] as libc::c_double,
        (*bs).lastgoal_teamgoal.maxs[0 as libc::c_int as usize] as libc::c_double,
        (*bs).lastgoal_teamgoal.maxs[1 as libc::c_int as usize] as libc::c_double,
        (*bs).lastgoal_teamgoal.maxs[2 as libc::c_int as usize] as libc::c_double,
        (*bs).formation_dist as libc::c_double,
    );
    var = crate::src::qcommon::q_shared::va(
        b"botsession%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (*bs).client,
    );
    crate::src::game::g_syscalls::trap_Cvar_Set(var, s);
}
/*
==============
BotReadSessionData
==============
*/
#[no_mangle]

pub unsafe extern "C" fn BotReadSessionData(mut bs: *mut crate::src::game::ai_main::bot_state_t) {
    let mut s: [libc::c_char; 1024] = [0; 1024];
    let mut var: *const libc::c_char = 0 as *const libc::c_char;
    var = crate::src::qcommon::q_shared::va(
        b"botsession%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (*bs).client,
    );
    crate::src::game::g_syscalls::trap_Cvar_VariableStringBuffer(
        var,
        s.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
    );
    ::libc::sscanf(
        s.as_mut_ptr(),
        b"%i %i %i %i %i %i %i %i %f %f %f %f %f %f %f %f %f %f\x00" as *const u8
            as *const libc::c_char,
        &mut (*bs).lastgoal_decisionmaker as *mut libc::c_int,
        &mut (*bs).lastgoal_ltgtype as *mut libc::c_int,
        &mut (*bs).lastgoal_teammate as *mut libc::c_int,
        &mut (*bs).lastgoal_teamgoal.areanum as *mut libc::c_int,
        &mut (*bs).lastgoal_teamgoal.entitynum as *mut libc::c_int,
        &mut (*bs).lastgoal_teamgoal.flags as *mut libc::c_int,
        &mut (*bs).lastgoal_teamgoal.iteminfo as *mut libc::c_int,
        &mut (*bs).lastgoal_teamgoal.number as *mut libc::c_int,
        &mut *(*bs)
            .lastgoal_teamgoal
            .origin
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize) as *mut crate::src::qcommon::q_shared::vec_t,
        &mut *(*bs)
            .lastgoal_teamgoal
            .origin
            .as_mut_ptr()
            .offset(1 as libc::c_int as isize) as *mut crate::src::qcommon::q_shared::vec_t,
        &mut *(*bs)
            .lastgoal_teamgoal
            .origin
            .as_mut_ptr()
            .offset(2 as libc::c_int as isize) as *mut crate::src::qcommon::q_shared::vec_t,
        &mut *(*bs)
            .lastgoal_teamgoal
            .mins
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize) as *mut crate::src::qcommon::q_shared::vec_t,
        &mut *(*bs)
            .lastgoal_teamgoal
            .mins
            .as_mut_ptr()
            .offset(1 as libc::c_int as isize) as *mut crate::src::qcommon::q_shared::vec_t,
        &mut *(*bs)
            .lastgoal_teamgoal
            .mins
            .as_mut_ptr()
            .offset(2 as libc::c_int as isize) as *mut crate::src::qcommon::q_shared::vec_t,
        &mut *(*bs)
            .lastgoal_teamgoal
            .maxs
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize) as *mut crate::src::qcommon::q_shared::vec_t,
        &mut *(*bs)
            .lastgoal_teamgoal
            .maxs
            .as_mut_ptr()
            .offset(1 as libc::c_int as isize) as *mut crate::src::qcommon::q_shared::vec_t,
        &mut *(*bs)
            .lastgoal_teamgoal
            .maxs
            .as_mut_ptr()
            .offset(2 as libc::c_int as isize) as *mut crate::src::qcommon::q_shared::vec_t,
        &mut (*bs).formation_dist as *mut libc::c_float,
    );
}
/*
==============
BotAISetupClient
==============
*/
#[no_mangle]

pub unsafe extern "C" fn BotAISetupClient(
    mut client: libc::c_int,
    mut settings: *mut crate::g_local_h::bot_settings_s,
    mut restart: crate::src::qcommon::q_shared::qboolean,
) -> libc::c_int {
    let mut filename: [libc::c_char; 144] = [0; 144];
    let mut name: [libc::c_char; 144] = [0; 144];
    let mut gender: [libc::c_char; 144] = [0; 144];
    let mut bs: *mut crate::src::game::ai_main::bot_state_t =
        0 as *mut crate::src::game::ai_main::bot_state_t;
    let mut errnum: libc::c_int = 0;
    if botstates[client as usize].is_null() {
        botstates[client as usize] = crate::src::game::g_mem::G_Alloc(::std::mem::size_of::<
            crate::src::game::ai_main::bot_state_t,
        >() as libc::c_ulong
            as libc::c_int)
            as *mut crate::src::game::ai_main::bot_state_t
    }
    bs = botstates[client as usize];
    if bs.is_null() {
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    }
    if !bs.is_null() && (*bs).inuse != 0 {
        BotAI_Print(
            4 as libc::c_int,
            b"BotAISetupClient: client %d already setup\n\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            client,
        );
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    }
    if crate::src::game::g_syscalls::trap_AAS_Initialized() == 0 {
        BotAI_Print(
            4 as libc::c_int,
            b"AAS not initialized\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    }
    //load the bot character
    (*bs).character = crate::src::game::g_syscalls::trap_BotLoadCharacter(
        (*settings).characterfile.as_mut_ptr(),
        (*settings).skill,
    );
    if (*bs).character == 0 {
        BotAI_Print(
            4 as libc::c_int,
            b"couldn\'t load skill %f from %s\n\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            (*settings).skill as libc::c_double,
            (*settings).characterfile.as_mut_ptr(),
        );
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    }
    //copy the settings
    crate::stdlib::memcpy(
        &mut (*bs).settings as *mut crate::g_local_h::bot_settings_t as *mut libc::c_void,
        settings as *const libc::c_void,
        ::std::mem::size_of::<crate::g_local_h::bot_settings_t>() as libc::c_ulong,
    );
    //allocate a goal state
    (*bs).gs = crate::src::game::g_syscalls::trap_BotAllocGoalState(client);
    //load the item weights
    crate::src::game::g_syscalls::trap_Characteristic_String(
        (*bs).character,
        40 as libc::c_int,
        filename.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 144]>() as libc::c_ulong as libc::c_int,
    );
    errnum = crate::src::game::g_syscalls::trap_BotLoadItemWeights((*bs).gs, filename.as_mut_ptr());
    if errnum != 0 as libc::c_int {
        crate::src::game::g_syscalls::trap_BotFreeGoalState((*bs).gs);
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    }
    //allocate a weapon state
    (*bs).ws = crate::src::game::g_syscalls::trap_BotAllocWeaponState();
    //load the weapon weights
    crate::src::game::g_syscalls::trap_Characteristic_String(
        (*bs).character,
        3 as libc::c_int,
        filename.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 144]>() as libc::c_ulong as libc::c_int,
    );
    errnum =
        crate::src::game::g_syscalls::trap_BotLoadWeaponWeights((*bs).ws, filename.as_mut_ptr());
    if errnum != 0 as libc::c_int {
        crate::src::game::g_syscalls::trap_BotFreeGoalState((*bs).gs);
        crate::src::game::g_syscalls::trap_BotFreeWeaponState((*bs).ws);
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    }
    //allocate a chat state
    (*bs).cs = crate::src::game::g_syscalls::trap_BotAllocChatState();
    //load the chat file
    crate::src::game::g_syscalls::trap_Characteristic_String(
        (*bs).character,
        21 as libc::c_int,
        filename.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 144]>() as libc::c_ulong as libc::c_int,
    );
    crate::src::game::g_syscalls::trap_Characteristic_String(
        (*bs).character,
        22 as libc::c_int,
        name.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 144]>() as libc::c_ulong as libc::c_int,
    );
    errnum = crate::src::game::g_syscalls::trap_BotLoadChatFile(
        (*bs).cs,
        filename.as_mut_ptr(),
        name.as_mut_ptr(),
    );
    if errnum != 0 as libc::c_int {
        crate::src::game::g_syscalls::trap_BotFreeChatState((*bs).cs);
        crate::src::game::g_syscalls::trap_BotFreeGoalState((*bs).gs);
        crate::src::game::g_syscalls::trap_BotFreeWeaponState((*bs).ws);
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    }
    //get the gender characteristic
    crate::src::game::g_syscalls::trap_Characteristic_String(
        (*bs).character,
        1 as libc::c_int,
        gender.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 144]>() as libc::c_ulong as libc::c_int,
    );
    //set the chat gender
    if *gender.as_mut_ptr() as libc::c_int == 'f' as i32
        || *gender.as_mut_ptr() as libc::c_int == 'F' as i32
    {
        crate::src::game::g_syscalls::trap_BotSetChatGender((*bs).cs, 1 as libc::c_int);
    } else if *gender.as_mut_ptr() as libc::c_int == 'm' as i32
        || *gender.as_mut_ptr() as libc::c_int == 'M' as i32
    {
        crate::src::game::g_syscalls::trap_BotSetChatGender((*bs).cs, 2 as libc::c_int);
    } else {
        crate::src::game::g_syscalls::trap_BotSetChatGender((*bs).cs, 0 as libc::c_int);
    }
    (*bs).inuse = crate::src::qcommon::q_shared::qtrue as libc::c_int;
    (*bs).client = client;
    (*bs).entitynum = client;
    (*bs).setupcount = 4 as libc::c_int;
    (*bs).entergame_time = floattime;
    (*bs).ms = crate::src::game::g_syscalls::trap_BotAllocMoveState();
    (*bs).walker = crate::src::game::g_syscalls::trap_Characteristic_BFloat(
        (*bs).character,
        48 as libc::c_int,
        0 as libc::c_int as libc::c_float,
        1 as libc::c_int as libc::c_float,
    );
    numbots += 1;
    if crate::src::game::g_syscalls::trap_Cvar_VariableIntegerValue(
        b"bot_testichat\x00" as *const u8 as *const libc::c_char,
    ) != 0
    {
        crate::src::game::g_syscalls::trap_BotLibVarSet(
            b"bot_testichat\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        crate::src::game::ai_chat::BotChatTest(bs as *mut crate::src::game::ai_main::bot_state_s);
    }
    //NOTE: reschedule the bot thinking
    BotScheduleBotThink();
    //if interbreeding start with a mutation
    if bot_interbreed != 0 {
        crate::src::game::g_syscalls::trap_BotMutateGoalFuzzyLogic(
            (*bs).gs,
            1 as libc::c_int as libc::c_float,
        );
    }
    // if we kept the bot client
    if restart as u64 != 0 {
        BotReadSessionData(bs);
    }
    //bot has been setup successfully
    return crate::src::qcommon::q_shared::qtrue as libc::c_int;
}
/*
==============
BotAIShutdownClient
==============
*/
#[no_mangle]

pub unsafe extern "C" fn BotAIShutdownClient(
    mut client: libc::c_int,
    mut restart: crate::src::qcommon::q_shared::qboolean,
) -> libc::c_int {
    let mut bs: *mut crate::src::game::ai_main::bot_state_t =
        0 as *mut crate::src::game::ai_main::bot_state_t;
    bs = botstates[client as usize];
    if bs.is_null() || (*bs).inuse == 0 {
        //BotAI_Print(PRT_ERROR, "BotAIShutdownClient: client %d already shutdown\n", client);
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    }
    if restart as u64 != 0 {
        BotWriteSessionData(bs);
    }
    if crate::src::game::ai_chat::BotChat_ExitGame(
        bs as *mut crate::src::game::ai_main::bot_state_s,
    ) != 0
    {
        crate::src::game::g_syscalls::trap_BotEnterChat((*bs).cs, (*bs).client, 0 as libc::c_int);
    }
    crate::src::game::g_syscalls::trap_BotFreeMoveState((*bs).ms);
    //free the goal state
    crate::src::game::g_syscalls::trap_BotFreeGoalState((*bs).gs);
    //free the chat file
    crate::src::game::g_syscalls::trap_BotFreeChatState((*bs).cs);
    //free the weapon weights
    crate::src::game::g_syscalls::trap_BotFreeWeaponState((*bs).ws);
    //free the bot character
    crate::src::game::g_syscalls::trap_BotFreeCharacter((*bs).character);
    //
    crate::src::game::ai_dmq3::BotFreeWaypoints(
        (*bs).checkpoints as *mut crate::src::game::ai_main::bot_waypoint_s,
    );
    crate::src::game::ai_dmq3::BotFreeWaypoints(
        (*bs).patrolpoints as *mut crate::src::game::ai_main::bot_waypoint_s,
    );
    //clear activate goal stack
    crate::src::game::ai_dmq3::BotClearActivateGoalStack(
        bs as *mut crate::src::game::ai_main::bot_state_s,
    );
    //clear the bot state
    crate::stdlib::memset(
        bs as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<crate::src::game::ai_main::bot_state_t>() as libc::c_ulong,
    );
    //set the inuse flag to qfalse
    (*bs).inuse = crate::src::qcommon::q_shared::qfalse as libc::c_int;
    //there's one bot less
    numbots -= 1;
    //everything went ok
    return crate::src::qcommon::q_shared::qtrue as libc::c_int;
}
//resets the whole bot state
/*
==============
BotResetState

called when a bot enters the intermission or observer mode and
when the level is changed
==============
*/
#[no_mangle]

pub unsafe extern "C" fn BotResetState(mut bs: *mut crate::src::game::ai_main::bot_state_t) {
    let mut client: libc::c_int = 0; //current player state
    let mut entitynum: libc::c_int = 0;
    let mut inuse: libc::c_int = 0;
    let mut movestate: libc::c_int = 0;
    let mut goalstate: libc::c_int = 0;
    let mut chatstate: libc::c_int = 0;
    let mut weaponstate: libc::c_int = 0;
    let mut settings: crate::g_local_h::bot_settings_t = crate::g_local_h::bot_settings_t {
        characterfile: [0; 144],
        skill: 0.,
    };
    let mut character: libc::c_int = 0;
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
    let mut entergame_time: libc::c_float = 0.;
    //save some things that should not be reset here
    crate::stdlib::memcpy(
        &mut settings as *mut crate::g_local_h::bot_settings_t as *mut libc::c_void,
        &mut (*bs).settings as *mut crate::g_local_h::bot_settings_t as *const libc::c_void,
        ::std::mem::size_of::<crate::g_local_h::bot_settings_t>() as libc::c_ulong,
    );
    crate::stdlib::memcpy(
        &mut ps as *mut crate::src::qcommon::q_shared::playerState_t as *mut libc::c_void,
        &mut (*bs).cur_ps as *mut crate::src::qcommon::q_shared::playerState_t
            as *const libc::c_void,
        ::std::mem::size_of::<crate::src::qcommon::q_shared::playerState_t>() as libc::c_ulong,
    );
    inuse = (*bs).inuse;
    client = (*bs).client;
    entitynum = (*bs).entitynum;
    character = (*bs).character;
    movestate = (*bs).ms;
    goalstate = (*bs).gs;
    chatstate = (*bs).cs;
    weaponstate = (*bs).ws;
    entergame_time = (*bs).entergame_time;
    //free checkpoints and patrol points
    crate::src::game::ai_dmq3::BotFreeWaypoints(
        (*bs).checkpoints as *mut crate::src::game::ai_main::bot_waypoint_s,
    );
    crate::src::game::ai_dmq3::BotFreeWaypoints(
        (*bs).patrolpoints as *mut crate::src::game::ai_main::bot_waypoint_s,
    );
    //reset the whole state
    crate::stdlib::memset(
        bs as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<crate::src::game::ai_main::bot_state_t>() as libc::c_ulong,
    );
    //copy back some state stuff that should not be reset
    (*bs).ms = movestate;
    (*bs).gs = goalstate;
    (*bs).cs = chatstate;
    (*bs).ws = weaponstate;
    crate::stdlib::memcpy(
        &mut (*bs).cur_ps as *mut crate::src::qcommon::q_shared::playerState_t as *mut libc::c_void,
        &mut ps as *mut crate::src::qcommon::q_shared::playerState_t as *const libc::c_void,
        ::std::mem::size_of::<crate::src::qcommon::q_shared::playerState_t>() as libc::c_ulong,
    );
    crate::stdlib::memcpy(
        &mut (*bs).settings as *mut crate::g_local_h::bot_settings_t as *mut libc::c_void,
        &mut settings as *mut crate::g_local_h::bot_settings_t as *const libc::c_void,
        ::std::mem::size_of::<crate::g_local_h::bot_settings_t>() as libc::c_ulong,
    );
    (*bs).inuse = inuse;
    (*bs).client = client;
    (*bs).entitynum = entitynum;
    (*bs).character = character;
    (*bs).entergame_time = entergame_time;
    //reset several states
    if (*bs).ms != 0 {
        crate::src::game::g_syscalls::trap_BotResetMoveState((*bs).ms);
    }
    if (*bs).gs != 0 {
        crate::src::game::g_syscalls::trap_BotResetGoalState((*bs).gs);
    }
    if (*bs).ws != 0 {
        crate::src::game::g_syscalls::trap_BotResetWeaponState((*bs).ws);
    }
    if (*bs).gs != 0 {
        crate::src::game::g_syscalls::trap_BotResetAvoidGoals((*bs).gs);
    }
    if (*bs).ms != 0 {
        crate::src::game::g_syscalls::trap_BotResetAvoidReach((*bs).ms);
    };
}
/*
==============
BotAILoadMap
==============
*/
#[no_mangle]

pub unsafe extern "C" fn BotAILoadMap(mut restart: libc::c_int) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut mapname: crate::src::qcommon::q_shared::vmCvar_t =
        crate::src::qcommon::q_shared::vmCvar_t {
            handle: 0,
            modificationCount: 0,
            value: 0.,
            integer: 0,
            string: [0; 256],
        };
    if restart == 0 {
        crate::src::game::g_syscalls::trap_Cvar_Register(
            &mut mapname as *mut _ as *mut crate::src::qcommon::q_shared::vmCvar_t,
            b"mapname\x00" as *const u8 as *const libc::c_char,
            b"\x00" as *const u8 as *const libc::c_char,
            0x4 as libc::c_int | 0x40 as libc::c_int,
        );
        crate::src::game::g_syscalls::trap_BotLibLoadMap(mapname.string.as_mut_ptr());
    }
    i = 0 as libc::c_int;
    while i < 64 as libc::c_int {
        if !botstates[i as usize].is_null() && (*botstates[i as usize]).inuse != 0 {
            BotResetState(botstates[i as usize]);
            (*botstates[i as usize]).setupcount = 4 as libc::c_int
        }
        i += 1
    }
    crate::src::game::ai_dmq3::BotSetupDeathmatchAI();
    return crate::src::qcommon::q_shared::qtrue as libc::c_int;
}
/*
==================
BotAIStartFrame
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotAIStartFrame(mut time: libc::c_int) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut ent: *mut crate::g_local_h::gentity_t = 0 as *mut crate::g_local_h::gentity_t;
    let mut state: crate::botlib_h::bot_entitystate_t = crate::botlib_h::bot_entitystate_t {
        type_0: 0,
        flags: 0,
        origin: [0.; 3],
        angles: [0.; 3],
        old_origin: [0.; 3],
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
    let mut elapsed_time: libc::c_int = 0;
    let mut thinktime: libc::c_int = 0;
    static mut local_time: libc::c_int = 0;
    static mut botlib_residual: libc::c_int = 0;
    static mut lastbotthink_time: libc::c_int = 0;
    crate::src::game::g_bot::G_CheckBotSpawn();
    crate::src::game::g_syscalls::trap_Cvar_Update(
        &mut crate::src::game::ai_dmq3::bot_rocketjump as *mut _
            as *mut crate::src::qcommon::q_shared::vmCvar_t,
    );
    crate::src::game::g_syscalls::trap_Cvar_Update(
        &mut crate::src::game::ai_dmq3::bot_grapple as *mut _
            as *mut crate::src::qcommon::q_shared::vmCvar_t,
    );
    crate::src::game::g_syscalls::trap_Cvar_Update(
        &mut crate::src::game::ai_dmq3::bot_fastchat as *mut _
            as *mut crate::src::qcommon::q_shared::vmCvar_t,
    );
    crate::src::game::g_syscalls::trap_Cvar_Update(
        &mut crate::src::game::ai_dmq3::bot_nochat as *mut _
            as *mut crate::src::qcommon::q_shared::vmCvar_t,
    );
    crate::src::game::g_syscalls::trap_Cvar_Update(
        &mut crate::src::game::ai_dmq3::bot_testrchat as *mut _
            as *mut crate::src::qcommon::q_shared::vmCvar_t,
    );
    crate::src::game::g_syscalls::trap_Cvar_Update(
        &mut bot_thinktime as *mut _ as *mut crate::src::qcommon::q_shared::vmCvar_t,
    );
    crate::src::game::g_syscalls::trap_Cvar_Update(
        &mut bot_memorydump as *mut _ as *mut crate::src::qcommon::q_shared::vmCvar_t,
    );
    crate::src::game::g_syscalls::trap_Cvar_Update(
        &mut bot_saveroutingcache as *mut _ as *mut crate::src::qcommon::q_shared::vmCvar_t,
    );
    crate::src::game::g_syscalls::trap_Cvar_Update(
        &mut bot_pause as *mut _ as *mut crate::src::qcommon::q_shared::vmCvar_t,
    );
    crate::src::game::g_syscalls::trap_Cvar_Update(
        &mut bot_report as *mut _ as *mut crate::src::qcommon::q_shared::vmCvar_t,
    );
    if bot_report.integer != 0 {
        //		BotTeamplayReport();
        //		trap_Cvar_Set("bot_report", "0");
        BotUpdateInfoConfigStrings();
    }
    if bot_pause.integer != 0 {
        // execute bot user commands every frame
        i = 0 as libc::c_int;
        while i < 64 as libc::c_int {
            if !(botstates[i as usize].is_null() || (*botstates[i as usize]).inuse == 0) {
                if !((*crate::src::game::g_main::g_entities[i as usize].client)
                    .pers
                    .connected as libc::c_uint
                    != crate::g_local_h::CON_CONNECTED as libc::c_int as libc::c_uint)
                {
                    (*botstates[i as usize]).lastucmd.forwardmove =
                        0 as libc::c_int as libc::c_schar;
                    (*botstates[i as usize]).lastucmd.rightmove = 0 as libc::c_int as libc::c_schar;
                    (*botstates[i as usize]).lastucmd.upmove = 0 as libc::c_int as libc::c_schar;
                    (*botstates[i as usize]).lastucmd.buttons = 0 as libc::c_int;
                    (*botstates[i as usize]).lastucmd.serverTime = time;
                    crate::src::game::g_syscalls::trap_BotUserCommand(
                        (*botstates[i as usize]).client,
                        &mut (**botstates.as_mut_ptr().offset(i as isize)).lastucmd as *mut _
                            as *mut crate::src::qcommon::q_shared::usercmd_s,
                    );
                }
            }
            i += 1
        }
        return crate::src::qcommon::q_shared::qtrue as libc::c_int;
    }
    if bot_memorydump.integer != 0 {
        crate::src::game::g_syscalls::trap_BotLibVarSet(
            b"memorydump\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        crate::src::game::g_syscalls::trap_Cvar_Set(
            b"bot_memorydump\x00" as *const u8 as *const libc::c_char,
            b"0\x00" as *const u8 as *const libc::c_char,
        );
    }
    if bot_saveroutingcache.integer != 0 {
        crate::src::game::g_syscalls::trap_BotLibVarSet(
            b"saveroutingcache\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        crate::src::game::g_syscalls::trap_Cvar_Set(
            b"bot_saveroutingcache\x00" as *const u8 as *const libc::c_char,
            b"0\x00" as *const u8 as *const libc::c_char,
        );
    }
    //check if bot interbreeding is activated
    BotInterbreeding();
    //cap the bot think time
    if bot_thinktime.integer > 200 as libc::c_int {
        crate::src::game::g_syscalls::trap_Cvar_Set(
            b"bot_thinktime\x00" as *const u8 as *const libc::c_char,
            b"200\x00" as *const u8 as *const libc::c_char,
        );
    }
    //if the bot think time changed we should reschedule the bots
    if bot_thinktime.integer != lastbotthink_time {
        lastbotthink_time = bot_thinktime.integer;
        BotScheduleBotThink();
    }
    elapsed_time = time - local_time;
    local_time = time;
    botlib_residual += elapsed_time;
    if elapsed_time > bot_thinktime.integer {
        thinktime = elapsed_time
    } else {
        thinktime = bot_thinktime.integer
    }
    // update the bot library
    if botlib_residual >= thinktime {
        botlib_residual -= thinktime;
        crate::src::game::g_syscalls::trap_BotLibStartFrame(
            time as libc::c_float / 1000 as libc::c_int as libc::c_float,
        );
        if crate::src::game::g_syscalls::trap_AAS_Initialized() == 0 {
            return crate::src::qcommon::q_shared::qfalse as libc::c_int;
        }
        //update entities in the botlib
        i = 0 as libc::c_int;
        while i < (1 as libc::c_int) << 10 as libc::c_int {
            ent = &mut *crate::src::game::g_main::g_entities
                .as_mut_ptr()
                .offset(i as isize) as *mut crate::g_local_h::gentity_t;
            if (*ent).inuse as u64 == 0 {
                crate::src::game::g_syscalls::trap_BotLibUpdateEntity(i, 0 as *mut libc::c_void);
            } else if (*ent).r.linked as u64 == 0 {
                crate::src::game::g_syscalls::trap_BotLibUpdateEntity(i, 0 as *mut libc::c_void);
            } else if (*ent).r.svFlags & 0x1 as libc::c_int != 0 {
                crate::src::game::g_syscalls::trap_BotLibUpdateEntity(i, 0 as *mut libc::c_void);
            } else if (*ent).s.eType == crate::bg_public_h::ET_MISSILE as libc::c_int
                && (*ent).s.weapon != crate::bg_public_h::WP_GRAPPLING_HOOK as libc::c_int
            {
                crate::src::game::g_syscalls::trap_BotLibUpdateEntity(i, 0 as *mut libc::c_void);
            } else if (*ent).s.eType > crate::bg_public_h::ET_EVENTS as libc::c_int {
                crate::src::game::g_syscalls::trap_BotLibUpdateEntity(i, 0 as *mut libc::c_void);
            } else {
                // do not update missiles
                // do not update event only entities
                //
                crate::stdlib::memset(
                    &mut state as *mut crate::botlib_h::bot_entitystate_t as *mut libc::c_void,
                    0 as libc::c_int,
                    ::std::mem::size_of::<crate::botlib_h::bot_entitystate_t>() as libc::c_ulong,
                );
                //
                state.origin[0 as libc::c_int as usize] =
                    (*ent).r.currentOrigin[0 as libc::c_int as usize];
                state.origin[1 as libc::c_int as usize] =
                    (*ent).r.currentOrigin[1 as libc::c_int as usize];
                state.origin[2 as libc::c_int as usize] =
                    (*ent).r.currentOrigin[2 as libc::c_int as usize];
                if i < 64 as libc::c_int {
                    state.angles[0 as libc::c_int as usize] =
                        (*ent).s.apos.trBase[0 as libc::c_int as usize];
                    state.angles[1 as libc::c_int as usize] =
                        (*ent).s.apos.trBase[1 as libc::c_int as usize];
                    state.angles[2 as libc::c_int as usize] =
                        (*ent).s.apos.trBase[2 as libc::c_int as usize]
                } else {
                    state.angles[0 as libc::c_int as usize] =
                        (*ent).r.currentAngles[0 as libc::c_int as usize];
                    state.angles[1 as libc::c_int as usize] =
                        (*ent).r.currentAngles[1 as libc::c_int as usize];
                    state.angles[2 as libc::c_int as usize] =
                        (*ent).r.currentAngles[2 as libc::c_int as usize]
                }
                state.old_origin[0 as libc::c_int as usize] =
                    (*ent).s.origin2[0 as libc::c_int as usize];
                state.old_origin[1 as libc::c_int as usize] =
                    (*ent).s.origin2[1 as libc::c_int as usize];
                state.old_origin[2 as libc::c_int as usize] =
                    (*ent).s.origin2[2 as libc::c_int as usize];
                state.mins[0 as libc::c_int as usize] = (*ent).r.mins[0 as libc::c_int as usize];
                state.mins[1 as libc::c_int as usize] = (*ent).r.mins[1 as libc::c_int as usize];
                state.mins[2 as libc::c_int as usize] = (*ent).r.mins[2 as libc::c_int as usize];
                state.maxs[0 as libc::c_int as usize] = (*ent).r.maxs[0 as libc::c_int as usize];
                state.maxs[1 as libc::c_int as usize] = (*ent).r.maxs[1 as libc::c_int as usize];
                state.maxs[2 as libc::c_int as usize] = (*ent).r.maxs[2 as libc::c_int as usize];
                state.type_0 = (*ent).s.eType;
                state.flags = (*ent).s.eFlags;
                if (*ent).r.bmodel as u64 != 0 {
                    state.solid = crate::be_aas_h::SOLID_BSP as libc::c_int
                } else {
                    state.solid = crate::be_aas_h::SOLID_BBOX as libc::c_int
                }
                state.groundent = (*ent).s.groundEntityNum;
                state.modelindex = (*ent).s.modelindex;
                state.modelindex2 = (*ent).s.modelindex2;
                state.frame = (*ent).s.frame;
                state.event = (*ent).s.event;
                state.eventParm = (*ent).s.eventParm;
                state.powerups = (*ent).s.powerups;
                state.legsAnim = (*ent).s.legsAnim;
                state.torsoAnim = (*ent).s.torsoAnim;
                state.weapon = (*ent).s.weapon;
                //
                crate::src::game::g_syscalls::trap_BotLibUpdateEntity(
                    i,
                    &mut state as *mut crate::botlib_h::bot_entitystate_t as *mut libc::c_void,
                );
            }
            i += 1
        }
        BotAIRegularUpdate();
    }
    floattime = crate::src::game::g_syscalls::trap_AAS_Time();
    // execute scheduled bot AI
    i = 0 as libc::c_int;
    while i < 64 as libc::c_int {
        if !(botstates[i as usize].is_null() || (*botstates[i as usize]).inuse == 0) {
            //
            (*botstates[i as usize]).botthink_residual += elapsed_time;
            //
            if (*botstates[i as usize]).botthink_residual >= thinktime {
                (*botstates[i as usize]).botthink_residual -= thinktime;
                if crate::src::game::g_syscalls::trap_AAS_Initialized() == 0 {
                    return crate::src::qcommon::q_shared::qfalse as libc::c_int;
                }
                if (*crate::src::game::g_main::g_entities[i as usize].client)
                    .pers
                    .connected as libc::c_uint
                    == crate::g_local_h::CON_CONNECTED as libc::c_int as libc::c_uint
                {
                    BotAI(
                        i,
                        thinktime as libc::c_float / 1000 as libc::c_int as libc::c_float,
                    );
                }
            }
        }
        i += 1
    }
    // execute bot user commands every frame
    i = 0 as libc::c_int;
    while i < 64 as libc::c_int {
        if !(botstates[i as usize].is_null() || (*botstates[i as usize]).inuse == 0) {
            if !((*crate::src::game::g_main::g_entities[i as usize].client)
                .pers
                .connected as libc::c_uint
                != crate::g_local_h::CON_CONNECTED as libc::c_int as libc::c_uint)
            {
                BotUpdateInput(botstates[i as usize], time, elapsed_time);
                crate::src::game::g_syscalls::trap_BotUserCommand(
                    (*botstates[i as usize]).client,
                    &mut (**botstates.as_mut_ptr().offset(i as isize)).lastucmd as *mut _
                        as *mut crate::src::qcommon::q_shared::usercmd_s,
                );
            }
        }
        i += 1
    }
    return crate::src::qcommon::q_shared::qtrue as libc::c_int;
}
/*
==============
BotInitLibrary
==============
*/
#[no_mangle]

pub unsafe extern "C" fn BotInitLibrary() -> libc::c_int {
    let mut buf: [libc::c_char; 144] = [0; 144];
    //set the maxclients and maxentities library variables before calling BotSetupLibrary
    crate::src::qcommon::q_shared::Com_sprintf(
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 144]>() as libc::c_ulong as libc::c_int,
        b"%d\x00" as *const u8 as *const libc::c_char,
        crate::src::game::g_main::level.maxclients,
    );
    crate::src::game::g_syscalls::trap_BotLibVarSet(
        b"maxclients\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        buf.as_mut_ptr(),
    );
    crate::src::qcommon::q_shared::Com_sprintf(
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 144]>() as libc::c_ulong as libc::c_int,
        b"%d\x00" as *const u8 as *const libc::c_char,
        (1 as libc::c_int) << 10 as libc::c_int,
    );
    crate::src::game::g_syscalls::trap_BotLibVarSet(
        b"maxentities\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        buf.as_mut_ptr(),
    );
    //bsp checksum
    crate::src::game::g_syscalls::trap_Cvar_VariableStringBuffer(
        b"sv_mapChecksum\x00" as *const u8 as *const libc::c_char,
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 144]>() as libc::c_ulong as libc::c_int,
    );
    if crate::stdlib::strlen(buf.as_mut_ptr()) != 0 {
        crate::src::game::g_syscalls::trap_BotLibVarSet(
            b"sv_mapChecksum\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            buf.as_mut_ptr(),
        );
    }
    //maximum number of aas links
    crate::src::game::g_syscalls::trap_Cvar_VariableStringBuffer(
        b"max_aaslinks\x00" as *const u8 as *const libc::c_char,
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 144]>() as libc::c_ulong as libc::c_int,
    );
    if crate::stdlib::strlen(buf.as_mut_ptr()) != 0 {
        crate::src::game::g_syscalls::trap_BotLibVarSet(
            b"max_aaslinks\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            buf.as_mut_ptr(),
        );
    }
    //maximum number of items in a level
    crate::src::game::g_syscalls::trap_Cvar_VariableStringBuffer(
        b"max_levelitems\x00" as *const u8 as *const libc::c_char,
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 144]>() as libc::c_ulong as libc::c_int,
    );
    if crate::stdlib::strlen(buf.as_mut_ptr()) != 0 {
        crate::src::game::g_syscalls::trap_BotLibVarSet(
            b"max_levelitems\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            buf.as_mut_ptr(),
        );
    }
    //game type
    crate::src::game::g_syscalls::trap_Cvar_VariableStringBuffer(
        b"g_gametype\x00" as *const u8 as *const libc::c_char,
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 144]>() as libc::c_ulong as libc::c_int,
    );
    if crate::stdlib::strlen(buf.as_mut_ptr()) == 0 {
        ::libc::strcpy(
            buf.as_mut_ptr(),
            b"0\x00" as *const u8 as *const libc::c_char,
        );
    }
    crate::src::game::g_syscalls::trap_BotLibVarSet(
        b"g_gametype\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        buf.as_mut_ptr(),
    );
    //bot developer mode and log file
    crate::src::game::g_syscalls::trap_BotLibVarSet(
        b"bot_developer\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        bot_developer.string.as_mut_ptr(),
    );
    crate::src::game::g_syscalls::trap_Cvar_VariableStringBuffer(
        b"logfile\x00" as *const u8 as *const libc::c_char,
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 144]>() as libc::c_ulong as libc::c_int,
    );
    crate::src::game::g_syscalls::trap_BotLibVarSet(
        b"log\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        buf.as_mut_ptr(),
    );
    //no chatting
    crate::src::game::g_syscalls::trap_Cvar_VariableStringBuffer(
        b"bot_nochat\x00" as *const u8 as *const libc::c_char,
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 144]>() as libc::c_ulong as libc::c_int,
    );
    if crate::stdlib::strlen(buf.as_mut_ptr()) != 0 {
        crate::src::game::g_syscalls::trap_BotLibVarSet(
            b"nochat\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            buf.as_mut_ptr(),
        );
    }
    //visualize jump pads
    crate::src::game::g_syscalls::trap_Cvar_VariableStringBuffer(
        b"bot_visualizejumppads\x00" as *const u8 as *const libc::c_char,
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 144]>() as libc::c_ulong as libc::c_int,
    );
    if crate::stdlib::strlen(buf.as_mut_ptr()) != 0 {
        crate::src::game::g_syscalls::trap_BotLibVarSet(
            b"bot_visualizejumppads\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            buf.as_mut_ptr(),
        );
    }
    //forced clustering calculations
    crate::src::game::g_syscalls::trap_Cvar_VariableStringBuffer(
        b"bot_forceclustering\x00" as *const u8 as *const libc::c_char,
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 144]>() as libc::c_ulong as libc::c_int,
    );
    if crate::stdlib::strlen(buf.as_mut_ptr()) != 0 {
        crate::src::game::g_syscalls::trap_BotLibVarSet(
            b"forceclustering\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            buf.as_mut_ptr(),
        );
    }
    //forced reachability calculations
    crate::src::game::g_syscalls::trap_Cvar_VariableStringBuffer(
        b"bot_forcereachability\x00" as *const u8 as *const libc::c_char,
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 144]>() as libc::c_ulong as libc::c_int,
    );
    if crate::stdlib::strlen(buf.as_mut_ptr()) != 0 {
        crate::src::game::g_syscalls::trap_BotLibVarSet(
            b"forcereachability\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            buf.as_mut_ptr(),
        );
    }
    //force writing of AAS to file
    crate::src::game::g_syscalls::trap_Cvar_VariableStringBuffer(
        b"bot_forcewrite\x00" as *const u8 as *const libc::c_char,
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 144]>() as libc::c_ulong as libc::c_int,
    );
    if crate::stdlib::strlen(buf.as_mut_ptr()) != 0 {
        crate::src::game::g_syscalls::trap_BotLibVarSet(
            b"forcewrite\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            buf.as_mut_ptr(),
        );
    }
    //no AAS optimization
    crate::src::game::g_syscalls::trap_Cvar_VariableStringBuffer(
        b"bot_aasoptimize\x00" as *const u8 as *const libc::c_char,
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 144]>() as libc::c_ulong as libc::c_int,
    );
    if crate::stdlib::strlen(buf.as_mut_ptr()) != 0 {
        crate::src::game::g_syscalls::trap_BotLibVarSet(
            b"aasoptimize\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            buf.as_mut_ptr(),
        );
    }
    //
    crate::src::game::g_syscalls::trap_Cvar_VariableStringBuffer(
        b"bot_saveroutingcache\x00" as *const u8 as *const libc::c_char,
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 144]>() as libc::c_ulong as libc::c_int,
    );
    if crate::stdlib::strlen(buf.as_mut_ptr()) != 0 {
        crate::src::game::g_syscalls::trap_BotLibVarSet(
            b"saveroutingcache\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            buf.as_mut_ptr(),
        );
    }
    //reload instead of cache bot character files
    crate::src::game::g_syscalls::trap_Cvar_VariableStringBuffer(
        b"bot_reloadcharacters\x00" as *const u8 as *const libc::c_char,
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 144]>() as libc::c_ulong as libc::c_int,
    );
    if crate::stdlib::strlen(buf.as_mut_ptr()) == 0 {
        ::libc::strcpy(
            buf.as_mut_ptr(),
            b"0\x00" as *const u8 as *const libc::c_char,
        );
    }
    crate::src::game::g_syscalls::trap_BotLibVarSet(
        b"bot_reloadcharacters\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        buf.as_mut_ptr(),
    );
    //base directory
    crate::src::game::g_syscalls::trap_Cvar_VariableStringBuffer(
        b"fs_basepath\x00" as *const u8 as *const libc::c_char,
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 144]>() as libc::c_ulong as libc::c_int,
    );
    if crate::stdlib::strlen(buf.as_mut_ptr()) != 0 {
        crate::src::game::g_syscalls::trap_BotLibVarSet(
            b"basedir\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            buf.as_mut_ptr(),
        );
    }
    //game directory
    crate::src::game::g_syscalls::trap_Cvar_VariableStringBuffer(
        b"fs_game\x00" as *const u8 as *const libc::c_char,
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 144]>() as libc::c_ulong as libc::c_int,
    );
    if crate::stdlib::strlen(buf.as_mut_ptr()) != 0 {
        crate::src::game::g_syscalls::trap_BotLibVarSet(
            b"gamedir\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            buf.as_mut_ptr(),
        );
    }
    //home directory
    crate::src::game::g_syscalls::trap_Cvar_VariableStringBuffer(
        b"fs_homepath\x00" as *const u8 as *const libc::c_char,
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 144]>() as libc::c_ulong as libc::c_int,
    );
    if crate::stdlib::strlen(buf.as_mut_ptr()) != 0 {
        crate::src::game::g_syscalls::trap_BotLibVarSet(
            b"homedir\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            buf.as_mut_ptr(),
        );
    }
    //
    //setup the bot library
    return crate::src::game::g_syscalls::trap_BotLibSetup();
}
/*
==============
BotAISetup
==============
*/
#[no_mangle]

pub unsafe extern "C" fn BotAISetup(mut restart: libc::c_int) -> libc::c_int {
    let mut errnum: libc::c_int = 0;
    crate::src::game::g_syscalls::trap_Cvar_Register(
        &mut bot_thinktime as *mut _ as *mut crate::src::qcommon::q_shared::vmCvar_t,
        b"bot_thinktime\x00" as *const u8 as *const libc::c_char,
        b"100\x00" as *const u8 as *const libc::c_char,
        0x200 as libc::c_int,
    );
    crate::src::game::g_syscalls::trap_Cvar_Register(
        &mut bot_memorydump as *mut _ as *mut crate::src::qcommon::q_shared::vmCvar_t,
        b"bot_memorydump\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x200 as libc::c_int,
    );
    crate::src::game::g_syscalls::trap_Cvar_Register(
        &mut bot_saveroutingcache as *mut _ as *mut crate::src::qcommon::q_shared::vmCvar_t,
        b"bot_saveroutingcache\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x200 as libc::c_int,
    );
    crate::src::game::g_syscalls::trap_Cvar_Register(
        &mut bot_pause as *mut _ as *mut crate::src::qcommon::q_shared::vmCvar_t,
        b"bot_pause\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x200 as libc::c_int,
    );
    crate::src::game::g_syscalls::trap_Cvar_Register(
        &mut bot_report as *mut _ as *mut crate::src::qcommon::q_shared::vmCvar_t,
        b"bot_report\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x200 as libc::c_int,
    );
    crate::src::game::g_syscalls::trap_Cvar_Register(
        &mut bot_testsolid as *mut _ as *mut crate::src::qcommon::q_shared::vmCvar_t,
        b"bot_testsolid\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x200 as libc::c_int,
    );
    crate::src::game::g_syscalls::trap_Cvar_Register(
        &mut bot_testclusters as *mut _ as *mut crate::src::qcommon::q_shared::vmCvar_t,
        b"bot_testclusters\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x200 as libc::c_int,
    );
    crate::src::game::g_syscalls::trap_Cvar_Register(
        &mut bot_developer as *mut _ as *mut crate::src::qcommon::q_shared::vmCvar_t,
        b"bot_developer\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x200 as libc::c_int,
    );
    crate::src::game::g_syscalls::trap_Cvar_Register(
        &mut bot_interbreedchar as *mut _ as *mut crate::src::qcommon::q_shared::vmCvar_t,
        b"bot_interbreedchar\x00" as *const u8 as *const libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
    );
    crate::src::game::g_syscalls::trap_Cvar_Register(
        &mut bot_interbreedbots as *mut _ as *mut crate::src::qcommon::q_shared::vmCvar_t,
        b"bot_interbreedbots\x00" as *const u8 as *const libc::c_char,
        b"10\x00" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
    );
    crate::src::game::g_syscalls::trap_Cvar_Register(
        &mut bot_interbreedcycle as *mut _ as *mut crate::src::qcommon::q_shared::vmCvar_t,
        b"bot_interbreedcycle\x00" as *const u8 as *const libc::c_char,
        b"20\x00" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
    );
    crate::src::game::g_syscalls::trap_Cvar_Register(
        &mut bot_interbreedwrite as *mut _ as *mut crate::src::qcommon::q_shared::vmCvar_t,
        b"bot_interbreedwrite\x00" as *const u8 as *const libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
    );
    //if the game is restarted for a tournament
    if restart != 0 {
        return crate::src::qcommon::q_shared::qtrue as libc::c_int;
    }
    //initialize the bot states
    crate::stdlib::memset(
        botstates.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[*mut crate::src::game::ai_main::bot_state_t; 64]>() as libc::c_ulong,
    );
    errnum = BotInitLibrary();
    if errnum != 0 as libc::c_int {
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    }
    return crate::src::qcommon::q_shared::qtrue as libc::c_int;
}
/*
==============
BotAIShutdown
==============
*/
#[no_mangle]

pub unsafe extern "C" fn BotAIShutdown(mut restart: libc::c_int) -> libc::c_int {
    let mut i: libc::c_int = 0;
    //if the game is restarted for a tournament
    if restart != 0 {
        //shutdown all the bots in the botlib
        i = 0 as libc::c_int;
        while i < 64 as libc::c_int {
            if !botstates[i as usize].is_null() && (*botstates[i as usize]).inuse != 0 {
                BotAIShutdownClient(
                    (*botstates[i as usize]).client,
                    restart as crate::src::qcommon::q_shared::qboolean,
                );
            }
            i += 1
        }
    //don't shutdown the bot library
    } else {
        crate::src::game::g_syscalls::trap_BotLibShutdown();
    }
    return crate::src::qcommon::q_shared::qtrue as libc::c_int;
}

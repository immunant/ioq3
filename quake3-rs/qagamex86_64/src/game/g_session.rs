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
pub use crate::g_local_h::clientConnected_t;
pub use crate::g_local_h::clientPersistant_t;
pub use crate::g_local_h::clientSession_t;
pub use crate::g_local_h::gclient_s;
pub use crate::g_local_h::gclient_t;
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
pub use crate::g_public_h::entityShared_t;
pub use crate::src::game::g_cmds::SetTeam;
pub use crate::src::game::g_main::g_entities;
pub use crate::src::game::g_main::g_gametype;
pub use crate::src::game::g_main::g_localTeamPref;
pub use crate::src::game::g_main::g_maxGameClients;
pub use crate::src::game::g_main::g_teamAutoJoin;
pub use crate::src::game::g_main::level;
pub use crate::src::game::g_main::AddTournamentQueue;
pub use crate::src::game::g_main::G_Printf;
pub use crate::src::game::g_session::stdlib_h::atoi;
pub use crate::src::game::g_syscalls::trap_Cvar_Set;
pub use crate::src::game::g_syscalls::trap_Cvar_VariableStringBuffer;
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
pub use crate::src::qcommon::q_shared::Info_ValueForKey;
pub use crate::src::qcommon::q_shared::TR_GRAVITY;
pub use crate::src::qcommon::q_shared::TR_INTERPOLATE;
pub use crate::src::qcommon::q_shared::TR_LINEAR;
pub use crate::src::qcommon::q_shared::TR_LINEAR_STOP;
pub use crate::src::qcommon::q_shared::TR_SINE;
pub use crate::src::qcommon::q_shared::TR_STATIONARY;

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
/*
=======================================================================

  SESSION DATA

Session data is the only data that stays persistant across level loads
and tournament restarts.
=======================================================================
*/
/*
================
G_WriteClientSessionData

Called on game shutdown
================
*/
#[no_mangle]

pub unsafe extern "C" fn G_WriteClientSessionData(mut client: *mut crate::g_local_h::gclient_t) {
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut var: *const libc::c_char = 0 as *const libc::c_char;
    s = crate::src::qcommon::q_shared::va(
        b"%i %i %i %i %i %i %i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (*client).sess.sessionTeam as libc::c_uint,
        (*client).sess.spectatorNum,
        (*client).sess.spectatorState as libc::c_uint,
        (*client).sess.spectatorClient,
        (*client).sess.wins,
        (*client).sess.losses,
        (*client).sess.teamLeader as libc::c_uint,
    );
    var = crate::src::qcommon::q_shared::va(
        b"session%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        client.offset_from(crate::src::game::g_main::level.clients) as libc::c_long
            as libc::c_int,
    );
    crate::src::game::g_syscalls::trap_Cvar_Set(var, s);
}
/*
================
G_ReadSessionData

Called on a reconnect
================
*/
#[no_mangle]

pub unsafe extern "C" fn G_ReadSessionData(mut client: *mut crate::g_local_h::gclient_t) {
    let mut s: [libc::c_char; 1024] = [0; 1024];
    let mut var: *const libc::c_char = 0 as *const libc::c_char;
    let mut teamLeader: libc::c_int = 0;
    let mut spectatorState: libc::c_int = 0;
    let mut sessionTeam: libc::c_int = 0;
    var = crate::src::qcommon::q_shared::va(
        b"session%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        client.offset_from(crate::src::game::g_main::level.clients) as libc::c_long
            as libc::c_int,
    );
    crate::src::game::g_syscalls::trap_Cvar_VariableStringBuffer(
        var,
        s.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
    );
    ::libc::sscanf(
        s.as_mut_ptr(),
        b"%i %i %i %i %i %i %i\x00" as *const u8 as *const libc::c_char,
        &mut sessionTeam as *mut libc::c_int,
        &mut (*client).sess.spectatorNum as *mut libc::c_int,
        &mut spectatorState as *mut libc::c_int,
        &mut (*client).sess.spectatorClient as *mut libc::c_int,
        &mut (*client).sess.wins as *mut libc::c_int,
        &mut (*client).sess.losses as *mut libc::c_int,
        &mut teamLeader as *mut libc::c_int,
    );
    (*client).sess.sessionTeam = sessionTeam as crate::bg_public_h::team_t;
    (*client).sess.spectatorState = spectatorState as crate::g_local_h::spectatorState_t;
    (*client).sess.teamLeader = teamLeader as crate::src::qcommon::q_shared::qboolean;
}
/*
================
G_InitSessionData

Called on a first-time connect
================
*/
#[no_mangle]

pub unsafe extern "C" fn G_InitSessionData(
    mut client: *mut crate::g_local_h::gclient_t,
    mut userinfo: *mut libc::c_char,
) {
    let mut sess: *mut crate::g_local_h::clientSession_t =
        0 as *mut crate::g_local_h::clientSession_t;
    let mut value: *const libc::c_char = 0 as *const libc::c_char;
    sess = &mut (*client).sess;
    // check for team preference, mainly for bots
    value = crate::src::qcommon::q_shared::Info_ValueForKey(
        userinfo,
        b"teampref\x00" as *const u8 as *const libc::c_char,
    );
    // check for human's team preference set by start server menu
    if *value.offset(0 as libc::c_int as isize) == 0
        && crate::src::game::g_main::g_localTeamPref.string[0 as libc::c_int as usize]
            as libc::c_int
            != 0
        && (*client).pers.localClient as libc::c_uint != 0
    {
        value = crate::src::game::g_main::g_localTeamPref
            .string
            .as_mut_ptr();
        // clear team so it's only used once
        crate::src::game::g_syscalls::trap_Cvar_Set(
            b"g_localTeamPref\x00" as *const u8 as *const libc::c_char,
            b"\x00" as *const u8 as *const libc::c_char,
        );
    }
    // initial team determination
    if crate::src::game::g_main::g_gametype.integer >= crate::bg_public_h::GT_TEAM as libc::c_int {
        // always spawn as spectator in team games
        (*sess).sessionTeam = crate::bg_public_h::TEAM_SPECTATOR;
        (*sess).spectatorState = crate::g_local_h::SPECTATOR_FREE;
        if *value.offset(0 as libc::c_int as isize) as libc::c_int != 0
            || crate::src::game::g_main::g_teamAutoJoin.integer != 0
        {
            crate::src::game::g_cmds::SetTeam(
                &mut *crate::src::game::g_main::g_entities.as_mut_ptr().offset(
                    client.offset_from(crate::src::game::g_main::level.clients)
                        as libc::c_long as isize,
                ) as *mut _ as *mut crate::g_local_h::gentity_s,
                value,
            );
        }
    } else {
        if *value.offset(0 as libc::c_int as isize) as libc::c_int == 's' as i32 {
            // a willing spectator, not a waiting-in-line
            (*sess).sessionTeam = crate::bg_public_h::TEAM_SPECTATOR
        } else {
            match crate::src::game::g_main::g_gametype.integer {
                1 => {
                    // if the game is full, go into a waiting mode
                    if crate::src::game::g_main::level.numNonSpectatorClients >= 2 as libc::c_int {
                        (*sess).sessionTeam = crate::bg_public_h::TEAM_SPECTATOR
                    } else {
                        (*sess).sessionTeam = crate::bg_public_h::TEAM_FREE
                    }
                }
                0 | 2 | _ => {
                    if crate::src::game::g_main::g_maxGameClients.integer > 0 as libc::c_int
                        && crate::src::game::g_main::level.numNonSpectatorClients
                            >= crate::src::game::g_main::g_maxGameClients.integer
                    {
                        (*sess).sessionTeam = crate::bg_public_h::TEAM_SPECTATOR
                    } else {
                        (*sess).sessionTeam = crate::bg_public_h::TEAM_FREE
                    }
                }
            }
        }
        (*sess).spectatorState = crate::g_local_h::SPECTATOR_FREE
    }
    crate::src::game::g_main::AddTournamentQueue(client as *mut crate::g_local_h::gclient_s);
    G_WriteClientSessionData(client);
}
/*
==================
G_InitWorldSession

==================
*/
#[no_mangle]

pub unsafe extern "C" fn G_InitWorldSession() {
    let mut s: [libc::c_char; 1024] = [0; 1024];
    let mut gt: libc::c_int = 0;
    crate::src::game::g_syscalls::trap_Cvar_VariableStringBuffer(
        b"session\x00" as *const u8 as *const libc::c_char,
        s.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
    );
    gt = atoi(s.as_mut_ptr());
    // if the gametype changed since the last session, don't use any
    // client sessions
    if crate::src::game::g_main::g_gametype.integer != gt {
        crate::src::game::g_main::level.newSession = crate::src::qcommon::q_shared::qtrue;
        crate::src::game::g_main::G_Printf(
            b"Gametype changed, clearing session data.\n\x00" as *const u8 as *const libc::c_char,
        );
    };
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
// g_local.h -- local definitions for game module
//==================================================================
// the "gameversion" client command will print this plus compile date
// msec
// gentity->flags
// not the first on the team
// spawn point not for bot use
// spawn point just for bots
// force gesture on client
// movers are things like doors, plats, buttons, etc
//============================================================================
// communicated by server to clients
// shared by both the server system and game
// DO NOT MODIFY ANYTHING ABOVE THIS, THE SERVER
// EXPECTS THE FIELDS IN THAT ORDER!
//================================
// NULL if not a client
// set in QuakeEd
// set in QuakeEd
// if true, FreeEntity will only unlink
// bodyque uses this
// FL_* variables
// level.time when the object was freed
// events will be cleared EVENT_VALID_MSEC after set
// if true, it can be pushed by movers and fall off edges
// all game items are physicsObjects,
// 1.0 = continuous bounce, 0.0 = no bounce
// brushes with this content value will be collided against
// when moving.  items and corpses do not collide against
// players, for instance
// movers
// body queue sinking, etc
// movers call this when hitting endpoint
// wind tunnel
// quad will increase this without increasing radius
// next entity in team
// master of the team
// timing variables
// for bonus items
// Beginning a team game, spawn at base
// Now actively playing
// client data that stays across multiple levels or tournament restarts
// this is achieved by writing all the data to cvar strings at game shutdown
// time and reading them back at connection time.  Anything added here
// MUST be dealt with in G_InitSessionData() / G_ReadSessionData() / G_WriteSessionData()
// for determining next-in-line to play
// for chasecam and follow mode
// tournament stats
// true when this client is a team leader
//
// client data that stays across multiple respawns, but is cleared
// on each level change or team change at ClientBegin()
// we would lose angles if not persistant
// true if "ip" info key is "localhost"
// the first spawn should be at a cool location
// based on cg_predictItems userinfo
//
// for handicapping
// level.time the client entered the game
// status in teamplay games
// to prevent people from constantly calling votes
// to prevent people from constantly calling votes
// send team overlay updates?
// this structure is cleared on each ClientSpawn(),
// except for 'client->pers' and 'client->sess'
// ps MUST be the first element, because the server expects it
// communicated by server to clients
// the rest of the structure is private to game
// wishes to leave the intermission
// level.time of last usercmd_t, for EF_CONNECTION
// we can't just use pers.lastCommand.time, because
// of the g_sycronousclients case
// sum up damage over an entire frame, so
// shotgun blasts give a single big kick
// damage absorbed by armor
// damage taken out of health
// impact damage
// origin for vector calculation
// if true, don't use the damage_from vector
// for "impressive" reward sound
// total number of shots
// total number of hits
//
// last client that this client killed
// last client that damaged this client
// type of damage the client did
// timers
// can respawn when time > this, force after g_forcerespwan
// kick players when time > this
// qtrue if the five seoond warning has been given
// clear the EF_AWARD_IMPRESSIVE, etc when time > this
// for multiple kill rewards
// used for hook
// grapple hook if out
// time the player switched teams
// timeResidual is used to handle events that happen every second
// like health / armor countdowns and regeneration
//
// this structure is cleared as each map is entered
//
// [maxclients]
// MAX_CLIENTS <= num_entities <= ENTITYNUM_MAX_NORMAL
// restart match at this time
// store latched cvars here that we want to get at often
// in msec
// so movers can back up when blocked
// level.time the map was started
// last time of client team location update
// don't use any old session data, because
// we changed gametype
// waiting for a map_restart to fire
// includes connecting clients
// connected, non-spectators
// sorted by score
// clientNums for auto-follow spectators
// sound index for standing in lava
// for detecting if g_warmup is changed
// voting state
// level.time vote was called
// time the vote is executed
// set by CalculateRanks
// team voting state
// level.time vote was called
// set by CalculateRanks
// spawn variables
// the G_Spawn*() functions are valid
// key / value pairs
// intermission state
// intermission was qualified, but
// wait INTERMISSION_DELAY_TIME before
// actually going there so the last
// frag can be watched.  Disable future
// kills during this delay
// time the intermission was started
// at least one client wants to exit
// also used for spectator spawns
// target_locations get linked
// head of the location list
// dead bodies
//
// g_spawn.c
//
// spawn string returns a temporary reference, you must CopyString() if you want to keep it
//
// g_cmds.c
//
//
// g_items.c
//
//
// g_utils.c
//
//
// g_combat.c
//
// damage flags
// damage was indirect
// armour does not protect from this damage
// do not affect velocity, just view angles
// armor, shields, invulnerability, and godmode have no effect
//
// g_missile.c
//
//
// g_mover.c
//
//
// g_trigger.c
//
//
// g_misc.c
//
//
// g_weapon.c
//
//
// g_client.c
//
//
// g_svcmds.c
//
//
// g_weapon.c
//
//
// g_cmds.c
//
//
// g_main.c
//
//
// g_client.c
//
//
// g_active.c
//
//
// g_team.c
//
//
// g_mem.c
//
//
// g_session.c
//
/*
==================
G_WriteSessionData

==================
*/
#[no_mangle]

pub unsafe extern "C" fn G_WriteSessionData() {
    let mut i: libc::c_int = 0;
    crate::src::game::g_syscalls::trap_Cvar_Set(
        b"session\x00" as *const u8 as *const libc::c_char,
        crate::src::qcommon::q_shared::va(
            b"%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            crate::src::game::g_main::g_gametype.integer,
        ),
    );
    i = 0 as libc::c_int;
    while i < crate::src::game::g_main::level.maxclients {
        if (*crate::src::game::g_main::level.clients.offset(i as isize))
            .pers
            .connected as libc::c_uint
            == crate::g_local_h::CON_CONNECTED as libc::c_int as libc::c_uint
        {
            G_WriteClientSessionData(
                &mut *crate::src::game::g_main::level.clients.offset(i as isize),
            );
        }
        i += 1
    }
}

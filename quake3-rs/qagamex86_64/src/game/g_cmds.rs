use ::libc;

pub mod ctype_h {

    #[inline]

    pub unsafe extern "C" fn tolower(mut __c: i32) -> i32 {
        return if __c >= -(128 as i32) && __c < 256 as i32 {
            *(*crate::stdlib::__ctype_tolower_loc()).offset(__c as isize)
        } else {
            __c
        };
    }
}

pub mod stdlib_float_h {
    #[inline]

    pub unsafe extern "C" fn atof(mut __nptr: *const libc::c_char) -> f64 {
        return ::libc::strtod(__nptr, 0 as *mut libc::c_void as *mut *mut libc::c_char);
    }
}

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

pub use crate::stdlib::__int32_t;

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
pub use crate::bg_public_h::MOD_BFG;
pub use crate::bg_public_h::MOD_BFG_SPLASH;
pub use crate::bg_public_h::MOD_CRUSH;
pub use crate::bg_public_h::MOD_FALLING;
pub use crate::bg_public_h::MOD_GAUNTLET;
pub use crate::bg_public_h::MOD_GRAPPLE;
pub use crate::bg_public_h::MOD_GRENADE;
pub use crate::bg_public_h::MOD_GRENADE_SPLASH;
pub use crate::bg_public_h::MOD_LAVA;
pub use crate::bg_public_h::MOD_LIGHTNING;
pub use crate::bg_public_h::MOD_MACHINEGUN;
pub use crate::bg_public_h::MOD_PLASMA;
pub use crate::bg_public_h::MOD_PLASMA_SPLASH;
pub use crate::bg_public_h::MOD_RAILGUN;
pub use crate::bg_public_h::MOD_ROCKET;
pub use crate::bg_public_h::MOD_ROCKET_SPLASH;
pub use crate::bg_public_h::MOD_SHOTGUN;
pub use crate::bg_public_h::MOD_SLIME;
pub use crate::bg_public_h::MOD_SUICIDE;
pub use crate::bg_public_h::MOD_TARGET_LASER;
pub use crate::bg_public_h::MOD_TELEFRAG;
pub use crate::bg_public_h::MOD_TRIGGER_HURT;
pub use crate::bg_public_h::MOD_UNKNOWN;
pub use crate::bg_public_h::MOD_WATER;
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
pub use crate::bg_public_h::STAT_ARMOR;
pub use crate::bg_public_h::STAT_CLIENTS_READY;
pub use crate::bg_public_h::STAT_DEAD_YAW;
pub use crate::bg_public_h::STAT_HEALTH;
pub use crate::bg_public_h::STAT_HOLDABLE_ITEM;
pub use crate::bg_public_h::STAT_MAX_HEALTH;
pub use crate::bg_public_h::STAT_WEAPONS;
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
pub use crate::src::game::bg_misc::BG_FindItem;
pub use crate::src::game::g_cmds::ctype_h::tolower;
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
pub use crate::src::qcommon::q_shared::Info_SetValueForKey;
pub use crate::src::qcommon::q_shared::Q_CleanStr;
pub use crate::src::qcommon::q_shared::Q_stricmp;
pub use crate::src::qcommon::q_shared::Q_stricmpn;
pub use crate::src::qcommon::q_shared::Q_strncpyz;
pub use crate::src::qcommon::q_shared::EXEC_APPEND;
pub use crate::src::qcommon::q_shared::EXEC_INSERT;
pub use crate::src::qcommon::q_shared::EXEC_NOW;
pub use crate::src::qcommon::q_shared::TR_GRAVITY;
pub use crate::src::qcommon::q_shared::TR_INTERPOLATE;
pub use crate::src::qcommon::q_shared::TR_LINEAR;
pub use crate::src::qcommon::q_shared::TR_LINEAR_STOP;
pub use crate::src::qcommon::q_shared::TR_SINE;
pub use crate::src::qcommon::q_shared::TR_STATIONARY;
pub use crate::stdlib::_ISalnum;
pub use crate::stdlib::_ISalpha;
pub use crate::stdlib::_ISblank;
pub use crate::stdlib::_IScntrl;
pub use crate::stdlib::_ISdigit;
pub use crate::stdlib::_ISgraph;
pub use crate::stdlib::_ISlower;
pub use crate::stdlib::_ISprint;
pub use crate::stdlib::_ISpunct;
pub use crate::stdlib::_ISspace;
pub use crate::stdlib::_ISupper;
pub use crate::stdlib::_ISxdigit;
pub use crate::stdlib::__ctype_b_loc;
pub use crate::stdlib::__ctype_tolower_loc;

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
pub use crate::src::game::g_client::ClientBegin;
pub use crate::src::game::g_client::ClientUserinfoChanged;
pub use crate::src::game::g_client::CopyToBodyQue;
pub use crate::src::game::g_client::PickTeam;
pub use crate::src::game::g_client::SetClientViewAngle;
pub use crate::src::game::g_client::TeamCount;
pub use crate::src::game::g_client::TeamLeader;
pub use crate::src::game::g_cmds::stdlib_float_h::atof;
pub use crate::src::game::g_cmds::stdlib_h::atoi;
pub use crate::src::game::g_combat::player_die;
pub use crate::src::game::g_items::FinishSpawningItem;
pub use crate::src::game::g_items::G_SpawnItem;
pub use crate::src::game::g_items::Touch_Item;
pub use crate::src::game::g_main::g_allowVote;
pub use crate::src::game::g_main::g_cheats;
pub use crate::src::game::g_main::g_dedicated;
pub use crate::src::game::g_main::g_entities;
pub use crate::src::game::g_main::g_gametype;
pub use crate::src::game::g_main::g_maxGameClients;
pub use crate::src::game::g_main::g_teamForceBalance;
pub use crate::src::game::g_main::level;
pub use crate::src::game::g_main::AddTournamentQueue;
pub use crate::src::game::g_main::BeginIntermission;
pub use crate::src::game::g_main::CheckTeamLeader;
pub use crate::src::game::g_main::G_Error;
pub use crate::src::game::g_main::G_LogPrintf;
pub use crate::src::game::g_main::G_Printf;
pub use crate::src::game::g_main::SetLeader;
pub use crate::src::game::g_misc::TeleportPlayer;
pub use crate::src::game::g_syscalls::trap_Argc;
pub use crate::src::game::g_syscalls::trap_Argv;
pub use crate::src::game::g_syscalls::trap_Cvar_VariableStringBuffer;
pub use crate::src::game::g_syscalls::trap_GetUserinfo;
pub use crate::src::game::g_syscalls::trap_SendConsoleCommand;
pub use crate::src::game::g_syscalls::trap_SendServerCommand;
pub use crate::src::game::g_syscalls::trap_SetConfigstring;
pub use crate::src::game::g_syscalls::trap_SetUserinfo;
pub use crate::src::game::g_team::OnSameTeam;

pub use crate::src::game::g_utils::vtos;
pub use crate::src::game::g_utils::G_FreeEntity;
pub use crate::src::game::g_utils::G_Spawn;

pub use ::libc::strtod;
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
==================
DeathmatchScoreboardMessage

==================
*/
#[no_mangle]

pub unsafe extern "C" fn DeathmatchScoreboardMessage(mut ent: *mut crate::g_local_h::gentity_t) {
    let mut entry: [libc::c_char; 1024] = [0; 1024];
    let mut string: [libc::c_char; 1000] = [0; 1000];
    let mut stringlength: i32 = 0;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut cl: *mut crate::g_local_h::gclient_t = 0 as *mut crate::g_local_h::gclient_t;
    let mut numSorted: i32 = 0;
    let mut scoreFlags: i32 = 0;
    let mut accuracy: i32 = 0;
    let mut perfect: i32 = 0;
    // don't send scores to bots, they don't parse it
    if (*ent).r.svFlags & 0x8 as i32 != 0 {
        return;
    }
    // send the latest information on all clients
    string[0 as i32 as usize] = 0 as i32 as libc::c_char;
    stringlength = 0 as i32;
    scoreFlags = 0 as i32;
    numSorted = crate::src::game::g_main::level.numConnectedClients;
    i = 0 as i32;
    while i < numSorted {
        let mut ping: i32 = 0;
        cl = &mut *crate::src::game::g_main::level.clients.offset(
            *crate::src::game::g_main::level
                .sortedClients
                .as_mut_ptr()
                .offset(i as isize) as isize,
        ) as *mut crate::g_local_h::gclient_s;
        if (*cl).pers.connected as u32 == crate::g_local_h::CON_CONNECTING as i32 as u32 {
            ping = -(1 as i32)
        } else {
            ping = if (*cl).ps.ping < 999 as i32 {
                (*cl).ps.ping
            } else {
                999 as i32
            }
        }
        if (*cl).accuracy_shots != 0 {
            accuracy = (*cl).accuracy_hits * 100 as i32 / (*cl).accuracy_shots
        } else {
            accuracy = 0 as i32
        }
        perfect = if (*cl).ps.persistant[crate::bg_public_h::PERS_RANK as i32 as usize] == 0 as i32
            && (*cl).ps.persistant[crate::bg_public_h::PERS_KILLED as i32 as usize] == 0 as i32
        {
            1 as i32
        } else {
            0 as i32
        };
        crate::src::qcommon::q_shared::Com_sprintf(
            entry.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as i32,
            b" %i %i %i %i %i %i %i %i %i %i %i %i %i %i\x00" as *const u8 as *const libc::c_char,
            crate::src::game::g_main::level.sortedClients[i as usize],
            (*cl).ps.persistant[crate::bg_public_h::PERS_SCORE as i32 as usize],
            ping,
            (crate::src::game::g_main::level.time - (*cl).pers.enterTime) / 60000 as i32,
            scoreFlags,
            crate::src::game::g_main::g_entities
                [crate::src::game::g_main::level.sortedClients[i as usize] as usize]
                .s
                .powerups,
            accuracy,
            (*cl).ps.persistant[crate::bg_public_h::PERS_IMPRESSIVE_COUNT as i32 as usize],
            (*cl).ps.persistant[crate::bg_public_h::PERS_EXCELLENT_COUNT as i32 as usize],
            (*cl).ps.persistant[crate::bg_public_h::PERS_GAUNTLET_FRAG_COUNT as i32 as usize],
            (*cl).ps.persistant[crate::bg_public_h::PERS_DEFEND_COUNT as i32 as usize],
            (*cl).ps.persistant[crate::bg_public_h::PERS_ASSIST_COUNT as i32 as usize],
            perfect,
            (*cl).ps.persistant[crate::bg_public_h::PERS_CAPTURES as i32 as usize],
        );
        j = crate::stdlib::strlen(entry.as_mut_ptr()) as i32;
        if (stringlength + j) as libc::c_ulong
            >= ::std::mem::size_of::<[libc::c_char; 1000]>() as libc::c_ulong
        {
            break;
        }
        ::libc::strcpy(
            string.as_mut_ptr().offset(stringlength as isize),
            entry.as_mut_ptr(),
        );
        stringlength += j;
        i += 1
    }
    crate::src::game::g_syscalls::trap_SendServerCommand(
        ent.offset_from(crate::src::game::g_main::g_entities.as_mut_ptr()) as isize as i32,
        crate::src::qcommon::q_shared::va(
            b"scores %i %i %i%s\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            i,
            crate::src::game::g_main::level.teamScores
                [crate::bg_public_h::TEAM_RED as i32 as usize],
            crate::src::game::g_main::level.teamScores
                [crate::bg_public_h::TEAM_BLUE as i32 as usize],
            string.as_mut_ptr(),
        ),
    );
}
/*
==================
Cmd_Score_f

Request current scoreboard information
==================
*/
#[no_mangle]

pub unsafe extern "C" fn Cmd_Score_f(mut ent: *mut crate::g_local_h::gentity_t) {
    DeathmatchScoreboardMessage(ent);
}
/*
==================
CheatsOk
==================
*/
#[no_mangle]

pub unsafe extern "C" fn CheatsOk(
    mut ent: *mut crate::g_local_h::gentity_t,
) -> crate::src::qcommon::q_shared::qboolean {
    if crate::src::game::g_main::g_cheats.integer == 0 {
        crate::src::game::g_syscalls::trap_SendServerCommand(
            ent.offset_from(crate::src::game::g_main::g_entities.as_mut_ptr()) as isize as i32,
            b"print \"Cheats are not enabled on this server.\n\"\x00" as *const u8
                as *const libc::c_char,
        );
        return crate::src::qcommon::q_shared::qfalse;
    }
    if (*ent).health <= 0 as i32 {
        crate::src::game::g_syscalls::trap_SendServerCommand(
            ent.offset_from(crate::src::game::g_main::g_entities.as_mut_ptr()) as isize as i32,
            b"print \"You must be alive to use this command.\n\"\x00" as *const u8
                as *const libc::c_char,
        );
        return crate::src::qcommon::q_shared::qfalse;
    }
    return crate::src::qcommon::q_shared::qtrue;
}
/*
==================
ConcatArgs
==================
*/
#[no_mangle]

pub unsafe extern "C" fn ConcatArgs(mut start: i32) -> *mut libc::c_char {
    let mut i: i32 = 0;
    let mut c: i32 = 0;
    let mut tlen: i32 = 0;
    static mut line: [libc::c_char; 1024] = [0; 1024];
    let mut len: i32 = 0;
    let mut arg: [libc::c_char; 1024] = [0; 1024];
    len = 0 as i32;
    c = crate::src::game::g_syscalls::trap_Argc();
    i = start;
    while i < c {
        crate::src::game::g_syscalls::trap_Argv(
            i,
            arg.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as i32,
        );
        tlen = crate::stdlib::strlen(arg.as_mut_ptr()) as i32;
        if len + tlen >= 1024 as i32 - 1 as i32 {
            break;
        }
        crate::stdlib::memcpy(
            line.as_mut_ptr().offset(len as isize) as *mut libc::c_void,
            arg.as_mut_ptr() as *const libc::c_void,
            tlen as libc::c_ulong,
        );
        len += tlen;
        if i != c - 1 as i32 {
            line[len as usize] = ' ' as i32 as libc::c_char;
            len += 1
        }
        i += 1
    }
    line[len as usize] = 0 as i32 as libc::c_char;
    return line.as_mut_ptr();
}
/*
==================
StringIsInteger
==================
*/
#[no_mangle]

pub unsafe extern "C" fn StringIsInteger(
    mut s: *const libc::c_char,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut i: i32 = 0;
    let mut len: i32 = 0;
    let mut foundDigit: crate::src::qcommon::q_shared::qboolean =
        crate::src::qcommon::q_shared::qfalse;
    len = crate::stdlib::strlen(s) as i32;
    foundDigit = crate::src::qcommon::q_shared::qfalse;
    i = 0 as i32;
    while i < len {
        if *(*crate::stdlib::__ctype_b_loc()).offset(*s.offset(i as isize) as i32 as isize) as i32
            & crate::stdlib::_ISdigit as i32 as u16 as i32
            == 0
        {
            return crate::src::qcommon::q_shared::qfalse;
        }
        foundDigit = crate::src::qcommon::q_shared::qtrue;
        i += 1
    }
    return foundDigit;
}
/*
==================
ClientNumberFromString

Returns a player number for either a number or name string
Returns -1 if invalid
==================
*/
#[no_mangle]

pub unsafe extern "C" fn ClientNumberFromString(
    mut to: *mut crate::g_local_h::gentity_t,
    mut s: *mut libc::c_char,
    mut checkNums: crate::src::qcommon::q_shared::qboolean,
    mut checkNames: crate::src::qcommon::q_shared::qboolean,
) -> i32 {
    let mut cl: *mut crate::g_local_h::gclient_t = 0 as *mut crate::g_local_h::gclient_t;
    let mut idnum: i32 = 0;
    let mut cleanName: [libc::c_char; 1024] = [0; 1024];
    if checkNums as u64 != 0 {
        // numeric values could be slot numbers
        if StringIsInteger(s) as u64 != 0 {
            idnum = atoi(s);
            if idnum >= 0 as i32 && idnum < crate::src::game::g_main::level.maxclients {
                cl = &mut *crate::src::game::g_main::level
                    .clients
                    .offset(idnum as isize)
                    as *mut crate::g_local_h::gclient_s;
                if (*cl).pers.connected as u32 == crate::g_local_h::CON_CONNECTED as i32 as u32 {
                    return idnum;
                }
            }
        }
    }
    if checkNames as u64 != 0 {
        // check for a name match
        idnum = 0 as i32;
        cl = crate::src::game::g_main::level.clients;
        while idnum < crate::src::game::g_main::level.maxclients {
            if !((*cl).pers.connected as u32 != crate::g_local_h::CON_CONNECTED as i32 as u32) {
                crate::src::qcommon::q_shared::Q_strncpyz(
                    cleanName.as_mut_ptr(),
                    (*cl).pers.netname.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as i32,
                );
                crate::src::qcommon::q_shared::Q_CleanStr(cleanName.as_mut_ptr());
                if crate::src::qcommon::q_shared::Q_stricmp(cleanName.as_mut_ptr(), s) == 0 {
                    return idnum;
                }
            }
            idnum += 1;
            cl = cl.offset(1)
        }
    }
    crate::src::game::g_syscalls::trap_SendServerCommand(
        to.offset_from(crate::src::game::g_main::g_entities.as_mut_ptr()) as isize as i32,
        crate::src::qcommon::q_shared::va(
            b"print \"User %s is not on the server\n\"\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            s,
        ),
    );
    return -(1 as i32);
}
/*
==================
Cmd_Give_f

Give items to a client
==================
*/
#[no_mangle]

pub unsafe extern "C" fn Cmd_Give_f(mut ent: *mut crate::g_local_h::gentity_t) {
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut it: *mut crate::bg_public_h::gitem_t = 0 as *mut crate::bg_public_h::gitem_t;
    let mut i: i32 = 0;
    let mut give_all: crate::src::qcommon::q_shared::qboolean =
        crate::src::qcommon::q_shared::qfalse;
    let mut it_ent: *mut crate::g_local_h::gentity_t = 0 as *mut crate::g_local_h::gentity_t;
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
    if CheatsOk(ent) as u64 == 0 {
        return;
    }
    name = ConcatArgs(1 as i32);
    if crate::src::qcommon::q_shared::Q_stricmp(
        name,
        b"all\x00" as *const u8 as *const libc::c_char,
    ) == 0 as i32
    {
        give_all = crate::src::qcommon::q_shared::qtrue
    } else {
        give_all = crate::src::qcommon::q_shared::qfalse
    }
    if give_all as u32 != 0
        || crate::src::qcommon::q_shared::Q_stricmp(
            name,
            b"health\x00" as *const u8 as *const libc::c_char,
        ) == 0 as i32
    {
        (*ent).health =
            (*(*ent).client).ps.stats[crate::bg_public_h::STAT_MAX_HEALTH as i32 as usize];
        if give_all as u64 == 0 {
            return;
        }
    }
    if give_all as u32 != 0
        || crate::src::qcommon::q_shared::Q_stricmp(
            name,
            b"weapons\x00" as *const u8 as *const libc::c_char,
        ) == 0 as i32
    {
        (*(*ent).client).ps.stats[crate::bg_public_h::STAT_WEAPONS as i32 as usize] = ((1 as i32)
            << crate::bg_public_h::WP_NUM_WEAPONS as i32)
            - 1 as i32
            - ((1 as i32) << crate::bg_public_h::WP_GRAPPLING_HOOK as i32)
            - ((1 as i32) << crate::bg_public_h::WP_NONE as i32);
        if give_all as u64 == 0 {
            return;
        }
    }
    if give_all as u32 != 0
        || crate::src::qcommon::q_shared::Q_stricmp(
            name,
            b"ammo\x00" as *const u8 as *const libc::c_char,
        ) == 0 as i32
    {
        i = 0 as i32;
        while i < 16 as i32 {
            (*(*ent).client).ps.ammo[i as usize] = 999 as i32;
            i += 1
        }
        if give_all as u64 == 0 {
            return;
        }
    }
    if give_all as u32 != 0
        || crate::src::qcommon::q_shared::Q_stricmp(
            name,
            b"armor\x00" as *const u8 as *const libc::c_char,
        ) == 0 as i32
    {
        (*(*ent).client).ps.stats[crate::bg_public_h::STAT_ARMOR as i32 as usize] = 200 as i32;
        if give_all as u64 == 0 {
            return;
        }
    }
    if crate::src::qcommon::q_shared::Q_stricmp(
        name,
        b"excellent\x00" as *const u8 as *const libc::c_char,
    ) == 0 as i32
    {
        (*(*ent).client).ps.persistant[crate::bg_public_h::PERS_EXCELLENT_COUNT as i32 as usize] +=
            1;
        return;
    }
    if crate::src::qcommon::q_shared::Q_stricmp(
        name,
        b"impressive\x00" as *const u8 as *const libc::c_char,
    ) == 0 as i32
    {
        (*(*ent).client).ps.persistant
            [crate::bg_public_h::PERS_IMPRESSIVE_COUNT as i32 as usize] += 1;
        return;
    }
    if crate::src::qcommon::q_shared::Q_stricmp(
        name,
        b"gauntletaward\x00" as *const u8 as *const libc::c_char,
    ) == 0 as i32
    {
        (*(*ent).client).ps.persistant
            [crate::bg_public_h::PERS_GAUNTLET_FRAG_COUNT as i32 as usize] += 1;
        return;
    }
    if crate::src::qcommon::q_shared::Q_stricmp(
        name,
        b"defend\x00" as *const u8 as *const libc::c_char,
    ) == 0 as i32
    {
        (*(*ent).client).ps.persistant[crate::bg_public_h::PERS_DEFEND_COUNT as i32 as usize] += 1;
        return;
    }
    if crate::src::qcommon::q_shared::Q_stricmp(
        name,
        b"assist\x00" as *const u8 as *const libc::c_char,
    ) == 0 as i32
    {
        (*(*ent).client).ps.persistant[crate::bg_public_h::PERS_ASSIST_COUNT as i32 as usize] += 1;
        return;
    }
    // spawn a specific item right on the player
    if give_all as u64 == 0 {
        it = crate::src::game::bg_misc::BG_FindItem(name) as *mut crate::bg_public_h::gitem_s;
        if it.is_null() {
            return;
        }
        it_ent = crate::src::game::g_utils::G_Spawn() as *mut crate::g_local_h::gentity_s;
        (*it_ent).s.origin[0 as i32 as usize] = (*ent).r.currentOrigin[0 as i32 as usize];
        (*it_ent).s.origin[1 as i32 as usize] = (*ent).r.currentOrigin[1 as i32 as usize];
        (*it_ent).s.origin[2 as i32 as usize] = (*ent).r.currentOrigin[2 as i32 as usize];
        (*it_ent).classname = (*it).classname;
        crate::src::game::g_items::G_SpawnItem(
            it_ent as *mut crate::g_local_h::gentity_s,
            it as *mut crate::bg_public_h::gitem_s,
        );
        crate::src::game::g_items::FinishSpawningItem(it_ent as *mut crate::g_local_h::gentity_s);
        crate::stdlib::memset(
            &mut trace as *mut crate::src::qcommon::q_shared::trace_t as *mut libc::c_void,
            0 as i32,
            ::std::mem::size_of::<crate::src::qcommon::q_shared::trace_t>() as libc::c_ulong,
        );
        crate::src::game::g_items::Touch_Item(
            it_ent as *mut crate::g_local_h::gentity_s,
            ent as *mut crate::g_local_h::gentity_s,
            &mut trace as *mut _ as *mut crate::src::qcommon::q_shared::trace_t,
        );
        if (*it_ent).inuse as u64 != 0 {
            crate::src::game::g_utils::G_FreeEntity(it_ent as *mut crate::g_local_h::gentity_s);
        }
    };
}
/*
==================
Cmd_God_f

Sets client to godmode

argv(0) god
==================
*/
#[no_mangle]

pub unsafe extern "C" fn Cmd_God_f(mut ent: *mut crate::g_local_h::gentity_t) {
    let mut msg: *mut libc::c_char = 0 as *mut libc::c_char;
    if CheatsOk(ent) as u64 == 0 {
        return;
    }
    (*ent).flags ^= 0x10 as i32;
    if (*ent).flags & 0x10 as i32 == 0 {
        msg = b"godmode OFF\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    } else {
        msg = b"godmode ON\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    }
    crate::src::game::g_syscalls::trap_SendServerCommand(
        ent.offset_from(crate::src::game::g_main::g_entities.as_mut_ptr()) as isize as i32,
        crate::src::qcommon::q_shared::va(
            b"print \"%s\"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            msg,
        ),
    );
}
/*
==================
Cmd_Notarget_f

Sets client to notarget

argv(0) notarget
==================
*/
#[no_mangle]

pub unsafe extern "C" fn Cmd_Notarget_f(mut ent: *mut crate::g_local_h::gentity_t) {
    let mut msg: *mut libc::c_char = 0 as *mut libc::c_char;
    if CheatsOk(ent) as u64 == 0 {
        return;
    }
    (*ent).flags ^= 0x20 as i32;
    if (*ent).flags & 0x20 as i32 == 0 {
        msg = b"notarget OFF\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    } else {
        msg = b"notarget ON\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    }
    crate::src::game::g_syscalls::trap_SendServerCommand(
        ent.offset_from(crate::src::game::g_main::g_entities.as_mut_ptr()) as isize as i32,
        crate::src::qcommon::q_shared::va(
            b"print \"%s\"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            msg,
        ),
    );
}
/*
==================
Cmd_Noclip_f

argv(0) noclip
==================
*/
#[no_mangle]

pub unsafe extern "C" fn Cmd_Noclip_f(mut ent: *mut crate::g_local_h::gentity_t) {
    let mut msg: *mut libc::c_char = 0 as *mut libc::c_char;
    if CheatsOk(ent) as u64 == 0 {
        return;
    }
    if (*(*ent).client).noclip as u64 != 0 {
        msg = b"noclip OFF\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    } else {
        msg = b"noclip ON\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    }
    (*(*ent).client).noclip =
        ((*(*ent).client).noclip as u64 == 0) as i32 as crate::src::qcommon::q_shared::qboolean;
    crate::src::game::g_syscalls::trap_SendServerCommand(
        ent.offset_from(crate::src::game::g_main::g_entities.as_mut_ptr()) as isize as i32,
        crate::src::qcommon::q_shared::va(
            b"print \"%s\"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            msg,
        ),
    );
}
/*
==================
Cmd_LevelShot_f

This is just to help generate the level pictures
for the menus.  It goes to the intermission immediately
and sends over a command to the client to resize the view,
hide the scoreboard, and take a special screenshot
==================
*/
#[no_mangle]

pub unsafe extern "C" fn Cmd_LevelShot_f(mut ent: *mut crate::g_local_h::gentity_t) {
    if (*(*ent).client).pers.localClient as u64 == 0 {
        crate::src::game::g_syscalls::trap_SendServerCommand(
            ent.offset_from(crate::src::game::g_main::g_entities.as_mut_ptr()) as isize as i32,
            b"print \"The levelshot command must be executed by a local client\n\"\x00" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    if CheatsOk(ent) as u64 == 0 {
        return;
    }
    // doesn't work in single player
    if crate::src::game::g_main::g_gametype.integer == crate::bg_public_h::GT_SINGLE_PLAYER as i32 {
        crate::src::game::g_syscalls::trap_SendServerCommand(
            ent.offset_from(crate::src::game::g_main::g_entities.as_mut_ptr()) as isize as i32,
            b"print \"Must not be in singleplayer mode for levelshot\n\"\x00" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    crate::src::game::g_main::BeginIntermission();
    crate::src::game::g_syscalls::trap_SendServerCommand(
        ent.offset_from(crate::src::game::g_main::g_entities.as_mut_ptr()) as isize as i32,
        b"clientLevelShot\x00" as *const u8 as *const libc::c_char,
    );
}
/*
==================
Cmd_TeamTask_f
==================
*/
#[no_mangle]

pub unsafe extern "C" fn Cmd_TeamTask_f(mut ent: *mut crate::g_local_h::gentity_t) {
    let mut userinfo: [libc::c_char; 1024] = [0; 1024];
    let mut arg: [libc::c_char; 1024] = [0; 1024];
    let mut task: i32 = 0;
    let mut client: i32 = (*ent)
        .client
        .offset_from(crate::src::game::g_main::level.clients) as isize
        as i32;
    if crate::src::game::g_syscalls::trap_Argc() != 2 as i32 {
        return;
    }
    crate::src::game::g_syscalls::trap_Argv(
        1 as i32,
        arg.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as i32,
    );
    task = atoi(arg.as_mut_ptr());
    crate::src::game::g_syscalls::trap_GetUserinfo(
        client,
        userinfo.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as i32,
    );
    crate::src::qcommon::q_shared::Info_SetValueForKey(
        userinfo.as_mut_ptr(),
        b"teamtask\x00" as *const u8 as *const libc::c_char,
        crate::src::qcommon::q_shared::va(
            b"%d\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            task,
        ),
    );
    crate::src::game::g_syscalls::trap_SetUserinfo(client, userinfo.as_mut_ptr());
    crate::src::game::g_client::ClientUserinfoChanged(client);
}
/*
=================
Cmd_Kill_f
=================
*/
#[no_mangle]

pub unsafe extern "C" fn Cmd_Kill_f(mut ent: *mut crate::g_local_h::gentity_t) {
    if (*(*ent).client).sess.sessionTeam as u32 == crate::bg_public_h::TEAM_SPECTATOR as i32 as u32
    {
        return;
    }
    if (*ent).health <= 0 as i32 {
        return;
    }
    (*ent).flags &= !(0x10 as i32);
    (*ent).health = -(999 as i32);
    (*(*ent).client).ps.stats[crate::bg_public_h::STAT_HEALTH as i32 as usize] = (*ent).health;
    crate::src::game::g_combat::player_die(
        ent as *mut crate::g_local_h::gentity_s,
        ent as *mut crate::g_local_h::gentity_s,
        ent as *mut crate::g_local_h::gentity_s,
        100000 as i32,
        crate::bg_public_h::MOD_SUICIDE as i32,
    );
}
/*
=================
BroadcastTeamChange

Let everyone know about a team change
=================
*/
#[no_mangle]

pub unsafe extern "C" fn BroadcastTeamChange(
    mut client: *mut crate::g_local_h::gclient_t,
    mut oldTeam: i32,
) {
    if (*client).sess.sessionTeam as u32 == crate::bg_public_h::TEAM_RED as i32 as u32 {
        crate::src::game::g_syscalls::trap_SendServerCommand(
            -(1 as i32),
            crate::src::qcommon::q_shared::va(
                b"cp \"%s^7 joined the red team.\n\"\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                (*client).pers.netname.as_mut_ptr(),
            ),
        );
    } else if (*client).sess.sessionTeam as u32 == crate::bg_public_h::TEAM_BLUE as i32 as u32 {
        crate::src::game::g_syscalls::trap_SendServerCommand(
            -(1 as i32),
            crate::src::qcommon::q_shared::va(
                b"cp \"%s^7 joined the blue team.\n\"\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                (*client).pers.netname.as_mut_ptr(),
            ),
        );
    } else if (*client).sess.sessionTeam as u32 == crate::bg_public_h::TEAM_SPECTATOR as i32 as u32
        && oldTeam != crate::bg_public_h::TEAM_SPECTATOR as i32
    {
        crate::src::game::g_syscalls::trap_SendServerCommand(
            -(1 as i32),
            crate::src::qcommon::q_shared::va(
                b"cp \"%s^7 joined the spectators.\n\"\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                (*client).pers.netname.as_mut_ptr(),
            ),
        );
    } else if (*client).sess.sessionTeam as u32 == crate::bg_public_h::TEAM_FREE as i32 as u32 {
        crate::src::game::g_syscalls::trap_SendServerCommand(
            -(1 as i32),
            crate::src::qcommon::q_shared::va(
                b"cp \"%s^7 joined the battle.\n\"\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                (*client).pers.netname.as_mut_ptr(),
            ),
        );
    };
}
/*
=================
SetTeam
=================
*/
#[no_mangle]

pub unsafe extern "C" fn SetTeam(
    mut ent: *mut crate::g_local_h::gentity_t,
    mut s: *const libc::c_char,
) {
    let mut team: i32 = 0;
    let mut oldTeam: i32 = 0;
    let mut client: *mut crate::g_local_h::gclient_t = 0 as *mut crate::g_local_h::gclient_t;
    let mut clientNum: i32 = 0;
    let mut specState: crate::g_local_h::spectatorState_t = crate::g_local_h::SPECTATOR_NOT;
    let mut specClient: i32 = 0;
    let mut teamLeader: i32 = 0;
    //
    // see what change is requested
    //
    client = (*ent).client;
    clientNum = client.offset_from(crate::src::game::g_main::level.clients) as isize as i32;
    specClient = 0 as i32;
    specState = crate::g_local_h::SPECTATOR_NOT;
    if crate::src::qcommon::q_shared::Q_stricmp(
        s,
        b"scoreboard\x00" as *const u8 as *const libc::c_char,
    ) == 0
        || crate::src::qcommon::q_shared::Q_stricmp(
            s,
            b"score\x00" as *const u8 as *const libc::c_char,
        ) == 0
    {
        team = crate::bg_public_h::TEAM_SPECTATOR as i32;
        specState = crate::g_local_h::SPECTATOR_SCOREBOARD
    } else if crate::src::qcommon::q_shared::Q_stricmp(
        s,
        b"follow1\x00" as *const u8 as *const libc::c_char,
    ) == 0
    {
        team = crate::bg_public_h::TEAM_SPECTATOR as i32;
        specState = crate::g_local_h::SPECTATOR_FOLLOW;
        specClient = -(1 as i32)
    } else if crate::src::qcommon::q_shared::Q_stricmp(
        s,
        b"follow2\x00" as *const u8 as *const libc::c_char,
    ) == 0
    {
        team = crate::bg_public_h::TEAM_SPECTATOR as i32;
        specState = crate::g_local_h::SPECTATOR_FOLLOW;
        specClient = -(2 as i32)
    } else if crate::src::qcommon::q_shared::Q_stricmp(
        s,
        b"spectator\x00" as *const u8 as *const libc::c_char,
    ) == 0
        || crate::src::qcommon::q_shared::Q_stricmp(s, b"s\x00" as *const u8 as *const libc::c_char)
            == 0
    {
        team = crate::bg_public_h::TEAM_SPECTATOR as i32;
        specState = crate::g_local_h::SPECTATOR_FREE
    } else if crate::src::game::g_main::g_gametype.integer >= crate::bg_public_h::GT_TEAM as i32 {
        // if running a team game, assign player to one of the teams
        specState = crate::g_local_h::SPECTATOR_NOT;
        if crate::src::qcommon::q_shared::Q_stricmp(
            s,
            b"red\x00" as *const u8 as *const libc::c_char,
        ) == 0
            || crate::src::qcommon::q_shared::Q_stricmp(
                s,
                b"r\x00" as *const u8 as *const libc::c_char,
            ) == 0
        {
            team = crate::bg_public_h::TEAM_RED as i32
        } else if crate::src::qcommon::q_shared::Q_stricmp(
            s,
            b"blue\x00" as *const u8 as *const libc::c_char,
        ) == 0
            || crate::src::qcommon::q_shared::Q_stricmp(
                s,
                b"b\x00" as *const u8 as *const libc::c_char,
            ) == 0
        {
            team = crate::bg_public_h::TEAM_BLUE as i32
        } else {
            // pick the team with the least number of players
            team = crate::src::game::g_client::PickTeam(clientNum) as i32
        }
        if crate::src::game::g_main::g_teamForceBalance.integer != 0
            && (*client).pers.localClient as u64 == 0
            && (*ent).r.svFlags & 0x8 as i32 == 0
        {
            let mut counts: [i32; 4] = [0; 4];
            counts[crate::bg_public_h::TEAM_BLUE as i32 as usize] =
                crate::src::game::g_client::TeamCount(clientNum, crate::bg_public_h::TEAM_BLUE);
            counts[crate::bg_public_h::TEAM_RED as i32 as usize] =
                crate::src::game::g_client::TeamCount(clientNum, crate::bg_public_h::TEAM_RED);
            // It's ok, the team we are switching to has less or same number of players
            if team == crate::bg_public_h::TEAM_RED as i32
                && counts[crate::bg_public_h::TEAM_RED as i32 as usize]
                    - counts[crate::bg_public_h::TEAM_BLUE as i32 as usize]
                    > 1 as i32
            {
                crate::src::game::g_syscalls::trap_SendServerCommand(
                    clientNum,
                    b"cp \"Red team has too many players.\n\"\x00" as *const u8
                        as *const libc::c_char,
                );
                return;
                // We allow a spread of two
                // ignore the request
            }
            if team == crate::bg_public_h::TEAM_BLUE as i32
                && counts[crate::bg_public_h::TEAM_BLUE as i32 as usize]
                    - counts[crate::bg_public_h::TEAM_RED as i32 as usize]
                    > 1 as i32
            {
                crate::src::game::g_syscalls::trap_SendServerCommand(
                    clientNum,
                    b"cp \"Blue team has too many players.\n\"\x00" as *const u8
                        as *const libc::c_char,
                );
                return;
                // ignore the request
            }
        }
    } else {
        // force them to spectators if there aren't any spots free
        team = crate::bg_public_h::TEAM_FREE as i32
    }
    // override decision if limiting the players
    if crate::src::game::g_main::g_gametype.integer == crate::bg_public_h::GT_TOURNAMENT as i32
        && crate::src::game::g_main::level.numNonSpectatorClients >= 2 as i32
    {
        team = crate::bg_public_h::TEAM_SPECTATOR as i32
    } else if crate::src::game::g_main::g_maxGameClients.integer > 0 as i32
        && crate::src::game::g_main::level.numNonSpectatorClients
            >= crate::src::game::g_main::g_maxGameClients.integer
    {
        team = crate::bg_public_h::TEAM_SPECTATOR as i32
    }
    //
    // decide if we will allow the change
    //
    oldTeam = (*client).sess.sessionTeam as i32;
    if team == oldTeam && team != crate::bg_public_h::TEAM_SPECTATOR as i32 {
        return;
    }
    //
    // execute the team change
    //
    // if the player was dead leave the body, but only if they're actually in game
    if (*client).ps.stats[crate::bg_public_h::STAT_HEALTH as i32 as usize] <= 0 as i32
        && (*client).pers.connected as u32 == crate::g_local_h::CON_CONNECTED as i32 as u32
    {
        crate::src::game::g_client::CopyToBodyQue(ent as *mut crate::g_local_h::gentity_s);
    }
    // he starts at 'base'
    (*client).pers.teamState.state = crate::g_local_h::TEAM_BEGIN;
    if oldTeam != crate::bg_public_h::TEAM_SPECTATOR as i32 {
        // Kill him (makes sure he loses flags, etc)
        (*ent).flags &= !(0x10 as i32);
        (*ent).health = 0 as i32;
        (*(*ent).client).ps.stats[crate::bg_public_h::STAT_HEALTH as i32 as usize] = (*ent).health;
        crate::src::game::g_combat::player_die(
            ent as *mut crate::g_local_h::gentity_s,
            ent as *mut crate::g_local_h::gentity_s,
            ent as *mut crate::g_local_h::gentity_s,
            100000 as i32,
            crate::bg_public_h::MOD_SUICIDE as i32,
        );
    }
    // they go to the end of the line for tournements
    if team == crate::bg_public_h::TEAM_SPECTATOR as i32 && oldTeam != team {
        crate::src::game::g_main::AddTournamentQueue(client as *mut crate::g_local_h::gclient_s);
    }
    (*client).sess.sessionTeam = team as crate::bg_public_h::team_t;
    (*client).sess.spectatorState = specState;
    (*client).sess.spectatorClient = specClient;
    (*client).sess.teamLeader = crate::src::qcommon::q_shared::qfalse;
    if team == crate::bg_public_h::TEAM_RED as i32 || team == crate::bg_public_h::TEAM_BLUE as i32 {
        teamLeader = crate::src::game::g_client::TeamLeader(team);
        // if there is no team leader or the team leader is a bot and this client is not a bot
        if teamLeader == -(1 as i32)
            || crate::src::game::g_main::g_entities[clientNum as usize]
                .r
                .svFlags
                & 0x8 as i32
                == 0
                && crate::src::game::g_main::g_entities[teamLeader as usize]
                    .r
                    .svFlags
                    & 0x8 as i32
                    != 0
        {
            crate::src::game::g_main::SetLeader(team, clientNum);
        }
    }
    // make sure there is a team leader on the team the player came from
    if oldTeam == crate::bg_public_h::TEAM_RED as i32
        || oldTeam == crate::bg_public_h::TEAM_BLUE as i32
    {
        crate::src::game::g_main::CheckTeamLeader(oldTeam);
    }
    BroadcastTeamChange(client, oldTeam);
    // get and distribute relevant parameters
    crate::src::game::g_client::ClientUserinfoChanged(clientNum);
    // client hasn't spawned yet, they sent an early team command, teampref userinfo, or g_teamAutoJoin is enabled
    if (*client).pers.connected as u32 != crate::g_local_h::CON_CONNECTED as i32 as u32 {
        return;
    }
    crate::src::game::g_client::ClientBegin(clientNum);
}
/*
=================
StopFollowing

If the client being followed leaves the game, or you just want to drop
to free floating spectator mode
=================
*/
#[no_mangle]

pub unsafe extern "C" fn StopFollowing(mut ent: *mut crate::g_local_h::gentity_t) {
    (*(*ent).client).ps.persistant[crate::bg_public_h::PERS_TEAM as i32 as usize] =
        crate::bg_public_h::TEAM_SPECTATOR as i32;
    (*(*ent).client).sess.sessionTeam = crate::bg_public_h::TEAM_SPECTATOR;
    (*(*ent).client).sess.spectatorState = crate::g_local_h::SPECTATOR_FREE;
    (*(*ent).client).ps.pm_flags &= !(4096 as i32);
    (*ent).r.svFlags &= !(0x8 as i32);
    (*(*ent).client).ps.clientNum =
        ent.offset_from(crate::src::game::g_main::g_entities.as_mut_ptr()) as isize as i32;
    crate::src::game::g_client::SetClientViewAngle(
        ent as *mut crate::g_local_h::gentity_s,
        (*(*ent).client).ps.viewangles.as_mut_ptr(),
    );
    // don't use dead view angles
    if (*(*ent).client).ps.stats[crate::bg_public_h::STAT_HEALTH as i32 as usize] <= 0 as i32 {
        (*(*ent).client).ps.stats[crate::bg_public_h::STAT_HEALTH as i32 as usize] = 1 as i32
    };
}
/*
=================
Cmd_Team_f
=================
*/
#[no_mangle]

pub unsafe extern "C" fn Cmd_Team_f(mut ent: *mut crate::g_local_h::gentity_t) {
    let mut oldTeam: i32 = 0;
    let mut s: [libc::c_char; 1024] = [0; 1024];
    if crate::src::game::g_syscalls::trap_Argc() != 2 as i32 {
        oldTeam = (*(*ent).client).sess.sessionTeam as i32;
        match oldTeam {
            2 => {
                crate::src::game::g_syscalls::trap_SendServerCommand(
                    ent.offset_from(crate::src::game::g_main::g_entities.as_mut_ptr()) as isize
                        as i32,
                    b"print \"Blue team\n\"\x00" as *const u8 as *const libc::c_char,
                );
            }
            1 => {
                crate::src::game::g_syscalls::trap_SendServerCommand(
                    ent.offset_from(crate::src::game::g_main::g_entities.as_mut_ptr()) as isize
                        as i32,
                    b"print \"Red team\n\"\x00" as *const u8 as *const libc::c_char,
                );
            }
            0 => {
                crate::src::game::g_syscalls::trap_SendServerCommand(
                    ent.offset_from(crate::src::game::g_main::g_entities.as_mut_ptr()) as isize
                        as i32,
                    b"print \"Free team\n\"\x00" as *const u8 as *const libc::c_char,
                );
            }
            3 => {
                crate::src::game::g_syscalls::trap_SendServerCommand(
                    ent.offset_from(crate::src::game::g_main::g_entities.as_mut_ptr()) as isize
                        as i32,
                    b"print \"Spectator team\n\"\x00" as *const u8 as *const libc::c_char,
                );
            }
            _ => {}
        }
        return;
    }
    if (*(*ent).client).switchTeamTime > crate::src::game::g_main::level.time {
        crate::src::game::g_syscalls::trap_SendServerCommand(
            ent.offset_from(crate::src::game::g_main::g_entities.as_mut_ptr()) as isize as i32,
            b"print \"May not switch teams more than once per 5 seconds.\n\"\x00" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    // if they are playing a tournement game, count as a loss
    if crate::src::game::g_main::g_gametype.integer == crate::bg_public_h::GT_TOURNAMENT as i32
        && (*(*ent).client).sess.sessionTeam as u32 == crate::bg_public_h::TEAM_FREE as i32 as u32
    {
        (*(*ent).client).sess.losses += 1
    }
    crate::src::game::g_syscalls::trap_Argv(
        1 as i32,
        s.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as i32,
    );
    SetTeam(ent, s.as_mut_ptr());
    (*(*ent).client).switchTeamTime = crate::src::game::g_main::level.time + 5000 as i32;
}
/*
=================
Cmd_Follow_f
=================
*/
#[no_mangle]

pub unsafe extern "C" fn Cmd_Follow_f(mut ent: *mut crate::g_local_h::gentity_t) {
    let mut i: i32 = 0;
    let mut arg: [libc::c_char; 1024] = [0; 1024];
    if crate::src::game::g_syscalls::trap_Argc() != 2 as i32 {
        if (*(*ent).client).sess.spectatorState as u32
            == crate::g_local_h::SPECTATOR_FOLLOW as i32 as u32
        {
            StopFollowing(ent);
        }
        return;
    }
    crate::src::game::g_syscalls::trap_Argv(
        1 as i32,
        arg.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as i32,
    );
    i = ClientNumberFromString(
        ent,
        arg.as_mut_ptr(),
        crate::src::qcommon::q_shared::qtrue,
        crate::src::qcommon::q_shared::qtrue,
    );
    if i == -(1 as i32) {
        return;
    }
    // can't follow self
    if &mut *crate::src::game::g_main::level.clients.offset(i as isize)
        as *mut crate::g_local_h::gclient_s
        == (*ent).client
    {
        return;
    }
    // can't follow another spectator
    if (*crate::src::game::g_main::level.clients.offset(i as isize))
        .sess
        .sessionTeam as u32
        == crate::bg_public_h::TEAM_SPECTATOR as i32 as u32
    {
        return;
    }
    // if they are playing a tournement game, count as a loss
    if crate::src::game::g_main::g_gametype.integer == crate::bg_public_h::GT_TOURNAMENT as i32
        && (*(*ent).client).sess.sessionTeam as u32 == crate::bg_public_h::TEAM_FREE as i32 as u32
    {
        (*(*ent).client).sess.losses += 1
    }
    // first set them to spectator
    if (*(*ent).client).sess.sessionTeam as u32 != crate::bg_public_h::TEAM_SPECTATOR as i32 as u32
    {
        SetTeam(ent, b"spectator\x00" as *const u8 as *const libc::c_char);
    }
    (*(*ent).client).sess.spectatorState = crate::g_local_h::SPECTATOR_FOLLOW;
    (*(*ent).client).sess.spectatorClient = i;
}
/*
=================
Cmd_FollowCycle_f
=================
*/
#[no_mangle]

pub unsafe extern "C" fn Cmd_FollowCycle_f(
    mut ent: *mut crate::g_local_h::gentity_t,
    mut dir: i32,
) {
    let mut clientnum: i32 = 0;
    let mut original: i32 = 0;
    // if they are playing a tournement game, count as a loss
    if crate::src::game::g_main::g_gametype.integer == crate::bg_public_h::GT_TOURNAMENT as i32
        && (*(*ent).client).sess.sessionTeam as u32 == crate::bg_public_h::TEAM_FREE as i32 as u32
    {
        (*(*ent).client).sess.losses += 1
    }
    // first set them to spectator
    if (*(*ent).client).sess.spectatorState as u32 == crate::g_local_h::SPECTATOR_NOT as i32 as u32
    {
        SetTeam(ent, b"spectator\x00" as *const u8 as *const libc::c_char);
    }
    if dir != 1 as i32 && dir != -(1 as i32) {
        crate::src::game::g_main::G_Error(
            b"Cmd_FollowCycle_f: bad dir %i\x00" as *const u8 as *const libc::c_char,
            dir,
        );
    }
    // if dedicated follow client, just switch between the two auto clients
    if (*(*ent).client).sess.spectatorClient < 0 as i32 {
        if (*(*ent).client).sess.spectatorClient == -(1 as i32) {
            (*(*ent).client).sess.spectatorClient = -(2 as i32)
        } else if (*(*ent).client).sess.spectatorClient == -(2 as i32) {
            (*(*ent).client).sess.spectatorClient = -(1 as i32)
        }
        return;
    }
    clientnum = (*(*ent).client).sess.spectatorClient;
    original = clientnum;
    loop {
        clientnum += dir;
        if clientnum >= crate::src::game::g_main::level.maxclients {
            clientnum = 0 as i32
        }
        if clientnum < 0 as i32 {
            clientnum = crate::src::game::g_main::level.maxclients - 1 as i32
        }
        // can only follow connected clients
        if !((*crate::src::game::g_main::level
            .clients
            .offset(clientnum as isize))
        .pers
        .connected as u32
            != crate::g_local_h::CON_CONNECTED as i32 as u32)
        {
            // can't follow another spectator
            if !((*crate::src::game::g_main::level
                .clients
                .offset(clientnum as isize))
            .sess
            .sessionTeam as u32
                == crate::bg_public_h::TEAM_SPECTATOR as i32 as u32)
            {
                // this is good, we can use it
                (*(*ent).client).sess.spectatorClient = clientnum;
                (*(*ent).client).sess.spectatorState = crate::g_local_h::SPECTATOR_FOLLOW;
                return;
            }
        }
        if !(clientnum != original) {
            break;
        }
    }
    // leave it where it was
}
/*
==================
G_Say
==================
*/

unsafe extern "C" fn G_SayTo(
    mut ent: *mut crate::g_local_h::gentity_t,
    mut other: *mut crate::g_local_h::gentity_t,
    mut mode: i32,
    mut color: i32,
    mut name: *const libc::c_char,
    mut message: *const libc::c_char,
) {
    if other.is_null() {
        return;
    }
    if (*other).inuse as u64 == 0 {
        return;
    }
    if (*other).client.is_null() {
        return;
    }
    if (*(*other).client).pers.connected as u32 != crate::g_local_h::CON_CONNECTED as i32 as u32 {
        return;
    }
    if mode == 1 as i32
        && crate::src::game::g_team::OnSameTeam(
            ent as *mut crate::g_local_h::gentity_s,
            other as *mut crate::g_local_h::gentity_s,
        ) as u64
            == 0
    {
        return;
    }
    // no chatting to players in tournements
    if crate::src::game::g_main::g_gametype.integer == crate::bg_public_h::GT_TOURNAMENT as i32
        && (*(*other).client).sess.sessionTeam as u32 == crate::bg_public_h::TEAM_FREE as i32 as u32
        && (*(*ent).client).sess.sessionTeam as u32 != crate::bg_public_h::TEAM_FREE as i32 as u32
    {
        return;
    }
    crate::src::game::g_syscalls::trap_SendServerCommand(
        other.offset_from(crate::src::game::g_main::g_entities.as_mut_ptr()) as isize as i32,
        crate::src::qcommon::q_shared::va(
            b"%s \"%s%c%c%s\"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            if mode == 1 as i32 {
                b"tchat\x00" as *const u8 as *const libc::c_char
            } else {
                b"chat\x00" as *const u8 as *const libc::c_char
            },
            name,
            '^' as i32,
            color,
            message,
        ),
    );
}
#[no_mangle]

pub unsafe extern "C" fn G_Say(
    mut ent: *mut crate::g_local_h::gentity_t,
    mut target: *mut crate::g_local_h::gentity_t,
    mut mode: i32,
    mut chatText: *const libc::c_char,
) {
    let mut j: i32 = 0;
    let mut other: *mut crate::g_local_h::gentity_t = 0 as *mut crate::g_local_h::gentity_t;
    let mut color: i32 = 0;
    let mut name: [libc::c_char; 64] = [0; 64];
    // don't let text be too long for malicious reasons
    let mut text: [libc::c_char; 150] = [0; 150];
    let mut location: [libc::c_char; 64] = [0; 64];
    if crate::src::game::g_main::g_gametype.integer < crate::bg_public_h::GT_TEAM as i32
        && mode == 1 as i32
    {
        mode = 0 as i32
    }
    match mode {
        1 => {
            crate::src::game::g_main::G_LogPrintf(
                b"sayteam: %s: %s\n\x00" as *const u8 as *const libc::c_char,
                (*(*ent).client).pers.netname.as_mut_ptr(),
                chatText,
            );
            if crate::src::game::g_team::Team_GetLocationMsg(
                ent as *mut crate::g_local_h::gentity_s,
                location.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as i32,
            ) as u64
                != 0
            {
                crate::src::qcommon::q_shared::Com_sprintf(
                    name.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as i32,
                    b"\x19(%s%c%c\x19) (%s)\x19: \x00" as *const u8 as *const libc::c_char,
                    (*(*ent).client).pers.netname.as_mut_ptr(),
                    '^' as i32,
                    '7' as i32,
                    location.as_mut_ptr(),
                );
            } else {
                crate::src::qcommon::q_shared::Com_sprintf(
                    name.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as i32,
                    b"\x19(%s%c%c\x19)\x19: \x00" as *const u8 as *const libc::c_char,
                    (*(*ent).client).pers.netname.as_mut_ptr(),
                    '^' as i32,
                    '7' as i32,
                );
            }
            color = '5' as i32
        }
        2 => {
            if !target.is_null()
                && (*target).inuse as u32 != 0
                && !(*target).client.is_null()
                && crate::src::game::g_main::g_gametype.integer
                    >= crate::bg_public_h::GT_TEAM as i32
                && (*(*target).client).sess.sessionTeam as u32
                    == (*(*ent).client).sess.sessionTeam as u32
                && crate::src::game::g_team::Team_GetLocationMsg(
                    ent as *mut crate::g_local_h::gentity_s,
                    location.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as i32,
                ) as u32
                    != 0
            {
                crate::src::qcommon::q_shared::Com_sprintf(
                    name.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as i32,
                    b"\x19[%s%c%c\x19] (%s)\x19: \x00" as *const u8 as *const libc::c_char,
                    (*(*ent).client).pers.netname.as_mut_ptr(),
                    '^' as i32,
                    '7' as i32,
                    location.as_mut_ptr(),
                );
            } else {
                crate::src::qcommon::q_shared::Com_sprintf(
                    name.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as i32,
                    b"\x19[%s%c%c\x19]\x19: \x00" as *const u8 as *const libc::c_char,
                    (*(*ent).client).pers.netname.as_mut_ptr(),
                    '^' as i32,
                    '7' as i32,
                );
            }
            color = '6' as i32
        }
        0 | _ => {
            crate::src::game::g_main::G_LogPrintf(
                b"say: %s: %s\n\x00" as *const u8 as *const libc::c_char,
                (*(*ent).client).pers.netname.as_mut_ptr(),
                chatText,
            );
            crate::src::qcommon::q_shared::Com_sprintf(
                name.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as i32,
                b"%s%c%c\x19: \x00" as *const u8 as *const libc::c_char,
                (*(*ent).client).pers.netname.as_mut_ptr(),
                '^' as i32,
                '7' as i32,
            );
            color = '2' as i32
        }
    }
    crate::src::qcommon::q_shared::Q_strncpyz(
        text.as_mut_ptr(),
        chatText,
        ::std::mem::size_of::<[libc::c_char; 150]>() as libc::c_ulong as i32,
    );
    if !target.is_null() {
        G_SayTo(
            ent,
            target,
            mode,
            color,
            name.as_mut_ptr(),
            text.as_mut_ptr(),
        );
        return;
    }
    // echo the text to the console
    if crate::src::game::g_main::g_dedicated.integer != 0 {
        crate::src::game::g_main::G_Printf(
            b"%s%s\n\x00" as *const u8 as *const libc::c_char,
            name.as_mut_ptr(),
            text.as_mut_ptr(),
        );
    }
    // send it to all the appropriate clients
    j = 0 as i32;
    while j < crate::src::game::g_main::level.maxclients {
        other = &mut *crate::src::game::g_main::g_entities
            .as_mut_ptr()
            .offset(j as isize) as *mut crate::g_local_h::gentity_t;
        G_SayTo(
            ent,
            other,
            mode,
            color,
            name.as_mut_ptr(),
            text.as_mut_ptr(),
        );
        j += 1
    }
}

unsafe extern "C" fn SanitizeChatText(mut text: *mut libc::c_char) {
    let mut i: i32 = 0;
    i = 0 as i32;
    while *text.offset(i as isize) != 0 {
        if *text.offset(i as isize) as i32 == '\n' as i32
            || *text.offset(i as isize) as i32 == '\r' as i32
        {
            *text.offset(i as isize) = ' ' as i32 as libc::c_char
        }
        i += 1
    }
}
/*
==================
Cmd_Say_f
==================
*/

unsafe extern "C" fn Cmd_Say_f(
    mut ent: *mut crate::g_local_h::gentity_t,
    mut mode: i32,
    mut arg0: crate::src::qcommon::q_shared::qboolean,
) {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    if crate::src::game::g_syscalls::trap_Argc() < 2 as i32 && arg0 as u64 == 0 {
        return;
    }
    if arg0 as u64 != 0 {
        p = ConcatArgs(0 as i32)
    } else {
        p = ConcatArgs(1 as i32)
    }
    SanitizeChatText(p);
    G_Say(ent, 0 as *mut crate::g_local_h::gentity_t, mode, p);
}
/*
==================
Cmd_Tell_f
==================
*/

unsafe extern "C" fn Cmd_Tell_f(mut ent: *mut crate::g_local_h::gentity_t) {
    let mut targetNum: i32 = 0;
    let mut target: *mut crate::g_local_h::gentity_t = 0 as *mut crate::g_local_h::gentity_t;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut arg: [libc::c_char; 1024] = [0; 1024];
    if crate::src::game::g_syscalls::trap_Argc() < 3 as i32 {
        crate::src::game::g_syscalls::trap_SendServerCommand(
            ent.offset_from(crate::src::game::g_main::g_entities.as_mut_ptr()) as isize as i32,
            b"print \"Usage: tell <player id> <message>\n\"\x00" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    crate::src::game::g_syscalls::trap_Argv(
        1 as i32,
        arg.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as i32,
    );
    targetNum = ClientNumberFromString(
        ent,
        arg.as_mut_ptr(),
        crate::src::qcommon::q_shared::qtrue,
        crate::src::qcommon::q_shared::qtrue,
    );
    if targetNum == -(1 as i32) {
        return;
    }
    target = &mut *crate::src::game::g_main::g_entities
        .as_mut_ptr()
        .offset(targetNum as isize) as *mut crate::g_local_h::gentity_t;
    if (*target).inuse as u64 == 0 || (*target).client.is_null() {
        return;
    }
    p = ConcatArgs(2 as i32);
    SanitizeChatText(p);
    crate::src::game::g_main::G_LogPrintf(
        b"tell: %s to %s: %s\n\x00" as *const u8 as *const libc::c_char,
        (*(*ent).client).pers.netname.as_mut_ptr(),
        (*(*target).client).pers.netname.as_mut_ptr(),
        p,
    );
    G_Say(ent, target, 2 as i32, p);
    // don't tell to the player self if it was already directed to this player
    // also don't send the chat back to a bot
    if ent != target && (*ent).r.svFlags & 0x8 as i32 == 0 {
        G_Say(ent, ent, 2 as i32, p);
    };
}

static mut gc_orders: [*mut libc::c_char; 7] = [
    b"hold your position\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"hold this position\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"come here\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"cover me\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"guard location\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"search and destroy\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"report\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
];
// Initialized in run_static_initializers

static mut numgc_orders: i32 = 0;
#[no_mangle]

pub unsafe extern "C" fn Cmd_GameCommand_f(mut ent: *mut crate::g_local_h::gentity_t) {
    let mut targetNum: i32 = 0;
    let mut target: *mut crate::g_local_h::gentity_t = 0 as *mut crate::g_local_h::gentity_t;
    let mut order: i32 = 0;
    let mut arg: [libc::c_char; 1024] = [0; 1024];
    if crate::src::game::g_syscalls::trap_Argc() != 3 as i32 {
        crate::src::game::g_syscalls::trap_SendServerCommand(
            ent.offset_from(crate::src::game::g_main::g_entities.as_mut_ptr()) as isize as i32,
            crate::src::qcommon::q_shared::va(
                b"print \"Usage: gc <player id> <order 0-%d>\n\"\x00" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                numgc_orders - 1 as i32,
            ),
        );
        return;
    }
    crate::src::game::g_syscalls::trap_Argv(
        2 as i32,
        arg.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as i32,
    );
    order = atoi(arg.as_mut_ptr());
    if order < 0 as i32 || order >= numgc_orders {
        crate::src::game::g_syscalls::trap_SendServerCommand(
            ent.offset_from(crate::src::game::g_main::g_entities.as_mut_ptr()) as isize as i32,
            crate::src::qcommon::q_shared::va(
                b"print \"Bad order: %i\n\"\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                order,
            ),
        );
        return;
    }
    crate::src::game::g_syscalls::trap_Argv(
        1 as i32,
        arg.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as i32,
    );
    targetNum = ClientNumberFromString(
        ent,
        arg.as_mut_ptr(),
        crate::src::qcommon::q_shared::qtrue,
        crate::src::qcommon::q_shared::qtrue,
    );
    if targetNum == -(1 as i32) {
        return;
    }
    target = &mut *crate::src::game::g_main::g_entities
        .as_mut_ptr()
        .offset(targetNum as isize) as *mut crate::g_local_h::gentity_t;
    if (*target).inuse as u64 == 0 || (*target).client.is_null() {
        return;
    }
    crate::src::game::g_main::G_LogPrintf(
        b"tell: %s to %s: %s\n\x00" as *const u8 as *const libc::c_char,
        (*(*ent).client).pers.netname.as_mut_ptr(),
        (*(*target).client).pers.netname.as_mut_ptr(),
        gc_orders[order as usize],
    );
    G_Say(ent, target, 2 as i32, gc_orders[order as usize]);
    // don't tell to the player self if it was already directed to this player
    // also don't send the chat back to a bot
    if ent != target && (*ent).r.svFlags & 0x8 as i32 == 0 {
        G_Say(ent, ent, 2 as i32, gc_orders[order as usize]);
    };
}
/*
==================
Cmd_Where_f
==================
*/
#[no_mangle]

pub unsafe extern "C" fn Cmd_Where_f(mut ent: *mut crate::g_local_h::gentity_t) {
    crate::src::game::g_syscalls::trap_SendServerCommand(
        ent.offset_from(crate::src::game::g_main::g_entities.as_mut_ptr()) as isize as i32,
        crate::src::qcommon::q_shared::va(
            b"print \"%s\n\"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            crate::src::game::g_utils::vtos(
                (*ent).r.currentOrigin.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t
            ),
        ),
    );
}

static mut gameNames: [*const libc::c_char; 8] = [
    b"Free For All\x00" as *const u8 as *const libc::c_char,
    b"Tournament\x00" as *const u8 as *const libc::c_char,
    b"Single Player\x00" as *const u8 as *const libc::c_char,
    b"Team Deathmatch\x00" as *const u8 as *const libc::c_char,
    b"Capture the Flag\x00" as *const u8 as *const libc::c_char,
    b"One Flag CTF\x00" as *const u8 as *const libc::c_char,
    b"Overload\x00" as *const u8 as *const libc::c_char,
    b"Harvester\x00" as *const u8 as *const libc::c_char,
];
/*
==================
Cmd_CallVote_f
==================
*/
#[no_mangle]

pub unsafe extern "C" fn Cmd_CallVote_f(mut ent: *mut crate::g_local_h::gentity_t) {
    let mut c: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: i32 = 0;
    let mut arg1: [libc::c_char; 1024] = [0; 1024];
    let mut arg2: [libc::c_char; 1024] = [0; 1024];
    if crate::src::game::g_main::g_allowVote.integer == 0 {
        crate::src::game::g_syscalls::trap_SendServerCommand(
            ent.offset_from(crate::src::game::g_main::g_entities.as_mut_ptr()) as isize as i32,
            b"print \"Voting not allowed here.\n\"\x00" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if crate::src::game::g_main::level.voteTime != 0 {
        crate::src::game::g_syscalls::trap_SendServerCommand(
            ent.offset_from(crate::src::game::g_main::g_entities.as_mut_ptr()) as isize as i32,
            b"print \"A vote is already in progress.\n\"\x00" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if (*(*ent).client).pers.voteCount >= 3 as i32 {
        crate::src::game::g_syscalls::trap_SendServerCommand(
            ent.offset_from(crate::src::game::g_main::g_entities.as_mut_ptr()) as isize as i32,
            b"print \"You have called the maximum number of votes.\n\"\x00" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    if (*(*ent).client).sess.sessionTeam as u32 == crate::bg_public_h::TEAM_SPECTATOR as i32 as u32
    {
        crate::src::game::g_syscalls::trap_SendServerCommand(
            ent.offset_from(crate::src::game::g_main::g_entities.as_mut_ptr()) as isize as i32,
            b"print \"Not allowed to call a vote as spectator.\n\"\x00" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    // make sure it is a valid command to vote on
    crate::src::game::g_syscalls::trap_Argv(
        1 as i32,
        arg1.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as i32,
    );
    crate::src::game::g_syscalls::trap_Argv(
        2 as i32,
        arg2.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as i32,
    );
    // check for command separators in arg2
    c = arg2.as_mut_ptr();
    while *c != 0 {
        match *c as i32 {
            10 | 13 | 59 => {
                crate::src::game::g_syscalls::trap_SendServerCommand(
                    ent.offset_from(crate::src::game::g_main::g_entities.as_mut_ptr()) as isize
                        as i32,
                    b"print \"Invalid vote string.\n\"\x00" as *const u8 as *const libc::c_char,
                );
                return;
            }
            _ => {}
        }
        c = c.offset(1)
    }
    if !(crate::src::qcommon::q_shared::Q_stricmp(
        arg1.as_mut_ptr(),
        b"map_restart\x00" as *const u8 as *const libc::c_char,
    ) == 0)
    {
        if !(crate::src::qcommon::q_shared::Q_stricmp(
            arg1.as_mut_ptr(),
            b"nextmap\x00" as *const u8 as *const libc::c_char,
        ) == 0)
        {
            if !(crate::src::qcommon::q_shared::Q_stricmp(
                arg1.as_mut_ptr(),
                b"map\x00" as *const u8 as *const libc::c_char,
            ) == 0)
            {
                if !(crate::src::qcommon::q_shared::Q_stricmp(
                    arg1.as_mut_ptr(),
                    b"g_gametype\x00" as *const u8 as *const libc::c_char,
                ) == 0)
                {
                    if !(crate::src::qcommon::q_shared::Q_stricmp(
                        arg1.as_mut_ptr(),
                        b"kick\x00" as *const u8 as *const libc::c_char,
                    ) == 0)
                    {
                        if !(crate::src::qcommon::q_shared::Q_stricmp(
                            arg1.as_mut_ptr(),
                            b"clientkick\x00" as *const u8 as *const libc::c_char,
                        ) == 0)
                        {
                            if !(crate::src::qcommon::q_shared::Q_stricmp(
                                arg1.as_mut_ptr(),
                                b"g_doWarmup\x00" as *const u8 as *const libc::c_char,
                            ) == 0)
                            {
                                if !(crate::src::qcommon::q_shared::Q_stricmp(
                                    arg1.as_mut_ptr(),
                                    b"timelimit\x00" as *const u8 as *const libc::c_char,
                                ) == 0)
                                {
                                    if crate::src::qcommon::q_shared::Q_stricmp(
                                        arg1.as_mut_ptr(),
                                        b"fraglimit\x00" as *const u8 as *const libc::c_char,
                                    ) == 0
                                    {
                                    } else {
                                        crate::src::game::g_syscalls::trap_SendServerCommand(
                                            ent.offset_from(
                                                crate::src::game::g_main::g_entities.as_mut_ptr(),
                                            ) as isize
                                                as i32,
                                            b"print \"Invalid vote string.\n\"\x00" as *const u8
                                                as *const libc::c_char,
                                        );
                                        crate::src::game::g_syscalls::trap_SendServerCommand(ent.offset_from(crate::src::game::g_main::g_entities.as_mut_ptr())
                                                                   as
                                                                   isize
                                                                   as
                                                                   i32,
                                                               b"print \"Vote commands are: map_restart, nextmap, map <mapname>, g_gametype <n>, kick <player>, clientkick <clientnum>, g_doWarmup, timelimit <time>, fraglimit <frags>.\n\"\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char);
                                        return;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    // if there is still a vote to be executed
    if crate::src::game::g_main::level.voteExecuteTime != 0 {
        // don't start a vote when map change or restart is in progress
        if crate::src::qcommon::q_shared::Q_stricmpn(
            crate::src::game::g_main::level.voteString.as_mut_ptr(),
            b"map\x00" as *const u8 as *const libc::c_char,
            3 as i32,
        ) == 0
            || crate::src::qcommon::q_shared::Q_stricmpn(
                crate::src::game::g_main::level.voteString.as_mut_ptr(),
                b"nextmap\x00" as *const u8 as *const libc::c_char,
                7 as i32,
            ) == 0
        {
            crate::src::game::g_syscalls::trap_SendServerCommand(
                ent.offset_from(crate::src::game::g_main::g_entities.as_mut_ptr()) as isize as i32,
                b"print \"Vote after map change.\n\"\x00" as *const u8 as *const libc::c_char,
            );
            return;
        }
        crate::src::game::g_main::level.voteExecuteTime = 0 as i32;
        crate::src::game::g_syscalls::trap_SendConsoleCommand(
            crate::src::qcommon::q_shared::EXEC_APPEND as i32,
            crate::src::qcommon::q_shared::va(
                b"%s\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                crate::src::game::g_main::level.voteString.as_mut_ptr(),
            ),
        );
    }
    // special case for g_gametype, check for bad values
    if crate::src::qcommon::q_shared::Q_stricmp(
        arg1.as_mut_ptr(),
        b"g_gametype\x00" as *const u8 as *const libc::c_char,
    ) == 0
    {
        i = atoi(arg2.as_mut_ptr());
        if i == crate::bg_public_h::GT_SINGLE_PLAYER as i32
            || i < crate::bg_public_h::GT_FFA as i32
            || i >= crate::bg_public_h::GT_MAX_GAME_TYPE as i32
        {
            crate::src::game::g_syscalls::trap_SendServerCommand(
                ent.offset_from(crate::src::game::g_main::g_entities.as_mut_ptr()) as isize as i32,
                b"print \"Invalid gametype.\n\"\x00" as *const u8 as *const libc::c_char,
            );
            return;
        }
        crate::src::qcommon::q_shared::Com_sprintf(
            crate::src::game::g_main::level.voteString.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as i32,
            b"%s %d\x00" as *const u8 as *const libc::c_char,
            arg1.as_mut_ptr(),
            i,
        );
        crate::src::qcommon::q_shared::Com_sprintf(
            crate::src::game::g_main::level
                .voteDisplayString
                .as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as i32,
            b"%s %s\x00" as *const u8 as *const libc::c_char,
            arg1.as_mut_ptr(),
            gameNames[i as usize],
        );
    } else if crate::src::qcommon::q_shared::Q_stricmp(
        arg1.as_mut_ptr(),
        b"map\x00" as *const u8 as *const libc::c_char,
    ) == 0
    {
        // special case for map changes, we want to reset the nextmap setting
        // this allows a player to change maps, but not upset the map rotation
        let mut s: [libc::c_char; 1024] = [0; 1024];
        crate::src::game::g_syscalls::trap_Cvar_VariableStringBuffer(
            b"nextmap\x00" as *const u8 as *const libc::c_char,
            s.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as i32,
        );
        if *s.as_mut_ptr() != 0 {
            crate::src::qcommon::q_shared::Com_sprintf(
                crate::src::game::g_main::level.voteString.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as i32,
                b"%s %s; set nextmap \"%s\"\x00" as *const u8 as *const libc::c_char,
                arg1.as_mut_ptr(),
                arg2.as_mut_ptr(),
                s.as_mut_ptr(),
            );
        } else {
            crate::src::qcommon::q_shared::Com_sprintf(
                crate::src::game::g_main::level.voteString.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as i32,
                b"%s %s\x00" as *const u8 as *const libc::c_char,
                arg1.as_mut_ptr(),
                arg2.as_mut_ptr(),
            );
        }
        crate::src::qcommon::q_shared::Com_sprintf(
            crate::src::game::g_main::level
                .voteDisplayString
                .as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as i32,
            b"%s\x00" as *const u8 as *const libc::c_char,
            crate::src::game::g_main::level.voteString.as_mut_ptr(),
        );
    } else if crate::src::qcommon::q_shared::Q_stricmp(
        arg1.as_mut_ptr(),
        b"nextmap\x00" as *const u8 as *const libc::c_char,
    ) == 0
    {
        let mut s_0: [libc::c_char; 1024] = [0; 1024];
        crate::src::game::g_syscalls::trap_Cvar_VariableStringBuffer(
            b"nextmap\x00" as *const u8 as *const libc::c_char,
            s_0.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as i32,
        );
        if *s_0.as_mut_ptr() == 0 {
            crate::src::game::g_syscalls::trap_SendServerCommand(
                ent.offset_from(crate::src::game::g_main::g_entities.as_mut_ptr()) as isize as i32,
                b"print \"nextmap not set.\n\"\x00" as *const u8 as *const libc::c_char,
            );
            return;
        }
        crate::src::qcommon::q_shared::Com_sprintf(
            crate::src::game::g_main::level.voteString.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as i32,
            b"vstr nextmap\x00" as *const u8 as *const libc::c_char,
        );
        crate::src::qcommon::q_shared::Com_sprintf(
            crate::src::game::g_main::level
                .voteDisplayString
                .as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as i32,
            b"%s\x00" as *const u8 as *const libc::c_char,
            crate::src::game::g_main::level.voteString.as_mut_ptr(),
        );
    } else if crate::src::qcommon::q_shared::Q_stricmp(
        arg1.as_mut_ptr(),
        b"clientkick\x00" as *const u8 as *const libc::c_char,
    ) == 0
        || crate::src::qcommon::q_shared::Q_stricmp(
            arg1.as_mut_ptr(),
            b"kick\x00" as *const u8 as *const libc::c_char,
        ) == 0
    {
        i = ClientNumberFromString(
            ent,
            arg2.as_mut_ptr(),
            (crate::src::qcommon::q_shared::Q_stricmp(
                arg1.as_mut_ptr(),
                b"clientkick\x00" as *const u8 as *const libc::c_char,
            ) == 0) as i32 as crate::src::qcommon::q_shared::qboolean,
            (crate::src::qcommon::q_shared::Q_stricmp(
                arg1.as_mut_ptr(),
                b"kick\x00" as *const u8 as *const libc::c_char,
            ) == 0) as i32 as crate::src::qcommon::q_shared::qboolean,
        );
        if i == -(1 as i32) {
            return;
        }
        if (*crate::src::game::g_main::level.clients.offset(i as isize))
            .pers
            .localClient as u64
            != 0
        {
            crate::src::game::g_syscalls::trap_SendServerCommand(
                ent.offset_from(crate::src::game::g_main::g_entities.as_mut_ptr()) as isize as i32,
                b"print \"Cannot kick host player.\n\"\x00" as *const u8 as *const libc::c_char,
            );
            return;
        }
        crate::src::qcommon::q_shared::Com_sprintf(
            crate::src::game::g_main::level.voteString.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as i32,
            b"clientkick %d\x00" as *const u8 as *const libc::c_char,
            i,
        );
        crate::src::qcommon::q_shared::Com_sprintf(
            crate::src::game::g_main::level
                .voteDisplayString
                .as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as i32,
            b"kick %s\x00" as *const u8 as *const libc::c_char,
            (*crate::src::game::g_main::level.clients.offset(i as isize))
                .pers
                .netname
                .as_mut_ptr(),
        );
    } else {
        crate::src::qcommon::q_shared::Com_sprintf(
            crate::src::game::g_main::level.voteString.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as i32,
            b"%s \"%s\"\x00" as *const u8 as *const libc::c_char,
            arg1.as_mut_ptr(),
            arg2.as_mut_ptr(),
        );
        crate::src::qcommon::q_shared::Com_sprintf(
            crate::src::game::g_main::level
                .voteDisplayString
                .as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as i32,
            b"%s\x00" as *const u8 as *const libc::c_char,
            crate::src::game::g_main::level.voteString.as_mut_ptr(),
        );
    }
    crate::src::game::g_syscalls::trap_SendServerCommand(
        -(1 as i32),
        crate::src::qcommon::q_shared::va(
            b"print \"%s called a vote.\n\"\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            (*(*ent).client).pers.netname.as_mut_ptr(),
        ),
    );
    // start the voting, the caller automatically votes yes
    crate::src::game::g_main::level.voteTime = crate::src::game::g_main::level.time;
    crate::src::game::g_main::level.voteYes = 1 as i32;
    crate::src::game::g_main::level.voteNo = 0 as i32;
    i = 0 as i32;
    while i < crate::src::game::g_main::level.maxclients {
        (*crate::src::game::g_main::level.clients.offset(i as isize))
            .ps
            .eFlags &= !(0x4000 as i32);
        i += 1
    }
    (*(*ent).client).ps.eFlags |= 0x4000 as i32;
    crate::src::game::g_syscalls::trap_SetConfigstring(
        8 as i32,
        crate::src::qcommon::q_shared::va(
            b"%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            crate::src::game::g_main::level.voteTime,
        ),
    );
    crate::src::game::g_syscalls::trap_SetConfigstring(
        9 as i32,
        crate::src::game::g_main::level
            .voteDisplayString
            .as_mut_ptr(),
    );
    crate::src::game::g_syscalls::trap_SetConfigstring(
        10 as i32,
        crate::src::qcommon::q_shared::va(
            b"%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            crate::src::game::g_main::level.voteYes,
        ),
    );
    crate::src::game::g_syscalls::trap_SetConfigstring(
        11 as i32,
        crate::src::qcommon::q_shared::va(
            b"%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            crate::src::game::g_main::level.voteNo,
        ),
    );
}
/*
==================
Cmd_Vote_f
==================
*/
#[no_mangle]

pub unsafe extern "C" fn Cmd_Vote_f(mut ent: *mut crate::g_local_h::gentity_t) {
    let mut msg: [libc::c_char; 64] = [0; 64];
    if crate::src::game::g_main::level.voteTime == 0 {
        crate::src::game::g_syscalls::trap_SendServerCommand(
            ent.offset_from(crate::src::game::g_main::g_entities.as_mut_ptr()) as isize as i32,
            b"print \"No vote in progress.\n\"\x00" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if (*(*ent).client).ps.eFlags & 0x4000 as i32 != 0 {
        crate::src::game::g_syscalls::trap_SendServerCommand(
            ent.offset_from(crate::src::game::g_main::g_entities.as_mut_ptr()) as isize as i32,
            b"print \"Vote already cast.\n\"\x00" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if (*(*ent).client).sess.sessionTeam as u32 == crate::bg_public_h::TEAM_SPECTATOR as i32 as u32
    {
        crate::src::game::g_syscalls::trap_SendServerCommand(
            ent.offset_from(crate::src::game::g_main::g_entities.as_mut_ptr()) as isize as i32,
            b"print \"Not allowed to vote as spectator.\n\"\x00" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    crate::src::game::g_syscalls::trap_SendServerCommand(
        ent.offset_from(crate::src::game::g_main::g_entities.as_mut_ptr()) as isize as i32,
        b"print \"Vote cast.\n\"\x00" as *const u8 as *const libc::c_char,
    );
    (*(*ent).client).ps.eFlags |= 0x4000 as i32;
    crate::src::game::g_syscalls::trap_Argv(
        1 as i32,
        msg.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as i32,
    );
    if ({
        let mut __res: i32 = 0;
        if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong > 1 as i32 as libc::c_ulong {
            if 0 != 0 {
                let mut __c: i32 = msg[0 as i32 as usize] as i32;
                __res = if __c < -(128 as i32) || __c > 255 as i32 {
                    __c
                } else {
                    *(*crate::stdlib::__ctype_tolower_loc()).offset(__c as isize)
                }
            } else {
                __res = tolower(msg[0 as i32 as usize] as i32)
            }
        } else {
            __res = *(*crate::stdlib::__ctype_tolower_loc())
                .offset(msg[0 as i32 as usize] as i32 as isize)
        }
        __res
    }) == 'y' as i32
        || msg[0 as i32 as usize] as i32 == '1' as i32
    {
        crate::src::game::g_main::level.voteYes += 1;
        crate::src::game::g_syscalls::trap_SetConfigstring(
            10 as i32,
            crate::src::qcommon::q_shared::va(
                b"%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                crate::src::game::g_main::level.voteYes,
            ),
        );
    } else {
        crate::src::game::g_main::level.voteNo += 1;
        crate::src::game::g_syscalls::trap_SetConfigstring(
            11 as i32,
            crate::src::qcommon::q_shared::va(
                b"%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                crate::src::game::g_main::level.voteNo,
            ),
        );
    };
    // a majority will be determined in CheckVote, which will also account
    // for players entering or leaving
}
/*
==================
Cmd_CallTeamVote_f
==================
*/
#[no_mangle]

pub unsafe extern "C" fn Cmd_CallTeamVote_f(mut ent: *mut crate::g_local_h::gentity_t) {
    let mut c: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: i32 = 0;
    let mut team: i32 = 0;
    let mut cs_offset: i32 = 0;
    let mut arg1: [libc::c_char; 1024] = [0; 1024];
    let mut arg2: [libc::c_char; 1024] = [0; 1024];
    team = (*(*ent).client).sess.sessionTeam as i32;
    if team == crate::bg_public_h::TEAM_RED as i32 {
        cs_offset = 0 as i32
    } else if team == crate::bg_public_h::TEAM_BLUE as i32 {
        cs_offset = 1 as i32
    } else {
        return;
    }
    if crate::src::game::g_main::g_allowVote.integer == 0 {
        crate::src::game::g_syscalls::trap_SendServerCommand(
            ent.offset_from(crate::src::game::g_main::g_entities.as_mut_ptr()) as isize as i32,
            b"print \"Voting not allowed here.\n\"\x00" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if crate::src::game::g_main::level.teamVoteTime[cs_offset as usize] != 0 {
        crate::src::game::g_syscalls::trap_SendServerCommand(
            ent.offset_from(crate::src::game::g_main::g_entities.as_mut_ptr()) as isize as i32,
            b"print \"A team vote is already in progress.\n\"\x00" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    if (*(*ent).client).pers.teamVoteCount >= 3 as i32 {
        crate::src::game::g_syscalls::trap_SendServerCommand(
            ent.offset_from(crate::src::game::g_main::g_entities.as_mut_ptr()) as isize as i32,
            b"print \"You have called the maximum number of team votes.\n\"\x00" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    if (*(*ent).client).sess.sessionTeam as u32 == crate::bg_public_h::TEAM_SPECTATOR as i32 as u32
    {
        crate::src::game::g_syscalls::trap_SendServerCommand(
            ent.offset_from(crate::src::game::g_main::g_entities.as_mut_ptr()) as isize as i32,
            b"print \"Not allowed to call a vote as spectator.\n\"\x00" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    // make sure it is a valid command to vote on
    crate::src::game::g_syscalls::trap_Argv(
        1 as i32,
        arg1.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as i32,
    );
    arg2[0 as i32 as usize] = '\u{0}' as i32 as libc::c_char;
    i = 2 as i32;
    while i < crate::src::game::g_syscalls::trap_Argc() {
        if i > 2 as i32 {
            ::libc::strcat(
                arg2.as_mut_ptr(),
                b" \x00" as *const u8 as *const libc::c_char,
            );
        }
        crate::src::game::g_syscalls::trap_Argv(
            i,
            &mut *arg2.as_mut_ptr().offset((crate::stdlib::strlen
                as unsafe extern "C" fn(_: *const libc::c_char) -> libc::c_ulong)(
                arg2.as_mut_ptr()
            ) as isize),
            (::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong)
                .wrapping_sub(crate::stdlib::strlen(arg2.as_mut_ptr())) as i32,
        );
        i += 1
    }
    // check for command separators in arg2
    c = arg2.as_mut_ptr();
    while *c != 0 {
        match *c as i32 {
            10 | 13 | 59 => {
                crate::src::game::g_syscalls::trap_SendServerCommand(
                    ent.offset_from(crate::src::game::g_main::g_entities.as_mut_ptr()) as isize
                        as i32,
                    b"print \"Invalid vote string.\n\"\x00" as *const u8 as *const libc::c_char,
                );
                return;
            }
            _ => {}
        }
        c = c.offset(1)
    }
    if crate::src::qcommon::q_shared::Q_stricmp(
        arg1.as_mut_ptr(),
        b"leader\x00" as *const u8 as *const libc::c_char,
    ) == 0
    {
        let mut netname: [libc::c_char; 36] = [0; 36];
        let mut leader: [libc::c_char; 36] = [0; 36];
        if arg2[0 as i32 as usize] == 0 {
            i = (*(*ent).client).ps.clientNum
        } else {
            // numeric values are just slot numbers
            i = 0 as i32;
            while i < 3 as i32 {
                if arg2[i as usize] == 0
                    || (arg2[i as usize] as i32) < '0' as i32
                    || arg2[i as usize] as i32 > '9' as i32
                {
                    break;
                }
                i += 1
            }
            if i >= 3 as i32 || arg2[i as usize] == 0 {
                i = atoi(arg2.as_mut_ptr());
                if i < 0 as i32 || i >= crate::src::game::g_main::level.maxclients {
                    crate::src::game::g_syscalls::trap_SendServerCommand(
                        ent.offset_from(crate::src::game::g_main::g_entities.as_mut_ptr()) as isize
                            as i32,
                        crate::src::qcommon::q_shared::va(
                            b"print \"Bad client slot: %i\n\"\x00" as *const u8
                                as *const libc::c_char
                                as *mut libc::c_char,
                            i,
                        ),
                    );
                    return;
                }
                if crate::src::game::g_main::g_entities[i as usize].inuse as u64 == 0 {
                    crate::src::game::g_syscalls::trap_SendServerCommand(
                        ent.offset_from(crate::src::game::g_main::g_entities.as_mut_ptr()) as isize
                            as i32,
                        crate::src::qcommon::q_shared::va(
                            b"print \"Client %i is not active\n\"\x00" as *const u8
                                as *const libc::c_char
                                as *mut libc::c_char,
                            i,
                        ),
                    );
                    return;
                }
            } else {
                crate::src::qcommon::q_shared::Q_strncpyz(
                    leader.as_mut_ptr(),
                    arg2.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as i32,
                );
                crate::src::qcommon::q_shared::Q_CleanStr(leader.as_mut_ptr());
                i = 0 as i32;
                while i < crate::src::game::g_main::level.maxclients {
                    if !((*crate::src::game::g_main::level.clients.offset(i as isize))
                        .pers
                        .connected as u32
                        == crate::g_local_h::CON_DISCONNECTED as i32 as u32)
                    {
                        if !((*crate::src::game::g_main::level.clients.offset(i as isize))
                            .sess
                            .sessionTeam as u32
                            != team as u32)
                        {
                            crate::src::qcommon::q_shared::Q_strncpyz(
                                netname.as_mut_ptr(),
                                (*crate::src::game::g_main::level.clients.offset(i as isize))
                                    .pers
                                    .netname
                                    .as_mut_ptr(),
                                ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as i32,
                            );
                            crate::src::qcommon::q_shared::Q_CleanStr(netname.as_mut_ptr());
                            if crate::src::qcommon::q_shared::Q_stricmp(
                                netname.as_mut_ptr(),
                                leader.as_mut_ptr(),
                            ) == 0
                            {
                                break;
                            }
                        }
                    }
                    i += 1
                }
                if i >= crate::src::game::g_main::level.maxclients {
                    crate::src::game::g_syscalls::trap_SendServerCommand(
                        ent.offset_from(crate::src::game::g_main::g_entities.as_mut_ptr()) as isize
                            as i32,
                        crate::src::qcommon::q_shared::va(
                            b"print \"%s is not a valid player on your team.\n\"\x00" as *const u8
                                as *const libc::c_char
                                as *mut libc::c_char,
                            arg2.as_mut_ptr(),
                        ),
                    );
                    return;
                }
            }
        }
        crate::src::qcommon::q_shared::Com_sprintf(
            arg2.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as i32,
            b"%d\x00" as *const u8 as *const libc::c_char,
            i,
        );
    } else {
        crate::src::game::g_syscalls::trap_SendServerCommand(
            ent.offset_from(crate::src::game::g_main::g_entities.as_mut_ptr()) as isize as i32,
            b"print \"Invalid vote string.\n\"\x00" as *const u8 as *const libc::c_char,
        );
        crate::src::game::g_syscalls::trap_SendServerCommand(
            ent.offset_from(crate::src::game::g_main::g_entities.as_mut_ptr()) as isize as i32,
            b"print \"Team vote commands are: leader <player>.\n\"\x00" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    crate::src::qcommon::q_shared::Com_sprintf(
        crate::src::game::g_main::level.teamVoteString[cs_offset as usize].as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as i32,
        b"%s %s\x00" as *const u8 as *const libc::c_char,
        arg1.as_mut_ptr(),
        arg2.as_mut_ptr(),
    );
    i = 0 as i32;
    while i < crate::src::game::g_main::level.maxclients {
        if !((*crate::src::game::g_main::level.clients.offset(i as isize))
            .pers
            .connected as u32
            == crate::g_local_h::CON_DISCONNECTED as i32 as u32)
        {
            if (*crate::src::game::g_main::level.clients.offset(i as isize))
                .sess
                .sessionTeam as u32
                == team as u32
            {
                crate::src::game::g_syscalls::trap_SendServerCommand(
                    i,
                    crate::src::qcommon::q_shared::va(
                        b"print \"%s called a team vote.\n\"\x00" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                        (*(*ent).client).pers.netname.as_mut_ptr(),
                    ),
                );
            }
        }
        i += 1
    }
    // start the voting, the caller automatically votes yes
    crate::src::game::g_main::level.teamVoteTime[cs_offset as usize] =
        crate::src::game::g_main::level.time;
    crate::src::game::g_main::level.teamVoteYes[cs_offset as usize] = 1 as i32;
    crate::src::game::g_main::level.teamVoteNo[cs_offset as usize] = 0 as i32;
    i = 0 as i32;
    while i < crate::src::game::g_main::level.maxclients {
        if (*crate::src::game::g_main::level.clients.offset(i as isize))
            .sess
            .sessionTeam as u32
            == team as u32
        {
            (*crate::src::game::g_main::level.clients.offset(i as isize))
                .ps
                .eFlags &= !(0x80000 as i32)
        }
        i += 1
    }
    (*(*ent).client).ps.eFlags |= 0x80000 as i32;
    crate::src::game::g_syscalls::trap_SetConfigstring(
        12 as i32 + cs_offset,
        crate::src::qcommon::q_shared::va(
            b"%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            crate::src::game::g_main::level.teamVoteTime[cs_offset as usize],
        ),
    );
    crate::src::game::g_syscalls::trap_SetConfigstring(
        14 as i32 + cs_offset,
        crate::src::game::g_main::level.teamVoteString[cs_offset as usize].as_mut_ptr(),
    );
    crate::src::game::g_syscalls::trap_SetConfigstring(
        16 as i32 + cs_offset,
        crate::src::qcommon::q_shared::va(
            b"%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            crate::src::game::g_main::level.teamVoteYes[cs_offset as usize],
        ),
    );
    crate::src::game::g_syscalls::trap_SetConfigstring(
        18 as i32 + cs_offset,
        crate::src::qcommon::q_shared::va(
            b"%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            crate::src::game::g_main::level.teamVoteNo[cs_offset as usize],
        ),
    );
}
/*
==================
Cmd_TeamVote_f
==================
*/
#[no_mangle]

pub unsafe extern "C" fn Cmd_TeamVote_f(mut ent: *mut crate::g_local_h::gentity_t) {
    let mut team: i32 = 0;
    let mut cs_offset: i32 = 0;
    let mut msg: [libc::c_char; 64] = [0; 64];
    team = (*(*ent).client).sess.sessionTeam as i32;
    if team == crate::bg_public_h::TEAM_RED as i32 {
        cs_offset = 0 as i32
    } else if team == crate::bg_public_h::TEAM_BLUE as i32 {
        cs_offset = 1 as i32
    } else {
        return;
    }
    if crate::src::game::g_main::level.teamVoteTime[cs_offset as usize] == 0 {
        crate::src::game::g_syscalls::trap_SendServerCommand(
            ent.offset_from(crate::src::game::g_main::g_entities.as_mut_ptr()) as isize as i32,
            b"print \"No team vote in progress.\n\"\x00" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if (*(*ent).client).ps.eFlags & 0x80000 as i32 != 0 {
        crate::src::game::g_syscalls::trap_SendServerCommand(
            ent.offset_from(crate::src::game::g_main::g_entities.as_mut_ptr()) as isize as i32,
            b"print \"Team vote already cast.\n\"\x00" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if (*(*ent).client).sess.sessionTeam as u32 == crate::bg_public_h::TEAM_SPECTATOR as i32 as u32
    {
        crate::src::game::g_syscalls::trap_SendServerCommand(
            ent.offset_from(crate::src::game::g_main::g_entities.as_mut_ptr()) as isize as i32,
            b"print \"Not allowed to vote as spectator.\n\"\x00" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    crate::src::game::g_syscalls::trap_SendServerCommand(
        ent.offset_from(crate::src::game::g_main::g_entities.as_mut_ptr()) as isize as i32,
        b"print \"Team vote cast.\n\"\x00" as *const u8 as *const libc::c_char,
    );
    (*(*ent).client).ps.eFlags |= 0x80000 as i32;
    crate::src::game::g_syscalls::trap_Argv(
        1 as i32,
        msg.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as i32,
    );
    if ({
        let mut __res: i32 = 0;
        if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong > 1 as i32 as libc::c_ulong {
            if 0 != 0 {
                let mut __c: i32 = msg[0 as i32 as usize] as i32;
                __res = if __c < -(128 as i32) || __c > 255 as i32 {
                    __c
                } else {
                    *(*crate::stdlib::__ctype_tolower_loc()).offset(__c as isize)
                }
            } else {
                __res = tolower(msg[0 as i32 as usize] as i32)
            }
        } else {
            __res = *(*crate::stdlib::__ctype_tolower_loc())
                .offset(msg[0 as i32 as usize] as i32 as isize)
        }
        __res
    }) == 'y' as i32
        || msg[0 as i32 as usize] as i32 == '1' as i32
    {
        crate::src::game::g_main::level.teamVoteYes[cs_offset as usize] += 1;
        crate::src::game::g_syscalls::trap_SetConfigstring(
            16 as i32 + cs_offset,
            crate::src::qcommon::q_shared::va(
                b"%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                crate::src::game::g_main::level.teamVoteYes[cs_offset as usize],
            ),
        );
    } else {
        crate::src::game::g_main::level.teamVoteNo[cs_offset as usize] += 1;
        crate::src::game::g_syscalls::trap_SetConfigstring(
            18 as i32 + cs_offset,
            crate::src::qcommon::q_shared::va(
                b"%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                crate::src::game::g_main::level.teamVoteNo[cs_offset as usize],
            ),
        );
    };
    // a majority will be determined in TeamCheckVote, which will also account
    // for players entering or leaving
}
/*
=================
Cmd_SetViewpos_f
=================
*/
#[no_mangle]

pub unsafe extern "C" fn Cmd_SetViewpos_f(mut ent: *mut crate::g_local_h::gentity_t) {
    let mut origin: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut angles: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut buffer: [libc::c_char; 1024] = [0; 1024];
    let mut i: i32 = 0;
    if crate::src::game::g_main::g_cheats.integer == 0 {
        crate::src::game::g_syscalls::trap_SendServerCommand(
            ent.offset_from(crate::src::game::g_main::g_entities.as_mut_ptr()) as isize as i32,
            b"print \"Cheats are not enabled on this server.\n\"\x00" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    if crate::src::game::g_syscalls::trap_Argc() != 5 as i32 {
        crate::src::game::g_syscalls::trap_SendServerCommand(
            ent.offset_from(crate::src::game::g_main::g_entities.as_mut_ptr()) as isize as i32,
            b"print \"usage: setviewpos x y z yaw\n\"\x00" as *const u8 as *const libc::c_char,
        );
        return;
    }
    angles[2 as i32 as usize] = 0 as i32 as crate::src::qcommon::q_shared::vec_t;
    angles[1 as i32 as usize] = angles[2 as i32 as usize];
    angles[0 as i32 as usize] = angles[1 as i32 as usize];
    i = 0 as i32;
    while i < 3 as i32 {
        crate::src::game::g_syscalls::trap_Argv(
            i + 1 as i32,
            buffer.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as i32,
        );
        origin[i as usize] = atof(buffer.as_mut_ptr()) as crate::src::qcommon::q_shared::vec_t;
        i += 1
    }
    crate::src::game::g_syscalls::trap_Argv(
        4 as i32,
        buffer.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as i32,
    );
    angles[1 as i32 as usize] = atof(buffer.as_mut_ptr()) as crate::src::qcommon::q_shared::vec_t;
    crate::src::game::g_misc::TeleportPlayer(
        ent as *mut crate::g_local_h::gentity_s,
        origin.as_mut_ptr(),
        angles.as_mut_ptr(),
    );
}
/*
=================
Cmd_Stats_f
=================
*/
#[no_mangle]

pub unsafe extern "C" fn Cmd_Stats_f(mut _ent: *mut crate::g_local_h::gentity_t) {
    /*
        int max, n, i;

        max = trap_AAS_PointReachabilityAreaIndex( NULL );

        n = 0;
        for ( i = 0; i < max; i++ ) {
            if ( ent->client->areabits[i >> 3] & (1 << (i & 7)) )
                n++;
        }

        //trap_SendServerCommand( ent-g_entities, va("print \"visited %d of %d areas\n\"", n, max));
        trap_SendServerCommand( ent-g_entities, va("print \"%d%% level coverage\n\"", n * 100 / max));
    */
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
/*
=================
ClientCommand
=================
*/
#[no_mangle]

pub unsafe extern "C" fn ClientCommand(mut clientNum: i32) {
    let mut ent: *mut crate::g_local_h::gentity_t = 0 as *mut crate::g_local_h::gentity_t;
    let mut cmd: [libc::c_char; 1024] = [0; 1024];
    ent = crate::src::game::g_main::g_entities
        .as_mut_ptr()
        .offset(clientNum as isize);
    if (*ent).client.is_null()
        || (*(*ent).client).pers.connected as u32 != crate::g_local_h::CON_CONNECTED as i32 as u32
    {
        if !(*ent).client.is_null() && (*(*ent).client).pers.localClient as u32 != 0 {
            // Handle early team command sent by UI when starting a local
            // team play game.
            crate::src::game::g_syscalls::trap_Argv(
                0 as i32,
                cmd.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as i32,
            );
            if crate::src::qcommon::q_shared::Q_stricmp(
                cmd.as_mut_ptr(),
                b"team\x00" as *const u8 as *const libc::c_char,
            ) == 0 as i32
            {
                Cmd_Team_f(ent);
            }
        }
        return;
        // not fully in game yet
    }
    crate::src::game::g_syscalls::trap_Argv(
        0 as i32,
        cmd.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as i32,
    );
    if crate::src::qcommon::q_shared::Q_stricmp(
        cmd.as_mut_ptr(),
        b"say\x00" as *const u8 as *const libc::c_char,
    ) == 0 as i32
    {
        Cmd_Say_f(ent, 0 as i32, crate::src::qcommon::q_shared::qfalse);
        return;
    }
    if crate::src::qcommon::q_shared::Q_stricmp(
        cmd.as_mut_ptr(),
        b"say_team\x00" as *const u8 as *const libc::c_char,
    ) == 0 as i32
    {
        Cmd_Say_f(ent, 1 as i32, crate::src::qcommon::q_shared::qfalse);
        return;
    }
    if crate::src::qcommon::q_shared::Q_stricmp(
        cmd.as_mut_ptr(),
        b"tell\x00" as *const u8 as *const libc::c_char,
    ) == 0 as i32
    {
        Cmd_Tell_f(ent);
        return;
    }
    if crate::src::qcommon::q_shared::Q_stricmp(
        cmd.as_mut_ptr(),
        b"score\x00" as *const u8 as *const libc::c_char,
    ) == 0 as i32
    {
        Cmd_Score_f(ent);
        return;
    }
    // ignore all other commands when at intermission
    if crate::src::game::g_main::level.intermissiontime != 0 {
        Cmd_Say_f(
            ent,
            crate::src::qcommon::q_shared::qfalse as i32,
            crate::src::qcommon::q_shared::qtrue,
        );
        return;
    }
    if crate::src::qcommon::q_shared::Q_stricmp(
        cmd.as_mut_ptr(),
        b"give\x00" as *const u8 as *const libc::c_char,
    ) == 0 as i32
    {
        Cmd_Give_f(ent);
    } else if crate::src::qcommon::q_shared::Q_stricmp(
        cmd.as_mut_ptr(),
        b"god\x00" as *const u8 as *const libc::c_char,
    ) == 0 as i32
    {
        Cmd_God_f(ent);
    } else if crate::src::qcommon::q_shared::Q_stricmp(
        cmd.as_mut_ptr(),
        b"notarget\x00" as *const u8 as *const libc::c_char,
    ) == 0 as i32
    {
        Cmd_Notarget_f(ent);
    } else if crate::src::qcommon::q_shared::Q_stricmp(
        cmd.as_mut_ptr(),
        b"noclip\x00" as *const u8 as *const libc::c_char,
    ) == 0 as i32
    {
        Cmd_Noclip_f(ent);
    } else if crate::src::qcommon::q_shared::Q_stricmp(
        cmd.as_mut_ptr(),
        b"kill\x00" as *const u8 as *const libc::c_char,
    ) == 0 as i32
    {
        Cmd_Kill_f(ent);
    } else if crate::src::qcommon::q_shared::Q_stricmp(
        cmd.as_mut_ptr(),
        b"teamtask\x00" as *const u8 as *const libc::c_char,
    ) == 0 as i32
    {
        Cmd_TeamTask_f(ent);
    } else if crate::src::qcommon::q_shared::Q_stricmp(
        cmd.as_mut_ptr(),
        b"levelshot\x00" as *const u8 as *const libc::c_char,
    ) == 0 as i32
    {
        Cmd_LevelShot_f(ent);
    } else if crate::src::qcommon::q_shared::Q_stricmp(
        cmd.as_mut_ptr(),
        b"follow\x00" as *const u8 as *const libc::c_char,
    ) == 0 as i32
    {
        Cmd_Follow_f(ent);
    } else if crate::src::qcommon::q_shared::Q_stricmp(
        cmd.as_mut_ptr(),
        b"follownext\x00" as *const u8 as *const libc::c_char,
    ) == 0 as i32
    {
        Cmd_FollowCycle_f(ent, 1 as i32);
    } else if crate::src::qcommon::q_shared::Q_stricmp(
        cmd.as_mut_ptr(),
        b"followprev\x00" as *const u8 as *const libc::c_char,
    ) == 0 as i32
    {
        Cmd_FollowCycle_f(ent, -(1 as i32));
    } else if crate::src::qcommon::q_shared::Q_stricmp(
        cmd.as_mut_ptr(),
        b"team\x00" as *const u8 as *const libc::c_char,
    ) == 0 as i32
    {
        Cmd_Team_f(ent);
    } else if crate::src::qcommon::q_shared::Q_stricmp(
        cmd.as_mut_ptr(),
        b"where\x00" as *const u8 as *const libc::c_char,
    ) == 0 as i32
    {
        Cmd_Where_f(ent);
    } else if crate::src::qcommon::q_shared::Q_stricmp(
        cmd.as_mut_ptr(),
        b"callvote\x00" as *const u8 as *const libc::c_char,
    ) == 0 as i32
    {
        Cmd_CallVote_f(ent);
    } else if crate::src::qcommon::q_shared::Q_stricmp(
        cmd.as_mut_ptr(),
        b"vote\x00" as *const u8 as *const libc::c_char,
    ) == 0 as i32
    {
        Cmd_Vote_f(ent);
    } else if crate::src::qcommon::q_shared::Q_stricmp(
        cmd.as_mut_ptr(),
        b"callteamvote\x00" as *const u8 as *const libc::c_char,
    ) == 0 as i32
    {
        Cmd_CallTeamVote_f(ent);
    } else if crate::src::qcommon::q_shared::Q_stricmp(
        cmd.as_mut_ptr(),
        b"teamvote\x00" as *const u8 as *const libc::c_char,
    ) == 0 as i32
    {
        Cmd_TeamVote_f(ent);
    } else if crate::src::qcommon::q_shared::Q_stricmp(
        cmd.as_mut_ptr(),
        b"gc\x00" as *const u8 as *const libc::c_char,
    ) == 0 as i32
    {
        Cmd_GameCommand_f(ent);
    } else if crate::src::qcommon::q_shared::Q_stricmp(
        cmd.as_mut_ptr(),
        b"setviewpos\x00" as *const u8 as *const libc::c_char,
    ) == 0 as i32
    {
        Cmd_SetViewpos_f(ent);
    } else if crate::src::qcommon::q_shared::Q_stricmp(
        cmd.as_mut_ptr(),
        b"stats\x00" as *const u8 as *const libc::c_char,
    ) == 0 as i32
    {
        Cmd_Stats_f(ent);
    } else {
        crate::src::game::g_syscalls::trap_SendServerCommand(
            clientNum,
            crate::src::qcommon::q_shared::va(
                b"print \"unknown cmd %s\n\"\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                cmd.as_mut_ptr(),
            ),
        );
    };
}
unsafe extern "C" fn run_static_initializers() {
    numgc_orders = (::std::mem::size_of::<[*mut libc::c_char; 7]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
        as i32
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];

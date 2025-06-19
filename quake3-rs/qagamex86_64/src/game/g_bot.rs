use ::libc;

pub mod stdlib_float_h {
    #[inline]

    pub unsafe extern "C" fn atof(mut __nptr: *const libc::c_char) -> f64 {
        return libc::strtod(__nptr, 0 as *mut libc::c_void as *mut *mut libc::c_char);
    }
}

pub mod stdlib_h {
    #[inline]

    pub unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> i32 {
        return libc::strtol(
            __nptr,
            0 as *mut libc::c_void as *mut *mut libc::c_char,
            10 as i32,
        ) as i32;
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
pub use crate::src::game::g_main::Com_Printf;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::cplane_s;
pub use crate::src::qcommon::q_shared::cplane_t;
pub use crate::src::qcommon::q_shared::cvarHandle_t;
pub use crate::src::qcommon::q_shared::entityState_s;
pub use crate::src::qcommon::q_shared::entityState_t;
pub use crate::src::qcommon::q_shared::fileHandle_t;
pub use crate::src::qcommon::q_shared::fsMode_t;
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
pub use crate::src::qcommon::q_shared::COM_Parse;
pub use crate::src::qcommon::q_shared::COM_ParseExt;
pub use crate::src::qcommon::q_shared::Com_Clamp;
pub use crate::src::qcommon::q_shared::Info_SetValueForKey;
pub use crate::src::qcommon::q_shared::Info_ValueForKey;
pub use crate::src::qcommon::q_shared::Q_stricmp;
pub use crate::src::qcommon::q_shared::Q_strncpyz;
pub use crate::src::qcommon::q_shared::EXEC_APPEND;
pub use crate::src::qcommon::q_shared::EXEC_INSERT;
pub use crate::src::qcommon::q_shared::EXEC_NOW;
pub use crate::src::qcommon::q_shared::FS_APPEND;
pub use crate::src::qcommon::q_shared::FS_APPEND_SYNC;
pub use crate::src::qcommon::q_shared::FS_READ;
pub use crate::src::qcommon::q_shared::FS_WRITE;
pub use crate::src::qcommon::q_shared::TR_GRAVITY;
pub use crate::src::qcommon::q_shared::TR_INTERPOLATE;
pub use crate::src::qcommon::q_shared::TR_LINEAR;
pub use crate::src::qcommon::q_shared::TR_LINEAR_STOP;
pub use crate::src::qcommon::q_shared::TR_SINE;
pub use crate::src::qcommon::q_shared::TR_STATIONARY;

pub use crate::g_local_h::bot_settings_s;
pub use crate::g_local_h::bot_settings_t;
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
pub use crate::src::game::ai_main::BotAISetupClient;
pub use crate::src::game::g_bot::stdlib_float_h::atof;
pub use crate::src::game::g_bot::stdlib_h::atoi;
pub use crate::src::game::g_client::ClientBegin;
pub use crate::src::game::g_client::ClientConnect;
pub use crate::src::game::g_client::PickTeam;
pub use crate::src::game::g_main::g_entities;
pub use crate::src::game::g_main::g_gametype;
pub use crate::src::game::g_main::g_maxclients;
pub use crate::src::game::g_main::level;
pub use crate::src::game::g_main::G_Printf;
pub use crate::src::game::g_mem::G_Alloc;
pub use crate::src::game::g_syscalls::trap_Argv;
pub use crate::src::game::g_syscalls::trap_BotAllocateClient;
pub use crate::src::game::g_syscalls::trap_BotFreeClient;
pub use crate::src::game::g_syscalls::trap_Cvar_Register;
pub use crate::src::game::g_syscalls::trap_Cvar_Set;
pub use crate::src::game::g_syscalls::trap_Cvar_Update;
pub use crate::src::game::g_syscalls::trap_Cvar_VariableIntegerValue;
pub use crate::src::game::g_syscalls::trap_Cvar_VariableStringBuffer;
pub use crate::src::game::g_syscalls::trap_DropClient;
pub use crate::src::game::g_syscalls::trap_FS_FCloseFile;
pub use crate::src::game::g_syscalls::trap_FS_FOpenFile;
pub use crate::src::game::g_syscalls::trap_FS_GetFileList;
pub use crate::src::game::g_syscalls::trap_FS_Read;
pub use crate::src::game::g_syscalls::trap_GetServerinfo;
pub use crate::src::game::g_syscalls::trap_GetUserinfo;
pub use crate::src::game::g_syscalls::trap_Print;
pub use crate::src::game::g_syscalls::trap_SendConsoleCommand;
pub use crate::src::game::g_syscalls::trap_SendServerCommand;
pub use crate::src::game::g_syscalls::trap_SetUserinfo;

pub use ::libc::rand;

pub use ::libc::strtod;
pub use ::libc::strtol;
extern "C" {
    #[no_mangle]
    pub static mut podium1: *mut gentity_t;
    #[no_mangle]
    pub static mut podium2: *mut gentity_t;
    #[no_mangle]
    pub static mut podium3: *mut gentity_t;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct botSpawnQueue_t {
    pub clientNum: i32,
    pub spawnTime: i32,
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
// g_bot.c

static mut g_numBots: i32 = 0;

static mut g_botInfos: [*mut libc::c_char; 1024] =
    [0 as *const libc::c_char as *mut libc::c_char; 1024];
#[no_mangle]

pub static mut g_numArenas: i32 = 0;

static mut g_arenaInfos: [*mut libc::c_char; 1024] =
    [0 as *const libc::c_char as *mut libc::c_char; 1024];

static mut botSpawnQueue: [botSpawnQueue_t; 16] = [botSpawnQueue_t {
    clientNum: 0,
    spawnTime: 0,
}; 16];
#[no_mangle]

pub static mut bot_minplayers: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]

pub unsafe extern "C" fn trap_Cvar_VariableValue(mut var_name: *const libc::c_char) -> f32 {
    let mut buf: [libc::c_char; 128] = [0; 128];
    trap_Cvar_VariableStringBuffer(
        var_name,
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 128]>() as usize as i32,
    );
    return atof(buf.as_mut_ptr()) as f32;
}
/*
===============
G_ParseInfos
===============
*/
#[no_mangle]

pub unsafe extern "C" fn G_ParseInfos(
    mut buf: *mut libc::c_char,
    mut max: i32,
    mut infos: *mut *mut libc::c_char,
) -> i32 {
    let mut token: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut count: i32 = 0;
    let mut key: [libc::c_char; 1024] = [0; 1024];
    let mut info: [libc::c_char; 1024] = [0; 1024];
    count = 0 as i32;
    loop {
        token = COM_Parse(&mut buf);
        if *token.offset(0 as i32 as isize) == 0 {
            break;
        }
        if libc::strcmp(token, b"{\x00" as *const u8 as *const libc::c_char) != 0 {
            Com_Printf(b"Missing { in info file\n\x00" as *const u8 as *const libc::c_char);
            break;
        } else if count == max {
            Com_Printf(b"Max infos exceeded\n\x00" as *const u8 as *const libc::c_char);
            break;
        } else {
            info[0 as i32 as usize] = '\u{0}' as i32 as libc::c_char;
            loop {
                token = COM_ParseExt(&mut buf, qtrue);
                if *token.offset(0 as i32 as isize) == 0 {
                    Com_Printf(
                        b"Unexpected end of info file\n\x00" as *const u8 as *const libc::c_char,
                    );
                    break;
                } else {
                    if libc::strcmp(token, b"}\x00" as *const u8 as *const libc::c_char) == 0 {
                        break;
                    }
                    Q_strncpyz(
                        key.as_mut_ptr(),
                        token,
                        ::std::mem::size_of::<[libc::c_char; 1024]>() as usize as i32,
                    );
                    token = COM_ParseExt(&mut buf, qfalse);
                    if *token.offset(0 as i32 as isize) == 0 {
                        libc::strcpy(token, b"<NULL>\x00" as *const u8 as *const libc::c_char);
                    }
                    Info_SetValueForKey(info.as_mut_ptr(), key.as_mut_ptr(), token);
                }
            }
            //NOTE: extra space for arena number
            let ref mut fresh0 = *infos.offset(count as isize);
            *fresh0 = G_Alloc(
                crate::stdlib::strlen(info.as_mut_ptr())
                    .wrapping_add(crate::stdlib::strlen(
                        b"\\num\\\x00" as *const u8 as *const libc::c_char,
                    ))
                    .wrapping_add(crate::stdlib::strlen(va(
                        b"%d\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        1024 as i32,
                    )))
                    .wrapping_add(1 as i32 as usize) as i32,
            ) as *mut libc::c_char;
            if !(*infos.offset(count as isize)).is_null() {
                libc::strcpy(*infos.offset(count as isize), info.as_mut_ptr());
                count += 1
            }
        }
    }
    return count;
}
/*
===============
G_LoadArenasFromFile
===============
*/

unsafe extern "C" fn G_LoadArenasFromFile(mut filename: *mut libc::c_char) {
    let mut len: i32 = 0;
    let mut f: fileHandle_t = 0;
    let mut buf: [libc::c_char; 8192] = [0; 8192];
    len = trap_FS_FOpenFile(filename, &mut f, FS_READ);
    if f == 0 {
        trap_Print(va(
            b"^1file not found: %s\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            filename,
        ));
        return;
    }
    if len >= 8192 as i32 {
        trap_FS_FCloseFile(f);
        trap_Print(va(
            b"^1file too large: %s is %i, max allowed is %i\n\x00" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            filename,
            len,
            8192 as i32,
        ));
        return;
    }
    trap_FS_Read(buf.as_mut_ptr() as *mut libc::c_void, len, f);
    buf[len as usize] = 0 as i32 as libc::c_char;
    trap_FS_FCloseFile(f);
    g_numArenas += G_ParseInfos(
        buf.as_mut_ptr(),
        1024 as i32 - g_numArenas,
        &mut *g_arenaInfos.as_mut_ptr().offset(g_numArenas as isize),
    );
}
/*
===============
G_LoadArenas
===============
*/

unsafe extern "C" fn G_LoadArenas() {
    let mut numdirs: i32 = 0;
    let mut arenasFile: vmCvar_t = vmCvar_t {
        handle: 0,
        modificationCount: 0,
        value: 0.,
        integer: 0,
        string: [0; 256],
    };
    let mut filename: [libc::c_char; 128] = [0; 128];
    let mut dirlist: [libc::c_char; 1024] = [0; 1024];
    let mut dirptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: i32 = 0;
    let mut n: i32 = 0;
    let mut dirlen: i32 = 0;
    g_numArenas = 0 as i32;
    trap_Cvar_Register(
        &mut arenasFile as *mut _ as *mut vmCvar_t,
        b"g_arenasFile\x00" as *const u8 as *const libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char,
        0x10 as i32 | 0x40 as i32,
    );
    if *arenasFile.string.as_mut_ptr() != 0 {
        G_LoadArenasFromFile(arenasFile.string.as_mut_ptr());
    } else {
        G_LoadArenasFromFile(
            b"scripts/arenas.txt\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    // get all arenas from .arena files
    numdirs = trap_FS_GetFileList(
        b"scripts\x00" as *const u8 as *const libc::c_char,
        b".arena\x00" as *const u8 as *const libc::c_char,
        dirlist.as_mut_ptr(),
        1024 as i32,
    );
    dirptr = dirlist.as_mut_ptr();
    i = 0 as i32;
    while i < numdirs {
        dirlen = crate::stdlib::strlen(dirptr) as i32;
        libc::strcpy(
            filename.as_mut_ptr(),
            b"scripts/\x00" as *const u8 as *const libc::c_char,
        );
        libc::strcat(filename.as_mut_ptr(), dirptr);
        G_LoadArenasFromFile(filename.as_mut_ptr());
        i += 1;
        dirptr = dirptr.offset((dirlen + 1 as i32) as isize)
    }
    trap_Print(va(
        b"%i arenas parsed\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        g_numArenas,
    ));
    n = 0 as i32;
    while n < g_numArenas {
        Info_SetValueForKey(
            g_arenaInfos[n as usize],
            b"num\x00" as *const u8 as *const libc::c_char,
            va(
                b"%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                n,
            ),
        );
        n += 1
    }
}
/*
===============
G_GetArenaInfoByNumber
===============
*/
#[no_mangle]

pub unsafe extern "C" fn G_GetArenaInfoByMap(mut map: *const libc::c_char) -> *const libc::c_char {
    let mut n: i32 = 0;
    n = 0 as i32;
    while n < g_numArenas {
        if Q_stricmp(
            Info_ValueForKey(
                g_arenaInfos[n as usize],
                b"map\x00" as *const u8 as *const libc::c_char,
            ),
            map,
        ) == 0 as i32
        {
            return g_arenaInfos[n as usize];
        }
        n += 1
    }
    return 0 as *const libc::c_char;
}
/*
=================
PlayerIntroSound
=================
*/

unsafe extern "C" fn PlayerIntroSound(mut modelAndSkin: *const libc::c_char) {
    let mut model: [libc::c_char; 64] = [0; 64];
    let mut skin: *mut libc::c_char = 0 as *mut libc::c_char;
    Q_strncpyz(
        model.as_mut_ptr(),
        modelAndSkin,
        ::std::mem::size_of::<[libc::c_char; 64]>() as usize as i32,
    );
    skin = libc::strrchr(model.as_mut_ptr(), '/' as i32);
    if !skin.is_null() {
        let fresh1 = skin;
        skin = skin.offset(1);
        *fresh1 = '\u{0}' as i32 as libc::c_char
    } else {
        skin = model.as_mut_ptr()
    }
    if Q_stricmp(skin, b"default\x00" as *const u8 as *const libc::c_char) == 0 as i32 {
        skin = model.as_mut_ptr()
    }
    trap_SendConsoleCommand(
        EXEC_APPEND as i32,
        va(
            b"play sound/player/announce/%s.wav\n\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            skin,
        ),
    );
}
/*
===============
G_CountBotPlayersByName

Check connected and connecting (delay join) bots.

Returns number of bots with name on specified team or whole server if team is -1.
===============
*/
#[no_mangle]

pub unsafe extern "C" fn G_CountBotPlayersByName(
    mut name: *const libc::c_char,
    mut team: i32,
) -> i32 {
    let mut i: i32 = 0;
    let mut num: i32 = 0;
    let mut cl: *mut gclient_t = 0 as *mut gclient_t;
    num = 0 as i32;
    i = 0 as i32;
    while i < g_maxclients.integer {
        cl = level.clients.offset(i as isize);
        if !((*cl).pers.connected as u32 == CON_DISCONNECTED as i32 as u32) {
            if !(g_entities[i as usize].r.svFlags & 0x8 as i32 == 0) {
                if !(team >= 0 as i32 && (*cl).sess.sessionTeam as u32 != team as u32) {
                    if !(!name.is_null() && Q_stricmp(name, (*cl).pers.netname.as_mut_ptr()) != 0) {
                        num += 1
                    }
                }
            }
        }
        i += 1
    }
    return num;
}
/*
===============
G_SelectRandomBotInfo

Get random least used bot info on team or whole server if team is -1.
===============
*/
#[no_mangle]

pub unsafe extern "C" fn G_SelectRandomBotInfo(mut team: i32) -> i32 {
    let mut selection: [i32; 1024] = [0; 1024];
    let mut n: i32 = 0;
    let mut num: i32 = 0;
    let mut count: i32 = 0;
    let mut bestCount: i32 = 0;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    // don't add duplicate bots to the server if there are less bots than bot types
    if team != -(1 as i32)
        && G_CountBotPlayersByName(0 as *const libc::c_char, -(1 as i32)) < g_numBots
    {
        team = -(1 as i32)
    }
    num = 0 as i32;
    bestCount = 64 as i32;
    n = 0 as i32;
    while n < g_numBots {
        value = Info_ValueForKey(
            g_botInfos[n as usize],
            b"funname\x00" as *const u8 as *const libc::c_char,
        );
        if *value.offset(0 as i32 as isize) == 0 {
            value = Info_ValueForKey(
                g_botInfos[n as usize],
                b"name\x00" as *const u8 as *const libc::c_char,
            )
        }
        //
        count = G_CountBotPlayersByName(value, team);
        if count < bestCount {
            bestCount = count;
            num = 0 as i32
        }
        if count == bestCount {
            let fresh2 = num;
            num = num + 1;
            selection[fresh2 as usize] = n;
            if num == 1024 as i32 {
                break;
            }
        }
        n += 1
    }
    if num > 0 as i32 {
        num = ((rand() & 0x7fff as i32) as f32 / 0x7fff as i32 as f32 * (num - 1 as i32) as f32)
            as i32;
        return selection[num as usize];
    }
    return -(1 as i32);
}
/*
===============
G_AddRandomBot
===============
*/
#[no_mangle]

pub unsafe extern "C" fn G_AddRandomBot(mut team: i32) {
    let mut teamstr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut skill: f32 = 0.;
    skill = trap_Cvar_VariableValue(b"g_spSkill\x00" as *const u8 as *const libc::c_char);
    if team == TEAM_RED as i32 {
        teamstr = b"red\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    } else if team == TEAM_BLUE as i32 {
        teamstr = b"blue\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    } else {
        teamstr = b"free\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    }
    trap_SendConsoleCommand(
        EXEC_INSERT as i32,
        va(
            b"addbot random %f %s %i\n\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            skill as f64,
            teamstr,
            0 as i32,
        ),
    );
}
/*
===============
G_RemoveRandomBot
===============
*/
#[no_mangle]

pub unsafe extern "C" fn G_RemoveRandomBot(mut team: i32) -> i32 {
    let mut i: i32 = 0;
    let mut cl: *mut gclient_t = 0 as *mut gclient_t;
    i = 0 as i32;
    while i < g_maxclients.integer {
        cl = level.clients.offset(i as isize);
        if !((*cl).pers.connected as u32 != CON_CONNECTED as i32 as u32) {
            if !(g_entities[i as usize].r.svFlags & 0x8 as i32 == 0) {
                if !(team >= 0 as i32 && (*cl).sess.sessionTeam as u32 != team as u32) {
                    trap_SendConsoleCommand(
                        EXEC_INSERT as i32,
                        va(
                            b"clientkick %d\n\x00" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                            i,
                        ),
                    );
                    return qtrue as i32;
                }
            }
        }
        i += 1
    }
    return qfalse as i32;
}
/*
===============
G_CountHumanPlayers
===============
*/
#[no_mangle]

pub unsafe extern "C" fn G_CountHumanPlayers(mut team: i32) -> i32 {
    let mut i: i32 = 0;
    let mut num: i32 = 0;
    let mut cl: *mut gclient_t = 0 as *mut gclient_t;
    num = 0 as i32;
    i = 0 as i32;
    while i < g_maxclients.integer {
        cl = level.clients.offset(i as isize);
        if !((*cl).pers.connected as u32 != CON_CONNECTED as i32 as u32) {
            if !(g_entities[i as usize].r.svFlags & 0x8 as i32 != 0) {
                if !(team >= 0 as i32 && (*cl).sess.sessionTeam as u32 != team as u32) {
                    num += 1
                }
            }
        }
        i += 1
    }
    return num;
}
/*
===============
G_CountBotPlayers

Check connected and connecting (delay join) bots.
===============
*/
#[no_mangle]

pub unsafe extern "C" fn G_CountBotPlayers(mut team: i32) -> i32 {
    let mut i: i32 = 0;
    let mut num: i32 = 0;
    let mut cl: *mut gclient_t = 0 as *mut gclient_t;
    num = 0 as i32;
    i = 0 as i32;
    while i < g_maxclients.integer {
        cl = level.clients.offset(i as isize);
        if !((*cl).pers.connected as u32 == CON_DISCONNECTED as i32 as u32) {
            if !(g_entities[i as usize].r.svFlags & 0x8 as i32 == 0) {
                if !(team >= 0 as i32 && (*cl).sess.sessionTeam as u32 != team as u32) {
                    num += 1
                }
            }
        }
        i += 1
    }
    return num;
}
/*
===============
G_CheckMinimumPlayers
===============
*/
#[no_mangle]

pub unsafe extern "C" fn G_CheckMinimumPlayers() {
    let mut minplayers: i32 = 0;
    let mut humanplayers: i32 = 0;
    let mut botplayers: i32 = 0;
    static mut checkminimumplayers_time: i32 = 0;
    if level.intermissiontime != 0 {
        return;
    }
    //only check once each 10 seconds
    if checkminimumplayers_time > level.time - 10000 as i32 {
        return;
    }
    checkminimumplayers_time = level.time;
    trap_Cvar_Update(&mut bot_minplayers as *mut _ as *mut vmCvar_t);
    minplayers = bot_minplayers.integer;
    if minplayers <= 0 as i32 {
        return;
    }
    if g_gametype.integer >= GT_TEAM as i32 {
        if minplayers >= g_maxclients.integer / 2 as i32 {
            minplayers = g_maxclients.integer / 2 as i32 - 1 as i32
        }
        humanplayers = G_CountHumanPlayers(TEAM_RED as i32);
        botplayers = G_CountBotPlayers(TEAM_RED as i32);
        //
        if humanplayers + botplayers < minplayers {
            G_AddRandomBot(TEAM_RED as i32);
        } else if humanplayers + botplayers > minplayers && botplayers != 0 {
            G_RemoveRandomBot(TEAM_RED as i32);
        }
        //
        humanplayers = G_CountHumanPlayers(TEAM_BLUE as i32);
        botplayers = G_CountBotPlayers(TEAM_BLUE as i32);
        //
        if humanplayers + botplayers < minplayers {
            G_AddRandomBot(TEAM_BLUE as i32);
        } else if humanplayers + botplayers > minplayers && botplayers != 0 {
            G_RemoveRandomBot(TEAM_BLUE as i32);
        }
    } else if g_gametype.integer == GT_TOURNAMENT as i32 {
        if minplayers >= g_maxclients.integer {
            minplayers = g_maxclients.integer - 1 as i32
        }
        humanplayers = G_CountHumanPlayers(-(1 as i32));
        botplayers = G_CountBotPlayers(-(1 as i32));
        //
        if humanplayers + botplayers < minplayers {
            G_AddRandomBot(TEAM_FREE as i32);
        } else if humanplayers + botplayers > minplayers && botplayers != 0 {
            // try to remove spectators first
            if G_RemoveRandomBot(TEAM_SPECTATOR as i32) == 0 {
                // just remove the bot that is playing
                G_RemoveRandomBot(-(1 as i32));
            }
        }
    } else if g_gametype.integer == GT_FFA as i32 {
        if minplayers >= g_maxclients.integer {
            minplayers = g_maxclients.integer - 1 as i32
        }
        humanplayers = G_CountHumanPlayers(TEAM_FREE as i32);
        botplayers = G_CountBotPlayers(TEAM_FREE as i32);
        //
        if humanplayers + botplayers < minplayers {
            G_AddRandomBot(TEAM_FREE as i32);
        } else if humanplayers + botplayers > minplayers && botplayers != 0 {
            G_RemoveRandomBot(TEAM_FREE as i32);
        }
    };
}
/*
===============
G_CheckBotSpawn
===============
*/
#[no_mangle]

pub unsafe extern "C" fn G_CheckBotSpawn() {
    let mut n: i32 = 0;
    let mut userinfo: [libc::c_char; 1024] = [0; 1024];
    G_CheckMinimumPlayers();
    n = 0 as i32;
    while n < 16 as i32 {
        if !(botSpawnQueue[n as usize].spawnTime == 0) {
            if !(botSpawnQueue[n as usize].spawnTime > level.time) {
                ClientBegin(botSpawnQueue[n as usize].clientNum);
                botSpawnQueue[n as usize].spawnTime = 0 as i32;
                if g_gametype.integer == GT_SINGLE_PLAYER as i32 {
                    trap_GetUserinfo(
                        botSpawnQueue[n as usize].clientNum,
                        userinfo.as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 1024]>() as usize as i32,
                    );
                    PlayerIntroSound(Info_ValueForKey(
                        userinfo.as_mut_ptr(),
                        b"model\x00" as *const u8 as *const libc::c_char,
                    ));
                }
            }
        }
        n += 1
    }
}
/*
===============
AddBotToSpawnQueue
===============
*/

unsafe extern "C" fn AddBotToSpawnQueue(mut clientNum: i32, mut delay: i32) {
    let mut n: i32 = 0;
    n = 0 as i32;
    while n < 16 as i32 {
        if botSpawnQueue[n as usize].spawnTime == 0 {
            botSpawnQueue[n as usize].spawnTime = level.time + delay;
            botSpawnQueue[n as usize].clientNum = clientNum;
            return;
        }
        n += 1
    }
    G_Printf(b"^3Unable to delay spawn\n\x00" as *const u8 as *const libc::c_char);
    ClientBegin(clientNum);
}
/*
===============
G_RemoveQueuedBotBegin

Called on client disconnect to make sure the delayed spawn
doesn't happen on a freed index
===============
*/
#[no_mangle]

pub unsafe extern "C" fn G_RemoveQueuedBotBegin(mut clientNum: i32) {
    let mut n: i32 = 0;
    n = 0 as i32;
    while n < 16 as i32 {
        if botSpawnQueue[n as usize].clientNum == clientNum {
            botSpawnQueue[n as usize].spawnTime = 0 as i32;
            return;
        }
        n += 1
    }
}
/*
===============
G_BotConnect
===============
*/
#[no_mangle]

pub unsafe extern "C" fn G_BotConnect(mut clientNum: i32, mut restart: qboolean) -> qboolean {
    let mut settings: bot_settings_t = bot_settings_t {
        characterfile: [0; 144],
        skill: 0.,
    };
    let mut userinfo: [libc::c_char; 1024] = [0; 1024];
    trap_GetUserinfo(
        clientNum,
        userinfo.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as usize as i32,
    );
    Q_strncpyz(
        settings.characterfile.as_mut_ptr(),
        Info_ValueForKey(
            userinfo.as_mut_ptr(),
            b"characterfile\x00" as *const u8 as *const libc::c_char,
        ),
        ::std::mem::size_of::<[libc::c_char; 144]>() as usize as i32,
    );
    settings.skill = atof(Info_ValueForKey(
        userinfo.as_mut_ptr(),
        b"skill\x00" as *const u8 as *const libc::c_char,
    )) as f32;
    if BotAISetupClient(
        clientNum,
        &mut settings as *mut _ as *mut bot_settings_s,
        restart,
    ) == 0
    {
        trap_DropClient(
            clientNum,
            b"BotAISetupClient failed\x00" as *const u8 as *const libc::c_char,
        );
        return qfalse;
    }
    return qtrue;
}
/*
===============
G_AddBot
===============
*/

unsafe extern "C" fn G_AddBot(
    mut name: *const libc::c_char,
    mut skill: f32,
    mut team: *const libc::c_char,
    mut delay: i32,
    mut altname: *mut libc::c_char,
) {
    let mut clientNum: i32 = 0;
    let mut teamNum: i32 = 0;
    let mut botinfoNum: i32 = 0;
    let mut botinfo: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut key: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut botname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut model: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut headmodel: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut userinfo: [libc::c_char; 1024] = [0; 1024];
    // have the server allocate a client slot
    clientNum = trap_BotAllocateClient();
    if clientNum == -(1 as i32) {
        G_Printf(
            b"^1Unable to add bot. All player slots are in use.\n\x00" as *const u8
                as *const libc::c_char,
        );
        G_Printf(b"^1Start server with more \'open\' slots (or check setting of sv_maxclients cvar).\n\x00"
                     as *const u8 as *const libc::c_char);
        return;
    }
    // set default team
    if team.is_null() || *team == 0 {
        if g_gametype.integer >= GT_TEAM as i32 {
            if PickTeam(clientNum) as u32 == TEAM_RED as i32 as u32 {
                team = b"red\x00" as *const u8 as *const libc::c_char
            } else {
                team = b"blue\x00" as *const u8 as *const libc::c_char
            }
        } else {
            team = b"free\x00" as *const u8 as *const libc::c_char
        }
    }
    // get the botinfo from bots.txt
    if Q_stricmp(name, b"random\x00" as *const u8 as *const libc::c_char) == 0 as i32 {
        if Q_stricmp(team, b"red\x00" as *const u8 as *const libc::c_char) == 0 as i32
            || Q_stricmp(team, b"r\x00" as *const u8 as *const libc::c_char) == 0 as i32
        {
            teamNum = TEAM_RED as i32
        } else if Q_stricmp(team, b"blue\x00" as *const u8 as *const libc::c_char) == 0 as i32
            || Q_stricmp(team, b"b\x00" as *const u8 as *const libc::c_char) == 0 as i32
        {
            teamNum = TEAM_BLUE as i32
        } else if Q_stricmp(team, b"spectator\x00" as *const u8 as *const libc::c_char) == 0
            || Q_stricmp(team, b"s\x00" as *const u8 as *const libc::c_char) == 0
        {
            teamNum = TEAM_SPECTATOR as i32
        } else {
            teamNum = TEAM_FREE as i32
        }
        botinfoNum = G_SelectRandomBotInfo(teamNum);
        if botinfoNum < 0 as i32 {
            G_Printf(
                b"^1Error: Cannot add random bot, no bot info available.\n\x00" as *const u8
                    as *const libc::c_char,
            );
            trap_BotFreeClient(clientNum);
            return;
        }
        botinfo = G_GetBotInfoByNumber(botinfoNum)
    } else {
        botinfo = G_GetBotInfoByName(name)
    }
    if botinfo.is_null() {
        G_Printf(
            b"^1Error: Bot \'%s\' not defined\n\x00" as *const u8 as *const libc::c_char,
            name,
        );
        trap_BotFreeClient(clientNum);
        return;
    }
    // create the bot's userinfo
    userinfo[0 as i32 as usize] = '\u{0}' as i32 as libc::c_char;
    botname = Info_ValueForKey(botinfo, b"funname\x00" as *const u8 as *const libc::c_char);
    if *botname.offset(0 as i32 as isize) == 0 {
        botname = Info_ValueForKey(botinfo, b"name\x00" as *const u8 as *const libc::c_char)
    }
    // check for an alternative name
    if !altname.is_null() && *altname.offset(0 as i32 as isize) as i32 != 0 {
        botname = altname
    }
    Info_SetValueForKey(
        userinfo.as_mut_ptr(),
        b"name\x00" as *const u8 as *const libc::c_char,
        botname,
    );
    Info_SetValueForKey(
        userinfo.as_mut_ptr(),
        b"rate\x00" as *const u8 as *const libc::c_char,
        b"25000\x00" as *const u8 as *const libc::c_char,
    );
    Info_SetValueForKey(
        userinfo.as_mut_ptr(),
        b"snaps\x00" as *const u8 as *const libc::c_char,
        b"20\x00" as *const u8 as *const libc::c_char,
    );
    Info_SetValueForKey(
        userinfo.as_mut_ptr(),
        b"skill\x00" as *const u8 as *const libc::c_char,
        va(
            b"%.2f\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            skill as f64,
        ),
    );
    Info_SetValueForKey(
        userinfo.as_mut_ptr(),
        b"teampref\x00" as *const u8 as *const libc::c_char,
        team,
    );
    if skill >= 1 as i32 as f32 && skill < 2 as i32 as f32 {
        Info_SetValueForKey(
            userinfo.as_mut_ptr(),
            b"handicap\x00" as *const u8 as *const libc::c_char,
            b"50\x00" as *const u8 as *const libc::c_char,
        );
    } else if skill >= 2 as i32 as f32 && skill < 3 as i32 as f32 {
        Info_SetValueForKey(
            userinfo.as_mut_ptr(),
            b"handicap\x00" as *const u8 as *const libc::c_char,
            b"70\x00" as *const u8 as *const libc::c_char,
        );
    } else if skill >= 3 as i32 as f32 && skill < 4 as i32 as f32 {
        Info_SetValueForKey(
            userinfo.as_mut_ptr(),
            b"handicap\x00" as *const u8 as *const libc::c_char,
            b"90\x00" as *const u8 as *const libc::c_char,
        );
    }
    key = b"model\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    model = Info_ValueForKey(botinfo, key);
    if *model == 0 {
        model = b"visor/default\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    }
    Info_SetValueForKey(userinfo.as_mut_ptr(), key, model);
    key = b"team_model\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    Info_SetValueForKey(userinfo.as_mut_ptr(), key, model);
    key = b"headmodel\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    headmodel = Info_ValueForKey(botinfo, key);
    if *headmodel == 0 {
        headmodel = model
    }
    Info_SetValueForKey(userinfo.as_mut_ptr(), key, headmodel);
    key = b"team_headmodel\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    Info_SetValueForKey(userinfo.as_mut_ptr(), key, headmodel);
    key = b"gender\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s = Info_ValueForKey(botinfo, key);
    if *s == 0 {
        s = b"male\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    }
    Info_SetValueForKey(
        userinfo.as_mut_ptr(),
        b"sex\x00" as *const u8 as *const libc::c_char,
        s,
    );
    key = b"color1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s = Info_ValueForKey(botinfo, key);
    if *s == 0 {
        s = b"4\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    }
    Info_SetValueForKey(userinfo.as_mut_ptr(), key, s);
    key = b"color2\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s = Info_ValueForKey(botinfo, key);
    if *s == 0 {
        s = b"5\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    }
    Info_SetValueForKey(userinfo.as_mut_ptr(), key, s);
    s = Info_ValueForKey(botinfo, b"aifile\x00" as *const u8 as *const libc::c_char);
    if *s == 0 {
        trap_Print(
            b"^1Error: bot has no aifile specified\n\x00" as *const u8 as *const libc::c_char,
        );
        trap_BotFreeClient(clientNum);
        return;
    }
    Info_SetValueForKey(
        userinfo.as_mut_ptr(),
        b"characterfile\x00" as *const u8 as *const libc::c_char,
        s,
    );
    // don't send tinfo to bots, they don't parse it
    Info_SetValueForKey(
        userinfo.as_mut_ptr(),
        b"teamoverlay\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
    );
    // register the userinfo
    trap_SetUserinfo(clientNum, userinfo.as_mut_ptr());
    // have it connect to the game as a normal client
    if !ClientConnect(clientNum, qtrue, qtrue).is_null() {
        return;
    }
    if delay == 0 as i32 {
        ClientBegin(clientNum);
        return;
    }
    AddBotToSpawnQueue(clientNum, delay);
}
/*
===============
Svcmd_AddBot_f
===============
*/
#[no_mangle]

pub unsafe extern "C" fn Svcmd_AddBot_f() {
    let mut skill: f32 = 0.;
    let mut delay: i32 = 0;
    let mut name: [libc::c_char; 1024] = [0; 1024];
    let mut altname: [libc::c_char; 1024] = [0; 1024];
    let mut string: [libc::c_char; 1024] = [0; 1024];
    let mut team: [libc::c_char; 1024] = [0; 1024];
    // are bots enabled?
    if trap_Cvar_VariableIntegerValue(b"bot_enable\x00" as *const u8 as *const libc::c_char) == 0 {
        return;
    }
    // name
    trap_Argv(
        1 as i32,
        name.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as usize as i32,
    );
    if name[0 as i32 as usize] == 0 {
        trap_Print(
            b"Usage: Addbot <botname> [skill 1-5] [team] [msec delay] [altname]\n\x00" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    // skill
    trap_Argv(
        2 as i32,
        string.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as usize as i32,
    );
    if string[0 as i32 as usize] == 0 {
        skill = 4 as i32 as f32
    } else {
        skill = Com_Clamp(
            1 as i32 as f32,
            5 as i32 as f32,
            atof(string.as_mut_ptr()) as f32,
        )
    }
    // team
    trap_Argv(
        3 as i32,
        team.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as usize as i32,
    );
    // delay
    trap_Argv(
        4 as i32,
        string.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as usize as i32,
    );
    if string[0 as i32 as usize] == 0 {
        delay = 0 as i32
    } else {
        delay = atoi(string.as_mut_ptr())
    }
    // alternative name
    trap_Argv(
        5 as i32,
        altname.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as usize as i32,
    );
    G_AddBot(
        name.as_mut_ptr(),
        skill,
        team.as_mut_ptr(),
        delay,
        altname.as_mut_ptr(),
    );
    // if this was issued during gameplay and we are playing locally,
    // go ahead and load the bot's media immediately
    if level.time - level.startTime > 1000 as i32
        && trap_Cvar_VariableIntegerValue(b"cl_running\x00" as *const u8 as *const libc::c_char)
            != 0
    {
        trap_SendServerCommand(
            -(1 as i32),
            b"loaddefered\n\x00" as *const u8 as *const libc::c_char,
        );
        // FIXME: spelled wrong, but not changing for demo
    };
}
/*
===============
Svcmd_BotList_f
===============
*/
#[no_mangle]

pub unsafe extern "C" fn Svcmd_BotList_f() {
    let mut i: i32 = 0;
    let mut name: [libc::c_char; 1024] = [0; 1024];
    let mut funname: [libc::c_char; 1024] = [0; 1024];
    let mut model: [libc::c_char; 1024] = [0; 1024];
    let mut aifile: [libc::c_char; 1024] = [0; 1024];
    trap_Print(
        b"^1name             model            aifile              funname\n\x00" as *const u8
            as *const libc::c_char,
    );
    i = 0 as i32;
    while i < g_numBots {
        Q_strncpyz(
            name.as_mut_ptr(),
            Info_ValueForKey(
                g_botInfos[i as usize],
                b"name\x00" as *const u8 as *const libc::c_char,
            ),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as usize as i32,
        );
        if *name.as_mut_ptr() == 0 {
            libc::strcpy(
                name.as_mut_ptr(),
                b"UnnamedPlayer\x00" as *const u8 as *const libc::c_char,
            );
        }
        Q_strncpyz(
            funname.as_mut_ptr(),
            Info_ValueForKey(
                g_botInfos[i as usize],
                b"funname\x00" as *const u8 as *const libc::c_char,
            ),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as usize as i32,
        );
        if *funname.as_mut_ptr() == 0 {
            libc::strcpy(
                funname.as_mut_ptr(),
                b"\x00" as *const u8 as *const libc::c_char,
            );
        }
        Q_strncpyz(
            model.as_mut_ptr(),
            Info_ValueForKey(
                g_botInfos[i as usize],
                b"model\x00" as *const u8 as *const libc::c_char,
            ),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as usize as i32,
        );
        if *model.as_mut_ptr() == 0 {
            libc::strcpy(
                model.as_mut_ptr(),
                b"visor/default\x00" as *const u8 as *const libc::c_char,
            );
        }
        Q_strncpyz(
            aifile.as_mut_ptr(),
            Info_ValueForKey(
                g_botInfos[i as usize],
                b"aifile\x00" as *const u8 as *const libc::c_char,
            ),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as usize as i32,
        );
        if *aifile.as_mut_ptr() == 0 {
            libc::strcpy(
                aifile.as_mut_ptr(),
                b"bots/default_c.c\x00" as *const u8 as *const libc::c_char,
            );
        }
        trap_Print(va(
            b"%-16s %-16s %-20s %-20s\n\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            name.as_mut_ptr(),
            model.as_mut_ptr(),
            aifile.as_mut_ptr(),
            funname.as_mut_ptr(),
        ));
        i += 1
    }
}
/*
===============
G_SpawnBots
===============
*/

unsafe extern "C" fn G_SpawnBots(mut botList: *mut libc::c_char, mut baseDelay: i32) {
    let mut bot: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut skill: f32 = 0.;
    let mut delay: i32 = 0;
    let mut bots: [libc::c_char; 1024] = [0; 1024];
    podium1 = 0 as *mut gentity_t;
    podium2 = 0 as *mut gentity_t;
    podium3 = 0 as *mut gentity_t;
    skill = trap_Cvar_VariableValue(b"g_spSkill\x00" as *const u8 as *const libc::c_char);
    if skill < 1 as i32 as f32 {
        trap_Cvar_Set(
            b"g_spSkill\x00" as *const u8 as *const libc::c_char,
            b"1\x00" as *const u8 as *const libc::c_char,
        );
        skill = 1 as i32 as f32
    } else if skill > 5 as i32 as f32 {
        trap_Cvar_Set(
            b"g_spSkill\x00" as *const u8 as *const libc::c_char,
            b"5\x00" as *const u8 as *const libc::c_char,
        );
        skill = 5 as i32 as f32
    }
    Q_strncpyz(
        bots.as_mut_ptr(),
        botList,
        ::std::mem::size_of::<[libc::c_char; 1024]>() as usize as i32,
    );
    p = &mut *bots.as_mut_ptr().offset(0 as i32 as isize) as *mut libc::c_char;
    delay = baseDelay;
    while *p != 0 {
        //skip spaces
        while *p as i32 != 0 && *p as i32 == ' ' as i32 {
            p = p.offset(1)
        }
        if *p == 0 {
            break;
        }
        // mark start of bot name
        bot = p;
        // skip until space of null
        while *p as i32 != 0 && *p as i32 != ' ' as i32 {
            p = p.offset(1)
        }
        if *p != 0 {
            let fresh3 = p;
            p = p.offset(1);
            *fresh3 = 0 as i32 as libc::c_char
        }
        // we must add the bot this way, calling G_AddBot directly at this stage
        // does "Bad Things"
        trap_SendConsoleCommand(
            EXEC_INSERT as i32,
            va(
                b"addbot %s %f free %i\n\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                bot,
                skill as f64,
                delay,
            ),
        );
        delay += 1500 as i32
    }
}
/*
===============
G_LoadBotsFromFile
===============
*/

unsafe extern "C" fn G_LoadBotsFromFile(mut filename: *mut libc::c_char) {
    let mut len: i32 = 0;
    let mut f: fileHandle_t = 0;
    let mut buf: [libc::c_char; 8192] = [0; 8192];
    len = trap_FS_FOpenFile(filename, &mut f, FS_READ);
    if f == 0 {
        trap_Print(va(
            b"^1file not found: %s\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            filename,
        ));
        return;
    }
    if len >= 8192 as i32 {
        trap_Print(va(
            b"^1file too large: %s is %i, max allowed is %i\n\x00" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            filename,
            len,
            8192 as i32,
        ));
        trap_FS_FCloseFile(f);
        return;
    }
    trap_FS_Read(buf.as_mut_ptr() as *mut libc::c_void, len, f);
    buf[len as usize] = 0 as i32 as libc::c_char;
    trap_FS_FCloseFile(f);
    g_numBots += G_ParseInfos(
        buf.as_mut_ptr(),
        1024 as i32 - g_numBots,
        &mut *g_botInfos.as_mut_ptr().offset(g_numBots as isize),
    );
}
/*
===============
G_LoadBots
===============
*/

unsafe extern "C" fn G_LoadBots() {
    let mut botsFile: vmCvar_t = vmCvar_t {
        handle: 0,
        modificationCount: 0,
        value: 0.,
        integer: 0,
        string: [0; 256],
    };
    let mut numdirs: i32 = 0;
    let mut filename: [libc::c_char; 128] = [0; 128];
    let mut dirlist: [libc::c_char; 1024] = [0; 1024];
    let mut dirptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: i32 = 0;
    let mut dirlen: i32 = 0;
    if trap_Cvar_VariableIntegerValue(b"bot_enable\x00" as *const u8 as *const libc::c_char) == 0 {
        return;
    }
    g_numBots = 0 as i32;
    trap_Cvar_Register(
        &mut botsFile as *mut _ as *mut vmCvar_t,
        b"g_botsFile\x00" as *const u8 as *const libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char,
        0x10 as i32 | 0x40 as i32,
    );
    if *botsFile.string.as_mut_ptr() != 0 {
        G_LoadBotsFromFile(botsFile.string.as_mut_ptr());
    } else {
        G_LoadBotsFromFile(
            b"scripts/bots.txt\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    // get all bots from .bot files
    numdirs = trap_FS_GetFileList(
        b"scripts\x00" as *const u8 as *const libc::c_char,
        b".bot\x00" as *const u8 as *const libc::c_char,
        dirlist.as_mut_ptr(),
        1024 as i32,
    );
    dirptr = dirlist.as_mut_ptr();
    i = 0 as i32;
    while i < numdirs {
        dirlen = crate::stdlib::strlen(dirptr) as i32;
        libc::strcpy(
            filename.as_mut_ptr(),
            b"scripts/\x00" as *const u8 as *const libc::c_char,
        );
        libc::strcat(filename.as_mut_ptr(), dirptr);
        G_LoadBotsFromFile(filename.as_mut_ptr());
        i += 1;
        dirptr = dirptr.offset((dirlen + 1 as i32) as isize)
    }
    trap_Print(va(
        b"%i bots parsed\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        g_numBots,
    ));
}
/*
===============
G_GetBotInfoByNumber
===============
*/
#[no_mangle]

pub unsafe extern "C" fn G_GetBotInfoByNumber(mut num: i32) -> *mut libc::c_char {
    if num < 0 as i32 || num >= g_numBots {
        trap_Print(va(
            b"^1Invalid bot number: %i\n\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            num,
        ));
        return 0 as *mut libc::c_char;
    }
    return g_botInfos[num as usize];
}
/*
===============
G_GetBotInfoByName
===============
*/
#[no_mangle]

pub unsafe extern "C" fn G_GetBotInfoByName(mut name: *const libc::c_char) -> *mut libc::c_char {
    let mut n: i32 = 0;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    n = 0 as i32;
    while n < g_numBots {
        value = Info_ValueForKey(
            g_botInfos[n as usize],
            b"name\x00" as *const u8 as *const libc::c_char,
        );
        if Q_stricmp(value, name) == 0 {
            return g_botInfos[n as usize];
        }
        n += 1
    }
    return 0 as *mut libc::c_char;
}
/*
===============
G_InitBots
===============
*/
#[no_mangle]

pub unsafe extern "C" fn G_InitBots(mut restart: qboolean) {
    let mut fragLimit: i32 = 0;
    let mut timeLimit: i32 = 0;
    let mut arenainfo: *const libc::c_char = 0 as *const libc::c_char;
    let mut strValue: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut basedelay: i32 = 0;
    let mut map: [libc::c_char; 64] = [0; 64];
    let mut serverinfo: [libc::c_char; 1024] = [0; 1024];
    G_LoadBots();
    G_LoadArenas();
    trap_Cvar_Register(
        &mut bot_minplayers as *mut _ as *mut vmCvar_t,
        b"bot_minplayers\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x4 as i32,
    );
    if g_gametype.integer == GT_SINGLE_PLAYER as i32 {
        trap_GetServerinfo(
            serverinfo.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as usize as i32,
        );
        Q_strncpyz(
            map.as_mut_ptr(),
            Info_ValueForKey(
                serverinfo.as_mut_ptr(),
                b"mapname\x00" as *const u8 as *const libc::c_char,
            ),
            ::std::mem::size_of::<[libc::c_char; 64]>() as usize as i32,
        );
        arenainfo = G_GetArenaInfoByMap(map.as_mut_ptr());
        if arenainfo.is_null() {
            return;
        }
        strValue = Info_ValueForKey(
            arenainfo,
            b"fraglimit\x00" as *const u8 as *const libc::c_char,
        );
        fragLimit = atoi(strValue);
        if fragLimit != 0 {
            trap_Cvar_Set(
                b"fraglimit\x00" as *const u8 as *const libc::c_char,
                strValue,
            );
        } else {
            trap_Cvar_Set(
                b"fraglimit\x00" as *const u8 as *const libc::c_char,
                b"0\x00" as *const u8 as *const libc::c_char,
            );
        }
        strValue = Info_ValueForKey(
            arenainfo,
            b"timelimit\x00" as *const u8 as *const libc::c_char,
        );
        timeLimit = atoi(strValue);
        if timeLimit != 0 {
            trap_Cvar_Set(
                b"timelimit\x00" as *const u8 as *const libc::c_char,
                strValue,
            );
        } else {
            trap_Cvar_Set(
                b"timelimit\x00" as *const u8 as *const libc::c_char,
                b"0\x00" as *const u8 as *const libc::c_char,
            );
        }
        if fragLimit == 0 && timeLimit == 0 {
            trap_Cvar_Set(
                b"fraglimit\x00" as *const u8 as *const libc::c_char,
                b"10\x00" as *const u8 as *const libc::c_char,
            );
            trap_Cvar_Set(
                b"timelimit\x00" as *const u8 as *const libc::c_char,
                b"0\x00" as *const u8 as *const libc::c_char,
            );
        }
        basedelay = 2000 as i32;
        strValue = Info_ValueForKey(
            arenainfo,
            b"special\x00" as *const u8 as *const libc::c_char,
        );
        if Q_stricmp(
            strValue,
            b"training\x00" as *const u8 as *const libc::c_char,
        ) == 0 as i32
        {
            basedelay += 10000 as i32
        }
        if restart as u64 == 0 {
            G_SpawnBots(
                Info_ValueForKey(arenainfo, b"bots\x00" as *const u8 as *const libc::c_char),
                basedelay,
            );
        }
    };
}

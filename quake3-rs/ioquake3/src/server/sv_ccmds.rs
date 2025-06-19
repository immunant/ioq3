use ::libc;

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

pub use crate::src::server::sv_ccmds::stdlib_h::atoi;
pub use crate::stddef_h::size_t;
pub use crate::stdlib::__compar_fn_t;
pub use crate::stdlib::intptr_t;
pub use crate::stdlib::qsort;
pub use ::libc::strtol;

pub use crate::be_aas_h::C2RustUnnamed_0;
pub use crate::bg_public_h::GT_1FCTF;
pub use crate::bg_public_h::GT_CTF;
pub use crate::bg_public_h::GT_FFA;
pub use crate::bg_public_h::GT_HARVESTER;
pub use crate::bg_public_h::GT_MAX_GAME_TYPE;
pub use crate::bg_public_h::GT_OBELISK;
pub use crate::bg_public_h::GT_SINGLE_PLAYER;
pub use crate::bg_public_h::GT_TEAM;
pub use crate::bg_public_h::GT_TOURNAMENT;
pub use crate::g_public_h::entityShared_t;
pub use crate::g_public_h::sharedEntity_t;
pub use crate::g_public_h::BOTAI_START_FRAME;
pub use crate::g_public_h::GAME_CLIENT_BEGIN;
pub use crate::g_public_h::GAME_CLIENT_COMMAND;
pub use crate::g_public_h::GAME_CLIENT_CONNECT;
pub use crate::g_public_h::GAME_CLIENT_DISCONNECT;
pub use crate::g_public_h::GAME_CLIENT_THINK;
pub use crate::g_public_h::GAME_CLIENT_USERINFO_CHANGED;
pub use crate::g_public_h::GAME_CONSOLE_COMMAND;
pub use crate::g_public_h::GAME_INIT;
pub use crate::g_public_h::GAME_RUN_FRAME;
pub use crate::g_public_h::GAME_SHUTDOWN;
pub use crate::qcommon_h::completionFunc_t;
pub use crate::qcommon_h::msg_t;
pub use crate::qcommon_h::netadr_t;
pub use crate::qcommon_h::netadrtype_t;
pub use crate::qcommon_h::netchan_t;
pub use crate::qcommon_h::netsrc_t;
pub use crate::qcommon_h::vm_t;
pub use crate::qcommon_h::xcommand_t;
pub use crate::qcommon_h::NA_BAD;
pub use crate::qcommon_h::NA_BOT;
pub use crate::qcommon_h::NA_BROADCAST;
pub use crate::qcommon_h::NA_IP;
pub use crate::qcommon_h::NA_IP6;
pub use crate::qcommon_h::NA_LOOPBACK;
pub use crate::qcommon_h::NA_MULTICAST6;
pub use crate::qcommon_h::NA_UNSPEC;
pub use crate::qcommon_h::NS_CLIENT;
pub use crate::qcommon_h::NS_SERVER;
pub use crate::server_h::challenge_t;
pub use crate::server_h::clientSnapshot_t;
pub use crate::server_h::clientState_t;
pub use crate::server_h::client_s;
pub use crate::server_h::client_t;
pub use crate::server_h::netchan_buffer_s;
pub use crate::server_h::netchan_buffer_t;
pub use crate::server_h::serverBan_t;
pub use crate::server_h::serverState_t;
pub use crate::server_h::serverStatic_t;
pub use crate::server_h::server_t;
pub use crate::server_h::svEntity_s;
pub use crate::server_h::svEntity_t;
pub use crate::server_h::voipServerPacket_s;
pub use crate::server_h::voipServerPacket_t;
pub use crate::server_h::worldSector_s;
pub use crate::server_h::CS_ACTIVE;
pub use crate::server_h::CS_CONNECTED;
pub use crate::server_h::CS_FREE;
pub use crate::server_h::CS_PRIMED;
pub use crate::server_h::CS_ZOMBIE;
pub use crate::server_h::SS_DEAD;
pub use crate::server_h::SS_GAME;
pub use crate::server_h::SS_LOADING;
pub use crate::src::qcommon::cmd::Cmd_AddCommand;
pub use crate::src::qcommon::cmd::Cmd_Argc;
pub use crate::src::qcommon::cmd::Cmd_Args;
pub use crate::src::qcommon::cmd::Cmd_ArgsFrom;
pub use crate::src::qcommon::cmd::Cmd_Argv;
pub use crate::src::qcommon::cmd::Cmd_SetCommandCompletionFunc;
pub use crate::src::qcommon::common::com_dedicated;
pub use crate::src::qcommon::common::com_frameTime;
pub use crate::src::qcommon::common::com_standalone;
pub use crate::src::qcommon::common::com_sv_running;
pub use crate::src::qcommon::common::Com_FieldStringToPlayerName;
pub use crate::src::qcommon::common::Com_Printf;
pub use crate::src::qcommon::common::Com_strCompare;
pub use crate::src::qcommon::common::Field_CompleteFilename;
pub use crate::src::qcommon::common::Field_CompletePlayerName;
pub use crate::src::qcommon::common::Info_Print;
pub use crate::src::qcommon::common::Z_Free;
pub use crate::src::qcommon::common::Z_Malloc;
pub use crate::src::qcommon::cvar::Cvar_Get;
pub use crate::src::qcommon::cvar::Cvar_InfoString;
pub use crate::src::qcommon::cvar::Cvar_InfoString_Big;
pub use crate::src::qcommon::cvar::Cvar_Set;
pub use crate::src::qcommon::cvar::Cvar_SetLatched;
pub use crate::src::qcommon::cvar::Cvar_SetValue;
pub use crate::src::qcommon::cvar::Cvar_VariableString;
pub use crate::src::qcommon::cvar::Cvar_VariableValue;
pub use crate::src::qcommon::files::FS_FCloseFile;
pub use crate::src::qcommon::files::FS_GetCurrentGameDir;
pub use crate::src::qcommon::files::FS_Read;
pub use crate::src::qcommon::files::FS_ReadFile;
pub use crate::src::qcommon::files::FS_SV_FOpenFileRead;
pub use crate::src::qcommon::files::FS_SV_FOpenFileWrite;
pub use crate::src::qcommon::files::FS_Write;
pub use crate::src::qcommon::net_chan::NET_OutOfBandPrint;
pub use crate::src::qcommon::net_chan::NET_StringToAdr;
pub use crate::src::qcommon::net_ip::NET_AdrToString;
pub use crate::src::qcommon::net_ip::NET_CompareBaseAdrMask;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::cvar_s;
pub use crate::src::qcommon::q_shared::cvar_t;
pub use crate::src::qcommon::q_shared::entityState_s;
pub use crate::src::qcommon::q_shared::entityState_t;
pub use crate::src::qcommon::q_shared::fileHandle_t;
pub use crate::src::qcommon::q_shared::playerState_s;
pub use crate::src::qcommon::q_shared::playerState_t;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::trType_t;
pub use crate::src::qcommon::q_shared::trajectory_t;
pub use crate::src::qcommon::q_shared::usercmd_s;
pub use crate::src::qcommon::q_shared::usercmd_t;
pub use crate::src::qcommon::q_shared::va;
pub use crate::src::qcommon::q_shared::vec3_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::qcommon::q_shared::Com_sprintf;
pub use crate::src::qcommon::q_shared::Q_CleanStr;
pub use crate::src::qcommon::q_shared::Q_IsColorString;
pub use crate::src::qcommon::q_shared::Q_stricmp;
pub use crate::src::qcommon::q_shared::Q_stricmpn;
pub use crate::src::qcommon::q_shared::Q_strncpyz;

pub use crate::src::qcommon::q_shared::TR_GRAVITY;
pub use crate::src::qcommon::q_shared::TR_INTERPOLATE;
pub use crate::src::qcommon::q_shared::TR_LINEAR;
pub use crate::src::qcommon::q_shared::TR_LINEAR_STOP;
pub use crate::src::qcommon::q_shared::TR_SINE;
pub use crate::src::qcommon::q_shared::TR_STATIONARY;
pub use crate::src::qcommon::vm::VM_Call;
pub use crate::src::qcommon::vm::VM_ExplicitArgPtr;
pub use crate::src::server::sv_client::SV_ClientEnterWorld;
pub use crate::src::server::sv_client::SV_DropClient;
pub use crate::src::server::sv_game::SV_GameClientNum;
pub use crate::src::server::sv_game::SV_RestartGameProgs;
pub use crate::src::server::sv_init::SV_SetConfigstring;
pub use crate::src::server::sv_init::SV_Shutdown;
pub use crate::src::server::sv_init::SV_SpawnServer;
pub use crate::src::server::sv_main::gvm;
pub use crate::src::server::sv_main::serverBans;
pub use crate::src::server::sv_main::serverBansCount;
pub use crate::src::server::sv_main::sv;
pub use crate::src::server::sv_main::sv_banFile;
pub use crate::src::server::sv_main::sv_gametype;
pub use crate::src::server::sv_main::sv_mapname;
pub use crate::src::server::sv_main::sv_maxclients;
pub use crate::src::server::sv_main::svs;
pub use crate::src::server::sv_main::SV_AddServerCommand;
pub use crate::src::server::sv_main::SV_SendServerCommand;
pub use crate::src::server::sv_world::SV_SectorList_f;

pub use crate::vm_local_h::vm_s;

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
/*
===============================================================================

OPERATOR CONSOLE ONLY COMMANDS

These commands can only be entered from stdin or by a remote operator datagram
===============================================================================
*/
/*
==================
SV_GetPlayerByHandle

Returns the player with player id or name from Cmd_Argv(1)
==================
*/

unsafe extern "C" fn SV_GetPlayerByHandle() -> *mut client_t {
    let mut cl: *mut client_t = 0 as *mut client_t;
    let mut i: i32 = 0;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cleanName: [libc::c_char; 64] = [0; 64];
    // make sure server is running
    if (*com_sv_running).integer == 0 {
        return 0 as *mut client_t;
    }
    if Cmd_Argc() < 2 as i32 {
        Com_Printf(
            b"No player specified.\n\x00" as *const u8 as *const libc::c_char,
        );
        return 0 as *mut client_t;
    }
    s = Cmd_Argv(1 as i32);
    // Check whether this is a numeric player handle
    i = 0 as i32;
    while *s.offset(i as isize) as i32 >= '0' as i32 && *s.offset(i as isize) as i32 <= '9' as i32 {
        i += 1
    }
    if *s.offset(i as isize) == 0 {
        let mut plid: i32 = atoi(s);
        // Check for numeric playerid match
        if plid >= 0 as i32 && plid < (*sv_maxclients).integer {
            cl = &mut *svs
                .clients
                .offset(plid as isize) as *mut client_t;
            if (*cl).state as u64 != 0 {
                return cl;
            }
        }
    }
    // check for a name match
    i = 0 as i32;
    cl = svs.clients;
    while i < (*sv_maxclients).integer {
        if !((*cl).state as u64 == 0) {
            if Q_stricmp((*cl).name.as_mut_ptr(), s) == 0 {
                return cl;
            }
            Q_strncpyz(
                cleanName.as_mut_ptr(),
                (*cl).name.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as i32,
            );
            Q_CleanStr(cleanName.as_mut_ptr());
            if Q_stricmp(cleanName.as_mut_ptr(), s) == 0 {
                return cl;
            }
        }
        i += 1;
        cl = cl.offset(1)
    }
    Com_Printf(
        b"Player %s is not on the server\n\x00" as *const u8 as *const libc::c_char,
        s,
    );
    return 0 as *mut client_t;
}
/*
==================
SV_GetPlayerByNum

Returns the player with idnum from Cmd_Argv(1)
==================
*/

unsafe extern "C" fn SV_GetPlayerByNum() -> *mut client_t {
    let mut cl: *mut client_t = 0 as *mut client_t;
    let mut i: i32 = 0;
    let mut idnum: i32 = 0;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    // make sure server is running
    if (*com_sv_running).integer == 0 {
        return 0 as *mut client_t;
    }
    if Cmd_Argc() < 2 as i32 {
        Com_Printf(
            b"No player specified.\n\x00" as *const u8 as *const libc::c_char,
        );
        return 0 as *mut client_t;
    }
    s = Cmd_Argv(1 as i32);
    i = 0 as i32;
    while *s.offset(i as isize) != 0 {
        if (*s.offset(i as isize) as i32) < '0' as i32 || *s.offset(i as isize) as i32 > '9' as i32
        {
            Com_Printf(
                b"Bad slot number: %s\n\x00" as *const u8 as *const libc::c_char,
                s,
            );
            return 0 as *mut client_t;
        }
        i += 1
    }
    idnum = atoi(s);
    if idnum < 0 as i32 || idnum >= (*sv_maxclients).integer {
        Com_Printf(
            b"Bad client slot: %i\n\x00" as *const u8 as *const libc::c_char,
            idnum,
        );
        return 0 as *mut client_t;
    }
    cl = &mut *svs
        .clients
        .offset(idnum as isize) as *mut client_t;
    if (*cl).state as u64 == 0 {
        Com_Printf(
            b"Client %i is not active\n\x00" as *const u8 as *const libc::c_char,
            idnum,
        );
        return 0 as *mut client_t;
    }
    return cl;
}
//=========================================================
/*
==================
SV_Map_f

Restart the server on a different map
==================
*/

unsafe extern "C" fn SV_Map_f() {
    let mut cmd: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut map: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut killBots: qboolean =
        qfalse;
    let mut cheat: qboolean = qfalse;
    let mut expanded: [libc::c_char; 64] = [0; 64];
    let mut mapname: [libc::c_char; 64] = [0; 64];
    map = Cmd_Argv(1 as i32);
    if map.is_null() {
        return;
    }
    // make sure the level exists before trying to change, so that
    // a typo at the server console won't end the game
    Com_sprintf(
        expanded.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as i32,
        b"maps/%s.bsp\x00" as *const u8 as *const libc::c_char,
        map,
    );
    if FS_ReadFile(expanded.as_mut_ptr(), 0 as *mut *mut libc::c_void)
        == -(1 as i32) as isize
    {
        Com_Printf(
            b"Can\'t find map %s\n\x00" as *const u8 as *const libc::c_char,
            expanded.as_mut_ptr(),
        );
        return;
    }
    // force latched values to get set

    Cvar_Get(
        b"g_gametype\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x4 as i32 | 0x2 as i32 | 0x20 as i32,
    ) as *mut cvar_s;
    cmd = Cmd_Argv(0 as i32);
    if Q_stricmpn(
        cmd,
        b"sp\x00" as *const u8 as *const libc::c_char,
        2 as i32,
    ) == 0 as i32
    {
        Cvar_SetValue(
            b"g_gametype\x00" as *const u8 as *const libc::c_char,
            GT_SINGLE_PLAYER as i32 as f32,
        );
        Cvar_SetValue(
            b"g_doWarmup\x00" as *const u8 as *const libc::c_char,
            0 as i32 as f32,
        );
        // may not set sv_maxclients directly, always set latched
        Cvar_SetLatched(
            b"sv_maxclients\x00" as *const u8 as *const libc::c_char,
            b"8\x00" as *const u8 as *const libc::c_char,
        );
        cmd = cmd.offset(2 as i32 as isize);
        if Q_stricmp(
            cmd,
            b"devmap\x00" as *const u8 as *const libc::c_char,
        ) == 0
        {
            cheat = qtrue
        } else {
            cheat = qfalse
        }
        killBots = qtrue
    } else {
        if Q_stricmp(
            cmd,
            b"devmap\x00" as *const u8 as *const libc::c_char,
        ) == 0
        {
            cheat = qtrue;
            killBots = qtrue
        } else {
            cheat = qfalse;
            killBots = qfalse
        }
        if (*sv_gametype).integer
            == GT_SINGLE_PLAYER as i32
        {
            Cvar_SetValue(
                b"g_gametype\x00" as *const u8 as *const libc::c_char,
                GT_FFA as i32 as f32,
            );
        }
    }
    // save the map name here cause on a map restart we reload the q3config.cfg
    // and thus nuke the arguments of the map command
    Q_strncpyz(
        mapname.as_mut_ptr(),
        map,
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as i32,
    );
    // start up the map
    SV_SpawnServer(mapname.as_mut_ptr(), killBots);
    // set the cheat value
    // if the level was started with "map <levelname>", then
    // cheats will not be allowed.  If started with "devmap <levelname>"
    // then cheats will be allowed
    if cheat as u64 != 0 {
        Cvar_Set(
            b"sv_cheats\x00" as *const u8 as *const libc::c_char,
            b"1\x00" as *const u8 as *const libc::c_char,
        );
    } else {
        Cvar_Set(
            b"sv_cheats\x00" as *const u8 as *const libc::c_char,
            b"0\x00" as *const u8 as *const libc::c_char,
        );
    };
}
/*
================
SV_MapRestart_f

Completely restarts a level, but doesn't send a new gamestate to the clients.
This allows fair starts with variable load times.
================
*/

unsafe extern "C" fn SV_MapRestart_f() {
    let mut i: i32 = 0;
    let mut client: *mut client_t = 0 as *mut client_t;
    let mut denied: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut isBot: qboolean = qfalse;
    let mut delay: i32 = 0;
    // make sure we aren't restarting twice in the same frame
    if com_frameTime == sv.serverId {
        return;
    }
    // make sure server is running
    if (*com_sv_running).integer == 0 {
        Com_Printf(
            b"Server is not running.\n\x00" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if sv.restartTime != 0 {
        return;
    }
    if Cmd_Argc() > 1 as i32 {
        delay = atoi(Cmd_Argv(1 as i32))
    } else {
        delay = 5 as i32
    }
    if delay != 0
        && Cvar_VariableValue(
            b"g_doWarmup\x00" as *const u8 as *const libc::c_char,
        ) == 0.
    {
        sv.restartTime =
            sv.time + delay * 1000 as i32;
        SV_SetConfigstring(
            5 as i32,
            va(
                b"%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                sv.restartTime,
            ),
        );
        return;
    }
    // check for changes in variables that can't just be restarted
    // check for maxclients change
    if (*sv_maxclients).modified as u32 != 0
        || (*sv_gametype).modified as u32 != 0
    {
        let mut mapname: [libc::c_char; 64] = [0; 64];
        Com_Printf(
            b"variable change -- restarting.\n\x00" as *const u8 as *const libc::c_char,
        );
        // restart the map the slow way
        Q_strncpyz(
            mapname.as_mut_ptr(),
            Cvar_VariableString(
                b"mapname\x00" as *const u8 as *const libc::c_char,
            ),
            ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as i32,
        );
        SV_SpawnServer(
            mapname.as_mut_ptr(),
            qfalse,
        );
        return;
    }
    // toggle the server bit so clients can detect that a
    // map_restart has happened
    svs.snapFlagServerBit ^= 4 as i32;
    // generate a new serverid
    // TTimo - don't update restartedserverId there, otherwise we won't deal correctly with multiple map_restart
    sv.serverId = com_frameTime;
    Cvar_Set(
        b"sv_serverid\x00" as *const u8 as *const libc::c_char,
        va(
            b"%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            sv.serverId,
        ),
    );
    // if a map_restart occurs while a client is changing maps, we need
    // to give them the correct time so that when they finish loading
    // they don't violate the backwards time check in cl_cgame.c
    i = 0 as i32;
    while i < (*sv_maxclients).integer {
        if (*svs.clients.offset(i as isize)).state as u32
            == CS_PRIMED as i32 as u32
        {
            (*svs.clients.offset(i as isize)).oldServerTime =
                sv.restartTime
        }
        i += 1
    }
    // reset all the vm data in place without changing memory allocation
    // note that we do NOT set sv.state = SS_LOADING, so configstrings that
    // had been changed from their default values will generate broadcast updates
    sv.state = SS_LOADING;
    sv.restarting = qtrue;
    SV_RestartGameProgs();
    // run a few frames to allow everything to settle
    i = 0 as i32;
    while i < 3 as i32 {
        VM_Call(
            gvm,
            GAME_RUN_FRAME as i32,
            sv.time,
        );
        sv.time += 100 as i32;
        svs.time += 100 as i32;
        i += 1
    }
    sv.state = SS_GAME;
    sv.restarting = qfalse;
    // connect and begin all the clients
    i = 0 as i32;
    while i < (*sv_maxclients).integer {
        client = &mut *svs.clients.offset(i as isize)
            as *mut client_t;
        // send the new gamestate to all connected clients
        if !(((*client).state as u32) < CS_CONNECTED as i32 as u32) {
            if (*client).netchan.remoteAddress.type_0 as u32
                == NA_BOT as i32 as u32
            {
                isBot = qtrue
            } else {
                isBot = qfalse
            }
            // add the map_restart command
            SV_AddServerCommand(
                client as *mut client_s,
                b"map_restart\n\x00" as *const u8 as *const libc::c_char,
            );
            // connect the client again, without the firstTime flag
            denied = VM_ExplicitArgPtr(
                gvm,
                VM_Call(
                    gvm,
                    GAME_CLIENT_CONNECT as i32,
                    i,
                    qfalse as i32,
                    isBot as u32,
                ),
            ) as *mut libc::c_char;
            if !denied.is_null() {
                // this generally shouldn't happen, because the client
                // was connected before the level change
                SV_DropClient(
                    client as *mut client_s,
                    denied,
                );
                Com_Printf(
                    b"SV_MapRestart_f(%d): dropped client %i - denied!\n\x00" as *const u8
                        as *const libc::c_char,
                    delay,
                    i,
                );
            } else if (*client).state as u32 == CS_ACTIVE as i32 as u32 {
                SV_ClientEnterWorld(
                    client as *mut client_s,
                    &mut (*client).lastUsercmd as *mut _
                        as *mut usercmd_s,
                );
            } else {
                // If we don't reset client->lastUsercmd and are restarting during map load,
                // the client will hang because we'll use the last Usercmd from the previous map,
                // which is wrong obviously.
                SV_ClientEnterWorld(
                    client as *mut client_s,
                    0 as *mut usercmd_t
                        as *mut usercmd_s,
                );
            }
        }
        i += 1
    }
    // run another frame to allow things to look at all the players
    VM_Call(
        gvm,
        GAME_RUN_FRAME as i32,
        sv.time,
    );
    sv.time += 100 as i32;
    svs.time += 100 as i32;
}
//===============================================================
/*
==================
SV_Kick_f

Kick a user off of the server
==================
*/

unsafe extern "C" fn SV_Kick_f() {
    let mut cl: *mut client_t = 0 as *mut client_t;
    let mut i: i32 = 0;
    // make sure server is running
    if (*com_sv_running).integer == 0 {
        Com_Printf(
            b"Server is not running.\n\x00" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if Cmd_Argc() != 2 as i32 {
        Com_Printf(b"Usage: kick <player name>\nkick all = kick everyone\nkick allbots = kick all bots\n\x00"
                       as *const u8 as *const libc::c_char);
        return;
    }
    cl = SV_GetPlayerByHandle();
    if cl.is_null() {
        if Q_stricmp(
            Cmd_Argv(1 as i32),
            b"all\x00" as *const u8 as *const libc::c_char,
        ) == 0
        {
            i = 0 as i32;
            cl = svs.clients;
            while i < (*sv_maxclients).integer {
                if !((*cl).state as u64 == 0) {
                    if !((*cl).netchan.remoteAddress.type_0 as u32
                        == NA_LOOPBACK as i32 as u32)
                    {
                        SV_DropClient(
                            cl as *mut client_s,
                            b"was kicked\x00" as *const u8 as *const libc::c_char,
                        );
                        (*cl).lastPacketTime = svs.time
                    }
                }
                i += 1;
                cl = cl.offset(1)
                // in case there is a funny zombie
            }
        } else if Q_stricmp(
            Cmd_Argv(1 as i32),
            b"allbots\x00" as *const u8 as *const libc::c_char,
        ) == 0
        {
            i = 0 as i32;
            cl = svs.clients;
            while i < (*sv_maxclients).integer {
                if !((*cl).state as u64 == 0) {
                    if !((*cl).netchan.remoteAddress.type_0 as u32
                        != NA_BOT as i32 as u32)
                    {
                        SV_DropClient(
                            cl as *mut client_s,
                            b"was kicked\x00" as *const u8 as *const libc::c_char,
                        );
                        (*cl).lastPacketTime = svs.time
                    }
                }
                i += 1;
                cl = cl.offset(1)
                // in case there is a funny zombie
            }
        }
        return;
    }
    if (*cl).netchan.remoteAddress.type_0 as u32 == NA_LOOPBACK as i32 as u32 {
        Com_Printf(
            b"Cannot kick host player\n\x00" as *const u8 as *const libc::c_char,
        );
        return;
    }
    SV_DropClient(
        cl as *mut client_s,
        b"was kicked\x00" as *const u8 as *const libc::c_char,
    );
    (*cl).lastPacketTime = svs.time;
    // in case there is a funny zombie
}
/*
==================
SV_KickBots_f

Kick all bots off of the server
==================
*/

unsafe extern "C" fn SV_KickBots_f() {
    let mut cl: *mut client_t = 0 as *mut client_t;
    let mut i: i32 = 0;
    // make sure server is running
    if (*com_sv_running).integer == 0 {
        Com_Printf(
            b"Server is not running.\n\x00" as *const u8 as *const libc::c_char,
        );
        return;
    }
    i = 0 as i32;
    cl = svs.clients;
    while i < (*sv_maxclients).integer {
        if !((*cl).state as u64 == 0) {
            if !((*cl).netchan.remoteAddress.type_0 as u32
                != NA_BOT as i32 as u32)
            {
                SV_DropClient(
                    cl as *mut client_s,
                    b"was kicked\x00" as *const u8 as *const libc::c_char,
                );
                (*cl).lastPacketTime = svs.time
            }
        }
        i += 1;
        cl = cl.offset(1)
        // in case there is a funny zombie
    }
}
/*
==================
SV_KickAll_f

Kick all users off of the server
==================
*/

unsafe extern "C" fn SV_KickAll_f() {
    let mut cl: *mut client_t = 0 as *mut client_t;
    let mut i: i32 = 0;
    // make sure server is running
    if (*com_sv_running).integer == 0 {
        Com_Printf(
            b"Server is not running.\n\x00" as *const u8 as *const libc::c_char,
        );
        return;
    }
    i = 0 as i32;
    cl = svs.clients;
    while i < (*sv_maxclients).integer {
        if !((*cl).state as u64 == 0) {
            if !((*cl).netchan.remoteAddress.type_0 as u32
                == NA_LOOPBACK as i32 as u32)
            {
                SV_DropClient(
                    cl as *mut client_s,
                    b"was kicked\x00" as *const u8 as *const libc::c_char,
                );
                (*cl).lastPacketTime = svs.time
            }
        }
        i += 1;
        cl = cl.offset(1)
        // in case there is a funny zombie
    }
}
/*
==================
SV_KickNum_f

Kick a user off of the server
==================
*/

unsafe extern "C" fn SV_KickNum_f() {
    let mut cl: *mut client_t = 0 as *mut client_t;
    // make sure server is running
    if (*com_sv_running).integer == 0 {
        Com_Printf(
            b"Server is not running.\n\x00" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if Cmd_Argc() != 2 as i32 {
        Com_Printf(
            b"Usage: %s <client number>\n\x00" as *const u8 as *const libc::c_char,
            Cmd_Argv(0 as i32),
        );
        return;
    }
    cl = SV_GetPlayerByNum();
    if cl.is_null() {
        return;
    }
    if (*cl).netchan.remoteAddress.type_0 as u32 == NA_LOOPBACK as i32 as u32 {
        Com_Printf(
            b"Cannot kick host player\n\x00" as *const u8 as *const libc::c_char,
        );
        return;
    }
    SV_DropClient(
        cl as *mut client_s,
        b"was kicked\x00" as *const u8 as *const libc::c_char,
    );
    (*cl).lastPacketTime = svs.time;
    // in case there is a funny zombie
}
// these functions require the auth server which of course is not available anymore for stand-alone games.
/*
==================
SV_Ban_f

Ban a user from being able to play on this server through the auth
server
==================
*/

unsafe extern "C" fn SV_Ban_f() {
    let mut cl: *mut client_t = 0 as *mut client_t;
    // make sure server is running
    if (*com_sv_running).integer == 0 {
        Com_Printf(
            b"Server is not running.\n\x00" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if Cmd_Argc() != 2 as i32 {
        Com_Printf(
            b"Usage: banUser <player name>\n\x00" as *const u8 as *const libc::c_char,
        );
        return;
    }
    cl = SV_GetPlayerByHandle();
    if cl.is_null() {
        return;
    }
    if (*cl).netchan.remoteAddress.type_0 as u32 == NA_LOOPBACK as i32 as u32 {
        Com_Printf(
            b"Cannot kick host player\n\x00" as *const u8 as *const libc::c_char,
        );
        return;
    }
    // look up the authorize server's IP
    if svs.authorizeAddress.ip[0 as i32 as usize] == 0
        && svs.authorizeAddress.type_0 as u32
            != NA_BAD as i32 as u32
    {
        Com_Printf(
            b"Resolving %s\n\x00" as *const u8 as *const libc::c_char,
            b"authorize.quake3arena.com\x00" as *const u8 as *const libc::c_char,
        );
        if NET_StringToAdr(
            b"authorize.quake3arena.com\x00" as *const u8 as *const libc::c_char,
            &mut svs.authorizeAddress as *mut _
                as *mut netadr_t,
            NA_IP,
        ) == 0
        {
            Com_Printf(
                b"Couldn\'t resolve address\n\x00" as *const u8 as *const libc::c_char,
            );
            return;
        }
        svs.authorizeAddress.port =
            crate::src::qcommon::q_shared::ShortSwap(27952 as i32 as i16) as u16;
        Com_Printf(
            b"%s resolved to %i.%i.%i.%i:%i\n\x00" as *const u8 as *const libc::c_char,
            b"authorize.quake3arena.com\x00" as *const u8 as *const libc::c_char,
            svs.authorizeAddress.ip[0 as i32 as usize] as i32,
            svs.authorizeAddress.ip[1 as i32 as usize] as i32,
            svs.authorizeAddress.ip[2 as i32 as usize] as i32,
            svs.authorizeAddress.ip[3 as i32 as usize] as i32,
            crate::src::qcommon::q_shared::ShortSwap(
                svs.authorizeAddress.port as i16,
            ) as i32,
        );
    }
    // otherwise send their ip to the authorize server
    if svs.authorizeAddress.type_0 as u32
        != NA_BAD as i32 as u32
    {
        NET_OutOfBandPrint(
            NS_SERVER,
            svs.authorizeAddress as netadr_t,
            b"banUser %i.%i.%i.%i\x00" as *const u8 as *const libc::c_char,
            (*cl).netchan.remoteAddress.ip[0 as i32 as usize] as i32,
            (*cl).netchan.remoteAddress.ip[1 as i32 as usize] as i32,
            (*cl).netchan.remoteAddress.ip[2 as i32 as usize] as i32,
            (*cl).netchan.remoteAddress.ip[3 as i32 as usize] as i32,
        );
        Com_Printf(
            b"%s was banned from coming back\n\x00" as *const u8 as *const libc::c_char,
            (*cl).name.as_mut_ptr(),
        );
    };
}
/*
==================
SV_BanNum_f

Ban a user from being able to play on this server through the auth
server
==================
*/

unsafe extern "C" fn SV_BanNum_f() {
    let mut cl: *mut client_t = 0 as *mut client_t;
    // make sure server is running
    if (*com_sv_running).integer == 0 {
        Com_Printf(
            b"Server is not running.\n\x00" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if Cmd_Argc() != 2 as i32 {
        Com_Printf(
            b"Usage: banClient <client number>\n\x00" as *const u8 as *const libc::c_char,
        );
        return;
    }
    cl = SV_GetPlayerByNum();
    if cl.is_null() {
        return;
    }
    if (*cl).netchan.remoteAddress.type_0 as u32 == NA_LOOPBACK as i32 as u32 {
        Com_Printf(
            b"Cannot kick host player\n\x00" as *const u8 as *const libc::c_char,
        );
        return;
    }
    // look up the authorize server's IP
    if svs.authorizeAddress.ip[0 as i32 as usize] == 0
        && svs.authorizeAddress.type_0 as u32
            != NA_BAD as i32 as u32
    {
        Com_Printf(
            b"Resolving %s\n\x00" as *const u8 as *const libc::c_char,
            b"authorize.quake3arena.com\x00" as *const u8 as *const libc::c_char,
        );
        if NET_StringToAdr(
            b"authorize.quake3arena.com\x00" as *const u8 as *const libc::c_char,
            &mut svs.authorizeAddress as *mut _
                as *mut netadr_t,
            NA_IP,
        ) == 0
        {
            Com_Printf(
                b"Couldn\'t resolve address\n\x00" as *const u8 as *const libc::c_char,
            );
            return;
        }
        svs.authorizeAddress.port =
            crate::src::qcommon::q_shared::ShortSwap(27952 as i32 as i16) as u16;
        Com_Printf(
            b"%s resolved to %i.%i.%i.%i:%i\n\x00" as *const u8 as *const libc::c_char,
            b"authorize.quake3arena.com\x00" as *const u8 as *const libc::c_char,
            svs.authorizeAddress.ip[0 as i32 as usize] as i32,
            svs.authorizeAddress.ip[1 as i32 as usize] as i32,
            svs.authorizeAddress.ip[2 as i32 as usize] as i32,
            svs.authorizeAddress.ip[3 as i32 as usize] as i32,
            crate::src::qcommon::q_shared::ShortSwap(
                svs.authorizeAddress.port as i16,
            ) as i32,
        );
    }
    // otherwise send their ip to the authorize server
    if svs.authorizeAddress.type_0 as u32
        != NA_BAD as i32 as u32
    {
        NET_OutOfBandPrint(
            NS_SERVER,
            svs.authorizeAddress as netadr_t,
            b"banUser %i.%i.%i.%i\x00" as *const u8 as *const libc::c_char,
            (*cl).netchan.remoteAddress.ip[0 as i32 as usize] as i32,
            (*cl).netchan.remoteAddress.ip[1 as i32 as usize] as i32,
            (*cl).netchan.remoteAddress.ip[2 as i32 as usize] as i32,
            (*cl).netchan.remoteAddress.ip[3 as i32 as usize] as i32,
        );
        Com_Printf(
            b"%s was banned from coming back\n\x00" as *const u8 as *const libc::c_char,
            (*cl).name.as_mut_ptr(),
        );
    };
}
/*
==================
SV_RehashBans_f

Load saved bans from file.
==================
*/

unsafe extern "C" fn SV_RehashBans_f() {
    let mut index: i32 = 0;
    let mut filelen: i32 = 0;
    let mut readfrom: fileHandle_t = 0;
    let mut textbuf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut curpos: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut maskpos: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut newlinepos: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut endpos: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut filepath: [libc::c_char; 64] = [0; 64];
    // make sure server is running
    if (*com_sv_running).integer == 0 {
        return;
    }
    serverBansCount = 0 as i32;
    if (*sv_banFile).string.is_null()
        || *(*sv_banFile).string == 0
    {
        return;
    }
    Com_sprintf(
        filepath.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as i32,
        b"%s/%s\x00" as *const u8 as *const libc::c_char,
        FS_GetCurrentGameDir(),
        (*sv_banFile).string,
    );
    filelen = FS_SV_FOpenFileRead(filepath.as_mut_ptr(), &mut readfrom)
        as i32;
    if filelen >= 0 as i32 {
        if filelen < 2 as i32 {
            // Don't bother if file is too short.
            FS_FCloseFile(readfrom);
            return;
        }
        textbuf = Z_Malloc(filelen) as *mut libc::c_char;
        curpos = textbuf;
        filelen =
            FS_Read(textbuf as *mut libc::c_void, filelen, readfrom);
        FS_FCloseFile(readfrom);
        endpos = textbuf.offset(filelen as isize);
        index = 0 as i32;
        while index < 1024 as i32 && curpos.offset(2 as i32 as isize) < endpos {
            // find the end of the address string
            maskpos = curpos.offset(2 as i32 as isize);
            while maskpos < endpos && *maskpos as i32 != ' ' as i32 {
                maskpos = maskpos.offset(1)
            }
            if maskpos.offset(1 as i32 as isize) >= endpos {
                break;
            }
            *maskpos = '\u{0}' as i32 as libc::c_char;
            maskpos = maskpos.offset(1);
            // find the end of the subnet specifier
            newlinepos = maskpos;
            while newlinepos < endpos && *newlinepos as i32 != '\n' as i32 {
                newlinepos = newlinepos.offset(1)
            }
            if newlinepos >= endpos {
                break;
            }
            *newlinepos = '\u{0}' as i32 as libc::c_char;
            if NET_StringToAdr(
                curpos.offset(2 as i32 as isize),
                &mut (*serverBans
                    .as_mut_ptr()
                    .offset(index as isize))
                .ip as *mut _ as *mut netadr_t,
                NA_UNSPEC,
            ) != 0
            {
                serverBans[index as usize].isexception =
                    (*curpos.offset(0 as i32 as isize) as i32 != '0' as i32) as i32
                        as qboolean;
                serverBans[index as usize].subnet = atoi(maskpos);
                if serverBans[index as usize]
                    .ip
                    .type_0 as u32
                    == NA_IP as i32 as u32
                    && (serverBans[index as usize].subnet < 1 as i32
                        || serverBans[index as usize].subnet
                            > 32 as i32)
                {
                    serverBans[index as usize].subnet = 32 as i32
                } else if serverBans[index as usize]
                    .ip
                    .type_0 as u32
                    == NA_IP6 as i32 as u32
                    && (serverBans[index as usize].subnet < 1 as i32
                        || serverBans[index as usize].subnet
                            > 128 as i32)
                {
                    serverBans[index as usize].subnet = 128 as i32
                }
            }
            curpos = newlinepos.offset(1 as i32 as isize);
            index += 1
        }
        serverBansCount = index;
        Z_Free(textbuf as *mut libc::c_void);
    };
}
/*
==================
SV_WriteBans

Save bans to file.
==================
*/

unsafe extern "C" fn SV_WriteBans() {
    let mut index: i32 = 0;
    let mut writeto: fileHandle_t = 0;
    let mut filepath: [libc::c_char; 64] = [0; 64];
    if (*sv_banFile).string.is_null()
        || *(*sv_banFile).string == 0
    {
        return;
    }
    Com_sprintf(
        filepath.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as i32,
        b"%s/%s\x00" as *const u8 as *const libc::c_char,
        FS_GetCurrentGameDir(),
        (*sv_banFile).string,
    );
    writeto = FS_SV_FOpenFileWrite(filepath.as_mut_ptr());
    if writeto != 0 {
        let mut writebuf: [libc::c_char; 128] = [0; 128];
        let mut curban: *mut serverBan_t = 0 as *mut serverBan_t;
        index = 0 as i32;
        while index < serverBansCount {
            curban = &mut *serverBans
                .as_mut_ptr()
                .offset(index as isize) as *mut serverBan_t;
            Com_sprintf(
                writebuf.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong as i32,
                b"%d %s %d\n\x00" as *const u8 as *const libc::c_char,
                (*curban).isexception as u32,
                NET_AdrToString(
                    (*curban).ip as netadr_t,
                ),
                (*curban).subnet,
            );
            FS_Write(
                writebuf.as_mut_ptr() as *const libc::c_void,
                crate::stdlib::strlen(writebuf.as_mut_ptr()) as i32,
                writeto,
            );
            index += 1
        }
        FS_FCloseFile(writeto);
    };
}
/*
==================
SV_DelBanEntryFromList

Remove a ban or an exception from the list.
==================
*/

unsafe extern "C" fn SV_DelBanEntryFromList(
    mut index: i32,
) -> qboolean {
    if index == serverBansCount - 1 as i32 {
        serverBansCount -= 1
    } else if (index as libc::c_ulong)
        < (::std::mem::size_of::<[serverBan_t; 1024]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<serverBan_t>() as libc::c_ulong)
            .wrapping_sub(1 as i32 as libc::c_ulong)
    {
        crate::stdlib::memmove(
            serverBans
                .as_mut_ptr()
                .offset(index as isize) as *mut libc::c_void,
            serverBans
                .as_mut_ptr()
                .offset(index as isize)
                .offset(1 as i32 as isize) as *const libc::c_void,
            ((serverBansCount - index - 1 as i32) as libc::c_ulong)
                .wrapping_mul(
                    ::std::mem::size_of::<serverBan_t>() as libc::c_ulong
                ),
        );
        serverBansCount -= 1
    } else {
        return qtrue;
    }
    return qfalse;
}
/*
==================
SV_ParseCIDRNotation

Parse a CIDR notation type string and return a netadr_t and suffix by reference
==================
*/

unsafe extern "C" fn SV_ParseCIDRNotation(
    mut dest: *mut netadr_t,
    mut mask: *mut i32,
    mut adrstr: *mut libc::c_char,
) -> qboolean {
    let mut suffix: *mut libc::c_char = 0 as *mut libc::c_char;
    suffix = libc::strchr(adrstr, '/' as i32);
    if !suffix.is_null() {
        *suffix = '\u{0}' as i32 as libc::c_char;
        suffix = suffix.offset(1)
    }
    if NET_StringToAdr(
        adrstr,
        dest as *mut netadr_t,
        NA_UNSPEC,
    ) == 0
    {
        return qtrue;
    }
    if !suffix.is_null() {
        *mask = atoi(suffix);
        if (*dest).type_0 as u32 == NA_IP as i32 as u32 {
            if *mask < 1 as i32 || *mask > 32 as i32 {
                *mask = 32 as i32
            }
        } else if *mask < 1 as i32 || *mask > 128 as i32 {
            *mask = 128 as i32
        }
    } else if (*dest).type_0 as u32 == NA_IP as i32 as u32 {
        *mask = 32 as i32
    } else {
        *mask = 128 as i32
    }
    return qfalse;
}
/*
==================
SV_AddBanToList

Ban a user from being able to play on this server based on his ip address.
==================
*/

unsafe extern "C" fn SV_AddBanToList(mut isexception: qboolean) {
    let mut banstring: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut addy2: [libc::c_char; 48] = [0; 48];
    let mut ip: netadr_t = netadr_t {
        type_0: NA_BAD,
        ip: [0; 4],
        ip6: [0; 16],
        port: 0,
        scope_id: 0,
    };
    let mut index: i32 = 0;
    let mut argc: i32 = 0;
    let mut mask: i32 = 0;
    let mut curban: *mut serverBan_t = 0 as *mut serverBan_t;
    // make sure server is running
    if (*com_sv_running).integer == 0 {
        Com_Printf(
            b"Server is not running.\n\x00" as *const u8 as *const libc::c_char,
        );
        return;
    }
    argc = Cmd_Argc();
    if argc < 2 as i32 || argc > 3 as i32 {
        Com_Printf(
            b"Usage: %s (ip[/subnet] | clientnum [subnet])\n\x00" as *const u8
                as *const libc::c_char,
            Cmd_Argv(0 as i32),
        );
        return;
    }
    if serverBansCount as libc::c_ulong
        >= (::std::mem::size_of::<[serverBan_t; 1024]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<serverBan_t>() as libc::c_ulong)
    {
        Com_Printf(
            b"Error: Maximum number of bans/exceptions exceeded.\n\x00" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    banstring = Cmd_Argv(1 as i32);
    if !libc::strchr(banstring, '.' as i32).is_null()
        || !libc::strchr(banstring, ':' as i32).is_null()
    {
        // This is an ip address, not a client num.
        if SV_ParseCIDRNotation(&mut ip, &mut mask, banstring) as u64 != 0 {
            Com_Printf(
                b"Error: Invalid address %s\n\x00" as *const u8 as *const libc::c_char,
                banstring,
            );
            return;
        }
    } else {
        let mut cl: *mut client_t = 0 as *mut client_t;
        // client num.
        cl = SV_GetPlayerByNum();
        if cl.is_null() {
            Com_Printf(
                b"Error: Playernum %s does not exist.\n\x00" as *const u8 as *const libc::c_char,
                Cmd_Argv(1 as i32),
            );
            return;
        }
        ip = (*cl).netchan.remoteAddress;
        if argc == 3 as i32 {
            mask = atoi(Cmd_Argv(2 as i32));
            if ip.type_0 as u32 == NA_IP as i32 as u32 {
                if mask < 1 as i32 || mask > 32 as i32 {
                    mask = 32 as i32
                }
            } else if mask < 1 as i32 || mask > 128 as i32 {
                mask = 128 as i32
            }
        } else {
            mask = if ip.type_0 as u32 == NA_IP6 as i32 as u32 {
                128 as i32
            } else {
                32 as i32
            }
        }
    }
    if ip.type_0 as u32 != NA_IP as i32 as u32
        && ip.type_0 as u32 != NA_IP6 as i32 as u32
    {
        Com_Printf(
            b"Error: Can ban players connected via the internet only.\n\x00" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    // first check whether a conflicting ban exists that would supersede the new one.
    index = 0 as i32;
    while index < serverBansCount {
        curban = &mut *serverBans
            .as_mut_ptr()
            .offset(index as isize) as *mut serverBan_t;
        if (*curban).subnet <= mask {
            if ((*curban).isexception as u32 != 0 || isexception as u64 == 0)
                && NET_CompareBaseAdrMask(
                    (*curban).ip as netadr_t,
                    ip as netadr_t,
                    (*curban).subnet,
                ) as u32
                    != 0
            {
                Q_strncpyz(
                    addy2.as_mut_ptr(),
                    NET_AdrToString(ip as netadr_t),
                    ::std::mem::size_of::<[libc::c_char; 48]>() as libc::c_ulong as i32,
                );
                Com_Printf(
                    b"Error: %s %s/%d supersedes %s %s/%d\n\x00" as *const u8
                        as *const libc::c_char,
                    if (*curban).isexception as u32 != 0 {
                        b"Exception\x00" as *const u8 as *const libc::c_char
                    } else {
                        b"Ban\x00" as *const u8 as *const libc::c_char
                    },
                    NET_AdrToString(
                        (*curban).ip as netadr_t,
                    ),
                    (*curban).subnet,
                    if isexception as u32 != 0 {
                        b"exception\x00" as *const u8 as *const libc::c_char
                    } else {
                        b"ban\x00" as *const u8 as *const libc::c_char
                    },
                    addy2.as_mut_ptr(),
                    mask,
                );
                return;
            }
        }
        if (*curban).subnet >= mask {
            if (*curban).isexception as u64 == 0
                && isexception as u32 != 0
                && NET_CompareBaseAdrMask(
                    (*curban).ip as netadr_t,
                    ip as netadr_t,
                    mask,
                ) as u32
                    != 0
            {
                Q_strncpyz(
                    addy2.as_mut_ptr(),
                    NET_AdrToString(
                        (*curban).ip as netadr_t,
                    ),
                    ::std::mem::size_of::<[libc::c_char; 48]>() as libc::c_ulong as i32,
                );
                Com_Printf(
                    b"Error: %s %s/%d supersedes already existing %s %s/%d\n\x00" as *const u8
                        as *const libc::c_char,
                    if isexception as u32 != 0 {
                        b"Exception\x00" as *const u8 as *const libc::c_char
                    } else {
                        b"Ban\x00" as *const u8 as *const libc::c_char
                    },
                    NET_AdrToString(ip as netadr_t),
                    mask,
                    if (*curban).isexception as u32 != 0 {
                        b"exception\x00" as *const u8 as *const libc::c_char
                    } else {
                        b"ban\x00" as *const u8 as *const libc::c_char
                    },
                    addy2.as_mut_ptr(),
                    (*curban).subnet,
                );
                return;
            }
        }
        index += 1
    }
    // now delete bans that are superseded by the new one
    index = 0 as i32;
    while index < serverBansCount {
        curban = &mut *serverBans
            .as_mut_ptr()
            .offset(index as isize) as *mut serverBan_t;
        if (*curban).subnet > mask
            && ((*curban).isexception as u64 == 0 || isexception as u32 != 0)
            && NET_CompareBaseAdrMask(
                (*curban).ip as netadr_t,
                ip as netadr_t,
                mask,
            ) as u32
                != 0
        {
            SV_DelBanEntryFromList(index);
        } else {
            index += 1
        }
    }
    serverBans
        [serverBansCount as usize]
        .ip = ip;
    serverBans
        [serverBansCount as usize]
        .subnet = mask;
    serverBans
        [serverBansCount as usize]
        .isexception = isexception;
    serverBansCount += 1;
    SV_WriteBans();
    Com_Printf(
        b"Added %s: %s/%d\n\x00" as *const u8 as *const libc::c_char,
        if isexception as u32 != 0 {
            b"ban exception\x00" as *const u8 as *const libc::c_char
        } else {
            b"ban\x00" as *const u8 as *const libc::c_char
        },
        NET_AdrToString(ip as netadr_t),
        mask,
    );
}
/*
==================
SV_DelBanFromList

Remove a ban or an exception from the list.
==================
*/

unsafe extern "C" fn SV_DelBanFromList(mut isexception: qboolean) {
    let mut index: i32 = 0;
    let mut count: i32 = 0 as i32;
    let mut todel: i32 = 0;
    let mut mask: i32 = 0;
    let mut ip: netadr_t = netadr_t {
        type_0: NA_BAD,
        ip: [0; 4],
        ip6: [0; 16],
        port: 0,
        scope_id: 0,
    };
    let mut banstring: *mut libc::c_char = 0 as *mut libc::c_char;
    // make sure server is running
    if (*com_sv_running).integer == 0 {
        Com_Printf(
            b"Server is not running.\n\x00" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if Cmd_Argc() != 2 as i32 {
        Com_Printf(
            b"Usage: %s (ip[/subnet] | num)\n\x00" as *const u8 as *const libc::c_char,
            Cmd_Argv(0 as i32),
        );
        return;
    }
    banstring = Cmd_Argv(1 as i32);
    if !libc::strchr(banstring, '.' as i32).is_null()
        || !libc::strchr(banstring, ':' as i32).is_null()
    {
        let mut curban: *mut serverBan_t = 0 as *mut serverBan_t;
        if SV_ParseCIDRNotation(&mut ip, &mut mask, banstring) as u64 != 0 {
            Com_Printf(
                b"Error: Invalid address %s\n\x00" as *const u8 as *const libc::c_char,
                banstring,
            );
            return;
        }
        index = 0 as i32;
        while index < serverBansCount {
            curban = &mut *serverBans
                .as_mut_ptr()
                .offset(index as isize) as *mut serverBan_t;
            if (*curban).isexception as u32 == isexception as u32
                && (*curban).subnet >= mask
                && NET_CompareBaseAdrMask(
                    (*curban).ip as netadr_t,
                    ip as netadr_t,
                    mask,
                ) as u32
                    != 0
            {
                Com_Printf(
                    b"Deleting %s %s/%d\n\x00" as *const u8 as *const libc::c_char,
                    if isexception as u32 != 0 {
                        b"exception\x00" as *const u8 as *const libc::c_char
                    } else {
                        b"ban\x00" as *const u8 as *const libc::c_char
                    },
                    NET_AdrToString(
                        (*curban).ip as netadr_t,
                    ),
                    (*curban).subnet,
                );
                SV_DelBanEntryFromList(index);
            } else {
                index += 1
            }
        }
    } else {
        todel = atoi(Cmd_Argv(1 as i32));
        if todel < 1 as i32 || todel > serverBansCount {
            Com_Printf(
                b"Error: Invalid ban number given\n\x00" as *const u8 as *const libc::c_char,
            );
            return;
        }
        index = 0 as i32;
        while index < serverBansCount {
            if serverBans[index as usize].isexception as u32
                == isexception as u32
            {
                count += 1;
                if count == todel {
                    Com_Printf(
                        b"Deleting %s %s/%d\n\x00" as *const u8 as *const libc::c_char,
                        if isexception as u32 != 0 {
                            b"exception\x00" as *const u8 as *const libc::c_char
                        } else {
                            b"ban\x00" as *const u8 as *const libc::c_char
                        },
                        NET_AdrToString(
                            serverBans[index as usize].ip
                                as netadr_t,
                        ),
                        serverBans[index as usize].subnet,
                    );
                    SV_DelBanEntryFromList(index);
                    break;
                }
            }
            index += 1
        }
    }
    SV_WriteBans();
}
/*
==================
SV_ListBans_f

List all bans and exceptions on console
==================
*/

unsafe extern "C" fn SV_ListBans_f() {
    let mut index: i32 = 0;
    let mut count: i32 = 0;
    let mut ban: *mut serverBan_t = 0 as *mut serverBan_t;
    // make sure server is running
    if (*com_sv_running).integer == 0 {
        Com_Printf(
            b"Server is not running.\n\x00" as *const u8 as *const libc::c_char,
        );
        return;
    }
    // List all bans
    count = 0 as i32;
    index = count;
    while index < serverBansCount {
        ban = &mut *serverBans
            .as_mut_ptr()
            .offset(index as isize) as *mut serverBan_t;
        if (*ban).isexception as u64 == 0 {
            count += 1;
            Com_Printf(
                b"Ban #%d: %s/%d\n\x00" as *const u8 as *const libc::c_char,
                count,
                NET_AdrToString(
                    (*ban).ip as netadr_t,
                ),
                (*ban).subnet,
            );
        }
        index += 1
    }
    // List all exceptions
    count = 0 as i32;
    index = count;
    while index < serverBansCount {
        ban = &mut *serverBans
            .as_mut_ptr()
            .offset(index as isize) as *mut serverBan_t;
        if (*ban).isexception as u64 != 0 {
            count += 1;
            Com_Printf(
                b"Except #%d: %s/%d\n\x00" as *const u8 as *const libc::c_char,
                count,
                NET_AdrToString(
                    (*ban).ip as netadr_t,
                ),
                (*ban).subnet,
            );
        }
        index += 1
    }
}
/*
==================
SV_FlushBans_f

Delete all bans and exceptions.
==================
*/

unsafe extern "C" fn SV_FlushBans_f() {
    // make sure server is running
    if (*com_sv_running).integer == 0 {
        Com_Printf(
            b"Server is not running.\n\x00" as *const u8 as *const libc::c_char,
        );
        return;
    }
    serverBansCount = 0 as i32;
    // empty the ban file.
    SV_WriteBans();
    Com_Printf(
        b"All bans and exceptions have been deleted.\n\x00" as *const u8 as *const libc::c_char,
    );
}

unsafe extern "C" fn SV_BanAddr_f() {
    SV_AddBanToList(qfalse);
}

unsafe extern "C" fn SV_ExceptAddr_f() {
    SV_AddBanToList(qtrue);
}

unsafe extern "C" fn SV_BanDel_f() {
    SV_DelBanFromList(qfalse);
}

unsafe extern "C" fn SV_ExceptDel_f() {
    SV_DelBanFromList(qtrue);
}
/*
** SV_Strlen -- skips color escape codes
*/

unsafe extern "C" fn SV_Strlen(mut str: *const libc::c_char) -> i32 {
    let mut s: *const libc::c_char = str;
    let mut count: i32 = 0 as i32;
    while *s != 0 {
        if Q_IsColorString(s) as u64 != 0 {
            s = s.offset(2 as i32 as isize)
        } else {
            count += 1;
            s = s.offset(1)
        }
    }
    return count;
}
/*
================
SV_Status_f
================
*/

unsafe extern "C" fn SV_Status_f() {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut l: i32 = 0;
    let mut cl: *mut client_t = 0 as *mut client_t;
    let mut ps: *mut playerState_t =
        0 as *mut playerState_t;
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut ping: i32 = 0;
    // make sure server is running
    if (*com_sv_running).integer == 0 {
        Com_Printf(
            b"Server is not running.\n\x00" as *const u8 as *const libc::c_char,
        );
        return;
    }
    Com_Printf(
        b"map: %s\n\x00" as *const u8 as *const libc::c_char,
        (*sv_mapname).string,
    );
    Com_Printf(
        b"cl score ping name            address                                 rate \n\x00"
            as *const u8 as *const libc::c_char,
    );
    Com_Printf(
        b"-- ----- ---- --------------- --------------------------------------- -----\n\x00"
            as *const u8 as *const libc::c_char,
    );
    i = 0 as i32;
    cl = svs.clients;
    while i < (*sv_maxclients).integer {
        if !((*cl).state as u64 == 0) {
            Com_Printf(
                b"%2i \x00" as *const u8 as *const libc::c_char,
                i,
            );
            ps = SV_GameClientNum(i)
                as *mut playerState_s;
            Com_Printf(
                b"%5i \x00" as *const u8 as *const libc::c_char,
                (*ps).persistant[0 as i32 as usize],
            );
            if (*cl).state as u32 == CS_CONNECTED as i32 as u32 {
                Com_Printf(
                    b"CON \x00" as *const u8 as *const libc::c_char,
                );
            } else if (*cl).state as u32 == CS_ZOMBIE as i32 as u32 {
                Com_Printf(
                    b"ZMB \x00" as *const u8 as *const libc::c_char,
                );
            } else {
                ping = if (*cl).ping < 9999 as i32 {
                    (*cl).ping
                } else {
                    9999 as i32
                };
                Com_Printf(
                    b"%4i \x00" as *const u8 as *const libc::c_char,
                    ping,
                );
            }
            Com_Printf(
                b"%s\x00" as *const u8 as *const libc::c_char,
                (*cl).name.as_mut_ptr(),
            );
            l = 16 as i32 - SV_Strlen((*cl).name.as_mut_ptr());
            j = 0 as i32;
            loop {
                Com_Printf(
                    b" \x00" as *const u8 as *const libc::c_char,
                );
                j += 1;
                if !(j < l) {
                    break;
                }
            }
            // TTimo adding a ^7 to reset the color
            s = NET_AdrToString(
                (*cl).netchan.remoteAddress as netadr_t,
            );
            Com_Printf(
                b"^7%s\x00" as *const u8 as *const libc::c_char,
                s,
            );
            l = (39 as i32 as libc::c_ulong).wrapping_sub(crate::stdlib::strlen(s)) as i32;
            j = 0 as i32;
            loop {
                Com_Printf(
                    b" \x00" as *const u8 as *const libc::c_char,
                );
                j += 1;
                if !(j < l) {
                    break;
                }
            }
            Com_Printf(
                b" %5i\x00" as *const u8 as *const libc::c_char,
                (*cl).rate,
            );
            Com_Printf(b"\n\x00" as *const u8 as *const libc::c_char);
        }
        i += 1;
        cl = cl.offset(1)
    }
    Com_Printf(b"\n\x00" as *const u8 as *const libc::c_char);
}
/*
==================
SV_ConSay_f
==================
*/

unsafe extern "C" fn SV_ConSay_f() {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut text: [libc::c_char; 1024] = [0; 1024];
    // make sure server is running
    if (*com_sv_running).integer == 0 {
        Com_Printf(
            b"Server is not running.\n\x00" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if Cmd_Argc() < 2 as i32 {
        return;
    }
    libc::strcpy(
        text.as_mut_ptr(),
        b"console: \x00" as *const u8 as *const libc::c_char,
    );
    p = Cmd_Args();
    if *p as i32 == '\"' as i32 {
        p = p.offset(1);
        *p.offset(crate::stdlib::strlen(p).wrapping_sub(1 as i32 as libc::c_ulong) as isize) =
            0 as i32 as libc::c_char
    }
    libc::strcat(text.as_mut_ptr(), p);
    Com_Printf(
        b"%s\n\x00" as *const u8 as *const libc::c_char,
        text.as_mut_ptr(),
    );
    SV_SendServerCommand(
        0 as *mut client_t as *mut client_s,
        b"chat \"%s\"\x00" as *const u8 as *const libc::c_char,
        text.as_mut_ptr(),
    );
}
/*
==================
SV_ConTell_f
==================
*/

unsafe extern "C" fn SV_ConTell_f() {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut text: [libc::c_char; 1024] = [0; 1024];
    let mut cl: *mut client_t = 0 as *mut client_t;
    // make sure server is running
    if (*com_sv_running).integer == 0 {
        Com_Printf(
            b"Server is not running.\n\x00" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if Cmd_Argc() < 3 as i32 {
        Com_Printf(
            b"Usage: tell <client number> <text>\n\x00" as *const u8 as *const libc::c_char,
        );
        return;
    }
    cl = SV_GetPlayerByNum();
    if cl.is_null() {
        return;
    }
    libc::strcpy(
        text.as_mut_ptr(),
        b"console_tell: \x00" as *const u8 as *const libc::c_char,
    );
    p = Cmd_ArgsFrom(2 as i32);
    if *p as i32 == '\"' as i32 {
        p = p.offset(1);
        *p.offset(crate::stdlib::strlen(p).wrapping_sub(1 as i32 as libc::c_ulong) as isize) =
            0 as i32 as libc::c_char
    }
    libc::strcat(text.as_mut_ptr(), p);
    Com_Printf(
        b"%s\n\x00" as *const u8 as *const libc::c_char,
        text.as_mut_ptr(),
    );
    SV_SendServerCommand(
        cl as *mut client_s,
        b"chat \"%s\"\x00" as *const u8 as *const libc::c_char,
        text.as_mut_ptr(),
    );
}
/*
==================
SV_ConSayto_f
==================
*/

unsafe extern "C" fn SV_ConSayto_f() {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut text: [libc::c_char; 1024] = [0; 1024];
    let mut cl: *mut client_t = 0 as *mut client_t;
    let mut rawname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut name: [libc::c_char; 32] = [0; 32];
    let mut cleanName: [libc::c_char; 32] = [0; 32];
    let mut saytocl: *mut client_t = 0 as *mut client_t;
    let mut i: i32 = 0;
    // make sure server is running
    if (*com_sv_running).integer == 0 {
        Com_Printf(
            b"Server is not running.\n\x00" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if Cmd_Argc() < 3 as i32 {
        Com_Printf(
            b"Usage: sayto <player name> <text>\n\x00" as *const u8 as *const libc::c_char,
        );
        return;
    }
    rawname = Cmd_Argv(1 as i32);
    //allowing special characters in the console
    //with hex strings for player names
    Com_FieldStringToPlayerName(name.as_mut_ptr(), 32 as i32, rawname);
    saytocl = 0 as *mut client_t;
    i = 0 as i32;
    cl = svs.clients;
    while i < (*sv_maxclients).integer {
        if !((*cl).state as u64 == 0) {
            Q_strncpyz(
                cleanName.as_mut_ptr(),
                (*cl).name.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong as i32,
            );
            Q_CleanStr(cleanName.as_mut_ptr());
            if Q_stricmp(cleanName.as_mut_ptr(), name.as_mut_ptr())
                == 0
            {
                saytocl = cl;
                break;
            }
        }
        i += 1;
        cl = cl.offset(1)
    }
    if saytocl.is_null() {
        Com_Printf(
            b"No such player name: %s.\n\x00" as *const u8 as *const libc::c_char,
            name.as_mut_ptr(),
        );
        return;
    }
    libc::strcpy(
        text.as_mut_ptr(),
        b"console_sayto: \x00" as *const u8 as *const libc::c_char,
    );
    p = Cmd_ArgsFrom(2 as i32);
    if *p as i32 == '\"' as i32 {
        p = p.offset(1);
        *p.offset(crate::stdlib::strlen(p).wrapping_sub(1 as i32 as libc::c_ulong) as isize) =
            0 as i32 as libc::c_char
    }
    libc::strcat(text.as_mut_ptr(), p);
    Com_Printf(
        b"%s\n\x00" as *const u8 as *const libc::c_char,
        text.as_mut_ptr(),
    );
    SV_SendServerCommand(
        saytocl as *mut client_s,
        b"chat \"%s\"\x00" as *const u8 as *const libc::c_char,
        text.as_mut_ptr(),
    );
}
//
// sv_init.c
//
//
// sv_client.c
//
//
// sv_ccmds.c
//
/*
==================
SV_Heartbeat_f

Also called by SV_DropClient, SV_DirectConnect, and SV_SpawnServer
==================
*/
#[no_mangle]

pub unsafe extern "C" fn SV_Heartbeat_f() {
    svs.nextHeartbeatTime = -(9999999 as i32);
}
/*
===========
SV_Serverinfo_f

Examine the serverinfo string
===========
*/

unsafe extern "C" fn SV_Serverinfo_f() {
    // make sure server is running
    if (*com_sv_running).integer == 0 {
        Com_Printf(
            b"Server is not running.\n\x00" as *const u8 as *const libc::c_char,
        );
        return;
    }
    Com_Printf(
        b"Server info settings:\n\x00" as *const u8 as *const libc::c_char,
    );
    Info_Print(Cvar_InfoString(0x4 as i32));
}
/*
===========
SV_Systeminfo_f

Examine the systeminfo string
===========
*/

unsafe extern "C" fn SV_Systeminfo_f() {
    // make sure server is running
    if (*com_sv_running).integer == 0 {
        Com_Printf(
            b"Server is not running.\n\x00" as *const u8 as *const libc::c_char,
        );
        return;
    }
    Com_Printf(
        b"System info settings:\n\x00" as *const u8 as *const libc::c_char,
    );
    Info_Print(Cvar_InfoString_Big(
        0x8 as i32,
    ));
}
/*
===========
SV_DumpUser_f

Examine all a users info strings
===========
*/

unsafe extern "C" fn SV_DumpUser_f() {
    let mut cl: *mut client_t = 0 as *mut client_t;
    // make sure server is running
    if (*com_sv_running).integer == 0 {
        Com_Printf(
            b"Server is not running.\n\x00" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if Cmd_Argc() != 2 as i32 {
        Com_Printf(
            b"Usage: dumpuser <userid>\n\x00" as *const u8 as *const libc::c_char,
        );
        return;
    }
    cl = SV_GetPlayerByHandle();
    if cl.is_null() {
        return;
    }
    Com_Printf(b"userinfo\n\x00" as *const u8 as *const libc::c_char);
    Com_Printf(b"--------\n\x00" as *const u8 as *const libc::c_char);
    Info_Print((*cl).userinfo.as_mut_ptr());
}
/*
=================
SV_KillServer
=================
*/

unsafe extern "C" fn SV_KillServer_f() {
    SV_Shutdown(
        b"killserver\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
//===========================================================
/*
==================
SV_CompleteMapName
==================
*/

unsafe extern "C" fn SV_CompleteMapName(mut _args: *mut libc::c_char, mut argNum: i32) {
    if argNum == 2 as i32 {
        Field_CompleteFilename(
            b"maps\x00" as *const u8 as *const libc::c_char,
            b"bsp\x00" as *const u8 as *const libc::c_char,
            qtrue,
            qfalse,
        );
    };
}
/*
==================
SV_CompletePlayerName
==================
*/

unsafe extern "C" fn SV_CompletePlayerName(mut _args: *mut libc::c_char, mut argNum: i32) {
    if argNum == 2 as i32 {
        let mut names: [[libc::c_char; 32]; 64] = [[0; 32]; 64];
        let mut namesPtr: [*const libc::c_char; 64] = [0 as *const libc::c_char; 64];
        let mut cl: *mut client_t = 0 as *mut client_t;
        let mut i: i32 = 0;
        let mut nameCount: i32 = 0;
        let mut clientCount: i32 = 0;
        nameCount = 0 as i32;
        clientCount = (*sv_maxclients).integer;
        i = 0 as i32;
        cl = svs.clients;
        while i < clientCount {
            if !((*cl).state as u64 == 0) {
                if i >= 64 as i32 {
                    break;
                }
                Q_strncpyz(
                    names[nameCount as usize].as_mut_ptr(),
                    (*cl).name.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong as i32,
                );
                Q_CleanStr(names[nameCount as usize].as_mut_ptr());
                namesPtr[nameCount as usize] = names[nameCount as usize].as_mut_ptr();
                nameCount += 1
            }
            i += 1;
            cl = cl.offset(1)
        }
        qsort(
            namesPtr.as_mut_ptr() as *mut libc::c_void,
            nameCount as size_t,
            ::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong,
            Some(
                Com_strCompare
                    as unsafe extern "C" fn(_: *const libc::c_void, _: *const libc::c_void) -> i32,
            ),
        );
        Field_CompletePlayerName(namesPtr.as_mut_ptr(), nameCount);
    };
}
/*
==================
SV_AddOperatorCommands
==================
*/
#[no_mangle]

pub unsafe extern "C" fn SV_AddOperatorCommands() {
    static mut initialized: qboolean =
        qfalse; // Legacy command
    if initialized as u64 != 0 {
        return;
    }
    initialized = qtrue;
    Cmd_AddCommand(
        b"heartbeat\x00" as *const u8 as *const libc::c_char,
        Some(SV_Heartbeat_f as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"kick\x00" as *const u8 as *const libc::c_char,
        Some(SV_Kick_f as unsafe extern "C" fn() -> ()),
    );
    if (*com_standalone).integer == 0 {
        Cmd_AddCommand(
            b"banUser\x00" as *const u8 as *const libc::c_char,
            Some(SV_Ban_f as unsafe extern "C" fn() -> ()),
        );
        Cmd_AddCommand(
            b"banClient\x00" as *const u8 as *const libc::c_char,
            Some(SV_BanNum_f as unsafe extern "C" fn() -> ()),
        );
    }
    Cmd_AddCommand(
        b"kickbots\x00" as *const u8 as *const libc::c_char,
        Some(SV_KickBots_f as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"kickall\x00" as *const u8 as *const libc::c_char,
        Some(SV_KickAll_f as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"kicknum\x00" as *const u8 as *const libc::c_char,
        Some(SV_KickNum_f as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"clientkick\x00" as *const u8 as *const libc::c_char,
        Some(SV_KickNum_f as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"status\x00" as *const u8 as *const libc::c_char,
        Some(SV_Status_f as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"serverinfo\x00" as *const u8 as *const libc::c_char,
        Some(SV_Serverinfo_f as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"systeminfo\x00" as *const u8 as *const libc::c_char,
        Some(SV_Systeminfo_f as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"dumpuser\x00" as *const u8 as *const libc::c_char,
        Some(SV_DumpUser_f as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"map_restart\x00" as *const u8 as *const libc::c_char,
        Some(SV_MapRestart_f as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"sectorlist\x00" as *const u8 as *const libc::c_char,
        Some(SV_SectorList_f as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"map\x00" as *const u8 as *const libc::c_char,
        Some(SV_Map_f as unsafe extern "C" fn() -> ()),
    );
    Cmd_SetCommandCompletionFunc(
        b"map\x00" as *const u8 as *const libc::c_char,
        Some(SV_CompleteMapName as unsafe extern "C" fn(_: *mut libc::c_char, _: i32) -> ()),
    );
    Cmd_AddCommand(
        b"devmap\x00" as *const u8 as *const libc::c_char,
        Some(SV_Map_f as unsafe extern "C" fn() -> ()),
    );
    Cmd_SetCommandCompletionFunc(
        b"devmap\x00" as *const u8 as *const libc::c_char,
        Some(SV_CompleteMapName as unsafe extern "C" fn(_: *mut libc::c_char, _: i32) -> ()),
    );
    Cmd_AddCommand(
        b"spmap\x00" as *const u8 as *const libc::c_char,
        Some(SV_Map_f as unsafe extern "C" fn() -> ()),
    );
    Cmd_SetCommandCompletionFunc(
        b"spmap\x00" as *const u8 as *const libc::c_char,
        Some(SV_CompleteMapName as unsafe extern "C" fn(_: *mut libc::c_char, _: i32) -> ()),
    );
    Cmd_AddCommand(
        b"spdevmap\x00" as *const u8 as *const libc::c_char,
        Some(SV_Map_f as unsafe extern "C" fn() -> ()),
    );
    Cmd_SetCommandCompletionFunc(
        b"spdevmap\x00" as *const u8 as *const libc::c_char,
        Some(SV_CompleteMapName as unsafe extern "C" fn(_: *mut libc::c_char, _: i32) -> ()),
    );
    Cmd_AddCommand(
        b"killserver\x00" as *const u8 as *const libc::c_char,
        Some(SV_KillServer_f as unsafe extern "C" fn() -> ()),
    );
    if (*com_dedicated).integer != 0 {
        Cmd_AddCommand(
            b"say\x00" as *const u8 as *const libc::c_char,
            Some(SV_ConSay_f as unsafe extern "C" fn() -> ()),
        );
        Cmd_AddCommand(
            b"tell\x00" as *const u8 as *const libc::c_char,
            Some(SV_ConTell_f as unsafe extern "C" fn() -> ()),
        );
        Cmd_AddCommand(
            b"sayto\x00" as *const u8 as *const libc::c_char,
            Some(SV_ConSayto_f as unsafe extern "C" fn() -> ()),
        );
        Cmd_SetCommandCompletionFunc(
            b"sayto\x00" as *const u8 as *const libc::c_char,
            Some(SV_CompletePlayerName as unsafe extern "C" fn(_: *mut libc::c_char, _: i32) -> ()),
        );
    }
    Cmd_AddCommand(
        b"rehashbans\x00" as *const u8 as *const libc::c_char,
        Some(SV_RehashBans_f as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"listbans\x00" as *const u8 as *const libc::c_char,
        Some(SV_ListBans_f as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"banaddr\x00" as *const u8 as *const libc::c_char,
        Some(SV_BanAddr_f as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"exceptaddr\x00" as *const u8 as *const libc::c_char,
        Some(SV_ExceptAddr_f as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"bandel\x00" as *const u8 as *const libc::c_char,
        Some(SV_BanDel_f as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"exceptdel\x00" as *const u8 as *const libc::c_char,
        Some(SV_ExceptDel_f as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"flushbans\x00" as *const u8 as *const libc::c_char,
        Some(SV_FlushBans_f as unsafe extern "C" fn() -> ()),
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
/*
==================
SV_RemoveOperatorCommands
==================
*/
#[no_mangle]

pub unsafe extern "C" fn SV_RemoveOperatorCommands() {}

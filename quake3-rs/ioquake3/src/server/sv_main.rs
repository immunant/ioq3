use ::libc;

pub use crate::internal::__builtin_va_list;
pub use crate::internal::__va_list_tag;
pub use crate::stdarg_h::va_list;
pub use crate::stddef_h::size_t;
pub use crate::stdlib::intptr_t;

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
pub use crate::qcommon_h::msg_t;
pub use crate::qcommon_h::netadr_t;
pub use crate::qcommon_h::netadrtype_t;
pub use crate::qcommon_h::netchan_t;
pub use crate::qcommon_h::netsrc_t;
pub use crate::qcommon_h::vm_t;
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
pub use crate::server_h::leakyBucket_s;
pub use crate::server_h::leakyBucket_t;
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
pub use crate::server_h::C2RustUnnamed_164;
pub use crate::server_h::CS_ACTIVE;
pub use crate::server_h::CS_CONNECTED;
pub use crate::server_h::CS_FREE;
pub use crate::server_h::CS_PRIMED;
pub use crate::server_h::CS_ZOMBIE;
pub use crate::server_h::SS_DEAD;
pub use crate::server_h::SS_GAME;
pub use crate::server_h::SS_LOADING;
pub use crate::src::qcommon::cmd::Cbuf_AddText;
pub use crate::src::qcommon::cmd::Cmd_ArgsFrom;
pub use crate::src::qcommon::cmd::Cmd_Argv;
pub use crate::src::qcommon::cmd::Cmd_Cmd;
pub use crate::src::qcommon::cmd::Cmd_ExecuteString;
pub use crate::src::qcommon::cmd::Cmd_TokenizeString;
pub use crate::src::qcommon::common::cl_paused;
pub use crate::src::qcommon::common::com_dedicated;
pub use crate::src::qcommon::common::com_gamename;
pub use crate::src::qcommon::common::com_legacyprotocol;
pub use crate::src::qcommon::common::com_protocol;
pub use crate::src::qcommon::common::com_speeds;
pub use crate::src::qcommon::common::com_sv_running;
pub use crate::src::qcommon::common::com_timescale;
pub use crate::src::qcommon::common::sv_paused;
pub use crate::src::qcommon::common::time_game;
pub use crate::src::qcommon::common::Com_BeginRedirect;
pub use crate::src::qcommon::common::Com_DPrintf;
pub use crate::src::qcommon::common::Com_EndRedirect;
pub use crate::src::qcommon::common::Com_Printf;
pub use crate::src::qcommon::cvar::cvar_modifiedFlags;
pub use crate::src::qcommon::cvar::Cvar_InfoString;
pub use crate::src::qcommon::cvar::Cvar_InfoString_Big;
pub use crate::src::qcommon::cvar::Cvar_Set;
pub use crate::src::qcommon::cvar::Cvar_VariableIntegerValue;
pub use crate::src::qcommon::cvar::Cvar_VariableString;
pub use crate::src::qcommon::cvar::Cvar_VariableValue;
pub use crate::src::qcommon::huffman::Huff_Decompress;
pub use crate::src::qcommon::msg::MSG_BeginReadingOOB;
pub use crate::src::qcommon::msg::MSG_ReadLong;
pub use crate::src::qcommon::msg::MSG_ReadShort;
pub use crate::src::qcommon::msg::MSG_ReadStringLine;
pub use crate::src::qcommon::net_chan::NET_OutOfBandPrint;
pub use crate::src::qcommon::net_chan::NET_StringToAdr;
pub use crate::src::qcommon::net_ip::NET_AdrToString;
pub use crate::src::qcommon::net_ip::NET_AdrToStringwPort;
pub use crate::src::qcommon::net_ip::NET_CompareBaseAdr;
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
pub use crate::src::qcommon::q_shared::Info_SetValueForKey;
pub use crate::src::qcommon::q_shared::Q_strcat;
pub use crate::src::qcommon::q_shared::Q_stricmp;
pub use crate::src::qcommon::q_shared::Q_strncmp;
pub use crate::src::qcommon::q_shared::Q_strncpyz;

pub use crate::src::qcommon::q_shared::TR_GRAVITY;
pub use crate::src::qcommon::q_shared::TR_INTERPOLATE;
pub use crate::src::qcommon::q_shared::TR_LINEAR;
pub use crate::src::qcommon::q_shared::TR_LINEAR_STOP;
pub use crate::src::qcommon::q_shared::TR_SINE;
pub use crate::src::qcommon::q_shared::TR_STATIONARY;
pub use crate::src::qcommon::vm::VM_Call;
pub use crate::src::server::sv_bot::SV_BotFrame;
pub use crate::src::server::sv_client::SV_AuthorizeIpPacket;
pub use crate::src::server::sv_client::SV_DirectConnect;
pub use crate::src::server::sv_client::SV_DropClient;
pub use crate::src::server::sv_client::SV_ExecuteClientMessage;
pub use crate::src::server::sv_client::SV_GetChallenge;
pub use crate::src::server::sv_client::SV_SendDownloadMessages;
pub use crate::src::server::sv_client::SV_SendQueuedMessages;
pub use crate::src::server::sv_game::SV_GameClientNum;
pub use crate::src::server::sv_init::SV_SetConfigstring;
pub use crate::src::server::sv_init::SV_Shutdown;
pub use crate::src::server::sv_net_chan::SV_Netchan_Process;
pub use crate::src::server::sv_snapshot::SV_SendClientMessages;
pub use crate::src::sys::sys_unix::Sys_Milliseconds;

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
#[no_mangle]

pub static mut sv_voip: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]

pub static mut sv_voipProtocol: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]

pub static mut svs: serverStatic_t = serverStatic_t {
    initialized: qfalse,
    time: 0,
    snapFlagServerBit: 0,
    clients: 0 as *const client_t as *mut client_t,
    numSnapshotEntities: 0,
    nextSnapshotEntities: 0,
    snapshotEntities: 0 as *const entityState_t as *mut entityState_t,
    nextHeartbeatTime: 0,
    challenges: [challenge_t {
        adr: netadr_t {
            type_0: NA_BAD,
            ip: [0; 4],
            ip6: [0; 16],
            port: 0,
            scope_id: 0,
        },
        challenge: 0,
        clientChallenge: 0,
        time: 0,
        pingTime: 0,
        firstTime: 0,
        wasrefused: qfalse,
        connected: qfalse,
    }; 2048],
    redirectAddress: netadr_t {
        type_0: NA_BAD,
        ip: [0; 4],
        ip6: [0; 16],
        port: 0,
        scope_id: 0,
    },
    authorizeAddress: netadr_t {
        type_0: NA_BAD,
        ip: [0; 4],
        ip6: [0; 16],
        port: 0,
        scope_id: 0,
    },
    masterResolveTime: [0; 5],
};
// persistant server info
#[no_mangle]

pub static mut sv: server_t = server_t {
    state: SS_DEAD,
    restarting: qfalse,
    serverId: 0,
    restartedServerId: 0,
    checksumFeed: 0,
    checksumFeedServerId: 0,
    snapshotCounter: 0,
    timeResidual: 0,
    nextFrameTime: 0,
    configstrings: [0 as *const libc::c_char as *mut libc::c_char; 1024],
    svEntities: [svEntity_t {
        worldSector: 0 as *const worldSector_s as *mut worldSector_s,
        nextEntityInWorldSector: 0 as *const svEntity_s as *mut svEntity_s,
        baseline: entityState_t {
            number: 0,
            eType: 0,
            eFlags: 0,
            pos: trajectory_t {
                trType: TR_STATIONARY,
                trTime: 0,
                trDuration: 0,
                trBase: [0.; 3],
                trDelta: [0.; 3],
            },
            apos: trajectory_t {
                trType: TR_STATIONARY,
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
        },
        numClusters: 0,
        clusternums: [0; 16],
        lastCluster: 0,
        areanum: 0,
        areanum2: 0,
        snapshotCounter: 0,
    }; 1024],
    entityParsePoint: 0 as *const libc::c_char as *mut libc::c_char,
    gentities: 0 as *const sharedEntity_t as *mut sharedEntity_t,
    gentitySize: 0,
    num_entities: 0,
    gameClients: 0 as *const playerState_t as *mut playerState_t,
    gameClientSize: 0,
    restartTime: 0,
    time: 0,
};
// local server
#[no_mangle]

pub static mut gvm: *mut vm_t = 0 as *const vm_t as *mut vm_t;
// game virtual machine
#[no_mangle]

pub static mut sv_fps: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
// time rate for running non-clients
#[no_mangle]

pub static mut sv_timeout: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
// seconds without any message
#[no_mangle]

pub static mut sv_zombietime: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
// seconds to sink messages after disconnect
#[no_mangle]

pub static mut sv_rconPassword: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
// password for remote server commands
#[no_mangle]

pub static mut sv_privatePassword: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
// password for the privateClient slots
#[no_mangle]

pub static mut sv_allowDownload: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]

pub static mut sv_maxclients: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]

pub static mut sv_privateClients: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
// number of clients reserved for password
#[no_mangle]

pub static mut sv_hostname: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]

pub static mut sv_master: [*mut cvar_t; 5] = [0 as *const cvar_t as *mut cvar_t; 5];
// master server ip address
#[no_mangle]

pub static mut sv_reconnectlimit: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
// minimum seconds between connect messages
#[no_mangle]

pub static mut sv_showloss: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
// report when usercmds are lost
#[no_mangle]

pub static mut sv_padPackets: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
// add nop bytes to messages
#[no_mangle]

pub static mut sv_killserver: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
// menu system can set to 1 to shut server down
#[no_mangle]

pub static mut sv_mapname: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]

pub static mut sv_mapChecksum: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]

pub static mut sv_serverid: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]

pub static mut sv_minRate: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]

pub static mut sv_maxRate: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]

pub static mut sv_dlRate: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]

pub static mut sv_minPing: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]

pub static mut sv_maxPing: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]

pub static mut sv_gametype: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]

pub static mut sv_pure: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]

pub static mut sv_floodProtect: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]

pub static mut sv_lanForceRate: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
// dedicated 1 (LAN) server forces local client rates to 99999 (bug #491)
#[no_mangle]

pub static mut sv_strictAuth: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]

pub static mut sv_banFile: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]

pub static mut serverBans: [serverBan_t; 1024] = [serverBan_t {
    ip: netadr_t {
        type_0: NA_BAD,
        ip: [0; 4],
        ip6: [0; 16],
        port: 0,
        scope_id: 0,
    },
    subnet: 0,
    isexception: qfalse,
}; 1024];
#[no_mangle]

pub static mut serverBansCount: i32 = 0 as i32;
/*
=============================================================================

EVENT MESSAGES

=============================================================================
*/
/*
===============
SV_ExpandNewlines

Converts newlines to "\n" so a line prints nicer
===============
*/

unsafe extern "C" fn SV_ExpandNewlines(mut in_0: *mut libc::c_char) -> *mut libc::c_char {
    static mut string: [libc::c_char; 1024] = [0; 1024];
    let mut l: i32 = 0;
    l = 0 as i32;
    while *in_0 as i32 != 0
        && (l as usize)
            < (::std::mem::size_of::<[libc::c_char; 1024]>() as usize)
                .wrapping_sub(3 as i32 as usize)
    {
        if *in_0 as i32 == '\n' as i32 {
            let fresh0 = l;
            l = l + 1;
            string[fresh0 as usize] = '\\' as i32 as libc::c_char;
            let fresh1 = l;
            l = l + 1;
            string[fresh1 as usize] = 'n' as i32 as libc::c_char
        } else {
            let fresh2 = l;
            l = l + 1;
            string[fresh2 as usize] = *in_0
        }
        in_0 = in_0.offset(1)
    }
    string[l as usize] = 0 as i32 as libc::c_char;
    return string.as_mut_ptr();
}
/*
======================
SV_ReplacePendingServerCommands

FIXME: This is ugly
======================
*/
// unused
/*
======================
SV_AddServerCommand

The given command will be transmitted to the client, and is guaranteed to
not have future snapshot_t executed before it is executed
======================
*/
#[no_mangle]

pub unsafe extern "C" fn SV_AddServerCommand(
    mut client: *mut client_t,
    mut cmd: *const libc::c_char,
) {
    let mut index: i32 = 0;
    let mut i: i32 = 0;
    // this is very ugly but it's also a waste to for instance send multiple config string updates
    // for the same config string index in one snapshot
    //	if ( SV_ReplacePendingServerCommands( client, cmd ) ) {
    //		return;
    //	}
    // do not send commands until the gamestate has been sent
    if ((*client).state as u32) < CS_PRIMED as i32 as u32 {
        return;
    }
    (*client).reliableSequence += 1;
    // if we would be losing an old command that hasn't been acknowledged,
    // we must drop the connection
    // we check == instead of >= so a broadcast print added by SV_DropClient()
    // doesn't cause a recursive drop client
    if (*client).reliableSequence - (*client).reliableAcknowledge == 64 as i32 + 1 as i32 {
        Com_Printf(
            b"===== pending server commands =====\n\x00" as *const u8 as *const libc::c_char,
        );
        i = (*client).reliableAcknowledge + 1 as i32;
        while i <= (*client).reliableSequence {
            Com_Printf(
                b"cmd %5d: %s\n\x00" as *const u8 as *const libc::c_char,
                i,
                (*client).reliableCommands[(i & 64 as i32 - 1 as i32) as usize].as_mut_ptr(),
            );
            i += 1
        }
        Com_Printf(
            b"cmd %5d: %s\n\x00" as *const u8 as *const libc::c_char,
            i,
            cmd,
        );
        SV_DropClient(
            client as *mut client_s,
            b"Server command overflow\x00" as *const u8 as *const libc::c_char,
        );
        return;
    }
    index = (*client).reliableSequence & 64 as i32 - 1 as i32;
    Q_strncpyz(
        (*client).reliableCommands[index as usize].as_mut_ptr(),
        cmd,
        ::std::mem::size_of::<[libc::c_char; 1024]>() as usize as i32,
    );
}
/*
=================
SV_SendServerCommand

Sends a reliable command string to be interpreted by
the client game module: "cp", "print", "chat", etc
A NULL client will broadcast to all clients
=================
*/
#[no_mangle]

pub unsafe extern "C" fn SV_SendServerCommand(
    mut cl: *mut client_t,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    let mut argptr: ::std::ffi::VaListImpl;
    let mut message: [byte; 16384] = [0; 16384];
    let mut client: *mut client_t = 0 as *mut client_t;
    let mut j: i32 = 0;
    argptr = args.clone();
    crate::stdlib::vsnprintf(
        message.as_mut_ptr() as *mut libc::c_char,
        ::std::mem::size_of::<[byte; 16384]>() as usize,
        fmt,
        argptr.as_va_list(),
    );
    // Fix to http://aluigi.altervista.org/adv/q3msgboom-adv.txt
    // The actual cause of the bug is probably further downstream
    // and should maybe be addressed later, but this certainly
    // fixes the problem for now
    if crate::stdlib::strlen(message.as_mut_ptr() as *mut libc::c_char)
        > 1022 as i32 as usize
    {
        return;
    }
    if !cl.is_null() {
        SV_AddServerCommand(cl, message.as_mut_ptr() as *mut libc::c_char);
        return;
    }
    // hack to echo broadcast prints to console
    if (*com_dedicated).integer != 0
        && crate::stdlib::strncmp(
            message.as_mut_ptr() as *mut libc::c_char,
            b"print\x00" as *const u8 as *const libc::c_char,
            5 as i32 as usize,
        ) == 0
    {
        Com_Printf(
            b"broadcast: %s\n\x00" as *const u8 as *const libc::c_char,
            SV_ExpandNewlines(message.as_mut_ptr() as *mut libc::c_char),
        );
    }
    // send the data to all relevant clients
    j = 0 as i32; // [2] for v4 and v6 address for the same address string.
    client = svs.clients;
    while j < (*sv_maxclients).integer {
        SV_AddServerCommand(client, message.as_mut_ptr() as *mut libc::c_char);
        j += 1;
        client = client.offset(1)
    }
}
#[no_mangle]

pub unsafe extern "C" fn SV_MasterHeartbeat(mut message: *const libc::c_char) {
    static mut adr: [[netadr_t; 2]; 5] = [[netadr_t {
        type_0: NA_BAD,
        ip: [0; 4],
        ip6: [0; 16],
        port: 0,
        scope_id: 0,
    }; 2]; 5];
    let mut i: i32 = 0;
    let mut res: i32 = 0;
    let mut netenabled: i32 = 0;
    netenabled = Cvar_VariableIntegerValue(b"net_enabled\x00" as *const u8 as *const libc::c_char);
    // "dedicated 1" is for lan play, "dedicated 2" is for inet public play
    if com_dedicated.is_null()
        || (*com_dedicated).integer != 2 as i32
        || netenabled & (0x1 as i32 | 0x2 as i32) == 0
    {
        return;
    } // only dedicated servers send heartbeats
      // if not time yet, don't send anything
    if svs.time < svs.nextHeartbeatTime {
        return;
    }
    if Q_stricmp(
        (*com_gamename).string,
        b"Quake3Arena\x00" as *const u8 as *const libc::c_char,
    ) == 0
    {
        message = b"QuakeArena-1\x00" as *const u8 as *const libc::c_char
    }
    svs.nextHeartbeatTime = svs.time + 300 as i32 * 1000 as i32;
    // send to group masters
    i = 0 as i32;
    while i < 5 as i32 {
        if !(*(*sv_master[i as usize]).string.offset(0 as i32 as isize) == 0) {
            // see if we haven't already resolved the name or if it's been over 24 hours
            // resolving usually causes hitches on win95, so only do it when needed
            if (*sv_master[i as usize]).modified as u32 != 0
                || svs.time > svs.masterResolveTime[i as usize]
            {
                (*sv_master[i as usize]).modified = qfalse;
                svs.masterResolveTime[i as usize] =
                    svs.time + 24 as i32 * 60 as i32 * 60 as i32 * 1000 as i32;
                if netenabled & 0x1 as i32 != 0 {
                    Com_Printf(
                        b"Resolving %s (IPv4)\n\x00" as *const u8 as *const libc::c_char,
                        (*sv_master[i as usize]).string,
                    );
                    res = NET_StringToAdr(
                        (*sv_master[i as usize]).string,
                        &mut *(*adr.as_mut_ptr().offset(i as isize))
                            .as_mut_ptr()
                            .offset(0 as i32 as isize) as *mut _
                            as *mut netadr_t,
                        NA_IP,
                    );
                    if res == 2 as i32 {
                        // if no port was specified, use the default master port
                        adr[i as usize][0 as i32 as usize].port =
                            crate::src::qcommon::q_shared::ShortSwap(27950 as i32 as i16) as u16
                    }
                    if res != 0 {
                        Com_Printf(
                            b"%s resolved to %s\n\x00" as *const u8 as *const libc::c_char,
                            (*sv_master[i as usize]).string,
                            NET_AdrToStringwPort(adr[i as usize][0 as i32 as usize] as netadr_t),
                        );
                    } else {
                        Com_Printf(
                            b"%s has no IPv4 address.\n\x00" as *const u8 as *const libc::c_char,
                            (*sv_master[i as usize]).string,
                        );
                    }
                }
                if netenabled & 0x2 as i32 != 0 {
                    Com_Printf(
                        b"Resolving %s (IPv6)\n\x00" as *const u8 as *const libc::c_char,
                        (*sv_master[i as usize]).string,
                    );
                    res = NET_StringToAdr(
                        (*sv_master[i as usize]).string,
                        &mut *(*adr.as_mut_ptr().offset(i as isize))
                            .as_mut_ptr()
                            .offset(1 as i32 as isize) as *mut _
                            as *mut netadr_t,
                        NA_IP6,
                    );
                    if res == 2 as i32 {
                        // if no port was specified, use the default master port
                        adr[i as usize][1 as i32 as usize].port =
                            crate::src::qcommon::q_shared::ShortSwap(27950 as i32 as i16) as u16
                    }
                    if res != 0 {
                        Com_Printf(
                            b"%s resolved to %s\n\x00" as *const u8 as *const libc::c_char,
                            (*sv_master[i as usize]).string,
                            NET_AdrToStringwPort(adr[i as usize][1 as i32 as usize] as netadr_t),
                        );
                    } else {
                        Com_Printf(
                            b"%s has no IPv6 address.\n\x00" as *const u8 as *const libc::c_char,
                            (*sv_master[i as usize]).string,
                        );
                    }
                }
            }
            if !(adr[i as usize][0 as i32 as usize].type_0 as u32 == NA_BAD as i32 as u32
                && adr[i as usize][1 as i32 as usize].type_0 as u32 == NA_BAD as i32 as u32)
            {
                Com_Printf(
                    b"Sending heartbeat to %s\n\x00" as *const u8 as *const libc::c_char,
                    (*sv_master[i as usize]).string,
                );
                // this command should be changed if the server info / status format
                // ever incompatably changes
                if adr[i as usize][0 as i32 as usize].type_0 as u32 != NA_BAD as i32 as u32 {
                    NET_OutOfBandPrint(
                        NS_SERVER,
                        adr[i as usize][0 as i32 as usize] as netadr_t,
                        b"heartbeat %s\n\x00" as *const u8 as *const libc::c_char,
                        message,
                    );
                }
                if adr[i as usize][1 as i32 as usize].type_0 as u32 != NA_BAD as i32 as u32 {
                    NET_OutOfBandPrint(
                        NS_SERVER,
                        adr[i as usize][1 as i32 as usize] as netadr_t,
                        b"heartbeat %s\n\x00" as *const u8 as *const libc::c_char,
                        message,
                    );
                }
            }
        }
        i += 1
    }
}
/*
=================
SV_MasterShutdown

Informs all masters that this server is going down
=================
*/
#[no_mangle]

pub unsafe extern "C" fn SV_MasterShutdown() {
    // send a heartbeat right now
    svs.nextHeartbeatTime = -(9999 as i32);
    SV_MasterHeartbeat(b"DarkPlaces\x00" as *const u8 as *const libc::c_char);
    // send it again to minimize chance of drops
    svs.nextHeartbeatTime = -(9999 as i32);
    SV_MasterHeartbeat(b"DarkPlaces\x00" as *const u8 as *const libc::c_char);
    // when the master tries to poll the server, it won't respond, so
    // it will be removed from the list
}

static mut buckets: [leakyBucket_t; 16384] = [leakyBucket_t {
    type_0: NA_BAD,
    ipv: C2RustUnnamed_164 { _4: [0; 4] },
    lastTime: 0,
    burst: 0,
    hash: 0,
    prev: 0 as *const leakyBucket_t as *mut leakyBucket_t,
    next: 0 as *const leakyBucket_t as *mut leakyBucket_t,
}; 16384];

static mut bucketHashes: [*mut leakyBucket_t; 1024] =
    [0 as *const leakyBucket_t as *mut leakyBucket_t; 1024];
#[no_mangle]

pub static mut outboundLeakyBucket: leakyBucket_t = leakyBucket_t {
    type_0: NA_BAD,
    ipv: C2RustUnnamed_164 { _4: [0; 4] },
    lastTime: 0,
    burst: 0,
    hash: 0,
    prev: 0 as *const leakyBucket_t as *mut leakyBucket_t,
    next: 0 as *const leakyBucket_t as *mut leakyBucket_t,
};
/*
================
SVC_HashForAddress
================
*/

unsafe extern "C" fn SVC_HashForAddress(mut address: netadr_t) -> isize {
    let mut ip: *mut byte = 0 as *mut byte;
    let mut size: size_t = 0 as i32 as size_t;
    let mut i: i32 = 0;
    let mut hash: isize = 0 as i32 as isize;
    match address.type_0 as u32 {
        4 => {
            ip = address.ip.as_mut_ptr();
            size = 4 as i32 as size_t
        }
        5 => {
            ip = address.ip6.as_mut_ptr();
            size = 16 as i32 as size_t
        }
        _ => {}
    }
    i = 0 as i32;
    while (i as usize) < size {
        hash += *ip.offset(i as isize) as isize * (i + 119 as i32) as isize;
        i += 1
    }
    hash = hash ^ hash >> 10 as i32 ^ hash >> 20 as i32;
    hash &= (1024 as i32 - 1 as i32) as isize;
    return hash;
}
/*
================
SVC_BucketForAddress

Find or allocate a bucket for an address
================
*/

unsafe extern "C" fn SVC_BucketForAddress(
    mut address: netadr_t,
    mut burst: i32,
    mut period: i32,
) -> *mut leakyBucket_t {
    let mut bucket: *mut leakyBucket_t = 0 as *mut leakyBucket_t;
    let mut i: i32 = 0;
    let mut hash: isize = SVC_HashForAddress(address);
    let mut now: i32 = Sys_Milliseconds();
    bucket = bucketHashes[hash as usize];
    while !bucket.is_null() {
        match (*bucket).type_0 as u32 {
            4 => {
                if crate::stdlib::memcmp(
                    (*bucket).ipv._4.as_mut_ptr() as *const libc::c_void,
                    address.ip.as_mut_ptr() as *const libc::c_void,
                    4 as i32 as usize,
                ) == 0 as i32
                {
                    return bucket;
                }
            }
            5 => {
                if crate::stdlib::memcmp(
                    (*bucket).ipv._6.as_mut_ptr() as *const libc::c_void,
                    address.ip6.as_mut_ptr() as *const libc::c_void,
                    16 as i32 as usize,
                ) == 0 as i32
                {
                    return bucket;
                }
            }
            _ => {}
        }
        bucket = (*bucket).next
    }
    i = 0 as i32;
    while i < 16384 as i32 {
        let mut interval: i32 = 0;
        bucket = &mut *buckets.as_mut_ptr().offset(i as isize) as *mut leakyBucket_t;
        interval = now - (*bucket).lastTime;
        // Reclaim expired buckets
        if (*bucket).lastTime > 0 as i32 && (interval > burst * period || interval < 0 as i32) {
            if !(*bucket).prev.is_null() {
                (*(*bucket).prev).next = (*bucket).next
            } else {
                bucketHashes[(*bucket).hash as usize] = (*bucket).next
            }
            if !(*bucket).next.is_null() {
                (*(*bucket).next).prev = (*bucket).prev
            }
            crate::stdlib::memset(
                bucket as *mut libc::c_void,
                0 as i32,
                ::std::mem::size_of::<leakyBucket_t>() as usize,
            );
        }
        if (*bucket).type_0 as u32 == NA_BAD as i32 as u32 {
            (*bucket).type_0 = address.type_0;
            match address.type_0 as u32 {
                4 => {
                    crate::stdlib::memcpy(
                        (*bucket).ipv._4.as_mut_ptr() as *mut libc::c_void,
                        address.ip.as_mut_ptr() as *const libc::c_void,
                        4 as i32 as usize,
                    );
                }
                5 => {
                    crate::stdlib::memcpy(
                        (*bucket).ipv._6.as_mut_ptr() as *mut libc::c_void,
                        address.ip6.as_mut_ptr() as *const libc::c_void,
                        16 as i32 as usize,
                    );
                }
                _ => {}
            }
            (*bucket).lastTime = now;
            (*bucket).burst = 0 as i32 as i8;
            (*bucket).hash = hash;
            // Add to the head of the relevant hash chain
            (*bucket).next = bucketHashes[hash as usize];
            if !bucketHashes[hash as usize].is_null() {
                (*bucketHashes[hash as usize]).prev = bucket
            }
            (*bucket).prev = 0 as *mut leakyBucket_t;
            bucketHashes[hash as usize] = bucket;
            return bucket;
        }
        i += 1
    }
    // Couldn't allocate a bucket for this address
    return 0 as *mut leakyBucket_t;
}
/*
================
SVC_RateLimit
================
*/
#[no_mangle]

pub unsafe extern "C" fn SVC_RateLimit(
    mut bucket: *mut leakyBucket_t,
    mut burst: i32,
    mut period: i32,
) -> qboolean {
    if !bucket.is_null() {
        let mut now: i32 = Sys_Milliseconds();
        let mut interval: i32 = now - (*bucket).lastTime;
        let mut expired: i32 = interval / period;
        let mut expiredRemainder: i32 = interval % period;
        if expired > (*bucket).burst as i32 || interval < 0 as i32 {
            (*bucket).burst = 0 as i32 as i8;
            (*bucket).lastTime = now
        } else {
            (*bucket).burst = ((*bucket).burst as i32 - expired) as i8;
            (*bucket).lastTime = now - expiredRemainder
        }
        if ((*bucket).burst as i32) < burst {
            (*bucket).burst += 1;
            return qfalse;
        }
    }
    return qtrue;
}
/*
================
SVC_RateLimitAddress

Rate limit for a particular address
================
*/
#[no_mangle]

pub unsafe extern "C" fn SVC_RateLimitAddress(
    mut from: netadr_t,
    mut burst: i32,
    mut period: i32,
) -> qboolean {
    let mut bucket: *mut leakyBucket_t = SVC_BucketForAddress(from, burst, period);
    return SVC_RateLimit(bucket, burst, period);
}
/*
================
SVC_Status

Responds with all the info that qplug or qspy can see about the server
and all connected players.  Used for getting detailed information after
the simple info query.
================
*/

unsafe extern "C" fn SVC_Status(mut from: netadr_t) {
    let mut player: [libc::c_char; 1024] = [0; 1024];
    let mut status: [libc::c_char; 16384] = [0; 16384];
    let mut i: i32 = 0;
    let mut cl: *mut client_t = 0 as *mut client_t;
    let mut ps: *mut playerState_t = 0 as *mut playerState_t;
    let mut statusLength: i32 = 0;
    let mut playerLength: i32 = 0;
    let mut infostring: [libc::c_char; 1024] = [0; 1024];
    // ignore if we are in single player
    if Cvar_VariableValue(b"g_gametype\x00" as *const u8 as *const libc::c_char)
        == GT_SINGLE_PLAYER as i32 as f32
        || Cvar_VariableValue(b"ui_singlePlayerActive\x00" as *const u8 as *const libc::c_char)
            != 0.
    {
        return;
    }
    // Prevent using getstatus as an amplifier
    if SVC_RateLimitAddress(from, 10 as i32, 1000 as i32) as u64 != 0 {
        Com_DPrintf(
            b"SVC_Status: rate limit from %s exceeded, dropping request\n\x00" as *const u8
                as *const libc::c_char,
            NET_AdrToString(from as netadr_t),
        );
        return;
    }
    // Allow getstatus to be DoSed relatively easily, but prevent
    // excess outbound bandwidth usage when being flooded inbound
    if SVC_RateLimit(&mut outboundLeakyBucket, 10 as i32, 100 as i32) as u64 != 0 {
        Com_DPrintf(
            b"SVC_Status: rate limit exceeded, dropping request\n\x00" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    // A maximum challenge length of 128 should be more than plenty.
    if crate::stdlib::strlen(Cmd_Argv(1 as i32)) > 128 as i32 as usize {
        return;
    }
    libc::strcpy(infostring.as_mut_ptr(), Cvar_InfoString(0x4 as i32));
    // echo back the parameter to status. so master servers can use it as a challenge
    // to prevent timed spoofed reply packets that add ghost servers
    Info_SetValueForKey(
        infostring.as_mut_ptr(),
        b"challenge\x00" as *const u8 as *const libc::c_char,
        Cmd_Argv(1 as i32),
    );
    status[0 as i32 as usize] = 0 as i32 as libc::c_char;
    statusLength = 0 as i32;
    i = 0 as i32;
    while i < (*sv_maxclients).integer {
        cl = &mut *svs.clients.offset(i as isize) as *mut client_t;
        if (*cl).state as u32 >= CS_CONNECTED as i32 as u32 {
            ps = SV_GameClientNum(i) as *mut playerState_s;
            Com_sprintf(
                player.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 1024]>() as usize as i32,
                b"%i %i \"%s\"\n\x00" as *const u8 as *const libc::c_char,
                (*ps).persistant[0 as i32 as usize],
                (*cl).ping,
                (*cl).name.as_mut_ptr(),
            );
            playerLength = crate::stdlib::strlen(player.as_mut_ptr()) as i32;
            if (statusLength + playerLength) as usize
                >= ::std::mem::size_of::<[libc::c_char; 16384]>() as usize
            {
                break;
            }
            libc::strcpy(
                status.as_mut_ptr().offset(statusLength as isize),
                player.as_mut_ptr(),
            );
            statusLength += playerLength
        }
        i += 1
    }
    NET_OutOfBandPrint(
        NS_SERVER,
        from as netadr_t,
        b"statusResponse\n%s\n%s\x00" as *const u8 as *const libc::c_char,
        infostring.as_mut_ptr(),
        status.as_mut_ptr(),
    );
}
/*
================
SVC_Info

Responds with a short info message that should be enough to determine
if a user is interested in a server to do a full status
================
*/
#[no_mangle]

pub unsafe extern "C" fn SVC_Info(mut from: netadr_t) {
    let mut i: i32 = 0;
    let mut count: i32 = 0;
    let mut humans: i32 = 0;
    let mut gamedir: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut infostring: [libc::c_char; 1024] = [0; 1024];
    // ignore if we are in single player
    if Cvar_VariableValue(b"g_gametype\x00" as *const u8 as *const libc::c_char)
        == GT_SINGLE_PLAYER as i32 as f32
        || Cvar_VariableValue(b"ui_singlePlayerActive\x00" as *const u8 as *const libc::c_char)
            != 0.
    {
        return;
    }
    // Prevent using getinfo as an amplifier
    if SVC_RateLimitAddress(from, 10 as i32, 1000 as i32) as u64 != 0 {
        Com_DPrintf(
            b"SVC_Info: rate limit from %s exceeded, dropping request\n\x00" as *const u8
                as *const libc::c_char,
            NET_AdrToString(from as netadr_t),
        );
        return;
    }
    // Allow getinfo to be DoSed relatively easily, but prevent
    // excess outbound bandwidth usage when being flooded inbound
    if SVC_RateLimit(&mut outboundLeakyBucket, 10 as i32, 100 as i32) as u64 != 0 {
        Com_DPrintf(
            b"SVC_Info: rate limit exceeded, dropping request\n\x00" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    /*
     * Check whether Cmd_Argv(1) has a sane length. This was not done in the original Quake3 version which led
     * to the Infostring bug discovered by Luigi Auriemma. See http://aluigi.altervista.org/ for the advisory.
     */
    // A maximum challenge length of 128 should be more than plenty.
    if crate::stdlib::strlen(Cmd_Argv(1 as i32)) > 128 as i32 as usize {
        return;
    }
    // don't count privateclients
    humans = 0 as i32;
    count = humans;
    i = (*sv_privateClients).integer;
    while i < (*sv_maxclients).integer {
        if (*svs.clients.offset(i as isize)).state as u32 >= CS_CONNECTED as i32 as u32 {
            count += 1;
            if (*svs.clients.offset(i as isize))
                .netchan
                .remoteAddress
                .type_0 as u32
                != NA_BOT as i32 as u32
            {
                humans += 1
            }
        }
        i += 1
    }
    infostring[0 as i32 as usize] = 0 as i32 as libc::c_char;
    // echo back the parameter to status. so servers can use it as a challenge
    // to prevent timed spoofed reply packets that add ghost servers
    Info_SetValueForKey(
        infostring.as_mut_ptr(),
        b"challenge\x00" as *const u8 as *const libc::c_char,
        Cmd_Argv(1 as i32),
    );
    Info_SetValueForKey(
        infostring.as_mut_ptr(),
        b"gamename\x00" as *const u8 as *const libc::c_char,
        (*com_gamename).string,
    );
    if (*com_legacyprotocol).integer > 0 as i32 {
        Info_SetValueForKey(
            infostring.as_mut_ptr(),
            b"protocol\x00" as *const u8 as *const libc::c_char,
            va(
                b"%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                (*com_legacyprotocol).integer,
            ),
        );
    } else {
        Info_SetValueForKey(
            infostring.as_mut_ptr(),
            b"protocol\x00" as *const u8 as *const libc::c_char,
            va(
                b"%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                (*com_protocol).integer,
            ),
        );
    }
    Info_SetValueForKey(
        infostring.as_mut_ptr(),
        b"hostname\x00" as *const u8 as *const libc::c_char,
        (*sv_hostname).string,
    );
    Info_SetValueForKey(
        infostring.as_mut_ptr(),
        b"mapname\x00" as *const u8 as *const libc::c_char,
        (*sv_mapname).string,
    );
    Info_SetValueForKey(
        infostring.as_mut_ptr(),
        b"clients\x00" as *const u8 as *const libc::c_char,
        va(
            b"%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            count,
        ),
    );
    Info_SetValueForKey(
        infostring.as_mut_ptr(),
        b"g_humanplayers\x00" as *const u8 as *const libc::c_char,
        va(
            b"%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            humans,
        ),
    );
    Info_SetValueForKey(
        infostring.as_mut_ptr(),
        b"sv_maxclients\x00" as *const u8 as *const libc::c_char,
        va(
            b"%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            (*sv_maxclients).integer - (*sv_privateClients).integer,
        ),
    );
    Info_SetValueForKey(
        infostring.as_mut_ptr(),
        b"gametype\x00" as *const u8 as *const libc::c_char,
        va(
            b"%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            (*sv_gametype).integer,
        ),
    );
    Info_SetValueForKey(
        infostring.as_mut_ptr(),
        b"pure\x00" as *const u8 as *const libc::c_char,
        va(
            b"%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            (*sv_pure).integer,
        ),
    );
    Info_SetValueForKey(
        infostring.as_mut_ptr(),
        b"g_needpass\x00" as *const u8 as *const libc::c_char,
        va(
            b"%d\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            Cvar_VariableIntegerValue(b"g_needpass\x00" as *const u8 as *const libc::c_char),
        ),
    );
    if !(*sv_voipProtocol).string.is_null() && *(*sv_voipProtocol).string as i32 != 0 {
        Info_SetValueForKey(
            infostring.as_mut_ptr(),
            b"voip\x00" as *const u8 as *const libc::c_char,
            (*sv_voipProtocol).string,
        );
    }
    if (*sv_minPing).integer != 0 {
        Info_SetValueForKey(
            infostring.as_mut_ptr(),
            b"minPing\x00" as *const u8 as *const libc::c_char,
            va(
                b"%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                (*sv_minPing).integer,
            ),
        );
    }
    if (*sv_maxPing).integer != 0 {
        Info_SetValueForKey(
            infostring.as_mut_ptr(),
            b"maxPing\x00" as *const u8 as *const libc::c_char,
            va(
                b"%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                (*sv_maxPing).integer,
            ),
        );
    }
    gamedir = Cvar_VariableString(b"fs_game\x00" as *const u8 as *const libc::c_char);
    if *gamedir != 0 {
        Info_SetValueForKey(
            infostring.as_mut_ptr(),
            b"game\x00" as *const u8 as *const libc::c_char,
            gamedir,
        );
    }
    NET_OutOfBandPrint(
        NS_SERVER,
        from as netadr_t,
        b"infoResponse\n%s\x00" as *const u8 as *const libc::c_char,
        infostring.as_mut_ptr(),
    );
}
/*
================
SVC_FlushRedirect

================
*/

unsafe extern "C" fn SV_FlushRedirect(mut outputbuf: *mut libc::c_char) {
    NET_OutOfBandPrint(
        NS_SERVER,
        svs.redirectAddress as netadr_t,
        b"print\n%s\x00" as *const u8 as *const libc::c_char,
        outputbuf,
    );
}
/*
===============
SVC_RemoteCommand

An rcon packet arrived from the network.
Shift down the remaining args
Redirect all printfs
===============
*/

unsafe extern "C" fn SVC_RemoteCommand(mut from: netadr_t, mut _msg: *mut msg_t) {
    let mut valid: qboolean = qfalse;
    let mut remaining: [libc::c_char; 1024] = [0; 1024];
    // TTimo - scaled down to accumulate, but not overflow anything network wise, print wise etc.
    // (OOB messages are the bottleneck here)
    let mut sv_outputbuf: [libc::c_char; 1008] = [0; 1008];
    let mut cmd_aux: *mut libc::c_char = 0 as *mut libc::c_char;
    // Prevent using rcon as an amplifier and make dictionary attacks impractical
    if SVC_RateLimitAddress(from, 10 as i32, 1000 as i32) as u64 != 0 {
        Com_DPrintf(
            b"SVC_RemoteCommand: rate limit from %s exceeded, dropping request\n\x00" as *const u8
                as *const libc::c_char,
            NET_AdrToString(from as netadr_t),
        );
        return;
    }
    if crate::stdlib::strlen((*sv_rconPassword).string) == 0
        || libc::strcmp(Cmd_Argv(1 as i32), (*sv_rconPassword).string) != 0
    {
        static mut bucket: leakyBucket_t = leakyBucket_t {
            type_0: NA_BAD,
            ipv: C2RustUnnamed_164 { _4: [0; 4] },
            lastTime: 0,
            burst: 0,
            hash: 0,
            prev: 0 as *const leakyBucket_t as *mut leakyBucket_t,
            next: 0 as *const leakyBucket_t as *mut leakyBucket_t,
        };
        // Make DoS via rcon impractical
        if SVC_RateLimit(&mut bucket, 10 as i32, 1000 as i32) as u64 != 0 {
            Com_DPrintf(
                b"SVC_RemoteCommand: rate limit exceeded, dropping request\n\x00" as *const u8
                    as *const libc::c_char,
            );
            return;
        }
        valid = qfalse;
        Com_Printf(
            b"Bad rcon from %s: %s\n\x00" as *const u8 as *const libc::c_char,
            NET_AdrToString(from as netadr_t),
            Cmd_ArgsFrom(2 as i32),
        );
    } else {
        valid = qtrue;
        Com_Printf(
            b"Rcon from %s: %s\n\x00" as *const u8 as *const libc::c_char,
            NET_AdrToString(from as netadr_t),
            Cmd_ArgsFrom(2 as i32),
        );
    }
    // start redirecting all print outputs to the packet
    svs.redirectAddress = from;
    Com_BeginRedirect(
        sv_outputbuf.as_mut_ptr(),
        1024 as i32 - 16 as i32,
        Some(SV_FlushRedirect as unsafe extern "C" fn(_: *mut libc::c_char) -> ()),
    );
    if crate::stdlib::strlen((*sv_rconPassword).string) == 0 {
        Com_Printf(b"No rconpassword set on the server.\n\x00" as *const u8 as *const libc::c_char);
    } else if valid as u64 == 0 {
        Com_Printf(b"Bad rconpassword.\n\x00" as *const u8 as *const libc::c_char);
    } else {
        remaining[0 as i32 as usize] = 0 as i32 as libc::c_char;
        // https://zerowing.idsoftware.com/bugzilla/show_bug.cgi?id=543
        // get the command directly, "rcon <pass> <command>" to avoid quoting issues
        // extract the command by walking
        // since the cmd formatting can fuckup (amount of spaces), using a dumb step by step parsing
        cmd_aux = Cmd_Cmd();
        cmd_aux = cmd_aux.offset(4 as i32 as isize);
        while *cmd_aux.offset(0 as i32 as isize) as i32 == ' ' as i32 {
            cmd_aux = cmd_aux.offset(1)
        }
        while *cmd_aux.offset(0 as i32 as isize) as i32 != 0
            && *cmd_aux.offset(0 as i32 as isize) as i32 != ' ' as i32
        {
            // password
            cmd_aux = cmd_aux.offset(1)
        }
        while *cmd_aux.offset(0 as i32 as isize) as i32 == ' ' as i32 {
            cmd_aux = cmd_aux.offset(1)
        }
        Q_strcat(
            remaining.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as usize as i32,
            cmd_aux,
        );
        Cmd_ExecuteString(remaining.as_mut_ptr());
    }
    Com_EndRedirect();
}
/*
=================
SV_ConnectionlessPacket

A connectionless packet has four leading 0xff
characters to distinguish it from a game channel.
Clients that are in the game can still send
connectionless packets.
=================
*/

unsafe extern "C" fn SV_ConnectionlessPacket(mut from: netadr_t, mut msg: *mut msg_t) {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char; // skip the -1 marker
    let mut c: *mut libc::c_char = 0 as *mut libc::c_char;
    MSG_BeginReadingOOB(msg as *mut msg_t);
    MSG_ReadLong(msg as *mut msg_t);
    if Q_strncmp(
        b"connect\x00" as *const u8 as *const libc::c_char,
        &mut *(*msg).data.offset(4 as i32 as isize) as *mut byte as *mut libc::c_char,
        7 as i32,
    ) == 0
    {
        Huff_Decompress(msg as *mut msg_t, 12 as i32);
    }
    s = MSG_ReadStringLine(msg as *mut msg_t);
    Cmd_TokenizeString(s);
    c = Cmd_Argv(0 as i32);
    Com_DPrintf(
        b"SV packet %s : %s\n\x00" as *const u8 as *const libc::c_char,
        NET_AdrToString(from as netadr_t),
        c,
    );
    if Q_stricmp(c, b"getstatus\x00" as *const u8 as *const libc::c_char) == 0 {
        SVC_Status(from);
    } else if Q_stricmp(c, b"getinfo\x00" as *const u8 as *const libc::c_char) == 0 {
        SVC_Info(from);
    } else if Q_stricmp(c, b"getchallenge\x00" as *const u8 as *const libc::c_char) == 0 {
        SV_GetChallenge(from as netadr_t);
    } else if Q_stricmp(c, b"connect\x00" as *const u8 as *const libc::c_char) == 0 {
        SV_DirectConnect(from as netadr_t);
    } else if Q_stricmp(c, b"ipAuthorize\x00" as *const u8 as *const libc::c_char) == 0 {
        SV_AuthorizeIpPacket(from as netadr_t);
    } else if Q_stricmp(c, b"rcon\x00" as *const u8 as *const libc::c_char) == 0 {
        SVC_RemoteCommand(from, msg);
    } else if !(Q_stricmp(c, b"disconnect\x00" as *const u8 as *const libc::c_char) == 0) {
        Com_DPrintf(
            b"bad connectionless packet from %s:\n%s\n\x00" as *const u8 as *const libc::c_char,
            NET_AdrToString(from as netadr_t),
            s,
        );
    };
}
//============================================================================
/*
=================
SV_PacketEvent
=================
*/
#[no_mangle]

pub unsafe extern "C" fn SV_PacketEvent(mut from: netadr_t, mut msg: *mut msg_t) {
    let mut i: i32 = 0;
    let mut cl: *mut client_t = 0 as *mut client_t;
    let mut qport: i32 = 0;
    // check for connectionless packet (0xffffffff) first
    if (*msg).cursize >= 4 as i32 && *((*msg).data as *mut i32) == -(1 as i32) {
        SV_ConnectionlessPacket(from, msg);
        return;
    }
    // read the qport out of the message so we can fix up
    // stupid address translating routers
    MSG_BeginReadingOOB(msg as *mut msg_t); // sequence number
    MSG_ReadLong(msg as *mut msg_t);
    qport = MSG_ReadShort(msg as *mut msg_t) & 0xffff as i32;
    // find which client the message is from
    i = 0 as i32;
    cl = svs.clients;
    while i < (*sv_maxclients).integer {
        if !((*cl).state as u32 == CS_FREE as i32 as u32) {
            if !(NET_CompareBaseAdr(from as netadr_t, (*cl).netchan.remoteAddress as netadr_t)
                as u64
                == 0)
            {
                // it is possible to have multiple clients from a single IP
                // address, so they are differentiated by the qport variable
                if !((*cl).netchan.qport != qport) {
                    // the IP port can't be used to differentiate them, because
                    // some address translating routers periodically change UDP
                    // port assignments
                    if (*cl).netchan.remoteAddress.port as i32 != from.port as i32 {
                        Com_Printf(
                            b"SV_PacketEvent: fixing up a translated port\n\x00" as *const u8
                                as *const libc::c_char,
                        );
                        (*cl).netchan.remoteAddress.port = from.port
                    }
                    // make sure it is a valid, in sequence packet
                    if SV_Netchan_Process(cl as *mut client_s, msg as *mut msg_t) as u64 != 0 {
                        // zombie clients still need to do the Netchan_Process
                        // to make sure they don't need to retransmit the final
                        // reliable message, but they don't do any other processing
                        if (*cl).state as u32 != CS_ZOMBIE as i32 as u32 {
                            (*cl).lastPacketTime = svs.time; // don't timeout
                            SV_ExecuteClientMessage(cl as *mut client_s, msg as *mut msg_t);
                        }
                    }
                    return;
                }
            }
        }
        i += 1;
        cl = cl.offset(1)
    }
}
/*
===================
SV_CalcPings

Updates the cl->ping variables
===================
*/

unsafe extern "C" fn SV_CalcPings() {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut cl: *mut client_t = 0 as *mut client_t;
    let mut total: i32 = 0;
    let mut count: i32 = 0;
    let mut delta: i32 = 0;
    let mut ps: *mut playerState_t = 0 as *mut playerState_t;
    i = 0 as i32;
    while i < (*sv_maxclients).integer {
        cl = &mut *svs.clients.offset(i as isize) as *mut client_t;
        if (*cl).state as u32 != CS_ACTIVE as i32 as u32 {
            (*cl).ping = 999 as i32
        } else if (*cl).gentity.is_null() {
            (*cl).ping = 999 as i32
        } else if (*(*cl).gentity).r.svFlags & 0x8 as i32 != 0 {
            (*cl).ping = 0 as i32
        } else {
            total = 0 as i32;
            count = 0 as i32;
            j = 0 as i32;
            while j < 32 as i32 {
                if !((*cl).frames[j as usize].messageAcked <= 0 as i32) {
                    delta = (*cl).frames[j as usize].messageAcked
                        - (*cl).frames[j as usize].messageSent;
                    count += 1;
                    total += delta
                }
                j += 1
            }
            if count == 0 {
                (*cl).ping = 999 as i32
            } else {
                (*cl).ping = total / count;
                if (*cl).ping > 999 as i32 {
                    (*cl).ping = 999 as i32
                }
            }
            // let the game dll know about the ping
            ps = SV_GameClientNum(i) as *mut playerState_s;
            (*ps).ping = (*cl).ping
        }
        i += 1
    }
}
/*
==================
SV_CheckTimeouts

If a packet has not been received from a client for timeout->integer
seconds, drop the conneciton.  Server time is used instead of
realtime to avoid dropping the local client while debugging.

When a client is normally dropped, the client_t goes into a zombie state
for a few seconds to make sure any final reliable message gets resent
if necessary
==================
*/

unsafe extern "C" fn SV_CheckTimeouts() {
    let mut i: i32 = 0;
    let mut cl: *mut client_t = 0 as *mut client_t;
    let mut droppoint: i32 = 0;
    let mut zombiepoint: i32 = 0;
    droppoint = svs.time - 1000 as i32 * (*sv_timeout).integer;
    zombiepoint = svs.time - 1000 as i32 * (*sv_zombietime).integer;
    i = 0 as i32;
    cl = svs.clients;
    while i < (*sv_maxclients).integer {
        // message times may be wrong across a changelevel
        if (*cl).lastPacketTime > svs.time {
            (*cl).lastPacketTime = svs.time
        }
        if (*cl).state as u32 == CS_ZOMBIE as i32 as u32 && (*cl).lastPacketTime < zombiepoint {
            // using the client id cause the cl->name is empty at this point
            Com_DPrintf(
                b"Going from CS_ZOMBIE to CS_FREE for client %d\n\x00" as *const u8
                    as *const libc::c_char,
                i,
            ); // can now be reused
            (*cl).state = CS_FREE
        } else if (*cl).state as u32 >= CS_CONNECTED as i32 as u32
            && (*cl).lastPacketTime < droppoint
        {
            // wait several frames so a debugger session doesn't
            // cause a timeout
            (*cl).timeoutCount += 1;
            if (*cl).timeoutCount > 5 as i32 {
                SV_DropClient(
                    cl as *mut client_s,
                    b"timed out\x00" as *const u8 as *const libc::c_char,
                );
                (*cl).state = CS_FREE
                // don't bother with zombie state
            }
        } else {
            (*cl).timeoutCount = 0 as i32
        }
        i += 1;
        cl = cl.offset(1)
    }
}
/*
==================
SV_CheckPaused
==================
*/

unsafe extern "C" fn SV_CheckPaused() -> qboolean {
    let mut count: i32 = 0;
    let mut cl: *mut client_t = 0 as *mut client_t;
    let mut i: i32 = 0;
    if (*cl_paused).integer == 0 {
        return qfalse;
    }
    // only pause if there is just a single client connected
    count = 0 as i32;
    i = 0 as i32;
    cl = svs.clients;
    while i < (*sv_maxclients).integer {
        if (*cl).state as u32 >= CS_CONNECTED as i32 as u32
            && (*cl).netchan.remoteAddress.type_0 as u32 != NA_BOT as i32 as u32
        {
            count += 1
        }
        i += 1;
        cl = cl.offset(1)
    }
    if count > 1 as i32 {
        // don't pause
        if (*sv_paused).integer != 0 {
            Cvar_Set(
                b"sv_paused\x00" as *const u8 as *const libc::c_char,
                b"0\x00" as *const u8 as *const libc::c_char,
            );
        }
        return qfalse;
    }
    if (*sv_paused).integer == 0 {
        Cvar_Set(
            b"sv_paused\x00" as *const u8 as *const libc::c_char,
            b"1\x00" as *const u8 as *const libc::c_char,
        );
    }
    return qtrue;
}
/*
==================
SV_FrameMsec
Return time in millseconds until processing of the next server frame.
==================
*/
#[no_mangle]

pub unsafe extern "C" fn SV_FrameMsec() -> i32 {
    if !sv_fps.is_null() {
        let mut frameMsec: i32 = 0;
        frameMsec = (1000.0f32 / (*sv_fps).value) as i32;
        if frameMsec < sv.timeResidual {
            return 0 as i32;
        } else {
            return frameMsec - sv.timeResidual;
        }
    } else {
        return 1 as i32;
    };
}
/*
==================
SV_Frame

Player movement occurs as a result of packet events, which
happen before SV_Frame is called
==================
*/
#[no_mangle]

pub unsafe extern "C" fn SV_Frame(mut msec: i32) {
    let mut frameMsec: i32 = 0;
    let mut startTime: i32 = 0;
    // the menu kills the server with this cvar
    if (*sv_killserver).integer != 0 {
        SV_Shutdown(
            b"Server was killed\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        Cvar_Set(
            b"sv_killserver\x00" as *const u8 as *const libc::c_char,
            b"0\x00" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if (*com_sv_running).integer == 0 {
        // Running as a server, but no map loaded
        return;
    }
    // allow pause if only the local client is connected
    if SV_CheckPaused() as u64 != 0 {
        return;
    }
    // if it isn't time for the next frame, do nothing
    if (*sv_fps).integer < 1 as i32 {
        Cvar_Set(
            b"sv_fps\x00" as *const u8 as *const libc::c_char,
            b"10\x00" as *const u8 as *const libc::c_char,
        );
    }
    frameMsec = ((1000 as i32 / (*sv_fps).integer) as f32 * (*com_timescale).value) as i32;
    // don't let it scale below 1ms
    if frameMsec < 1 as i32 {
        Cvar_Set(
            b"timescale\x00" as *const u8 as *const libc::c_char,
            va(
                b"%f\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ((*sv_fps).integer as f32 / 1000.0f32) as f64,
            ),
        );
        frameMsec = 1 as i32
    }
    sv.timeResidual += msec;
    if (*com_dedicated).integer == 0 {
        SV_BotFrame(sv.time + sv.timeResidual);
    }
    // if time is about to hit the 32nd bit, kick all clients
    // and clear sv.time, rather
    // than checking for negative time wraparound everywhere.
    // 2giga-milliseconds = 23 days, so it won't be too often
    if svs.time > 0x70000000 as i32 {
        SV_Shutdown(
            b"Restarting server due to time wrapping\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        Cbuf_AddText(va(
            b"map %s\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            Cvar_VariableString(b"mapname\x00" as *const u8 as *const libc::c_char),
        ));
        return;
    }
    // this can happen considerably earlier when lots of clients play and the map doesn't change
    if svs.nextSnapshotEntities >= 0x7ffffffe as i32 - svs.numSnapshotEntities {
        SV_Shutdown(
            b"Restarting server due to numSnapshotEntities wrapping\x00" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        );
        Cbuf_AddText(va(
            b"map %s\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            Cvar_VariableString(b"mapname\x00" as *const u8 as *const libc::c_char),
        ));
        return;
    }
    if sv.restartTime != 0 && sv.time >= sv.restartTime {
        sv.restartTime = 0 as i32;
        Cbuf_AddText(b"map_restart 0\n\x00" as *const u8 as *const libc::c_char);
        return;
    }
    // update infostrings if anything has been changed
    if cvar_modifiedFlags & 0x4 as i32 != 0 {
        SV_SetConfigstring(0 as i32, Cvar_InfoString(0x4 as i32));
        cvar_modifiedFlags &= !(0x4 as i32)
    }
    if cvar_modifiedFlags & 0x8 as i32 != 0 {
        SV_SetConfigstring(1 as i32, Cvar_InfoString_Big(0x8 as i32));
        cvar_modifiedFlags &= !(0x8 as i32)
    }
    if (*com_speeds).integer != 0 {
        startTime = Sys_Milliseconds()
    } else {
        startTime = 0 as i32
        // quite a compiler warning
    }
    // update ping based on the all received frames
    SV_CalcPings();
    if (*com_dedicated).integer != 0 {
        SV_BotFrame(sv.time);
    }
    // run the game simulation in chunks
    while sv.timeResidual >= frameMsec {
        sv.timeResidual -= frameMsec;
        svs.time += frameMsec;
        sv.time += frameMsec;
        // let everything in the world think and move
        VM_Call(gvm, GAME_RUN_FRAME as i32, sv.time);
    }
    if (*com_speeds).integer != 0 {
        time_game = Sys_Milliseconds() - startTime
    }
    // check timeouts
    SV_CheckTimeouts();
    // send messages back to the clients
    SV_SendClientMessages();
    // send a heartbeat to the master if needed
    SV_MasterHeartbeat(b"DarkPlaces\x00" as *const u8 as *const libc::c_char);
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
#[no_mangle]

pub unsafe extern "C" fn SV_RateMsec(mut client: *mut client_t) -> i32 {
    let mut rate: i32 = 0;
    let mut rateMsec: i32 = 0;
    let mut messageSize: i32 = 0;
    messageSize = (*client).netchan.lastSentSize;
    rate = (*client).rate;
    if (*sv_maxRate).integer != 0 {
        if (*sv_maxRate).integer < 1000 as i32 {
            Cvar_Set(
                b"sv_MaxRate\x00" as *const u8 as *const libc::c_char,
                b"1000\x00" as *const u8 as *const libc::c_char,
            );
        }
        if (*sv_maxRate).integer < rate {
            rate = (*sv_maxRate).integer
        }
    }
    if (*sv_minRate).integer != 0 {
        if (*sv_minRate).integer < 1000 as i32 {
            Cvar_Set(
                b"sv_minRate\x00" as *const u8 as *const libc::c_char,
                b"1000\x00" as *const u8 as *const libc::c_char,
            );
        }
        if (*sv_minRate).integer > rate {
            rate = (*sv_minRate).integer
        }
    }
    if (*client).netchan.remoteAddress.type_0 as u32 == NA_IP6 as i32 as u32 {
        messageSize += 48 as i32
    } else {
        messageSize += 28 as i32
    }
    rateMsec = messageSize * 1000 as i32 / (rate as f32 * (*com_timescale).value) as i32;
    rate = Sys_Milliseconds() - (*client).netchan.lastSentTime;
    if rate > rateMsec {
        return 0 as i32;
    } else {
        return rateMsec - rate;
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
// qcommon.h -- definitions common between client and server, but not game.or ref modules
//Ignore __attribute__ on non-gcc platforms
//#define	PRE_RELEASE_DEMO
//============================================================================
//
// msg.c
//
// if false, do a Com_Error
// set to true if the buffer size failed (with allowoverflow set)
// set to true if the buffer size failed (with allowoverflow set)
// for bitwise reads and writes
// TTimo
// copy a msg_t in case we need to store it as is for a bit
// (as I needed this to keep an msg_t from a static var for later use)
// sets data buffer as MSG_Init does prior to do the copy
//============================================================================
/*
==============================================================

NET

==============================================================
*/
// if this flag is set, always attempt ipv6 connections instead of ipv4 if a v6 address is found.
// disables ipv6 multicast support if set.
// number of old messages that must be kept on client and
// server for delta comrpession and ping estimation
// max number of usercmd_t in a packet
// max string commands buffered for restransmit
// an address lookup failed
// maximum length of an IPv6 address string including trailing '\0'
// Needed for IPv6 link-local addresses
// max length of a message, which may
// be fragmented into multiple packets
// ACK window of 48 download chunks. Cannot set this higher, or clients
// will overflow the reliable commands buffer
// 896 byte block chunks
/*
Netchan handles packet fragmentation and out of order / duplicate suppression
*/
// between last packet and previous
// qport value to write when transmitting
// sequencing variables
// incoming fragment assembly buffer
// outgoing fragment buffer
// we need to space out the sending of large fragmented messages
/*
==============================================================

PROTOCOL

==============================================================
*/
// 1.31 - 67
// maintain a list of compatible protocols for demo playing
// NOTE: that stuff only works with two digits protocols
// override on command line, config files etc.
// broadcast scan this many ports after
// PORT_SERVER so a single machine can
// run multiple servers
// the svc_strings[] array in cl_parse.c should mirror this
//
// server to client
//
// [short] [string] only in gamestate messages
// only in gamestate messages
// [string] to be executed by client game module
// [short] size [size bytes]
// new commands, supported only by ioquake3 protocol but not legacy
// not wrapped in USE_VOIP, so this value is reserved.
//
//
// client to server
//
// [[usercmd_t]
// [[usercmd_t]
// [string] message
// new commands, supported only by ioquake3 protocol but not legacy
// not wrapped in USE_VOIP, so this value is reserved.
//
/*
==============================================================

VIRTUAL MACHINE

==============================================================
*/
// module should be bare: "cgame", not "cgame.dll" or "vm/cgame.qvm"
/*
==============================================================

CMD

Command text buffering and command execution

==============================================================
*/
/*

Any number of commands can be added in a frame, from several different sources.
Most commands come from either keybindings or console line input, but entire text
files can be execed.

*/
// allocates an initial text buffer that will grow as needed
// Adds command text at the end of the buffer, does NOT add a final \n
// this can be used in place of either Cbuf_AddText or Cbuf_InsertText
// Pulls off \n terminated lines of text from the command buffer and sends
// them through Cmd_ExecuteString.  Stops when the buffer is empty.
// Normally called once per frame, but may be explicitly invoked.
// Do not call inside a command function, or current args will be destroyed.
//===========================================================================
/*

Command execution takes a null terminated string, breaks it into tokens,
then searches for a command or variable that matches the first token.

*/
// called by the init functions of other parts of the program to
// register commands and functions to call for them.
// The cmd_name is referenced later, so it should not be in temp memory
// if function is NULL, the command will be forwarded to the server
// as a clc_clientCommand instead of executed locally
// don't allow VMs to remove system commands
// callback with each valid string
// The functions that execute commands get their parameters with these
// functions. Cmd_Argv () will return an empty string, not a NULL
// if arg > argc, so string operations are allways safe.
// Takes a null terminated string.  Does not need to be /n terminated.
// breaks the string up into arg tokens.
// Parses a single line of text into arguments and tries to execute it
// as if it was typed at the console
/*
==============================================================

CVAR

==============================================================
*/
/*

cvar_t variables are used to hold scalar or string variables that can be changed
or displayed at the console or prog code as well as accessed directly
in C code.

The user can access cvars from the console in three ways:
r_draworder			prints the current value
r_draworder 0		sets the current value to 0
set r_draworder 0	as above, but creates the cvar if not present

Cvars are restricted from having the same names as commands to keep this
interface from being ambiguous.

The are also occasionally used to communicated information between different
modules of the program.

*/
// creates the variable if it doesn't exist, or returns the existing one
// if it exists, the value will not be changed, but flags will be ORed in
// that allows variables to be unarchived without needing bitflags
// if value is "", the value will not override a previously set value.
// basically a slightly modified Cvar_Get for the interpreted modules
// updates an interpreted modules' version of a cvar
// will create the variable with no flags if it doesn't exist
// same as Cvar_Set, but allows more control over setting of cvar
// sometimes we set variables from an untrusted source: fail if flags & CVAR_PROTECTED
// don't set the cvar immediately
// expands value to a string and calls Cvar_Set/Cvar_SetSafe
// returns 0 if not defined or non numeric
// returns an empty string if not defined
// returns CVAR_NONEXISTENT if cvar doesn't exist or the flags of that particular CVAR.
// callback with each valid string
// reset all testing vars to a safe value
// called by Cmd_ExecuteString when Cmd_Argv(0) doesn't match a known
// command.  Returns true if the command was a variable reference that
// was handled. (print or change)
// writes lines containing "set variable value" for all variables
// with the archive flag set to true.
// returns an info string containing all the cvars that have the given bit set
// in their flags ( CVAR_USERINFO, CVAR_SERVERINFO, CVAR_SYSTEMINFO, etc )
// whenever a cvar is modifed, its flags will be OR'd into this, so
// a single check can determine if any CVAR_USERINFO, CVAR_SERVERINFO,
// etc, variables have been modified since the last check.  The bit
// can then be cleared to allow another change detection.
/*
==============================================================

FILESYSTEM

No stdio calls should be used by any part of the game, because
we need to deal with all sorts of directory and seperator char
issues.
==============================================================
*/
// referenced flags
// these are in loop specific order so don't change the order
// number of id paks that will never be autodownloaded from baseq3/missionpack
// shutdown and restart the filesystem so changes to fs_gamedir can take effect
// directory should not have either a leading or trailing /
// if extension is "/", only subdirectories will be returned
// the returned files will not include any directories or /
// will properly create any needed paths and deal with seperater character issues
// if uniqueFILE is true, then a new FILE will be fopened even if the file
// is found in an already open pak file.  If uniqueFILE is false, you must call
// FS_FCloseFile instead of fclose, otherwise the pak FILE would be improperly closed
// It is generally safe to always set uniqueFILE to true, because the majority of
// file IO goes through FS_ReadFile, which Does The Right Thing already.
// returns 1 if a file is in the PAK file, otherwise -1
// properly handles partial reads and reads from other dlls
// note: you can't just fclose from another DLL, due to MS libc issues
// returns the length of the file
// a null buffer will just return the file length without loading
// as a quick check for existence. -1 length == not present
// A 0 byte will always be appended at the end, so string ops are safe.
// the buffer should be considered read-only, because it may be cached
// for other uses.
// forces flush on files we're writing to.
// frees the memory returned by FS_ReadFile
// writes a complete file, creating any subdirectories needed
// doesn't work for files that are opened from a pack file
// where are we?
// like fprintf
// opens a file for reading, writing, or appending depending on the value of mode
// seek on a file
// Returns a space separated string containing the checksums of all loaded pk3 files.
// Servers with sv_pure set will get this string and pass it to clients.
// Returns a space separated string containing the checksums of all loaded
// AND referenced pk3 files. Servers with sv_pure set will get this string
// back from clients for pure validation
// clears referenced booleans on loaded pk3s
// If the string is empty, all data sources will be allowed.
// If not empty, only pk3 files that match one of the space
// separated checksums will be checked for files, with the
// sole exception of .cfg files.
/*
==============================================================

Edit fields and command line history/completion

==============================================================
*/
/*
==============================================================

MISC

==============================================================
*/
// centralizing the declarations for cl_cdkey
// https://zerowing.idsoftware.com/bugzilla/show_bug.cgi?id=470
// returned by Sys_GetProcessorFeatures
// centralized and cleaned, that's the max string you can send to a Com_Printf / Com_DPrintf (above gets truncated)
// SE_NONE must be zero
// evTime is still valid
// evValue is a key code, evValue2 is the down flag
// evValue is an ascii char
// evValue and evValue2 are relative signed x / y moves
// evValue is an axis number and evValue2 is the current state (-127 to 127)
// evPtr is a char*
// bytes of data pointed to by evPtr, for journaling
// this must be manually freed if not NULL
// will be journaled properly
// checks for and removes command line "+set var arg" constructs
// if match is NULL, all set commands will be executed, otherwise
// only a set with the exact name.  Only used during startup.
// for building release pak files
// both client and server must agree to pause
// com_speeds times
// renderer backend time
/*

--- low memory ----
server vm
server clipmap
---mark---
renderer initialization (shaders, etc)
UI vm
cgame vm
renderer map
renderer models

---free---

temp file loading
--- high memory ---

*/
// NOT 0 filled memory
// returns 0 filled memory
// NOT 0 filled memory only for small allocations
// commandLine should not include the executable name (argv[0])
/*
==============================================================

CLIENT / SERVER SYSTEMS

==============================================================
*/
//
// client interface
//
// the keyboard binding interface must be setup before execing
// config files, but the rest of client startup will happen later
// char events are for field typing, not game control
// do a screen update before starting to load a map
// when the server is going to load a new map, the entire hunk
// will be cleared, so the client must shutdown cgame, ui, and
// the renderer
// adds the current command line as a clc_clientCommand to the client message.
// things like godmode, noclip, etc, are commands directed to the server,
// so when they are typed in at the console, they will need to be forwarded.
// bring up the "need a cd to play" dialog
// dump all memory on an error
// shutdown client
// initialize renderer interface
// start all the client stuff using the hunk
// Restart sound subsystem
// for keyname autocompletion
// for writing the config files
// call before filesystem access
// FIXME: move logging to common?
// AVI files have the start of pixel lines 4 byte-aligned
//
// server interface
//
/*
====================
SV_SendQueuedPackets

Send download messages and queued packets in the time that we're idle, i.e.
not computing a server frame or sending client snapshots.
Return the time in msec until we expect to be called next
====================
*/
#[no_mangle]

pub unsafe extern "C" fn SV_SendQueuedPackets() -> i32 {
    let mut numBlocks: i32 = 0;
    let mut dlStart: i32 = 0;
    let mut deltaT: i32 = 0;
    let mut delayT: i32 = 0;
    static mut dlNextRound: i32 = 0 as i32;
    let mut timeVal: i32 = 2147483647 as i32;
    // Send out fragmented packets now that we're idle
    delayT = SV_SendQueuedMessages();
    if delayT >= 0 as i32 {
        timeVal = delayT
    }
    if (*sv_dlRate).integer != 0 {
        // Rate limiting. This is very imprecise for high
        // download rates due to millisecond timedelta resolution
        dlStart = Sys_Milliseconds();
        deltaT = dlNextRound - dlStart;
        if deltaT > 0 as i32 {
            if deltaT < timeVal {
                timeVal = deltaT + 1 as i32
            }
        } else {
            numBlocks = SV_SendDownloadMessages();
            if numBlocks != 0 {
                // There are active downloads
                deltaT = Sys_Milliseconds() - dlStart;
                delayT = 1000 as i32 * numBlocks * 1024 as i32;
                delayT /= (*sv_dlRate).integer * 1024 as i32;
                if delayT <= deltaT + 1 as i32 {
                    // Sending the last round of download messages
                    // took too long for given rate, don't wait for
                    // next round, but always enforce a 1ms delay
                    // between DL message rounds so we don't hog
                    // all of the bandwidth. This will result in an
                    // effective maximum rate of 1MB/s per user, but the
                    // low download window size limits this anyways.
                    if timeVal > 2 as i32 {
                        timeVal = 2 as i32
                    }
                    dlNextRound = dlStart + deltaT + 1 as i32
                } else {
                    dlNextRound = dlStart + delayT;
                    delayT -= deltaT;
                    if delayT < timeVal {
                        timeVal = delayT
                    }
                }
            }
        }
    } else if SV_SendDownloadMessages() != 0 {
        timeVal = 0 as i32
    }
    return timeVal;
}

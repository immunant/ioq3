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
pub use crate::src::qcommon::q_shared::Q_strcat;
pub use crate::src::qcommon::q_shared::Q_stricmp;
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
pub use crate::src::game::g_arenas::Svcmd_AbortPodium_f;
pub use crate::src::game::g_bot::Svcmd_AddBot_f;
pub use crate::src::game::g_bot::Svcmd_BotList_f;
pub use crate::src::game::g_cmds::SetTeam;
pub use crate::src::game::g_main::g_banIPs;
pub use crate::src::game::g_main::g_dedicated;
pub use crate::src::game::g_main::g_entities;
pub use crate::src::game::g_main::g_filterBan;
pub use crate::src::game::g_main::level;
pub use crate::src::game::g_main::G_Printf;
pub use crate::src::game::g_mem::Svcmd_GameMem_f;
pub use crate::src::game::g_svcmds::stdlib_h::atoi;
pub use crate::src::game::g_syscalls::trap_Argc;
pub use crate::src::game::g_syscalls::trap_Argv;
pub use crate::src::game::g_syscalls::trap_Cvar_Set;
pub use crate::src::game::g_syscalls::trap_SendConsoleCommand;
pub use crate::src::game::g_syscalls::trap_SendServerCommand;

pub use ::libc::strtol;
extern "C" {
    #[no_mangle]
    pub fn ConcatArgs(start: i32) -> *mut libc::c_char;
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
// this file holds commands that can be executed by the server console, but not remote clients
/*
==============================================================================

PACKET FILTERING


You can add or remove addresses from the filter list with:

addip <ip>
removeip <ip>

The ip address is specified in dot format, and you can use '*' to match any value
so you can specify an entire class C network with "addip 192.246.40.*"

Removeip will only remove an address specified exactly the same way.  You cannot addip a subnet, then removeip a single host.

listip
Prints the current list of filters.

g_filterban <0 or 1>

If 1 (the default), then ip addresses matching the current list will be prohibited from entering the game.  This is the default setting.

If 0, then only addresses matching the list will be allowed.  This lets you easily set up a private game, or a game that only allows players from your local network.

TTimo NOTE: for persistence, bans are stored in g_banIPs cvar MAX_CVAR_VALUE_STRING
The size of the cvar string buffer is limiting the banning to around 20 masks
this could be improved by putting some g_banIPs2 g_banIps3 etc. maybe
still, you should rely on PB for banning instead

==============================================================================
*/

pub type ipFilter_t = ipFilter_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipFilter_s {
    pub mask: u32,
    pub compare: u32,
}

static mut ipFilters: [ipFilter_t; 1024] = [ipFilter_t {
    mask: 0,
    compare: 0,
}; 1024];

static mut numIPFilters: i32 = 0;
/*
=================
StringToFilter
=================
*/

unsafe extern "C" fn StringToFilter(mut s: *mut libc::c_char, mut f: *mut ipFilter_t) -> qboolean {
    let mut num: [libc::c_char; 128] = [0; 128];
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut b: [byte; 4] = [0; 4];
    let mut m: [byte; 4] = [0; 4];
    i = 0 as i32;
    while i < 4 as i32 {
        b[i as usize] = 0 as i32 as byte;
        m[i as usize] = 0 as i32 as byte;
        i += 1
    }
    i = 0 as i32;
    while i < 4 as i32 {
        if (*s as i32) < '0' as i32 || *s as i32 > '9' as i32 {
            if *s as i32 == '*' as i32 {
                // 'match any'
                // b[i] and m[i] to 0
                s = s.offset(1);
                if *s == 0 {
                    break;
                }
                s = s.offset(1)
            } else {
                G_Printf(
                    b"Bad filter address: %s\n\x00" as *const u8 as *const libc::c_char,
                    s,
                );
                return qfalse;
            }
        } else {
            j = 0 as i32;
            while *s as i32 >= '0' as i32 && *s as i32 <= '9' as i32 {
                let fresh0 = s;
                s = s.offset(1);
                let fresh1 = j;
                j = j + 1;
                num[fresh1 as usize] = *fresh0
            }
            num[j as usize] = 0 as i32 as libc::c_char;
            b[i as usize] = atoi(num.as_mut_ptr()) as byte;
            m[i as usize] = 255 as i32 as byte;
            if *s == 0 {
                break;
            }
            s = s.offset(1)
        }
        i += 1
    }
    (*f).mask = *(m.as_mut_ptr() as *mut u32);
    (*f).compare = *(b.as_mut_ptr() as *mut u32);
    return qtrue;
}
/*
=================
UpdateIPBans
=================
*/

unsafe extern "C" fn UpdateIPBans() {
    let mut b: [byte; 4] = [0 as i32 as byte, 0, 0, 0];
    let mut m: [byte; 4] = [0 as i32 as byte, 0, 0, 0];
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut iplist_final: [libc::c_char; 256] = [
        0 as i32 as libc::c_char,
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
        0,
        0,
        0,
    ];
    let mut ip: [libc::c_char; 64] = [
        0 as i32 as libc::c_char,
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
    *iplist_final.as_mut_ptr() = 0 as i32 as libc::c_char;
    i = 0 as i32;
    while i < numIPFilters {
        if !(ipFilters[i as usize].compare == 0xffffffff as u32) {
            *(b.as_mut_ptr() as *mut u32) = ipFilters[i as usize].compare;
            *(m.as_mut_ptr() as *mut u32) = ipFilters[i as usize].mask;
            *ip.as_mut_ptr() = 0 as i32 as libc::c_char;
            j = 0 as i32;
            while j < 4 as i32 {
                if m[j as usize] as i32 != 255 as i32 {
                    Q_strcat(
                        ip.as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as i32,
                        b"*\x00" as *const u8 as *const libc::c_char,
                    );
                } else {
                    Q_strcat(
                        ip.as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as i32,
                        va(
                            b"%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                            b[j as usize] as i32,
                        ),
                    );
                }
                Q_strcat(
                    ip.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as i32,
                    if j < 3 as i32 {
                        b".\x00" as *const u8 as *const libc::c_char
                    } else {
                        b" \x00" as *const u8 as *const libc::c_char
                    },
                );
                j += 1
            }
            if crate::stdlib::strlen(iplist_final.as_mut_ptr())
                .wrapping_add(crate::stdlib::strlen(ip.as_mut_ptr()))
                < 256 as i32 as libc::c_ulong
            {
                Q_strcat(
                    iplist_final.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as i32,
                    ip.as_mut_ptr(),
                );
            } else {
                Com_Printf(
                    b"g_banIPs overflowed at MAX_CVAR_VALUE_STRING\n\x00" as *const u8
                        as *const libc::c_char,
                );
                break;
            }
        }
        i += 1
    }
    trap_Cvar_Set(
        b"g_banIPs\x00" as *const u8 as *const libc::c_char,
        iplist_final.as_mut_ptr(),
    );
}
/*
=================
G_FilterPacket
=================
*/
#[no_mangle]

pub unsafe extern "C" fn G_FilterPacket(mut from: *mut libc::c_char) -> qboolean {
    let mut i: i32 = 0;
    let mut in_0: u32 = 0;
    let mut m: [byte; 4] = [0 as i32 as byte, 0, 0, 0];
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    i = 0 as i32;
    p = from;
    while *p as i32 != 0 && i < 4 as i32 {
        m[i as usize] = 0 as i32 as byte;
        while *p as i32 >= '0' as i32 && *p as i32 <= '9' as i32 {
            m[i as usize] = (m[i as usize] as i32 * 10 as i32 + (*p as i32 - '0' as i32)) as byte;
            p = p.offset(1)
        }
        if *p == 0 || *p as i32 == ':' as i32 {
            break;
        }
        i += 1;
        p = p.offset(1)
    }
    in_0 = *(m.as_mut_ptr() as *mut u32);
    i = 0 as i32;
    while i < numIPFilters {
        if in_0 & ipFilters[i as usize].mask == ipFilters[i as usize].compare {
            return (g_filterBan.integer != 0 as i32) as i32 as qboolean;
        }
        i += 1
    }
    return (g_filterBan.integer == 0 as i32) as i32 as qboolean;
}
/*
=================
AddIP
=================
*/

unsafe extern "C" fn AddIP(mut str: *mut libc::c_char) {
    let mut i: i32 = 0; // free spot
    i = 0 as i32;
    while i < numIPFilters {
        if ipFilters[i as usize].compare == 0xffffffff as u32 {
            break;
        }
        i += 1
    }
    if i == numIPFilters {
        if numIPFilters == 1024 as i32 {
            G_Printf(b"IP filter list is full\n\x00" as *const u8 as *const libc::c_char);
            return;
        }
        numIPFilters += 1
    }
    if StringToFilter(str, &mut *ipFilters.as_mut_ptr().offset(i as isize)) as u64 == 0 {
        ipFilters[i as usize].compare = 0xffffffff as u32
    }
    UpdateIPBans();
}
/*
=================
G_ProcessIPBans
=================
*/
#[no_mangle]

pub unsafe extern "C" fn G_ProcessIPBans() {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut str: [libc::c_char; 256] = [0; 256];
    Q_strncpyz(
        str.as_mut_ptr(),
        g_banIPs.string.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as i32,
    );
    s = g_banIPs.string.as_mut_ptr();
    t = s;
    while *t != 0 {
        /* */
        s = libc::strchr(s, ' ' as i32);
        if s.is_null() {
            break;
        }
        while *s as i32 == ' ' as i32 {
            let fresh2 = s;
            s = s.offset(1);
            *fresh2 = 0 as i32 as libc::c_char
        }
        if *t != 0 {
            AddIP(t);
        }
        t = s
    }
}
/*
=================
Svcmd_AddIP_f
=================
*/
#[no_mangle]

pub unsafe extern "C" fn Svcmd_AddIP_f() {
    let mut str: [libc::c_char; 1024] = [0; 1024];
    if trap_Argc() < 2 as i32 {
        G_Printf(b"Usage: addip <ip-mask>\n\x00" as *const u8 as *const libc::c_char);
        return;
    }
    trap_Argv(
        1 as i32,
        str.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as i32,
    );
    AddIP(str.as_mut_ptr());
}
/*
=================
Svcmd_RemoveIP_f
=================
*/
#[no_mangle]

pub unsafe extern "C" fn Svcmd_RemoveIP_f() {
    let mut f: ipFilter_t = ipFilter_t {
        mask: 0,
        compare: 0,
    };
    let mut i: i32 = 0;
    let mut str: [libc::c_char; 1024] = [0; 1024];
    if trap_Argc() < 2 as i32 {
        G_Printf(b"Usage: removeip <ip-mask>\n\x00" as *const u8 as *const libc::c_char);
        return;
    }
    trap_Argv(
        1 as i32,
        str.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as i32,
    );
    if StringToFilter(str.as_mut_ptr(), &mut f) as u64 == 0 {
        return;
    }
    i = 0 as i32;
    while i < numIPFilters {
        if ipFilters[i as usize].mask == f.mask && ipFilters[i as usize].compare == f.compare {
            ipFilters[i as usize].compare = 0xffffffff as u32;
            G_Printf(b"Removed.\n\x00" as *const u8 as *const libc::c_char);
            UpdateIPBans();
            return;
        }
        i += 1
    }
    G_Printf(
        b"Didn\'t find %s.\n\x00" as *const u8 as *const libc::c_char,
        str.as_mut_ptr(),
    );
}
/*
===================
Svcmd_EntityList_f
===================
*/
#[no_mangle]

pub unsafe extern "C" fn Svcmd_EntityList_f() {
    let mut e: i32 = 0;
    let mut check: *mut gentity_t = 0 as *mut gentity_t;
    check = g_entities.as_mut_ptr();
    e = 0 as i32;
    while e < level.num_entities {
        if !((*check).inuse as u64 == 0) {
            G_Printf(b"%3i:\x00" as *const u8 as *const libc::c_char, e);
            match (*check).s.eType {
                0 => {
                    G_Printf(b"ET_GENERAL          \x00" as *const u8 as *const libc::c_char);
                }
                1 => {
                    G_Printf(b"ET_PLAYER           \x00" as *const u8 as *const libc::c_char);
                }
                2 => {
                    G_Printf(b"ET_ITEM             \x00" as *const u8 as *const libc::c_char);
                }
                3 => {
                    G_Printf(b"ET_MISSILE          \x00" as *const u8 as *const libc::c_char);
                }
                4 => {
                    G_Printf(b"ET_MOVER            \x00" as *const u8 as *const libc::c_char);
                }
                5 => {
                    G_Printf(b"ET_BEAM             \x00" as *const u8 as *const libc::c_char);
                }
                6 => {
                    G_Printf(b"ET_PORTAL           \x00" as *const u8 as *const libc::c_char);
                }
                7 => {
                    G_Printf(b"ET_SPEAKER          \x00" as *const u8 as *const libc::c_char);
                }
                8 => {
                    G_Printf(b"ET_PUSH_TRIGGER     \x00" as *const u8 as *const libc::c_char);
                }
                9 => {
                    G_Printf(b"ET_TELEPORT_TRIGGER \x00" as *const u8 as *const libc::c_char);
                }
                10 => {
                    G_Printf(b"ET_INVISIBLE        \x00" as *const u8 as *const libc::c_char);
                }
                11 => {
                    G_Printf(b"ET_GRAPPLE          \x00" as *const u8 as *const libc::c_char);
                }
                _ => {
                    G_Printf(
                        b"%3i                 \x00" as *const u8 as *const libc::c_char,
                        (*check).s.eType,
                    );
                }
            }
            if !(*check).classname.is_null() {
                G_Printf(
                    b"%s\x00" as *const u8 as *const libc::c_char,
                    (*check).classname,
                );
            }
            G_Printf(b"\n\x00" as *const u8 as *const libc::c_char);
        }
        e += 1;
        check = check.offset(1)
    }
}
#[no_mangle]

pub unsafe extern "C" fn ClientForString(mut s: *const libc::c_char) -> *mut gclient_t {
    let mut cl: *mut gclient_t = 0 as *mut gclient_t;
    let mut i: i32 = 0;
    let mut idnum: i32 = 0;
    // numeric values are just slot numbers
    if *s.offset(0 as i32 as isize) as i32 >= '0' as i32
        && *s.offset(0 as i32 as isize) as i32 <= '9' as i32
    {
        idnum = atoi(s);
        if idnum < 0 as i32 || idnum >= level.maxclients {
            Com_Printf(
                b"Bad client slot: %i\n\x00" as *const u8 as *const libc::c_char,
                idnum,
            );
            return 0 as *mut gclient_t;
        }
        cl = &mut *level.clients.offset(idnum as isize) as *mut gclient_s;
        if (*cl).pers.connected as u32 == CON_DISCONNECTED as i32 as u32 {
            G_Printf(
                b"Client %i is not connected\n\x00" as *const u8 as *const libc::c_char,
                idnum,
            );
            return 0 as *mut gclient_t;
        }
        return cl;
    }
    // check for a name match
    i = 0 as i32;
    while i < level.maxclients {
        cl = &mut *level.clients.offset(i as isize) as *mut gclient_s;
        if !((*cl).pers.connected as u32 == CON_DISCONNECTED as i32 as u32) {
            if Q_stricmp((*cl).pers.netname.as_mut_ptr(), s) == 0 {
                return cl;
            }
        }
        i += 1
    }
    G_Printf(
        b"User %s is not on the server\n\x00" as *const u8 as *const libc::c_char,
        s,
    );
    return 0 as *mut gclient_t;
}
/*
===================
Svcmd_ForceTeam_f

forceteam <player> <team>
===================
*/
#[no_mangle]

pub unsafe extern "C" fn Svcmd_ForceTeam_f() {
    let mut cl: *mut gclient_t = 0 as *mut gclient_t;
    let mut str: [libc::c_char; 1024] = [0; 1024];
    if trap_Argc() < 3 as i32 {
        G_Printf(b"Usage: forceteam <player> <team>\n\x00" as *const u8 as *const libc::c_char);
        return;
    }
    // find the player
    trap_Argv(
        1 as i32,
        str.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as i32,
    );
    cl = ClientForString(str.as_mut_ptr());
    if cl.is_null() {
        return;
    }
    // set the team
    trap_Argv(
        2 as i32,
        str.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as i32,
    );
    SetTeam(
        &mut *g_entities
            .as_mut_ptr()
            .offset(cl.offset_from(level.clients) as isize) as *mut _ as *mut gentity_s,
        str.as_mut_ptr(),
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
/*
=================
ConsoleCommand

=================
*/
#[no_mangle]

pub unsafe extern "C" fn ConsoleCommand() -> qboolean {
    let mut cmd: [libc::c_char; 1024] = [0; 1024];
    trap_Argv(
        0 as i32,
        cmd.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as i32,
    );
    if Q_stricmp(
        cmd.as_mut_ptr(),
        b"entitylist\x00" as *const u8 as *const libc::c_char,
    ) == 0 as i32
    {
        Svcmd_EntityList_f();
        return qtrue;
    }
    if Q_stricmp(
        cmd.as_mut_ptr(),
        b"forceteam\x00" as *const u8 as *const libc::c_char,
    ) == 0 as i32
    {
        Svcmd_ForceTeam_f();
        return qtrue;
    }
    if Q_stricmp(
        cmd.as_mut_ptr(),
        b"game_memory\x00" as *const u8 as *const libc::c_char,
    ) == 0 as i32
    {
        Svcmd_GameMem_f();
        return qtrue;
    }
    if Q_stricmp(
        cmd.as_mut_ptr(),
        b"addbot\x00" as *const u8 as *const libc::c_char,
    ) == 0 as i32
    {
        Svcmd_AddBot_f();
        return qtrue;
    }
    if Q_stricmp(
        cmd.as_mut_ptr(),
        b"botlist\x00" as *const u8 as *const libc::c_char,
    ) == 0 as i32
    {
        Svcmd_BotList_f();
        return qtrue;
    }
    if Q_stricmp(
        cmd.as_mut_ptr(),
        b"abort_podium\x00" as *const u8 as *const libc::c_char,
    ) == 0 as i32
    {
        Svcmd_AbortPodium_f();
        return qtrue;
    }
    if Q_stricmp(
        cmd.as_mut_ptr(),
        b"addip\x00" as *const u8 as *const libc::c_char,
    ) == 0 as i32
    {
        Svcmd_AddIP_f();
        return qtrue;
    }
    if Q_stricmp(
        cmd.as_mut_ptr(),
        b"removeip\x00" as *const u8 as *const libc::c_char,
    ) == 0 as i32
    {
        Svcmd_RemoveIP_f();
        return qtrue;
    }
    if Q_stricmp(
        cmd.as_mut_ptr(),
        b"listip\x00" as *const u8 as *const libc::c_char,
    ) == 0 as i32
    {
        trap_SendConsoleCommand(
            EXEC_NOW as i32,
            b"g_banIPs\n\x00" as *const u8 as *const libc::c_char,
        );
        return qtrue;
    }
    if g_dedicated.integer != 0 {
        if Q_stricmp(
            cmd.as_mut_ptr(),
            b"say\x00" as *const u8 as *const libc::c_char,
        ) == 0 as i32
        {
            trap_SendServerCommand(
                -(1 as i32),
                va(
                    b"print \"server: %s\n\"\x00" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    ConcatArgs(1 as i32),
                ),
            );
            return qtrue;
        }
        // everything else will also be printed as a say command
        trap_SendServerCommand(
            -(1 as i32),
            va(
                b"print \"server: %s\n\"\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ConcatArgs(0 as i32),
            ),
        );
        return qtrue;
    }
    return qfalse;
}

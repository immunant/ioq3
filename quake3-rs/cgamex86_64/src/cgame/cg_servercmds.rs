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

pub use crate::bg_public_h::animation_s;
pub use crate::bg_public_h::animation_t;
pub use crate::bg_public_h::gametype_t;
pub use crate::bg_public_h::gender_t;
pub use crate::bg_public_h::team_t;
pub use crate::bg_public_h::GENDER_FEMALE;
pub use crate::bg_public_h::GENDER_MALE;
pub use crate::bg_public_h::GENDER_NEUTER;
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
pub use crate::cg_public_h::snapshot_t;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::cvarHandle_t;
pub use crate::src::qcommon::q_shared::entityState_s;
pub use crate::src::qcommon::q_shared::entityState_t;
pub use crate::src::qcommon::q_shared::gameState_t;
pub use crate::src::qcommon::q_shared::playerState_s;
pub use crate::src::qcommon::q_shared::playerState_t;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qhandle_t;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::sfxHandle_t;
pub use crate::src::qcommon::q_shared::trType_t;
pub use crate::src::qcommon::q_shared::trajectory_t;
pub use crate::src::qcommon::q_shared::va;
pub use crate::src::qcommon::q_shared::vec3_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::qcommon::q_shared::vmCvar_t;
pub use crate::src::qcommon::q_shared::C2RustUnnamed_0;
pub use crate::src::qcommon::q_shared::Com_sprintf;
pub use crate::src::qcommon::q_shared::Info_ValueForKey;
pub use crate::src::qcommon::q_shared::Q_IsColorString;
pub use crate::src::qcommon::q_shared::Q_stricmp;
pub use crate::src::qcommon::q_shared::Q_strncpyz;
pub use crate::src::qcommon::q_shared::CHAN_ANNOUNCER;
pub use crate::src::qcommon::q_shared::CHAN_AUTO;
pub use crate::src::qcommon::q_shared::CHAN_BODY;
pub use crate::src::qcommon::q_shared::CHAN_ITEM;
pub use crate::src::qcommon::q_shared::CHAN_LOCAL;
pub use crate::src::qcommon::q_shared::CHAN_LOCAL_SOUND;
pub use crate::src::qcommon::q_shared::CHAN_VOICE;
pub use crate::src::qcommon::q_shared::CHAN_WEAPON;
pub use crate::src::qcommon::q_shared::TR_GRAVITY;
pub use crate::src::qcommon::q_shared::TR_INTERPOLATE;
pub use crate::src::qcommon::q_shared::TR_LINEAR;
pub use crate::src::qcommon::q_shared::TR_LINEAR_STOP;
pub use crate::src::qcommon::q_shared::TR_SINE;
pub use crate::src::qcommon::q_shared::TR_STATIONARY;
pub use crate::tr_types_h::glDriverType_t;
pub use crate::tr_types_h::glHardwareType_t;
pub use crate::tr_types_h::glconfig_t;
pub use crate::tr_types_h::refEntityType_t;
pub use crate::tr_types_h::refEntity_t;
pub use crate::tr_types_h::refdef_t;
pub use crate::tr_types_h::textureCompression_t;
pub use crate::tr_types_h::GLDRV_ICD;
pub use crate::tr_types_h::GLDRV_STANDALONE;
pub use crate::tr_types_h::GLDRV_VOODOO;
pub use crate::tr_types_h::GLHW_3DFX_2D3D;
pub use crate::tr_types_h::GLHW_GENERIC;
pub use crate::tr_types_h::GLHW_PERMEDIA2;
pub use crate::tr_types_h::GLHW_RAGEPRO;
pub use crate::tr_types_h::GLHW_RIVA128;
pub use crate::tr_types_h::RT_BEAM;
pub use crate::tr_types_h::RT_LIGHTNING;
pub use crate::tr_types_h::RT_MAX_REF_ENTITY_TYPE;
pub use crate::tr_types_h::RT_MODEL;
pub use crate::tr_types_h::RT_POLY;
pub use crate::tr_types_h::RT_PORTALSURFACE;
pub use crate::tr_types_h::RT_RAIL_CORE;
pub use crate::tr_types_h::RT_RAIL_RINGS;
pub use crate::tr_types_h::RT_SPRITE;
pub use crate::tr_types_h::TC_NONE;
pub use crate::tr_types_h::TC_S3TC;
pub use crate::tr_types_h::TC_S3TC_ARB;

pub use crate::cg_local_h::centity_s;
pub use crate::cg_local_h::centity_t;
pub use crate::cg_local_h::cgMedia_t;
pub use crate::cg_local_h::cg_t;
pub use crate::cg_local_h::cgs_t;
pub use crate::cg_local_h::clientInfo_t;
pub use crate::cg_local_h::footstep_t;
pub use crate::cg_local_h::lerpFrame_t;
pub use crate::cg_local_h::playerEntity_t;
pub use crate::cg_local_h::score_t;
pub use crate::cg_local_h::FOOTSTEP_BOOT;
pub use crate::cg_local_h::FOOTSTEP_ENERGY;
pub use crate::cg_local_h::FOOTSTEP_FLESH;
pub use crate::cg_local_h::FOOTSTEP_MECH;
pub use crate::cg_local_h::FOOTSTEP_METAL;
pub use crate::cg_local_h::FOOTSTEP_NORMAL;
pub use crate::cg_local_h::FOOTSTEP_SPLASH;
pub use crate::cg_local_h::FOOTSTEP_TOTAL;
pub use crate::src::cgame::cg_draw::numSortedTeamPlayers;
pub use crate::src::cgame::cg_draw::sortedTeamPlayers;
pub use crate::src::cgame::cg_draw::CG_CenterPrint;
pub use crate::src::cgame::cg_localents::CG_InitLocalEntities;
pub use crate::src::cgame::cg_main::cg;
pub use crate::src::cgame::cg_main::cg_showmiss;
pub use crate::src::cgame::cg_main::cg_teamChatHeight;
pub use crate::src::cgame::cg_main::cg_teamChatTime;
pub use crate::src::cgame::cg_main::cg_teamChatsOnly;
pub use crate::src::cgame::cg_main::cgs;
pub use crate::src::cgame::cg_main::CG_Argv;
pub use crate::src::cgame::cg_main::CG_BuildSpectatorString;
pub use crate::src::cgame::cg_main::CG_ConfigString;
pub use crate::src::cgame::cg_main::CG_Error;
pub use crate::src::cgame::cg_main::CG_Printf;
pub use crate::src::cgame::cg_main::CG_StartMusic;
pub use crate::src::cgame::cg_marks::CG_InitMarkPolys;
pub use crate::src::cgame::cg_particles::CG_ClearParticles;
pub use crate::src::cgame::cg_players::CG_LoadDeferredPlayers;
pub use crate::src::cgame::cg_players::CG_NewClientInfo;
pub use crate::src::cgame::cg_servercmds::stdlib_h::atoi;
pub use crate::src::cgame::cg_syscalls::trap_Argc;
pub use crate::src::cgame::cg_syscalls::trap_Cvar_Set;
pub use crate::src::cgame::cg_syscalls::trap_GetGameState;
pub use crate::src::cgame::cg_syscalls::trap_GetServerCommand;
pub use crate::src::cgame::cg_syscalls::trap_R_RegisterModel;
pub use crate::src::cgame::cg_syscalls::trap_R_RemapShader;
pub use crate::src::cgame::cg_syscalls::trap_S_ClearLoopingSounds;
pub use crate::src::cgame::cg_syscalls::trap_S_RegisterSound;
pub use crate::src::cgame::cg_syscalls::trap_S_StartLocalSound;

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
// cg_servercmds.c -- reliably sequenced text commands sent by the server
// these are processed at snapshot transition time, so there will definitely
// be a valid snapshot this frame
/*
=================
CG_ParseScores

=================
*/

unsafe extern "C" fn CG_ParseScores() {
    let mut i: i32 = 0;
    let mut powerups: i32 = 0;
    crate::src::cgame::cg_main::cg.numScores = atoi(crate::src::cgame::cg_main::CG_Argv(1 as i32));
    if crate::src::cgame::cg_main::cg.numScores > 64 as i32 {
        crate::src::cgame::cg_main::cg.numScores = 64 as i32
    }
    crate::src::cgame::cg_main::cg.teamScores[0 as i32 as usize] =
        atoi(crate::src::cgame::cg_main::CG_Argv(2 as i32));
    crate::src::cgame::cg_main::cg.teamScores[1 as i32 as usize] =
        atoi(crate::src::cgame::cg_main::CG_Argv(3 as i32));
    crate::stdlib::memset(
        crate::src::cgame::cg_main::cg.scores.as_mut_ptr() as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<[crate::cg_local_h::score_t; 64]>() as libc::c_ulong,
    );
    i = 0 as i32;
    while i < crate::src::cgame::cg_main::cg.numScores {
        //
        crate::src::cgame::cg_main::cg.scores[i as usize].client = atoi(
            crate::src::cgame::cg_main::CG_Argv(i * 14 as i32 + 4 as i32),
        );
        crate::src::cgame::cg_main::cg.scores[i as usize].score = atoi(
            crate::src::cgame::cg_main::CG_Argv(i * 14 as i32 + 5 as i32),
        );
        crate::src::cgame::cg_main::cg.scores[i as usize].ping = atoi(
            crate::src::cgame::cg_main::CG_Argv(i * 14 as i32 + 6 as i32),
        );
        crate::src::cgame::cg_main::cg.scores[i as usize].time = atoi(
            crate::src::cgame::cg_main::CG_Argv(i * 14 as i32 + 7 as i32),
        );
        crate::src::cgame::cg_main::cg.scores[i as usize].scoreFlags = atoi(
            crate::src::cgame::cg_main::CG_Argv(i * 14 as i32 + 8 as i32),
        );
        powerups = atoi(crate::src::cgame::cg_main::CG_Argv(
            i * 14 as i32 + 9 as i32,
        ));
        crate::src::cgame::cg_main::cg.scores[i as usize].accuracy = atoi(
            crate::src::cgame::cg_main::CG_Argv(i * 14 as i32 + 10 as i32),
        );
        crate::src::cgame::cg_main::cg.scores[i as usize].impressiveCount = atoi(
            crate::src::cgame::cg_main::CG_Argv(i * 14 as i32 + 11 as i32),
        );
        crate::src::cgame::cg_main::cg.scores[i as usize].excellentCount = atoi(
            crate::src::cgame::cg_main::CG_Argv(i * 14 as i32 + 12 as i32),
        );
        crate::src::cgame::cg_main::cg.scores[i as usize].guantletCount = atoi(
            crate::src::cgame::cg_main::CG_Argv(i * 14 as i32 + 13 as i32),
        );
        crate::src::cgame::cg_main::cg.scores[i as usize].defendCount = atoi(
            crate::src::cgame::cg_main::CG_Argv(i * 14 as i32 + 14 as i32),
        );
        crate::src::cgame::cg_main::cg.scores[i as usize].assistCount = atoi(
            crate::src::cgame::cg_main::CG_Argv(i * 14 as i32 + 15 as i32),
        );
        crate::src::cgame::cg_main::cg.scores[i as usize].perfect = atoi(
            crate::src::cgame::cg_main::CG_Argv(i * 14 as i32 + 16 as i32),
        )
            as crate::src::qcommon::q_shared::qboolean;
        crate::src::cgame::cg_main::cg.scores[i as usize].captures = atoi(
            crate::src::cgame::cg_main::CG_Argv(i * 14 as i32 + 17 as i32),
        );
        if crate::src::cgame::cg_main::cg.scores[i as usize].client < 0 as i32
            || crate::src::cgame::cg_main::cg.scores[i as usize].client >= 64 as i32
        {
            crate::src::cgame::cg_main::cg.scores[i as usize].client = 0 as i32
        }
        crate::src::cgame::cg_main::cgs.clientinfo
            [crate::src::cgame::cg_main::cg.scores[i as usize].client as usize]
            .score = crate::src::cgame::cg_main::cg.scores[i as usize].score;
        crate::src::cgame::cg_main::cgs.clientinfo
            [crate::src::cgame::cg_main::cg.scores[i as usize].client as usize]
            .powerups = powerups;
        crate::src::cgame::cg_main::cg.scores[i as usize].team = crate::src::cgame::cg_main::cgs
            .clientinfo[crate::src::cgame::cg_main::cg.scores[i as usize].client as usize]
            .team as i32;
        i += 1
    }
}
/*
=================
CG_ParseTeamInfo

=================
*/

unsafe extern "C" fn CG_ParseTeamInfo() {
    let mut i: i32 = 0;
    let mut client: i32 = 0;
    crate::src::cgame::cg_draw::numSortedTeamPlayers =
        atoi(crate::src::cgame::cg_main::CG_Argv(1 as i32));
    if crate::src::cgame::cg_draw::numSortedTeamPlayers < 0 as i32
        || crate::src::cgame::cg_draw::numSortedTeamPlayers > 32 as i32
    {
        crate::src::cgame::cg_main::CG_Error(
            b"CG_ParseTeamInfo: numSortedTeamPlayers out of range (%d)\x00" as *const u8
                as *const libc::c_char,
            crate::src::cgame::cg_draw::numSortedTeamPlayers,
        );
    }
    i = 0 as i32;
    while i < crate::src::cgame::cg_draw::numSortedTeamPlayers {
        client = atoi(crate::src::cgame::cg_main::CG_Argv(i * 6 as i32 + 2 as i32));
        if client < 0 as i32 || client >= 64 as i32 {
            crate::src::cgame::cg_main::CG_Error(
                b"CG_ParseTeamInfo: bad client number: %d\x00" as *const u8 as *const libc::c_char,
                client,
            );
        }
        crate::src::cgame::cg_draw::sortedTeamPlayers[i as usize] = client;
        crate::src::cgame::cg_main::cgs.clientinfo[client as usize].location =
            atoi(crate::src::cgame::cg_main::CG_Argv(i * 6 as i32 + 3 as i32));
        crate::src::cgame::cg_main::cgs.clientinfo[client as usize].health =
            atoi(crate::src::cgame::cg_main::CG_Argv(i * 6 as i32 + 4 as i32));
        crate::src::cgame::cg_main::cgs.clientinfo[client as usize].armor =
            atoi(crate::src::cgame::cg_main::CG_Argv(i * 6 as i32 + 5 as i32));
        crate::src::cgame::cg_main::cgs.clientinfo[client as usize].curWeapon =
            atoi(crate::src::cgame::cg_main::CG_Argv(i * 6 as i32 + 6 as i32));
        crate::src::cgame::cg_main::cgs.clientinfo[client as usize].powerups =
            atoi(crate::src::cgame::cg_main::CG_Argv(i * 6 as i32 + 7 as i32));
        i += 1
    }
}
/*
================
CG_ParseServerinfo

This is called explicitly when the gamestate is first received,
and whenever the server updates any serverinfo flagged cvars
================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_ParseServerinfo() {
    let mut info: *const libc::c_char = 0 as *const libc::c_char;
    let mut mapname: *mut libc::c_char = 0 as *mut libc::c_char;
    info = crate::src::cgame::cg_main::CG_ConfigString(0 as i32);
    crate::src::cgame::cg_main::cgs.gametype =
        atoi(crate::src::qcommon::q_shared::Info_ValueForKey(
            info,
            b"g_gametype\x00" as *const u8 as *const libc::c_char,
        )) as crate::bg_public_h::gametype_t;
    crate::src::cgame::cg_syscalls::trap_Cvar_Set(
        b"g_gametype\x00" as *const u8 as *const libc::c_char,
        crate::src::qcommon::q_shared::va(
            b"%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            crate::src::cgame::cg_main::cgs.gametype as u32,
        ),
    );
    crate::src::cgame::cg_main::cgs.dmflags =
        atoi(crate::src::qcommon::q_shared::Info_ValueForKey(
            info,
            b"dmflags\x00" as *const u8 as *const libc::c_char,
        ));
    crate::src::cgame::cg_main::cgs.teamflags =
        atoi(crate::src::qcommon::q_shared::Info_ValueForKey(
            info,
            b"teamflags\x00" as *const u8 as *const libc::c_char,
        ));
    crate::src::cgame::cg_main::cgs.fraglimit =
        atoi(crate::src::qcommon::q_shared::Info_ValueForKey(
            info,
            b"fraglimit\x00" as *const u8 as *const libc::c_char,
        ));
    crate::src::cgame::cg_main::cgs.capturelimit =
        atoi(crate::src::qcommon::q_shared::Info_ValueForKey(
            info,
            b"capturelimit\x00" as *const u8 as *const libc::c_char,
        ));
    crate::src::cgame::cg_main::cgs.timelimit =
        atoi(crate::src::qcommon::q_shared::Info_ValueForKey(
            info,
            b"timelimit\x00" as *const u8 as *const libc::c_char,
        ));
    crate::src::cgame::cg_main::cgs.maxclients =
        atoi(crate::src::qcommon::q_shared::Info_ValueForKey(
            info,
            b"sv_maxclients\x00" as *const u8 as *const libc::c_char,
        ));
    mapname = crate::src::qcommon::q_shared::Info_ValueForKey(
        info,
        b"mapname\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::qcommon::q_shared::Com_sprintf(
        crate::src::cgame::cg_main::cgs.mapname.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as i32,
        b"maps/%s.bsp\x00" as *const u8 as *const libc::c_char,
        mapname,
    );
    crate::src::qcommon::q_shared::Q_strncpyz(
        crate::src::cgame::cg_main::cgs.redTeam.as_mut_ptr(),
        crate::src::qcommon::q_shared::Info_ValueForKey(
            info,
            b"g_redTeam\x00" as *const u8 as *const libc::c_char,
        ),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as i32,
    );
    crate::src::cgame::cg_syscalls::trap_Cvar_Set(
        b"g_redTeam\x00" as *const u8 as *const libc::c_char,
        crate::src::cgame::cg_main::cgs.redTeam.as_mut_ptr(),
    );
    crate::src::qcommon::q_shared::Q_strncpyz(
        crate::src::cgame::cg_main::cgs.blueTeam.as_mut_ptr(),
        crate::src::qcommon::q_shared::Info_ValueForKey(
            info,
            b"g_blueTeam\x00" as *const u8 as *const libc::c_char,
        ),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as i32,
    );
    crate::src::cgame::cg_syscalls::trap_Cvar_Set(
        b"g_blueTeam\x00" as *const u8 as *const libc::c_char,
        crate::src::cgame::cg_main::cgs.blueTeam.as_mut_ptr(),
    );
}
/*
==================
CG_ParseWarmup
==================
*/

unsafe extern "C" fn CG_ParseWarmup() {
    let mut info: *const libc::c_char = 0 as *const libc::c_char;
    let mut warmup: i32 = 0;
    info = crate::src::cgame::cg_main::CG_ConfigString(5 as i32);
    warmup = atoi(info);
    crate::src::cgame::cg_main::cg.warmupCount = -(1 as i32);
    if !(warmup == 0 as i32 && crate::src::cgame::cg_main::cg.warmup != 0) {
        if warmup > 0 as i32 && crate::src::cgame::cg_main::cg.warmup <= 0 as i32 {
            crate::src::cgame::cg_syscalls::trap_S_StartLocalSound(
                crate::src::cgame::cg_main::cgs.media.countPrepareSound,
                crate::src::qcommon::q_shared::CHAN_ANNOUNCER as i32,
            );
        }
    }
    crate::src::cgame::cg_main::cg.warmup = warmup;
}
/*
================
CG_SetConfigValues

Called on load to set the initial values from configure strings
================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_SetConfigValues() {
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    crate::src::cgame::cg_main::cgs.scores1 =
        atoi(crate::src::cgame::cg_main::CG_ConfigString(6 as i32));
    crate::src::cgame::cg_main::cgs.scores2 =
        atoi(crate::src::cgame::cg_main::CG_ConfigString(7 as i32));
    crate::src::cgame::cg_main::cgs.levelStartTime =
        atoi(crate::src::cgame::cg_main::CG_ConfigString(21 as i32));
    if crate::src::cgame::cg_main::cgs.gametype as u32 == crate::bg_public_h::GT_CTF as i32 as u32 {
        s = crate::src::cgame::cg_main::CG_ConfigString(23 as i32);
        crate::src::cgame::cg_main::cgs.redflag = *s.offset(0 as i32 as isize) as i32 - '0' as i32;
        crate::src::cgame::cg_main::cgs.blueflag = *s.offset(1 as i32 as isize) as i32 - '0' as i32
    }
    crate::src::cgame::cg_main::cg.warmup =
        atoi(crate::src::cgame::cg_main::CG_ConfigString(5 as i32));
}
/*
=====================
CG_ShaderStateChanged
=====================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_ShaderStateChanged() {
    let mut originalShader: [libc::c_char; 64] = [0; 64];
    let mut newShader: [libc::c_char; 64] = [0; 64];
    let mut timeOffset: [libc::c_char; 16] = [0; 16];
    let mut o: *const libc::c_char = 0 as *const libc::c_char;
    let mut n: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    o = crate::src::cgame::cg_main::CG_ConfigString(24 as i32);
    while !o.is_null() && *o as i32 != 0 {
        n = ::libc::strstr(o, b"=\x00" as *const u8 as *const libc::c_char);
        if !(!n.is_null() && *n as i32 != 0) {
            break;
        }
        crate::stdlib::strncpy(
            originalShader.as_mut_ptr(),
            o,
            n.offset_from(o) as libc::c_long as libc::c_ulong,
        );
        originalShader[n.offset_from(o) as libc::c_long as usize] = 0 as i32 as libc::c_char;
        n = n.offset(1);
        t = ::libc::strstr(n, b":\x00" as *const u8 as *const libc::c_char);
        if !(!t.is_null() && *t as i32 != 0) {
            break;
        }
        crate::stdlib::strncpy(
            newShader.as_mut_ptr(),
            n,
            t.offset_from(n) as libc::c_long as libc::c_ulong,
        );
        newShader[t.offset_from(n) as libc::c_long as usize] = 0 as i32 as libc::c_char;
        t = t.offset(1);
        o = ::libc::strstr(t, b"@\x00" as *const u8 as *const libc::c_char);
        if !o.is_null() {
            crate::stdlib::strncpy(
                timeOffset.as_mut_ptr(),
                t,
                o.offset_from(t) as libc::c_long as libc::c_ulong,
            );
            timeOffset[o.offset_from(t) as libc::c_long as usize] = 0 as i32 as libc::c_char;
            o = o.offset(1);
            crate::src::cgame::cg_syscalls::trap_R_RemapShader(
                originalShader.as_mut_ptr(),
                newShader.as_mut_ptr(),
                timeOffset.as_mut_ptr(),
            );
        }
    }
}
/*
================
CG_ConfigStringModified

================
*/

unsafe extern "C" fn CG_ConfigStringModified() {
    let mut str: *const libc::c_char = 0 as *const libc::c_char;
    let mut num: i32 = 0;
    num = atoi(crate::src::cgame::cg_main::CG_Argv(1 as i32));
    // get the gamestate from the client system, which will have the
    // new configstring already integrated
    crate::src::cgame::cg_syscalls::trap_GetGameState(
        &mut crate::src::cgame::cg_main::cgs.gameState as *mut _
            as *mut crate::src::qcommon::q_shared::gameState_t,
    );
    // look up the individual string that was modified
    str = crate::src::cgame::cg_main::CG_ConfigString(num);
    // do something with it if necessary
    if num == 2 as i32 {
        crate::src::cgame::cg_main::CG_StartMusic();
    } else if num == 0 as i32 {
        CG_ParseServerinfo();
    } else if num == 5 as i32 {
        CG_ParseWarmup();
    } else if num == 6 as i32 {
        crate::src::cgame::cg_main::cgs.scores1 = atoi(str)
    } else if num == 7 as i32 {
        crate::src::cgame::cg_main::cgs.scores2 = atoi(str)
    } else if num == 21 as i32 {
        crate::src::cgame::cg_main::cgs.levelStartTime = atoi(str)
    } else if num == 8 as i32 {
        crate::src::cgame::cg_main::cgs.voteTime = atoi(str);
        crate::src::cgame::cg_main::cgs.voteModified = crate::src::qcommon::q_shared::qtrue
    } else if num == 10 as i32 {
        crate::src::cgame::cg_main::cgs.voteYes = atoi(str);
        crate::src::cgame::cg_main::cgs.voteModified = crate::src::qcommon::q_shared::qtrue
    } else if num == 11 as i32 {
        crate::src::cgame::cg_main::cgs.voteNo = atoi(str);
        crate::src::cgame::cg_main::cgs.voteModified = crate::src::qcommon::q_shared::qtrue
    } else if num == 9 as i32 {
        crate::src::qcommon::q_shared::Q_strncpyz(
            crate::src::cgame::cg_main::cgs.voteString.as_mut_ptr(),
            str,
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as i32,
        );
    //MISSIONPACK
    } else if num >= 12 as i32 && num <= 12 as i32 + 1 as i32 {
        crate::src::cgame::cg_main::cgs.teamVoteTime[(num - 12 as i32) as usize] = atoi(str);
        crate::src::cgame::cg_main::cgs.teamVoteModified[(num - 12 as i32) as usize] =
            crate::src::qcommon::q_shared::qtrue
    } else if num >= 16 as i32 && num <= 16 as i32 + 1 as i32 {
        crate::src::cgame::cg_main::cgs.teamVoteYes[(num - 16 as i32) as usize] = atoi(str);
        crate::src::cgame::cg_main::cgs.teamVoteModified[(num - 16 as i32) as usize] =
            crate::src::qcommon::q_shared::qtrue
    } else if num >= 18 as i32 && num <= 18 as i32 + 1 as i32 {
        crate::src::cgame::cg_main::cgs.teamVoteNo[(num - 18 as i32) as usize] = atoi(str);
        crate::src::cgame::cg_main::cgs.teamVoteModified[(num - 18 as i32) as usize] =
            crate::src::qcommon::q_shared::qtrue
    } else if num >= 14 as i32 && num <= 14 as i32 + 1 as i32 {
        crate::src::qcommon::q_shared::Q_strncpyz(
            crate::src::cgame::cg_main::cgs.teamVoteString[(num - 14 as i32) as usize].as_mut_ptr(),
            str,
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as i32,
        );
    } else if num == 22 as i32 {
        crate::src::cgame::cg_main::cg.intermissionStarted =
            atoi(str) as crate::src::qcommon::q_shared::qboolean
    } else if num >= 32 as i32 && num < 32 as i32 + 256 as i32 {
        crate::src::cgame::cg_main::cgs.gameModels[(num - 32 as i32) as usize] =
            crate::src::cgame::cg_syscalls::trap_R_RegisterModel(str)
    } else if num >= 32 as i32 + 256 as i32 && num < 32 as i32 + 256 as i32 + 256 as i32 {
        if *str.offset(0 as i32 as isize) as i32 != '*' as i32 {
            // player specific sounds don't register here
            crate::src::cgame::cg_main::cgs.gameSounds[(num - (32 as i32 + 256 as i32)) as usize] =
                crate::src::cgame::cg_syscalls::trap_S_RegisterSound(
                    str,
                    crate::src::qcommon::q_shared::qfalse,
                )
        }
    } else if num >= 32 as i32 + 256 as i32 + 256 as i32
        && num < 32 as i32 + 256 as i32 + 256 as i32 + 64 as i32
    {
        crate::src::cgame::cg_players::CG_NewClientInfo(
            num - (32 as i32 + 256 as i32 + 256 as i32),
        );
        crate::src::cgame::cg_main::CG_BuildSpectatorString();
    } else if num == 23 as i32 {
        if crate::src::cgame::cg_main::cgs.gametype as u32
            == crate::bg_public_h::GT_CTF as i32 as u32
        {
            // format is rb where its red/blue, 0 is at base, 1 is taken, 2 is dropped
            crate::src::cgame::cg_main::cgs.redflag =
                *str.offset(0 as i32 as isize) as i32 - '0' as i32;
            crate::src::cgame::cg_main::cgs.blueflag =
                *str.offset(1 as i32 as isize) as i32 - '0' as i32
        }
    } else if num == 24 as i32 {
        CG_ShaderStateChanged();
    };
}
/*
=======================
CG_AddToTeamChat

=======================
*/

unsafe extern "C" fn CG_AddToTeamChat(mut str: *const libc::c_char) {
    let mut len: i32 = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ls: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut lastcolor: i32 = 0;
    let mut chatHeight: i32 = 0;
    if crate::src::cgame::cg_main::cg_teamChatHeight.integer < 8 as i32 {
        chatHeight = crate::src::cgame::cg_main::cg_teamChatHeight.integer
    } else {
        chatHeight = 8 as i32
    }
    if chatHeight <= 0 as i32 || crate::src::cgame::cg_main::cg_teamChatTime.integer <= 0 as i32 {
        // team chat disabled, dump into normal chat
        crate::src::cgame::cg_main::cgs.teamLastChatPos = 0 as i32;
        crate::src::cgame::cg_main::cgs.teamChatPos =
            crate::src::cgame::cg_main::cgs.teamLastChatPos;
        return;
    }
    len = 0 as i32;
    p = crate::src::cgame::cg_main::cgs.teamChatMsgs
        [(crate::src::cgame::cg_main::cgs.teamChatPos % chatHeight) as usize]
        .as_mut_ptr();
    *p = 0 as i32 as libc::c_char;
    lastcolor = '7' as i32;
    ls = 0 as *mut libc::c_char;
    while *str != 0 {
        if len > 80 as i32 - 1 as i32 {
            if !ls.is_null() {
                str = str.offset(-(p.offset_from(ls) as libc::c_long as isize));
                str = str.offset(1);
                p = p.offset(-(p.offset_from(ls) as libc::c_long as isize))
            }
            *p = 0 as i32 as libc::c_char;
            crate::src::cgame::cg_main::cgs.teamChatMsgTimes
                [(crate::src::cgame::cg_main::cgs.teamChatPos % chatHeight) as usize] =
                crate::src::cgame::cg_main::cg.time;
            crate::src::cgame::cg_main::cgs.teamChatPos += 1;
            p = crate::src::cgame::cg_main::cgs.teamChatMsgs
                [(crate::src::cgame::cg_main::cgs.teamChatPos % chatHeight) as usize]
                .as_mut_ptr();
            *p = 0 as i32 as libc::c_char;
            let fresh0 = p;
            p = p.offset(1);
            *fresh0 = '^' as i32 as libc::c_char;
            let fresh1 = p;
            p = p.offset(1);
            *fresh1 = lastcolor as libc::c_char;
            len = 0 as i32;
            ls = 0 as *mut libc::c_char
        }
        if crate::src::qcommon::q_shared::Q_IsColorString(str) as u64 != 0 {
            let fresh2 = str;
            str = str.offset(1);
            let fresh3 = p;
            p = p.offset(1);
            *fresh3 = *fresh2;
            lastcolor = *str as i32;
            let fresh4 = str;
            str = str.offset(1);
            let fresh5 = p;
            p = p.offset(1);
            *fresh5 = *fresh4
        } else {
            if *str as i32 == ' ' as i32 {
                ls = p
            }
            let fresh6 = str;
            str = str.offset(1);
            let fresh7 = p;
            p = p.offset(1);
            *fresh7 = *fresh6;
            len += 1
        }
    }
    *p = 0 as i32 as libc::c_char;
    crate::src::cgame::cg_main::cgs.teamChatMsgTimes
        [(crate::src::cgame::cg_main::cgs.teamChatPos % chatHeight) as usize] =
        crate::src::cgame::cg_main::cg.time;
    crate::src::cgame::cg_main::cgs.teamChatPos += 1;
    if crate::src::cgame::cg_main::cgs.teamChatPos - crate::src::cgame::cg_main::cgs.teamLastChatPos
        > chatHeight
    {
        crate::src::cgame::cg_main::cgs.teamLastChatPos =
            crate::src::cgame::cg_main::cgs.teamChatPos - chatHeight
    };
}
/*
===============
CG_MapRestart

The server has issued a map_restart, so the next snapshot
is completely new and should not be interpolated to.

A tournement restart will clear everything, but doesn't
require a reload of all the media
===============
*/

unsafe extern "C" fn CG_MapRestart() {
    if crate::src::cgame::cg_main::cg_showmiss.integer != 0 {
        crate::src::cgame::cg_main::CG_Printf(
            b"CG_MapRestart\n\x00" as *const u8 as *const libc::c_char,
        );
    }
    crate::src::cgame::cg_localents::CG_InitLocalEntities();
    crate::src::cgame::cg_marks::CG_InitMarkPolys();
    crate::src::cgame::cg_particles::CG_ClearParticles();
    // make sure the "3 frags left" warnings play again
    crate::src::cgame::cg_main::cg.fraglimitWarnings = 0 as i32;
    crate::src::cgame::cg_main::cg.timelimitWarnings = 0 as i32;
    crate::src::cgame::cg_main::cg.rewardTime = 0 as i32;
    crate::src::cgame::cg_main::cg.rewardStack = 0 as i32;
    crate::src::cgame::cg_main::cg.intermissionStarted = crate::src::qcommon::q_shared::qfalse;
    crate::src::cgame::cg_main::cg.levelShot = crate::src::qcommon::q_shared::qfalse;
    crate::src::cgame::cg_main::cgs.voteTime = 0 as i32;
    crate::src::cgame::cg_main::cg.mapRestart = crate::src::qcommon::q_shared::qtrue;
    crate::src::cgame::cg_main::CG_StartMusic();
    crate::src::cgame::cg_syscalls::trap_S_ClearLoopingSounds(crate::src::qcommon::q_shared::qtrue);
    // we really should clear more parts of cg here and stop sounds
    // play the "fight" sound if this is a restart without warmup
    if crate::src::cgame::cg_main::cg.warmup == 0 as i32 {
        /* && cgs.gametype == GT_TOURNAMENT */
        crate::src::cgame::cg_syscalls::trap_S_StartLocalSound(
            crate::src::cgame::cg_main::cgs.media.countFightSound,
            crate::src::qcommon::q_shared::CHAN_ANNOUNCER as i32,
        );
        crate::src::cgame::cg_draw::CG_CenterPrint(
            b"FIGHT!\x00" as *const u8 as *const libc::c_char,
            120 as i32,
            32 as i32 * 2 as i32,
        );
    }
    crate::src::cgame::cg_syscalls::trap_Cvar_Set(
        b"cg_thirdPerson\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
    );
}
// MISSIONPACK
/*
=================
CG_RemoveChatEscapeChar
=================
*/

unsafe extern "C" fn CG_RemoveChatEscapeChar(mut text: *mut libc::c_char) {
    let mut i: i32 = 0;
    let mut l: i32 = 0;
    l = 0 as i32;
    i = 0 as i32;
    while *text.offset(i as isize) != 0 {
        if !(*text.offset(i as isize) as i32 == '\u{19}' as i32) {
            let fresh8 = l;
            l = l + 1;
            *text.offset(fresh8 as isize) = *text.offset(i as isize)
        }
        i += 1
    }
    *text.offset(l as isize) = '\u{0}' as i32 as libc::c_char;
}
/*
=================
CG_ServerCommand

The string has been tokenized and can be retrieved with
Cmd_Argc() / Cmd_Argv()
=================
*/

unsafe extern "C" fn CG_ServerCommand() {
    let mut cmd: *const libc::c_char = 0 as *const libc::c_char;
    let mut text: [libc::c_char; 150] = [0; 150];
    cmd = crate::src::cgame::cg_main::CG_Argv(0 as i32);
    if *cmd.offset(0 as i32 as isize) == 0 {
        // server claimed the command
        return;
    }
    if ::libc::strcmp(cmd, b"cp\x00" as *const u8 as *const libc::c_char) == 0 {
        crate::src::cgame::cg_draw::CG_CenterPrint(
            crate::src::cgame::cg_main::CG_Argv(1 as i32),
            (480 as i32 as f64 * 0.30f64) as i32,
            16 as i32,
        );
        return;
    }
    if ::libc::strcmp(cmd, b"cs\x00" as *const u8 as *const libc::c_char) == 0 {
        CG_ConfigStringModified();
        return;
    }
    if ::libc::strcmp(cmd, b"print\x00" as *const u8 as *const libc::c_char) == 0 {
        crate::src::cgame::cg_main::CG_Printf(
            b"%s\x00" as *const u8 as *const libc::c_char,
            crate::src::cgame::cg_main::CG_Argv(1 as i32),
        );
        return;
    }
    if ::libc::strcmp(cmd, b"chat\x00" as *const u8 as *const libc::c_char) == 0 {
        if crate::src::cgame::cg_main::cgs.gametype as u32
            >= crate::bg_public_h::GT_TEAM as i32 as u32
            && crate::src::cgame::cg_main::cg_teamChatsOnly.integer != 0
        {
            return;
        }
        crate::src::cgame::cg_syscalls::trap_S_StartLocalSound(
            crate::src::cgame::cg_main::cgs.media.talkSound,
            crate::src::qcommon::q_shared::CHAN_LOCAL_SOUND as i32,
        );
        crate::src::qcommon::q_shared::Q_strncpyz(
            text.as_mut_ptr(),
            crate::src::cgame::cg_main::CG_Argv(1 as i32),
            150 as i32,
        );
        CG_RemoveChatEscapeChar(text.as_mut_ptr());
        crate::src::cgame::cg_main::CG_Printf(
            b"%s\n\x00" as *const u8 as *const libc::c_char,
            text.as_mut_ptr(),
        );
        return;
    }
    if ::libc::strcmp(cmd, b"tchat\x00" as *const u8 as *const libc::c_char) == 0 {
        crate::src::cgame::cg_syscalls::trap_S_StartLocalSound(
            crate::src::cgame::cg_main::cgs.media.talkSound,
            crate::src::qcommon::q_shared::CHAN_LOCAL_SOUND as i32,
        );
        crate::src::qcommon::q_shared::Q_strncpyz(
            text.as_mut_ptr(),
            crate::src::cgame::cg_main::CG_Argv(1 as i32),
            150 as i32,
        );
        CG_RemoveChatEscapeChar(text.as_mut_ptr());
        CG_AddToTeamChat(text.as_mut_ptr());
        crate::src::cgame::cg_main::CG_Printf(
            b"%s\n\x00" as *const u8 as *const libc::c_char,
            text.as_mut_ptr(),
        );
        return;
    }
    if ::libc::strcmp(cmd, b"scores\x00" as *const u8 as *const libc::c_char) == 0 {
        CG_ParseScores();
        return;
    }
    if ::libc::strcmp(cmd, b"tinfo\x00" as *const u8 as *const libc::c_char) == 0 {
        CG_ParseTeamInfo();
        return;
    }
    if ::libc::strcmp(cmd, b"map_restart\x00" as *const u8 as *const libc::c_char) == 0 {
        CG_MapRestart();
        return;
    }
    if crate::src::qcommon::q_shared::Q_stricmp(
        cmd,
        b"remapShader\x00" as *const u8 as *const libc::c_char,
    ) == 0 as i32
    {
        if crate::src::cgame::cg_syscalls::trap_Argc() == 4 as i32 {
            let mut shader1: [libc::c_char; 64] = [0; 64];
            let mut shader2: [libc::c_char; 64] = [0; 64];
            let mut shader3: [libc::c_char; 64] = [0; 64];
            crate::src::qcommon::q_shared::Q_strncpyz(
                shader1.as_mut_ptr(),
                crate::src::cgame::cg_main::CG_Argv(1 as i32),
                ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as i32,
            );
            crate::src::qcommon::q_shared::Q_strncpyz(
                shader2.as_mut_ptr(),
                crate::src::cgame::cg_main::CG_Argv(2 as i32),
                ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as i32,
            );
            crate::src::qcommon::q_shared::Q_strncpyz(
                shader3.as_mut_ptr(),
                crate::src::cgame::cg_main::CG_Argv(3 as i32),
                ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as i32,
            );
            crate::src::cgame::cg_syscalls::trap_R_RemapShader(
                shader1.as_mut_ptr(),
                shader2.as_mut_ptr(),
                shader3.as_mut_ptr(),
            );
        }
        return;
    }
    // loaddeferred can be both a servercmd and a consolecmd
    if ::libc::strcmp(cmd, b"loaddefered\x00" as *const u8 as *const libc::c_char) == 0 {
        // FIXME: spelled wrong, but not changing for demo
        crate::src::cgame::cg_players::CG_LoadDeferredPlayers();
        return;
    }
    // clientLevelShot is sent before taking a special screenshot for
    // the menu system during development
    if ::libc::strcmp(
        cmd,
        b"clientLevelShot\x00" as *const u8 as *const libc::c_char,
    ) == 0
    {
        crate::src::cgame::cg_main::cg.levelShot = crate::src::qcommon::q_shared::qtrue;
        return;
    }
    crate::src::cgame::cg_main::CG_Printf(
        b"Unknown client game command: %s\n\x00" as *const u8 as *const libc::c_char,
        cmd,
    );
}
//
// cg_servercmds.c
//
/*
====================
CG_ExecuteNewServerCommands

Execute all of the server commands that were received along
with this this snapshot.
====================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_ExecuteNewServerCommands(mut latestSequence: i32) {
    while crate::src::cgame::cg_main::cgs.serverCommandSequence < latestSequence {
        crate::src::cgame::cg_main::cgs.serverCommandSequence += 1;
        if crate::src::cgame::cg_syscalls::trap_GetServerCommand(
            crate::src::cgame::cg_main::cgs.serverCommandSequence,
        ) as u64
            != 0
        {
            CG_ServerCommand();
        }
    }
}

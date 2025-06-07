use ::libc;

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
pub use crate::bg_public_h::PM_DEAD;
pub use crate::bg_public_h::PM_FREEZE;
pub use crate::bg_public_h::PM_INTERMISSION;
pub use crate::bg_public_h::PM_NOCLIP;
pub use crate::bg_public_h::PM_NORMAL;
pub use crate::bg_public_h::PM_SPECTATOR;
pub use crate::bg_public_h::PM_SPINTERMISSION;
pub use crate::bg_public_h::PW_AMMOREGEN;
pub use crate::bg_public_h::PW_BATTLESUIT;
pub use crate::bg_public_h::PW_BLUEFLAG;
pub use crate::bg_public_h::PW_DOUBLER;
pub use crate::bg_public_h::PW_FLIGHT;
pub use crate::bg_public_h::PW_GUARD;
pub use crate::bg_public_h::PW_HASTE;
pub use crate::bg_public_h::PW_INVIS;
pub use crate::bg_public_h::PW_INVULNERABILITY;
pub use crate::bg_public_h::PW_NEUTRALFLAG;
pub use crate::bg_public_h::PW_NONE;
pub use crate::bg_public_h::PW_NUM_POWERUPS;
pub use crate::bg_public_h::PW_QUAD;
pub use crate::bg_public_h::PW_REDFLAG;
pub use crate::bg_public_h::PW_REGEN;
pub use crate::bg_public_h::PW_SCOUT;
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
pub use crate::cg_public_h::snapshot_t;
pub use crate::src::cgame::cg_draw::CG_DrawFlagModel;
pub use crate::src::cgame::cg_draw::CG_DrawHead;
pub use crate::src::cgame::cg_draw::CG_DrawTeamBackground;
pub use crate::src::cgame::cg_drawtools::CG_DrawBigString;
pub use crate::src::cgame::cg_drawtools::CG_DrawBigStringColor;
pub use crate::src::cgame::cg_drawtools::CG_DrawPic;
pub use crate::src::cgame::cg_drawtools::CG_DrawSmallStringColor;
pub use crate::src::cgame::cg_drawtools::CG_DrawStringExt;
pub use crate::src::cgame::cg_drawtools::CG_DrawStrlen;
pub use crate::src::cgame::cg_drawtools::CG_FadeColor;
pub use crate::src::cgame::cg_drawtools::CG_FillRect;
pub use crate::src::cgame::cg_event::CG_PlaceString;
pub use crate::src::cgame::cg_main::cg;
pub use crate::src::cgame::cg_main::cg_drawIcons;
pub use crate::src::cgame::cg_main::cg_paused;
pub use crate::src::cgame::cg_main::cgs;
pub use crate::src::cgame::cg_main::CG_ConfigString;
pub use crate::src::cgame::cg_main::Com_Printf;
pub use crate::src::cgame::cg_players::CG_LoadDeferredPlayers;
pub use crate::src::cgame::cg_syscalls::trap_SendClientCommand;
pub use crate::src::qcommon::q_math::colorWhite;
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
pub use crate::src::qcommon::q_shared::vec4_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::qcommon::q_shared::vmCvar_t;
pub use crate::src::qcommon::q_shared::C2RustUnnamed_0;
pub use crate::src::qcommon::q_shared::Com_sprintf;
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
// width 15
// The new and improved score board
//
// In cases where the number of clients is high, the score board heads are interleaved
// here's the layout
//
//	0   32   80  112  144   240  320  400   <-- pixel position
//  bot head bot head score ping time name
//
//  wins/losses are drawn on bot icon now

static mut localClient: crate::src::qcommon::q_shared::qboolean =
    crate::src::qcommon::q_shared::qfalse;
// true if local client has been displayed
/*
=================
CG_DrawScoreboard
=================
*/

unsafe extern "C" fn CG_DrawClientScore(
    mut y: libc::c_int,
    mut score: *mut crate::cg_local_h::score_t,
    mut color: *mut libc::c_float,
    mut fade: libc::c_float,
    mut largeFormat: crate::src::qcommon::q_shared::qboolean,
) {
    let mut string: [libc::c_char; 1024] = [0; 1024];
    let mut headAngles: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut ci: *mut crate::cg_local_h::clientInfo_t = 0 as *mut crate::cg_local_h::clientInfo_t;
    let mut iconx: libc::c_int = 0;
    let mut headx: libc::c_int = 0;
    if (*score).client < 0 as libc::c_int
        || (*score).client >= crate::src::cgame::cg_main::cgs.maxclients
    {
        crate::src::cgame::cg_main::Com_Printf(
            b"Bad score->client: %i\n\x00" as *const u8 as *const libc::c_char,
            (*score).client,
        );
        return;
    }
    ci = &mut *crate::src::cgame::cg_main::cgs
        .clientinfo
        .as_mut_ptr()
        .offset((*score).client as isize) as *mut crate::cg_local_h::clientInfo_t;
    iconx = 0 as libc::c_int
        + 32 as libc::c_int
        + 6 as libc::c_int * 16 as libc::c_int / 2 as libc::c_int;
    headx = 0 as libc::c_int
        + 64 as libc::c_int
        + 6 as libc::c_int * 16 as libc::c_int / 2 as libc::c_int;
    // draw the handicap or bot skill marker (unless player has flag)
    if (*ci).powerups & (1 as libc::c_int) << crate::bg_public_h::PW_NEUTRALFLAG as libc::c_int != 0
    {
        if largeFormat as u64 != 0 {
            crate::src::cgame::cg_draw::CG_DrawFlagModel(
                iconx as libc::c_float,
                (y - (32 as libc::c_int - 16 as libc::c_int) / 2 as libc::c_int) as libc::c_float,
                32 as libc::c_int as libc::c_float,
                32 as libc::c_int as libc::c_float,
                crate::bg_public_h::TEAM_FREE as libc::c_int,
                crate::src::qcommon::q_shared::qfalse,
            );
        } else {
            crate::src::cgame::cg_draw::CG_DrawFlagModel(
                iconx as libc::c_float,
                y as libc::c_float,
                16 as libc::c_int as libc::c_float,
                16 as libc::c_int as libc::c_float,
                crate::bg_public_h::TEAM_FREE as libc::c_int,
                crate::src::qcommon::q_shared::qfalse,
            );
        }
    } else if (*ci).powerups & (1 as libc::c_int) << crate::bg_public_h::PW_REDFLAG as libc::c_int
        != 0
    {
        if largeFormat as u64 != 0 {
            crate::src::cgame::cg_draw::CG_DrawFlagModel(
                iconx as libc::c_float,
                (y - (32 as libc::c_int - 16 as libc::c_int) / 2 as libc::c_int) as libc::c_float,
                32 as libc::c_int as libc::c_float,
                32 as libc::c_int as libc::c_float,
                crate::bg_public_h::TEAM_RED as libc::c_int,
                crate::src::qcommon::q_shared::qfalse,
            );
        } else {
            crate::src::cgame::cg_draw::CG_DrawFlagModel(
                iconx as libc::c_float,
                y as libc::c_float,
                16 as libc::c_int as libc::c_float,
                16 as libc::c_int as libc::c_float,
                crate::bg_public_h::TEAM_RED as libc::c_int,
                crate::src::qcommon::q_shared::qfalse,
            );
        }
    } else if (*ci).powerups & (1 as libc::c_int) << crate::bg_public_h::PW_BLUEFLAG as libc::c_int
        != 0
    {
        if largeFormat as u64 != 0 {
            crate::src::cgame::cg_draw::CG_DrawFlagModel(
                iconx as libc::c_float,
                (y - (32 as libc::c_int - 16 as libc::c_int) / 2 as libc::c_int) as libc::c_float,
                32 as libc::c_int as libc::c_float,
                32 as libc::c_int as libc::c_float,
                crate::bg_public_h::TEAM_BLUE as libc::c_int,
                crate::src::qcommon::q_shared::qfalse,
            );
        } else {
            crate::src::cgame::cg_draw::CG_DrawFlagModel(
                iconx as libc::c_float,
                y as libc::c_float,
                16 as libc::c_int as libc::c_float,
                16 as libc::c_int as libc::c_float,
                crate::bg_public_h::TEAM_BLUE as libc::c_int,
                crate::src::qcommon::q_shared::qfalse,
            );
        }
    } else {
        if (*ci).botSkill > 0 as libc::c_int && (*ci).botSkill <= 5 as libc::c_int {
            if crate::src::cgame::cg_main::cg_drawIcons.integer != 0 {
                if largeFormat as u64 != 0 {
                    crate::src::cgame::cg_drawtools::CG_DrawPic(
                        iconx as libc::c_float,
                        (y - (32 as libc::c_int - 16 as libc::c_int) / 2 as libc::c_int)
                            as libc::c_float,
                        32 as libc::c_int as libc::c_float,
                        32 as libc::c_int as libc::c_float,
                        crate::src::cgame::cg_main::cgs.media.botSkillShaders
                            [((*ci).botSkill - 1 as libc::c_int) as usize],
                    );
                } else {
                    crate::src::cgame::cg_drawtools::CG_DrawPic(
                        iconx as libc::c_float,
                        y as libc::c_float,
                        16 as libc::c_int as libc::c_float,
                        16 as libc::c_int as libc::c_float,
                        crate::src::cgame::cg_main::cgs.media.botSkillShaders
                            [((*ci).botSkill - 1 as libc::c_int) as usize],
                    );
                }
            }
        } else if (*ci).handicap < 100 as libc::c_int {
            crate::src::qcommon::q_shared::Com_sprintf(
                string.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
                b"%i\x00" as *const u8 as *const libc::c_char,
                (*ci).handicap,
            );
            if crate::src::cgame::cg_main::cgs.gametype as libc::c_uint
                == crate::bg_public_h::GT_TOURNAMENT as libc::c_int as libc::c_uint
            {
                crate::src::cgame::cg_drawtools::CG_DrawSmallStringColor(
                    iconx,
                    y - 16 as libc::c_int / 2 as libc::c_int,
                    string.as_mut_ptr(),
                    color,
                );
            } else {
                crate::src::cgame::cg_drawtools::CG_DrawSmallStringColor(
                    iconx,
                    y,
                    string.as_mut_ptr(),
                    color,
                );
            }
        }
        // draw the wins / losses
        if crate::src::cgame::cg_main::cgs.gametype as libc::c_uint
            == crate::bg_public_h::GT_TOURNAMENT as libc::c_int as libc::c_uint
        {
            crate::src::qcommon::q_shared::Com_sprintf(
                string.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
                b"%i/%i\x00" as *const u8 as *const libc::c_char,
                (*ci).wins,
                (*ci).losses,
            );
            if (*ci).handicap < 100 as libc::c_int && (*ci).botSkill == 0 {
                crate::src::cgame::cg_drawtools::CG_DrawSmallStringColor(
                    iconx,
                    y + 16 as libc::c_int / 2 as libc::c_int,
                    string.as_mut_ptr(),
                    color,
                );
            } else {
                crate::src::cgame::cg_drawtools::CG_DrawSmallStringColor(
                    iconx,
                    y,
                    string.as_mut_ptr(),
                    color,
                );
            }
        }
    }
    // draw the face
    headAngles[2 as libc::c_int as usize] =
        0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
    headAngles[1 as libc::c_int as usize] = headAngles[2 as libc::c_int as usize];
    headAngles[0 as libc::c_int as usize] = headAngles[1 as libc::c_int as usize];
    headAngles[1 as libc::c_int as usize] =
        180 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
    if largeFormat as u64 != 0 {
        crate::src::cgame::cg_draw::CG_DrawHead(
            headx as libc::c_float,
            (y - (48 as libc::c_int - 16 as libc::c_int) / 2 as libc::c_int) as libc::c_float,
            48 as libc::c_int as libc::c_float,
            48 as libc::c_int as libc::c_float,
            (*score).client,
            headAngles.as_mut_ptr(),
        );
    } else {
        crate::src::cgame::cg_draw::CG_DrawHead(
            headx as libc::c_float,
            y as libc::c_float,
            16 as libc::c_int as libc::c_float,
            16 as libc::c_int as libc::c_float,
            (*score).client,
            headAngles.as_mut_ptr(),
        );
    }
    // draw the score line
    if (*score).ping == -(1 as libc::c_int) {
        crate::src::qcommon::q_shared::Com_sprintf(
            string.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
            b" connecting    %s\x00" as *const u8 as *const libc::c_char,
            (*ci).name.as_mut_ptr(),
        );
    } else if (*ci).team as libc::c_uint
        == crate::bg_public_h::TEAM_SPECTATOR as libc::c_int as libc::c_uint
    {
        crate::src::qcommon::q_shared::Com_sprintf(
            string.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
            b" SPECT %3i %4i %s\x00" as *const u8 as *const libc::c_char,
            (*score).ping,
            (*score).time,
            (*ci).name.as_mut_ptr(),
        );
    } else {
        crate::src::qcommon::q_shared::Com_sprintf(
            string.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
            b"%5i %4i %4i %s\x00" as *const u8 as *const libc::c_char,
            (*score).score,
            (*score).ping,
            (*score).time,
            (*ci).name.as_mut_ptr(),
        );
    }
    // highlight your position
    if (*score).client == (*crate::src::cgame::cg_main::cg.snap).ps.clientNum {
        let mut hcolor: [libc::c_float; 4] = [0.; 4];
        let mut rank: libc::c_int = 0;
        localClient = crate::src::qcommon::q_shared::qtrue;
        if (*crate::src::cgame::cg_main::cg.snap).ps.persistant
            [crate::bg_public_h::PERS_TEAM as libc::c_int as usize]
            == crate::bg_public_h::TEAM_SPECTATOR as libc::c_int
            || crate::src::cgame::cg_main::cgs.gametype as libc::c_uint
                >= crate::bg_public_h::GT_TEAM as libc::c_int as libc::c_uint
        {
            rank = -(1 as libc::c_int)
        } else {
            rank = (*crate::src::cgame::cg_main::cg.snap).ps.persistant
                [crate::bg_public_h::PERS_RANK as libc::c_int as usize]
                & !(0x4000 as libc::c_int)
        }
        if rank == 0 as libc::c_int {
            hcolor[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_float;
            hcolor[1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_float;
            hcolor[2 as libc::c_int as usize] = 0.7f32
        } else if rank == 1 as libc::c_int {
            hcolor[0 as libc::c_int as usize] = 0.7f32;
            hcolor[1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_float;
            hcolor[2 as libc::c_int as usize] = 0 as libc::c_int as libc::c_float
        } else if rank == 2 as libc::c_int {
            hcolor[0 as libc::c_int as usize] = 0.7f32;
            hcolor[1 as libc::c_int as usize] = 0.7f32;
            hcolor[2 as libc::c_int as usize] = 0 as libc::c_int as libc::c_float
        } else {
            hcolor[0 as libc::c_int as usize] = 0.7f32;
            hcolor[1 as libc::c_int as usize] = 0.7f32;
            hcolor[2 as libc::c_int as usize] = 0.7f32
        }
        hcolor[3 as libc::c_int as usize] = (fade as libc::c_double * 0.7f64) as libc::c_float;
        crate::src::cgame::cg_drawtools::CG_FillRect(
            (112 as libc::c_int
                + 16 as libc::c_int
                + 6 as libc::c_int * 16 as libc::c_int / 2 as libc::c_int)
                as libc::c_float,
            y as libc::c_float,
            (640 as libc::c_int - 112 as libc::c_int - 16 as libc::c_int) as libc::c_float,
            (16 as libc::c_int + 1 as libc::c_int) as libc::c_float,
            hcolor.as_mut_ptr(),
        );
    }
    crate::src::cgame::cg_drawtools::CG_DrawBigString(
        112 as libc::c_int + 6 as libc::c_int * 16 as libc::c_int / 2 as libc::c_int,
        y,
        string.as_mut_ptr(),
        fade,
    );
    // add the "ready" marker for intermission exiting
    if (*crate::src::cgame::cg_main::cg.snap).ps.stats
        [crate::bg_public_h::STAT_CLIENTS_READY as libc::c_int as usize]
        & (1 as libc::c_int) << (*score).client
        != 0
    {
        crate::src::cgame::cg_drawtools::CG_DrawBigStringColor(
            iconx,
            y,
            b"READY\x00" as *const u8 as *const libc::c_char,
            color,
        );
    };
}
/*
=================
CG_TeamScoreboard
=================
*/

unsafe extern "C" fn CG_TeamScoreboard(
    mut y: libc::c_int,
    mut team: crate::bg_public_h::team_t,
    mut fade: libc::c_float,
    mut maxClients: libc::c_int,
    mut lineHeight: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut score: *mut crate::cg_local_h::score_t = 0 as *mut crate::cg_local_h::score_t;
    let mut color: [libc::c_float; 4] = [0.; 4];
    let mut count: libc::c_int = 0;
    let mut ci: *mut crate::cg_local_h::clientInfo_t = 0 as *mut crate::cg_local_h::clientInfo_t;
    color[2 as libc::c_int as usize] = 1.0f64 as libc::c_float;
    color[1 as libc::c_int as usize] = color[2 as libc::c_int as usize];
    color[0 as libc::c_int as usize] = color[1 as libc::c_int as usize];
    color[3 as libc::c_int as usize] = fade;
    count = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < crate::src::cgame::cg_main::cg.numScores && count < maxClients {
        score = &mut *crate::src::cgame::cg_main::cg
            .scores
            .as_mut_ptr()
            .offset(i as isize) as *mut crate::cg_local_h::score_t;
        ci = &mut *crate::src::cgame::cg_main::cgs
            .clientinfo
            .as_mut_ptr()
            .offset((*score).client as isize) as *mut crate::cg_local_h::clientInfo_t;
        if !(team as libc::c_uint != (*ci).team as libc::c_uint) {
            CG_DrawClientScore(
                y + lineHeight * count,
                score,
                color.as_mut_ptr(),
                fade,
                (lineHeight == 40 as libc::c_int) as libc::c_int
                    as crate::src::qcommon::q_shared::qboolean,
            );
            count += 1
        }
        i += 1
    }
    return count;
}
/*
=================
CG_DrawScoreboard

Draw the normal in-game scoreboard
=================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_DrawOldScoreboard() -> crate::src::qcommon::q_shared::qboolean {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut w: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut n1: libc::c_int = 0;
    let mut n2: libc::c_int = 0;
    let mut fade: libc::c_float = 0.;
    let mut fadeColor: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut maxClients: libc::c_int = 0;
    let mut lineHeight: libc::c_int = 0;
    let mut topBorderSize: libc::c_int = 0;
    let mut bottomBorderSize: libc::c_int = 0;
    // don't draw amuthing if the menu or console is up
    if crate::src::cgame::cg_main::cg_paused.integer != 0 {
        crate::src::cgame::cg_main::cg.deferredPlayerLoading = 0 as libc::c_int;
        return crate::src::qcommon::q_shared::qfalse;
    }
    if crate::src::cgame::cg_main::cgs.gametype as libc::c_uint
        == crate::bg_public_h::GT_SINGLE_PLAYER as libc::c_int as libc::c_uint
        && crate::src::cgame::cg_main::cg.predictedPlayerState.pm_type
            == crate::bg_public_h::PM_INTERMISSION as libc::c_int
    {
        crate::src::cgame::cg_main::cg.deferredPlayerLoading = 0 as libc::c_int;
        return crate::src::qcommon::q_shared::qfalse;
    }
    // don't draw scoreboard during death while warmup up
    if crate::src::cgame::cg_main::cg.warmup != 0
        && crate::src::cgame::cg_main::cg.showScores as u64 == 0
    {
        return crate::src::qcommon::q_shared::qfalse;
    }
    if crate::src::cgame::cg_main::cg.showScores as libc::c_uint != 0
        || crate::src::cgame::cg_main::cg.predictedPlayerState.pm_type
            == crate::bg_public_h::PM_DEAD as libc::c_int
        || crate::src::cgame::cg_main::cg.predictedPlayerState.pm_type
            == crate::bg_public_h::PM_INTERMISSION as libc::c_int
    {
        fade = 1.0f64 as libc::c_float;
        fadeColor = crate::src::qcommon::q_math::colorWhite.as_mut_ptr()
    } else {
        fadeColor = crate::src::cgame::cg_drawtools::CG_FadeColor(
            crate::src::cgame::cg_main::cg.scoreFadeTime,
            200 as libc::c_int,
        );
        if fadeColor.is_null() {
            // next time scoreboard comes up, don't print killer
            crate::src::cgame::cg_main::cg.deferredPlayerLoading = 0 as libc::c_int;
            crate::src::cgame::cg_main::cg.killerName[0 as libc::c_int as usize] =
                0 as libc::c_int as libc::c_char;
            return crate::src::qcommon::q_shared::qfalse;
        }
        fade = *fadeColor
    }
    // fragged by ... line
    if crate::src::cgame::cg_main::cg.killerName[0 as libc::c_int as usize] != 0 {
        s = crate::src::qcommon::q_shared::va(
            b"Fragged by %s\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            crate::src::cgame::cg_main::cg.killerName.as_mut_ptr(),
        );
        w = crate::src::cgame::cg_drawtools::CG_DrawStrlen(s) * 16 as libc::c_int;
        x = (640 as libc::c_int - w) / 2 as libc::c_int;
        y = 40 as libc::c_int;
        crate::src::cgame::cg_drawtools::CG_DrawBigString(x, y, s, fade);
    }
    // current rank
    if (crate::src::cgame::cg_main::cgs.gametype as libc::c_uint)
        < crate::bg_public_h::GT_TEAM as libc::c_int as libc::c_uint
    {
        if (*crate::src::cgame::cg_main::cg.snap).ps.persistant
            [crate::bg_public_h::PERS_TEAM as libc::c_int as usize]
            != crate::bg_public_h::TEAM_SPECTATOR as libc::c_int
        {
            s = crate::src::qcommon::q_shared::va(
                b"%s place with %i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                crate::src::cgame::cg_event::CG_PlaceString(
                    (*crate::src::cgame::cg_main::cg.snap).ps.persistant
                        [crate::bg_public_h::PERS_RANK as libc::c_int as usize]
                        + 1 as libc::c_int,
                ),
                (*crate::src::cgame::cg_main::cg.snap).ps.persistant
                    [crate::bg_public_h::PERS_SCORE as libc::c_int as usize],
            );
            w = crate::src::cgame::cg_drawtools::CG_DrawStrlen(s) * 16 as libc::c_int;
            x = (640 as libc::c_int - w) / 2 as libc::c_int;
            y = 60 as libc::c_int;
            crate::src::cgame::cg_drawtools::CG_DrawBigString(x, y, s, fade);
        }
    } else {
        if crate::src::cgame::cg_main::cg.teamScores[0 as libc::c_int as usize]
            == crate::src::cgame::cg_main::cg.teamScores[1 as libc::c_int as usize]
        {
            s = crate::src::qcommon::q_shared::va(
                b"Teams are tied at %i\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                crate::src::cgame::cg_main::cg.teamScores[0 as libc::c_int as usize],
            )
        } else if crate::src::cgame::cg_main::cg.teamScores[0 as libc::c_int as usize]
            >= crate::src::cgame::cg_main::cg.teamScores[1 as libc::c_int as usize]
        {
            s = crate::src::qcommon::q_shared::va(
                b"Red leads %i to %i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                crate::src::cgame::cg_main::cg.teamScores[0 as libc::c_int as usize],
                crate::src::cgame::cg_main::cg.teamScores[1 as libc::c_int as usize],
            )
        } else {
            s = crate::src::qcommon::q_shared::va(
                b"Blue leads %i to %i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                crate::src::cgame::cg_main::cg.teamScores[1 as libc::c_int as usize],
                crate::src::cgame::cg_main::cg.teamScores[0 as libc::c_int as usize],
            )
        }
        w = crate::src::cgame::cg_drawtools::CG_DrawStrlen(s) * 16 as libc::c_int;
        x = (640 as libc::c_int - w) / 2 as libc::c_int;
        y = 60 as libc::c_int;
        crate::src::cgame::cg_drawtools::CG_DrawBigString(x, y, s, fade);
    }
    // scoreboard
    y = 86 as libc::c_int;
    crate::src::cgame::cg_drawtools::CG_DrawPic(
        (112 as libc::c_int
            + 16 as libc::c_int
            + 6 as libc::c_int * 16 as libc::c_int / 2 as libc::c_int) as libc::c_float,
        y as libc::c_float,
        64 as libc::c_int as libc::c_float,
        32 as libc::c_int as libc::c_float,
        crate::src::cgame::cg_main::cgs.media.scoreboardScore,
    );
    crate::src::cgame::cg_drawtools::CG_DrawPic(
        (112 as libc::c_int + 12 as libc::c_int * 16 as libc::c_int + 8 as libc::c_int
            - 6 as libc::c_int * 16 as libc::c_int / 2 as libc::c_int) as libc::c_float,
        y as libc::c_float,
        64 as libc::c_int as libc::c_float,
        32 as libc::c_int as libc::c_float,
        crate::src::cgame::cg_main::cgs.media.scoreboardPing,
    );
    crate::src::cgame::cg_drawtools::CG_DrawPic(
        (112 as libc::c_int + 17 as libc::c_int * 16 as libc::c_int + 8 as libc::c_int
            - 6 as libc::c_int * 16 as libc::c_int / 2 as libc::c_int) as libc::c_float,
        y as libc::c_float,
        64 as libc::c_int as libc::c_float,
        32 as libc::c_int as libc::c_float,
        crate::src::cgame::cg_main::cgs.media.scoreboardTime,
    );
    crate::src::cgame::cg_drawtools::CG_DrawPic(
        (112 as libc::c_int + 22 as libc::c_int * 16 as libc::c_int
            - 6 as libc::c_int * 16 as libc::c_int / 2 as libc::c_int) as libc::c_float,
        y as libc::c_float,
        64 as libc::c_int as libc::c_float,
        32 as libc::c_int as libc::c_float,
        crate::src::cgame::cg_main::cgs.media.scoreboardName,
    );
    y = 86 as libc::c_int + 32 as libc::c_int;
    // If there are more than SB_MAXCLIENTS_NORMAL, use the interleaved scores
    if crate::src::cgame::cg_main::cg.numScores
        > (420 as libc::c_int - (86 as libc::c_int + 32 as libc::c_int)) / 40 as libc::c_int
    {
        maxClients = (420 as libc::c_int - (86 as libc::c_int + 32 as libc::c_int))
            / 16 as libc::c_int
            - 1 as libc::c_int;
        lineHeight = 16 as libc::c_int;
        topBorderSize = 8 as libc::c_int;
        bottomBorderSize = 16 as libc::c_int
    } else {
        maxClients =
            (420 as libc::c_int - (86 as libc::c_int + 32 as libc::c_int)) / 40 as libc::c_int;
        lineHeight = 40 as libc::c_int;
        topBorderSize = 16 as libc::c_int;
        bottomBorderSize = 16 as libc::c_int
    }
    localClient = crate::src::qcommon::q_shared::qfalse;
    if crate::src::cgame::cg_main::cgs.gametype as libc::c_uint
        >= crate::bg_public_h::GT_TEAM as libc::c_int as libc::c_uint
    {
        //
        // teamplay scoreboard
        //
        y += lineHeight / 2 as libc::c_int;
        if crate::src::cgame::cg_main::cg.teamScores[0 as libc::c_int as usize]
            >= crate::src::cgame::cg_main::cg.teamScores[1 as libc::c_int as usize]
        {
            n1 = CG_TeamScoreboard(
                y,
                crate::bg_public_h::TEAM_RED,
                fade,
                maxClients,
                lineHeight,
            );
            crate::src::cgame::cg_draw::CG_DrawTeamBackground(
                0 as libc::c_int,
                y - topBorderSize,
                640 as libc::c_int,
                n1 * lineHeight + bottomBorderSize,
                0.33f32,
                crate::bg_public_h::TEAM_RED as libc::c_int,
            );
            y += n1 * lineHeight + 16 as libc::c_int;
            maxClients -= n1;
            n2 = CG_TeamScoreboard(
                y,
                crate::bg_public_h::TEAM_BLUE,
                fade,
                maxClients,
                lineHeight,
            );
            crate::src::cgame::cg_draw::CG_DrawTeamBackground(
                0 as libc::c_int,
                y - topBorderSize,
                640 as libc::c_int,
                n2 * lineHeight + bottomBorderSize,
                0.33f32,
                crate::bg_public_h::TEAM_BLUE as libc::c_int,
            );
            y += n2 * lineHeight + 16 as libc::c_int;
            maxClients -= n2
        } else {
            n1 = CG_TeamScoreboard(
                y,
                crate::bg_public_h::TEAM_BLUE,
                fade,
                maxClients,
                lineHeight,
            );
            crate::src::cgame::cg_draw::CG_DrawTeamBackground(
                0 as libc::c_int,
                y - topBorderSize,
                640 as libc::c_int,
                n1 * lineHeight + bottomBorderSize,
                0.33f32,
                crate::bg_public_h::TEAM_BLUE as libc::c_int,
            );
            y += n1 * lineHeight + 16 as libc::c_int;
            maxClients -= n1;
            n2 = CG_TeamScoreboard(
                y,
                crate::bg_public_h::TEAM_RED,
                fade,
                maxClients,
                lineHeight,
            );
            crate::src::cgame::cg_draw::CG_DrawTeamBackground(
                0 as libc::c_int,
                y - topBorderSize,
                640 as libc::c_int,
                n2 * lineHeight + bottomBorderSize,
                0.33f32,
                crate::bg_public_h::TEAM_RED as libc::c_int,
            );
            y += n2 * lineHeight + 16 as libc::c_int;
            maxClients -= n2
        }
        n1 = CG_TeamScoreboard(
            y,
            crate::bg_public_h::TEAM_SPECTATOR,
            fade,
            maxClients,
            lineHeight,
        );
        y += n1 * lineHeight + 16 as libc::c_int
    } else {
        //
        // free for all scoreboard
        //
        n1 = CG_TeamScoreboard(
            y,
            crate::bg_public_h::TEAM_FREE,
            fade,
            maxClients,
            lineHeight,
        );
        y += n1 * lineHeight + 16 as libc::c_int;
        n2 = CG_TeamScoreboard(
            y,
            crate::bg_public_h::TEAM_SPECTATOR,
            fade,
            maxClients - n1,
            lineHeight,
        );
        y += n2 * lineHeight + 16 as libc::c_int
    }
    if localClient as u64 == 0 {
        // draw local client at the bottom
        i = 0 as libc::c_int;
        while i < crate::src::cgame::cg_main::cg.numScores {
            if crate::src::cgame::cg_main::cg.scores[i as usize].client
                == (*crate::src::cgame::cg_main::cg.snap).ps.clientNum
            {
                CG_DrawClientScore(
                    y,
                    &mut *crate::src::cgame::cg_main::cg
                        .scores
                        .as_mut_ptr()
                        .offset(i as isize),
                    fadeColor,
                    fade,
                    (lineHeight == 40 as libc::c_int) as libc::c_int
                        as crate::src::qcommon::q_shared::qboolean,
                );
                break;
            } else {
                i += 1
            }
        }
    }
    // load any models that have been deferred
    crate::src::cgame::cg_main::cg.deferredPlayerLoading += 1;
    if crate::src::cgame::cg_main::cg.deferredPlayerLoading > 10 as libc::c_int {
        crate::src::cgame::cg_players::CG_LoadDeferredPlayers();
    }
    return crate::src::qcommon::q_shared::qtrue;
}
//================================================================================
/*
================
CG_CenterGiantLine
================
*/

unsafe extern "C" fn CG_CenterGiantLine(mut y: libc::c_float, mut string: *const libc::c_char) {
    let mut x: libc::c_float = 0.;
    let mut color: crate::src::qcommon::q_shared::vec4_t = [0.; 4];
    color[0 as libc::c_int as usize] = 1 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
    color[1 as libc::c_int as usize] = 1 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
    color[2 as libc::c_int as usize] = 1 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
    color[3 as libc::c_int as usize] = 1 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
    x = (0.5f64
        * (640 as libc::c_int
            - 32 as libc::c_int * crate::src::cgame::cg_drawtools::CG_DrawStrlen(string))
            as libc::c_double) as libc::c_float;
    crate::src::cgame::cg_drawtools::CG_DrawStringExt(
        x as libc::c_int,
        y as libc::c_int,
        string,
        color.as_mut_ptr(),
        crate::src::qcommon::q_shared::qtrue,
        crate::src::qcommon::q_shared::qtrue,
        32 as libc::c_int,
        48 as libc::c_int,
        0 as libc::c_int,
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
// The entire cgame module is unloaded and reloaded on each level change,
// so there is NO persistant data between levels on the client side.
// If you absolutely need something stored, it can either be kept
// by the server in the server stored userinfos, or stashed in a cvar.
// time for fragments to sink into ground before going away
// amount to scale up the icons when activating
// num frame for '-' stats digit
// very large characters
//=================================================
// player entities need to track more information
// than any other type of entity.
// note that not every player entity is a client entity,
// because corpses after respawn are outside the normal
// client numbering range
// when changing animation, set animationTime to frameTime + lerping time
// The current lerp will finish out, then it will lerp to the new animation
// time when ->oldFrame was exactly on
// time when ->frame will be exactly on
// may include ANIM_TOGGLEBIT
// time when the first frame of the animation will be exact
// flip from 0 to 1
// machinegun spinning
//=================================================
// centity_t have a direct corespondence with gentity_t in the game, but
// only the entityState_t is directly communicated to the cgame
// from cg.frame
// from cg.nextFrame, if available
// true if next is valid to interpolate to
// true if cg.frame holds this entity
// move to playerEntity?
// so missile trails can handle dropped initial packets
// last time this entity was found in a snapshot
// decay the error from this time
// false if origin / angles is an interpolation
// exact interpolated position of entity on this frame
//======================================================================
// local entities are created as a result of events or predicted actions,
// and live independently from all server transmitted entities
// fade alpha instead of rgb
// do not scale size over time
// tumble over time, used for ejecting shells
// sound 1 for kamikaze
// sound 2 for kamikaze
// fragment local entities can leave marks on walls
// fragment local entities can make sounds on impacts
// 1.0 / (endTime - startTime)
// 0.0 = no bounce, 1.0 = perfect
// mark to leave on fragment impact
//======================================================================
// each client has an associated clientInfo_t
// that contains media references necessary to present the
// client model and other color coded effects
// this is regenerated each time a client's configstring changes,
// usually as a result of a userinfo (name, model, etc) change
// 0 = not bot, 1-5 = bot
// updated by score servercmds
// location index for team mode
// you only get this info about your teammates
// in tourney mode
// task in teamplay (offence/defence)
// true when this is a team leader
// so can display quad/flag status
// when clientinfo is changed, the loading of models/skins/sounds
// can be deferred until you are dead, to prevent hitches in
// gameplay
// true if using the new mission pack animations
// true if legs yaw is always the same as torso yaw
// true if torso never changes yaw
// move head in icon views
// from model
// each WP_* weapon enum has an associated weaponInfo_t
// that contains media references necessary to present the
// weapon and its effects
// the hands don't actually draw, they just position the weapon
// so it will rotate centered instead of by tag
// fast firing weapons randomly choose
// each IT_* item has an associated itemInfo_t
// that constains media references necessary to present the
// item and its effects
//======================================================================
// all cg.stepTime, cg.duckTime, cg.landTime, etc are set to cg.time when the action
// occurs, and they will have visible effects for #define STEP_TIME or whatever msec after
// incremented each frame
// taking a level menu screenshot
// don't defer players at initial startup
// don't play voice rewards, because game will end shortly
// there are only one or two snapshot_t that are relevant at a time
// the number of snapshots the client system has received
// the time from latestSnapshotNum, so we don't need to read the snapshot yet
// cg.snap->serverTime <= cg.time
// cg.nextSnap->serverTime > cg.time, or NULL
// (float)( cg.time - cg.frame->serverTime ) / (cg.nextFrame->serverTime - cg.frame->serverTime)
// cg.time - cg.oldTime
// this is the time value that the client
// is rendering at.
// time at last frame, used for missile trails and prediction checking
// either cg.snap->time or cg.nextSnap->time
// 5 min, 1 min, overtime
// set on a map restart to set back the weapon
// during deaths, chasecams, etc
// prediction state
// true if prediction has hit a trigger_teleport
// clear until the first call to CG_PredictPlayerState
// for stair up smoothing
// for duck viewheight smoothing
// for landing hard
// input state sent to server
// auto rotating items
// view rendering
// will be converted to refdef.viewaxis
// zoom key
// information screen text during loading
// scoreboard
// list of names
// length of list
// width in device units
// next time to offset
// current paint x
// current paint x
// current offset from start
// current offset from start
// centerprinting
// low ammo warning state
// 1 = low, 2 = empty
// crosshair client ID
// powerup active flashing
// attacking player
// reward medals
// sound buffer mainly for announcer sounds
// warmup countdown
//==========================
// the pulse around the crosshair is timed separately
// blend blobs
// status bar head
// view movement
// temp working variables for player view
//qboolean cameraMode;		// if rendering from a loaded camera
// development tool
// all of the model, shader, and sound references that are
// loaded at gamestate time are stored in cgMedia_t
// Other media that can be tied to clients, weapons, or items are
// stored in the clientInfo_t, itemInfo_t, weaponInfo_t, and powerupInfo_t
// gib explosions
// wall mark shaders
// powerup shaders
// weapon effect models
// weapon effect shaders
// special effects models
// scoreboard headers
// medals shown during gameplay
// sounds
//sfxHandle_t	sfx_railg;
// teamplay sounds
// tournament sounds
// The client game static (cgs) structure hold everything
// loaded or calculated from the gamestate.  It will NOT
// be cleared when a tournement restart is done, allowing
// all clients to begin playing instantly
// gamestate from server
// rendering configuration
// derived from glconfig
// reliable command stream counter
// the number of snapshots cgame has requested
// detected on startup by checking sv_running
// parsed from serverinfo
// beep whenever changed
// beep whenever changed
// from configstrings
// flag status from configstrings
//
// locally derived information from gamestate
//
// teamchat width is *3 because of embedded color codes
// orders
// media
//==============================================================================
//extern	vmCvar_t		cg_pmove_fixed;
//
// cg_main.c
//
//
// cg_view.c
//
//
// cg_drawtools.c
//
//
// cg_draw.c, cg_newDraw.c
//
//
// cg_player.c
//
//
// cg_predict.c
//
//
// cg_events.c
//
//
// cg_ents.c
//
//
// cg_weapons.c
//
// should this be in pmove?
//
// cg_marks.c
//
//
// cg_localents.c
//
//
// cg_effects.c
//
//
// cg_snapshot.c
//
//
// cg_info.c
//
//
// cg_scoreboard.c
//
/*
=================
CG_DrawTourneyScoreboard

Draw the oversize scoreboard for tournements
=================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_DrawTourneyScoreboard() {
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut color: crate::src::qcommon::q_shared::vec4_t = [0.; 4];
    let mut min: libc::c_int = 0;
    let mut tens: libc::c_int = 0;
    let mut ones: libc::c_int = 0;
    let mut ci: *mut crate::cg_local_h::clientInfo_t = 0 as *mut crate::cg_local_h::clientInfo_t;
    let mut y: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    // request more scores regularly
    if (crate::src::cgame::cg_main::cg.scoresRequestTime + 2000 as libc::c_int)
        < crate::src::cgame::cg_main::cg.time
    {
        crate::src::cgame::cg_main::cg.scoresRequestTime = crate::src::cgame::cg_main::cg.time;
        crate::src::cgame::cg_syscalls::trap_SendClientCommand(
            b"score\x00" as *const u8 as *const libc::c_char,
        );
    }
    // draw the dialog background
    color[2 as libc::c_int as usize] = 0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
    color[1 as libc::c_int as usize] = color[2 as libc::c_int as usize];
    color[0 as libc::c_int as usize] = color[1 as libc::c_int as usize];
    color[3 as libc::c_int as usize] = 1 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
    crate::src::cgame::cg_drawtools::CG_FillRect(
        0 as libc::c_int as libc::c_float,
        0 as libc::c_int as libc::c_float,
        640 as libc::c_int as libc::c_float,
        480 as libc::c_int as libc::c_float,
        color.as_mut_ptr(),
    );
    color[0 as libc::c_int as usize] = 1 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
    color[1 as libc::c_int as usize] = 1 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
    color[2 as libc::c_int as usize] = 1 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
    color[3 as libc::c_int as usize] = 1 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
    // print the mesage of the day
    s = crate::src::cgame::cg_main::CG_ConfigString(4 as libc::c_int);
    if *s.offset(0 as libc::c_int as isize) == 0 {
        s = b"Scoreboard\x00" as *const u8 as *const libc::c_char
    }
    // print optional title
    CG_CenterGiantLine(8 as libc::c_int as libc::c_float, s);
    // print server time
    ones = crate::src::cgame::cg_main::cg.time / 1000 as libc::c_int;
    min = ones / 60 as libc::c_int;
    ones %= 60 as libc::c_int;
    tens = ones / 10 as libc::c_int;
    ones %= 10 as libc::c_int;
    s = crate::src::qcommon::q_shared::va(
        b"%i:%i%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        min,
        tens,
        ones,
    );
    CG_CenterGiantLine(64 as libc::c_int as libc::c_float, s);
    // print the two scores
    y = 160 as libc::c_int;
    if crate::src::cgame::cg_main::cgs.gametype as libc::c_uint
        >= crate::bg_public_h::GT_TEAM as libc::c_int as libc::c_uint
    {
        //
        // teamplay scoreboard
        //
        crate::src::cgame::cg_drawtools::CG_DrawStringExt(
            8 as libc::c_int,
            y,
            b"Red Team\x00" as *const u8 as *const libc::c_char,
            color.as_mut_ptr(),
            crate::src::qcommon::q_shared::qtrue,
            crate::src::qcommon::q_shared::qtrue,
            32 as libc::c_int,
            48 as libc::c_int,
            0 as libc::c_int,
        );
        s = crate::src::qcommon::q_shared::va(
            b"%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            crate::src::cgame::cg_main::cg.teamScores[0 as libc::c_int as usize],
        );
        crate::src::cgame::cg_drawtools::CG_DrawStringExt(
            (632 as libc::c_int as libc::c_ulong).wrapping_sub(
                (32 as libc::c_int as libc::c_ulong).wrapping_mul(crate::stdlib::strlen(s)),
            ) as libc::c_int,
            y,
            s,
            color.as_mut_ptr(),
            crate::src::qcommon::q_shared::qtrue,
            crate::src::qcommon::q_shared::qtrue,
            32 as libc::c_int,
            48 as libc::c_int,
            0 as libc::c_int,
        );
        y += 64 as libc::c_int;
        crate::src::cgame::cg_drawtools::CG_DrawStringExt(
            8 as libc::c_int,
            y,
            b"Blue Team\x00" as *const u8 as *const libc::c_char,
            color.as_mut_ptr(),
            crate::src::qcommon::q_shared::qtrue,
            crate::src::qcommon::q_shared::qtrue,
            32 as libc::c_int,
            48 as libc::c_int,
            0 as libc::c_int,
        );
        s = crate::src::qcommon::q_shared::va(
            b"%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            crate::src::cgame::cg_main::cg.teamScores[1 as libc::c_int as usize],
        );
        crate::src::cgame::cg_drawtools::CG_DrawStringExt(
            (632 as libc::c_int as libc::c_ulong).wrapping_sub(
                (32 as libc::c_int as libc::c_ulong).wrapping_mul(crate::stdlib::strlen(s)),
            ) as libc::c_int,
            y,
            s,
            color.as_mut_ptr(),
            crate::src::qcommon::q_shared::qtrue,
            crate::src::qcommon::q_shared::qtrue,
            32 as libc::c_int,
            48 as libc::c_int,
            0 as libc::c_int,
        );
    } else {
        //
        // free for all scoreboard
        //
        i = 0 as libc::c_int;
        while i < 64 as libc::c_int {
            ci = &mut *crate::src::cgame::cg_main::cgs
                .clientinfo
                .as_mut_ptr()
                .offset(i as isize) as *mut crate::cg_local_h::clientInfo_t;
            if !((*ci).infoValid as u64 == 0) {
                if !((*ci).team as libc::c_uint
                    != crate::bg_public_h::TEAM_FREE as libc::c_int as libc::c_uint)
                {
                    crate::src::cgame::cg_drawtools::CG_DrawStringExt(
                        8 as libc::c_int,
                        y,
                        (*ci).name.as_mut_ptr(),
                        color.as_mut_ptr(),
                        crate::src::qcommon::q_shared::qtrue,
                        crate::src::qcommon::q_shared::qtrue,
                        32 as libc::c_int,
                        48 as libc::c_int,
                        0 as libc::c_int,
                    );
                    s = crate::src::qcommon::q_shared::va(
                        b"%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        (*ci).score,
                    );
                    crate::src::cgame::cg_drawtools::CG_DrawStringExt(
                        (632 as libc::c_int as libc::c_ulong).wrapping_sub(
                            (32 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(crate::stdlib::strlen(s)),
                        ) as libc::c_int,
                        y,
                        s,
                        color.as_mut_ptr(),
                        crate::src::qcommon::q_shared::qtrue,
                        crate::src::qcommon::q_shared::qtrue,
                        32 as libc::c_int,
                        48 as libc::c_int,
                        0 as libc::c_int,
                    );
                    y += 64 as libc::c_int
                }
            }
            i += 1
        }
    };
}

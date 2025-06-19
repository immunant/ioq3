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

pub use crate::bg_public_h::animation_s;
pub use crate::bg_public_h::animation_t;
pub use crate::bg_public_h::gametype_t;
pub use crate::bg_public_h::gender_t;
pub use crate::bg_public_h::gitem_s;
pub use crate::bg_public_h::gitem_t;
pub use crate::bg_public_h::itemType_t;
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
pub use crate::src::cgame::cg_drawtools::CG_DrawPic;
pub use crate::src::cgame::cg_drawtools::UI_DrawProportionalString;
pub use crate::src::cgame::cg_info::stdlib_h::atoi;
pub use crate::src::cgame::cg_main::cg;
pub use crate::src::cgame::cg_main::cgs;
pub use crate::src::cgame::cg_main::CG_ConfigString;
pub use crate::src::cgame::cg_syscalls::trap_Cvar_VariableStringBuffer;
pub use crate::src::cgame::cg_syscalls::trap_R_DrawStretchPic;
pub use crate::src::cgame::cg_syscalls::trap_R_RegisterShader;
pub use crate::src::cgame::cg_syscalls::trap_R_RegisterShaderNoMip;
pub use crate::src::cgame::cg_syscalls::trap_R_SetColor;
pub use crate::src::cgame::cg_syscalls::trap_S_RegisterSound;
pub use crate::src::cgame::cg_syscalls::trap_UpdateScreen;
pub use crate::src::game::bg_misc::bg_itemlist;
pub use crate::src::qcommon::q_math::colorWhite;
pub use crate::src::qcommon::q_shared::byte;
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
pub use crate::src::qcommon::q_shared::Com_sprintf;
pub use crate::src::qcommon::q_shared::Info_ValueForKey;
pub use crate::src::qcommon::q_shared::Q_CleanStr;
pub use crate::src::qcommon::q_shared::Q_strncpyz;
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

pub use ::libc::strtol;

static mut loadingPlayerIconCount: i32 = 0;

static mut loadingItemIconCount: i32 = 0;

static mut loadingPlayerIcons: [qhandle_t; 16] = [0; 16];

static mut loadingItemIcons: [qhandle_t; 26] = [0; 26];
/*
===================
CG_DrawLoadingIcons
===================
*/

unsafe extern "C" fn CG_DrawLoadingIcons() {
    let mut n: i32 = 0;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    n = 0 as i32;
    while n < loadingPlayerIconCount {
        x = 16 as i32 + n * 78 as i32;
        y = 324 as i32 - 40 as i32;
        CG_DrawPic(
            x as f32,
            y as f32,
            64 as i32 as f32,
            64 as i32 as f32,
            loadingPlayerIcons[n as usize],
        );
        n += 1
    }
    n = 0 as i32;
    while n < loadingItemIconCount {
        y = 400 as i32 - 40 as i32;
        if n >= 13 as i32 {
            y += 40 as i32
        }
        x = 16 as i32 + n % 13 as i32 * 48 as i32;
        CG_DrawPic(
            x as f32,
            y as f32,
            32 as i32 as f32,
            32 as i32 as f32,
            loadingItemIcons[n as usize],
        );
        n += 1
    }
}
/*
======================
CG_LoadingString

======================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_LoadingString(mut s: *const libc::c_char) {
    Q_strncpyz(
        cg.infoScreenText.as_mut_ptr(),
        s,
        ::std::mem::size_of::<[libc::c_char; 1024]>() as usize as i32,
    );
    trap_UpdateScreen();
}
/*
===================
CG_LoadingItem
===================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_LoadingItem(mut itemNum: i32) {
    let mut item: *mut gitem_t = 0 as *mut gitem_t;
    item = &mut *bg_itemlist.as_mut_ptr().offset(itemNum as isize) as *mut gitem_t;
    if !(*item).icon.is_null() && loadingItemIconCount < 26 as i32 {
        let fresh0 = loadingItemIconCount;
        loadingItemIconCount = loadingItemIconCount + 1;
        loadingItemIcons[fresh0 as usize] = trap_R_RegisterShaderNoMip((*item).icon)
    }
    CG_LoadingString((*item).pickup_name);
}
/*
===================
CG_LoadingClient
===================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_LoadingClient(mut clientNum: i32) {
    let mut info: *const libc::c_char = 0 as *const libc::c_char;
    let mut skin: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut personality: [libc::c_char; 64] = [0; 64];
    let mut model: [libc::c_char; 64] = [0; 64];
    let mut iconName: [libc::c_char; 64] = [0; 64];
    info = CG_ConfigString(32 as i32 + 256 as i32 + 256 as i32 + clientNum);
    if loadingPlayerIconCount < 16 as i32 {
        Q_strncpyz(
            model.as_mut_ptr(),
            Info_ValueForKey(info, b"model\x00" as *const u8 as *const libc::c_char),
            ::std::mem::size_of::<[libc::c_char; 64]>() as usize as i32,
        );
        skin = libc::strrchr(model.as_mut_ptr(), '/' as i32);
        if !skin.is_null() {
            let fresh1 = skin;
            skin = skin.offset(1);
            *fresh1 = '\u{0}' as i32 as libc::c_char
        } else {
            skin = b"default\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
        }
        Com_sprintf(
            iconName.as_mut_ptr(),
            64 as i32,
            b"models/players/%s/icon_%s.tga\x00" as *const u8 as *const libc::c_char,
            model.as_mut_ptr(),
            skin,
        );
        loadingPlayerIcons[loadingPlayerIconCount as usize] =
            trap_R_RegisterShaderNoMip(iconName.as_mut_ptr());
        if loadingPlayerIcons[loadingPlayerIconCount as usize] == 0 {
            Com_sprintf(
                iconName.as_mut_ptr(),
                64 as i32,
                b"models/players/characters/%s/icon_%s.tga\x00" as *const u8 as *const libc::c_char,
                model.as_mut_ptr(),
                skin,
            );
            loadingPlayerIcons[loadingPlayerIconCount as usize] =
                trap_R_RegisterShaderNoMip(iconName.as_mut_ptr())
        }
        if loadingPlayerIcons[loadingPlayerIconCount as usize] == 0 {
            Com_sprintf(
                iconName.as_mut_ptr(),
                64 as i32,
                b"models/players/%s/icon_%s.tga\x00" as *const u8 as *const libc::c_char,
                b"sarge\x00" as *const u8 as *const libc::c_char,
                b"default\x00" as *const u8 as *const libc::c_char,
            );
            loadingPlayerIcons[loadingPlayerIconCount as usize] =
                trap_R_RegisterShaderNoMip(iconName.as_mut_ptr())
        }
        if loadingPlayerIcons[loadingPlayerIconCount as usize] != 0 {
            loadingPlayerIconCount += 1
        }
    }
    Q_strncpyz(
        personality.as_mut_ptr(),
        Info_ValueForKey(info, b"n\x00" as *const u8 as *const libc::c_char),
        ::std::mem::size_of::<[libc::c_char; 64]>() as usize as i32,
    );
    Q_CleanStr(personality.as_mut_ptr());
    if cgs.gametype as u32 == GT_SINGLE_PLAYER as i32 as u32 {
        trap_S_RegisterSound(
            va(
                b"sound/player/announce/%s.wav\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                personality.as_mut_ptr(),
            ),
            qtrue,
        );
    }
    CG_LoadingString(personality.as_mut_ptr());
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
/*
====================
CG_DrawInformation

Draw all the status / pacifier stuff during level loading
====================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_DrawInformation() {
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut info: *const libc::c_char = 0 as *const libc::c_char;
    let mut sysInfo: *const libc::c_char = 0 as *const libc::c_char;
    let mut y: i32 = 0;
    let mut value: i32 = 0;
    let mut levelshot: qhandle_t = 0;
    let mut detail: qhandle_t = 0;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    info = CG_ConfigString(0 as i32);
    sysInfo = CG_ConfigString(1 as i32);
    s = Info_ValueForKey(info, b"mapname\x00" as *const u8 as *const libc::c_char);
    levelshot = trap_R_RegisterShaderNoMip(va(
        b"levelshots/%s.tga\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        s,
    ));
    if levelshot == 0 {
        levelshot = trap_R_RegisterShaderNoMip(
            b"menu/art/unknownmap\x00" as *const u8 as *const libc::c_char,
        )
    }
    trap_R_SetColor(0 as *const f32);
    CG_DrawPic(
        0 as i32 as f32,
        0 as i32 as f32,
        640 as i32 as f32,
        480 as i32 as f32,
        levelshot,
    );
    // blend a detail texture over it
    detail = trap_R_RegisterShader(b"levelShotDetail\x00" as *const u8 as *const libc::c_char);
    trap_R_DrawStretchPic(
        0 as i32 as f32,
        0 as i32 as f32,
        cgs.glconfig.vidWidth as f32,
        cgs.glconfig.vidHeight as f32,
        0 as i32 as f32,
        0 as i32 as f32,
        2.5f64 as f32,
        2 as i32 as f32,
        detail,
    );
    // draw the icons of things as they are loaded
    CG_DrawLoadingIcons();
    // the first 150 rows are reserved for the client connection
    // screen to write into
    if cg.infoScreenText[0 as i32 as usize] != 0 {
        UI_DrawProportionalString(
            320 as i32,
            128 as i32 - 32 as i32,
            va(
                b"Loading... %s\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                cg.infoScreenText.as_mut_ptr(),
            ),
            0x1 as i32 | 0x10 as i32 | 0x800 as i32,
            colorWhite.as_mut_ptr(),
        );
    } else {
        UI_DrawProportionalString(
            320 as i32,
            128 as i32 - 32 as i32,
            b"Awaiting snapshot...\x00" as *const u8 as *const libc::c_char,
            0x1 as i32 | 0x10 as i32 | 0x800 as i32,
            colorWhite.as_mut_ptr(),
        );
    }
    // draw info string information
    y = 180 as i32 - 32 as i32;
    // don't print server lines if playing a local game
    trap_Cvar_VariableStringBuffer(
        b"sv_running\x00" as *const u8 as *const libc::c_char,
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as usize as i32,
    );
    if atoi(buf.as_mut_ptr()) == 0 {
        // server hostname
        Q_strncpyz(
            buf.as_mut_ptr(),
            Info_ValueForKey(info, b"sv_hostname\x00" as *const u8 as *const libc::c_char),
            1024 as i32,
        );
        Q_CleanStr(buf.as_mut_ptr());
        UI_DrawProportionalString(
            320 as i32,
            y,
            buf.as_mut_ptr(),
            0x1 as i32 | 0x10 as i32 | 0x800 as i32,
            colorWhite.as_mut_ptr(),
        );
        y += 27 as i32;
        // pure server
        s = Info_ValueForKey(sysInfo, b"sv_pure\x00" as *const u8 as *const libc::c_char);
        if *s.offset(0 as i32 as isize) as i32 == '1' as i32 {
            UI_DrawProportionalString(
                320 as i32,
                y,
                b"Pure Server\x00" as *const u8 as *const libc::c_char,
                0x1 as i32 | 0x10 as i32 | 0x800 as i32,
                colorWhite.as_mut_ptr(),
            );
            y += 27 as i32
        }
        // server-specific message of the day
        s = CG_ConfigString(4 as i32);
        if *s.offset(0 as i32 as isize) != 0 {
            UI_DrawProportionalString(
                320 as i32,
                y,
                s,
                0x1 as i32 | 0x10 as i32 | 0x800 as i32,
                colorWhite.as_mut_ptr(),
            );
            y += 27 as i32
        }
        // some extra space after hostname and motd
        y += 10 as i32
    }
    // map-specific message (long map name)
    s = CG_ConfigString(3 as i32);
    if *s.offset(0 as i32 as isize) != 0 {
        UI_DrawProportionalString(
            320 as i32,
            y,
            s,
            0x1 as i32 | 0x10 as i32 | 0x800 as i32,
            colorWhite.as_mut_ptr(),
        );
        y += 27 as i32
    }
    // cheats warning
    s = Info_ValueForKey(
        sysInfo,
        b"sv_cheats\x00" as *const u8 as *const libc::c_char,
    );
    if *s.offset(0 as i32 as isize) as i32 == '1' as i32 {
        UI_DrawProportionalString(
            320 as i32,
            y,
            b"CHEATS ARE ENABLED\x00" as *const u8 as *const libc::c_char,
            0x1 as i32 | 0x10 as i32 | 0x800 as i32,
            colorWhite.as_mut_ptr(),
        );
        y += 27 as i32
    }
    // game type
    match cgs.gametype as u32 {
        0 => s = b"Free For All\x00" as *const u8 as *const libc::c_char,
        2 => s = b"Single Player\x00" as *const u8 as *const libc::c_char,
        1 => s = b"Tournament\x00" as *const u8 as *const libc::c_char,
        3 => s = b"Team Deathmatch\x00" as *const u8 as *const libc::c_char,
        4 => s = b"Capture The Flag\x00" as *const u8 as *const libc::c_char,
        _ => s = b"Unknown Gametype\x00" as *const u8 as *const libc::c_char,
    }
    UI_DrawProportionalString(
        320 as i32,
        y,
        s,
        0x1 as i32 | 0x10 as i32 | 0x800 as i32,
        colorWhite.as_mut_ptr(),
    );
    y += 27 as i32;
    value = atoi(Info_ValueForKey(
        info,
        b"timelimit\x00" as *const u8 as *const libc::c_char,
    ));
    if value != 0 {
        UI_DrawProportionalString(
            320 as i32,
            y,
            va(
                b"timelimit %i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                value,
            ),
            0x1 as i32 | 0x10 as i32 | 0x800 as i32,
            colorWhite.as_mut_ptr(),
        );
        y += 27 as i32
    }
    if (cgs.gametype as u32) < GT_CTF as i32 as u32 {
        value = atoi(Info_ValueForKey(
            info,
            b"fraglimit\x00" as *const u8 as *const libc::c_char,
        ));
        if value != 0 {
            UI_DrawProportionalString(
                320 as i32,
                y,
                va(
                    b"fraglimit %i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    value,
                ),
                0x1 as i32 | 0x10 as i32 | 0x800 as i32,
                colorWhite.as_mut_ptr(),
            );
            y += 27 as i32
        }
    }
    if cgs.gametype as u32 >= GT_CTF as i32 as u32 {
        value = atoi(Info_ValueForKey(
            info,
            b"capturelimit\x00" as *const u8 as *const libc::c_char,
        ));
        if value != 0 {
            UI_DrawProportionalString(
                320 as i32,
                y,
                va(
                    b"capturelimit %i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    value,
                ),
                0x1 as i32 | 0x10 as i32 | 0x800 as i32,
                colorWhite.as_mut_ptr(),
            );
        }
    };
}

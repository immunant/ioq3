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
pub use crate::bg_public_h::TEAM_BLUE;
pub use crate::bg_public_h::TEAM_FREE;
pub use crate::bg_public_h::TEAM_NUM_TEAMS;
pub use crate::bg_public_h::TEAM_RED;
pub use crate::bg_public_h::TEAM_SPECTATOR;
pub use crate::g_public_h::entityShared_t;
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

pub use crate::be_aas_h::aas_entityinfo_s;
pub use crate::be_aas_h::aas_entityinfo_t;
pub use crate::be_ai_goal_h::bot_goal_s;
pub use crate::be_ai_goal_h::bot_goal_t;
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
pub use crate::src::game::ai_chat::stdlib_h::atoi;

pub use crate::src::game::ai_main::bot_activategoal_s;
pub use crate::src::game::ai_main::bot_activategoal_t;
pub use crate::src::game::ai_main::bot_state_s;
pub use crate::src::game::ai_main::bot_state_t;
pub use crate::src::game::ai_main::bot_waypoint_s;
pub use crate::src::game::ai_main::bot_waypoint_t;
pub use crate::src::game::ai_main::floattime;
pub use crate::src::game::ai_main::BotAI_BotInitialChat;
pub use crate::src::game::ai_main::BotAI_GetClientState;
pub use crate::src::game::ai_main::BotAI_Trace;
pub use crate::src::game::ai_main::BotEntityInfo;
pub use crate::src::game::g_main::g_entities;
pub use crate::src::game::g_main::level;
pub use crate::src::game::g_syscalls::trap_AAS_PresenceTypeBoundingBox;
pub use crate::src::game::g_syscalls::trap_BotEnterChat;
pub use crate::src::game::g_syscalls::trap_BotNumInitialChats;
pub use crate::src::game::g_syscalls::trap_Characteristic_BFloat;
pub use crate::src::game::g_syscalls::trap_GetConfigstring;
pub use crate::src::game::g_syscalls::trap_GetServerinfo;
pub use crate::src::game::g_syscalls::trap_PointContents;

pub use ::libc::rand;

pub use ::libc::strtol;
/*
==================
BotNumActivePlayers
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotNumActivePlayers() -> i32 {
    let mut i: i32 = 0;
    let mut num: i32 = 0;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    num = 0 as i32;
    i = 0 as i32;
    while i < level.maxclients {
        trap_GetConfigstring(
            32 as i32 + 256 as i32 + 256 as i32 + i,
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as i32,
        );
        //if no config string or no name
        if !(crate::stdlib::strlen(buf.as_mut_ptr()) == 0
            || crate::stdlib::strlen(Info_ValueForKey(
                buf.as_mut_ptr(),
                b"n\x00" as *const u8 as *const libc::c_char,
            )) == 0)
        {
            //skip spectators
            if !(atoi(Info_ValueForKey(
                buf.as_mut_ptr(),
                b"t\x00" as *const u8 as *const libc::c_char,
            )) == TEAM_SPECTATOR as i32)
            {
                //
                num += 1
            }
        }
        i += 1
    }
    return num;
}
/*
==================
BotIsFirstInRankings
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotIsFirstInRankings(mut bs: *mut bot_state_t) -> i32 {
    let mut i: i32 = 0;
    let mut score: i32 = 0;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut ps: playerState_t = playerState_t {
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
    score = (*bs).cur_ps.persistant[PERS_SCORE as i32 as usize];
    i = 0 as i32;
    while i < level.maxclients {
        trap_GetConfigstring(
            32 as i32 + 256 as i32 + 256 as i32 + i,
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as i32,
        );
        //if no config string or no name
        if !(crate::stdlib::strlen(buf.as_mut_ptr()) == 0
            || crate::stdlib::strlen(Info_ValueForKey(
                buf.as_mut_ptr(),
                b"n\x00" as *const u8 as *const libc::c_char,
            )) == 0)
        {
            //skip spectators
            if !(atoi(Info_ValueForKey(
                buf.as_mut_ptr(),
                b"t\x00" as *const u8 as *const libc::c_char,
            )) == TEAM_SPECTATOR as i32)
            {
                //
                if BotAI_GetClientState(i, &mut ps as *mut _ as *mut playerState_s) != 0
                    && score < ps.persistant[PERS_SCORE as i32 as usize]
                {
                    return qfalse as i32;
                }
            }
        }
        i += 1
    }
    return qtrue as i32;
}
/*
==================
BotIsLastInRankings
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotIsLastInRankings(mut bs: *mut bot_state_t) -> i32 {
    let mut i: i32 = 0;
    let mut score: i32 = 0;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut ps: playerState_t = playerState_t {
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
    score = (*bs).cur_ps.persistant[PERS_SCORE as i32 as usize];
    i = 0 as i32;
    while i < level.maxclients {
        trap_GetConfigstring(
            32 as i32 + 256 as i32 + 256 as i32 + i,
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as i32,
        );
        //if no config string or no name
        if !(crate::stdlib::strlen(buf.as_mut_ptr()) == 0
            || crate::stdlib::strlen(Info_ValueForKey(
                buf.as_mut_ptr(),
                b"n\x00" as *const u8 as *const libc::c_char,
            )) == 0)
        {
            //skip spectators
            if !(atoi(Info_ValueForKey(
                buf.as_mut_ptr(),
                b"t\x00" as *const u8 as *const libc::c_char,
            )) == TEAM_SPECTATOR as i32)
            {
                //
                if BotAI_GetClientState(i, &mut ps as *mut _ as *mut playerState_s) != 0
                    && score > ps.persistant[PERS_SCORE as i32 as usize]
                {
                    return qfalse as i32;
                }
            }
        }
        i += 1
    }
    return qtrue as i32;
}
/*
==================
BotFirstClientInRankings
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotFirstClientInRankings() -> *mut libc::c_char {
    let mut i: i32 = 0;
    let mut bestscore: i32 = 0;
    let mut bestclient: i32 = 0;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    static mut name: [libc::c_char; 32] = [0; 32];
    let mut ps: playerState_t = playerState_t {
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
    bestscore = -(999999 as i32);
    bestclient = 0 as i32;
    i = 0 as i32;
    while i < level.maxclients {
        trap_GetConfigstring(
            32 as i32 + 256 as i32 + 256 as i32 + i,
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as i32,
        );
        //if no config string or no name
        if !(crate::stdlib::strlen(buf.as_mut_ptr()) == 0
            || crate::stdlib::strlen(Info_ValueForKey(
                buf.as_mut_ptr(),
                b"n\x00" as *const u8 as *const libc::c_char,
            )) == 0)
        {
            //skip spectators
            if !(atoi(Info_ValueForKey(
                buf.as_mut_ptr(),
                b"t\x00" as *const u8 as *const libc::c_char,
            )) == TEAM_SPECTATOR as i32)
            {
                //
                if BotAI_GetClientState(i, &mut ps as *mut _ as *mut playerState_s) != 0
                    && ps.persistant[PERS_SCORE as i32 as usize] > bestscore
                {
                    bestscore = ps.persistant[PERS_SCORE as i32 as usize];
                    bestclient = i
                }
            }
        }
        i += 1
    }
    crate::src::game::ai_dmq3::EasyClientName(bestclient, name.as_mut_ptr(), 32 as i32);
    return name.as_mut_ptr();
}
/*
==================
BotLastClientInRankings
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotLastClientInRankings() -> *mut libc::c_char {
    let mut i: i32 = 0;
    let mut worstscore: i32 = 0;
    let mut bestclient: i32 = 0;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    static mut name: [libc::c_char; 32] = [0; 32];
    let mut ps: playerState_t = playerState_t {
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
    worstscore = 999999 as i32;
    bestclient = 0 as i32;
    i = 0 as i32;
    while i < level.maxclients {
        trap_GetConfigstring(
            32 as i32 + 256 as i32 + 256 as i32 + i,
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as i32,
        );
        //if no config string or no name
        if !(crate::stdlib::strlen(buf.as_mut_ptr()) == 0
            || crate::stdlib::strlen(Info_ValueForKey(
                buf.as_mut_ptr(),
                b"n\x00" as *const u8 as *const libc::c_char,
            )) == 0)
        {
            //skip spectators
            if !(atoi(Info_ValueForKey(
                buf.as_mut_ptr(),
                b"t\x00" as *const u8 as *const libc::c_char,
            )) == TEAM_SPECTATOR as i32)
            {
                //
                if BotAI_GetClientState(i, &mut ps as *mut _ as *mut playerState_s) != 0
                    && ps.persistant[PERS_SCORE as i32 as usize] < worstscore
                {
                    worstscore = ps.persistant[PERS_SCORE as i32 as usize];
                    bestclient = i
                }
            }
        }
        i += 1
    }
    crate::src::game::ai_dmq3::EasyClientName(bestclient, name.as_mut_ptr(), 32 as i32);
    return name.as_mut_ptr();
}
/*
==================
BotRandomOpponentName
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotRandomOpponentName(mut bs: *mut bot_state_t) -> *mut libc::c_char {
    let mut i: i32 = 0;
    let mut count: i32 = 0;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut opponents: [i32; 64] = [0; 64];
    let mut numopponents: i32 = 0;
    static mut name: [libc::c_char; 32] = [0; 32];
    numopponents = 0 as i32;
    opponents[0 as i32 as usize] = 0 as i32;
    i = 0 as i32;
    while i < level.maxclients {
        if !(i == (*bs).client) {
            //
            trap_GetConfigstring(
                32 as i32 + 256 as i32 + 256 as i32 + i,
                buf.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as i32,
            );
            //if no config string or no name
            if !(crate::stdlib::strlen(buf.as_mut_ptr()) == 0
                || crate::stdlib::strlen(Info_ValueForKey(
                    buf.as_mut_ptr(),
                    b"n\x00" as *const u8 as *const libc::c_char,
                )) == 0)
            {
                //skip spectators
                if !(atoi(Info_ValueForKey(
                    buf.as_mut_ptr(),
                    b"t\x00" as *const u8 as *const libc::c_char,
                )) == TEAM_SPECTATOR as i32)
                {
                    //skip team mates
                    if !(crate::src::game::ai_dmq3::BotSameTeam(bs as *mut bot_state_s, i) != 0) {
                        //
                        opponents[numopponents as usize] = i;
                        numopponents += 1
                    }
                }
            }
        }
        i += 1
    }
    count = ((rand() & 0x7fff as i32) as f32 / 0x7fff as i32 as f32 * numopponents as f32) as i32;
    i = 0 as i32;
    while i < numopponents {
        count -= 1;
        if count <= 0 as i32 {
            crate::src::game::ai_dmq3::EasyClientName(
                opponents[i as usize],
                name.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong as i32,
            );
            return name.as_mut_ptr();
        }
        i += 1
    }
    crate::src::game::ai_dmq3::EasyClientName(
        opponents[0 as i32 as usize],
        name.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong as i32,
    );
    return name.as_mut_ptr();
}
/*
==================
BotMapTitle
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotMapTitle() -> *mut libc::c_char {
    let mut info: [libc::c_char; 1024] = [0; 1024];
    static mut mapname: [libc::c_char; 128] = [0; 128];
    trap_GetServerinfo(
        info.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as i32,
    );
    crate::stdlib::strncpy(
        mapname.as_mut_ptr(),
        Info_ValueForKey(
            info.as_mut_ptr(),
            b"mapname\x00" as *const u8 as *const libc::c_char,
        ),
        (::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong)
            .wrapping_sub(1 as i32 as libc::c_ulong),
    );
    mapname[(::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong)
        .wrapping_sub(1 as i32 as libc::c_ulong) as usize] = '\u{0}' as i32 as libc::c_char;
    return mapname.as_mut_ptr();
}
/*
==================
BotWeaponNameForMeansOfDeath
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotWeaponNameForMeansOfDeath(mut mod_0: i32) -> *mut libc::c_char {
    match mod_0 {
        1 => return b"Shotgun\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        2 => return b"Gauntlet\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        3 => return b"Machinegun\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        4 | 5 => {
            return b"Grenade Launcher\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
        }
        6 | 7 => {
            return b"Rocket Launcher\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
        }
        8 | 9 => return b"Plasmagun\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        10 => return b"Railgun\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        11 => return b"Lightning Gun\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        12 | 13 => return b"BFG10K\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        23 => return b"Grapple\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        _ => {
            return b"[unknown weapon]\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
        }
    };
}
/*
==================
BotRandomWeaponName
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotRandomWeaponName() -> *mut libc::c_char {
    let mut rnd: i32 = 0;
    rnd = (((rand() & 0x7fff as i32) as f32 / 0x7fff as i32 as f32) as f64 * 8.9f64) as i32;
    match rnd {
        0 => return b"Gauntlet\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1 => return b"Shotgun\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        2 => return b"Machinegun\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        3 => {
            return b"Grenade Launcher\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
        }
        4 => {
            return b"Rocket Launcher\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
        }
        5 => return b"Plasmagun\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        6 => return b"Railgun\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        7 => return b"Lightning Gun\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        _ => return b"BFG10K\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    };
}
/*
==================
BotVisibleEnemies
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotVisibleEnemies(mut bs: *mut bot_state_t) -> i32 {
    let mut vis: f32 = 0.;
    let mut i: i32 = 0;
    let mut entinfo: aas_entityinfo_t = aas_entityinfo_t {
        valid: 0,
        type_0: 0,
        flags: 0,
        ltime: 0.,
        update_time: 0.,
        number: 0,
        origin: [0.; 3],
        angles: [0.; 3],
        old_origin: [0.; 3],
        lastvisorigin: [0.; 3],
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
    i = 0 as i32;
    while i < 64 as i32 {
        if !(i == (*bs).client) {
            //
            BotEntityInfo(i, &mut entinfo as *mut _ as *mut aas_entityinfo_s);
            //
            if !(entinfo.valid == 0) {
                //if the enemy isn't dead and the enemy isn't the bot self
                if !(crate::src::game::ai_dmq3::EntityIsDead(
                    &mut entinfo as *mut _ as *mut aas_entityinfo_s,
                ) as u32
                    != 0
                    || entinfo.number == (*bs).entitynum)
                {
                    //if the enemy is invisible and not shooting
                    if !(crate::src::game::ai_dmq3::EntityIsInvisible(
                        &mut entinfo as *mut _ as *mut aas_entityinfo_s,
                    ) as u32
                        != 0
                        && crate::src::game::ai_dmq3::EntityIsShooting(
                            &mut entinfo as *mut _ as *mut aas_entityinfo_s,
                        ) as u64
                            == 0)
                    {
                        //if on the same team
                        if !(crate::src::game::ai_dmq3::BotSameTeam(bs as *mut bot_state_s, i) != 0)
                        {
                            //check if the enemy is visible
                            vis = crate::src::game::ai_dmq3::BotEntityVisible(
                                (*bs).entitynum,
                                (*bs).eye.as_mut_ptr(),
                                (*bs).viewangles.as_mut_ptr(),
                                360 as i32 as f32,
                                i,
                            );
                            if vis > 0 as i32 as f32 {
                                return qtrue as i32;
                            }
                        }
                    }
                }
            }
        }
        i += 1
    }
    return qfalse as i32;
}
// returns true if the bot can chat at the current position
/*
==================
BotValidChatPosition
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotValidChatPosition(mut bs: *mut bot_state_t) -> i32 {
    let mut point: vec3_t = [0.; 3];
    let mut start: vec3_t = [0.; 3];
    let mut end: vec3_t = [0.; 3];
    let mut mins: vec3_t = [0.; 3];
    let mut maxs: vec3_t = [0.; 3];
    let mut trace: bsp_trace_t = bsp_trace_t {
        allsolid: qfalse,
        startsolid: qfalse,
        fraction: 0.,
        endpos: [0.; 3],
        plane: cplane_t {
            normal: [0.; 3],
            dist: 0.,
            type_0: 0,
            signbits: 0,
            pad: [0; 2],
        },
        exp_dist: 0.,
        sidenum: 0,
        surface: bsp_surface_t {
            name: [0; 16],
            flags: 0,
            value: 0,
        },
        contents: 0,
        ent: 0,
    };
    //if the bot is dead all positions are valid
    if crate::src::game::ai_dmq3::BotIsDead(bs as *mut bot_state_s) as u64 != 0 {
        return qtrue as i32;
    }
    //never start chatting with a powerup
    if (*bs).inventory[35 as i32 as usize] != 0
        || (*bs).inventory[36 as i32 as usize] != 0
        || (*bs).inventory[37 as i32 as usize] != 0
        || (*bs).inventory[38 as i32 as usize] != 0
        || (*bs).inventory[39 as i32 as usize] != 0
        || (*bs).inventory[40 as i32 as usize] != 0
    {
        return qfalse as i32;
    }
    //must be on the ground
    //if (bs->cur_ps.groundEntityNum != ENTITYNUM_NONE) return qfalse;
    //do not chat if in lava or slime
    point[0 as i32 as usize] = (*bs).origin[0 as i32 as usize];
    point[1 as i32 as usize] = (*bs).origin[1 as i32 as usize];
    point[2 as i32 as usize] = (*bs).origin[2 as i32 as usize];
    point[2 as i32 as usize] -= 24 as i32 as f32;
    if trap_PointContents(point.as_mut_ptr() as *const vec_t, (*bs).entitynum)
        & (8 as i32 | 16 as i32)
        != 0
    {
        return qfalse as i32;
    }
    //do not chat if under water
    point[0 as i32 as usize] = (*bs).origin[0 as i32 as usize];
    point[1 as i32 as usize] = (*bs).origin[1 as i32 as usize];
    point[2 as i32 as usize] = (*bs).origin[2 as i32 as usize];
    point[2 as i32 as usize] += 32 as i32 as f32;
    if trap_PointContents(point.as_mut_ptr() as *const vec_t, (*bs).entitynum)
        & (32 as i32 | 8 as i32 | 16 as i32)
        != 0
    {
        return qfalse as i32;
    }
    //must be standing on the world entity
    start[0 as i32 as usize] = (*bs).origin[0 as i32 as usize];
    start[1 as i32 as usize] = (*bs).origin[1 as i32 as usize];
    start[2 as i32 as usize] = (*bs).origin[2 as i32 as usize];
    end[0 as i32 as usize] = (*bs).origin[0 as i32 as usize];
    end[1 as i32 as usize] = (*bs).origin[1 as i32 as usize];
    end[2 as i32 as usize] = (*bs).origin[2 as i32 as usize];
    start[2 as i32 as usize] += 1 as i32 as f32;
    end[2 as i32 as usize] -= 10 as i32 as f32;
    trap_AAS_PresenceTypeBoundingBox(4 as i32, mins.as_mut_ptr(), maxs.as_mut_ptr());
    BotAI_Trace(
        &mut trace as *mut _ as *mut bsp_trace_s,
        start.as_mut_ptr(),
        mins.as_mut_ptr(),
        maxs.as_mut_ptr(),
        end.as_mut_ptr(),
        (*bs).client,
        1 as i32,
    );
    if trace.ent != ((1 as i32) << 10 as i32) - 2 as i32 {
        return qfalse as i32;
    }
    //the bot is in a position where it can chat
    return qtrue as i32;
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
 * name:		ai_chat.h
 *
 * desc:		Quake3 bot AI
 *
 * $Archive: /source/code/botai/ai_chat.c $
 *
 *****************************************************************************/
//
/*
==================
BotChat_EnterGame
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotChat_EnterGame(mut bs: *mut bot_state_t) -> i32 {
    let mut name: [libc::c_char; 32] = [0; 32];
    let mut rnd: f32 = 0.;
    if crate::src::game::ai_dmq3::bot_nochat.integer != 0 {
        return qfalse as i32;
    }
    if (*bs).lastchat_time > floattime - 25 as i32 as f32 {
        return qfalse as i32;
    }
    //don't chat in teamplay
    if crate::src::game::ai_dmq3::TeamPlayIsOn() != 0 {
        return qfalse as i32;
    }
    // don't chat in tournament mode
    if crate::src::game::ai_dmq3::gametype == GT_TOURNAMENT as i32 {
        return qfalse as i32;
    }
    rnd = trap_Characteristic_BFloat((*bs).character, 27 as i32, 0 as i32 as f32, 1 as i32 as f32);
    if crate::src::game::ai_dmq3::bot_fastchat.integer == 0 {
        if (rand() & 0x7fff as i32) as f32 / 0x7fff as i32 as f32 > rnd {
            return qfalse as i32;
        }
    }
    if BotNumActivePlayers() <= 1 as i32 {
        return qfalse as i32;
    }
    if BotValidChatPosition(bs) == 0 {
        return qfalse as i32;
    }
    BotAI_BotInitialChat(
        bs as *mut bot_state_s,
        b"game_enter\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        crate::src::game::ai_dmq3::EasyClientName((*bs).client, name.as_mut_ptr(), 32 as i32),
        BotRandomOpponentName(bs),
        b"[invalid var]\x00" as *const u8 as *const libc::c_char,
        b"[invalid var]\x00" as *const u8 as *const libc::c_char,
        BotMapTitle(),
        0 as *mut libc::c_void,
    );
    (*bs).lastchat_time = floattime;
    (*bs).chatto = 0 as i32;
    return qtrue as i32;
}
//
/*
==================
BotChat_ExitGame
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotChat_ExitGame(mut bs: *mut bot_state_t) -> i32 {
    let mut name: [libc::c_char; 32] = [0; 32];
    let mut rnd: f32 = 0.;
    if crate::src::game::ai_dmq3::bot_nochat.integer != 0 {
        return qfalse as i32;
    }
    if (*bs).lastchat_time > floattime - 25 as i32 as f32 {
        return qfalse as i32;
    }
    //don't chat in teamplay
    if crate::src::game::ai_dmq3::TeamPlayIsOn() != 0 {
        return qfalse as i32;
    }
    // don't chat in tournament mode
    if crate::src::game::ai_dmq3::gametype == GT_TOURNAMENT as i32 {
        return qfalse as i32;
    }
    rnd = trap_Characteristic_BFloat((*bs).character, 27 as i32, 0 as i32 as f32, 1 as i32 as f32);
    if crate::src::game::ai_dmq3::bot_fastchat.integer == 0 {
        if (rand() & 0x7fff as i32) as f32 / 0x7fff as i32 as f32 > rnd {
            return qfalse as i32;
        }
    }
    if BotNumActivePlayers() <= 1 as i32 {
        return qfalse as i32;
    }
    //
    BotAI_BotInitialChat(
        bs as *mut bot_state_s,
        b"game_exit\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        crate::src::game::ai_dmq3::EasyClientName((*bs).client, name.as_mut_ptr(), 32 as i32),
        BotRandomOpponentName(bs),
        b"[invalid var]\x00" as *const u8 as *const libc::c_char,
        b"[invalid var]\x00" as *const u8 as *const libc::c_char,
        BotMapTitle(),
        0 as *mut libc::c_void,
    );
    (*bs).lastchat_time = floattime;
    (*bs).chatto = 0 as i32;
    return qtrue as i32;
}
//
/*
==================
BotChat_StartLevel
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotChat_StartLevel(mut bs: *mut bot_state_t) -> i32 {
    let mut name: [libc::c_char; 32] = [0; 32];
    let mut rnd: f32 = 0.;
    if crate::src::game::ai_dmq3::bot_nochat.integer != 0 {
        return qfalse as i32;
    }
    if crate::src::game::ai_dmq3::BotIsObserver(bs as *mut bot_state_s) as u64 != 0 {
        return qfalse as i32;
    }
    if (*bs).lastchat_time > floattime - 25 as i32 as f32 {
        return qfalse as i32;
    }
    //don't chat in teamplay
    if crate::src::game::ai_dmq3::TeamPlayIsOn() != 0 {
        return qfalse as i32;
    }
    // don't chat in tournament mode
    if crate::src::game::ai_dmq3::gametype == GT_TOURNAMENT as i32 {
        return qfalse as i32;
    }
    rnd = trap_Characteristic_BFloat((*bs).character, 26 as i32, 0 as i32 as f32, 1 as i32 as f32);
    if crate::src::game::ai_dmq3::bot_fastchat.integer == 0 {
        if (rand() & 0x7fff as i32) as f32 / 0x7fff as i32 as f32 > rnd {
            return qfalse as i32;
        }
    }
    if BotNumActivePlayers() <= 1 as i32 {
        return qfalse as i32;
    }
    BotAI_BotInitialChat(
        bs as *mut bot_state_s,
        b"level_start\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        crate::src::game::ai_dmq3::EasyClientName((*bs).client, name.as_mut_ptr(), 32 as i32),
        0 as *mut libc::c_void,
    );
    (*bs).lastchat_time = floattime;
    (*bs).chatto = 0 as i32;
    return qtrue as i32;
}
//
/*
==================
BotChat_EndLevel
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotChat_EndLevel(mut bs: *mut bot_state_t) -> i32 {
    let mut name: [libc::c_char; 32] = [0; 32];
    let mut rnd: f32 = 0.;
    if crate::src::game::ai_dmq3::bot_nochat.integer != 0 {
        return qfalse as i32;
    }
    if crate::src::game::ai_dmq3::BotIsObserver(bs as *mut bot_state_s) as u64 != 0 {
        return qfalse as i32;
    }
    if (*bs).lastchat_time > floattime - 25 as i32 as f32 {
        return qfalse as i32;
    }
    // teamplay
    if crate::src::game::ai_dmq3::TeamPlayIsOn() != 0 {
        return qtrue as i32;
    }
    // don't chat in tournament mode
    if crate::src::game::ai_dmq3::gametype == GT_TOURNAMENT as i32 {
        return qfalse as i32;
    }
    rnd = trap_Characteristic_BFloat((*bs).character, 26 as i32, 0 as i32 as f32, 1 as i32 as f32);
    if crate::src::game::ai_dmq3::bot_fastchat.integer == 0 {
        if (rand() & 0x7fff as i32) as f32 / 0x7fff as i32 as f32 > rnd {
            return qfalse as i32;
        }
    }
    if BotNumActivePlayers() <= 1 as i32 {
        return qfalse as i32;
    }
    //
    if BotIsFirstInRankings(bs) != 0 {
        BotAI_BotInitialChat(
            bs as *mut bot_state_s,
            b"level_end_victory\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            crate::src::game::ai_dmq3::EasyClientName((*bs).client, name.as_mut_ptr(), 32 as i32),
            BotRandomOpponentName(bs),
            b"[invalid var]\x00" as *const u8 as *const libc::c_char,
            BotLastClientInRankings(),
            BotMapTitle(),
            0 as *mut libc::c_void,
        );
    } else if BotIsLastInRankings(bs) != 0 {
        BotAI_BotInitialChat(
            bs as *mut bot_state_s,
            b"level_end_lose\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            crate::src::game::ai_dmq3::EasyClientName((*bs).client, name.as_mut_ptr(), 32 as i32),
            BotRandomOpponentName(bs),
            BotFirstClientInRankings(),
            b"[invalid var]\x00" as *const u8 as *const libc::c_char,
            BotMapTitle(),
            0 as *mut libc::c_void,
        );
    } else {
        BotAI_BotInitialChat(
            bs as *mut bot_state_s,
            b"level_end\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            crate::src::game::ai_dmq3::EasyClientName((*bs).client, name.as_mut_ptr(), 32 as i32),
            BotRandomOpponentName(bs),
            BotFirstClientInRankings(),
            BotLastClientInRankings(),
            BotMapTitle(),
            0 as *mut libc::c_void,
        );
    }
    (*bs).lastchat_time = floattime;
    (*bs).chatto = 0 as i32;
    return qtrue as i32;
}
//
/*
==================
BotChat_Death
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotChat_Death(mut bs: *mut bot_state_t) -> i32 {
    let mut name: [libc::c_char; 32] = [0; 32];
    let mut rnd: f32 = 0.;
    if crate::src::game::ai_dmq3::bot_nochat.integer != 0 {
        return qfalse as i32;
    }
    if (*bs).lastchat_time > floattime - 25 as i32 as f32 {
        return qfalse as i32;
    }
    rnd = trap_Characteristic_BFloat((*bs).character, 29 as i32, 0 as i32 as f32, 1 as i32 as f32);
    // don't chat in tournament mode
    if crate::src::game::ai_dmq3::gametype == GT_TOURNAMENT as i32 {
        return qfalse as i32;
    }
    //if fast chatting is off
    if crate::src::game::ai_dmq3::bot_fastchat.integer == 0 {
        if (rand() & 0x7fff as i32) as f32 / 0x7fff as i32 as f32 > rnd {
            return qfalse as i32;
        }
    }
    if BotNumActivePlayers() <= 1 as i32 {
        return qfalse as i32;
    }
    //
    if (*bs).lastkilledby >= 0 as i32 && (*bs).lastkilledby < 64 as i32 {
        crate::src::game::ai_dmq3::EasyClientName((*bs).lastkilledby, name.as_mut_ptr(), 32 as i32);
    } else {
        libc::strcpy(
            name.as_mut_ptr(),
            b"[world]\x00" as *const u8 as *const libc::c_char,
        );
    }
    //
    if crate::src::game::ai_dmq3::TeamPlayIsOn() != 0
        && crate::src::game::ai_dmq3::BotSameTeam(bs as *mut bot_state_s, (*bs).lastkilledby) != 0
    {
        if (*bs).lastkilledby == (*bs).client {
            return qfalse as i32;
        }
        BotAI_BotInitialChat(
            bs as *mut bot_state_s,
            b"death_teammate\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            name.as_mut_ptr(),
            0 as *mut libc::c_void,
        );
        (*bs).chatto = 1 as i32
    } else {
        //teamplay
        if crate::src::game::ai_dmq3::TeamPlayIsOn() != 0 {
            return qtrue as i32;
        }
        //
        if (*bs).botdeathtype == MOD_WATER as i32 {
            BotAI_BotInitialChat(
                bs as *mut bot_state_s,
                b"death_drown\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                BotRandomOpponentName(bs),
                0 as *mut libc::c_void,
            );
        } else if (*bs).botdeathtype == MOD_SLIME as i32 {
            BotAI_BotInitialChat(
                bs as *mut bot_state_s,
                b"death_slime\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                BotRandomOpponentName(bs),
                0 as *mut libc::c_void,
            );
        } else if (*bs).botdeathtype == MOD_LAVA as i32 {
            BotAI_BotInitialChat(
                bs as *mut bot_state_s,
                b"death_lava\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                BotRandomOpponentName(bs),
                0 as *mut libc::c_void,
            );
        } else if (*bs).botdeathtype == MOD_FALLING as i32 {
            BotAI_BotInitialChat(
                bs as *mut bot_state_s,
                b"death_cratered\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                BotRandomOpponentName(bs),
                0 as *mut libc::c_void,
            );
        } else if (*bs).botsuicide != 0
            || (*bs).botdeathtype == MOD_CRUSH as i32
            || (*bs).botdeathtype == MOD_SUICIDE as i32
            || (*bs).botdeathtype == MOD_TARGET_LASER as i32
            || (*bs).botdeathtype == MOD_TRIGGER_HURT as i32
            || (*bs).botdeathtype == MOD_UNKNOWN as i32
        {
            BotAI_BotInitialChat(
                bs as *mut bot_state_s,
                b"death_suicide\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                BotRandomOpponentName(bs),
                0 as *mut libc::c_void,
            );
        } else if (*bs).botdeathtype == MOD_TELEFRAG as i32 {
            BotAI_BotInitialChat(
                bs as *mut bot_state_s,
                b"death_telefrag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                name.as_mut_ptr(),
                0 as *mut libc::c_void,
            );
        } else if ((*bs).botdeathtype == MOD_GAUNTLET as i32
            || (*bs).botdeathtype == MOD_RAILGUN as i32
            || (*bs).botdeathtype == MOD_BFG as i32
            || (*bs).botdeathtype == MOD_BFG_SPLASH as i32)
            && (((rand() & 0x7fff as i32) as f32 / 0x7fff as i32 as f32) as f64) < 0.5f64
        {
            if (*bs).botdeathtype == MOD_GAUNTLET as i32 {
                BotAI_BotInitialChat(
                    bs as *mut bot_state_s,
                    b"death_gauntlet\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    name.as_mut_ptr(),
                    BotWeaponNameForMeansOfDeath((*bs).botdeathtype),
                    0 as *mut libc::c_void,
                );
            } else if (*bs).botdeathtype == MOD_RAILGUN as i32 {
                BotAI_BotInitialChat(
                    bs as *mut bot_state_s,
                    b"death_rail\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    name.as_mut_ptr(),
                    BotWeaponNameForMeansOfDeath((*bs).botdeathtype),
                    0 as *mut libc::c_void,
                );
            } else {
                BotAI_BotInitialChat(
                    bs as *mut bot_state_s,
                    b"death_bfg\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    name.as_mut_ptr(),
                    BotWeaponNameForMeansOfDeath((*bs).botdeathtype),
                    0 as *mut libc::c_void,
                );
            }
        } else if ((rand() & 0x7fff as i32) as f32 / 0x7fff as i32 as f32)
            < trap_Characteristic_BFloat(
                (*bs).character,
                24 as i32,
                0 as i32 as f32,
                1 as i32 as f32,
            )
        {
            BotAI_BotInitialChat(
                bs as *mut bot_state_s,
                b"death_insult\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                name.as_mut_ptr(),
                BotWeaponNameForMeansOfDeath((*bs).botdeathtype),
                0 as *mut libc::c_void,
            );
        } else {
            BotAI_BotInitialChat(
                bs as *mut bot_state_s,
                b"death_praise\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                name.as_mut_ptr(),
                BotWeaponNameForMeansOfDeath((*bs).botdeathtype),
                0 as *mut libc::c_void,
            );
        }
        (*bs).chatto = 0 as i32
    }
    (*bs).lastchat_time = floattime;
    return qtrue as i32;
}
//choose between insult and praise
//
/*
==================
BotChat_Kill
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotChat_Kill(mut bs: *mut bot_state_t) -> i32 {
    let mut name: [libc::c_char; 32] = [0; 32];
    let mut rnd: f32 = 0.;
    if crate::src::game::ai_dmq3::bot_nochat.integer != 0 {
        return qfalse as i32;
    }
    if (*bs).lastchat_time > floattime - 25 as i32 as f32 {
        return qfalse as i32;
    }
    rnd = trap_Characteristic_BFloat((*bs).character, 28 as i32, 0 as i32 as f32, 1 as i32 as f32);
    // don't chat in tournament mode
    if crate::src::game::ai_dmq3::gametype == GT_TOURNAMENT as i32 {
        return qfalse as i32;
    }
    //if fast chat is off
    if crate::src::game::ai_dmq3::bot_fastchat.integer == 0 {
        if (rand() & 0x7fff as i32) as f32 / 0x7fff as i32 as f32 > rnd {
            return qfalse as i32;
        }
    }
    if (*bs).lastkilledplayer == (*bs).client {
        return qfalse as i32;
    }
    if BotNumActivePlayers() <= 1 as i32 {
        return qfalse as i32;
    }
    if BotValidChatPosition(bs) == 0 {
        return qfalse as i32;
    }
    //
    if BotVisibleEnemies(bs) != 0 {
        return qfalse as i32;
    }
    //
    crate::src::game::ai_dmq3::EasyClientName((*bs).lastkilledplayer, name.as_mut_ptr(), 32 as i32);
    //
    (*bs).chatto = 0 as i32;
    if crate::src::game::ai_dmq3::TeamPlayIsOn() != 0
        && crate::src::game::ai_dmq3::BotSameTeam(bs as *mut bot_state_s, (*bs).lastkilledplayer)
            != 0
    {
        BotAI_BotInitialChat(
            bs as *mut bot_state_s,
            b"kill_teammate\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            name.as_mut_ptr(),
            0 as *mut libc::c_void,
        );
        (*bs).chatto = 1 as i32
    } else {
        //don't chat in teamplay
        if crate::src::game::ai_dmq3::TeamPlayIsOn() != 0 {
            return qfalse as i32;
            // don't wait
        }
        //
        if (*bs).enemydeathtype == MOD_GAUNTLET as i32 {
            BotAI_BotInitialChat(
                bs as *mut bot_state_s,
                b"kill_gauntlet\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                name.as_mut_ptr(),
                0 as *mut libc::c_void,
            );
        } else if (*bs).enemydeathtype == MOD_RAILGUN as i32 {
            BotAI_BotInitialChat(
                bs as *mut bot_state_s,
                b"kill_rail\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                name.as_mut_ptr(),
                0 as *mut libc::c_void,
            );
        } else if (*bs).enemydeathtype == MOD_TELEFRAG as i32 {
            BotAI_BotInitialChat(
                bs as *mut bot_state_s,
                b"kill_telefrag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                name.as_mut_ptr(),
                0 as *mut libc::c_void,
            );
        } else if ((rand() & 0x7fff as i32) as f32 / 0x7fff as i32 as f32)
            < trap_Characteristic_BFloat(
                (*bs).character,
                24 as i32,
                0 as i32 as f32,
                1 as i32 as f32,
            )
        {
            BotAI_BotInitialChat(
                bs as *mut bot_state_s,
                b"kill_insult\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                name.as_mut_ptr(),
                0 as *mut libc::c_void,
            );
        } else {
            BotAI_BotInitialChat(
                bs as *mut bot_state_s,
                b"kill_praise\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                name.as_mut_ptr(),
                0 as *mut libc::c_void,
            );
        }
    }
    (*bs).lastchat_time = floattime;
    return qtrue as i32;
}
//choose between insult and praise
//
/*
==================
BotChat_EnemySuicide
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotChat_EnemySuicide(mut bs: *mut bot_state_t) -> i32 {
    let mut name: [libc::c_char; 32] = [0; 32];
    let mut rnd: f32 = 0.;
    if crate::src::game::ai_dmq3::bot_nochat.integer != 0 {
        return qfalse as i32;
    }
    if (*bs).lastchat_time > floattime - 25 as i32 as f32 {
        return qfalse as i32;
    }
    if BotNumActivePlayers() <= 1 as i32 {
        return qfalse as i32;
    }
    //
    rnd = trap_Characteristic_BFloat((*bs).character, 30 as i32, 0 as i32 as f32, 1 as i32 as f32);
    //don't chat in teamplay
    if crate::src::game::ai_dmq3::TeamPlayIsOn() != 0 {
        return qfalse as i32;
    }
    // don't chat in tournament mode
    if crate::src::game::ai_dmq3::gametype == GT_TOURNAMENT as i32 {
        return qfalse as i32;
    }
    //if fast chat is off
    if crate::src::game::ai_dmq3::bot_fastchat.integer == 0 {
        if (rand() & 0x7fff as i32) as f32 / 0x7fff as i32 as f32 > rnd {
            return qfalse as i32;
        }
    }
    if BotValidChatPosition(bs) == 0 {
        return qfalse as i32;
    }
    //
    if BotVisibleEnemies(bs) != 0 {
        return qfalse as i32;
    }
    //
    if (*bs).enemy >= 0 as i32 {
        crate::src::game::ai_dmq3::EasyClientName((*bs).enemy, name.as_mut_ptr(), 32 as i32);
    } else {
        libc::strcpy(
            name.as_mut_ptr(),
            b"\x00" as *const u8 as *const libc::c_char,
        );
    }
    BotAI_BotInitialChat(
        bs as *mut bot_state_s,
        b"enemy_suicide\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        name.as_mut_ptr(),
        0 as *mut libc::c_void,
    );
    (*bs).lastchat_time = floattime;
    (*bs).chatto = 0 as i32;
    return qtrue as i32;
}
//
/*
==================
BotChat_HitTalking
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotChat_HitTalking(mut bs: *mut bot_state_t) -> i32 {
    let mut name: [libc::c_char; 32] = [0; 32];
    let mut weap: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut lasthurt_client: i32 = 0;
    let mut rnd: f32 = 0.;
    if crate::src::game::ai_dmq3::bot_nochat.integer != 0 {
        return qfalse as i32;
    }
    if (*bs).lastchat_time > floattime - 25 as i32 as f32 {
        return qfalse as i32;
    }
    if BotNumActivePlayers() <= 1 as i32 {
        return qfalse as i32;
    }
    lasthurt_client = (*g_entities[(*bs).client as usize].client).lasthurt_client;
    if lasthurt_client == 0 {
        return qfalse as i32;
    }
    if lasthurt_client == (*bs).client {
        return qfalse as i32;
    }
    //
    if lasthurt_client < 0 as i32 || lasthurt_client >= 64 as i32 {
        return qfalse as i32;
    }
    //
    rnd = trap_Characteristic_BFloat((*bs).character, 31 as i32, 0 as i32 as f32, 1 as i32 as f32);
    //don't chat in teamplay
    if crate::src::game::ai_dmq3::TeamPlayIsOn() != 0 {
        return qfalse as i32;
    }
    // don't chat in tournament mode
    if crate::src::game::ai_dmq3::gametype == GT_TOURNAMENT as i32 {
        return qfalse as i32;
    }
    //if fast chat is off
    if crate::src::game::ai_dmq3::bot_fastchat.integer == 0 {
        if ((rand() & 0x7fff as i32) as f32 / 0x7fff as i32 as f32) as f64 > rnd as f64 * 0.5f64 {
            return qfalse as i32;
        }
    }
    if BotValidChatPosition(bs) == 0 {
        return qfalse as i32;
    }
    //
    crate::src::game::ai_dmq3::ClientName(
        (*g_entities[(*bs).client as usize].client).lasthurt_client,
        name.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong as i32,
    );
    weap = BotWeaponNameForMeansOfDeath((*g_entities[(*bs).client as usize].client).lasthurt_mod);
    //
    BotAI_BotInitialChat(
        bs as *mut bot_state_s,
        b"hit_talking\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        name.as_mut_ptr(),
        weap,
        0 as *mut libc::c_void,
    );
    (*bs).lastchat_time = floattime;
    (*bs).chatto = 0 as i32;
    return qtrue as i32;
}
//
/*
==================
BotChat_HitNoDeath
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotChat_HitNoDeath(mut bs: *mut bot_state_t) -> i32 {
    let mut name: [libc::c_char; 32] = [0; 32];
    let mut weap: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut rnd: f32 = 0.;
    let mut lasthurt_client: i32 = 0;
    let mut entinfo: aas_entityinfo_t = aas_entityinfo_t {
        valid: 0,
        type_0: 0,
        flags: 0,
        ltime: 0.,
        update_time: 0.,
        number: 0,
        origin: [0.; 3],
        angles: [0.; 3],
        old_origin: [0.; 3],
        lastvisorigin: [0.; 3],
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
    lasthurt_client = (*g_entities[(*bs).client as usize].client).lasthurt_client;
    if lasthurt_client == 0 {
        return qfalse as i32;
    }
    if lasthurt_client == (*bs).client {
        return qfalse as i32;
    }
    //
    if lasthurt_client < 0 as i32 || lasthurt_client >= 64 as i32 {
        return qfalse as i32;
    }
    //
    if crate::src::game::ai_dmq3::bot_nochat.integer != 0 {
        return qfalse as i32;
    }
    if (*bs).lastchat_time > floattime - 25 as i32 as f32 {
        return qfalse as i32;
    }
    if BotNumActivePlayers() <= 1 as i32 {
        return qfalse as i32;
    }
    rnd = trap_Characteristic_BFloat((*bs).character, 32 as i32, 0 as i32 as f32, 1 as i32 as f32);
    //don't chat in teamplay
    if crate::src::game::ai_dmq3::TeamPlayIsOn() != 0 {
        return qfalse as i32;
    }
    // don't chat in tournament mode
    if crate::src::game::ai_dmq3::gametype == GT_TOURNAMENT as i32 {
        return qfalse as i32;
    }
    //if fast chat is off
    if crate::src::game::ai_dmq3::bot_fastchat.integer == 0 {
        if ((rand() & 0x7fff as i32) as f32 / 0x7fff as i32 as f32) as f64 > rnd as f64 * 0.5f64 {
            return qfalse as i32;
        }
    }
    if BotValidChatPosition(bs) == 0 {
        return qfalse as i32;
    }
    //
    if BotVisibleEnemies(bs) != 0 {
        return qfalse as i32;
    }
    //
    BotEntityInfo((*bs).enemy, &mut entinfo as *mut _ as *mut aas_entityinfo_s);
    if crate::src::game::ai_dmq3::EntityIsShooting(&mut entinfo as *mut _ as *mut aas_entityinfo_s)
        as u64
        != 0
    {
        return qfalse as i32;
    }
    //
    crate::src::game::ai_dmq3::ClientName(
        lasthurt_client,
        name.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong as i32,
    );
    weap = BotWeaponNameForMeansOfDeath((*g_entities[(*bs).client as usize].client).lasthurt_mod);
    //
    BotAI_BotInitialChat(
        bs as *mut bot_state_s,
        b"hit_nodeath\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        name.as_mut_ptr(),
        weap,
        0 as *mut libc::c_void,
    );
    (*bs).lastchat_time = floattime;
    (*bs).chatto = 0 as i32;
    return qtrue as i32;
}
//
/*
==================
BotChat_HitNoKill
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotChat_HitNoKill(mut bs: *mut bot_state_t) -> i32 {
    let mut name: [libc::c_char; 32] = [0; 32];
    let mut weap: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut rnd: f32 = 0.;
    let mut entinfo: aas_entityinfo_t = aas_entityinfo_t {
        valid: 0,
        type_0: 0,
        flags: 0,
        ltime: 0.,
        update_time: 0.,
        number: 0,
        origin: [0.; 3],
        angles: [0.; 3],
        old_origin: [0.; 3],
        lastvisorigin: [0.; 3],
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
    if crate::src::game::ai_dmq3::bot_nochat.integer != 0 {
        return qfalse as i32;
    }
    if (*bs).lastchat_time > floattime - 25 as i32 as f32 {
        return qfalse as i32;
    }
    if BotNumActivePlayers() <= 1 as i32 {
        return qfalse as i32;
    }
    rnd = trap_Characteristic_BFloat((*bs).character, 33 as i32, 0 as i32 as f32, 1 as i32 as f32);
    //don't chat in teamplay
    if crate::src::game::ai_dmq3::TeamPlayIsOn() != 0 {
        return qfalse as i32;
    }
    // don't chat in tournament mode
    if crate::src::game::ai_dmq3::gametype == GT_TOURNAMENT as i32 {
        return qfalse as i32;
    }
    //if fast chat is off
    if crate::src::game::ai_dmq3::bot_fastchat.integer == 0 {
        if ((rand() & 0x7fff as i32) as f32 / 0x7fff as i32 as f32) as f64 > rnd as f64 * 0.5f64 {
            return qfalse as i32;
        }
    }
    if BotValidChatPosition(bs) == 0 {
        return qfalse as i32;
    }
    //
    if BotVisibleEnemies(bs) != 0 {
        return qfalse as i32;
    }
    //
    BotEntityInfo((*bs).enemy, &mut entinfo as *mut _ as *mut aas_entityinfo_s);
    if crate::src::game::ai_dmq3::EntityIsShooting(&mut entinfo as *mut _ as *mut aas_entityinfo_s)
        as u64
        != 0
    {
        return qfalse as i32;
    }
    //
    crate::src::game::ai_dmq3::ClientName(
        (*bs).enemy,
        name.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong as i32,
    );
    weap = BotWeaponNameForMeansOfDeath((*g_entities[(*bs).enemy as usize].client).lasthurt_mod);
    //
    BotAI_BotInitialChat(
        bs as *mut bot_state_s,
        b"hit_nokill\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        name.as_mut_ptr(),
        weap,
        0 as *mut libc::c_void,
    );
    (*bs).lastchat_time = floattime;
    (*bs).chatto = 0 as i32;
    return qtrue as i32;
}
//
/*
==================
BotChat_Random
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotChat_Random(mut bs: *mut bot_state_t) -> i32 {
    let mut rnd: f32 = 0.;
    let mut name: [libc::c_char; 32] = [0; 32];
    if crate::src::game::ai_dmq3::bot_nochat.integer != 0 {
        return qfalse as i32;
    }
    if crate::src::game::ai_dmq3::BotIsObserver(bs as *mut bot_state_s) as u64 != 0 {
        return qfalse as i32;
    }
    if (*bs).lastchat_time > floattime - 25 as i32 as f32 {
        return qfalse as i32;
    }
    // don't chat in tournament mode
    if crate::src::game::ai_dmq3::gametype == GT_TOURNAMENT as i32 {
        return qfalse as i32;
    }
    //don't chat when doing something important :)
    if (*bs).ltgtype == 1 as i32 || (*bs).ltgtype == 2 as i32 || (*bs).ltgtype == 5 as i32 {
        return qfalse as i32;
    }
    //
    rnd = trap_Characteristic_BFloat((*bs).character, 34 as i32, 0 as i32 as f32, 1 as i32 as f32);
    if ((rand() & 0x7fff as i32) as f32 / 0x7fff as i32 as f32) as f64
        > (*bs).thinktime as f64 * 0.1f64
    {
        return qfalse as i32;
    }
    if crate::src::game::ai_dmq3::bot_fastchat.integer == 0 {
        if (rand() & 0x7fff as i32) as f32 / 0x7fff as i32 as f32 > rnd {
            return qfalse as i32;
        }
        if ((rand() & 0x7fff as i32) as f32 / 0x7fff as i32 as f32) as f64 > 0.25f64 {
            return qfalse as i32;
        }
    }
    if BotNumActivePlayers() <= 1 as i32 {
        return qfalse as i32;
    }
    //
    if BotValidChatPosition(bs) == 0 {
        return qfalse as i32;
    }
    //
    if BotVisibleEnemies(bs) != 0 {
        return qfalse as i32;
    }
    //
    if (*bs).lastkilledplayer == (*bs).client {
        libc::strcpy(name.as_mut_ptr(), BotRandomOpponentName(bs));
    } else {
        crate::src::game::ai_dmq3::EasyClientName(
            (*bs).lastkilledplayer,
            name.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong as i32,
        );
    }
    if crate::src::game::ai_dmq3::TeamPlayIsOn() != 0 {
        return qfalse as i32;
        // don't wait
    }
    //
    if ((rand() & 0x7fff as i32) as f32 / 0x7fff as i32 as f32)
        < trap_Characteristic_BFloat((*bs).character, 25 as i32, 0 as i32 as f32, 1 as i32 as f32)
    {
        BotAI_BotInitialChat(
            bs as *mut bot_state_s,
            b"random_misc\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            BotRandomOpponentName(bs),
            name.as_mut_ptr(),
            b"[invalid var]\x00" as *const u8 as *const libc::c_char,
            b"[invalid var]\x00" as *const u8 as *const libc::c_char,
            BotMapTitle(),
            BotRandomWeaponName(),
            0 as *mut libc::c_void,
        );
    } else {
        BotAI_BotInitialChat(
            bs as *mut bot_state_s,
            b"random_insult\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            BotRandomOpponentName(bs),
            name.as_mut_ptr(),
            b"[invalid var]\x00" as *const u8 as *const libc::c_char,
            b"[invalid var]\x00" as *const u8 as *const libc::c_char,
            BotMapTitle(),
            BotRandomWeaponName(),
            0 as *mut libc::c_void,
        );
    }
    (*bs).lastchat_time = floattime;
    (*bs).chatto = 0 as i32;
    return qtrue as i32;
}
// time the selected chat takes to type in
/*
==================
BotChatTime
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotChatTime(mut _bs: *mut bot_state_t) -> f32 {
    //int cpm;
    //cpm = trap_Characteristic_BInteger(bs->character, CHARACTERISTIC_CHAT_CPM, 1, 4000);
    return 2.0f64 as f32;
    //(float) trap_BotChatLength(bs->cs) * 30 / cpm;
}
// test the initial bot chats
/*
==================
BotChatTest
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotChatTest(mut bs: *mut bot_state_t) {
    let mut name: [libc::c_char; 32] = [0; 32];
    let mut weap: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut num: i32 = 0;
    let mut i: i32 = 0;
    num = trap_BotNumInitialChats(
        (*bs).cs,
        b"game_enter\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    i = 0 as i32;
    while i < num {
        BotAI_BotInitialChat(
            bs as *mut bot_state_s,
            b"game_enter\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            crate::src::game::ai_dmq3::EasyClientName((*bs).client, name.as_mut_ptr(), 32 as i32),
            BotRandomOpponentName(bs),
            b"[invalid var]\x00" as *const u8 as *const libc::c_char,
            b"[invalid var]\x00" as *const u8 as *const libc::c_char,
            BotMapTitle(),
            0 as *mut libc::c_void,
        );
        trap_BotEnterChat((*bs).cs, 0 as i32, 0 as i32);
        i += 1
    }
    num = trap_BotNumInitialChats(
        (*bs).cs,
        b"game_exit\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    i = 0 as i32;
    while i < num {
        BotAI_BotInitialChat(
            bs as *mut bot_state_s,
            b"game_exit\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            crate::src::game::ai_dmq3::EasyClientName((*bs).client, name.as_mut_ptr(), 32 as i32),
            BotRandomOpponentName(bs),
            b"[invalid var]\x00" as *const u8 as *const libc::c_char,
            b"[invalid var]\x00" as *const u8 as *const libc::c_char,
            BotMapTitle(),
            0 as *mut libc::c_void,
        );
        trap_BotEnterChat((*bs).cs, 0 as i32, 0 as i32);
        i += 1
    }
    num = trap_BotNumInitialChats(
        (*bs).cs,
        b"level_start\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    i = 0 as i32;
    while i < num {
        BotAI_BotInitialChat(
            bs as *mut bot_state_s,
            b"level_start\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            crate::src::game::ai_dmq3::EasyClientName((*bs).client, name.as_mut_ptr(), 32 as i32),
            0 as *mut libc::c_void,
        );
        trap_BotEnterChat((*bs).cs, 0 as i32, 0 as i32);
        i += 1
    }
    num = trap_BotNumInitialChats(
        (*bs).cs,
        b"level_end_victory\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    i = 0 as i32;
    while i < num {
        BotAI_BotInitialChat(
            bs as *mut bot_state_s,
            b"level_end_victory\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            crate::src::game::ai_dmq3::EasyClientName((*bs).client, name.as_mut_ptr(), 32 as i32),
            BotRandomOpponentName(bs),
            BotFirstClientInRankings(),
            BotLastClientInRankings(),
            BotMapTitle(),
            0 as *mut libc::c_void,
        );
        trap_BotEnterChat((*bs).cs, 0 as i32, 0 as i32);
        i += 1
    }
    num = trap_BotNumInitialChats(
        (*bs).cs,
        b"level_end_lose\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    i = 0 as i32;
    while i < num {
        BotAI_BotInitialChat(
            bs as *mut bot_state_s,
            b"level_end_lose\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            crate::src::game::ai_dmq3::EasyClientName((*bs).client, name.as_mut_ptr(), 32 as i32),
            BotRandomOpponentName(bs),
            BotFirstClientInRankings(),
            BotLastClientInRankings(),
            BotMapTitle(),
            0 as *mut libc::c_void,
        );
        trap_BotEnterChat((*bs).cs, 0 as i32, 0 as i32);
        i += 1
    }
    num = trap_BotNumInitialChats(
        (*bs).cs,
        b"level_end\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    i = 0 as i32;
    while i < num {
        BotAI_BotInitialChat(
            bs as *mut bot_state_s,
            b"level_end\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            crate::src::game::ai_dmq3::EasyClientName((*bs).client, name.as_mut_ptr(), 32 as i32),
            BotRandomOpponentName(bs),
            BotFirstClientInRankings(),
            BotLastClientInRankings(),
            BotMapTitle(),
            0 as *mut libc::c_void,
        );
        trap_BotEnterChat((*bs).cs, 0 as i32, 0 as i32);
        i += 1
    }
    crate::src::game::ai_dmq3::EasyClientName(
        (*bs).lastkilledby,
        name.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong as i32,
    );
    num = trap_BotNumInitialChats(
        (*bs).cs,
        b"death_drown\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    i = 0 as i32;
    while i < num {
        //
        BotAI_BotInitialChat(
            bs as *mut bot_state_s,
            b"death_drown\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            name.as_mut_ptr(),
            0 as *mut libc::c_void,
        );
        trap_BotEnterChat((*bs).cs, 0 as i32, 0 as i32);
        i += 1
    }
    num = trap_BotNumInitialChats(
        (*bs).cs,
        b"death_slime\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    i = 0 as i32;
    while i < num {
        BotAI_BotInitialChat(
            bs as *mut bot_state_s,
            b"death_slime\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            name.as_mut_ptr(),
            0 as *mut libc::c_void,
        );
        trap_BotEnterChat((*bs).cs, 0 as i32, 0 as i32);
        i += 1
    }
    num = trap_BotNumInitialChats(
        (*bs).cs,
        b"death_lava\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    i = 0 as i32;
    while i < num {
        BotAI_BotInitialChat(
            bs as *mut bot_state_s,
            b"death_lava\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            name.as_mut_ptr(),
            0 as *mut libc::c_void,
        );
        trap_BotEnterChat((*bs).cs, 0 as i32, 0 as i32);
        i += 1
    }
    num = trap_BotNumInitialChats(
        (*bs).cs,
        b"death_cratered\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    i = 0 as i32;
    while i < num {
        BotAI_BotInitialChat(
            bs as *mut bot_state_s,
            b"death_cratered\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            name.as_mut_ptr(),
            0 as *mut libc::c_void,
        );
        trap_BotEnterChat((*bs).cs, 0 as i32, 0 as i32);
        i += 1
    }
    num = trap_BotNumInitialChats(
        (*bs).cs,
        b"death_suicide\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    i = 0 as i32;
    while i < num {
        BotAI_BotInitialChat(
            bs as *mut bot_state_s,
            b"death_suicide\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            name.as_mut_ptr(),
            0 as *mut libc::c_void,
        );
        trap_BotEnterChat((*bs).cs, 0 as i32, 0 as i32);
        i += 1
    }
    num = trap_BotNumInitialChats(
        (*bs).cs,
        b"death_telefrag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    i = 0 as i32;
    while i < num {
        BotAI_BotInitialChat(
            bs as *mut bot_state_s,
            b"death_telefrag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            name.as_mut_ptr(),
            0 as *mut libc::c_void,
        );
        trap_BotEnterChat((*bs).cs, 0 as i32, 0 as i32);
        i += 1
    }
    num = trap_BotNumInitialChats(
        (*bs).cs,
        b"death_gauntlet\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    i = 0 as i32;
    while i < num {
        BotAI_BotInitialChat(
            bs as *mut bot_state_s,
            b"death_gauntlet\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            name.as_mut_ptr(),
            BotWeaponNameForMeansOfDeath((*bs).botdeathtype),
            0 as *mut libc::c_void,
        );
        trap_BotEnterChat((*bs).cs, 0 as i32, 0 as i32);
        i += 1
    }
    num = trap_BotNumInitialChats(
        (*bs).cs,
        b"death_rail\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    i = 0 as i32;
    while i < num {
        BotAI_BotInitialChat(
            bs as *mut bot_state_s,
            b"death_rail\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            name.as_mut_ptr(),
            BotWeaponNameForMeansOfDeath((*bs).botdeathtype),
            0 as *mut libc::c_void,
        );
        trap_BotEnterChat((*bs).cs, 0 as i32, 0 as i32);
        i += 1
    }
    num = trap_BotNumInitialChats(
        (*bs).cs,
        b"death_bfg\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    i = 0 as i32;
    while i < num {
        BotAI_BotInitialChat(
            bs as *mut bot_state_s,
            b"death_bfg\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            name.as_mut_ptr(),
            BotWeaponNameForMeansOfDeath((*bs).botdeathtype),
            0 as *mut libc::c_void,
        );
        trap_BotEnterChat((*bs).cs, 0 as i32, 0 as i32);
        i += 1
    }
    num = trap_BotNumInitialChats(
        (*bs).cs,
        b"death_insult\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    i = 0 as i32;
    while i < num {
        BotAI_BotInitialChat(
            bs as *mut bot_state_s,
            b"death_insult\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            name.as_mut_ptr(),
            BotWeaponNameForMeansOfDeath((*bs).botdeathtype),
            0 as *mut libc::c_void,
        );
        trap_BotEnterChat((*bs).cs, 0 as i32, 0 as i32);
        i += 1
    }
    num = trap_BotNumInitialChats(
        (*bs).cs,
        b"death_praise\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    i = 0 as i32;
    while i < num {
        BotAI_BotInitialChat(
            bs as *mut bot_state_s,
            b"death_praise\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            name.as_mut_ptr(),
            BotWeaponNameForMeansOfDeath((*bs).botdeathtype),
            0 as *mut libc::c_void,
        );
        trap_BotEnterChat((*bs).cs, 0 as i32, 0 as i32);
        i += 1
    }
    //
    crate::src::game::ai_dmq3::EasyClientName((*bs).lastkilledplayer, name.as_mut_ptr(), 32 as i32);
    //
    num = trap_BotNumInitialChats(
        (*bs).cs,
        b"kill_gauntlet\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    i = 0 as i32;
    while i < num {
        //
        BotAI_BotInitialChat(
            bs as *mut bot_state_s,
            b"kill_gauntlet\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            name.as_mut_ptr(),
            0 as *mut libc::c_void,
        );
        trap_BotEnterChat((*bs).cs, 0 as i32, 0 as i32);
        i += 1
    }
    num = trap_BotNumInitialChats(
        (*bs).cs,
        b"kill_rail\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    i = 0 as i32;
    while i < num {
        BotAI_BotInitialChat(
            bs as *mut bot_state_s,
            b"kill_rail\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            name.as_mut_ptr(),
            0 as *mut libc::c_void,
        );
        trap_BotEnterChat((*bs).cs, 0 as i32, 0 as i32);
        i += 1
    }
    num = trap_BotNumInitialChats(
        (*bs).cs,
        b"kill_telefrag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    i = 0 as i32;
    while i < num {
        BotAI_BotInitialChat(
            bs as *mut bot_state_s,
            b"kill_telefrag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            name.as_mut_ptr(),
            0 as *mut libc::c_void,
        );
        trap_BotEnterChat((*bs).cs, 0 as i32, 0 as i32);
        i += 1
    }
    num = trap_BotNumInitialChats(
        (*bs).cs,
        b"kill_insult\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    i = 0 as i32;
    while i < num {
        BotAI_BotInitialChat(
            bs as *mut bot_state_s,
            b"kill_insult\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            name.as_mut_ptr(),
            0 as *mut libc::c_void,
        );
        trap_BotEnterChat((*bs).cs, 0 as i32, 0 as i32);
        i += 1
    }
    num = trap_BotNumInitialChats(
        (*bs).cs,
        b"kill_praise\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    i = 0 as i32;
    while i < num {
        BotAI_BotInitialChat(
            bs as *mut bot_state_s,
            b"kill_praise\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            name.as_mut_ptr(),
            0 as *mut libc::c_void,
        );
        trap_BotEnterChat((*bs).cs, 0 as i32, 0 as i32);
        i += 1
    }
    num = trap_BotNumInitialChats(
        (*bs).cs,
        b"enemy_suicide\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    i = 0 as i32;
    while i < num {
        BotAI_BotInitialChat(
            bs as *mut bot_state_s,
            b"enemy_suicide\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            name.as_mut_ptr(),
            0 as *mut libc::c_void,
        );
        trap_BotEnterChat((*bs).cs, 0 as i32, 0 as i32);
        i += 1
    }
    crate::src::game::ai_dmq3::ClientName(
        (*g_entities[(*bs).client as usize].client).lasthurt_client,
        name.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong as i32,
    );
    weap =
        BotWeaponNameForMeansOfDeath((*g_entities[(*bs).client as usize].client).lasthurt_client);
    num = trap_BotNumInitialChats(
        (*bs).cs,
        b"hit_talking\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    i = 0 as i32;
    while i < num {
        BotAI_BotInitialChat(
            bs as *mut bot_state_s,
            b"hit_talking\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            name.as_mut_ptr(),
            weap,
            0 as *mut libc::c_void,
        );
        trap_BotEnterChat((*bs).cs, 0 as i32, 0 as i32);
        i += 1
    }
    num = trap_BotNumInitialChats(
        (*bs).cs,
        b"hit_nodeath\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    i = 0 as i32;
    while i < num {
        BotAI_BotInitialChat(
            bs as *mut bot_state_s,
            b"hit_nodeath\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            name.as_mut_ptr(),
            weap,
            0 as *mut libc::c_void,
        );
        trap_BotEnterChat((*bs).cs, 0 as i32, 0 as i32);
        i += 1
    }
    num = trap_BotNumInitialChats(
        (*bs).cs,
        b"hit_nokill\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    i = 0 as i32;
    while i < num {
        BotAI_BotInitialChat(
            bs as *mut bot_state_s,
            b"hit_nokill\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            name.as_mut_ptr(),
            weap,
            0 as *mut libc::c_void,
        );
        trap_BotEnterChat((*bs).cs, 0 as i32, 0 as i32);
        i += 1
    }
    //
    if (*bs).lastkilledplayer == (*bs).client {
        libc::strcpy(name.as_mut_ptr(), BotRandomOpponentName(bs));
    } else {
        crate::src::game::ai_dmq3::EasyClientName(
            (*bs).lastkilledplayer,
            name.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong as i32,
        );
    }
    //
    num = trap_BotNumInitialChats(
        (*bs).cs,
        b"random_misc\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    i = 0 as i32;
    while i < num {
        //
        BotAI_BotInitialChat(
            bs as *mut bot_state_s,
            b"random_misc\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            BotRandomOpponentName(bs),
            name.as_mut_ptr(),
            b"[invalid var]\x00" as *const u8 as *const libc::c_char,
            b"[invalid var]\x00" as *const u8 as *const libc::c_char,
            BotMapTitle(),
            BotRandomWeaponName(),
            0 as *mut libc::c_void,
        );
        trap_BotEnterChat((*bs).cs, 0 as i32, 0 as i32);
        i += 1
    }
    num = trap_BotNumInitialChats(
        (*bs).cs,
        b"random_insult\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    i = 0 as i32;
    while i < num {
        BotAI_BotInitialChat(
            bs as *mut bot_state_s,
            b"random_insult\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            BotRandomOpponentName(bs),
            name.as_mut_ptr(),
            b"[invalid var]\x00" as *const u8 as *const libc::c_char,
            b"[invalid var]\x00" as *const u8 as *const libc::c_char,
            BotMapTitle(),
            BotRandomWeaponName(),
            0 as *mut libc::c_void,
        );
        trap_BotEnterChat((*bs).cs, 0 as i32, 0 as i32);
        i += 1
    }
}

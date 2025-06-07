use ::libc;

pub use crate::bg_public_h::gitem_s;
pub use crate::bg_public_h::gitem_t;
pub use crate::bg_public_h::itemType_t;
pub use crate::bg_public_h::team_t;
pub use crate::bg_public_h::weapon_t;
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
pub use crate::bg_public_h::EV_BULLET;
pub use crate::bg_public_h::EV_BULLET_HIT_FLESH;
pub use crate::bg_public_h::EV_BULLET_HIT_WALL;
pub use crate::bg_public_h::EV_CHANGE_WEAPON;
pub use crate::bg_public_h::EV_DEATH1;
pub use crate::bg_public_h::EV_DEATH2;
pub use crate::bg_public_h::EV_DEATH3;
pub use crate::bg_public_h::EV_DEBUG_LINE;
pub use crate::bg_public_h::EV_FALL_FAR;
pub use crate::bg_public_h::EV_FALL_MEDIUM;
pub use crate::bg_public_h::EV_FALL_SHORT;
pub use crate::bg_public_h::EV_FIRE_WEAPON;
pub use crate::bg_public_h::EV_FOOTSPLASH;
pub use crate::bg_public_h::EV_FOOTSTEP;
pub use crate::bg_public_h::EV_FOOTSTEP_METAL;
pub use crate::bg_public_h::EV_FOOTWADE;
pub use crate::bg_public_h::EV_GENERAL_SOUND;
pub use crate::bg_public_h::EV_GIB_PLAYER;
pub use crate::bg_public_h::EV_GLOBAL_ITEM_PICKUP;
pub use crate::bg_public_h::EV_GLOBAL_SOUND;
pub use crate::bg_public_h::EV_GLOBAL_TEAM_SOUND;
pub use crate::bg_public_h::EV_GRENADE_BOUNCE;
pub use crate::bg_public_h::EV_INVUL_IMPACT;
pub use crate::bg_public_h::EV_ITEM_PICKUP;
pub use crate::bg_public_h::EV_ITEM_POP;
pub use crate::bg_public_h::EV_ITEM_RESPAWN;
pub use crate::bg_public_h::EV_JUICED;
pub use crate::bg_public_h::EV_JUMP;
pub use crate::bg_public_h::EV_JUMP_PAD;
pub use crate::bg_public_h::EV_KAMIKAZE;
pub use crate::bg_public_h::EV_LIGHTNINGBOLT;
pub use crate::bg_public_h::EV_MISSILE_HIT;
pub use crate::bg_public_h::EV_MISSILE_MISS;
pub use crate::bg_public_h::EV_MISSILE_MISS_METAL;
pub use crate::bg_public_h::EV_NOAMMO;
pub use crate::bg_public_h::EV_NONE;
pub use crate::bg_public_h::EV_OBELISKEXPLODE;
pub use crate::bg_public_h::EV_OBELISKPAIN;
pub use crate::bg_public_h::EV_OBITUARY;
pub use crate::bg_public_h::EV_PAIN;
pub use crate::bg_public_h::EV_PLAYER_TELEPORT_IN;
pub use crate::bg_public_h::EV_PLAYER_TELEPORT_OUT;
pub use crate::bg_public_h::EV_POWERUP_BATTLESUIT;
pub use crate::bg_public_h::EV_POWERUP_QUAD;
pub use crate::bg_public_h::EV_POWERUP_REGEN;
pub use crate::bg_public_h::EV_PROXIMITY_MINE_STICK;
pub use crate::bg_public_h::EV_PROXIMITY_MINE_TRIGGER;
pub use crate::bg_public_h::EV_RAILTRAIL;
pub use crate::bg_public_h::EV_SCOREPLUM;
pub use crate::bg_public_h::EV_SHOTGUN;
pub use crate::bg_public_h::EV_STEP_12;
pub use crate::bg_public_h::EV_STEP_16;
pub use crate::bg_public_h::EV_STEP_4;
pub use crate::bg_public_h::EV_STEP_8;
pub use crate::bg_public_h::EV_STOPLOOPINGSOUND;
pub use crate::bg_public_h::EV_SWIM;
pub use crate::bg_public_h::EV_TAUNT;
pub use crate::bg_public_h::EV_TAUNT_FOLLOWME;
pub use crate::bg_public_h::EV_TAUNT_GETFLAG;
pub use crate::bg_public_h::EV_TAUNT_GUARDBASE;
pub use crate::bg_public_h::EV_TAUNT_NO;
pub use crate::bg_public_h::EV_TAUNT_PATROL;
pub use crate::bg_public_h::EV_TAUNT_YES;
pub use crate::bg_public_h::EV_USE_ITEM0;
pub use crate::bg_public_h::EV_USE_ITEM1;
pub use crate::bg_public_h::EV_USE_ITEM10;
pub use crate::bg_public_h::EV_USE_ITEM11;
pub use crate::bg_public_h::EV_USE_ITEM12;
pub use crate::bg_public_h::EV_USE_ITEM13;
pub use crate::bg_public_h::EV_USE_ITEM14;
pub use crate::bg_public_h::EV_USE_ITEM15;
pub use crate::bg_public_h::EV_USE_ITEM2;
pub use crate::bg_public_h::EV_USE_ITEM3;
pub use crate::bg_public_h::EV_USE_ITEM4;
pub use crate::bg_public_h::EV_USE_ITEM5;
pub use crate::bg_public_h::EV_USE_ITEM6;
pub use crate::bg_public_h::EV_USE_ITEM7;
pub use crate::bg_public_h::EV_USE_ITEM8;
pub use crate::bg_public_h::EV_USE_ITEM9;
pub use crate::bg_public_h::EV_WATER_CLEAR;
pub use crate::bg_public_h::EV_WATER_LEAVE;
pub use crate::bg_public_h::EV_WATER_TOUCH;
pub use crate::bg_public_h::EV_WATER_UNDER;
pub use crate::bg_public_h::GT_1FCTF;
pub use crate::bg_public_h::GT_CTF;
pub use crate::bg_public_h::GT_FFA;
pub use crate::bg_public_h::GT_HARVESTER;
pub use crate::bg_public_h::GT_MAX_GAME_TYPE;
pub use crate::bg_public_h::GT_OBELISK;
pub use crate::bg_public_h::GT_SINGLE_PLAYER;
pub use crate::bg_public_h::GT_TEAM;
pub use crate::bg_public_h::GT_TOURNAMENT;
pub use crate::bg_public_h::HI_INVULNERABILITY;
pub use crate::bg_public_h::HI_KAMIKAZE;
pub use crate::bg_public_h::HI_MEDKIT;
pub use crate::bg_public_h::HI_NONE;
pub use crate::bg_public_h::HI_NUM_HOLDABLE;
pub use crate::bg_public_h::HI_PORTAL;
pub use crate::bg_public_h::HI_TELEPORTER;
pub use crate::bg_public_h::IT_AMMO;
pub use crate::bg_public_h::IT_ARMOR;
pub use crate::bg_public_h::IT_BAD;
pub use crate::bg_public_h::IT_HEALTH;
pub use crate::bg_public_h::IT_HOLDABLE;
pub use crate::bg_public_h::IT_PERSISTANT_POWERUP;
pub use crate::bg_public_h::IT_POWERUP;
pub use crate::bg_public_h::IT_TEAM;
pub use crate::bg_public_h::IT_WEAPON;
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
pub use crate::src::game::bg_misc::bg_itemlist;
pub use crate::src::game::bg_misc::bg_numItems;
pub use crate::src::game::bg_misc::BG_CanItemBeGrabbed;
pub use crate::src::game::bg_misc::BG_EvaluateTrajectory;
pub use crate::src::game::bg_misc::BG_EvaluateTrajectoryDelta;
pub use crate::src::game::bg_misc::BG_FindItem;
pub use crate::src::game::bg_misc::BG_FindItemForWeapon;
pub use crate::src::game::g_main::g_gametype;
pub use crate::src::game::g_main::g_weaponRespawn;
pub use crate::src::game::g_main::g_weaponTeamRespawn;
pub use crate::src::game::g_main::level;
pub use crate::src::game::g_main::G_Error;
pub use crate::src::game::g_main::G_LogPrintf;
pub use crate::src::game::g_main::G_Printf;
pub use crate::src::game::g_main::G_RunThink;
pub use crate::src::game::g_spawn::G_SpawnFloat;
pub use crate::src::game::g_syscalls::trap_Cvar_VariableIntegerValue;
pub use crate::src::game::g_syscalls::trap_LinkEntity;
pub use crate::src::game::g_syscalls::trap_PointContents;
pub use crate::src::game::g_syscalls::trap_SetConfigstring;
pub use crate::src::game::g_syscalls::trap_Trace;

pub use crate::src::game::g_team::Team_CheckDroppedItem;

pub use crate::src::game::g_utils::vtos;
pub use crate::src::game::g_utils::G_AddEvent;
pub use crate::src::game::g_utils::G_AddPredictableEvent;
pub use crate::src::game::g_utils::G_FreeEntity;
pub use crate::src::game::g_utils::G_SetOrigin;
pub use crate::src::game::g_utils::G_SoundIndex;
pub use crate::src::game::g_utils::G_Spawn;
pub use crate::src::game::g_utils::G_TempEntity;
pub use crate::src::game::g_utils::G_UseTargets;
pub use crate::src::qcommon::q_math::AngleVectors;
pub use crate::src::qcommon::q_math::VectorNormalize;
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
pub use crate::src::qcommon::q_shared::Com_sprintf;
pub use crate::src::qcommon::q_shared::TR_GRAVITY;
pub use crate::src::qcommon::q_shared::TR_INTERPOLATE;
pub use crate::src::qcommon::q_shared::TR_LINEAR;
pub use crate::src::qcommon::q_shared::TR_LINEAR_STOP;
pub use crate::src::qcommon::q_shared::TR_SINE;
pub use crate::src::qcommon::q_shared::TR_STATIONARY;

//======================================================================
#[no_mangle]

pub unsafe extern "C" fn Pickup_Powerup(
    mut ent: *mut crate::g_local_h::gentity_t,
    mut other: *mut crate::g_local_h::gentity_t,
) -> libc::c_int {
    let mut quantity: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut client: *mut crate::g_local_h::gclient_t = 0 as *mut crate::g_local_h::gclient_t;
    if (*(*other).client).ps.powerups[(*(*ent).item).giTag as usize] == 0 {
        // round timing to seconds to make multiple powerup timers
        // count in sync
        (*(*other).client).ps.powerups[(*(*ent).item).giTag as usize] =
            crate::src::game::g_main::level.time
                - crate::src::game::g_main::level.time % 1000 as libc::c_int
    }
    if (*ent).count != 0 {
        quantity = (*ent).count
    } else {
        quantity = (*(*ent).item).quantity
    }
    (*(*other).client).ps.powerups[(*(*ent).item).giTag as usize] += quantity * 1000 as libc::c_int;
    // give any nearby players a "denied" anti-reward
    i = 0 as libc::c_int;
    while i < crate::src::game::g_main::level.maxclients {
        let mut delta: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
        let mut len: libc::c_float = 0.;
        let mut forward: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
        let mut tr: crate::src::qcommon::q_shared::trace_t =
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
        client = &mut *crate::src::game::g_main::level.clients.offset(i as isize)
            as *mut crate::g_local_h::gclient_s;
        if !(client == (*other).client) {
            if !((*client).pers.connected as libc::c_uint
                == crate::g_local_h::CON_DISCONNECTED as libc::c_int as libc::c_uint)
            {
                if !((*client).ps.stats[crate::bg_public_h::STAT_HEALTH as libc::c_int as usize]
                    <= 0 as libc::c_int)
                {
                    // if same team in team game, no sound
                    // cannot use OnSameTeam as it expects to g_entities, not clients
                    if !(crate::src::game::g_main::g_gametype.integer
                        >= crate::bg_public_h::GT_TEAM as libc::c_int
                        && (*(*other).client).sess.sessionTeam as libc::c_uint
                            == (*client).sess.sessionTeam as libc::c_uint)
                    {
                        // if too far away, no sound
                        delta[0 as libc::c_int as usize] = (*ent).s.pos.trBase
                            [0 as libc::c_int as usize]
                            - (*client).ps.origin[0 as libc::c_int as usize];
                        delta[1 as libc::c_int as usize] = (*ent).s.pos.trBase
                            [1 as libc::c_int as usize]
                            - (*client).ps.origin[1 as libc::c_int as usize];
                        delta[2 as libc::c_int as usize] = (*ent).s.pos.trBase
                            [2 as libc::c_int as usize]
                            - (*client).ps.origin[2 as libc::c_int as usize];
                        len = crate::src::qcommon::q_math::VectorNormalize(delta.as_mut_ptr());
                        if !(len > 192 as libc::c_int as libc::c_float) {
                            // if not facing, no sound
                            crate::src::qcommon::q_math::AngleVectors(
                                (*client).ps.viewangles.as_mut_ptr()
                                    as *const crate::src::qcommon::q_shared::vec_t,
                                forward.as_mut_ptr(),
                                0 as *mut crate::src::qcommon::q_shared::vec_t,
                                0 as *mut crate::src::qcommon::q_shared::vec_t,
                            );
                            if !(((delta[0 as libc::c_int as usize]
                                * forward[0 as libc::c_int as usize]
                                + delta[1 as libc::c_int as usize]
                                    * forward[1 as libc::c_int as usize]
                                + delta[2 as libc::c_int as usize]
                                    * forward[2 as libc::c_int as usize])
                                as libc::c_double)
                                < 0.4f64)
                            {
                                // if not line of sight, no sound
                                crate::src::game::g_syscalls::trap_Trace(
                                    &mut tr as *mut _
                                        as *mut crate::src::qcommon::q_shared::trace_t,
                                    (*client).ps.origin.as_mut_ptr()
                                        as *const crate::src::qcommon::q_shared::vec_t,
                                    0 as *const crate::src::qcommon::q_shared::vec_t,
                                    0 as *const crate::src::qcommon::q_shared::vec_t,
                                    (*ent).s.pos.trBase.as_mut_ptr()
                                        as *const crate::src::qcommon::q_shared::vec_t,
                                    ((1 as libc::c_int) << 10 as libc::c_int) - 1 as libc::c_int,
                                    1 as libc::c_int,
                                );
                                if !(tr.fraction as libc::c_double != 1.0f64) {
                                    // anti-reward
                                    (*client).ps.persistant[crate::bg_public_h::PERS_PLAYEREVENTS
                                        as libc::c_int
                                        as usize] ^= 0x1 as libc::c_int
                                }
                            }
                        }
                    }
                }
            }
        }
        i += 1
    }
    return 120 as libc::c_int;
}
//======================================================================
#[no_mangle]

pub unsafe extern "C" fn Pickup_Holdable(
    mut ent: *mut crate::g_local_h::gentity_t,
    mut other: *mut crate::g_local_h::gentity_t,
) -> libc::c_int {
    (*(*other).client).ps.stats[crate::bg_public_h::STAT_HOLDABLE_ITEM as libc::c_int as usize] =
        (*ent)
            .item
            .wrapping_offset_from(crate::src::game::bg_misc::bg_itemlist.as_mut_ptr())
            as libc::c_long as libc::c_int;
    if (*(*ent).item).giTag == crate::bg_public_h::HI_KAMIKAZE as libc::c_int {
        (*(*other).client).ps.eFlags |= 0x200 as libc::c_int
    }
    return 60 as libc::c_int;
}
//======================================================================
#[no_mangle]

pub unsafe extern "C" fn Add_Ammo(
    mut ent: *mut crate::g_local_h::gentity_t,
    mut weapon: libc::c_int,
    mut count: libc::c_int,
) {
    (*(*ent).client).ps.ammo[weapon as usize] += count;
    if (*(*ent).client).ps.ammo[weapon as usize] > 200 as libc::c_int {
        (*(*ent).client).ps.ammo[weapon as usize] = 200 as libc::c_int
    };
}
#[no_mangle]

pub unsafe extern "C" fn Pickup_Ammo(
    mut ent: *mut crate::g_local_h::gentity_t,
    mut other: *mut crate::g_local_h::gentity_t,
) -> libc::c_int {
    let mut quantity: libc::c_int = 0;
    if (*ent).count != 0 {
        quantity = (*ent).count
    } else {
        quantity = (*(*ent).item).quantity
    }
    Add_Ammo(other, (*(*ent).item).giTag, quantity);
    return 40 as libc::c_int;
}
//======================================================================
#[no_mangle]

pub unsafe extern "C" fn Pickup_Weapon(
    mut ent: *mut crate::g_local_h::gentity_t,
    mut other: *mut crate::g_local_h::gentity_t,
) -> libc::c_int {
    let mut quantity: libc::c_int = 0;
    if (*ent).count < 0 as libc::c_int {
        quantity = 0 as libc::c_int
    // None for you, sir!
    } else {
        if (*ent).count != 0 {
            quantity = (*ent).count
        } else {
            quantity = (*(*ent).item).quantity
        }
        // dropped items and teamplay weapons always have full ammo
        if (*ent).flags & 0x1000 as libc::c_int == 0
            && crate::src::game::g_main::g_gametype.integer
                != crate::bg_public_h::GT_TEAM as libc::c_int
        {
            // respawning rules
            // drop the quantity if the already have over the minimum
            if (*(*other).client).ps.ammo[(*(*ent).item).giTag as usize] < quantity {
                quantity = quantity - (*(*other).client).ps.ammo[(*(*ent).item).giTag as usize]
            } else {
                quantity = 1 as libc::c_int
                // only add a single shot
            }
        }
    }
    // add the weapon
    (*(*other).client).ps.stats[crate::bg_public_h::STAT_WEAPONS as libc::c_int as usize] |=
        (1 as libc::c_int) << (*(*ent).item).giTag; // unlimited ammo
    Add_Ammo(other, (*(*ent).item).giTag, quantity);
    if (*(*ent).item).giTag == crate::bg_public_h::WP_GRAPPLING_HOOK as libc::c_int {
        (*(*other).client).ps.ammo[(*(*ent).item).giTag as usize] = -(1 as libc::c_int)
    }
    // team deathmatch has slow weapon respawns
    if crate::src::game::g_main::g_gametype.integer == crate::bg_public_h::GT_TEAM as libc::c_int {
        return crate::src::game::g_main::g_weaponTeamRespawn.integer;
    }
    return crate::src::game::g_main::g_weaponRespawn.integer;
}
//======================================================================
#[no_mangle]

pub unsafe extern "C" fn Pickup_Health(
    mut ent: *mut crate::g_local_h::gentity_t,
    mut other: *mut crate::g_local_h::gentity_t,
) -> libc::c_int {
    let mut max: libc::c_int = 0;
    let mut quantity: libc::c_int = 0;
    // small and mega healths will go over the max
    if (*(*ent).item).quantity != 5 as libc::c_int && (*(*ent).item).quantity != 100 as libc::c_int
    {
        max =
            (*(*other).client).ps.stats[crate::bg_public_h::STAT_MAX_HEALTH as libc::c_int as usize]
    } else {
        max = (*(*other).client).ps.stats
            [crate::bg_public_h::STAT_MAX_HEALTH as libc::c_int as usize]
            * 2 as libc::c_int
    }
    if (*ent).count != 0 {
        quantity = (*ent).count
    } else {
        quantity = (*(*ent).item).quantity
    }
    (*other).health += quantity;
    if (*other).health > max {
        (*other).health = max
    }
    (*(*other).client).ps.stats[crate::bg_public_h::STAT_HEALTH as libc::c_int as usize] =
        (*other).health;
    if (*(*ent).item).quantity == 100 as libc::c_int {
        // mega health respawns slow
        return 35 as libc::c_int;
    }
    return 35 as libc::c_int;
}
//======================================================================
#[no_mangle]

pub unsafe extern "C" fn Pickup_Armor(
    mut ent: *mut crate::g_local_h::gentity_t,
    mut other: *mut crate::g_local_h::gentity_t,
) -> libc::c_int {
    (*(*other).client).ps.stats[crate::bg_public_h::STAT_ARMOR as libc::c_int as usize] +=
        (*(*ent).item).quantity;
    if (*(*other).client).ps.stats[crate::bg_public_h::STAT_ARMOR as libc::c_int as usize]
        > (*(*other).client).ps.stats[crate::bg_public_h::STAT_MAX_HEALTH as libc::c_int as usize]
            * 2 as libc::c_int
    {
        (*(*other).client).ps.stats[crate::bg_public_h::STAT_ARMOR as libc::c_int as usize] =
            (*(*other).client).ps.stats[crate::bg_public_h::STAT_MAX_HEALTH as libc::c_int as usize]
                * 2 as libc::c_int
    }
    return 25 as libc::c_int;
}
//======================================================================
/*
===============
RespawnItem
===============
*/
#[no_mangle]

pub unsafe extern "C" fn RespawnItem(mut ent: *mut crate::g_local_h::gentity_t) {
    if ent.is_null() {
        return;
    }
    // randomly select from teamed entities
    if !(*ent).team.is_null() {
        let mut master: *mut crate::g_local_h::gentity_t = 0 as *mut crate::g_local_h::gentity_t;
        let mut count: libc::c_int = 0;
        let mut choice: libc::c_int = 0;
        if (*ent).teammaster.is_null() {
            crate::src::game::g_main::G_Error(
                b"RespawnItem: bad teammaster\x00" as *const u8 as *const libc::c_char,
            );
        }
        master = (*ent).teammaster;
        count = 0 as libc::c_int;
        ent = master;
        while !ent.is_null() {
            ent = (*ent).teamchain;
            count += 1
        }
        choice = ::libc::rand() % count;
        count = 0 as libc::c_int;
        ent = master;
        while !ent.is_null() && count < choice {
            ent = (*ent).teamchain;
            count += 1
        }
    }
    if ent.is_null() {
        return;
    }
    (*ent).r.contents = 0x40000000 as libc::c_int;
    (*ent).s.eFlags &= !(0x80 as libc::c_int);
    (*ent).r.svFlags &= !(0x1 as libc::c_int);
    crate::src::game::g_syscalls::trap_LinkEntity(ent as *mut crate::g_local_h::gentity_s);
    if (*(*ent).item).giType as libc::c_uint
        == crate::bg_public_h::IT_POWERUP as libc::c_int as libc::c_uint
    {
        // play powerup spawn sound to all clients
        let mut te: *mut crate::g_local_h::gentity_t = 0 as *mut crate::g_local_h::gentity_t;
        // if the powerup respawn sound should Not be global
        if (*ent).speed != 0. {
            te = crate::src::game::g_utils::G_TempEntity(
                (*ent).s.pos.trBase.as_mut_ptr(),
                crate::bg_public_h::EV_GENERAL_SOUND as libc::c_int,
            ) as *mut crate::g_local_h::gentity_s
        } else {
            te = crate::src::game::g_utils::G_TempEntity(
                (*ent).s.pos.trBase.as_mut_ptr(),
                crate::bg_public_h::EV_GLOBAL_SOUND as libc::c_int,
            ) as *mut crate::g_local_h::gentity_s
        }
        (*te).s.eventParm = crate::src::game::g_utils::G_SoundIndex(
            b"sound/items/poweruprespawn.wav\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        (*te).r.svFlags |= 0x20 as libc::c_int
    }
    if (*(*ent).item).giType as libc::c_uint
        == crate::bg_public_h::IT_HOLDABLE as libc::c_int as libc::c_uint
        && (*(*ent).item).giTag == crate::bg_public_h::HI_KAMIKAZE as libc::c_int
    {
        // play powerup spawn sound to all clients
        let mut te_0: *mut crate::g_local_h::gentity_t = 0 as *mut crate::g_local_h::gentity_t;
        // if the powerup respawn sound should Not be global
        if (*ent).speed != 0. {
            te_0 = crate::src::game::g_utils::G_TempEntity(
                (*ent).s.pos.trBase.as_mut_ptr(),
                crate::bg_public_h::EV_GENERAL_SOUND as libc::c_int,
            ) as *mut crate::g_local_h::gentity_s
        } else {
            te_0 = crate::src::game::g_utils::G_TempEntity(
                (*ent).s.pos.trBase.as_mut_ptr(),
                crate::bg_public_h::EV_GLOBAL_SOUND as libc::c_int,
            ) as *mut crate::g_local_h::gentity_s
        }
        (*te_0).s.eventParm = crate::src::game::g_utils::G_SoundIndex(
            b"sound/items/kamikazerespawn.wav\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        (*te_0).r.svFlags |= 0x20 as libc::c_int
    }
    // play the normal respawn sound only to nearby clients
    crate::src::game::g_utils::G_AddEvent(
        ent as *mut crate::g_local_h::gentity_s,
        crate::bg_public_h::EV_ITEM_RESPAWN as libc::c_int,
        0 as libc::c_int,
    );
    (*ent).nextthink = 0 as libc::c_int;
}
/*
===============
Touch_Item
===============
*/
#[no_mangle]

pub unsafe extern "C" fn Touch_Item(
    mut ent: *mut crate::g_local_h::gentity_t,
    mut other: *mut crate::g_local_h::gentity_t,
    mut _trace: *mut crate::src::qcommon::q_shared::trace_t,
) {
    let mut respawn: libc::c_int = 0; // dead people can't pickup
    let mut predict: crate::src::qcommon::q_shared::qboolean =
        crate::src::qcommon::q_shared::qfalse;
    if (*other).client.is_null() {
        return;
    }
    if (*other).health < 1 as libc::c_int {
        return;
    }
    // the same pickup rules are used for client side and server side
    if crate::src::game::bg_misc::BG_CanItemBeGrabbed(
        crate::src::game::g_main::g_gametype.integer,
        &mut (*ent).s as *mut _ as *const crate::src::qcommon::q_shared::entityState_s,
        &mut (*(*other).client).ps as *mut _ as *const crate::src::qcommon::q_shared::playerState_s,
    ) as u64
        == 0
    {
        return;
    }
    crate::src::game::g_main::G_LogPrintf(
        b"Item: %i %s\n\x00" as *const u8 as *const libc::c_char,
        (*other).s.number,
        (*(*ent).item).classname,
    );
    predict = (*(*other).client).pers.predictItemPickup;
    // call the item-specific pickup function
    match (*(*ent).item).giType as libc::c_uint {
        1 => respawn = Pickup_Weapon(ent, other),
        2 => respawn = Pickup_Ammo(ent, other),
        3 => respawn = Pickup_Armor(ent, other),
        4 => respawn = Pickup_Health(ent, other),
        5 => {
            respawn = Pickup_Powerup(ent, other);
            predict = crate::src::qcommon::q_shared::qfalse
        }
        8 => {
            respawn = crate::src::game::g_team::Pickup_Team(
                ent as *mut crate::g_local_h::gentity_s,
                other as *mut crate::g_local_h::gentity_s,
            )
        }
        6 => respawn = Pickup_Holdable(ent, other),
        _ => return,
    }
    if respawn == 0 {
        return;
    }
    // play the normal pickup sound
    if predict as u64 != 0 {
        crate::src::game::g_utils::G_AddPredictableEvent(
            other as *mut crate::g_local_h::gentity_s,
            crate::bg_public_h::EV_ITEM_PICKUP as libc::c_int,
            (*ent).s.modelindex,
        );
    } else {
        crate::src::game::g_utils::G_AddEvent(
            other as *mut crate::g_local_h::gentity_s,
            crate::bg_public_h::EV_ITEM_PICKUP as libc::c_int,
            (*ent).s.modelindex,
        );
    }
    // powerup pickups are global broadcasts
    if (*(*ent).item).giType as libc::c_uint
        == crate::bg_public_h::IT_POWERUP as libc::c_int as libc::c_uint
        || (*(*ent).item).giType as libc::c_uint
            == crate::bg_public_h::IT_TEAM as libc::c_int as libc::c_uint
    {
        // if we want the global sound to play
        if (*ent).speed == 0. {
            let mut te: *mut crate::g_local_h::gentity_t = 0 as *mut crate::g_local_h::gentity_t;
            te = crate::src::game::g_utils::G_TempEntity(
                (*ent).s.pos.trBase.as_mut_ptr(),
                crate::bg_public_h::EV_GLOBAL_ITEM_PICKUP as libc::c_int,
            ) as *mut crate::g_local_h::gentity_s;
            (*te).s.eventParm = (*ent).s.modelindex;
            (*te).r.svFlags |= 0x20 as libc::c_int
        } else {
            let mut te_0: *mut crate::g_local_h::gentity_t = 0 as *mut crate::g_local_h::gentity_t;
            te_0 = crate::src::game::g_utils::G_TempEntity(
                (*ent).s.pos.trBase.as_mut_ptr(),
                crate::bg_public_h::EV_GLOBAL_ITEM_PICKUP as libc::c_int,
            ) as *mut crate::g_local_h::gentity_s;
            (*te_0).s.eventParm = (*ent).s.modelindex;
            // only send this temp entity to a single client
            (*te_0).r.svFlags |= 0x100 as libc::c_int;
            (*te_0).r.singleClient = (*other).s.number
        }
    }
    // fire item targets
    crate::src::game::g_utils::G_UseTargets(
        ent as *mut crate::g_local_h::gentity_s,
        other as *mut crate::g_local_h::gentity_s,
    );
    // wait of -1 will not respawn
    if (*ent).wait == -(1 as libc::c_int) as libc::c_float {
        (*ent).r.svFlags |= 0x1 as libc::c_int;
        (*ent).s.eFlags |= 0x80 as libc::c_int;
        (*ent).r.contents = 0 as libc::c_int;
        (*ent).unlinkAfterEvent = crate::src::qcommon::q_shared::qtrue;
        return;
    }
    // non zero wait overrides respawn time
    if (*ent).wait != 0. {
        respawn = (*ent).wait as libc::c_int
    }
    // random can be used to vary the respawn time
    if (*ent).random != 0. {
        respawn = (respawn as libc::c_double
            + 2.0f64
                * (((::libc::rand() & 0x7fff as libc::c_int) as libc::c_float
                    / 0x7fff as libc::c_int as libc::c_float) as libc::c_double
                    - 0.5f64)
                * (*ent).random as libc::c_double) as libc::c_int;
        if respawn < 1 as libc::c_int {
            respawn = 1 as libc::c_int
        }
    }
    // dropped items will not respawn
    if (*ent).flags & 0x1000 as libc::c_int != 0 {
        (*ent).freeAfterEvent = crate::src::qcommon::q_shared::qtrue
    }
    // picked up items still stay around, they just don't
    // draw anything.  This allows respawnable items
    // to be placed on movers.
    (*ent).r.svFlags |= 0x1 as libc::c_int;
    (*ent).s.eFlags |= 0x80 as libc::c_int;
    (*ent).r.contents = 0 as libc::c_int;
    // ZOID
    // A negative respawn times means to never respawn this item (but don't
    // delete it).  This is used by items that are respawned by third party
    // events such as ctf flags
    if respawn <= 0 as libc::c_int {
        (*ent).nextthink = 0 as libc::c_int;
        (*ent).think = None
    } else {
        (*ent).nextthink = crate::src::game::g_main::level.time + respawn * 1000 as libc::c_int;
        (*ent).think =
            Some(RespawnItem as unsafe extern "C" fn(_: *mut crate::g_local_h::gentity_t) -> ())
    }
    crate::src::game::g_syscalls::trap_LinkEntity(ent as *mut crate::g_local_h::gentity_s);
}
//======================================================================
/*
================
LaunchItem

Spawns an item and tosses it forward
================
*/
#[no_mangle]

pub unsafe extern "C" fn LaunchItem(
    mut item: *mut crate::bg_public_h::gitem_t,
    mut origin: *mut crate::src::qcommon::q_shared::vec_t,
    mut velocity: *mut crate::src::qcommon::q_shared::vec_t,
) -> *mut crate::g_local_h::gentity_t {
    let mut dropped: *mut crate::g_local_h::gentity_t = 0 as *mut crate::g_local_h::gentity_t; // store item number in modelindex
    dropped = crate::src::game::g_utils::G_Spawn() as *mut crate::g_local_h::gentity_s; // This is non-zero is it's a dropped item
    (*dropped).s.eType = crate::bg_public_h::ET_ITEM as libc::c_int; // auto-remove after 30 seconds
    (*dropped).s.modelindex = item
        .wrapping_offset_from(crate::src::game::bg_misc::bg_itemlist.as_mut_ptr())
        as libc::c_long as libc::c_int;
    (*dropped).s.modelindex2 = 1 as libc::c_int;
    (*dropped).classname = (*item).classname;
    (*dropped).item = item;
    (*dropped).r.mins[0 as libc::c_int as usize] =
        -(15 as libc::c_int) as crate::src::qcommon::q_shared::vec_t;
    (*dropped).r.mins[1 as libc::c_int as usize] =
        -(15 as libc::c_int) as crate::src::qcommon::q_shared::vec_t;
    (*dropped).r.mins[2 as libc::c_int as usize] =
        -(15 as libc::c_int) as crate::src::qcommon::q_shared::vec_t;
    (*dropped).r.maxs[0 as libc::c_int as usize] =
        15 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
    (*dropped).r.maxs[1 as libc::c_int as usize] =
        15 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
    (*dropped).r.maxs[2 as libc::c_int as usize] =
        15 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
    (*dropped).r.contents = 0x40000000 as libc::c_int;
    (*dropped).touch = Some(
        Touch_Item
            as unsafe extern "C" fn(
                _: *mut crate::g_local_h::gentity_t,
                _: *mut crate::g_local_h::gentity_t,
                _: *mut crate::src::qcommon::q_shared::trace_t,
            ) -> (),
    );
    crate::src::game::g_utils::G_SetOrigin(dropped as *mut crate::g_local_h::gentity_s, origin);
    (*dropped).s.pos.trType = crate::src::qcommon::q_shared::TR_GRAVITY;
    (*dropped).s.pos.trTime = crate::src::game::g_main::level.time;
    (*dropped).s.pos.trDelta[0 as libc::c_int as usize] =
        *velocity.offset(0 as libc::c_int as isize);
    (*dropped).s.pos.trDelta[1 as libc::c_int as usize] =
        *velocity.offset(1 as libc::c_int as isize);
    (*dropped).s.pos.trDelta[2 as libc::c_int as usize] =
        *velocity.offset(2 as libc::c_int as isize);
    (*dropped).s.eFlags |= 0x20 as libc::c_int;
    if crate::src::game::g_main::g_gametype.integer == crate::bg_public_h::GT_CTF as libc::c_int
        && (*item).giType as libc::c_uint
            == crate::bg_public_h::IT_TEAM as libc::c_int as libc::c_uint
    {
        // Special case for CTF flags
        (*dropped).think = Some(
            crate::src::game::g_team::Team_DroppedFlagThink
                as unsafe extern "C" fn(_: *mut crate::g_local_h::gentity_t) -> (),
        );
        (*dropped).nextthink = crate::src::game::g_main::level.time + 30000 as libc::c_int;
        crate::src::game::g_team::Team_CheckDroppedItem(
            dropped as *mut crate::g_local_h::gentity_s,
        );
    } else {
        (*dropped).think = Some(
            crate::src::game::g_utils::G_FreeEntity
                as unsafe extern "C" fn(_: *mut crate::g_local_h::gentity_t) -> (),
        );
        (*dropped).nextthink = crate::src::game::g_main::level.time + 30000 as libc::c_int
    }
    (*dropped).flags = 0x1000 as libc::c_int;
    crate::src::game::g_syscalls::trap_LinkEntity(dropped as *mut crate::g_local_h::gentity_s);
    return dropped;
}
/*
================
Drop_Item

Spawns an item and tosses it forward
================
*/
#[no_mangle]

pub unsafe extern "C" fn Drop_Item(
    mut ent: *mut crate::g_local_h::gentity_t,
    mut item: *mut crate::bg_public_h::gitem_t,
    mut angle: libc::c_float,
) -> *mut crate::g_local_h::gentity_t {
    let mut velocity: crate::src::qcommon::q_shared::vec3_t = [0.; 3]; // always forward
    let mut angles: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    angles[0 as libc::c_int as usize] = (*ent).s.apos.trBase[0 as libc::c_int as usize];
    angles[1 as libc::c_int as usize] = (*ent).s.apos.trBase[1 as libc::c_int as usize];
    angles[2 as libc::c_int as usize] = (*ent).s.apos.trBase[2 as libc::c_int as usize];
    angles[1 as libc::c_int as usize] += angle;
    angles[0 as libc::c_int as usize] = 0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
    crate::src::qcommon::q_math::AngleVectors(
        angles.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        velocity.as_mut_ptr(),
        0 as *mut crate::src::qcommon::q_shared::vec_t,
        0 as *mut crate::src::qcommon::q_shared::vec_t,
    );
    velocity[0 as libc::c_int as usize] =
        velocity[0 as libc::c_int as usize] * 150 as libc::c_int as libc::c_float;
    velocity[1 as libc::c_int as usize] =
        velocity[1 as libc::c_int as usize] * 150 as libc::c_int as libc::c_float;
    velocity[2 as libc::c_int as usize] =
        velocity[2 as libc::c_int as usize] * 150 as libc::c_int as libc::c_float;
    velocity[2 as libc::c_int as usize] = (velocity[2 as libc::c_int as usize] as libc::c_double
        + (200 as libc::c_int as libc::c_double
            + 2.0f64
                * (((::libc::rand() & 0x7fff as libc::c_int) as libc::c_float
                    / 0x7fff as libc::c_int as libc::c_float)
                    as libc::c_double
                    - 0.5f64)
                * 50 as libc::c_int as libc::c_double))
        as crate::src::qcommon::q_shared::vec_t;
    return LaunchItem(
        item,
        (*ent).s.pos.trBase.as_mut_ptr(),
        velocity.as_mut_ptr(),
    );
}
/*
================
Use_Item

Respawn the item
================
*/
#[no_mangle]

pub unsafe extern "C" fn Use_Item(
    mut ent: *mut crate::g_local_h::gentity_t,
    mut _other: *mut crate::g_local_h::gentity_t,
    mut _activator: *mut crate::g_local_h::gentity_t,
) {
    RespawnItem(ent);
}
//======================================================================
/*
================
FinishSpawningItem

Traces down to find where an item should rest, instead of letting them
free fall from their spawn points
================
*/
#[no_mangle]

pub unsafe extern "C" fn FinishSpawningItem(mut ent: *mut crate::g_local_h::gentity_t) {
    let mut tr: crate::src::qcommon::q_shared::trace_t = crate::src::qcommon::q_shared::trace_t {
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
    }; // store item number in modelindex
    let mut dest: crate::src::qcommon::q_shared::vec3_t = [0.; 3]; // zero indicates this isn't a dropped item
    (*ent).r.mins[0 as libc::c_int as usize] =
        -(15 as libc::c_int) as crate::src::qcommon::q_shared::vec_t;
    (*ent).r.mins[1 as libc::c_int as usize] =
        -(15 as libc::c_int) as crate::src::qcommon::q_shared::vec_t;
    (*ent).r.mins[2 as libc::c_int as usize] =
        -(15 as libc::c_int) as crate::src::qcommon::q_shared::vec_t;
    (*ent).r.maxs[0 as libc::c_int as usize] =
        15 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
    (*ent).r.maxs[1 as libc::c_int as usize] =
        15 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
    (*ent).r.maxs[2 as libc::c_int as usize] =
        15 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
    (*ent).s.eType = crate::bg_public_h::ET_ITEM as libc::c_int;
    (*ent).s.modelindex = (*ent)
        .item
        .wrapping_offset_from(crate::src::game::bg_misc::bg_itemlist.as_mut_ptr())
        as libc::c_long as libc::c_int;
    (*ent).s.modelindex2 = 0 as libc::c_int;
    (*ent).r.contents = 0x40000000 as libc::c_int;
    (*ent).touch = Some(
        Touch_Item
            as unsafe extern "C" fn(
                _: *mut crate::g_local_h::gentity_t,
                _: *mut crate::g_local_h::gentity_t,
                _: *mut crate::src::qcommon::q_shared::trace_t,
            ) -> (),
    );
    // using an item causes it to respawn
    (*ent).use_0 = Some(
        Use_Item
            as unsafe extern "C" fn(
                _: *mut crate::g_local_h::gentity_t,
                _: *mut crate::g_local_h::gentity_t,
                _: *mut crate::g_local_h::gentity_t,
            ) -> (),
    );
    if (*ent).spawnflags & 1 as libc::c_int != 0 {
        // suspended
        crate::src::game::g_utils::G_SetOrigin(
            ent as *mut crate::g_local_h::gentity_s,
            (*ent).s.origin.as_mut_ptr(),
        );
    } else {
        // drop to floor
        dest[0 as libc::c_int as usize] = (*ent).s.origin[0 as libc::c_int as usize];
        dest[1 as libc::c_int as usize] = (*ent).s.origin[1 as libc::c_int as usize];
        dest[2 as libc::c_int as usize] =
            (*ent).s.origin[2 as libc::c_int as usize] - 4096 as libc::c_int as libc::c_float;
        crate::src::game::g_syscalls::trap_Trace(
            &mut tr as *mut _ as *mut crate::src::qcommon::q_shared::trace_t,
            (*ent).s.origin.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            (*ent).r.mins.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            (*ent).r.maxs.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            dest.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            (*ent).s.number,
            1 as libc::c_int,
        );
        if tr.startsolid as u64 != 0 {
            crate::src::game::g_main::G_Printf(
                b"FinishSpawningItem: %s startsolid at %s\n\x00" as *const u8
                    as *const libc::c_char,
                (*ent).classname,
                crate::src::game::g_utils::vtos(
                    (*ent).s.origin.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t
                ),
            );
            crate::src::game::g_utils::G_FreeEntity(ent as *mut crate::g_local_h::gentity_s);
            return;
        }
        // allow to ride movers
        (*ent).s.groundEntityNum = tr.entityNum;
        crate::src::game::g_utils::G_SetOrigin(
            ent as *mut crate::g_local_h::gentity_s,
            tr.endpos.as_mut_ptr(),
        );
    }
    // team slaves and targeted items aren't present at start
    if (*ent).flags & 0x400 as libc::c_int != 0 || !(*ent).targetname.is_null() {
        (*ent).s.eFlags |= 0x80 as libc::c_int;
        (*ent).r.contents = 0 as libc::c_int;
        return;
    }
    // powerups don't spawn in for a while
    if (*(*ent).item).giType as libc::c_uint
        == crate::bg_public_h::IT_POWERUP as libc::c_int as libc::c_uint
    {
        let mut respawn: libc::c_float = 0.;
        respawn = (45 as libc::c_int as libc::c_double
            + 2.0f64
                * (((::libc::rand() & 0x7fff as libc::c_int) as libc::c_float
                    / 0x7fff as libc::c_int as libc::c_float) as libc::c_double
                    - 0.5f64)
                * 15 as libc::c_int as libc::c_double) as libc::c_float;
        (*ent).s.eFlags |= 0x80 as libc::c_int;
        (*ent).r.contents = 0 as libc::c_int;
        (*ent).nextthink = (crate::src::game::g_main::level.time as libc::c_float
            + respawn * 1000 as libc::c_int as libc::c_float)
            as libc::c_int;
        (*ent).think =
            Some(RespawnItem as unsafe extern "C" fn(_: *mut crate::g_local_h::gentity_t) -> ());
        return;
    }
    crate::src::game::g_syscalls::trap_LinkEntity(ent as *mut crate::g_local_h::gentity_s);
}
#[no_mangle]

pub static mut itemRegistered: [crate::src::qcommon::q_shared::qboolean; 256] =
    [crate::src::qcommon::q_shared::qfalse; 256];
/*
==================
G_CheckTeamItems
==================
*/
#[no_mangle]

pub unsafe extern "C" fn G_CheckTeamItems() {
    // Set up team stuff
    crate::src::game::g_team::Team_InitGame();
    if crate::src::game::g_main::g_gametype.integer == crate::bg_public_h::GT_CTF as libc::c_int {
        let mut item: *mut crate::bg_public_h::gitem_t = 0 as *mut crate::bg_public_h::gitem_t;
        // check for the two flags
        item = crate::src::game::bg_misc::BG_FindItem(
            b"Red Flag\x00" as *const u8 as *const libc::c_char,
        ) as *mut crate::bg_public_h::gitem_s;
        if item.is_null()
            || itemRegistered[item
                .wrapping_offset_from(crate::src::game::bg_misc::bg_itemlist.as_mut_ptr())
                as libc::c_long as usize] as u64
                == 0
        {
            crate::src::game::g_main::G_Printf(
                b"^3WARNING: No team_CTF_redflag in map\n\x00" as *const u8 as *const libc::c_char,
            );
        }
        item = crate::src::game::bg_misc::BG_FindItem(
            b"Blue Flag\x00" as *const u8 as *const libc::c_char,
        ) as *mut crate::bg_public_h::gitem_s;
        if item.is_null()
            || itemRegistered[item
                .wrapping_offset_from(crate::src::game::bg_misc::bg_itemlist.as_mut_ptr())
                as libc::c_long as usize] as u64
                == 0
        {
            crate::src::game::g_main::G_Printf(
                b"^3WARNING: No team_CTF_blueflag in map\n\x00" as *const u8 as *const libc::c_char,
            );
        }
    };
}
/*
==============
ClearRegisteredItems
==============
*/
#[no_mangle]

pub unsafe extern "C" fn ClearRegisteredItems() {
    crate::stdlib::memset(
        itemRegistered.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[crate::src::qcommon::q_shared::qboolean; 256]>() as libc::c_ulong,
    );
    // players always start with the base weapon
    RegisterItem(
        crate::src::game::bg_misc::BG_FindItemForWeapon(crate::bg_public_h::WP_MACHINEGUN)
            as *mut crate::bg_public_h::gitem_s,
    );
    RegisterItem(
        crate::src::game::bg_misc::BG_FindItemForWeapon(crate::bg_public_h::WP_GAUNTLET)
            as *mut crate::bg_public_h::gitem_s,
    );
}
/*
===============
RegisterItem

The item will be added to the precache list
===============
*/
#[no_mangle]

pub unsafe extern "C" fn RegisterItem(mut item: *mut crate::bg_public_h::gitem_t) {
    if item.is_null() {
        crate::src::game::g_main::G_Error(
            b"RegisterItem: NULL\x00" as *const u8 as *const libc::c_char,
        );
    }
    itemRegistered[item.wrapping_offset_from(crate::src::game::bg_misc::bg_itemlist.as_mut_ptr())
        as libc::c_long as usize] = crate::src::qcommon::q_shared::qtrue;
}
/*
===============
SaveRegisteredItems

Write the needed items to a config string
so the client will know which ones to precache
===============
*/
#[no_mangle]

pub unsafe extern "C" fn SaveRegisteredItems() {
    let mut string: [libc::c_char; 257] = [0; 257];
    let mut i: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    count = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < crate::src::game::bg_misc::bg_numItems {
        if itemRegistered[i as usize] as u64 != 0 {
            count += 1;
            string[i as usize] = '1' as i32 as libc::c_char
        } else {
            string[i as usize] = '0' as i32 as libc::c_char
        }
        i += 1
    }
    string[crate::src::game::bg_misc::bg_numItems as usize] = 0 as libc::c_int as libc::c_char;
    crate::src::game::g_main::G_Printf(
        b"%i items registered\n\x00" as *const u8 as *const libc::c_char,
        count,
    );
    crate::src::game::g_syscalls::trap_SetConfigstring(27 as libc::c_int, string.as_mut_ptr());
}
/*
============
G_ItemDisabled
============
*/
#[no_mangle]

pub unsafe extern "C" fn G_ItemDisabled(mut item: *mut crate::bg_public_h::gitem_t) -> libc::c_int {
    let mut name: [libc::c_char; 128] = [0; 128];
    crate::src::qcommon::q_shared::Com_sprintf(
        name.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong as libc::c_int,
        b"disable_%s\x00" as *const u8 as *const libc::c_char,
        (*item).classname,
    );
    return crate::src::game::g_syscalls::trap_Cvar_VariableIntegerValue(name.as_mut_ptr());
}
/*
============
G_SpawnItem

Sets the clipping size and plants the object on the floor.

Items can't be immediately dropped to floor, because they might
be on an entity that hasn't spawned yet.
============
*/
#[no_mangle]

pub unsafe extern "C" fn G_SpawnItem(
    mut ent: *mut crate::g_local_h::gentity_t,
    mut item: *mut crate::bg_public_h::gitem_t,
) {
    crate::src::game::g_spawn::G_SpawnFloat(
        b"random\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        &mut (*ent).random,
    );
    crate::src::game::g_spawn::G_SpawnFloat(
        b"wait\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        &mut (*ent).wait,
    );
    RegisterItem(item);
    if G_ItemDisabled(item) != 0 {
        return;
    }
    (*ent).item = item;
    // some movers spawn on the second frame, so delay item
    // spawns until the third frame so they can ride trains
    (*ent).nextthink = crate::src::game::g_main::level.time + 100 as libc::c_int * 2 as libc::c_int; // items are bouncy
    (*ent).think =
        Some(FinishSpawningItem as unsafe extern "C" fn(_: *mut crate::g_local_h::gentity_t) -> ());
    (*ent).physicsBounce = 0.50f64 as libc::c_float;
    if (*item).giType as libc::c_uint
        == crate::bg_public_h::IT_POWERUP as libc::c_int as libc::c_uint
    {
        crate::src::game::g_utils::G_SoundIndex(
            b"sound/items/poweruprespawn.wav\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        crate::src::game::g_spawn::G_SpawnFloat(
            b"noglobalsound\x00" as *const u8 as *const libc::c_char,
            b"0\x00" as *const u8 as *const libc::c_char,
            &mut (*ent).speed,
        );
    };
}
/*
================
G_BounceItem

================
*/
#[no_mangle]

pub unsafe extern "C" fn G_BounceItem(
    mut ent: *mut crate::g_local_h::gentity_t,
    mut trace: *mut crate::src::qcommon::q_shared::trace_t,
) {
    let mut velocity: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut dot: libc::c_float = 0.;
    let mut hitTime: libc::c_int = 0;
    // reflect the velocity on the trace plane
    hitTime = (crate::src::game::g_main::level.previousTime as libc::c_float
        + (crate::src::game::g_main::level.time - crate::src::game::g_main::level.previousTime)
            as libc::c_float
            * (*trace).fraction) as libc::c_int;
    crate::src::game::bg_misc::BG_EvaluateTrajectoryDelta(
        &mut (*ent).s.pos as *mut _ as *const crate::src::qcommon::q_shared::trajectory_t,
        hitTime,
        velocity.as_mut_ptr(),
    );
    dot = velocity[0 as libc::c_int as usize] * (*trace).plane.normal[0 as libc::c_int as usize]
        + velocity[1 as libc::c_int as usize] * (*trace).plane.normal[1 as libc::c_int as usize]
        + velocity[2 as libc::c_int as usize] * (*trace).plane.normal[2 as libc::c_int as usize];
    (*ent).s.pos.trDelta[0 as libc::c_int as usize] = velocity[0 as libc::c_int as usize]
        + (*trace).plane.normal[0 as libc::c_int as usize]
            * (-(2 as libc::c_int) as libc::c_float * dot);
    (*ent).s.pos.trDelta[1 as libc::c_int as usize] = velocity[1 as libc::c_int as usize]
        + (*trace).plane.normal[1 as libc::c_int as usize]
            * (-(2 as libc::c_int) as libc::c_float * dot);
    (*ent).s.pos.trDelta[2 as libc::c_int as usize] = velocity[2 as libc::c_int as usize]
        + (*trace).plane.normal[2 as libc::c_int as usize]
            * (-(2 as libc::c_int) as libc::c_float * dot);
    // cut the velocity to keep from bouncing forever
    (*ent).s.pos.trDelta[0 as libc::c_int as usize] =
        (*ent).s.pos.trDelta[0 as libc::c_int as usize] * (*ent).physicsBounce;
    (*ent).s.pos.trDelta[1 as libc::c_int as usize] =
        (*ent).s.pos.trDelta[1 as libc::c_int as usize] * (*ent).physicsBounce;
    (*ent).s.pos.trDelta[2 as libc::c_int as usize] =
        (*ent).s.pos.trDelta[2 as libc::c_int as usize] * (*ent).physicsBounce;
    // check for stop
    if (*trace).plane.normal[2 as libc::c_int as usize] > 0 as libc::c_int as libc::c_float
        && (*ent).s.pos.trDelta[2 as libc::c_int as usize] < 40 as libc::c_int as libc::c_float
    {
        (*trace).endpos[2 as libc::c_int as usize] =
            ((*trace).endpos[2 as libc::c_int as usize] as libc::c_double + 1.0f64)
                as crate::src::qcommon::q_shared::vec_t; // make sure it is off ground
        (*trace).endpos[0 as libc::c_int as usize] = (*trace).endpos[0 as libc::c_int as usize]
            as libc::c_int
            as crate::src::qcommon::q_shared::vec_t;
        (*trace).endpos[1 as libc::c_int as usize] = (*trace).endpos[1 as libc::c_int as usize]
            as libc::c_int
            as crate::src::qcommon::q_shared::vec_t;
        (*trace).endpos[2 as libc::c_int as usize] = (*trace).endpos[2 as libc::c_int as usize]
            as libc::c_int
            as crate::src::qcommon::q_shared::vec_t;
        crate::src::game::g_utils::G_SetOrigin(
            ent as *mut crate::g_local_h::gentity_s,
            (*trace).endpos.as_mut_ptr(),
        );
        (*ent).s.groundEntityNum = (*trace).entityNum;
        return;
    }
    (*ent).r.currentOrigin[0 as libc::c_int as usize] = (*ent).r.currentOrigin
        [0 as libc::c_int as usize]
        + (*trace).plane.normal[0 as libc::c_int as usize];
    (*ent).r.currentOrigin[1 as libc::c_int as usize] = (*ent).r.currentOrigin
        [1 as libc::c_int as usize]
        + (*trace).plane.normal[1 as libc::c_int as usize];
    (*ent).r.currentOrigin[2 as libc::c_int as usize] = (*ent).r.currentOrigin
        [2 as libc::c_int as usize]
        + (*trace).plane.normal[2 as libc::c_int as usize];
    (*ent).s.pos.trBase[0 as libc::c_int as usize] =
        (*ent).r.currentOrigin[0 as libc::c_int as usize];
    (*ent).s.pos.trBase[1 as libc::c_int as usize] =
        (*ent).r.currentOrigin[1 as libc::c_int as usize];
    (*ent).s.pos.trBase[2 as libc::c_int as usize] =
        (*ent).r.currentOrigin[2 as libc::c_int as usize];
    (*ent).s.pos.trTime = crate::src::game::g_main::level.time;
}
/*
================
G_RunItem

================
*/
#[no_mangle]

pub unsafe extern "C" fn G_RunItem(mut ent: *mut crate::g_local_h::gentity_t) {
    let mut origin: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut tr: crate::src::qcommon::q_shared::trace_t = crate::src::qcommon::q_shared::trace_t {
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
    let mut contents: libc::c_int = 0;
    let mut mask: libc::c_int = 0;
    // if its groundentity has been set to none, it may have been pushed off an edge
    if (*ent).s.groundEntityNum == ((1 as libc::c_int) << 10 as libc::c_int) - 1 as libc::c_int {
        if (*ent).s.pos.trType as libc::c_uint
            != crate::src::qcommon::q_shared::TR_GRAVITY as libc::c_int as libc::c_uint
        {
            (*ent).s.pos.trType = crate::src::qcommon::q_shared::TR_GRAVITY;
            (*ent).s.pos.trTime = crate::src::game::g_main::level.time
        }
    }
    if (*ent).s.pos.trType as libc::c_uint
        == crate::src::qcommon::q_shared::TR_STATIONARY as libc::c_int as libc::c_uint
    {
        // check think function
        crate::src::game::g_main::G_RunThink(ent as *mut crate::g_local_h::gentity_s);
        return;
    }
    // get current position
    crate::src::game::bg_misc::BG_EvaluateTrajectory(
        &mut (*ent).s.pos as *mut _ as *const crate::src::qcommon::q_shared::trajectory_t,
        crate::src::game::g_main::level.time,
        origin.as_mut_ptr(),
    );
    // trace a line from the previous position to the current position
    if (*ent).clipmask != 0 {
        mask = (*ent).clipmask
    } else {
        mask = (1 as libc::c_int | 0x10000 as libc::c_int | 0x2000000 as libc::c_int)
            & !(0x2000000 as libc::c_int)
        //MASK_SOLID;
    } // FIXME: avoid this for stationary?
    crate::src::game::g_syscalls::trap_Trace(
        &mut tr as *mut _ as *mut crate::src::qcommon::q_shared::trace_t,
        (*ent).r.currentOrigin.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        (*ent).r.mins.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        (*ent).r.maxs.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        origin.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        (*ent).r.ownerNum,
        mask,
    );
    (*ent).r.currentOrigin[0 as libc::c_int as usize] = tr.endpos[0 as libc::c_int as usize];
    (*ent).r.currentOrigin[1 as libc::c_int as usize] = tr.endpos[1 as libc::c_int as usize];
    (*ent).r.currentOrigin[2 as libc::c_int as usize] = tr.endpos[2 as libc::c_int as usize];
    if tr.startsolid as u64 != 0 {
        tr.fraction = 0 as libc::c_int as libc::c_float
    }
    crate::src::game::g_syscalls::trap_LinkEntity(ent as *mut crate::g_local_h::gentity_s);
    // check think function
    crate::src::game::g_main::G_RunThink(ent as *mut crate::g_local_h::gentity_s);
    if tr.fraction == 1 as libc::c_int as libc::c_float {
        return;
    }
    // if it is in a nodrop volume, remove it
    contents = crate::src::game::g_syscalls::trap_PointContents(
        (*ent).r.currentOrigin.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        -(1 as libc::c_int),
    );
    if contents as libc::c_uint & 0x80000000 as libc::c_uint != 0 {
        if !(*ent).item.is_null()
            && (*(*ent).item).giType as libc::c_uint
                == crate::bg_public_h::IT_TEAM as libc::c_int as libc::c_uint
        {
            crate::src::game::g_team::Team_FreeEntity(ent as *mut crate::g_local_h::gentity_s);
        } else {
            crate::src::game::g_utils::G_FreeEntity(ent as *mut crate::g_local_h::gentity_s);
        }
        return;
    }
    G_BounceItem(ent, &mut tr);
}

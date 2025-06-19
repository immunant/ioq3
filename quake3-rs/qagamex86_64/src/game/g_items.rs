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

pub unsafe extern "C" fn Pickup_Powerup(mut ent: *mut gentity_t, mut other: *mut gentity_t) -> i32 {
    let mut quantity: i32 = 0;
    let mut i: i32 = 0;
    let mut client: *mut gclient_t = 0 as *mut gclient_t;
    if (*(*other).client).ps.powerups[(*(*ent).item).giTag as usize] == 0 {
        // round timing to seconds to make multiple powerup timers
        // count in sync
        (*(*other).client).ps.powerups[(*(*ent).item).giTag as usize] =
            level.time - level.time % 1000 as i32
    }
    if (*ent).count != 0 {
        quantity = (*ent).count
    } else {
        quantity = (*(*ent).item).quantity
    }
    (*(*other).client).ps.powerups[(*(*ent).item).giTag as usize] += quantity * 1000 as i32;
    // give any nearby players a "denied" anti-reward
    i = 0 as i32;
    while i < level.maxclients {
        let mut delta: vec3_t = [0.; 3];
        let mut len: f32 = 0.;
        let mut forward: vec3_t = [0.; 3];
        let mut tr: trace_t = trace_t {
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
            surfaceFlags: 0,
            contents: 0,
            entityNum: 0,
        };
        client = &mut *level.clients.offset(i as isize) as *mut gclient_s;
        if !(client == (*other).client) {
            if !((*client).pers.connected as u32 == CON_DISCONNECTED as i32 as u32) {
                if !((*client).ps.stats[STAT_HEALTH as i32 as usize] <= 0 as i32) {
                    // if same team in team game, no sound
                    // cannot use OnSameTeam as it expects to g_entities, not clients
                    if !(g_gametype.integer >= GT_TEAM as i32
                        && (*(*other).client).sess.sessionTeam as u32
                            == (*client).sess.sessionTeam as u32)
                    {
                        // if too far away, no sound
                        delta[0 as i32 as usize] = (*ent).s.pos.trBase[0 as i32 as usize]
                            - (*client).ps.origin[0 as i32 as usize];
                        delta[1 as i32 as usize] = (*ent).s.pos.trBase[1 as i32 as usize]
                            - (*client).ps.origin[1 as i32 as usize];
                        delta[2 as i32 as usize] = (*ent).s.pos.trBase[2 as i32 as usize]
                            - (*client).ps.origin[2 as i32 as usize];
                        len = VectorNormalize(delta.as_mut_ptr());
                        if !(len > 192 as i32 as f32) {
                            // if not facing, no sound
                            AngleVectors(
                                (*client).ps.viewangles.as_mut_ptr() as *const vec_t,
                                forward.as_mut_ptr(),
                                0 as *mut vec_t,
                                0 as *mut vec_t,
                            );
                            if !(((delta[0 as i32 as usize] * forward[0 as i32 as usize]
                                + delta[1 as i32 as usize] * forward[1 as i32 as usize]
                                + delta[2 as i32 as usize] * forward[2 as i32 as usize])
                                as f64)
                                < 0.4f64)
                            {
                                // if not line of sight, no sound
                                trap_Trace(
                                    &mut tr as *mut _ as *mut trace_t,
                                    (*client).ps.origin.as_mut_ptr() as *const vec_t,
                                    0 as *const vec_t,
                                    0 as *const vec_t,
                                    (*ent).s.pos.trBase.as_mut_ptr() as *const vec_t,
                                    ((1 as i32) << 10 as i32) - 1 as i32,
                                    1 as i32,
                                );
                                if !(tr.fraction as f64 != 1.0f64) {
                                    // anti-reward
                                    (*client).ps.persistant[PERS_PLAYEREVENTS as i32 as usize] ^=
                                        0x1 as i32
                                }
                            }
                        }
                    }
                }
            }
        }
        i += 1
    }
    return 120 as i32;
}
//======================================================================
#[no_mangle]

pub unsafe extern "C" fn Pickup_Holdable(
    mut ent: *mut gentity_t,
    mut other: *mut gentity_t,
) -> i32 {
    (*(*other).client).ps.stats[STAT_HOLDABLE_ITEM as i32 as usize] =
        (*ent).item.offset_from(bg_itemlist.as_mut_ptr()) as isize as i32;
    if (*(*ent).item).giTag == HI_KAMIKAZE as i32 {
        (*(*other).client).ps.eFlags |= 0x200 as i32
    }
    return 60 as i32;
}
//======================================================================
#[no_mangle]

pub unsafe extern "C" fn Add_Ammo(mut ent: *mut gentity_t, mut weapon: i32, mut count: i32) {
    (*(*ent).client).ps.ammo[weapon as usize] += count;
    if (*(*ent).client).ps.ammo[weapon as usize] > 200 as i32 {
        (*(*ent).client).ps.ammo[weapon as usize] = 200 as i32
    };
}
#[no_mangle]

pub unsafe extern "C" fn Pickup_Ammo(mut ent: *mut gentity_t, mut other: *mut gentity_t) -> i32 {
    let mut quantity: i32 = 0;
    if (*ent).count != 0 {
        quantity = (*ent).count
    } else {
        quantity = (*(*ent).item).quantity
    }
    Add_Ammo(other, (*(*ent).item).giTag, quantity);
    return 40 as i32;
}
//======================================================================
#[no_mangle]

pub unsafe extern "C" fn Pickup_Weapon(mut ent: *mut gentity_t, mut other: *mut gentity_t) -> i32 {
    let mut quantity: i32 = 0;
    if (*ent).count < 0 as i32 {
        quantity = 0 as i32
    // None for you, sir!
    } else {
        if (*ent).count != 0 {
            quantity = (*ent).count
        } else {
            quantity = (*(*ent).item).quantity
        }
        // dropped items and teamplay weapons always have full ammo
        if (*ent).flags & 0x1000 as i32 == 0 && g_gametype.integer != GT_TEAM as i32 {
            // respawning rules
            // drop the quantity if the already have over the minimum
            if (*(*other).client).ps.ammo[(*(*ent).item).giTag as usize] < quantity {
                quantity = quantity - (*(*other).client).ps.ammo[(*(*ent).item).giTag as usize]
            } else {
                quantity = 1 as i32
                // only add a single shot
            }
        }
    }
    // add the weapon
    (*(*other).client).ps.stats[STAT_WEAPONS as i32 as usize] |= (1 as i32) << (*(*ent).item).giTag; // unlimited ammo
    Add_Ammo(other, (*(*ent).item).giTag, quantity);
    if (*(*ent).item).giTag == WP_GRAPPLING_HOOK as i32 {
        (*(*other).client).ps.ammo[(*(*ent).item).giTag as usize] = -(1 as i32)
    }
    // team deathmatch has slow weapon respawns
    if g_gametype.integer == GT_TEAM as i32 {
        return g_weaponTeamRespawn.integer;
    }
    return g_weaponRespawn.integer;
}
//======================================================================
#[no_mangle]

pub unsafe extern "C" fn Pickup_Health(mut ent: *mut gentity_t, mut other: *mut gentity_t) -> i32 {
    let mut max: i32 = 0;
    let mut quantity: i32 = 0;
    // small and mega healths will go over the max
    if (*(*ent).item).quantity != 5 as i32 && (*(*ent).item).quantity != 100 as i32 {
        max = (*(*other).client).ps.stats[STAT_MAX_HEALTH as i32 as usize]
    } else {
        max = (*(*other).client).ps.stats[STAT_MAX_HEALTH as i32 as usize] * 2 as i32
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
    (*(*other).client).ps.stats[STAT_HEALTH as i32 as usize] = (*other).health;
    if (*(*ent).item).quantity == 100 as i32 {
        // mega health respawns slow
        return 35 as i32;
    }
    return 35 as i32;
}
//======================================================================
#[no_mangle]

pub unsafe extern "C" fn Pickup_Armor(mut ent: *mut gentity_t, mut other: *mut gentity_t) -> i32 {
    (*(*other).client).ps.stats[STAT_ARMOR as i32 as usize] += (*(*ent).item).quantity;
    if (*(*other).client).ps.stats[STAT_ARMOR as i32 as usize]
        > (*(*other).client).ps.stats[STAT_MAX_HEALTH as i32 as usize] * 2 as i32
    {
        (*(*other).client).ps.stats[STAT_ARMOR as i32 as usize] =
            (*(*other).client).ps.stats[STAT_MAX_HEALTH as i32 as usize] * 2 as i32
    }
    return 25 as i32;
}
//======================================================================
/*
===============
RespawnItem
===============
*/
#[no_mangle]

pub unsafe extern "C" fn RespawnItem(mut ent: *mut gentity_t) {
    if ent.is_null() {
        return;
    }
    // randomly select from teamed entities
    if !(*ent).team.is_null() {
        let mut master: *mut gentity_t = 0 as *mut gentity_t;
        let mut count: i32 = 0;
        let mut choice: i32 = 0;
        if (*ent).teammaster.is_null() {
            G_Error(b"RespawnItem: bad teammaster\x00" as *const u8 as *const libc::c_char);
        }
        master = (*ent).teammaster;
        count = 0 as i32;
        ent = master;
        while !ent.is_null() {
            ent = (*ent).teamchain;
            count += 1
        }
        choice = libc::rand() % count;
        count = 0 as i32;
        ent = master;
        while !ent.is_null() && count < choice {
            ent = (*ent).teamchain;
            count += 1
        }
    }
    if ent.is_null() {
        return;
    }
    (*ent).r.contents = 0x40000000 as i32;
    (*ent).s.eFlags &= !(0x80 as i32);
    (*ent).r.svFlags &= !(0x1 as i32);
    trap_LinkEntity(ent as *mut gentity_s);
    if (*(*ent).item).giType as u32 == IT_POWERUP as i32 as u32 {
        // play powerup spawn sound to all clients
        let mut te: *mut gentity_t = 0 as *mut gentity_t;
        // if the powerup respawn sound should Not be global
        if (*ent).speed != 0. {
            te = G_TempEntity((*ent).s.pos.trBase.as_mut_ptr(), EV_GENERAL_SOUND as i32)
                as *mut gentity_s
        } else {
            te = G_TempEntity((*ent).s.pos.trBase.as_mut_ptr(), EV_GLOBAL_SOUND as i32)
                as *mut gentity_s
        }
        (*te).s.eventParm = G_SoundIndex(
            b"sound/items/poweruprespawn.wav\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        (*te).r.svFlags |= 0x20 as i32
    }
    if (*(*ent).item).giType as u32 == IT_HOLDABLE as i32 as u32
        && (*(*ent).item).giTag == HI_KAMIKAZE as i32
    {
        // play powerup spawn sound to all clients
        let mut te_0: *mut gentity_t = 0 as *mut gentity_t;
        // if the powerup respawn sound should Not be global
        if (*ent).speed != 0. {
            te_0 = G_TempEntity((*ent).s.pos.trBase.as_mut_ptr(), EV_GENERAL_SOUND as i32)
                as *mut gentity_s
        } else {
            te_0 = G_TempEntity((*ent).s.pos.trBase.as_mut_ptr(), EV_GLOBAL_SOUND as i32)
                as *mut gentity_s
        }
        (*te_0).s.eventParm = G_SoundIndex(
            b"sound/items/kamikazerespawn.wav\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        (*te_0).r.svFlags |= 0x20 as i32
    }
    // play the normal respawn sound only to nearby clients
    G_AddEvent(ent as *mut gentity_s, EV_ITEM_RESPAWN as i32, 0 as i32);
    (*ent).nextthink = 0 as i32;
}
/*
===============
Touch_Item
===============
*/
#[no_mangle]

pub unsafe extern "C" fn Touch_Item(
    mut ent: *mut gentity_t,
    mut other: *mut gentity_t,
    mut _trace: *mut trace_t,
) {
    let mut respawn: i32 = 0; // dead people can't pickup
    let mut predict: qboolean = qfalse;
    if (*other).client.is_null() {
        return;
    }
    if (*other).health < 1 as i32 {
        return;
    }
    // the same pickup rules are used for client side and server side
    if BG_CanItemBeGrabbed(
        g_gametype.integer,
        &mut (*ent).s as *mut _ as *const entityState_s,
        &mut (*(*other).client).ps as *mut _ as *const playerState_s,
    ) as u64
        == 0
    {
        return;
    }
    G_LogPrintf(
        b"Item: %i %s\n\x00" as *const u8 as *const libc::c_char,
        (*other).s.number,
        (*(*ent).item).classname,
    );
    predict = (*(*other).client).pers.predictItemPickup;
    // call the item-specific pickup function
    match (*(*ent).item).giType as u32 {
        1 => respawn = Pickup_Weapon(ent, other),
        2 => respawn = Pickup_Ammo(ent, other),
        3 => respawn = Pickup_Armor(ent, other),
        4 => respawn = Pickup_Health(ent, other),
        5 => {
            respawn = Pickup_Powerup(ent, other);
            predict = qfalse
        }
        8 => {
            respawn = crate::src::game::g_team::Pickup_Team(
                ent as *mut gentity_s,
                other as *mut gentity_s,
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
        G_AddPredictableEvent(
            other as *mut gentity_s,
            EV_ITEM_PICKUP as i32,
            (*ent).s.modelindex,
        );
    } else {
        G_AddEvent(
            other as *mut gentity_s,
            EV_ITEM_PICKUP as i32,
            (*ent).s.modelindex,
        );
    }
    // powerup pickups are global broadcasts
    if (*(*ent).item).giType as u32 == IT_POWERUP as i32 as u32
        || (*(*ent).item).giType as u32 == IT_TEAM as i32 as u32
    {
        // if we want the global sound to play
        if (*ent).speed == 0. {
            let mut te: *mut gentity_t = 0 as *mut gentity_t;
            te = G_TempEntity(
                (*ent).s.pos.trBase.as_mut_ptr(),
                EV_GLOBAL_ITEM_PICKUP as i32,
            ) as *mut gentity_s;
            (*te).s.eventParm = (*ent).s.modelindex;
            (*te).r.svFlags |= 0x20 as i32
        } else {
            let mut te_0: *mut gentity_t = 0 as *mut gentity_t;
            te_0 = G_TempEntity(
                (*ent).s.pos.trBase.as_mut_ptr(),
                EV_GLOBAL_ITEM_PICKUP as i32,
            ) as *mut gentity_s;
            (*te_0).s.eventParm = (*ent).s.modelindex;
            // only send this temp entity to a single client
            (*te_0).r.svFlags |= 0x100 as i32;
            (*te_0).r.singleClient = (*other).s.number
        }
    }
    // fire item targets
    G_UseTargets(ent as *mut gentity_s, other as *mut gentity_s);
    // wait of -1 will not respawn
    if (*ent).wait == -(1 as i32) as f32 {
        (*ent).r.svFlags |= 0x1 as i32;
        (*ent).s.eFlags |= 0x80 as i32;
        (*ent).r.contents = 0 as i32;
        (*ent).unlinkAfterEvent = qtrue;
        return;
    }
    // non zero wait overrides respawn time
    if (*ent).wait != 0. {
        respawn = (*ent).wait as i32
    }
    // random can be used to vary the respawn time
    if (*ent).random != 0. {
        respawn = (respawn as f64
            + 2.0f64
                * (((libc::rand() & 0x7fff as i32) as f32 / 0x7fff as i32 as f32) as f64 - 0.5f64)
                * (*ent).random as f64) as i32;
        if respawn < 1 as i32 {
            respawn = 1 as i32
        }
    }
    // dropped items will not respawn
    if (*ent).flags & 0x1000 as i32 != 0 {
        (*ent).freeAfterEvent = qtrue
    }
    // picked up items still stay around, they just don't
    // draw anything.  This allows respawnable items
    // to be placed on movers.
    (*ent).r.svFlags |= 0x1 as i32;
    (*ent).s.eFlags |= 0x80 as i32;
    (*ent).r.contents = 0 as i32;
    // ZOID
    // A negative respawn times means to never respawn this item (but don't
    // delete it).  This is used by items that are respawned by third party
    // events such as ctf flags
    if respawn <= 0 as i32 {
        (*ent).nextthink = 0 as i32;
        (*ent).think = None
    } else {
        (*ent).nextthink = level.time + respawn * 1000 as i32;
        (*ent).think = Some(RespawnItem as unsafe extern "C" fn(_: *mut gentity_t) -> ())
    }
    trap_LinkEntity(ent as *mut gentity_s);
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
    mut item: *mut gitem_t,
    mut origin: *mut vec_t,
    mut velocity: *mut vec_t,
) -> *mut gentity_t {
    let mut dropped: *mut gentity_t = 0 as *mut gentity_t; // store item number in modelindex
    dropped = G_Spawn() as *mut gentity_s; // This is non-zero is it's a dropped item
    (*dropped).s.eType = ET_ITEM as i32; // auto-remove after 30 seconds
    (*dropped).s.modelindex = item.offset_from(bg_itemlist.as_mut_ptr()) as isize as i32;
    (*dropped).s.modelindex2 = 1 as i32;
    (*dropped).classname = (*item).classname;
    (*dropped).item = item;
    (*dropped).r.mins[0 as i32 as usize] = -(15 as i32) as vec_t;
    (*dropped).r.mins[1 as i32 as usize] = -(15 as i32) as vec_t;
    (*dropped).r.mins[2 as i32 as usize] = -(15 as i32) as vec_t;
    (*dropped).r.maxs[0 as i32 as usize] = 15 as i32 as vec_t;
    (*dropped).r.maxs[1 as i32 as usize] = 15 as i32 as vec_t;
    (*dropped).r.maxs[2 as i32 as usize] = 15 as i32 as vec_t;
    (*dropped).r.contents = 0x40000000 as i32;
    (*dropped).touch = Some(
        Touch_Item
            as unsafe extern "C" fn(_: *mut gentity_t, _: *mut gentity_t, _: *mut trace_t) -> (),
    );
    G_SetOrigin(dropped as *mut gentity_s, origin);
    (*dropped).s.pos.trType = TR_GRAVITY;
    (*dropped).s.pos.trTime = level.time;
    (*dropped).s.pos.trDelta[0 as i32 as usize] = *velocity.offset(0 as i32 as isize);
    (*dropped).s.pos.trDelta[1 as i32 as usize] = *velocity.offset(1 as i32 as isize);
    (*dropped).s.pos.trDelta[2 as i32 as usize] = *velocity.offset(2 as i32 as isize);
    (*dropped).s.eFlags |= 0x20 as i32;
    if g_gametype.integer == GT_CTF as i32 && (*item).giType as u32 == IT_TEAM as i32 as u32 {
        // Special case for CTF flags
        (*dropped).think = Some(
            crate::src::game::g_team::Team_DroppedFlagThink
                as unsafe extern "C" fn(_: *mut gentity_t) -> (),
        );
        (*dropped).nextthink = level.time + 30000 as i32;
        Team_CheckDroppedItem(dropped as *mut gentity_s);
    } else {
        (*dropped).think = Some(G_FreeEntity as unsafe extern "C" fn(_: *mut gentity_t) -> ());
        (*dropped).nextthink = level.time + 30000 as i32
    }
    (*dropped).flags = 0x1000 as i32;
    trap_LinkEntity(dropped as *mut gentity_s);
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
    mut ent: *mut gentity_t,
    mut item: *mut gitem_t,
    mut angle: f32,
) -> *mut gentity_t {
    let mut velocity: vec3_t = [0.; 3]; // always forward
    let mut angles: vec3_t = [0.; 3];
    angles[0 as i32 as usize] = (*ent).s.apos.trBase[0 as i32 as usize];
    angles[1 as i32 as usize] = (*ent).s.apos.trBase[1 as i32 as usize];
    angles[2 as i32 as usize] = (*ent).s.apos.trBase[2 as i32 as usize];
    angles[1 as i32 as usize] += angle;
    angles[0 as i32 as usize] = 0 as i32 as vec_t;
    AngleVectors(
        angles.as_mut_ptr() as *const vec_t,
        velocity.as_mut_ptr(),
        0 as *mut vec_t,
        0 as *mut vec_t,
    );
    velocity[0 as i32 as usize] = velocity[0 as i32 as usize] * 150 as i32 as f32;
    velocity[1 as i32 as usize] = velocity[1 as i32 as usize] * 150 as i32 as f32;
    velocity[2 as i32 as usize] = velocity[2 as i32 as usize] * 150 as i32 as f32;
    velocity[2 as i32 as usize] = (velocity[2 as i32 as usize] as f64
        + (200 as i32 as f64
            + 2.0f64
                * (((libc::rand() & 0x7fff as i32) as f32 / 0x7fff as i32 as f32) as f64 - 0.5f64)
                * 50 as i32 as f64)) as vec_t;
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
    mut ent: *mut gentity_t,
    mut _other: *mut gentity_t,
    mut _activator: *mut gentity_t,
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

pub unsafe extern "C" fn FinishSpawningItem(mut ent: *mut gentity_t) {
    let mut tr: trace_t = trace_t {
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
        surfaceFlags: 0,
        contents: 0,
        entityNum: 0,
    }; // store item number in modelindex
    let mut dest: vec3_t = [0.; 3]; // zero indicates this isn't a dropped item
    (*ent).r.mins[0 as i32 as usize] = -(15 as i32) as vec_t;
    (*ent).r.mins[1 as i32 as usize] = -(15 as i32) as vec_t;
    (*ent).r.mins[2 as i32 as usize] = -(15 as i32) as vec_t;
    (*ent).r.maxs[0 as i32 as usize] = 15 as i32 as vec_t;
    (*ent).r.maxs[1 as i32 as usize] = 15 as i32 as vec_t;
    (*ent).r.maxs[2 as i32 as usize] = 15 as i32 as vec_t;
    (*ent).s.eType = ET_ITEM as i32;
    (*ent).s.modelindex = (*ent).item.offset_from(bg_itemlist.as_mut_ptr()) as isize as i32;
    (*ent).s.modelindex2 = 0 as i32;
    (*ent).r.contents = 0x40000000 as i32;
    (*ent).touch = Some(
        Touch_Item
            as unsafe extern "C" fn(_: *mut gentity_t, _: *mut gentity_t, _: *mut trace_t) -> (),
    );
    // using an item causes it to respawn
    (*ent).use_0 = Some(
        Use_Item
            as unsafe extern "C" fn(_: *mut gentity_t, _: *mut gentity_t, _: *mut gentity_t) -> (),
    );
    if (*ent).spawnflags & 1 as i32 != 0 {
        // suspended
        G_SetOrigin(ent as *mut gentity_s, (*ent).s.origin.as_mut_ptr());
    } else {
        // drop to floor
        dest[0 as i32 as usize] = (*ent).s.origin[0 as i32 as usize];
        dest[1 as i32 as usize] = (*ent).s.origin[1 as i32 as usize];
        dest[2 as i32 as usize] = (*ent).s.origin[2 as i32 as usize] - 4096 as i32 as f32;
        trap_Trace(
            &mut tr as *mut _ as *mut trace_t,
            (*ent).s.origin.as_mut_ptr() as *const vec_t,
            (*ent).r.mins.as_mut_ptr() as *const vec_t,
            (*ent).r.maxs.as_mut_ptr() as *const vec_t,
            dest.as_mut_ptr() as *const vec_t,
            (*ent).s.number,
            1 as i32,
        );
        if tr.startsolid as u64 != 0 {
            G_Printf(
                b"FinishSpawningItem: %s startsolid at %s\n\x00" as *const u8
                    as *const libc::c_char,
                (*ent).classname,
                vtos((*ent).s.origin.as_mut_ptr() as *const vec_t),
            );
            G_FreeEntity(ent as *mut gentity_s);
            return;
        }
        // allow to ride movers
        (*ent).s.groundEntityNum = tr.entityNum;
        G_SetOrigin(ent as *mut gentity_s, tr.endpos.as_mut_ptr());
    }
    // team slaves and targeted items aren't present at start
    if (*ent).flags & 0x400 as i32 != 0 || !(*ent).targetname.is_null() {
        (*ent).s.eFlags |= 0x80 as i32;
        (*ent).r.contents = 0 as i32;
        return;
    }
    // powerups don't spawn in for a while
    if (*(*ent).item).giType as u32 == IT_POWERUP as i32 as u32 {
        let mut respawn: f32 = 0.;
        respawn = (45 as i32 as f64
            + 2.0f64
                * (((libc::rand() & 0x7fff as i32) as f32 / 0x7fff as i32 as f32) as f64 - 0.5f64)
                * 15 as i32 as f64) as f32;
        (*ent).s.eFlags |= 0x80 as i32;
        (*ent).r.contents = 0 as i32;
        (*ent).nextthink = (level.time as f32 + respawn * 1000 as i32 as f32) as i32;
        (*ent).think = Some(RespawnItem as unsafe extern "C" fn(_: *mut gentity_t) -> ());
        return;
    }
    trap_LinkEntity(ent as *mut gentity_s);
}
#[no_mangle]

pub static mut itemRegistered: [qboolean; 256] = [qfalse; 256];
/*
==================
G_CheckTeamItems
==================
*/
#[no_mangle]

pub unsafe extern "C" fn G_CheckTeamItems() {
    // Set up team stuff
    crate::src::game::g_team::Team_InitGame();
    if g_gametype.integer == GT_CTF as i32 {
        let mut item: *mut gitem_t = 0 as *mut gitem_t;
        // check for the two flags
        item = BG_FindItem(b"Red Flag\x00" as *const u8 as *const libc::c_char) as *mut gitem_s;
        if item.is_null()
            || itemRegistered[item.offset_from(bg_itemlist.as_mut_ptr()) as isize as usize] as u64
                == 0
        {
            G_Printf(
                b"^3WARNING: No team_CTF_redflag in map\n\x00" as *const u8 as *const libc::c_char,
            );
        }
        item = BG_FindItem(b"Blue Flag\x00" as *const u8 as *const libc::c_char) as *mut gitem_s;
        if item.is_null()
            || itemRegistered[item.offset_from(bg_itemlist.as_mut_ptr()) as isize as usize] as u64
                == 0
        {
            G_Printf(
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
        0 as i32,
        ::std::mem::size_of::<[qboolean; 256]>() as usize,
    );
    // players always start with the base weapon
    RegisterItem(BG_FindItemForWeapon(WP_MACHINEGUN) as *mut gitem_s);
    RegisterItem(BG_FindItemForWeapon(WP_GAUNTLET) as *mut gitem_s);
}
/*
===============
RegisterItem

The item will be added to the precache list
===============
*/
#[no_mangle]

pub unsafe extern "C" fn RegisterItem(mut item: *mut gitem_t) {
    if item.is_null() {
        G_Error(b"RegisterItem: NULL\x00" as *const u8 as *const libc::c_char);
    }
    itemRegistered[item.offset_from(bg_itemlist.as_mut_ptr()) as isize as usize] = qtrue;
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
    let mut i: i32 = 0;
    let mut count: i32 = 0;
    count = 0 as i32;
    i = 0 as i32;
    while i < bg_numItems {
        if itemRegistered[i as usize] as u64 != 0 {
            count += 1;
            string[i as usize] = '1' as i32 as libc::c_char
        } else {
            string[i as usize] = '0' as i32 as libc::c_char
        }
        i += 1
    }
    string[bg_numItems as usize] = 0 as i32 as libc::c_char;
    G_Printf(
        b"%i items registered\n\x00" as *const u8 as *const libc::c_char,
        count,
    );
    trap_SetConfigstring(27 as i32, string.as_mut_ptr());
}
/*
============
G_ItemDisabled
============
*/
#[no_mangle]

pub unsafe extern "C" fn G_ItemDisabled(mut item: *mut gitem_t) -> i32 {
    let mut name: [libc::c_char; 128] = [0; 128];
    Com_sprintf(
        name.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 128]>() as usize as i32,
        b"disable_%s\x00" as *const u8 as *const libc::c_char,
        (*item).classname,
    );
    return trap_Cvar_VariableIntegerValue(name.as_mut_ptr());
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

pub unsafe extern "C" fn G_SpawnItem(mut ent: *mut gentity_t, mut item: *mut gitem_t) {
    G_SpawnFloat(
        b"random\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        &mut (*ent).random,
    );
    G_SpawnFloat(
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
    (*ent).nextthink = level.time + 100 as i32 * 2 as i32; // items are bouncy
    (*ent).think = Some(FinishSpawningItem as unsafe extern "C" fn(_: *mut gentity_t) -> ());
    (*ent).physicsBounce = 0.50f64 as f32;
    if (*item).giType as u32 == IT_POWERUP as i32 as u32 {
        G_SoundIndex(
            b"sound/items/poweruprespawn.wav\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        G_SpawnFloat(
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

pub unsafe extern "C" fn G_BounceItem(mut ent: *mut gentity_t, mut trace: *mut trace_t) {
    let mut velocity: vec3_t = [0.; 3];
    let mut dot: f32 = 0.;
    let mut hitTime: i32 = 0;
    // reflect the velocity on the trace plane
    hitTime = (level.previousTime as f32
        + (level.time - level.previousTime) as f32 * (*trace).fraction) as i32;
    BG_EvaluateTrajectoryDelta(
        &mut (*ent).s.pos as *mut _ as *const trajectory_t,
        hitTime,
        velocity.as_mut_ptr(),
    );
    dot = velocity[0 as i32 as usize] * (*trace).plane.normal[0 as i32 as usize]
        + velocity[1 as i32 as usize] * (*trace).plane.normal[1 as i32 as usize]
        + velocity[2 as i32 as usize] * (*trace).plane.normal[2 as i32 as usize];
    (*ent).s.pos.trDelta[0 as i32 as usize] = velocity[0 as i32 as usize]
        + (*trace).plane.normal[0 as i32 as usize] * (-(2 as i32) as f32 * dot);
    (*ent).s.pos.trDelta[1 as i32 as usize] = velocity[1 as i32 as usize]
        + (*trace).plane.normal[1 as i32 as usize] * (-(2 as i32) as f32 * dot);
    (*ent).s.pos.trDelta[2 as i32 as usize] = velocity[2 as i32 as usize]
        + (*trace).plane.normal[2 as i32 as usize] * (-(2 as i32) as f32 * dot);
    // cut the velocity to keep from bouncing forever
    (*ent).s.pos.trDelta[0 as i32 as usize] =
        (*ent).s.pos.trDelta[0 as i32 as usize] * (*ent).physicsBounce;
    (*ent).s.pos.trDelta[1 as i32 as usize] =
        (*ent).s.pos.trDelta[1 as i32 as usize] * (*ent).physicsBounce;
    (*ent).s.pos.trDelta[2 as i32 as usize] =
        (*ent).s.pos.trDelta[2 as i32 as usize] * (*ent).physicsBounce;
    // check for stop
    if (*trace).plane.normal[2 as i32 as usize] > 0 as i32 as f32
        && (*ent).s.pos.trDelta[2 as i32 as usize] < 40 as i32 as f32
    {
        (*trace).endpos[2 as i32 as usize] =
            ((*trace).endpos[2 as i32 as usize] as f64 + 1.0f64) as vec_t; // make sure it is off ground
        (*trace).endpos[0 as i32 as usize] = (*trace).endpos[0 as i32 as usize] as i32 as vec_t;
        (*trace).endpos[1 as i32 as usize] = (*trace).endpos[1 as i32 as usize] as i32 as vec_t;
        (*trace).endpos[2 as i32 as usize] = (*trace).endpos[2 as i32 as usize] as i32 as vec_t;
        G_SetOrigin(ent as *mut gentity_s, (*trace).endpos.as_mut_ptr());
        (*ent).s.groundEntityNum = (*trace).entityNum;
        return;
    }
    (*ent).r.currentOrigin[0 as i32 as usize] =
        (*ent).r.currentOrigin[0 as i32 as usize] + (*trace).plane.normal[0 as i32 as usize];
    (*ent).r.currentOrigin[1 as i32 as usize] =
        (*ent).r.currentOrigin[1 as i32 as usize] + (*trace).plane.normal[1 as i32 as usize];
    (*ent).r.currentOrigin[2 as i32 as usize] =
        (*ent).r.currentOrigin[2 as i32 as usize] + (*trace).plane.normal[2 as i32 as usize];
    (*ent).s.pos.trBase[0 as i32 as usize] = (*ent).r.currentOrigin[0 as i32 as usize];
    (*ent).s.pos.trBase[1 as i32 as usize] = (*ent).r.currentOrigin[1 as i32 as usize];
    (*ent).s.pos.trBase[2 as i32 as usize] = (*ent).r.currentOrigin[2 as i32 as usize];
    (*ent).s.pos.trTime = level.time;
}
/*
================
G_RunItem

================
*/
#[no_mangle]

pub unsafe extern "C" fn G_RunItem(mut ent: *mut gentity_t) {
    let mut origin: vec3_t = [0.; 3];
    let mut tr: trace_t = trace_t {
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
        surfaceFlags: 0,
        contents: 0,
        entityNum: 0,
    };
    let mut contents: i32 = 0;
    let mut mask: i32 = 0;
    // if its groundentity has been set to none, it may have been pushed off an edge
    if (*ent).s.groundEntityNum == ((1 as i32) << 10 as i32) - 1 as i32 {
        if (*ent).s.pos.trType as u32 != TR_GRAVITY as i32 as u32 {
            (*ent).s.pos.trType = TR_GRAVITY;
            (*ent).s.pos.trTime = level.time
        }
    }
    if (*ent).s.pos.trType as u32 == TR_STATIONARY as i32 as u32 {
        // check think function
        G_RunThink(ent as *mut gentity_s);
        return;
    }
    // get current position
    BG_EvaluateTrajectory(
        &mut (*ent).s.pos as *mut _ as *const trajectory_t,
        level.time,
        origin.as_mut_ptr(),
    );
    // trace a line from the previous position to the current position
    if (*ent).clipmask != 0 {
        mask = (*ent).clipmask
    } else {
        mask = (1 as i32 | 0x10000 as i32 | 0x2000000 as i32) & !(0x2000000 as i32)
        //MASK_SOLID;
    } // FIXME: avoid this for stationary?
    trap_Trace(
        &mut tr as *mut _ as *mut trace_t,
        (*ent).r.currentOrigin.as_mut_ptr() as *const vec_t,
        (*ent).r.mins.as_mut_ptr() as *const vec_t,
        (*ent).r.maxs.as_mut_ptr() as *const vec_t,
        origin.as_mut_ptr() as *const vec_t,
        (*ent).r.ownerNum,
        mask,
    );
    (*ent).r.currentOrigin[0 as i32 as usize] = tr.endpos[0 as i32 as usize];
    (*ent).r.currentOrigin[1 as i32 as usize] = tr.endpos[1 as i32 as usize];
    (*ent).r.currentOrigin[2 as i32 as usize] = tr.endpos[2 as i32 as usize];
    if tr.startsolid as u64 != 0 {
        tr.fraction = 0 as i32 as f32
    }
    trap_LinkEntity(ent as *mut gentity_s);
    // check think function
    G_RunThink(ent as *mut gentity_s);
    if tr.fraction == 1 as i32 as f32 {
        return;
    }
    // if it is in a nodrop volume, remove it
    contents = trap_PointContents(
        (*ent).r.currentOrigin.as_mut_ptr() as *const vec_t,
        -(1 as i32),
    );
    if contents as u32 & 0x80000000 as u32 != 0 {
        if !(*ent).item.is_null() && (*(*ent).item).giType as u32 == IT_TEAM as i32 as u32 {
            crate::src::game::g_team::Team_FreeEntity(ent as *mut gentity_s);
        } else {
            G_FreeEntity(ent as *mut gentity_s);
        }
        return;
    }
    G_BounceItem(ent, &mut tr);
}

use ::libc;

pub use crate::bg_public_h::gitem_s;
pub use crate::bg_public_h::gitem_t;
pub use crate::bg_public_h::itemType_t;
pub use crate::bg_public_h::pmove_t;
pub use crate::bg_public_h::powerup_t;
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
pub use crate::src::game::ai_main::BotTestAAS;
pub use crate::src::game::bg_misc::BG_FindItemForPowerup;
pub use crate::src::game::bg_misc::BG_PlayerStateToEntityState;
pub use crate::src::game::bg_misc::BG_PlayerStateToEntityStateExtraPolate;
pub use crate::src::game::bg_misc::BG_PlayerTouchesItem;
pub use crate::src::game::bg_pmove::Pmove;
pub use crate::src::game::g_client::ClientBegin;
pub use crate::src::game::g_client::ClientRespawn;
pub use crate::src::game::g_client::SelectSpawnPoint;
pub use crate::src::game::g_cmds::Cmd_FollowCycle_f;
pub use crate::src::game::g_combat::G_Damage;
pub use crate::src::game::g_items::Drop_Item;
pub use crate::src::game::g_main::g_debugMove;
pub use crate::src::game::g_main::g_dmflags;
pub use crate::src::game::g_main::g_entities;
pub use crate::src::game::g_main::g_forcerespawn;
pub use crate::src::game::g_main::g_gravity;
pub use crate::src::game::g_main::g_inactivity;
pub use crate::src::game::g_main::g_smoothClients;
pub use crate::src::game::g_main::g_speed;
pub use crate::src::game::g_main::g_synchronousClients;
pub use crate::src::game::g_main::level;
pub use crate::src::game::g_main::pmove_fixed;
pub use crate::src::game::g_main::pmove_msec;
pub use crate::src::game::g_misc::TeleportPlayer;
pub use crate::src::game::g_mover::Touch_DoorTrigger;
pub use crate::src::game::g_syscalls::trap_Cvar_Set;
pub use crate::src::game::g_syscalls::trap_Cvar_Update;
pub use crate::src::game::g_syscalls::trap_DropClient;
pub use crate::src::game::g_syscalls::trap_EntitiesInBox;
pub use crate::src::game::g_syscalls::trap_EntityContact;
pub use crate::src::game::g_syscalls::trap_GetUsercmd;
pub use crate::src::game::g_syscalls::trap_LinkEntity;
pub use crate::src::game::g_syscalls::trap_PointContents;
pub use crate::src::game::g_syscalls::trap_SendServerCommand;
pub use crate::src::game::g_syscalls::trap_Trace;
pub use crate::src::game::g_syscalls::trap_UnlinkEntity;
pub use crate::src::game::g_utils::G_AddEvent;
pub use crate::src::game::g_utils::G_TempEntity;
pub use crate::src::game::g_weapon::CheckGauntletAttack;
pub use crate::src::game::g_weapon::FireWeapon;
pub use crate::src::game::g_weapon::Weapon_HookFree;
pub use crate::src::qcommon::q_math::vectoangles;
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
pub use crate::src::qcommon::q_shared::TR_GRAVITY;
pub use crate::src::qcommon::q_shared::TR_INTERPOLATE;
pub use crate::src::qcommon::q_shared::TR_LINEAR;
pub use crate::src::qcommon::q_shared::TR_LINEAR_STOP;
pub use crate::src::qcommon::q_shared::TR_SINE;
pub use crate::src::qcommon::q_shared::TR_STATIONARY;

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
/*
===============
G_DamageFeedback

Called just before a snapshot is sent to the given player.
Totals up all damage and generates both the player_state_t
damage values to that client for pain blends and kicks, and
global pain sound events for all clients.
===============
*/
#[no_mangle]

pub unsafe extern "C" fn P_DamageFeedback(mut player: *mut crate::g_local_h::gentity_t) {
    let mut client: *mut crate::g_local_h::gclient_t = 0 as *mut crate::g_local_h::gclient_t;
    let mut count: f32 = 0.;
    let mut angles: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    client = (*player).client;
    if (*client).ps.pm_type == crate::bg_public_h::PM_DEAD as i32 {
        return;
    }
    // total points of damage shot at the player this frame
    count = ((*client).damage_blood + (*client).damage_armor) as f32;
    if count == 0 as i32 as f32 {
        return;
        // didn't take any damage
    }
    if count > 255 as i32 as f32 {
        count = 255 as i32 as f32
    }
    // send the information to the client
    // world damage (falling, slime, etc) uses a special code
    // to make the blend blob centered instead of positional
    if (*client).damage_fromWorld as u64 != 0 {
        (*client).ps.damagePitch = 255 as i32;
        (*client).ps.damageYaw = 255 as i32;
        (*client).damage_fromWorld = crate::src::qcommon::q_shared::qfalse
    } else {
        crate::src::qcommon::q_math::vectoangles(
            (*client).damage_from.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            angles.as_mut_ptr(),
        );
        (*client).ps.damagePitch =
            (angles[0 as i32 as usize] as f64 / 360.0f64 * 256 as i32 as f64) as i32;
        (*client).ps.damageYaw =
            (angles[1 as i32 as usize] as f64 / 360.0f64 * 256 as i32 as f64) as i32
    }
    // play an appropriate pain sound
    if crate::src::game::g_main::level.time > (*player).pain_debounce_time
        && (*player).flags & 0x10 as i32 == 0
    {
        (*player).pain_debounce_time = crate::src::game::g_main::level.time + 700 as i32;
        crate::src::game::g_utils::G_AddEvent(
            player as *mut crate::g_local_h::gentity_s,
            crate::bg_public_h::EV_PAIN as i32,
            (*player).health,
        );
        (*client).ps.damageEvent += 1
    }
    (*client).ps.damageCount = count as i32;
    //
    // clear totals
    //
    (*client).damage_blood = 0 as i32;
    (*client).damage_armor = 0 as i32;
    (*client).damage_knockback = 0 as i32;
}
/*
=============
P_WorldEffects

Check for lava / slime contents and drowning
=============
*/
#[no_mangle]

pub unsafe extern "C" fn P_WorldEffects(mut ent: *mut crate::g_local_h::gentity_t) {
    let mut envirosuit: crate::src::qcommon::q_shared::qboolean =
        crate::src::qcommon::q_shared::qfalse; // don't need air
    let mut waterlevel: i32 = 0;
    if (*(*ent).client).noclip as u64 != 0 {
        (*(*ent).client).airOutTime = crate::src::game::g_main::level.time + 12000 as i32;
        return;
    }
    waterlevel = (*ent).waterlevel;
    envirosuit = ((*(*ent).client).ps.powerups[crate::bg_public_h::PW_BATTLESUIT as i32 as usize]
        > crate::src::game::g_main::level.time) as i32
        as crate::src::qcommon::q_shared::qboolean;
    //
    // check for drowning
    //
    if waterlevel == 3 as i32 {
        // envirosuit give air
        if envirosuit as u64 != 0 {
            (*(*ent).client).airOutTime = crate::src::game::g_main::level.time + 10000 as i32
        }
        // if out of air, start drowning
        if (*(*ent).client).airOutTime < crate::src::game::g_main::level.time {
            // drown!
            (*(*ent).client).airOutTime += 1000 as i32;
            if (*ent).health > 0 as i32 {
                // take more damage the longer underwater
                (*ent).damage += 2 as i32;
                if (*ent).damage > 15 as i32 {
                    (*ent).damage = 15 as i32
                }
                // don't play a normal pain sound
                (*ent).pain_debounce_time = crate::src::game::g_main::level.time + 200 as i32;
                crate::src::game::g_combat::G_Damage(
                    ent as *mut crate::g_local_h::gentity_s,
                    0 as *mut crate::g_local_h::gentity_t as *mut crate::g_local_h::gentity_s,
                    0 as *mut crate::g_local_h::gentity_t as *mut crate::g_local_h::gentity_s,
                    0 as *mut crate::src::qcommon::q_shared::vec_t,
                    0 as *mut crate::src::qcommon::q_shared::vec_t,
                    (*ent).damage,
                    0x2 as i32,
                    crate::bg_public_h::MOD_WATER as i32,
                );
            }
        }
    } else {
        (*(*ent).client).airOutTime = crate::src::game::g_main::level.time + 12000 as i32;
        (*ent).damage = 2 as i32
    }
    //
    // check for sizzle damage (move to pmove?)
    //
    if waterlevel != 0 && (*ent).watertype & (8 as i32 | 16 as i32) != 0 {
        if (*ent).health > 0 as i32
            && (*ent).pain_debounce_time <= crate::src::game::g_main::level.time
        {
            if envirosuit as u64 != 0 {
                crate::src::game::g_utils::G_AddEvent(
                    ent as *mut crate::g_local_h::gentity_s,
                    crate::bg_public_h::EV_POWERUP_BATTLESUIT as i32,
                    0 as i32,
                );
            } else {
                if (*ent).watertype & 8 as i32 != 0 {
                    crate::src::game::g_combat::G_Damage(
                        ent as *mut crate::g_local_h::gentity_s,
                        0 as *mut crate::g_local_h::gentity_t as *mut crate::g_local_h::gentity_s,
                        0 as *mut crate::g_local_h::gentity_t as *mut crate::g_local_h::gentity_s,
                        0 as *mut crate::src::qcommon::q_shared::vec_t,
                        0 as *mut crate::src::qcommon::q_shared::vec_t,
                        30 as i32 * waterlevel,
                        0 as i32,
                        crate::bg_public_h::MOD_LAVA as i32,
                    );
                }
                if (*ent).watertype & 16 as i32 != 0 {
                    crate::src::game::g_combat::G_Damage(
                        ent as *mut crate::g_local_h::gentity_s,
                        0 as *mut crate::g_local_h::gentity_t as *mut crate::g_local_h::gentity_s,
                        0 as *mut crate::g_local_h::gentity_t as *mut crate::g_local_h::gentity_s,
                        0 as *mut crate::src::qcommon::q_shared::vec_t,
                        0 as *mut crate::src::qcommon::q_shared::vec_t,
                        10 as i32 * waterlevel,
                        0 as i32,
                        crate::bg_public_h::MOD_SLIME as i32,
                    );
                }
            }
        }
    };
}
/*
===============
G_SetClientSound
===============
*/
#[no_mangle]

pub unsafe extern "C" fn G_SetClientSound(mut ent: *mut crate::g_local_h::gentity_t) {
    if (*ent).waterlevel != 0 && (*ent).watertype & (8 as i32 | 16 as i32) != 0 {
        (*(*ent).client).ps.loopSound = crate::src::game::g_main::level.snd_fry
    } else {
        (*(*ent).client).ps.loopSound = 0 as i32
    };
}
//==============================================================
/*
==============
ClientImpacts
==============
*/
#[no_mangle]

pub unsafe extern "C" fn ClientImpacts(
    mut ent: *mut crate::g_local_h::gentity_t,
    mut pm: *mut crate::bg_public_h::pmove_t,
) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut trace: crate::src::qcommon::q_shared::trace_t =
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
    let mut other: *mut crate::g_local_h::gentity_t = 0 as *mut crate::g_local_h::gentity_t;
    crate::stdlib::memset(
        &mut trace as *mut crate::src::qcommon::q_shared::trace_t as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<crate::src::qcommon::q_shared::trace_t>() as libc::c_ulong,
    );
    i = 0 as i32;
    while i < (*pm).numtouch {
        j = 0 as i32;
        while j < i {
            if (*pm).touchents[j as usize] == (*pm).touchents[i as usize] {
                break;
            }
            j += 1
        }
        if !(j != i) {
            other = &mut *crate::src::game::g_main::g_entities
                .as_mut_ptr()
                .offset(*(*pm).touchents.as_mut_ptr().offset(i as isize) as isize)
                as *mut crate::g_local_h::gentity_t;
            if (*ent).r.svFlags & 0x8 as i32 != 0 && (*ent).touch.is_some() {
                (*ent).touch.expect("non-null function pointer")(ent, other, &mut trace);
            }
            if !(*other).touch.is_none() {
                (*other).touch.expect("non-null function pointer")(other, ent, &mut trace);
            }
        }
        i += 1
        // duplicated
    }
}
/*
============
G_TouchTriggers

Find all trigger entities that ent's current position touches.
Spectators will only interact with teleporters.
============
*/
#[no_mangle]

pub unsafe extern "C" fn G_TouchTriggers(mut ent: *mut crate::g_local_h::gentity_t) {
    let mut i: i32 = 0;
    let mut num: i32 = 0;
    let mut touch: [i32; 1024] = [0; 1024];
    let mut hit: *mut crate::g_local_h::gentity_t = 0 as *mut crate::g_local_h::gentity_t;
    let mut trace: crate::src::qcommon::q_shared::trace_t =
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
    let mut mins: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut maxs: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    static mut range: crate::src::qcommon::q_shared::vec3_t = [
        40 as i32 as crate::src::qcommon::q_shared::vec_t,
        40 as i32 as crate::src::qcommon::q_shared::vec_t,
        52 as i32 as crate::src::qcommon::q_shared::vec_t,
    ];
    if (*ent).client.is_null() {
        return;
    }
    // dead clients don't activate triggers!
    if (*(*ent).client).ps.stats[crate::bg_public_h::STAT_HEALTH as i32 as usize] <= 0 as i32 {
        return;
    }
    mins[0 as i32 as usize] =
        (*(*ent).client).ps.origin[0 as i32 as usize] - range[0 as i32 as usize];
    mins[1 as i32 as usize] =
        (*(*ent).client).ps.origin[1 as i32 as usize] - range[1 as i32 as usize];
    mins[2 as i32 as usize] =
        (*(*ent).client).ps.origin[2 as i32 as usize] - range[2 as i32 as usize];
    maxs[0 as i32 as usize] =
        (*(*ent).client).ps.origin[0 as i32 as usize] + range[0 as i32 as usize];
    maxs[1 as i32 as usize] =
        (*(*ent).client).ps.origin[1 as i32 as usize] + range[1 as i32 as usize];
    maxs[2 as i32 as usize] =
        (*(*ent).client).ps.origin[2 as i32 as usize] + range[2 as i32 as usize];
    num = crate::src::game::g_syscalls::trap_EntitiesInBox(
        mins.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        maxs.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        touch.as_mut_ptr(),
        (1 as i32) << 10 as i32,
    );
    // can't use ent->absmin, because that has a one unit pad
    mins[0 as i32 as usize] =
        (*(*ent).client).ps.origin[0 as i32 as usize] + (*ent).r.mins[0 as i32 as usize];
    mins[1 as i32 as usize] =
        (*(*ent).client).ps.origin[1 as i32 as usize] + (*ent).r.mins[1 as i32 as usize];
    mins[2 as i32 as usize] =
        (*(*ent).client).ps.origin[2 as i32 as usize] + (*ent).r.mins[2 as i32 as usize];
    maxs[0 as i32 as usize] =
        (*(*ent).client).ps.origin[0 as i32 as usize] + (*ent).r.maxs[0 as i32 as usize];
    maxs[1 as i32 as usize] =
        (*(*ent).client).ps.origin[1 as i32 as usize] + (*ent).r.maxs[1 as i32 as usize];
    maxs[2 as i32 as usize] =
        (*(*ent).client).ps.origin[2 as i32 as usize] + (*ent).r.maxs[2 as i32 as usize];
    let mut current_block_19: u64;
    i = 0 as i32;
    while i < num {
        hit = &mut *crate::src::game::g_main::g_entities
            .as_mut_ptr()
            .offset(*touch.as_mut_ptr().offset(i as isize) as isize)
            as *mut crate::g_local_h::gentity_t;
        if !((*hit).touch.is_none() && (*ent).touch.is_none()) {
            if !((*hit).r.contents & 0x40000000 as i32 == 0) {
                // ignore most entities if a spectator
                if (*(*ent).client).sess.sessionTeam as u32
                    == crate::bg_public_h::TEAM_SPECTATOR as i32 as u32
                {
                    if (*hit).s.eType != crate::bg_public_h::ET_TELEPORT_TRIGGER as i32
                        && (*hit).touch
                            != Some(
                                crate::src::game::g_mover::Touch_DoorTrigger
                                    as unsafe extern "C" fn(
                                        _: *mut crate::g_local_h::gentity_t,
                                        _: *mut crate::g_local_h::gentity_t,
                                        _: *mut crate::src::qcommon::q_shared::trace_t,
                                    )
                                        -> (),
                            )
                    {
                        current_block_19 = 13586036798005543211;
                    } else {
                        current_block_19 = 2668756484064249700;
                    }
                } else {
                    current_block_19 = 2668756484064249700;
                }
                match current_block_19 {
                    13586036798005543211 => {}
                    _ =>
                    // use separate code for determining if an item is picked up
                    // so you don't have to actually contact its bounding box
                    {
                        if (*hit).s.eType == crate::bg_public_h::ET_ITEM as i32 {
                            if crate::src::game::bg_misc::BG_PlayerTouchesItem(
                                &mut (*(*ent).client).ps as *mut _
                                    as *mut crate::src::qcommon::q_shared::playerState_s,
                                &mut (*hit).s as *mut _
                                    as *mut crate::src::qcommon::q_shared::entityState_s,
                                crate::src::game::g_main::level.time,
                            ) as u64
                                == 0
                            {
                                current_block_19 = 13586036798005543211;
                            } else {
                                current_block_19 = 15345278821338558188;
                            }
                        } else if crate::src::game::g_syscalls::trap_EntityContact(
                            mins.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
                            maxs.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
                            hit as *const crate::g_local_h::gentity_s,
                        ) as u64
                            == 0
                        {
                            current_block_19 = 13586036798005543211;
                        } else {
                            current_block_19 = 15345278821338558188;
                        }
                        match current_block_19 {
                            13586036798005543211 => {}
                            _ => {
                                crate::stdlib::memset(
                                    &mut trace as *mut crate::src::qcommon::q_shared::trace_t
                                        as *mut libc::c_void,
                                    0 as i32,
                                    ::std::mem::size_of::<crate::src::qcommon::q_shared::trace_t>()
                                        as libc::c_ulong,
                                );
                                if (*hit).touch.is_some() {
                                    (*hit).touch.expect("non-null function pointer")(
                                        hit, ent, &mut trace,
                                    );
                                }
                                if (*ent).r.svFlags & 0x8 as i32 != 0 && (*ent).touch.is_some() {
                                    (*ent).touch.expect("non-null function pointer")(
                                        ent, hit, &mut trace,
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
        i += 1
    }
    // if we didn't touch a jump pad this pmove frame
    if (*(*ent).client).ps.jumppad_frame != (*(*ent).client).ps.pmove_framecount {
        (*(*ent).client).ps.jumppad_frame = 0 as i32;
        (*(*ent).client).ps.jumppad_ent = 0 as i32
    };
}
/*
=================
SpectatorThink
=================
*/
#[no_mangle]

pub unsafe extern "C" fn SpectatorThink(
    mut ent: *mut crate::g_local_h::gentity_t,
    mut ucmd: *mut crate::src::qcommon::q_shared::usercmd_t,
) {
    let mut pm: crate::bg_public_h::pmove_t = crate::bg_public_h::pmove_t {
        ps: 0 as *mut crate::src::qcommon::q_shared::playerState_t,
        cmd: crate::src::qcommon::q_shared::usercmd_t {
            serverTime: 0,
            angles: [0; 3],
            buttons: 0,
            weapon: 0,
            forwardmove: 0,
            rightmove: 0,
            upmove: 0,
        },
        tracemask: 0,
        debugLevel: 0,
        noFootsteps: crate::src::qcommon::q_shared::qfalse,
        gauntletHit: crate::src::qcommon::q_shared::qfalse,
        framecount: 0,
        numtouch: 0,
        touchents: [0; 32],
        mins: [0.; 3],
        maxs: [0.; 3],
        watertype: 0,
        waterlevel: 0,
        xyspeed: 0.,
        pmove_fixed: 0,
        pmove_msec: 0,
        trace: None,
        pointcontents: None,
    }; // faster than normal
    let mut client: *mut crate::g_local_h::gclient_t = 0 as *mut crate::g_local_h::gclient_t;
    client = (*ent).client;
    if (*client).sess.spectatorState as u32 != crate::g_local_h::SPECTATOR_FOLLOW as i32 as u32
        || (*client).ps.pm_flags & 4096 as i32 == 0
    {
        if (*client).sess.spectatorState as u32 == crate::g_local_h::SPECTATOR_FREE as i32 as u32 {
            if (*client).noclip as u64 != 0 {
                (*client).ps.pm_type = crate::bg_public_h::PM_NOCLIP as i32
            } else {
                (*client).ps.pm_type = crate::bg_public_h::PM_SPECTATOR as i32
            }
        } else {
            (*client).ps.pm_type = crate::bg_public_h::PM_FREEZE as i32
        }
        (*client).ps.speed = 400 as i32;
        // set up for pmove
        crate::stdlib::memset(
            &mut pm as *mut crate::bg_public_h::pmove_t as *mut libc::c_void,
            0 as i32,
            ::std::mem::size_of::<crate::bg_public_h::pmove_t>() as libc::c_ulong,
        ); // spectators can fly through bodies
        pm.ps = &mut (*client).ps;
        pm.cmd = *ucmd;
        pm.tracemask = (1 as i32 | 0x10000 as i32 | 0x2000000 as i32) & !(0x2000000 as i32);
        pm.trace = Some(
            crate::src::game::g_syscalls::trap_Trace
                as unsafe extern "C" fn(
                    _: *mut crate::src::qcommon::q_shared::trace_t,
                    _: *const crate::src::qcommon::q_shared::vec_t,
                    _: *const crate::src::qcommon::q_shared::vec_t,
                    _: *const crate::src::qcommon::q_shared::vec_t,
                    _: *const crate::src::qcommon::q_shared::vec_t,
                    _: i32,
                    _: i32,
                ) -> (),
        );
        pm.pointcontents = Some(
            crate::src::game::g_syscalls::trap_PointContents
                as unsafe extern "C" fn(
                    _: *const crate::src::qcommon::q_shared::vec_t,
                    _: i32,
                ) -> i32,
        );
        // perform a pmove
        crate::src::game::bg_pmove::Pmove(&mut pm as *mut _ as *mut crate::bg_public_h::pmove_t);
        // save results of pmove
        (*ent).s.origin[0 as i32 as usize] = (*client).ps.origin[0 as i32 as usize];
        (*ent).s.origin[1 as i32 as usize] = (*client).ps.origin[1 as i32 as usize];
        (*ent).s.origin[2 as i32 as usize] = (*client).ps.origin[2 as i32 as usize];
        G_TouchTriggers(ent);
        crate::src::game::g_syscalls::trap_UnlinkEntity(ent as *mut crate::g_local_h::gentity_s);
    }
    (*client).oldbuttons = (*client).buttons;
    (*client).buttons = (*ucmd).buttons;
    // attack button cycles through spectators
    if (*client).buttons & 1 as i32 != 0 && (*client).oldbuttons & 1 as i32 == 0 {
        crate::src::game::g_cmds::Cmd_FollowCycle_f(
            ent as *mut crate::g_local_h::gentity_s,
            1 as i32,
        );
    };
}
/*
=================
ClientInactivityTimer

Returns qfalse if the client is dropped
=================
*/
#[no_mangle]

pub unsafe extern "C" fn ClientInactivityTimer(
    mut client: *mut crate::g_local_h::gclient_t,
) -> crate::src::qcommon::q_shared::qboolean {
    if crate::src::game::g_main::g_inactivity.integer == 0 {
        // give everyone some time, so if the operator sets g_inactivity during
        // gameplay, everyone isn't kicked
        (*client).inactivityTime = crate::src::game::g_main::level.time + 60 as i32 * 1000 as i32;
        (*client).inactivityWarning = crate::src::qcommon::q_shared::qfalse
    } else if (*client).pers.cmd.forwardmove as i32 != 0
        || (*client).pers.cmd.rightmove as i32 != 0
        || (*client).pers.cmd.upmove as i32 != 0
        || (*client).pers.cmd.buttons & 1 as i32 != 0
    {
        (*client).inactivityTime = crate::src::game::g_main::level.time
            + crate::src::game::g_main::g_inactivity.integer * 1000 as i32;
        (*client).inactivityWarning = crate::src::qcommon::q_shared::qfalse
    } else if (*client).pers.localClient as u64 == 0 {
        if crate::src::game::g_main::level.time > (*client).inactivityTime {
            crate::src::game::g_syscalls::trap_DropClient(
                client.offset_from(crate::src::game::g_main::level.clients) as isize as i32,
                b"Dropped due to inactivity\x00" as *const u8 as *const libc::c_char,
            );
            return crate::src::qcommon::q_shared::qfalse;
        }
        if crate::src::game::g_main::level.time > (*client).inactivityTime - 10000 as i32
            && (*client).inactivityWarning as u64 == 0
        {
            (*client).inactivityWarning = crate::src::qcommon::q_shared::qtrue;
            crate::src::game::g_syscalls::trap_SendServerCommand(
                client.offset_from(crate::src::game::g_main::level.clients) as isize as i32,
                b"cp \"Ten seconds until inactivity drop!\n\"\x00" as *const u8
                    as *const libc::c_char,
            );
        }
    }
    return crate::src::qcommon::q_shared::qtrue;
}
/*
==================
ClientTimerActions

Actions that happen once a second
==================
*/
#[no_mangle]

pub unsafe extern "C" fn ClientTimerActions(
    mut ent: *mut crate::g_local_h::gentity_t,
    mut msec: i32,
) {
    let mut client: *mut crate::g_local_h::gclient_t = 0 as *mut crate::g_local_h::gclient_t;
    client = (*ent).client;
    (*client).timeResidual += msec;
    while (*client).timeResidual >= 1000 as i32 {
        (*client).timeResidual -= 1000 as i32;
        // regenerate
        if (*client).ps.powerups[crate::bg_public_h::PW_REGEN as i32 as usize] != 0 {
            if (*ent).health
                < (*client).ps.stats[crate::bg_public_h::STAT_MAX_HEALTH as i32 as usize]
            {
                (*ent).health += 15 as i32;
                if (*ent).health as f64
                    > (*client).ps.stats[crate::bg_public_h::STAT_MAX_HEALTH as i32 as usize] as f64
                        * 1.1f64
                {
                    (*ent).health = ((*client).ps.stats
                        [crate::bg_public_h::STAT_MAX_HEALTH as i32 as usize]
                        as f64
                        * 1.1f64) as i32
                }
                crate::src::game::g_utils::G_AddEvent(
                    ent as *mut crate::g_local_h::gentity_s,
                    crate::bg_public_h::EV_POWERUP_REGEN as i32,
                    0 as i32,
                );
            } else if (*ent).health
                < (*client).ps.stats[crate::bg_public_h::STAT_MAX_HEALTH as i32 as usize] * 2 as i32
            {
                (*ent).health += 5 as i32;
                if (*ent).health
                    > (*client).ps.stats[crate::bg_public_h::STAT_MAX_HEALTH as i32 as usize]
                        * 2 as i32
                {
                    (*ent).health = (*client).ps.stats
                        [crate::bg_public_h::STAT_MAX_HEALTH as i32 as usize]
                        * 2 as i32
                }
                crate::src::game::g_utils::G_AddEvent(
                    ent as *mut crate::g_local_h::gentity_s,
                    crate::bg_public_h::EV_POWERUP_REGEN as i32,
                    0 as i32,
                );
            }
        } else if (*ent).health
            > (*client).ps.stats[crate::bg_public_h::STAT_MAX_HEALTH as i32 as usize]
        {
            (*ent).health -= 1
        }
        // count down health when over max
        // count down armor when over max
        if (*client).ps.stats[crate::bg_public_h::STAT_ARMOR as i32 as usize]
            > (*client).ps.stats[crate::bg_public_h::STAT_MAX_HEALTH as i32 as usize]
        {
            (*client).ps.stats[crate::bg_public_h::STAT_ARMOR as i32 as usize] -= 1
        }
    }
}
/*
====================
ClientIntermissionThink
====================
*/
#[no_mangle]

pub unsafe extern "C" fn ClientIntermissionThink(mut client: *mut crate::g_local_h::gclient_t) {
    (*client).ps.eFlags &= !(0x1000 as i32);
    (*client).ps.eFlags &= !(0x100 as i32);
    // the level will exit when everyone wants to or after timeouts
    // swap and latch button actions
    (*client).oldbuttons = (*client).buttons;
    (*client).buttons = (*client).pers.cmd.buttons;
    if (*client).buttons & (1 as i32 | 4 as i32) & ((*client).oldbuttons ^ (*client).buttons) != 0 {
        // this used to be an ^1 but once a player says ready, it should stick
        (*client).readyToExit = crate::src::qcommon::q_shared::qtrue
    };
}
/*
================
ClientEvents

Events will be passed on to the clients for presentation,
but any server game effects are handled here
================
*/
#[no_mangle]

pub unsafe extern "C" fn ClientEvents(
    mut ent: *mut crate::g_local_h::gentity_t,
    mut oldEventSequence: i32,
) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut event: i32 = 0;
    let mut client: *mut crate::g_local_h::gclient_t = 0 as *mut crate::g_local_h::gclient_t;
    let mut damage: i32 = 0;
    let mut origin: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut angles: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    //	qboolean	fired;
    let mut item: *mut crate::bg_public_h::gitem_t = 0 as *mut crate::bg_public_h::gitem_t; // no normal pain sound
    let mut drop_0: *mut crate::g_local_h::gentity_t = 0 as *mut crate::g_local_h::gentity_t;
    client = (*ent).client;
    if oldEventSequence < (*client).ps.eventSequence - 2 as i32 {
        oldEventSequence = (*client).ps.eventSequence - 2 as i32
    }
    i = oldEventSequence;
    while i < (*client).ps.eventSequence {
        event = (*client).ps.events[(i & 2 as i32 - 1 as i32) as usize];
        match event {
            11 | 12 => {
                if !((*ent).s.eType != crate::bg_public_h::ET_PLAYER as i32) {
                    if !(crate::src::game::g_main::g_dmflags.integer & 8 as i32 != 0) {
                        if event == crate::bg_public_h::EV_FALL_FAR as i32 {
                            damage = 10 as i32
                        } else {
                            damage = 5 as i32
                        }
                        (*ent).pain_debounce_time =
                            crate::src::game::g_main::level.time + 200 as i32;
                        crate::src::game::g_combat::G_Damage(
                            ent as *mut crate::g_local_h::gentity_s,
                            0 as *mut crate::g_local_h::gentity_t
                                as *mut crate::g_local_h::gentity_s,
                            0 as *mut crate::g_local_h::gentity_t
                                as *mut crate::g_local_h::gentity_s,
                            0 as *mut crate::src::qcommon::q_shared::vec_t,
                            0 as *mut crate::src::qcommon::q_shared::vec_t,
                            damage,
                            0 as i32,
                            crate::bg_public_h::MOD_FALLING as i32,
                        );
                    }
                }
            }
            23 => {
                crate::src::game::g_weapon::FireWeapon(ent as *mut crate::g_local_h::gentity_s);
            }
            25 => {
                // teleporter
                // drop flags in CTF
                item = 0 as *mut crate::bg_public_h::gitem_t;
                j = 0 as i32;
                if (*(*ent).client).ps.powerups[crate::bg_public_h::PW_REDFLAG as i32 as usize] != 0
                {
                    item = crate::src::game::bg_misc::BG_FindItemForPowerup(
                        crate::bg_public_h::PW_REDFLAG,
                    ) as *mut crate::bg_public_h::gitem_s;
                    j = crate::bg_public_h::PW_REDFLAG as i32
                } else if (*(*ent).client).ps.powerups
                    [crate::bg_public_h::PW_BLUEFLAG as i32 as usize]
                    != 0
                {
                    item = crate::src::game::bg_misc::BG_FindItemForPowerup(
                        crate::bg_public_h::PW_BLUEFLAG,
                    ) as *mut crate::bg_public_h::gitem_s;
                    j = crate::bg_public_h::PW_BLUEFLAG as i32
                } else if (*(*ent).client).ps.powerups
                    [crate::bg_public_h::PW_NEUTRALFLAG as i32 as usize]
                    != 0
                {
                    item = crate::src::game::bg_misc::BG_FindItemForPowerup(
                        crate::bg_public_h::PW_NEUTRALFLAG,
                    ) as *mut crate::bg_public_h::gitem_s;
                    j = crate::bg_public_h::PW_NEUTRALFLAG as i32
                }
                if !item.is_null() {
                    drop_0 = crate::src::game::g_items::Drop_Item(
                        ent as *mut crate::g_local_h::gentity_s,
                        item as *mut crate::bg_public_h::gitem_s,
                        0 as i32 as f32,
                    ) as *mut crate::g_local_h::gentity_s;
                    // decide how many seconds it has left
                    (*drop_0).count = ((*(*ent).client).ps.powerups[j as usize]
                        - crate::src::game::g_main::level.time)
                        / 1000 as i32;
                    if (*drop_0).count < 1 as i32 {
                        (*drop_0).count = 1 as i32
                    }
                    (*(*ent).client).ps.powerups[j as usize] = 0 as i32
                }

                crate::src::game::g_client::SelectSpawnPoint(
                    (*(*ent).client).ps.origin.as_mut_ptr(),
                    origin.as_mut_ptr(),
                    angles.as_mut_ptr(),
                    crate::src::qcommon::q_shared::qfalse,
                ) as *mut crate::g_local_h::gentity_s;
                crate::src::game::g_misc::TeleportPlayer(
                    ent as *mut crate::g_local_h::gentity_s,
                    origin.as_mut_ptr(),
                    angles.as_mut_ptr(),
                );
            }
            26 => {
                // medkit
                (*ent).health = (*(*ent).client).ps.stats
                    [crate::bg_public_h::STAT_MAX_HEALTH as i32 as usize]
                    + 25 as i32
            }
            _ => {}
        }
        i += 1
    }
}
/*
==============
SendPendingPredictableEvents
==============
*/
#[no_mangle]

pub unsafe extern "C" fn SendPendingPredictableEvents(
    mut ps: *mut crate::src::qcommon::q_shared::playerState_t,
) {
    let mut t: *mut crate::g_local_h::gentity_t = 0 as *mut crate::g_local_h::gentity_t;
    let mut event: i32 = 0;
    let mut seq: i32 = 0;
    let mut extEvent: i32 = 0;
    let mut number: i32 = 0;
    // if there are still events pending
    if (*ps).entityEventSequence < (*ps).eventSequence {
        // create a temporary entity for this event which is sent to everyone
        // except the client who generated the event
        seq = (*ps).entityEventSequence & 2 as i32 - 1 as i32;
        event = (*ps).events[seq as usize] | ((*ps).entityEventSequence & 3 as i32) << 8 as i32;
        // set external event to zero before calling BG_PlayerStateToEntityState
        extEvent = (*ps).externalEvent;
        (*ps).externalEvent = 0 as i32;
        // create temporary entity for event
        t = crate::src::game::g_utils::G_TempEntity((*ps).origin.as_mut_ptr(), event)
            as *mut crate::g_local_h::gentity_s;
        number = (*t).s.number;
        crate::src::game::bg_misc::BG_PlayerStateToEntityState(
            ps as *mut crate::src::qcommon::q_shared::playerState_s,
            &mut (*t).s as *mut _ as *mut crate::src::qcommon::q_shared::entityState_s,
            crate::src::qcommon::q_shared::qtrue,
        );
        (*t).s.number = number;
        (*t).s.eType = crate::bg_public_h::ET_EVENTS as i32 + event;
        (*t).s.eFlags |= 0x10 as i32;
        (*t).s.otherEntityNum = (*ps).clientNum;
        // send to everyone except the client who generated the event
        (*t).r.svFlags |= 0x800 as i32;
        (*t).r.singleClient = (*ps).clientNum;
        // set back external event
        (*ps).externalEvent = extEvent
    };
}
/*
==============
ClientThink

This will be called once for each client frame, which will
usually be a couple times for each server frame on fast clients.

If "g_synchronousClients 1" is set, this will be called exactly
once for each server frame, which makes for smooth demo recording.
==============
*/
#[no_mangle]

pub unsafe extern "C" fn ClientThink_real(mut ent: *mut crate::g_local_h::gentity_t) {
    let mut client: *mut crate::g_local_h::gclient_t = 0 as *mut crate::g_local_h::gclient_t;
    let mut pm: crate::bg_public_h::pmove_t = crate::bg_public_h::pmove_t {
        ps: 0 as *mut crate::src::qcommon::q_shared::playerState_t,
        cmd: crate::src::qcommon::q_shared::usercmd_t {
            serverTime: 0,
            angles: [0; 3],
            buttons: 0,
            weapon: 0,
            forwardmove: 0,
            rightmove: 0,
            upmove: 0,
        },
        tracemask: 0,
        debugLevel: 0,
        noFootsteps: crate::src::qcommon::q_shared::qfalse,
        gauntletHit: crate::src::qcommon::q_shared::qfalse,
        framecount: 0,
        numtouch: 0,
        touchents: [0; 32],
        mins: [0.; 3],
        maxs: [0.; 3],
        watertype: 0,
        waterlevel: 0,
        xyspeed: 0.,
        pmove_fixed: 0,
        pmove_msec: 0,
        trace: None,
        pointcontents: None,
    };
    let mut oldEventSequence: i32 = 0;
    let mut msec: i32 = 0;
    let mut ucmd: *mut crate::src::qcommon::q_shared::usercmd_t =
        0 as *mut crate::src::qcommon::q_shared::usercmd_t;
    client = (*ent).client;
    // don't think if the client is not yet connected (and thus not yet spawned in)
    if (*client).pers.connected as u32 != crate::g_local_h::CON_CONNECTED as i32 as u32 {
        return;
    }
    // mark the time, so the connection sprite can be removed
    ucmd = &mut (*(*ent).client).pers.cmd;
    // sanity check the command time to prevent speedup cheating
    if (*ucmd).serverTime > crate::src::game::g_main::level.time + 200 as i32 {
        (*ucmd).serverTime = crate::src::game::g_main::level.time + 200 as i32
        //		G_Printf("serverTime <<<<<\n" );
    }
    if (*ucmd).serverTime < crate::src::game::g_main::level.time - 1000 as i32 {
        (*ucmd).serverTime = crate::src::game::g_main::level.time - 1000 as i32
        //		G_Printf("serverTime >>>>>\n" );
    }
    msec = (*ucmd).serverTime - (*client).ps.commandTime;
    // following others may result in bad times, but we still want
    // to check for follow toggles
    if msec < 1 as i32
        && (*client).sess.spectatorState as u32 != crate::g_local_h::SPECTATOR_FOLLOW as i32 as u32
    {
        return;
    }
    if msec > 200 as i32 {
        msec = 200 as i32
    }
    if crate::src::game::g_main::pmove_msec.integer < 8 as i32 {
        crate::src::game::g_syscalls::trap_Cvar_Set(
            b"pmove_msec\x00" as *const u8 as *const libc::c_char,
            b"8\x00" as *const u8 as *const libc::c_char,
        );
        crate::src::game::g_syscalls::trap_Cvar_Update(
            &mut crate::src::game::g_main::pmove_msec as *mut _
                as *mut crate::src::qcommon::q_shared::vmCvar_t,
        );
    } else if crate::src::game::g_main::pmove_msec.integer > 33 as i32 {
        crate::src::game::g_syscalls::trap_Cvar_Set(
            b"pmove_msec\x00" as *const u8 as *const libc::c_char,
            b"33\x00" as *const u8 as *const libc::c_char,
        );
        crate::src::game::g_syscalls::trap_Cvar_Update(
            &mut crate::src::game::g_main::pmove_msec as *mut _
                as *mut crate::src::qcommon::q_shared::vmCvar_t,
        );
    }
    if crate::src::game::g_main::pmove_fixed.integer != 0 || (*client).pers.pmoveFixed as u32 != 0 {
        (*ucmd).serverTime = ((*ucmd).serverTime + crate::src::game::g_main::pmove_msec.integer
            - 1 as i32)
            / crate::src::game::g_main::pmove_msec.integer
            * crate::src::game::g_main::pmove_msec.integer
        //if (ucmd->serverTime - client->ps.commandTime <= 0)
        //	return;
    }
    //
    // check for exiting intermission
    //
    if crate::src::game::g_main::level.intermissiontime != 0 {
        ClientIntermissionThink(client);
        return;
    }
    // spectators don't do much
    if (*client).sess.sessionTeam as u32 == crate::bg_public_h::TEAM_SPECTATOR as i32 as u32 {
        if (*client).sess.spectatorState as u32
            == crate::g_local_h::SPECTATOR_SCOREBOARD as i32 as u32
        {
            return;
        }
        SpectatorThink(ent, ucmd);
        return;
    }
    // check for inactivity timer, but never drop the local client of a non-dedicated server
    if ClientInactivityTimer(client) as u64 == 0 {
        return;
    }
    // clear the rewards if time
    if crate::src::game::g_main::level.time > (*client).rewardTime {
        (*client).ps.eFlags &= !(0x8000 as i32
            | 0x8 as i32
            | 0x40 as i32
            | 0x20000 as i32
            | 0x10000 as i32
            | 0x800 as i32)
    }
    if (*client).noclip as u64 != 0 {
        (*client).ps.pm_type = crate::bg_public_h::PM_NOCLIP as i32
    } else if (*client).ps.stats[crate::bg_public_h::STAT_HEALTH as i32 as usize] <= 0 as i32 {
        (*client).ps.pm_type = crate::bg_public_h::PM_DEAD as i32
    } else {
        (*client).ps.pm_type = crate::bg_public_h::PM_NORMAL as i32
    }
    (*client).ps.gravity = crate::src::game::g_main::g_gravity.value as i32;
    // set speed
    (*client).ps.speed = crate::src::game::g_main::g_speed.value as i32;
    if (*client).ps.powerups[crate::bg_public_h::PW_HASTE as i32 as usize] != 0 {
        (*client).ps.speed = ((*client).ps.speed as f64 * 1.3f64) as i32
    }
    // Let go of the hook if we aren't firing
    if (*client).ps.weapon == crate::bg_public_h::WP_GRAPPLING_HOOK as i32
        && !(*client).hook.is_null()
        && (*ucmd).buttons & 1 as i32 == 0
    {
        crate::src::game::g_weapon::Weapon_HookFree(
            (*client).hook as *mut crate::g_local_h::gentity_s,
        );
    }
    // set up for pmove
    oldEventSequence = (*client).ps.eventSequence;
    crate::stdlib::memset(
        &mut pm as *mut crate::bg_public_h::pmove_t as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<crate::bg_public_h::pmove_t>() as libc::c_ulong,
    );
    // check for the hit-scan gauntlet, don't let the action
    // go through as an attack unless it actually hits something
    if (*client).ps.weapon == crate::bg_public_h::WP_GAUNTLET as i32
        && (*ucmd).buttons & 2 as i32 == 0
        && (*ucmd).buttons & 1 as i32 != 0
        && (*client).ps.weaponTime <= 0 as i32
    {
        pm.gauntletHit =
            crate::src::game::g_weapon::CheckGauntletAttack(ent as *mut crate::g_local_h::gentity_s)
    }
    if (*ent).flags & 0x8000 as i32 != 0 {
        (*ent).flags &= !(0x8000 as i32);
        (*(*ent).client).pers.cmd.buttons |= 8 as i32
    }
    pm.ps = &mut (*client).ps;
    pm.cmd = *ucmd;
    if (*pm.ps).pm_type == crate::bg_public_h::PM_DEAD as i32 {
        pm.tracemask = (1 as i32 | 0x10000 as i32 | 0x2000000 as i32) & !(0x2000000 as i32)
    } else if (*ent).r.svFlags & 0x8 as i32 != 0 {
        pm.tracemask = 1 as i32 | 0x10000 as i32 | 0x2000000 as i32 | 0x400000 as i32
    } else {
        pm.tracemask = 1 as i32 | 0x10000 as i32 | 0x2000000 as i32
    }
    pm.trace = Some(
        crate::src::game::g_syscalls::trap_Trace
            as unsafe extern "C" fn(
                _: *mut crate::src::qcommon::q_shared::trace_t,
                _: *const crate::src::qcommon::q_shared::vec_t,
                _: *const crate::src::qcommon::q_shared::vec_t,
                _: *const crate::src::qcommon::q_shared::vec_t,
                _: *const crate::src::qcommon::q_shared::vec_t,
                _: i32,
                _: i32,
            ) -> (),
    );
    pm.pointcontents = Some(
        crate::src::game::g_syscalls::trap_PointContents
            as unsafe extern "C" fn(_: *const crate::src::qcommon::q_shared::vec_t, _: i32) -> i32,
    );
    pm.debugLevel = crate::src::game::g_main::g_debugMove.integer;
    pm.noFootsteps = (crate::src::game::g_main::g_dmflags.integer & 32 as i32 > 0 as i32) as i32
        as crate::src::qcommon::q_shared::qboolean;
    pm.pmove_fixed = (crate::src::game::g_main::pmove_fixed.integer as u32
        | (*client).pers.pmoveFixed as u32) as i32;
    pm.pmove_msec = crate::src::game::g_main::pmove_msec.integer;
    (*client).oldOrigin[0 as i32 as usize] = (*client).ps.origin[0 as i32 as usize];
    (*client).oldOrigin[1 as i32 as usize] = (*client).ps.origin[1 as i32 as usize];
    (*client).oldOrigin[2 as i32 as usize] = (*client).ps.origin[2 as i32 as usize];
    crate::src::game::bg_pmove::Pmove(&mut pm as *mut _ as *mut crate::bg_public_h::pmove_t);
    // save results of pmove
    if (*(*ent).client).ps.eventSequence != oldEventSequence {
        (*ent).eventTime = crate::src::game::g_main::level.time
    }
    if crate::src::game::g_main::g_smoothClients.integer != 0 {
        crate::src::game::bg_misc::BG_PlayerStateToEntityStateExtraPolate(
            &mut (*(*ent).client).ps as *mut _ as *mut crate::src::qcommon::q_shared::playerState_s,
            &mut (*ent).s as *mut _ as *mut crate::src::qcommon::q_shared::entityState_s,
            (*(*ent).client).ps.commandTime,
            crate::src::qcommon::q_shared::qtrue,
        );
    } else {
        crate::src::game::bg_misc::BG_PlayerStateToEntityState(
            &mut (*(*ent).client).ps as *mut _ as *mut crate::src::qcommon::q_shared::playerState_s,
            &mut (*ent).s as *mut _ as *mut crate::src::qcommon::q_shared::entityState_s,
            crate::src::qcommon::q_shared::qtrue,
        );
    }
    SendPendingPredictableEvents(&mut (*(*ent).client).ps);
    if (*(*ent).client).ps.eFlags & 0x100 as i32 == 0 {
        (*client).fireHeld = crate::src::qcommon::q_shared::qfalse
        // for grapple
    }
    // use the snapped origin for linking so it matches client predicted versions
    (*ent).r.currentOrigin[0 as i32 as usize] = (*ent).s.pos.trBase[0 as i32 as usize];
    (*ent).r.currentOrigin[1 as i32 as usize] = (*ent).s.pos.trBase[1 as i32 as usize];
    (*ent).r.currentOrigin[2 as i32 as usize] = (*ent).s.pos.trBase[2 as i32 as usize];
    (*ent).r.mins[0 as i32 as usize] = pm.mins[0 as i32 as usize];
    (*ent).r.mins[1 as i32 as usize] = pm.mins[1 as i32 as usize];
    (*ent).r.mins[2 as i32 as usize] = pm.mins[2 as i32 as usize];
    (*ent).r.maxs[0 as i32 as usize] = pm.maxs[0 as i32 as usize];
    (*ent).r.maxs[1 as i32 as usize] = pm.maxs[1 as i32 as usize];
    (*ent).r.maxs[2 as i32 as usize] = pm.maxs[2 as i32 as usize];
    (*ent).waterlevel = pm.waterlevel;
    (*ent).watertype = pm.watertype;
    // execute client events
    ClientEvents(ent, oldEventSequence);
    // link entity now, after any personal teleporters have been used
    crate::src::game::g_syscalls::trap_LinkEntity(ent as *mut crate::g_local_h::gentity_s);
    if (*(*ent).client).noclip as u64 == 0 {
        G_TouchTriggers(ent);
    }
    // NOTE: now copy the exact origin over otherwise clients can be snapped into solid
    (*ent).r.currentOrigin[0 as i32 as usize] = (*(*ent).client).ps.origin[0 as i32 as usize];
    (*ent).r.currentOrigin[1 as i32 as usize] = (*(*ent).client).ps.origin[1 as i32 as usize];
    (*ent).r.currentOrigin[2 as i32 as usize] = (*(*ent).client).ps.origin[2 as i32 as usize];
    //test for solid areas in the AAS file
    crate::src::game::ai_main::BotTestAAS((*ent).r.currentOrigin.as_mut_ptr());
    // touch other objects
    ClientImpacts(ent, &mut pm);
    // save results of triggers and client events
    if (*(*ent).client).ps.eventSequence != oldEventSequence {
        (*ent).eventTime = crate::src::game::g_main::level.time
    }
    // swap and latch button actions
    (*client).oldbuttons = (*client).buttons;
    (*client).buttons = (*ucmd).buttons;
    (*client).latched_buttons |= (*client).buttons & !(*client).oldbuttons;
    // check for respawning
    if (*client).ps.stats[crate::bg_public_h::STAT_HEALTH as i32 as usize] <= 0 as i32 {
        // wait for the attack button to be pressed
        if crate::src::game::g_main::level.time > (*client).respawnTime {
            // forcerespawn is to prevent users from waiting out powerups
            if crate::src::game::g_main::g_forcerespawn.integer > 0 as i32
                && crate::src::game::g_main::level.time - (*client).respawnTime
                    > crate::src::game::g_main::g_forcerespawn.integer * 1000 as i32
            {
                crate::src::game::g_client::ClientRespawn(ent as *mut crate::g_local_h::gentity_s);
                return;
            }
            // pressing attack or use is the normal respawn method
            if (*ucmd).buttons & (1 as i32 | 4 as i32) != 0 {
                crate::src::game::g_client::ClientRespawn(ent as *mut crate::g_local_h::gentity_s);
            }
        }
        return;
    }
    // perform once-a-second actions
    ClientTimerActions(ent, msec);
}
/*
==================
ClientThink

A new command has arrived from the client
==================
*/
#[no_mangle]

pub unsafe extern "C" fn ClientThink(mut clientNum: i32) {
    let mut ent: *mut crate::g_local_h::gentity_t = 0 as *mut crate::g_local_h::gentity_t;
    ent = crate::src::game::g_main::g_entities
        .as_mut_ptr()
        .offset(clientNum as isize);
    crate::src::game::g_syscalls::trap_GetUsercmd(
        clientNum,
        &mut (*(*ent).client).pers.cmd as *mut _ as *mut crate::src::qcommon::q_shared::usercmd_s,
    );
    // mark the time we got info, so we can display the
    // phone jack if they don't get any for a while
    (*(*ent).client).lastCmdTime = crate::src::game::g_main::level.time;
    if (*ent).r.svFlags & 0x8 as i32 == 0
        && crate::src::game::g_main::g_synchronousClients.integer == 0
    {
        ClientThink_real(ent);
    };
}
#[no_mangle]

pub unsafe extern "C" fn G_RunClient(mut ent: *mut crate::g_local_h::gentity_t) {
    if (*ent).r.svFlags & 0x8 as i32 == 0
        && crate::src::game::g_main::g_synchronousClients.integer == 0
    {
        return;
    }
    (*(*ent).client).pers.cmd.serverTime = crate::src::game::g_main::level.time;
    ClientThink_real(ent);
}
/*
==================
SpectatorClientEndFrame

==================
*/
#[no_mangle]

pub unsafe extern "C" fn SpectatorClientEndFrame(mut ent: *mut crate::g_local_h::gentity_t) {
    let mut cl: *mut crate::g_local_h::gclient_t = 0 as *mut crate::g_local_h::gclient_t;
    // if we are doing a chase cam or a remote view, grab the latest info
    if (*(*ent).client).sess.spectatorState as u32
        == crate::g_local_h::SPECTATOR_FOLLOW as i32 as u32
    {
        let mut clientNum: i32 = 0;
        let mut flags: i32 = 0;
        clientNum = (*(*ent).client).sess.spectatorClient;
        // team follow1 and team follow2 go to whatever clients are playing
        if clientNum == -(1 as i32) {
            clientNum = crate::src::game::g_main::level.follow1
        } else if clientNum == -(2 as i32) {
            clientNum = crate::src::game::g_main::level.follow2
        }
        if clientNum >= 0 as i32 {
            cl = &mut *crate::src::game::g_main::level
                .clients
                .offset(clientNum as isize) as *mut crate::g_local_h::gclient_s;
            if (*cl).pers.connected as u32 == crate::g_local_h::CON_CONNECTED as i32 as u32
                && (*cl).sess.sessionTeam as u32 != crate::bg_public_h::TEAM_SPECTATOR as i32 as u32
            {
                flags = (*cl).ps.eFlags & !(0x4000 as i32 | 0x80000 as i32)
                    | (*(*ent).client).ps.eFlags & (0x4000 as i32 | 0x80000 as i32);
                (*(*ent).client).ps = (*cl).ps;
                (*(*ent).client).ps.pm_flags |= 4096 as i32;
                (*(*ent).client).ps.eFlags = flags;
                return;
            }
        }
        if (*(*ent).client).ps.pm_flags & 4096 as i32 != 0 {
            // drop them to free spectators unless they are dedicated camera followers
            if (*(*ent).client).sess.spectatorClient >= 0 as i32 {
                (*(*ent).client).sess.spectatorState = crate::g_local_h::SPECTATOR_FREE
            }
            crate::src::game::g_client::ClientBegin(
                (*ent)
                    .client
                    .offset_from(crate::src::game::g_main::level.clients)
                    as isize as i32,
            );
        }
    }
    if (*(*ent).client).sess.spectatorState as u32
        == crate::g_local_h::SPECTATOR_SCOREBOARD as i32 as u32
    {
        (*(*ent).client).ps.pm_flags |= 8192 as i32
    } else {
        (*(*ent).client).ps.pm_flags &= !(8192 as i32)
    };
}
/*
==============
ClientEndFrame

Called at the end of each server frame for each connected client
A fast client will have multiple ClientThink for each ClientEdFrame,
while a slow client may have multiple ClientEndFrame between ClientThink.
==============
*/
#[no_mangle]

pub unsafe extern "C" fn ClientEndFrame(mut ent: *mut crate::g_local_h::gentity_t) {
    let mut i: i32 = 0;
    if (*(*ent).client).sess.sessionTeam as u32 == crate::bg_public_h::TEAM_SPECTATOR as i32 as u32
    {
        SpectatorClientEndFrame(ent);
        return;
    }
    // turn off any expired powerups
    i = 0 as i32;
    while i < 16 as i32 {
        if (*(*ent).client).ps.powerups[i as usize] < crate::src::game::g_main::level.time {
            (*(*ent).client).ps.powerups[i as usize] = 0 as i32
        }
        i += 1
    }
    // save network bandwidth
    //
    // If the end of unit layout is displayed, don't give
    // the player any normal movement attributes
    //
    if crate::src::game::g_main::level.intermissiontime != 0 {
        return;
    }
    // burn from lava, etc
    P_WorldEffects(ent);
    // apply all the damage taken this frame
    P_DamageFeedback(ent);
    // add the EF_CONNECTION flag if we haven't gotten commands recently
    if crate::src::game::g_main::level.time - (*(*ent).client).lastCmdTime > 1000 as i32 {
        (*(*ent).client).ps.eFlags |= 0x2000 as i32
    } else {
        (*(*ent).client).ps.eFlags &= !(0x2000 as i32)
    } // FIXME: get rid of ent->health...
    (*(*ent).client).ps.stats[crate::bg_public_h::STAT_HEALTH as i32 as usize] = (*ent).health;
    G_SetClientSound(ent);
    // set the latest infor
    if crate::src::game::g_main::g_smoothClients.integer != 0 {
        crate::src::game::bg_misc::BG_PlayerStateToEntityStateExtraPolate(
            &mut (*(*ent).client).ps as *mut _ as *mut crate::src::qcommon::q_shared::playerState_s,
            &mut (*ent).s as *mut _ as *mut crate::src::qcommon::q_shared::entityState_s,
            (*(*ent).client).ps.commandTime,
            crate::src::qcommon::q_shared::qtrue,
        );
    } else {
        crate::src::game::bg_misc::BG_PlayerStateToEntityState(
            &mut (*(*ent).client).ps as *mut _ as *mut crate::src::qcommon::q_shared::playerState_s,
            &mut (*ent).s as *mut _ as *mut crate::src::qcommon::q_shared::entityState_s,
            crate::src::qcommon::q_shared::qtrue,
        );
    }
    SendPendingPredictableEvents(&mut (*(*ent).client).ps);
    // set the bit for the reachability area the client is currently in
    //	i = trap_AAS_PointReachabilityAreaIndex( ent->client->ps.origin );
    //	ent->client->areabits[i >> 3] |= 1 << (i & 7);
}

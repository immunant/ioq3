use ::libc;

pub use crate::stdlib::intptr_t;

pub use crate::bg_public_h::gitem_s;
pub use crate::bg_public_h::gitem_t;
pub use crate::bg_public_h::itemType_t;
pub use crate::bg_public_h::team_t;
pub use crate::bg_public_h::C2RustUnnamed_0;
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
pub use crate::g_local_h::clientConnected_t;
pub use crate::g_local_h::clientPersistant_t;
pub use crate::g_local_h::clientSession_t;
pub use crate::g_local_h::gclient_s;
pub use crate::g_local_h::gentity_s;
pub use crate::g_local_h::gentity_t;
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
pub use crate::g_public_h::BOTLIB_AAS_ALTERNATIVE_ROUTE_GOAL;
pub use crate::g_public_h::BOTLIB_AAS_AREA_INFO;
pub use crate::g_public_h::BOTLIB_AAS_AREA_REACHABILITY;
pub use crate::g_public_h::BOTLIB_AAS_AREA_TRAVEL_TIME_TO_GOAL_AREA;
pub use crate::g_public_h::BOTLIB_AAS_BBOX_AREAS;
pub use crate::g_public_h::BOTLIB_AAS_ENABLE_ROUTING_AREA;
pub use crate::g_public_h::BOTLIB_AAS_ENTITY_INFO;
pub use crate::g_public_h::BOTLIB_AAS_FLOAT_FOR_BSP_EPAIR_KEY;
pub use crate::g_public_h::BOTLIB_AAS_INITIALIZED;
pub use crate::g_public_h::BOTLIB_AAS_INT_FOR_BSP_EPAIR_KEY;
pub use crate::g_public_h::BOTLIB_AAS_NEXT_BSP_ENTITY;
pub use crate::g_public_h::BOTLIB_AAS_POINT_AREA_NUM;
pub use crate::g_public_h::BOTLIB_AAS_POINT_CONTENTS;
pub use crate::g_public_h::BOTLIB_AAS_POINT_REACHABILITY_AREA_INDEX;
pub use crate::g_public_h::BOTLIB_AAS_PREDICT_CLIENT_MOVEMENT;
pub use crate::g_public_h::BOTLIB_AAS_PREDICT_ROUTE;
pub use crate::g_public_h::BOTLIB_AAS_PRESENCE_TYPE_BOUNDING_BOX;
pub use crate::g_public_h::BOTLIB_AAS_SWIMMING;
pub use crate::g_public_h::BOTLIB_AAS_TIME;
pub use crate::g_public_h::BOTLIB_AAS_TRACE_AREAS;
pub use crate::g_public_h::BOTLIB_AAS_VALUE_FOR_BSP_EPAIR_KEY;
pub use crate::g_public_h::BOTLIB_AAS_VECTOR_FOR_BSP_EPAIR_KEY;
pub use crate::g_public_h::BOTLIB_AI_ADD_AVOID_SPOT;
pub use crate::g_public_h::BOTLIB_AI_ALLOC_CHAT_STATE;
pub use crate::g_public_h::BOTLIB_AI_ALLOC_GOAL_STATE;
pub use crate::g_public_h::BOTLIB_AI_ALLOC_MOVE_STATE;
pub use crate::g_public_h::BOTLIB_AI_ALLOC_WEAPON_STATE;
pub use crate::g_public_h::BOTLIB_AI_AVOID_GOAL_TIME;
pub use crate::g_public_h::BOTLIB_AI_CHARACTERISTIC_BFLOAT;
pub use crate::g_public_h::BOTLIB_AI_CHARACTERISTIC_BINTEGER;
pub use crate::g_public_h::BOTLIB_AI_CHARACTERISTIC_FLOAT;
pub use crate::g_public_h::BOTLIB_AI_CHARACTERISTIC_INTEGER;
pub use crate::g_public_h::BOTLIB_AI_CHARACTERISTIC_STRING;
pub use crate::g_public_h::BOTLIB_AI_CHAT_LENGTH;
pub use crate::g_public_h::BOTLIB_AI_CHOOSE_BEST_FIGHT_WEAPON;
pub use crate::g_public_h::BOTLIB_AI_CHOOSE_LTG_ITEM;
pub use crate::g_public_h::BOTLIB_AI_CHOOSE_NBG_ITEM;
pub use crate::g_public_h::BOTLIB_AI_DUMP_AVOID_GOALS;
pub use crate::g_public_h::BOTLIB_AI_DUMP_GOAL_STACK;
pub use crate::g_public_h::BOTLIB_AI_EMPTY_GOAL_STACK;
pub use crate::g_public_h::BOTLIB_AI_ENTER_CHAT;
pub use crate::g_public_h::BOTLIB_AI_FIND_MATCH;
pub use crate::g_public_h::BOTLIB_AI_FREE_CHARACTER;
pub use crate::g_public_h::BOTLIB_AI_FREE_CHAT_STATE;
pub use crate::g_public_h::BOTLIB_AI_FREE_GOAL_STATE;
pub use crate::g_public_h::BOTLIB_AI_FREE_ITEM_WEIGHTS;
pub use crate::g_public_h::BOTLIB_AI_FREE_MOVE_STATE;
pub use crate::g_public_h::BOTLIB_AI_FREE_WEAPON_STATE;
pub use crate::g_public_h::BOTLIB_AI_GENETIC_PARENTS_AND_CHILD_SELECTION;
pub use crate::g_public_h::BOTLIB_AI_GET_CHAT_MESSAGE;
pub use crate::g_public_h::BOTLIB_AI_GET_LEVEL_ITEM_GOAL;
pub use crate::g_public_h::BOTLIB_AI_GET_MAP_LOCATION_GOAL;
pub use crate::g_public_h::BOTLIB_AI_GET_NEXT_CAMP_SPOT_GOAL;
pub use crate::g_public_h::BOTLIB_AI_GET_SECOND_GOAL;
pub use crate::g_public_h::BOTLIB_AI_GET_TOP_GOAL;
pub use crate::g_public_h::BOTLIB_AI_GET_WEAPON_INFO;
pub use crate::g_public_h::BOTLIB_AI_GOAL_NAME;
pub use crate::g_public_h::BOTLIB_AI_INITIAL_CHAT;
pub use crate::g_public_h::BOTLIB_AI_INIT_LEVEL_ITEMS;
pub use crate::g_public_h::BOTLIB_AI_INIT_MOVE_STATE;
pub use crate::g_public_h::BOTLIB_AI_INTERBREED_GOAL_FUZZY_LOGIC;
pub use crate::g_public_h::BOTLIB_AI_ITEM_GOAL_IN_VIS_BUT_NOT_VISIBLE;
pub use crate::g_public_h::BOTLIB_AI_LOAD_CHARACTER;
pub use crate::g_public_h::BOTLIB_AI_LOAD_CHAT_FILE;
pub use crate::g_public_h::BOTLIB_AI_LOAD_ITEM_WEIGHTS;
pub use crate::g_public_h::BOTLIB_AI_LOAD_WEAPON_WEIGHTS;
pub use crate::g_public_h::BOTLIB_AI_MATCH_VARIABLE;
pub use crate::g_public_h::BOTLIB_AI_MOVEMENT_VIEW_TARGET;
pub use crate::g_public_h::BOTLIB_AI_MOVE_IN_DIRECTION;
pub use crate::g_public_h::BOTLIB_AI_MOVE_TO_GOAL;
pub use crate::g_public_h::BOTLIB_AI_MUTATE_GOAL_FUZZY_LOGIC;
pub use crate::g_public_h::BOTLIB_AI_NEXT_CONSOLE_MESSAGE;
pub use crate::g_public_h::BOTLIB_AI_NUM_CONSOLE_MESSAGE;
pub use crate::g_public_h::BOTLIB_AI_NUM_INITIAL_CHATS;
pub use crate::g_public_h::BOTLIB_AI_POP_GOAL;
pub use crate::g_public_h::BOTLIB_AI_PREDICT_VISIBLE_POSITION;
pub use crate::g_public_h::BOTLIB_AI_PUSH_GOAL;
pub use crate::g_public_h::BOTLIB_AI_QUEUE_CONSOLE_MESSAGE;
pub use crate::g_public_h::BOTLIB_AI_REACHABILITY_AREA;
pub use crate::g_public_h::BOTLIB_AI_REMOVE_CONSOLE_MESSAGE;
pub use crate::g_public_h::BOTLIB_AI_REMOVE_FROM_AVOID_GOALS;
pub use crate::g_public_h::BOTLIB_AI_REPLACE_SYNONYMS;
pub use crate::g_public_h::BOTLIB_AI_REPLY_CHAT;
pub use crate::g_public_h::BOTLIB_AI_RESET_AVOID_GOALS;
pub use crate::g_public_h::BOTLIB_AI_RESET_AVOID_REACH;
pub use crate::g_public_h::BOTLIB_AI_RESET_GOAL_STATE;
pub use crate::g_public_h::BOTLIB_AI_RESET_LAST_AVOID_REACH;
pub use crate::g_public_h::BOTLIB_AI_RESET_MOVE_STATE;
pub use crate::g_public_h::BOTLIB_AI_RESET_WEAPON_STATE;
pub use crate::g_public_h::BOTLIB_AI_SAVE_GOAL_FUZZY_LOGIC;
pub use crate::g_public_h::BOTLIB_AI_SET_AVOID_GOAL_TIME;
pub use crate::g_public_h::BOTLIB_AI_SET_CHAT_GENDER;
pub use crate::g_public_h::BOTLIB_AI_SET_CHAT_NAME;
pub use crate::g_public_h::BOTLIB_AI_STRING_CONTAINS;
pub use crate::g_public_h::BOTLIB_AI_TOUCHING_GOAL;
pub use crate::g_public_h::BOTLIB_AI_UNIFY_WHITE_SPACES;
pub use crate::g_public_h::BOTLIB_AI_UPDATE_ENTITY_ITEMS;
pub use crate::g_public_h::BOTLIB_EA_ACTION;
pub use crate::g_public_h::BOTLIB_EA_ATTACK;
pub use crate::g_public_h::BOTLIB_EA_COMMAND;
pub use crate::g_public_h::BOTLIB_EA_CROUCH;
pub use crate::g_public_h::BOTLIB_EA_DELAYED_JUMP;
pub use crate::g_public_h::BOTLIB_EA_END_REGULAR;
pub use crate::g_public_h::BOTLIB_EA_GESTURE;
pub use crate::g_public_h::BOTLIB_EA_GET_INPUT;
pub use crate::g_public_h::BOTLIB_EA_JUMP;
pub use crate::g_public_h::BOTLIB_EA_MOVE;
pub use crate::g_public_h::BOTLIB_EA_MOVE_BACK;
pub use crate::g_public_h::BOTLIB_EA_MOVE_DOWN;
pub use crate::g_public_h::BOTLIB_EA_MOVE_FORWARD;
pub use crate::g_public_h::BOTLIB_EA_MOVE_LEFT;
pub use crate::g_public_h::BOTLIB_EA_MOVE_RIGHT;
pub use crate::g_public_h::BOTLIB_EA_MOVE_UP;
pub use crate::g_public_h::BOTLIB_EA_RESET_INPUT;
pub use crate::g_public_h::BOTLIB_EA_RESPAWN;
pub use crate::g_public_h::BOTLIB_EA_SAY;
pub use crate::g_public_h::BOTLIB_EA_SAY_TEAM;
pub use crate::g_public_h::BOTLIB_EA_SELECT_WEAPON;
pub use crate::g_public_h::BOTLIB_EA_TALK;
pub use crate::g_public_h::BOTLIB_EA_USE;
pub use crate::g_public_h::BOTLIB_EA_VIEW;
pub use crate::g_public_h::BOTLIB_GET_CONSOLE_MESSAGE;
pub use crate::g_public_h::BOTLIB_GET_SNAPSHOT_ENTITY;
pub use crate::g_public_h::BOTLIB_LIBVAR_GET;
pub use crate::g_public_h::BOTLIB_LIBVAR_SET;
pub use crate::g_public_h::BOTLIB_LOAD_MAP;
pub use crate::g_public_h::BOTLIB_PC_ADD_GLOBAL_DEFINE;
pub use crate::g_public_h::BOTLIB_PC_FREE_SOURCE;
pub use crate::g_public_h::BOTLIB_PC_LOAD_SOURCE;
pub use crate::g_public_h::BOTLIB_PC_READ_TOKEN;
pub use crate::g_public_h::BOTLIB_PC_SOURCE_FILE_AND_LINE;
pub use crate::g_public_h::BOTLIB_SETUP;
pub use crate::g_public_h::BOTLIB_SHUTDOWN;
pub use crate::g_public_h::BOTLIB_START_FRAME;
pub use crate::g_public_h::BOTLIB_TEST;
pub use crate::g_public_h::BOTLIB_UPDATENTITY;
pub use crate::g_public_h::BOTLIB_USER_COMMAND;
pub use crate::g_public_h::G_ADJUST_AREA_PORTAL_STATE;
pub use crate::g_public_h::G_AREAS_CONNECTED;
pub use crate::g_public_h::G_ARGC;
pub use crate::g_public_h::G_ARGV;
pub use crate::g_public_h::G_BOT_ALLOCATE_CLIENT;
pub use crate::g_public_h::G_BOT_FREE_CLIENT;
pub use crate::g_public_h::G_CVAR_REGISTER;
pub use crate::g_public_h::G_CVAR_SET;
pub use crate::g_public_h::G_CVAR_UPDATE;
pub use crate::g_public_h::G_CVAR_VARIABLE_INTEGER_VALUE;
pub use crate::g_public_h::G_CVAR_VARIABLE_STRING_BUFFER;
pub use crate::g_public_h::G_DEBUG_POLYGON_CREATE;
pub use crate::g_public_h::G_DEBUG_POLYGON_DELETE;
pub use crate::g_public_h::G_DROP_CLIENT;
pub use crate::g_public_h::G_ENTITIES_IN_BOX;
pub use crate::g_public_h::G_ENTITY_CONTACT;
pub use crate::g_public_h::G_ENTITY_CONTACTCAPSULE;
pub use crate::g_public_h::G_ERROR;
pub use crate::g_public_h::G_FS_FCLOSE_FILE;
pub use crate::g_public_h::G_FS_FOPEN_FILE;
pub use crate::g_public_h::G_FS_GETFILELIST;
pub use crate::g_public_h::G_FS_READ;
pub use crate::g_public_h::G_FS_SEEK;
pub use crate::g_public_h::G_FS_WRITE;
pub use crate::g_public_h::G_GET_CONFIGSTRING;
pub use crate::g_public_h::G_GET_ENTITY_TOKEN;
pub use crate::g_public_h::G_GET_SERVERINFO;
pub use crate::g_public_h::G_GET_USERCMD;
pub use crate::g_public_h::G_GET_USERINFO;
pub use crate::g_public_h::G_IN_PVS;
pub use crate::g_public_h::G_IN_PVS_IGNORE_PORTALS;
pub use crate::g_public_h::G_LINKENTITY;
pub use crate::g_public_h::G_LOCATE_GAME_DATA;
pub use crate::g_public_h::G_MILLISECONDS;
pub use crate::g_public_h::G_POINT_CONTENTS;
pub use crate::g_public_h::G_PRINT;
pub use crate::g_public_h::G_REAL_TIME;
pub use crate::g_public_h::G_SEND_CONSOLE_COMMAND;
pub use crate::g_public_h::G_SEND_SERVER_COMMAND;
pub use crate::g_public_h::G_SET_BRUSH_MODEL;
pub use crate::g_public_h::G_SET_CONFIGSTRING;
pub use crate::g_public_h::G_SET_USERINFO;
pub use crate::g_public_h::G_SNAPVECTOR;
pub use crate::g_public_h::G_TRACE;
pub use crate::g_public_h::G_TRACECAPSULE;
pub use crate::g_public_h::G_UNLINKENTITY;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::cplane_s;
pub use crate::src::qcommon::q_shared::cplane_t;
pub use crate::src::qcommon::q_shared::cvarHandle_t;
pub use crate::src::qcommon::q_shared::entityState_s;
pub use crate::src::qcommon::q_shared::entityState_t;
pub use crate::src::qcommon::q_shared::fileHandle_t;
pub use crate::src::qcommon::q_shared::floatint_t;
pub use crate::src::qcommon::q_shared::fsMode_t;
pub use crate::src::qcommon::q_shared::pc_token_s;
pub use crate::src::qcommon::q_shared::pc_token_t;
pub use crate::src::qcommon::q_shared::playerState_s;
pub use crate::src::qcommon::q_shared::playerState_t;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qtime_s;
pub use crate::src::qcommon::q_shared::qtime_t;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::trType_t;
pub use crate::src::qcommon::q_shared::trace_t;
pub use crate::src::qcommon::q_shared::trajectory_t;
pub use crate::src::qcommon::q_shared::usercmd_s;
pub use crate::src::qcommon::q_shared::usercmd_t;
pub use crate::src::qcommon::q_shared::vec3_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::qcommon::q_shared::vmCvar_t;
pub use crate::src::qcommon::q_shared::FS_APPEND;
pub use crate::src::qcommon::q_shared::FS_APPEND_SYNC;
pub use crate::src::qcommon::q_shared::FS_READ;
pub use crate::src::qcommon::q_shared::FS_WRITE;
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
// this file is only included when building a dll
// g_syscalls.asm is included instead when building a qvm
// Initialized in run_static_initializers

static mut syscall: Option<
    unsafe extern "C" fn(_: crate::stdlib::intptr_t, _: ...) -> crate::stdlib::intptr_t,
> = None;
#[no_mangle]

pub unsafe extern "C" fn dllEntry(
    mut syscallptr: Option<
        unsafe extern "C" fn(_: crate::stdlib::intptr_t, _: ...) -> crate::stdlib::intptr_t,
    >,
) {
    syscall = syscallptr;
}
#[no_mangle]

pub unsafe extern "C" fn PASSFLOAT(mut x: f32) -> i32 {
    let mut fi: crate::src::qcommon::q_shared::floatint_t =
        crate::src::qcommon::q_shared::floatint_t { f: 0. };
    fi.f = x;
    return fi.i;
}
#[no_mangle]

pub unsafe extern "C" fn trap_Print(mut text: *const libc::c_char) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::G_PRINT as i32 as crate::stdlib::intptr_t,
        text,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_Error(mut text: *const libc::c_char) -> ! {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::G_ERROR as i32 as crate::stdlib::intptr_t,
        text,
    );
    // shut up GCC warning about returning functions, because we know better
    ::libc::exit(1 as i32);
}
#[no_mangle]

pub unsafe extern "C" fn trap_Milliseconds() -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::G_MILLISECONDS as i32 as crate::stdlib::intptr_t,
    ) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn trap_Argc() -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::G_ARGC as i32 as crate::stdlib::intptr_t,
    ) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn trap_Argv(
    mut n: i32,
    mut buffer: *mut libc::c_char,
    mut bufferLength: i32,
) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::G_ARGV as i32 as crate::stdlib::intptr_t,
        n,
        buffer,
        bufferLength,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_FS_FOpenFile(
    mut qpath: *const libc::c_char,
    mut f: *mut crate::src::qcommon::q_shared::fileHandle_t,
    mut mode: crate::src::qcommon::q_shared::fsMode_t,
) -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::G_FS_FOPEN_FILE as i32 as crate::stdlib::intptr_t,
        qpath,
        f,
        mode as u32,
    ) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn trap_FS_Read(
    mut buffer: *mut libc::c_void,
    mut len: i32,
    mut f: crate::src::qcommon::q_shared::fileHandle_t,
) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::G_FS_READ as i32 as crate::stdlib::intptr_t,
        buffer,
        len,
        f,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_FS_Write(
    mut buffer: *const libc::c_void,
    mut len: i32,
    mut f: crate::src::qcommon::q_shared::fileHandle_t,
) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::G_FS_WRITE as i32 as crate::stdlib::intptr_t,
        buffer,
        len,
        f,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_FS_FCloseFile(mut f: crate::src::qcommon::q_shared::fileHandle_t) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::G_FS_FCLOSE_FILE as i32 as crate::stdlib::intptr_t,
        f,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_FS_GetFileList(
    mut path: *const libc::c_char,
    mut extension: *const libc::c_char,
    mut listbuf: *mut libc::c_char,
    mut bufsize: i32,
) -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::G_FS_GETFILELIST as i32 as crate::stdlib::intptr_t,
        path,
        extension,
        listbuf,
        bufsize,
    ) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn trap_FS_Seek(
    mut f: crate::src::qcommon::q_shared::fileHandle_t,
    mut offset: isize,
    mut origin: i32,
) -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::G_FS_SEEK as i32 as crate::stdlib::intptr_t,
        f,
        offset,
        origin,
    ) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn trap_SendConsoleCommand(
    mut exec_when: i32,
    mut text: *const libc::c_char,
) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::G_SEND_CONSOLE_COMMAND as i32 as crate::stdlib::intptr_t,
        exec_when,
        text,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_Cvar_Register(
    mut cvar: *mut crate::src::qcommon::q_shared::vmCvar_t,
    mut var_name: *const libc::c_char,
    mut value: *const libc::c_char,
    mut flags: i32,
) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::G_CVAR_REGISTER as i32 as crate::stdlib::intptr_t,
        cvar,
        var_name,
        value,
        flags,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_Cvar_Update(mut cvar: *mut crate::src::qcommon::q_shared::vmCvar_t) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::G_CVAR_UPDATE as i32 as crate::stdlib::intptr_t,
        cvar,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_Cvar_Set(
    mut var_name: *const libc::c_char,
    mut value: *const libc::c_char,
) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::G_CVAR_SET as i32 as crate::stdlib::intptr_t,
        var_name,
        value,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_Cvar_VariableIntegerValue(mut var_name: *const libc::c_char) -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::G_CVAR_VARIABLE_INTEGER_VALUE as i32 as crate::stdlib::intptr_t,
        var_name,
    ) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn trap_Cvar_VariableStringBuffer(
    mut var_name: *const libc::c_char,
    mut buffer: *mut libc::c_char,
    mut bufsize: i32,
) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::G_CVAR_VARIABLE_STRING_BUFFER as i32 as crate::stdlib::intptr_t,
        var_name,
        buffer,
        bufsize,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_LocateGameData(
    mut gEnts: *mut crate::g_local_h::gentity_t,
    mut numGEntities: i32,
    mut sizeofGEntity_t: i32,
    mut clients: *mut crate::src::qcommon::q_shared::playerState_t,
    mut sizeofGClient: i32,
) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::G_LOCATE_GAME_DATA as i32 as crate::stdlib::intptr_t,
        gEnts,
        numGEntities,
        sizeofGEntity_t,
        clients,
        sizeofGClient,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_DropClient(mut clientNum: i32, mut reason: *const libc::c_char) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::G_DROP_CLIENT as i32 as crate::stdlib::intptr_t,
        clientNum,
        reason,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_SendServerCommand(mut clientNum: i32, mut text: *const libc::c_char) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::G_SEND_SERVER_COMMAND as i32 as crate::stdlib::intptr_t,
        clientNum,
        text,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_SetConfigstring(mut num: i32, mut string: *const libc::c_char) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::G_SET_CONFIGSTRING as i32 as crate::stdlib::intptr_t,
        num,
        string,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_GetConfigstring(
    mut num: i32,
    mut buffer: *mut libc::c_char,
    mut bufferSize: i32,
) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::G_GET_CONFIGSTRING as i32 as crate::stdlib::intptr_t,
        num,
        buffer,
        bufferSize,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_GetUserinfo(
    mut num: i32,
    mut buffer: *mut libc::c_char,
    mut bufferSize: i32,
) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::G_GET_USERINFO as i32 as crate::stdlib::intptr_t,
        num,
        buffer,
        bufferSize,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_SetUserinfo(mut num: i32, mut buffer: *const libc::c_char) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::G_SET_USERINFO as i32 as crate::stdlib::intptr_t,
        num,
        buffer,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_GetServerinfo(mut buffer: *mut libc::c_char, mut bufferSize: i32) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::G_GET_SERVERINFO as i32 as crate::stdlib::intptr_t,
        buffer,
        bufferSize,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_SetBrushModel(
    mut ent: *mut crate::g_local_h::gentity_t,
    mut name: *const libc::c_char,
) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::G_SET_BRUSH_MODEL as i32 as crate::stdlib::intptr_t,
        ent,
        name,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_Trace(
    mut results: *mut crate::src::qcommon::q_shared::trace_t,
    mut start: *const crate::src::qcommon::q_shared::vec_t,
    mut mins: *const crate::src::qcommon::q_shared::vec_t,
    mut maxs: *const crate::src::qcommon::q_shared::vec_t,
    mut end: *const crate::src::qcommon::q_shared::vec_t,
    mut passEntityNum: i32,
    mut contentmask: i32,
) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::G_TRACE as i32 as crate::stdlib::intptr_t,
        results,
        start,
        mins,
        maxs,
        end,
        passEntityNum,
        contentmask,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_TraceCapsule(
    mut results: *mut crate::src::qcommon::q_shared::trace_t,
    mut start: *const crate::src::qcommon::q_shared::vec_t,
    mut mins: *const crate::src::qcommon::q_shared::vec_t,
    mut maxs: *const crate::src::qcommon::q_shared::vec_t,
    mut end: *const crate::src::qcommon::q_shared::vec_t,
    mut passEntityNum: i32,
    mut contentmask: i32,
) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::G_TRACECAPSULE as i32 as crate::stdlib::intptr_t,
        results,
        start,
        mins,
        maxs,
        end,
        passEntityNum,
        contentmask,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_PointContents(
    mut point: *const crate::src::qcommon::q_shared::vec_t,
    mut passEntityNum: i32,
) -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::G_POINT_CONTENTS as i32 as crate::stdlib::intptr_t,
        point,
        passEntityNum,
    ) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn trap_InPVS(
    mut p1: *const crate::src::qcommon::q_shared::vec_t,
    mut p2: *const crate::src::qcommon::q_shared::vec_t,
) -> crate::src::qcommon::q_shared::qboolean {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::G_IN_PVS as i32 as crate::stdlib::intptr_t,
        p1,
        p2,
    ) as crate::src::qcommon::q_shared::qboolean;
}
#[no_mangle]

pub unsafe extern "C" fn trap_InPVSIgnorePortals(
    mut p1: *const crate::src::qcommon::q_shared::vec_t,
    mut p2: *const crate::src::qcommon::q_shared::vec_t,
) -> crate::src::qcommon::q_shared::qboolean {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::G_IN_PVS_IGNORE_PORTALS as i32 as crate::stdlib::intptr_t,
        p1,
        p2,
    ) as crate::src::qcommon::q_shared::qboolean;
}
#[no_mangle]

pub unsafe extern "C" fn trap_AdjustAreaPortalState(
    mut ent: *mut crate::g_local_h::gentity_t,
    mut open: crate::src::qcommon::q_shared::qboolean,
) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::G_ADJUST_AREA_PORTAL_STATE as i32 as crate::stdlib::intptr_t,
        ent,
        open as u32,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_AreasConnected(
    mut area1: i32,
    mut area2: i32,
) -> crate::src::qcommon::q_shared::qboolean {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::G_AREAS_CONNECTED as i32 as crate::stdlib::intptr_t,
        area1,
        area2,
    ) as crate::src::qcommon::q_shared::qboolean;
}
#[no_mangle]

pub unsafe extern "C" fn trap_LinkEntity(mut ent: *mut crate::g_local_h::gentity_t) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::G_LINKENTITY as i32 as crate::stdlib::intptr_t,
        ent,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_UnlinkEntity(mut ent: *mut crate::g_local_h::gentity_t) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::G_UNLINKENTITY as i32 as crate::stdlib::intptr_t,
        ent,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_EntitiesInBox(
    mut mins: *const crate::src::qcommon::q_shared::vec_t,
    mut maxs: *const crate::src::qcommon::q_shared::vec_t,
    mut list: *mut i32,
    mut maxcount: i32,
) -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::G_ENTITIES_IN_BOX as i32 as crate::stdlib::intptr_t,
        mins,
        maxs,
        list,
        maxcount,
    ) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn trap_EntityContact(
    mut mins: *const crate::src::qcommon::q_shared::vec_t,
    mut maxs: *const crate::src::qcommon::q_shared::vec_t,
    mut ent: *const crate::g_local_h::gentity_t,
) -> crate::src::qcommon::q_shared::qboolean {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::G_ENTITY_CONTACT as i32 as crate::stdlib::intptr_t,
        mins,
        maxs,
        ent,
    ) as crate::src::qcommon::q_shared::qboolean;
}
#[no_mangle]

pub unsafe extern "C" fn trap_EntityContactCapsule(
    mut mins: *const crate::src::qcommon::q_shared::vec_t,
    mut maxs: *const crate::src::qcommon::q_shared::vec_t,
    mut ent: *const crate::g_local_h::gentity_t,
) -> crate::src::qcommon::q_shared::qboolean {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::G_ENTITY_CONTACTCAPSULE as i32 as crate::stdlib::intptr_t,
        mins,
        maxs,
        ent,
    ) as crate::src::qcommon::q_shared::qboolean;
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotAllocateClient() -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::G_BOT_ALLOCATE_CLIENT as i32 as crate::stdlib::intptr_t,
    ) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotFreeClient(mut clientNum: i32) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::G_BOT_FREE_CLIENT as i32 as crate::stdlib::intptr_t,
        clientNum,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_GetUsercmd(
    mut clientNum: i32,
    mut cmd: *mut crate::src::qcommon::q_shared::usercmd_t,
) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::G_GET_USERCMD as i32 as crate::stdlib::intptr_t,
        clientNum,
        cmd,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_GetEntityToken(
    mut buffer: *mut libc::c_char,
    mut bufferSize: i32,
) -> crate::src::qcommon::q_shared::qboolean {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::G_GET_ENTITY_TOKEN as i32 as crate::stdlib::intptr_t,
        buffer,
        bufferSize,
    ) as crate::src::qcommon::q_shared::qboolean;
}
#[no_mangle]

pub unsafe extern "C" fn trap_DebugPolygonCreate(
    mut color: i32,
    mut numPoints: i32,
    mut points: *mut crate::src::qcommon::q_shared::vec3_t,
) -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::G_DEBUG_POLYGON_CREATE as i32 as crate::stdlib::intptr_t,
        color,
        numPoints,
        points,
    ) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn trap_DebugPolygonDelete(mut id: i32) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::G_DEBUG_POLYGON_DELETE as i32 as crate::stdlib::intptr_t,
        id,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_RealTime(
    mut qtime: *mut crate::src::qcommon::q_shared::qtime_t,
) -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::G_REAL_TIME as i32 as crate::stdlib::intptr_t,
        qtime,
    ) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn trap_SnapVector(mut v: *mut f32) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::G_SNAPVECTOR as i32 as crate::stdlib::intptr_t,
        v,
    );
}
// BotLib traps start here
#[no_mangle]

pub unsafe extern "C" fn trap_BotLibSetup() -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_SETUP as i32 as crate::stdlib::intptr_t,
    ) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotLibShutdown() -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_SHUTDOWN as i32 as crate::stdlib::intptr_t,
    ) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotLibVarSet(
    mut var_name: *mut libc::c_char,
    mut value: *mut libc::c_char,
) -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_LIBVAR_SET as i32 as crate::stdlib::intptr_t,
        var_name,
        value,
    ) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotLibVarGet(
    mut var_name: *mut libc::c_char,
    mut value: *mut libc::c_char,
    mut size: i32,
) -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_LIBVAR_GET as i32 as crate::stdlib::intptr_t,
        var_name,
        value,
        size,
    ) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotLibDefine(mut string: *mut libc::c_char) -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_PC_ADD_GLOBAL_DEFINE as i32 as crate::stdlib::intptr_t,
        string,
    ) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotLibStartFrame(mut time: f32) -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_START_FRAME as i32 as crate::stdlib::intptr_t,
        PASSFLOAT(time),
    ) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotLibLoadMap(mut mapname: *const libc::c_char) -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_LOAD_MAP as i32 as crate::stdlib::intptr_t,
        mapname,
    ) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotLibUpdateEntity(mut ent: i32, mut bue: *mut libc::c_void) -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_UPDATENTITY as i32 as crate::stdlib::intptr_t,
        ent,
        bue,
    ) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotLibTest(
    mut parm0: i32,
    mut parm1: *mut libc::c_char,
    mut parm2: *mut crate::src::qcommon::q_shared::vec_t,
    mut parm3: *mut crate::src::qcommon::q_shared::vec_t,
) -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_TEST as i32 as crate::stdlib::intptr_t,
        parm0,
        parm1,
        parm2,
        parm3,
    ) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotGetSnapshotEntity(mut clientNum: i32, mut sequence: i32) -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_GET_SNAPSHOT_ENTITY as i32 as crate::stdlib::intptr_t,
        clientNum,
        sequence,
    ) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotGetServerCommand(
    mut clientNum: i32,
    mut message: *mut libc::c_char,
    mut size: i32,
) -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_GET_CONSOLE_MESSAGE as i32 as crate::stdlib::intptr_t,
        clientNum,
        message,
        size,
    ) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotUserCommand(
    mut clientNum: i32,
    mut ucmd: *mut crate::src::qcommon::q_shared::usercmd_t,
) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_USER_COMMAND as i32 as crate::stdlib::intptr_t,
        clientNum,
        ucmd,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_AAS_EntityInfo(mut entnum: i32, mut info: *mut libc::c_void) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AAS_ENTITY_INFO as i32 as crate::stdlib::intptr_t,
        entnum,
        info,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_AAS_Initialized() -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AAS_INITIALIZED as i32 as crate::stdlib::intptr_t,
    ) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn trap_AAS_PresenceTypeBoundingBox(
    mut presencetype: i32,
    mut mins: *mut crate::src::qcommon::q_shared::vec_t,
    mut maxs: *mut crate::src::qcommon::q_shared::vec_t,
) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AAS_PRESENCE_TYPE_BOUNDING_BOX as i32 as crate::stdlib::intptr_t,
        presencetype,
        mins,
        maxs,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_AAS_Time() -> f32 {
    let mut fi: crate::src::qcommon::q_shared::floatint_t =
        crate::src::qcommon::q_shared::floatint_t { f: 0. };
    fi.i = syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AAS_TIME as i32 as crate::stdlib::intptr_t,
    ) as i32;
    return fi.f;
}
#[no_mangle]

pub unsafe extern "C" fn trap_AAS_PointAreaNum(
    mut point: *mut crate::src::qcommon::q_shared::vec_t,
) -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AAS_POINT_AREA_NUM as i32 as crate::stdlib::intptr_t,
        point,
    ) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn trap_AAS_PointReachabilityAreaIndex(
    mut point: *mut crate::src::qcommon::q_shared::vec_t,
) -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AAS_POINT_REACHABILITY_AREA_INDEX as i32
            as crate::stdlib::intptr_t,
        point,
    ) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn trap_AAS_TraceAreas(
    mut start: *mut crate::src::qcommon::q_shared::vec_t,
    mut end: *mut crate::src::qcommon::q_shared::vec_t,
    mut areas: *mut i32,
    mut points: *mut crate::src::qcommon::q_shared::vec3_t,
    mut maxareas: i32,
) -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AAS_TRACE_AREAS as i32 as crate::stdlib::intptr_t,
        start,
        end,
        areas,
        points,
        maxareas,
    ) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn trap_AAS_BBoxAreas(
    mut absmins: *mut crate::src::qcommon::q_shared::vec_t,
    mut absmaxs: *mut crate::src::qcommon::q_shared::vec_t,
    mut areas: *mut i32,
    mut maxareas: i32,
) -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AAS_BBOX_AREAS as i32 as crate::stdlib::intptr_t,
        absmins,
        absmaxs,
        areas,
        maxareas,
    ) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn trap_AAS_AreaInfo(mut areanum: i32, mut info: *mut libc::c_void) -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AAS_AREA_INFO as i32 as crate::stdlib::intptr_t,
        areanum,
        info,
    ) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn trap_AAS_PointContents(
    mut point: *mut crate::src::qcommon::q_shared::vec_t,
) -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AAS_POINT_CONTENTS as i32 as crate::stdlib::intptr_t,
        point,
    ) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn trap_AAS_NextBSPEntity(mut ent: i32) -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AAS_NEXT_BSP_ENTITY as i32 as crate::stdlib::intptr_t,
        ent,
    ) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn trap_AAS_ValueForBSPEpairKey(
    mut ent: i32,
    mut key: *mut libc::c_char,
    mut value: *mut libc::c_char,
    mut size: i32,
) -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AAS_VALUE_FOR_BSP_EPAIR_KEY as i32 as crate::stdlib::intptr_t,
        ent,
        key,
        value,
        size,
    ) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn trap_AAS_VectorForBSPEpairKey(
    mut ent: i32,
    mut key: *mut libc::c_char,
    mut v: *mut crate::src::qcommon::q_shared::vec_t,
) -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AAS_VECTOR_FOR_BSP_EPAIR_KEY as i32 as crate::stdlib::intptr_t,
        ent,
        key,
        v,
    ) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn trap_AAS_FloatForBSPEpairKey(
    mut ent: i32,
    mut key: *mut libc::c_char,
    mut value: *mut f32,
) -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AAS_FLOAT_FOR_BSP_EPAIR_KEY as i32 as crate::stdlib::intptr_t,
        ent,
        key,
        value,
    ) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn trap_AAS_IntForBSPEpairKey(
    mut ent: i32,
    mut key: *mut libc::c_char,
    mut value: *mut i32,
) -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AAS_INT_FOR_BSP_EPAIR_KEY as i32 as crate::stdlib::intptr_t,
        ent,
        key,
        value,
    ) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn trap_AAS_AreaReachability(mut areanum: i32) -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AAS_AREA_REACHABILITY as i32 as crate::stdlib::intptr_t,
        areanum,
    ) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn trap_AAS_AreaTravelTimeToGoalArea(
    mut areanum: i32,
    mut origin: *mut crate::src::qcommon::q_shared::vec_t,
    mut goalareanum: i32,
    mut travelflags: i32,
) -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AAS_AREA_TRAVEL_TIME_TO_GOAL_AREA as i32
            as crate::stdlib::intptr_t,
        areanum,
        origin,
        goalareanum,
        travelflags,
    ) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn trap_AAS_EnableRoutingArea(mut areanum: i32, mut enable: i32) -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AAS_ENABLE_ROUTING_AREA as i32 as crate::stdlib::intptr_t,
        areanum,
        enable,
    ) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn trap_AAS_PredictRoute(
    mut route: *mut libc::c_void,
    mut areanum: i32,
    mut origin: *mut crate::src::qcommon::q_shared::vec_t,
    mut goalareanum: i32,
    mut travelflags: i32,
    mut maxareas: i32,
    mut maxtime: i32,
    mut stopevent: i32,
    mut stopcontents: i32,
    mut stoptfl: i32,
    mut stopareanum: i32,
) -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AAS_PREDICT_ROUTE as i32 as crate::stdlib::intptr_t,
        route,
        areanum,
        origin,
        goalareanum,
        travelflags,
        maxareas,
        maxtime,
        stopevent,
        stopcontents,
        stoptfl,
        stopareanum,
    ) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn trap_AAS_AlternativeRouteGoals(
    mut start: *mut crate::src::qcommon::q_shared::vec_t,
    mut startareanum: i32,
    mut goal: *mut crate::src::qcommon::q_shared::vec_t,
    mut goalareanum: i32,
    mut travelflags: i32,
    mut altroutegoals: *mut libc::c_void,
    mut maxaltroutegoals: i32,
    mut type_0: i32,
) -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AAS_ALTERNATIVE_ROUTE_GOAL as i32 as crate::stdlib::intptr_t,
        start,
        startareanum,
        goal,
        goalareanum,
        travelflags,
        altroutegoals,
        maxaltroutegoals,
        type_0,
    ) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn trap_AAS_Swimming(
    mut origin: *mut crate::src::qcommon::q_shared::vec_t,
) -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AAS_SWIMMING as i32 as crate::stdlib::intptr_t,
        origin,
    ) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn trap_AAS_PredictClientMovement(
    mut move_0: *mut libc::c_void,
    mut entnum: i32,
    mut origin: *mut crate::src::qcommon::q_shared::vec_t,
    mut presencetype: i32,
    mut onground: i32,
    mut velocity: *mut crate::src::qcommon::q_shared::vec_t,
    mut cmdmove: *mut crate::src::qcommon::q_shared::vec_t,
    mut cmdframes: i32,
    mut maxframes: i32,
    mut frametime: f32,
    mut stopevent: i32,
    mut stopareanum: i32,
    mut visualize: i32,
) -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AAS_PREDICT_CLIENT_MOVEMENT as i32 as crate::stdlib::intptr_t,
        move_0,
        entnum,
        origin,
        presencetype,
        onground,
        velocity,
        cmdmove,
        cmdframes,
        maxframes,
        PASSFLOAT(frametime),
        stopevent,
        stopareanum,
        visualize,
    ) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn trap_EA_Say(mut client: i32, mut str: *mut libc::c_char) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_EA_SAY as i32 as crate::stdlib::intptr_t,
        client,
        str,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_EA_SayTeam(mut client: i32, mut str: *mut libc::c_char) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_EA_SAY_TEAM as i32 as crate::stdlib::intptr_t,
        client,
        str,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_EA_Command(mut client: i32, mut command: *mut libc::c_char) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_EA_COMMAND as i32 as crate::stdlib::intptr_t,
        client,
        command,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_EA_Action(mut client: i32, mut action: i32) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_EA_ACTION as i32 as crate::stdlib::intptr_t,
        client,
        action,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_EA_Gesture(mut client: i32) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_EA_GESTURE as i32 as crate::stdlib::intptr_t,
        client,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_EA_Talk(mut client: i32) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_EA_TALK as i32 as crate::stdlib::intptr_t,
        client,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_EA_Attack(mut client: i32) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_EA_ATTACK as i32 as crate::stdlib::intptr_t,
        client,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_EA_Use(mut client: i32) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_EA_USE as i32 as crate::stdlib::intptr_t,
        client,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_EA_Respawn(mut client: i32) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_EA_RESPAWN as i32 as crate::stdlib::intptr_t,
        client,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_EA_Crouch(mut client: i32) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_EA_CROUCH as i32 as crate::stdlib::intptr_t,
        client,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_EA_MoveUp(mut client: i32) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_EA_MOVE_UP as i32 as crate::stdlib::intptr_t,
        client,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_EA_MoveDown(mut client: i32) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_EA_MOVE_DOWN as i32 as crate::stdlib::intptr_t,
        client,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_EA_MoveForward(mut client: i32) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_EA_MOVE_FORWARD as i32 as crate::stdlib::intptr_t,
        client,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_EA_MoveBack(mut client: i32) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_EA_MOVE_BACK as i32 as crate::stdlib::intptr_t,
        client,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_EA_MoveLeft(mut client: i32) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_EA_MOVE_LEFT as i32 as crate::stdlib::intptr_t,
        client,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_EA_MoveRight(mut client: i32) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_EA_MOVE_RIGHT as i32 as crate::stdlib::intptr_t,
        client,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_EA_SelectWeapon(mut client: i32, mut weapon: i32) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_EA_SELECT_WEAPON as i32 as crate::stdlib::intptr_t,
        client,
        weapon,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_EA_Jump(mut client: i32) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_EA_JUMP as i32 as crate::stdlib::intptr_t,
        client,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_EA_DelayedJump(mut client: i32) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_EA_DELAYED_JUMP as i32 as crate::stdlib::intptr_t,
        client,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_EA_Move(
    mut client: i32,
    mut dir: *mut crate::src::qcommon::q_shared::vec_t,
    mut speed: f32,
) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_EA_MOVE as i32 as crate::stdlib::intptr_t,
        client,
        dir,
        PASSFLOAT(speed),
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_EA_View(
    mut client: i32,
    mut viewangles: *mut crate::src::qcommon::q_shared::vec_t,
) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_EA_VIEW as i32 as crate::stdlib::intptr_t,
        client,
        viewangles,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_EA_EndRegular(mut client: i32, mut thinktime: f32) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_EA_END_REGULAR as i32 as crate::stdlib::intptr_t,
        client,
        PASSFLOAT(thinktime),
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_EA_GetInput(
    mut client: i32,
    mut thinktime: f32,
    mut input: *mut libc::c_void,
) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_EA_GET_INPUT as i32 as crate::stdlib::intptr_t,
        client,
        PASSFLOAT(thinktime),
        input,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_EA_ResetInput(mut client: i32) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_EA_RESET_INPUT as i32 as crate::stdlib::intptr_t,
        client,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotLoadCharacter(
    mut charfile: *mut libc::c_char,
    mut skill: f32,
) -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_LOAD_CHARACTER as i32 as crate::stdlib::intptr_t,
        charfile,
        PASSFLOAT(skill),
    ) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotFreeCharacter(mut character: i32) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_FREE_CHARACTER as i32 as crate::stdlib::intptr_t,
        character,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_Characteristic_Float(mut character: i32, mut index: i32) -> f32 {
    let mut fi: crate::src::qcommon::q_shared::floatint_t =
        crate::src::qcommon::q_shared::floatint_t { f: 0. };
    fi.i = syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_CHARACTERISTIC_FLOAT as i32 as crate::stdlib::intptr_t,
        character,
        index,
    ) as i32;
    return fi.f;
}
#[no_mangle]

pub unsafe extern "C" fn trap_Characteristic_BFloat(
    mut character: i32,
    mut index: i32,
    mut min: f32,
    mut max: f32,
) -> f32 {
    let mut fi: crate::src::qcommon::q_shared::floatint_t =
        crate::src::qcommon::q_shared::floatint_t { f: 0. };
    fi.i = syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_CHARACTERISTIC_BFLOAT as i32 as crate::stdlib::intptr_t,
        character,
        index,
        PASSFLOAT(min),
        PASSFLOAT(max),
    ) as i32;
    return fi.f;
}
#[no_mangle]

pub unsafe extern "C" fn trap_Characteristic_Integer(mut character: i32, mut index: i32) -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_CHARACTERISTIC_INTEGER as i32 as crate::stdlib::intptr_t,
        character,
        index,
    ) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn trap_Characteristic_BInteger(
    mut character: i32,
    mut index: i32,
    mut min: i32,
    mut max: i32,
) -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_CHARACTERISTIC_BINTEGER as i32 as crate::stdlib::intptr_t,
        character,
        index,
        min,
        max,
    ) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn trap_Characteristic_String(
    mut character: i32,
    mut index: i32,
    mut buf: *mut libc::c_char,
    mut size: i32,
) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_CHARACTERISTIC_STRING as i32 as crate::stdlib::intptr_t,
        character,
        index,
        buf,
        size,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotAllocChatState() -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_ALLOC_CHAT_STATE as i32 as crate::stdlib::intptr_t,
    ) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotFreeChatState(mut handle: i32) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_FREE_CHAT_STATE as i32 as crate::stdlib::intptr_t,
        handle,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotQueueConsoleMessage(
    mut chatstate: i32,
    mut type_0: i32,
    mut message: *mut libc::c_char,
) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_QUEUE_CONSOLE_MESSAGE as i32 as crate::stdlib::intptr_t,
        chatstate,
        type_0,
        message,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotRemoveConsoleMessage(mut chatstate: i32, mut handle: i32) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_REMOVE_CONSOLE_MESSAGE as i32 as crate::stdlib::intptr_t,
        chatstate,
        handle,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotNextConsoleMessage(
    mut chatstate: i32,
    mut cm: *mut libc::c_void,
) -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_NEXT_CONSOLE_MESSAGE as i32 as crate::stdlib::intptr_t,
        chatstate,
        cm,
    ) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotNumConsoleMessages(mut chatstate: i32) -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_NUM_CONSOLE_MESSAGE as i32 as crate::stdlib::intptr_t,
        chatstate,
    ) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotInitialChat(
    mut chatstate: i32,
    mut type_0: *mut libc::c_char,
    mut mcontext: i32,
    mut var0: *mut libc::c_char,
    mut var1: *mut libc::c_char,
    mut var2: *mut libc::c_char,
    mut var3: *mut libc::c_char,
    mut var4: *mut libc::c_char,
    mut var5: *mut libc::c_char,
    mut var6: *mut libc::c_char,
    mut var7: *mut libc::c_char,
) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_INITIAL_CHAT as i32 as crate::stdlib::intptr_t,
        chatstate,
        type_0,
        mcontext,
        var0,
        var1,
        var2,
        var3,
        var4,
        var5,
        var6,
        var7,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotNumInitialChats(
    mut chatstate: i32,
    mut type_0: *mut libc::c_char,
) -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_NUM_INITIAL_CHATS as i32 as crate::stdlib::intptr_t,
        chatstate,
        type_0,
    ) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotReplyChat(
    mut chatstate: i32,
    mut message: *mut libc::c_char,
    mut mcontext: i32,
    mut vcontext: i32,
    mut var0: *mut libc::c_char,
    mut var1: *mut libc::c_char,
    mut var2: *mut libc::c_char,
    mut var3: *mut libc::c_char,
    mut var4: *mut libc::c_char,
    mut var5: *mut libc::c_char,
    mut var6: *mut libc::c_char,
    mut var7: *mut libc::c_char,
) -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_REPLY_CHAT as i32 as crate::stdlib::intptr_t,
        chatstate,
        message,
        mcontext,
        vcontext,
        var0,
        var1,
        var2,
        var3,
        var4,
        var5,
        var6,
        var7,
    ) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotChatLength(mut chatstate: i32) -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_CHAT_LENGTH as i32 as crate::stdlib::intptr_t,
        chatstate,
    ) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotEnterChat(mut chatstate: i32, mut client: i32, mut sendto: i32) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_ENTER_CHAT as i32 as crate::stdlib::intptr_t,
        chatstate,
        client,
        sendto,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotGetChatMessage(
    mut chatstate: i32,
    mut buf: *mut libc::c_char,
    mut size: i32,
) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_GET_CHAT_MESSAGE as i32 as crate::stdlib::intptr_t,
        chatstate,
        buf,
        size,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_StringContains(
    mut str1: *mut libc::c_char,
    mut str2: *mut libc::c_char,
    mut casesensitive: i32,
) -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_STRING_CONTAINS as i32 as crate::stdlib::intptr_t,
        str1,
        str2,
        casesensitive,
    ) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotFindMatch(
    mut str: *mut libc::c_char,
    mut match_0: *mut libc::c_void,
    mut context: libc::c_ulong,
) -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_FIND_MATCH as i32 as crate::stdlib::intptr_t,
        str,
        match_0,
        context,
    ) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotMatchVariable(
    mut match_0: *mut libc::c_void,
    mut variable: i32,
    mut buf: *mut libc::c_char,
    mut size: i32,
) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_MATCH_VARIABLE as i32 as crate::stdlib::intptr_t,
        match_0,
        variable,
        buf,
        size,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_UnifyWhiteSpaces(mut string: *mut libc::c_char) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_UNIFY_WHITE_SPACES as i32 as crate::stdlib::intptr_t,
        string,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotReplaceSynonyms(
    mut string: *mut libc::c_char,
    mut context: libc::c_ulong,
) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_REPLACE_SYNONYMS as i32 as crate::stdlib::intptr_t,
        string,
        context,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotLoadChatFile(
    mut chatstate: i32,
    mut chatfile: *mut libc::c_char,
    mut chatname: *mut libc::c_char,
) -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_LOAD_CHAT_FILE as i32 as crate::stdlib::intptr_t,
        chatstate,
        chatfile,
        chatname,
    ) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotSetChatGender(mut chatstate: i32, mut gender: i32) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_SET_CHAT_GENDER as i32 as crate::stdlib::intptr_t,
        chatstate,
        gender,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotSetChatName(
    mut chatstate: i32,
    mut name: *mut libc::c_char,
    mut client: i32,
) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_SET_CHAT_NAME as i32 as crate::stdlib::intptr_t,
        chatstate,
        name,
        client,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotResetGoalState(mut goalstate: i32) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_RESET_GOAL_STATE as i32 as crate::stdlib::intptr_t,
        goalstate,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotResetAvoidGoals(mut goalstate: i32) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_RESET_AVOID_GOALS as i32 as crate::stdlib::intptr_t,
        goalstate,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotRemoveFromAvoidGoals(mut goalstate: i32, mut number: i32) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_REMOVE_FROM_AVOID_GOALS as i32 as crate::stdlib::intptr_t,
        goalstate,
        number,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotPushGoal(mut goalstate: i32, mut goal: *mut libc::c_void) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_PUSH_GOAL as i32 as crate::stdlib::intptr_t,
        goalstate,
        goal,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotPopGoal(mut goalstate: i32) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_POP_GOAL as i32 as crate::stdlib::intptr_t,
        goalstate,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotEmptyGoalStack(mut goalstate: i32) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_EMPTY_GOAL_STACK as i32 as crate::stdlib::intptr_t,
        goalstate,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotDumpAvoidGoals(mut goalstate: i32) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_DUMP_AVOID_GOALS as i32 as crate::stdlib::intptr_t,
        goalstate,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotDumpGoalStack(mut goalstate: i32) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_DUMP_GOAL_STACK as i32 as crate::stdlib::intptr_t,
        goalstate,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotGoalName(
    mut number: i32,
    mut name: *mut libc::c_char,
    mut size: i32,
) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_GOAL_NAME as i32 as crate::stdlib::intptr_t,
        number,
        name,
        size,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotGetTopGoal(
    mut goalstate: i32,
    mut goal: *mut libc::c_void,
) -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_GET_TOP_GOAL as i32 as crate::stdlib::intptr_t,
        goalstate,
        goal,
    ) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotGetSecondGoal(
    mut goalstate: i32,
    mut goal: *mut libc::c_void,
) -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_GET_SECOND_GOAL as i32 as crate::stdlib::intptr_t,
        goalstate,
        goal,
    ) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotChooseLTGItem(
    mut goalstate: i32,
    mut origin: *mut crate::src::qcommon::q_shared::vec_t,
    mut inventory: *mut i32,
    mut travelflags: i32,
) -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_CHOOSE_LTG_ITEM as i32 as crate::stdlib::intptr_t,
        goalstate,
        origin,
        inventory,
        travelflags,
    ) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotChooseNBGItem(
    mut goalstate: i32,
    mut origin: *mut crate::src::qcommon::q_shared::vec_t,
    mut inventory: *mut i32,
    mut travelflags: i32,
    mut ltg: *mut libc::c_void,
    mut maxtime: f32,
) -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_CHOOSE_NBG_ITEM as i32 as crate::stdlib::intptr_t,
        goalstate,
        origin,
        inventory,
        travelflags,
        ltg,
        PASSFLOAT(maxtime),
    ) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotTouchingGoal(
    mut origin: *mut crate::src::qcommon::q_shared::vec_t,
    mut goal: *mut libc::c_void,
) -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_TOUCHING_GOAL as i32 as crate::stdlib::intptr_t,
        origin,
        goal,
    ) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotItemGoalInVisButNotVisible(
    mut viewer: i32,
    mut eye: *mut crate::src::qcommon::q_shared::vec_t,
    mut viewangles: *mut crate::src::qcommon::q_shared::vec_t,
    mut goal: *mut libc::c_void,
) -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_ITEM_GOAL_IN_VIS_BUT_NOT_VISIBLE as i32
            as crate::stdlib::intptr_t,
        viewer,
        eye,
        viewangles,
        goal,
    ) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotGetLevelItemGoal(
    mut index: i32,
    mut classname: *mut libc::c_char,
    mut goal: *mut libc::c_void,
) -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_GET_LEVEL_ITEM_GOAL as i32 as crate::stdlib::intptr_t,
        index,
        classname,
        goal,
    ) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotGetNextCampSpotGoal(
    mut num: i32,
    mut goal: *mut libc::c_void,
) -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_GET_NEXT_CAMP_SPOT_GOAL as i32 as crate::stdlib::intptr_t,
        num,
        goal,
    ) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotGetMapLocationGoal(
    mut name: *mut libc::c_char,
    mut goal: *mut libc::c_void,
) -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_GET_MAP_LOCATION_GOAL as i32 as crate::stdlib::intptr_t,
        name,
        goal,
    ) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotAvoidGoalTime(mut goalstate: i32, mut number: i32) -> f32 {
    let mut fi: crate::src::qcommon::q_shared::floatint_t =
        crate::src::qcommon::q_shared::floatint_t { f: 0. };
    fi.i = syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_AVOID_GOAL_TIME as i32 as crate::stdlib::intptr_t,
        goalstate,
        number,
    ) as i32;
    return fi.f;
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotSetAvoidGoalTime(
    mut goalstate: i32,
    mut number: i32,
    mut avoidtime: f32,
) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_SET_AVOID_GOAL_TIME as i32 as crate::stdlib::intptr_t,
        goalstate,
        number,
        PASSFLOAT(avoidtime),
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotInitLevelItems() {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_INIT_LEVEL_ITEMS as i32 as crate::stdlib::intptr_t,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotUpdateEntityItems() {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_UPDATE_ENTITY_ITEMS as i32 as crate::stdlib::intptr_t,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotLoadItemWeights(
    mut goalstate: i32,
    mut filename: *mut libc::c_char,
) -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_LOAD_ITEM_WEIGHTS as i32 as crate::stdlib::intptr_t,
        goalstate,
        filename,
    ) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotFreeItemWeights(mut goalstate: i32) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_FREE_ITEM_WEIGHTS as i32 as crate::stdlib::intptr_t,
        goalstate,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotInterbreedGoalFuzzyLogic(
    mut parent1: i32,
    mut parent2: i32,
    mut child: i32,
) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_INTERBREED_GOAL_FUZZY_LOGIC as i32 as crate::stdlib::intptr_t,
        parent1,
        parent2,
        child,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotSaveGoalFuzzyLogic(
    mut goalstate: i32,
    mut filename: *mut libc::c_char,
) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_SAVE_GOAL_FUZZY_LOGIC as i32 as crate::stdlib::intptr_t,
        goalstate,
        filename,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotMutateGoalFuzzyLogic(mut goalstate: i32, mut range: f32) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_MUTATE_GOAL_FUZZY_LOGIC as i32 as crate::stdlib::intptr_t,
        goalstate,
        PASSFLOAT(range),
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotAllocGoalState(mut state: i32) -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_ALLOC_GOAL_STATE as i32 as crate::stdlib::intptr_t,
        state,
    ) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotFreeGoalState(mut handle: i32) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_FREE_GOAL_STATE as i32 as crate::stdlib::intptr_t,
        handle,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotResetMoveState(mut movestate: i32) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_RESET_MOVE_STATE as i32 as crate::stdlib::intptr_t,
        movestate,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotAddAvoidSpot(
    mut movestate: i32,
    mut origin: *mut crate::src::qcommon::q_shared::vec_t,
    mut radius: f32,
    mut type_0: i32,
) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_ADD_AVOID_SPOT as i32 as crate::stdlib::intptr_t,
        movestate,
        origin,
        PASSFLOAT(radius),
        type_0,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotMoveToGoal(
    mut result: *mut libc::c_void,
    mut movestate: i32,
    mut goal: *mut libc::c_void,
    mut travelflags: i32,
) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_MOVE_TO_GOAL as i32 as crate::stdlib::intptr_t,
        result,
        movestate,
        goal,
        travelflags,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotMoveInDirection(
    mut movestate: i32,
    mut dir: *mut crate::src::qcommon::q_shared::vec_t,
    mut speed: f32,
    mut type_0: i32,
) -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_MOVE_IN_DIRECTION as i32 as crate::stdlib::intptr_t,
        movestate,
        dir,
        PASSFLOAT(speed),
        type_0,
    ) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotResetAvoidReach(mut movestate: i32) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_RESET_AVOID_REACH as i32 as crate::stdlib::intptr_t,
        movestate,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotResetLastAvoidReach(mut movestate: i32) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_RESET_LAST_AVOID_REACH as i32 as crate::stdlib::intptr_t,
        movestate,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotReachabilityArea(
    mut origin: *mut crate::src::qcommon::q_shared::vec_t,
    mut testground: i32,
) -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_REACHABILITY_AREA as i32 as crate::stdlib::intptr_t,
        origin,
        testground,
    ) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotMovementViewTarget(
    mut movestate: i32,
    mut goal: *mut libc::c_void,
    mut travelflags: i32,
    mut lookahead: f32,
    mut target: *mut crate::src::qcommon::q_shared::vec_t,
) -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_MOVEMENT_VIEW_TARGET as i32 as crate::stdlib::intptr_t,
        movestate,
        goal,
        travelflags,
        PASSFLOAT(lookahead),
        target,
    ) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotPredictVisiblePosition(
    mut origin: *mut crate::src::qcommon::q_shared::vec_t,
    mut areanum: i32,
    mut goal: *mut libc::c_void,
    mut travelflags: i32,
    mut target: *mut crate::src::qcommon::q_shared::vec_t,
) -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_PREDICT_VISIBLE_POSITION as i32 as crate::stdlib::intptr_t,
        origin,
        areanum,
        goal,
        travelflags,
        target,
    ) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotAllocMoveState() -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_ALLOC_MOVE_STATE as i32 as crate::stdlib::intptr_t,
    ) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotFreeMoveState(mut handle: i32) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_FREE_MOVE_STATE as i32 as crate::stdlib::intptr_t,
        handle,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotInitMoveState(mut handle: i32, mut initmove: *mut libc::c_void) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_INIT_MOVE_STATE as i32 as crate::stdlib::intptr_t,
        handle,
        initmove,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotChooseBestFightWeapon(
    mut weaponstate: i32,
    mut inventory: *mut i32,
) -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_CHOOSE_BEST_FIGHT_WEAPON as i32 as crate::stdlib::intptr_t,
        weaponstate,
        inventory,
    ) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotGetWeaponInfo(
    mut weaponstate: i32,
    mut weapon: i32,
    mut weaponinfo: *mut libc::c_void,
) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_GET_WEAPON_INFO as i32 as crate::stdlib::intptr_t,
        weaponstate,
        weapon,
        weaponinfo,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotLoadWeaponWeights(
    mut weaponstate: i32,
    mut filename: *mut libc::c_char,
) -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_LOAD_WEAPON_WEIGHTS as i32 as crate::stdlib::intptr_t,
        weaponstate,
        filename,
    ) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotAllocWeaponState() -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_ALLOC_WEAPON_STATE as i32 as crate::stdlib::intptr_t,
    ) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotFreeWeaponState(mut weaponstate: i32) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_FREE_WEAPON_STATE as i32 as crate::stdlib::intptr_t,
        weaponstate,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotResetWeaponState(mut weaponstate: i32) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_RESET_WEAPON_STATE as i32 as crate::stdlib::intptr_t,
        weaponstate,
    );
}
// fsOrigin_t
/* struct bot_updateentity_s */
/* struct aas_areainfo_s */
/* struct aas_entityinfo_s */
/*struct aas_predictroute_s*/
/*struct aas_altroutegoal_s*/
/* aas_clientmove_s */
/* struct bot_input_s */
/* struct bot_consolemessage_s */
/* struct bot_match_s */
/* struct bot_match_s */
/* struct bot_goal_s */
/* struct bot_goal_s */
/* struct bot_goal_s */
/* struct bot_goal_s */
/* struct bot_goal_s */
/* struct bot_goal_s */
/* struct bot_goal_s */
/* struct bot_goal_s */
/* struct bot_goal_s */
/* struct bot_moveresult_s */
/* struct bot_goal_s */
/* struct bot_goal_s */
/* struct bot_goal_s */
/* struct bot_initmove_s */
/* struct weaponinfo_s */
#[no_mangle]

pub unsafe extern "C" fn trap_GeneticParentsAndChildSelection(
    mut numranks: i32,
    mut ranks: *mut f32,
    mut parent1: *mut i32,
    mut parent2: *mut i32,
    mut child: *mut i32,
) -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_GENETIC_PARENTS_AND_CHILD_SELECTION as i32
            as crate::stdlib::intptr_t,
        numranks,
        ranks,
        parent1,
        parent2,
        child,
    ) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn trap_PC_LoadSource(mut filename: *const libc::c_char) -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_PC_LOAD_SOURCE as i32 as crate::stdlib::intptr_t,
        filename,
    ) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn trap_PC_FreeSource(mut handle: i32) -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_PC_FREE_SOURCE as i32 as crate::stdlib::intptr_t,
        handle,
    ) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn trap_PC_ReadToken(
    mut handle: i32,
    mut pc_token: *mut crate::src::qcommon::q_shared::pc_token_t,
) -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_PC_READ_TOKEN as i32 as crate::stdlib::intptr_t,
        handle,
        pc_token,
    ) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn trap_PC_SourceFileAndLine(
    mut handle: i32,
    mut filename: *mut libc::c_char,
    mut line: *mut i32,
) -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_PC_SOURCE_FILE_AND_LINE as i32 as crate::stdlib::intptr_t,
        handle,
        filename,
        line,
    ) as i32;
}
unsafe extern "C" fn run_static_initializers() {
    syscall = ::std::mem::transmute::<
        isize,
        Option<unsafe extern "C" fn(_: crate::stdlib::intptr_t, _: ...) -> crate::stdlib::intptr_t>,
    >(-(1 as i32) as isize)
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];

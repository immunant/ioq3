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

pub unsafe extern "C" fn PASSFLOAT(mut x: libc::c_float) -> libc::c_int {
    let mut fi: crate::src::qcommon::q_shared::floatint_t =
        crate::src::qcommon::q_shared::floatint_t { f: 0. };
    fi.f = x;
    return fi.i;
}
#[no_mangle]

pub unsafe extern "C" fn trap_Print(mut text: *const libc::c_char) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::G_PRINT as libc::c_int as crate::stdlib::intptr_t,
        text,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_Error(mut text: *const libc::c_char) -> ! {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::G_ERROR as libc::c_int as crate::stdlib::intptr_t,
        text,
    );
    // shut up GCC warning about returning functions, because we know better
    ::libc::exit(1 as libc::c_int);
}
#[no_mangle]

pub unsafe extern "C" fn trap_Milliseconds() -> libc::c_int {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::G_MILLISECONDS as libc::c_int as crate::stdlib::intptr_t,
    ) as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn trap_Argc() -> libc::c_int {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::G_ARGC as libc::c_int as crate::stdlib::intptr_t,
    ) as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn trap_Argv(
    mut n: libc::c_int,
    mut buffer: *mut libc::c_char,
    mut bufferLength: libc::c_int,
) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::G_ARGV as libc::c_int as crate::stdlib::intptr_t,
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
) -> libc::c_int {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::G_FS_FOPEN_FILE as libc::c_int as crate::stdlib::intptr_t,
        qpath,
        f,
        mode as libc::c_uint,
    ) as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn trap_FS_Read(
    mut buffer: *mut libc::c_void,
    mut len: libc::c_int,
    mut f: crate::src::qcommon::q_shared::fileHandle_t,
) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::G_FS_READ as libc::c_int as crate::stdlib::intptr_t,
        buffer,
        len,
        f,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_FS_Write(
    mut buffer: *const libc::c_void,
    mut len: libc::c_int,
    mut f: crate::src::qcommon::q_shared::fileHandle_t,
) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::G_FS_WRITE as libc::c_int as crate::stdlib::intptr_t,
        buffer,
        len,
        f,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_FS_FCloseFile(mut f: crate::src::qcommon::q_shared::fileHandle_t) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::G_FS_FCLOSE_FILE as libc::c_int as crate::stdlib::intptr_t,
        f,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_FS_GetFileList(
    mut path: *const libc::c_char,
    mut extension: *const libc::c_char,
    mut listbuf: *mut libc::c_char,
    mut bufsize: libc::c_int,
) -> libc::c_int {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::G_FS_GETFILELIST as libc::c_int as crate::stdlib::intptr_t,
        path,
        extension,
        listbuf,
        bufsize,
    ) as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn trap_FS_Seek(
    mut f: crate::src::qcommon::q_shared::fileHandle_t,
    mut offset: libc::c_long,
    mut origin: libc::c_int,
) -> libc::c_int {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::G_FS_SEEK as libc::c_int as crate::stdlib::intptr_t,
        f,
        offset,
        origin,
    ) as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn trap_SendConsoleCommand(
    mut exec_when: libc::c_int,
    mut text: *const libc::c_char,
) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::G_SEND_CONSOLE_COMMAND as libc::c_int as crate::stdlib::intptr_t,
        exec_when,
        text,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_Cvar_Register(
    mut cvar: *mut crate::src::qcommon::q_shared::vmCvar_t,
    mut var_name: *const libc::c_char,
    mut value: *const libc::c_char,
    mut flags: libc::c_int,
) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::G_CVAR_REGISTER as libc::c_int as crate::stdlib::intptr_t,
        cvar,
        var_name,
        value,
        flags,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_Cvar_Update(mut cvar: *mut crate::src::qcommon::q_shared::vmCvar_t) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::G_CVAR_UPDATE as libc::c_int as crate::stdlib::intptr_t,
        cvar,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_Cvar_Set(
    mut var_name: *const libc::c_char,
    mut value: *const libc::c_char,
) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::G_CVAR_SET as libc::c_int as crate::stdlib::intptr_t,
        var_name,
        value,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_Cvar_VariableIntegerValue(
    mut var_name: *const libc::c_char,
) -> libc::c_int {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::G_CVAR_VARIABLE_INTEGER_VALUE as libc::c_int as crate::stdlib::intptr_t,
        var_name,
    ) as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn trap_Cvar_VariableStringBuffer(
    mut var_name: *const libc::c_char,
    mut buffer: *mut libc::c_char,
    mut bufsize: libc::c_int,
) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::G_CVAR_VARIABLE_STRING_BUFFER as libc::c_int as crate::stdlib::intptr_t,
        var_name,
        buffer,
        bufsize,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_LocateGameData(
    mut gEnts: *mut crate::g_local_h::gentity_t,
    mut numGEntities: libc::c_int,
    mut sizeofGEntity_t: libc::c_int,
    mut clients: *mut crate::src::qcommon::q_shared::playerState_t,
    mut sizeofGClient: libc::c_int,
) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::G_LOCATE_GAME_DATA as libc::c_int as crate::stdlib::intptr_t,
        gEnts,
        numGEntities,
        sizeofGEntity_t,
        clients,
        sizeofGClient,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_DropClient(
    mut clientNum: libc::c_int,
    mut reason: *const libc::c_char,
) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::G_DROP_CLIENT as libc::c_int as crate::stdlib::intptr_t,
        clientNum,
        reason,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_SendServerCommand(
    mut clientNum: libc::c_int,
    mut text: *const libc::c_char,
) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::G_SEND_SERVER_COMMAND as libc::c_int as crate::stdlib::intptr_t,
        clientNum,
        text,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_SetConfigstring(
    mut num: libc::c_int,
    mut string: *const libc::c_char,
) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::G_SET_CONFIGSTRING as libc::c_int as crate::stdlib::intptr_t,
        num,
        string,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_GetConfigstring(
    mut num: libc::c_int,
    mut buffer: *mut libc::c_char,
    mut bufferSize: libc::c_int,
) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::G_GET_CONFIGSTRING as libc::c_int as crate::stdlib::intptr_t,
        num,
        buffer,
        bufferSize,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_GetUserinfo(
    mut num: libc::c_int,
    mut buffer: *mut libc::c_char,
    mut bufferSize: libc::c_int,
) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::G_GET_USERINFO as libc::c_int as crate::stdlib::intptr_t,
        num,
        buffer,
        bufferSize,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_SetUserinfo(mut num: libc::c_int, mut buffer: *const libc::c_char) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::G_SET_USERINFO as libc::c_int as crate::stdlib::intptr_t,
        num,
        buffer,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_GetServerinfo(
    mut buffer: *mut libc::c_char,
    mut bufferSize: libc::c_int,
) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::G_GET_SERVERINFO as libc::c_int as crate::stdlib::intptr_t,
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
        crate::g_public_h::G_SET_BRUSH_MODEL as libc::c_int as crate::stdlib::intptr_t,
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
    mut passEntityNum: libc::c_int,
    mut contentmask: libc::c_int,
) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::G_TRACE as libc::c_int as crate::stdlib::intptr_t,
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
    mut passEntityNum: libc::c_int,
    mut contentmask: libc::c_int,
) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::G_TRACECAPSULE as libc::c_int as crate::stdlib::intptr_t,
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
    mut passEntityNum: libc::c_int,
) -> libc::c_int {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::G_POINT_CONTENTS as libc::c_int as crate::stdlib::intptr_t,
        point,
        passEntityNum,
    ) as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn trap_InPVS(
    mut p1: *const crate::src::qcommon::q_shared::vec_t,
    mut p2: *const crate::src::qcommon::q_shared::vec_t,
) -> crate::src::qcommon::q_shared::qboolean {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::G_IN_PVS as libc::c_int as crate::stdlib::intptr_t,
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
        crate::g_public_h::G_IN_PVS_IGNORE_PORTALS as libc::c_int as crate::stdlib::intptr_t,
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
        crate::g_public_h::G_ADJUST_AREA_PORTAL_STATE as libc::c_int as crate::stdlib::intptr_t,
        ent,
        open as libc::c_uint,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_AreasConnected(
    mut area1: libc::c_int,
    mut area2: libc::c_int,
) -> crate::src::qcommon::q_shared::qboolean {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::G_AREAS_CONNECTED as libc::c_int as crate::stdlib::intptr_t,
        area1,
        area2,
    ) as crate::src::qcommon::q_shared::qboolean;
}
#[no_mangle]

pub unsafe extern "C" fn trap_LinkEntity(mut ent: *mut crate::g_local_h::gentity_t) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::G_LINKENTITY as libc::c_int as crate::stdlib::intptr_t,
        ent,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_UnlinkEntity(mut ent: *mut crate::g_local_h::gentity_t) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::G_UNLINKENTITY as libc::c_int as crate::stdlib::intptr_t,
        ent,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_EntitiesInBox(
    mut mins: *const crate::src::qcommon::q_shared::vec_t,
    mut maxs: *const crate::src::qcommon::q_shared::vec_t,
    mut list: *mut libc::c_int,
    mut maxcount: libc::c_int,
) -> libc::c_int {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::G_ENTITIES_IN_BOX as libc::c_int as crate::stdlib::intptr_t,
        mins,
        maxs,
        list,
        maxcount,
    ) as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn trap_EntityContact(
    mut mins: *const crate::src::qcommon::q_shared::vec_t,
    mut maxs: *const crate::src::qcommon::q_shared::vec_t,
    mut ent: *const crate::g_local_h::gentity_t,
) -> crate::src::qcommon::q_shared::qboolean {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::G_ENTITY_CONTACT as libc::c_int as crate::stdlib::intptr_t,
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
        crate::g_public_h::G_ENTITY_CONTACTCAPSULE as libc::c_int as crate::stdlib::intptr_t,
        mins,
        maxs,
        ent,
    ) as crate::src::qcommon::q_shared::qboolean;
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotAllocateClient() -> libc::c_int {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::G_BOT_ALLOCATE_CLIENT as libc::c_int as crate::stdlib::intptr_t,
    ) as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotFreeClient(mut clientNum: libc::c_int) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::G_BOT_FREE_CLIENT as libc::c_int as crate::stdlib::intptr_t,
        clientNum,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_GetUsercmd(
    mut clientNum: libc::c_int,
    mut cmd: *mut crate::src::qcommon::q_shared::usercmd_t,
) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::G_GET_USERCMD as libc::c_int as crate::stdlib::intptr_t,
        clientNum,
        cmd,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_GetEntityToken(
    mut buffer: *mut libc::c_char,
    mut bufferSize: libc::c_int,
) -> crate::src::qcommon::q_shared::qboolean {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::G_GET_ENTITY_TOKEN as libc::c_int as crate::stdlib::intptr_t,
        buffer,
        bufferSize,
    ) as crate::src::qcommon::q_shared::qboolean;
}
#[no_mangle]

pub unsafe extern "C" fn trap_DebugPolygonCreate(
    mut color: libc::c_int,
    mut numPoints: libc::c_int,
    mut points: *mut crate::src::qcommon::q_shared::vec3_t,
) -> libc::c_int {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::G_DEBUG_POLYGON_CREATE as libc::c_int as crate::stdlib::intptr_t,
        color,
        numPoints,
        points,
    ) as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn trap_DebugPolygonDelete(mut id: libc::c_int) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::G_DEBUG_POLYGON_DELETE as libc::c_int as crate::stdlib::intptr_t,
        id,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_RealTime(
    mut qtime: *mut crate::src::qcommon::q_shared::qtime_t,
) -> libc::c_int {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::G_REAL_TIME as libc::c_int as crate::stdlib::intptr_t,
        qtime,
    ) as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn trap_SnapVector(mut v: *mut libc::c_float) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::G_SNAPVECTOR as libc::c_int as crate::stdlib::intptr_t,
        v,
    );
}
// BotLib traps start here
#[no_mangle]

pub unsafe extern "C" fn trap_BotLibSetup() -> libc::c_int {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_SETUP as libc::c_int as crate::stdlib::intptr_t,
    ) as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotLibShutdown() -> libc::c_int {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_SHUTDOWN as libc::c_int as crate::stdlib::intptr_t,
    ) as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotLibVarSet(
    mut var_name: *mut libc::c_char,
    mut value: *mut libc::c_char,
) -> libc::c_int {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_LIBVAR_SET as libc::c_int as crate::stdlib::intptr_t,
        var_name,
        value,
    ) as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotLibVarGet(
    mut var_name: *mut libc::c_char,
    mut value: *mut libc::c_char,
    mut size: libc::c_int,
) -> libc::c_int {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_LIBVAR_GET as libc::c_int as crate::stdlib::intptr_t,
        var_name,
        value,
        size,
    ) as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotLibDefine(mut string: *mut libc::c_char) -> libc::c_int {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_PC_ADD_GLOBAL_DEFINE as libc::c_int as crate::stdlib::intptr_t,
        string,
    ) as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotLibStartFrame(mut time: libc::c_float) -> libc::c_int {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_START_FRAME as libc::c_int as crate::stdlib::intptr_t,
        PASSFLOAT(time),
    ) as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotLibLoadMap(mut mapname: *const libc::c_char) -> libc::c_int {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_LOAD_MAP as libc::c_int as crate::stdlib::intptr_t,
        mapname,
    ) as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotLibUpdateEntity(
    mut ent: libc::c_int,
    mut bue: *mut libc::c_void,
) -> libc::c_int {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_UPDATENTITY as libc::c_int as crate::stdlib::intptr_t,
        ent,
        bue,
    ) as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotLibTest(
    mut parm0: libc::c_int,
    mut parm1: *mut libc::c_char,
    mut parm2: *mut crate::src::qcommon::q_shared::vec_t,
    mut parm3: *mut crate::src::qcommon::q_shared::vec_t,
) -> libc::c_int {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_TEST as libc::c_int as crate::stdlib::intptr_t,
        parm0,
        parm1,
        parm2,
        parm3,
    ) as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotGetSnapshotEntity(
    mut clientNum: libc::c_int,
    mut sequence: libc::c_int,
) -> libc::c_int {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_GET_SNAPSHOT_ENTITY as libc::c_int as crate::stdlib::intptr_t,
        clientNum,
        sequence,
    ) as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotGetServerCommand(
    mut clientNum: libc::c_int,
    mut message: *mut libc::c_char,
    mut size: libc::c_int,
) -> libc::c_int {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_GET_CONSOLE_MESSAGE as libc::c_int as crate::stdlib::intptr_t,
        clientNum,
        message,
        size,
    ) as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotUserCommand(
    mut clientNum: libc::c_int,
    mut ucmd: *mut crate::src::qcommon::q_shared::usercmd_t,
) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_USER_COMMAND as libc::c_int as crate::stdlib::intptr_t,
        clientNum,
        ucmd,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_AAS_EntityInfo(mut entnum: libc::c_int, mut info: *mut libc::c_void) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AAS_ENTITY_INFO as libc::c_int as crate::stdlib::intptr_t,
        entnum,
        info,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_AAS_Initialized() -> libc::c_int {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AAS_INITIALIZED as libc::c_int as crate::stdlib::intptr_t,
    ) as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn trap_AAS_PresenceTypeBoundingBox(
    mut presencetype: libc::c_int,
    mut mins: *mut crate::src::qcommon::q_shared::vec_t,
    mut maxs: *mut crate::src::qcommon::q_shared::vec_t,
) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AAS_PRESENCE_TYPE_BOUNDING_BOX as libc::c_int
            as crate::stdlib::intptr_t,
        presencetype,
        mins,
        maxs,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_AAS_Time() -> libc::c_float {
    let mut fi: crate::src::qcommon::q_shared::floatint_t =
        crate::src::qcommon::q_shared::floatint_t { f: 0. };
    fi.i = syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AAS_TIME as libc::c_int as crate::stdlib::intptr_t,
    ) as libc::c_int;
    return fi.f;
}
#[no_mangle]

pub unsafe extern "C" fn trap_AAS_PointAreaNum(
    mut point: *mut crate::src::qcommon::q_shared::vec_t,
) -> libc::c_int {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AAS_POINT_AREA_NUM as libc::c_int as crate::stdlib::intptr_t,
        point,
    ) as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn trap_AAS_PointReachabilityAreaIndex(
    mut point: *mut crate::src::qcommon::q_shared::vec_t,
) -> libc::c_int {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AAS_POINT_REACHABILITY_AREA_INDEX as libc::c_int
            as crate::stdlib::intptr_t,
        point,
    ) as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn trap_AAS_TraceAreas(
    mut start: *mut crate::src::qcommon::q_shared::vec_t,
    mut end: *mut crate::src::qcommon::q_shared::vec_t,
    mut areas: *mut libc::c_int,
    mut points: *mut crate::src::qcommon::q_shared::vec3_t,
    mut maxareas: libc::c_int,
) -> libc::c_int {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AAS_TRACE_AREAS as libc::c_int as crate::stdlib::intptr_t,
        start,
        end,
        areas,
        points,
        maxareas,
    ) as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn trap_AAS_BBoxAreas(
    mut absmins: *mut crate::src::qcommon::q_shared::vec_t,
    mut absmaxs: *mut crate::src::qcommon::q_shared::vec_t,
    mut areas: *mut libc::c_int,
    mut maxareas: libc::c_int,
) -> libc::c_int {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AAS_BBOX_AREAS as libc::c_int as crate::stdlib::intptr_t,
        absmins,
        absmaxs,
        areas,
        maxareas,
    ) as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn trap_AAS_AreaInfo(
    mut areanum: libc::c_int,
    mut info: *mut libc::c_void,
) -> libc::c_int {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AAS_AREA_INFO as libc::c_int as crate::stdlib::intptr_t,
        areanum,
        info,
    ) as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn trap_AAS_PointContents(
    mut point: *mut crate::src::qcommon::q_shared::vec_t,
) -> libc::c_int {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AAS_POINT_CONTENTS as libc::c_int as crate::stdlib::intptr_t,
        point,
    ) as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn trap_AAS_NextBSPEntity(mut ent: libc::c_int) -> libc::c_int {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AAS_NEXT_BSP_ENTITY as libc::c_int as crate::stdlib::intptr_t,
        ent,
    ) as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn trap_AAS_ValueForBSPEpairKey(
    mut ent: libc::c_int,
    mut key: *mut libc::c_char,
    mut value: *mut libc::c_char,
    mut size: libc::c_int,
) -> libc::c_int {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AAS_VALUE_FOR_BSP_EPAIR_KEY as libc::c_int
            as crate::stdlib::intptr_t,
        ent,
        key,
        value,
        size,
    ) as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn trap_AAS_VectorForBSPEpairKey(
    mut ent: libc::c_int,
    mut key: *mut libc::c_char,
    mut v: *mut crate::src::qcommon::q_shared::vec_t,
) -> libc::c_int {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AAS_VECTOR_FOR_BSP_EPAIR_KEY as libc::c_int
            as crate::stdlib::intptr_t,
        ent,
        key,
        v,
    ) as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn trap_AAS_FloatForBSPEpairKey(
    mut ent: libc::c_int,
    mut key: *mut libc::c_char,
    mut value: *mut libc::c_float,
) -> libc::c_int {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AAS_FLOAT_FOR_BSP_EPAIR_KEY as libc::c_int
            as crate::stdlib::intptr_t,
        ent,
        key,
        value,
    ) as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn trap_AAS_IntForBSPEpairKey(
    mut ent: libc::c_int,
    mut key: *mut libc::c_char,
    mut value: *mut libc::c_int,
) -> libc::c_int {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AAS_INT_FOR_BSP_EPAIR_KEY as libc::c_int
            as crate::stdlib::intptr_t,
        ent,
        key,
        value,
    ) as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn trap_AAS_AreaReachability(mut areanum: libc::c_int) -> libc::c_int {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AAS_AREA_REACHABILITY as libc::c_int as crate::stdlib::intptr_t,
        areanum,
    ) as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn trap_AAS_AreaTravelTimeToGoalArea(
    mut areanum: libc::c_int,
    mut origin: *mut crate::src::qcommon::q_shared::vec_t,
    mut goalareanum: libc::c_int,
    mut travelflags: libc::c_int,
) -> libc::c_int {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AAS_AREA_TRAVEL_TIME_TO_GOAL_AREA as libc::c_int
            as crate::stdlib::intptr_t,
        areanum,
        origin,
        goalareanum,
        travelflags,
    ) as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn trap_AAS_EnableRoutingArea(
    mut areanum: libc::c_int,
    mut enable: libc::c_int,
) -> libc::c_int {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AAS_ENABLE_ROUTING_AREA as libc::c_int as crate::stdlib::intptr_t,
        areanum,
        enable,
    ) as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn trap_AAS_PredictRoute(
    mut route: *mut libc::c_void,
    mut areanum: libc::c_int,
    mut origin: *mut crate::src::qcommon::q_shared::vec_t,
    mut goalareanum: libc::c_int,
    mut travelflags: libc::c_int,
    mut maxareas: libc::c_int,
    mut maxtime: libc::c_int,
    mut stopevent: libc::c_int,
    mut stopcontents: libc::c_int,
    mut stoptfl: libc::c_int,
    mut stopareanum: libc::c_int,
) -> libc::c_int {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AAS_PREDICT_ROUTE as libc::c_int as crate::stdlib::intptr_t,
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
    ) as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn trap_AAS_AlternativeRouteGoals(
    mut start: *mut crate::src::qcommon::q_shared::vec_t,
    mut startareanum: libc::c_int,
    mut goal: *mut crate::src::qcommon::q_shared::vec_t,
    mut goalareanum: libc::c_int,
    mut travelflags: libc::c_int,
    mut altroutegoals: *mut libc::c_void,
    mut maxaltroutegoals: libc::c_int,
    mut type_0: libc::c_int,
) -> libc::c_int {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AAS_ALTERNATIVE_ROUTE_GOAL as libc::c_int
            as crate::stdlib::intptr_t,
        start,
        startareanum,
        goal,
        goalareanum,
        travelflags,
        altroutegoals,
        maxaltroutegoals,
        type_0,
    ) as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn trap_AAS_Swimming(
    mut origin: *mut crate::src::qcommon::q_shared::vec_t,
) -> libc::c_int {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AAS_SWIMMING as libc::c_int as crate::stdlib::intptr_t,
        origin,
    ) as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn trap_AAS_PredictClientMovement(
    mut move_0: *mut libc::c_void,
    mut entnum: libc::c_int,
    mut origin: *mut crate::src::qcommon::q_shared::vec_t,
    mut presencetype: libc::c_int,
    mut onground: libc::c_int,
    mut velocity: *mut crate::src::qcommon::q_shared::vec_t,
    mut cmdmove: *mut crate::src::qcommon::q_shared::vec_t,
    mut cmdframes: libc::c_int,
    mut maxframes: libc::c_int,
    mut frametime: libc::c_float,
    mut stopevent: libc::c_int,
    mut stopareanum: libc::c_int,
    mut visualize: libc::c_int,
) -> libc::c_int {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AAS_PREDICT_CLIENT_MOVEMENT as libc::c_int
            as crate::stdlib::intptr_t,
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
    ) as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn trap_EA_Say(mut client: libc::c_int, mut str: *mut libc::c_char) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_EA_SAY as libc::c_int as crate::stdlib::intptr_t,
        client,
        str,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_EA_SayTeam(mut client: libc::c_int, mut str: *mut libc::c_char) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_EA_SAY_TEAM as libc::c_int as crate::stdlib::intptr_t,
        client,
        str,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_EA_Command(mut client: libc::c_int, mut command: *mut libc::c_char) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_EA_COMMAND as libc::c_int as crate::stdlib::intptr_t,
        client,
        command,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_EA_Action(mut client: libc::c_int, mut action: libc::c_int) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_EA_ACTION as libc::c_int as crate::stdlib::intptr_t,
        client,
        action,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_EA_Gesture(mut client: libc::c_int) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_EA_GESTURE as libc::c_int as crate::stdlib::intptr_t,
        client,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_EA_Talk(mut client: libc::c_int) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_EA_TALK as libc::c_int as crate::stdlib::intptr_t,
        client,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_EA_Attack(mut client: libc::c_int) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_EA_ATTACK as libc::c_int as crate::stdlib::intptr_t,
        client,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_EA_Use(mut client: libc::c_int) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_EA_USE as libc::c_int as crate::stdlib::intptr_t,
        client,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_EA_Respawn(mut client: libc::c_int) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_EA_RESPAWN as libc::c_int as crate::stdlib::intptr_t,
        client,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_EA_Crouch(mut client: libc::c_int) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_EA_CROUCH as libc::c_int as crate::stdlib::intptr_t,
        client,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_EA_MoveUp(mut client: libc::c_int) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_EA_MOVE_UP as libc::c_int as crate::stdlib::intptr_t,
        client,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_EA_MoveDown(mut client: libc::c_int) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_EA_MOVE_DOWN as libc::c_int as crate::stdlib::intptr_t,
        client,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_EA_MoveForward(mut client: libc::c_int) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_EA_MOVE_FORWARD as libc::c_int as crate::stdlib::intptr_t,
        client,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_EA_MoveBack(mut client: libc::c_int) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_EA_MOVE_BACK as libc::c_int as crate::stdlib::intptr_t,
        client,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_EA_MoveLeft(mut client: libc::c_int) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_EA_MOVE_LEFT as libc::c_int as crate::stdlib::intptr_t,
        client,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_EA_MoveRight(mut client: libc::c_int) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_EA_MOVE_RIGHT as libc::c_int as crate::stdlib::intptr_t,
        client,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_EA_SelectWeapon(mut client: libc::c_int, mut weapon: libc::c_int) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_EA_SELECT_WEAPON as libc::c_int as crate::stdlib::intptr_t,
        client,
        weapon,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_EA_Jump(mut client: libc::c_int) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_EA_JUMP as libc::c_int as crate::stdlib::intptr_t,
        client,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_EA_DelayedJump(mut client: libc::c_int) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_EA_DELAYED_JUMP as libc::c_int as crate::stdlib::intptr_t,
        client,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_EA_Move(
    mut client: libc::c_int,
    mut dir: *mut crate::src::qcommon::q_shared::vec_t,
    mut speed: libc::c_float,
) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_EA_MOVE as libc::c_int as crate::stdlib::intptr_t,
        client,
        dir,
        PASSFLOAT(speed),
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_EA_View(
    mut client: libc::c_int,
    mut viewangles: *mut crate::src::qcommon::q_shared::vec_t,
) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_EA_VIEW as libc::c_int as crate::stdlib::intptr_t,
        client,
        viewangles,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_EA_EndRegular(mut client: libc::c_int, mut thinktime: libc::c_float) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_EA_END_REGULAR as libc::c_int as crate::stdlib::intptr_t,
        client,
        PASSFLOAT(thinktime),
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_EA_GetInput(
    mut client: libc::c_int,
    mut thinktime: libc::c_float,
    mut input: *mut libc::c_void,
) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_EA_GET_INPUT as libc::c_int as crate::stdlib::intptr_t,
        client,
        PASSFLOAT(thinktime),
        input,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_EA_ResetInput(mut client: libc::c_int) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_EA_RESET_INPUT as libc::c_int as crate::stdlib::intptr_t,
        client,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotLoadCharacter(
    mut charfile: *mut libc::c_char,
    mut skill: libc::c_float,
) -> libc::c_int {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_LOAD_CHARACTER as libc::c_int as crate::stdlib::intptr_t,
        charfile,
        PASSFLOAT(skill),
    ) as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotFreeCharacter(mut character: libc::c_int) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_FREE_CHARACTER as libc::c_int as crate::stdlib::intptr_t,
        character,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_Characteristic_Float(
    mut character: libc::c_int,
    mut index: libc::c_int,
) -> libc::c_float {
    let mut fi: crate::src::qcommon::q_shared::floatint_t =
        crate::src::qcommon::q_shared::floatint_t { f: 0. };
    fi.i = syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_CHARACTERISTIC_FLOAT as libc::c_int as crate::stdlib::intptr_t,
        character,
        index,
    ) as libc::c_int;
    return fi.f;
}
#[no_mangle]

pub unsafe extern "C" fn trap_Characteristic_BFloat(
    mut character: libc::c_int,
    mut index: libc::c_int,
    mut min: libc::c_float,
    mut max: libc::c_float,
) -> libc::c_float {
    let mut fi: crate::src::qcommon::q_shared::floatint_t =
        crate::src::qcommon::q_shared::floatint_t { f: 0. };
    fi.i = syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_CHARACTERISTIC_BFLOAT as libc::c_int
            as crate::stdlib::intptr_t,
        character,
        index,
        PASSFLOAT(min),
        PASSFLOAT(max),
    ) as libc::c_int;
    return fi.f;
}
#[no_mangle]

pub unsafe extern "C" fn trap_Characteristic_Integer(
    mut character: libc::c_int,
    mut index: libc::c_int,
) -> libc::c_int {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_CHARACTERISTIC_INTEGER as libc::c_int
            as crate::stdlib::intptr_t,
        character,
        index,
    ) as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn trap_Characteristic_BInteger(
    mut character: libc::c_int,
    mut index: libc::c_int,
    mut min: libc::c_int,
    mut max: libc::c_int,
) -> libc::c_int {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_CHARACTERISTIC_BINTEGER as libc::c_int
            as crate::stdlib::intptr_t,
        character,
        index,
        min,
        max,
    ) as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn trap_Characteristic_String(
    mut character: libc::c_int,
    mut index: libc::c_int,
    mut buf: *mut libc::c_char,
    mut size: libc::c_int,
) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_CHARACTERISTIC_STRING as libc::c_int
            as crate::stdlib::intptr_t,
        character,
        index,
        buf,
        size,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotAllocChatState() -> libc::c_int {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_ALLOC_CHAT_STATE as libc::c_int as crate::stdlib::intptr_t,
    ) as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotFreeChatState(mut handle: libc::c_int) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_FREE_CHAT_STATE as libc::c_int as crate::stdlib::intptr_t,
        handle,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotQueueConsoleMessage(
    mut chatstate: libc::c_int,
    mut type_0: libc::c_int,
    mut message: *mut libc::c_char,
) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_QUEUE_CONSOLE_MESSAGE as libc::c_int
            as crate::stdlib::intptr_t,
        chatstate,
        type_0,
        message,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotRemoveConsoleMessage(
    mut chatstate: libc::c_int,
    mut handle: libc::c_int,
) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_REMOVE_CONSOLE_MESSAGE as libc::c_int
            as crate::stdlib::intptr_t,
        chatstate,
        handle,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotNextConsoleMessage(
    mut chatstate: libc::c_int,
    mut cm: *mut libc::c_void,
) -> libc::c_int {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_NEXT_CONSOLE_MESSAGE as libc::c_int as crate::stdlib::intptr_t,
        chatstate,
        cm,
    ) as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotNumConsoleMessages(mut chatstate: libc::c_int) -> libc::c_int {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_NUM_CONSOLE_MESSAGE as libc::c_int as crate::stdlib::intptr_t,
        chatstate,
    ) as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotInitialChat(
    mut chatstate: libc::c_int,
    mut type_0: *mut libc::c_char,
    mut mcontext: libc::c_int,
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
        crate::g_public_h::BOTLIB_AI_INITIAL_CHAT as libc::c_int as crate::stdlib::intptr_t,
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
    mut chatstate: libc::c_int,
    mut type_0: *mut libc::c_char,
) -> libc::c_int {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_NUM_INITIAL_CHATS as libc::c_int as crate::stdlib::intptr_t,
        chatstate,
        type_0,
    ) as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotReplyChat(
    mut chatstate: libc::c_int,
    mut message: *mut libc::c_char,
    mut mcontext: libc::c_int,
    mut vcontext: libc::c_int,
    mut var0: *mut libc::c_char,
    mut var1: *mut libc::c_char,
    mut var2: *mut libc::c_char,
    mut var3: *mut libc::c_char,
    mut var4: *mut libc::c_char,
    mut var5: *mut libc::c_char,
    mut var6: *mut libc::c_char,
    mut var7: *mut libc::c_char,
) -> libc::c_int {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_REPLY_CHAT as libc::c_int as crate::stdlib::intptr_t,
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
    ) as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotChatLength(mut chatstate: libc::c_int) -> libc::c_int {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_CHAT_LENGTH as libc::c_int as crate::stdlib::intptr_t,
        chatstate,
    ) as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotEnterChat(
    mut chatstate: libc::c_int,
    mut client: libc::c_int,
    mut sendto: libc::c_int,
) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_ENTER_CHAT as libc::c_int as crate::stdlib::intptr_t,
        chatstate,
        client,
        sendto,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotGetChatMessage(
    mut chatstate: libc::c_int,
    mut buf: *mut libc::c_char,
    mut size: libc::c_int,
) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_GET_CHAT_MESSAGE as libc::c_int as crate::stdlib::intptr_t,
        chatstate,
        buf,
        size,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_StringContains(
    mut str1: *mut libc::c_char,
    mut str2: *mut libc::c_char,
    mut casesensitive: libc::c_int,
) -> libc::c_int {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_STRING_CONTAINS as libc::c_int as crate::stdlib::intptr_t,
        str1,
        str2,
        casesensitive,
    ) as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotFindMatch(
    mut str: *mut libc::c_char,
    mut match_0: *mut libc::c_void,
    mut context: libc::c_ulong,
) -> libc::c_int {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_FIND_MATCH as libc::c_int as crate::stdlib::intptr_t,
        str,
        match_0,
        context,
    ) as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotMatchVariable(
    mut match_0: *mut libc::c_void,
    mut variable: libc::c_int,
    mut buf: *mut libc::c_char,
    mut size: libc::c_int,
) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_MATCH_VARIABLE as libc::c_int as crate::stdlib::intptr_t,
        match_0,
        variable,
        buf,
        size,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_UnifyWhiteSpaces(mut string: *mut libc::c_char) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_UNIFY_WHITE_SPACES as libc::c_int as crate::stdlib::intptr_t,
        string,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotReplaceSynonyms(
    mut string: *mut libc::c_char,
    mut context: libc::c_ulong,
) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_REPLACE_SYNONYMS as libc::c_int as crate::stdlib::intptr_t,
        string,
        context,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotLoadChatFile(
    mut chatstate: libc::c_int,
    mut chatfile: *mut libc::c_char,
    mut chatname: *mut libc::c_char,
) -> libc::c_int {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_LOAD_CHAT_FILE as libc::c_int as crate::stdlib::intptr_t,
        chatstate,
        chatfile,
        chatname,
    ) as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotSetChatGender(
    mut chatstate: libc::c_int,
    mut gender: libc::c_int,
) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_SET_CHAT_GENDER as libc::c_int as crate::stdlib::intptr_t,
        chatstate,
        gender,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotSetChatName(
    mut chatstate: libc::c_int,
    mut name: *mut libc::c_char,
    mut client: libc::c_int,
) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_SET_CHAT_NAME as libc::c_int as crate::stdlib::intptr_t,
        chatstate,
        name,
        client,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotResetGoalState(mut goalstate: libc::c_int) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_RESET_GOAL_STATE as libc::c_int as crate::stdlib::intptr_t,
        goalstate,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotResetAvoidGoals(mut goalstate: libc::c_int) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_RESET_AVOID_GOALS as libc::c_int as crate::stdlib::intptr_t,
        goalstate,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotRemoveFromAvoidGoals(
    mut goalstate: libc::c_int,
    mut number: libc::c_int,
) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_REMOVE_FROM_AVOID_GOALS as libc::c_int
            as crate::stdlib::intptr_t,
        goalstate,
        number,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotPushGoal(mut goalstate: libc::c_int, mut goal: *mut libc::c_void) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_PUSH_GOAL as libc::c_int as crate::stdlib::intptr_t,
        goalstate,
        goal,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotPopGoal(mut goalstate: libc::c_int) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_POP_GOAL as libc::c_int as crate::stdlib::intptr_t,
        goalstate,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotEmptyGoalStack(mut goalstate: libc::c_int) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_EMPTY_GOAL_STACK as libc::c_int as crate::stdlib::intptr_t,
        goalstate,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotDumpAvoidGoals(mut goalstate: libc::c_int) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_DUMP_AVOID_GOALS as libc::c_int as crate::stdlib::intptr_t,
        goalstate,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotDumpGoalStack(mut goalstate: libc::c_int) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_DUMP_GOAL_STACK as libc::c_int as crate::stdlib::intptr_t,
        goalstate,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotGoalName(
    mut number: libc::c_int,
    mut name: *mut libc::c_char,
    mut size: libc::c_int,
) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_GOAL_NAME as libc::c_int as crate::stdlib::intptr_t,
        number,
        name,
        size,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotGetTopGoal(
    mut goalstate: libc::c_int,
    mut goal: *mut libc::c_void,
) -> libc::c_int {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_GET_TOP_GOAL as libc::c_int as crate::stdlib::intptr_t,
        goalstate,
        goal,
    ) as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotGetSecondGoal(
    mut goalstate: libc::c_int,
    mut goal: *mut libc::c_void,
) -> libc::c_int {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_GET_SECOND_GOAL as libc::c_int as crate::stdlib::intptr_t,
        goalstate,
        goal,
    ) as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotChooseLTGItem(
    mut goalstate: libc::c_int,
    mut origin: *mut crate::src::qcommon::q_shared::vec_t,
    mut inventory: *mut libc::c_int,
    mut travelflags: libc::c_int,
) -> libc::c_int {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_CHOOSE_LTG_ITEM as libc::c_int as crate::stdlib::intptr_t,
        goalstate,
        origin,
        inventory,
        travelflags,
    ) as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotChooseNBGItem(
    mut goalstate: libc::c_int,
    mut origin: *mut crate::src::qcommon::q_shared::vec_t,
    mut inventory: *mut libc::c_int,
    mut travelflags: libc::c_int,
    mut ltg: *mut libc::c_void,
    mut maxtime: libc::c_float,
) -> libc::c_int {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_CHOOSE_NBG_ITEM as libc::c_int as crate::stdlib::intptr_t,
        goalstate,
        origin,
        inventory,
        travelflags,
        ltg,
        PASSFLOAT(maxtime),
    ) as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotTouchingGoal(
    mut origin: *mut crate::src::qcommon::q_shared::vec_t,
    mut goal: *mut libc::c_void,
) -> libc::c_int {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_TOUCHING_GOAL as libc::c_int as crate::stdlib::intptr_t,
        origin,
        goal,
    ) as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotItemGoalInVisButNotVisible(
    mut viewer: libc::c_int,
    mut eye: *mut crate::src::qcommon::q_shared::vec_t,
    mut viewangles: *mut crate::src::qcommon::q_shared::vec_t,
    mut goal: *mut libc::c_void,
) -> libc::c_int {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_ITEM_GOAL_IN_VIS_BUT_NOT_VISIBLE as libc::c_int
            as crate::stdlib::intptr_t,
        viewer,
        eye,
        viewangles,
        goal,
    ) as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotGetLevelItemGoal(
    mut index: libc::c_int,
    mut classname: *mut libc::c_char,
    mut goal: *mut libc::c_void,
) -> libc::c_int {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_GET_LEVEL_ITEM_GOAL as libc::c_int as crate::stdlib::intptr_t,
        index,
        classname,
        goal,
    ) as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotGetNextCampSpotGoal(
    mut num: libc::c_int,
    mut goal: *mut libc::c_void,
) -> libc::c_int {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_GET_NEXT_CAMP_SPOT_GOAL as libc::c_int
            as crate::stdlib::intptr_t,
        num,
        goal,
    ) as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotGetMapLocationGoal(
    mut name: *mut libc::c_char,
    mut goal: *mut libc::c_void,
) -> libc::c_int {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_GET_MAP_LOCATION_GOAL as libc::c_int
            as crate::stdlib::intptr_t,
        name,
        goal,
    ) as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotAvoidGoalTime(
    mut goalstate: libc::c_int,
    mut number: libc::c_int,
) -> libc::c_float {
    let mut fi: crate::src::qcommon::q_shared::floatint_t =
        crate::src::qcommon::q_shared::floatint_t { f: 0. };
    fi.i = syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_AVOID_GOAL_TIME as libc::c_int as crate::stdlib::intptr_t,
        goalstate,
        number,
    ) as libc::c_int;
    return fi.f;
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotSetAvoidGoalTime(
    mut goalstate: libc::c_int,
    mut number: libc::c_int,
    mut avoidtime: libc::c_float,
) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_SET_AVOID_GOAL_TIME as libc::c_int as crate::stdlib::intptr_t,
        goalstate,
        number,
        PASSFLOAT(avoidtime),
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotInitLevelItems() {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_INIT_LEVEL_ITEMS as libc::c_int as crate::stdlib::intptr_t,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotUpdateEntityItems() {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_UPDATE_ENTITY_ITEMS as libc::c_int as crate::stdlib::intptr_t,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotLoadItemWeights(
    mut goalstate: libc::c_int,
    mut filename: *mut libc::c_char,
) -> libc::c_int {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_LOAD_ITEM_WEIGHTS as libc::c_int as crate::stdlib::intptr_t,
        goalstate,
        filename,
    ) as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotFreeItemWeights(mut goalstate: libc::c_int) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_FREE_ITEM_WEIGHTS as libc::c_int as crate::stdlib::intptr_t,
        goalstate,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotInterbreedGoalFuzzyLogic(
    mut parent1: libc::c_int,
    mut parent2: libc::c_int,
    mut child: libc::c_int,
) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_INTERBREED_GOAL_FUZZY_LOGIC as libc::c_int
            as crate::stdlib::intptr_t,
        parent1,
        parent2,
        child,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotSaveGoalFuzzyLogic(
    mut goalstate: libc::c_int,
    mut filename: *mut libc::c_char,
) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_SAVE_GOAL_FUZZY_LOGIC as libc::c_int
            as crate::stdlib::intptr_t,
        goalstate,
        filename,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotMutateGoalFuzzyLogic(
    mut goalstate: libc::c_int,
    mut range: libc::c_float,
) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_MUTATE_GOAL_FUZZY_LOGIC as libc::c_int
            as crate::stdlib::intptr_t,
        goalstate,
        PASSFLOAT(range),
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotAllocGoalState(mut state: libc::c_int) -> libc::c_int {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_ALLOC_GOAL_STATE as libc::c_int as crate::stdlib::intptr_t,
        state,
    ) as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotFreeGoalState(mut handle: libc::c_int) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_FREE_GOAL_STATE as libc::c_int as crate::stdlib::intptr_t,
        handle,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotResetMoveState(mut movestate: libc::c_int) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_RESET_MOVE_STATE as libc::c_int as crate::stdlib::intptr_t,
        movestate,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotAddAvoidSpot(
    mut movestate: libc::c_int,
    mut origin: *mut crate::src::qcommon::q_shared::vec_t,
    mut radius: libc::c_float,
    mut type_0: libc::c_int,
) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_ADD_AVOID_SPOT as libc::c_int as crate::stdlib::intptr_t,
        movestate,
        origin,
        PASSFLOAT(radius),
        type_0,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotMoveToGoal(
    mut result: *mut libc::c_void,
    mut movestate: libc::c_int,
    mut goal: *mut libc::c_void,
    mut travelflags: libc::c_int,
) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_MOVE_TO_GOAL as libc::c_int as crate::stdlib::intptr_t,
        result,
        movestate,
        goal,
        travelflags,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotMoveInDirection(
    mut movestate: libc::c_int,
    mut dir: *mut crate::src::qcommon::q_shared::vec_t,
    mut speed: libc::c_float,
    mut type_0: libc::c_int,
) -> libc::c_int {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_MOVE_IN_DIRECTION as libc::c_int as crate::stdlib::intptr_t,
        movestate,
        dir,
        PASSFLOAT(speed),
        type_0,
    ) as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotResetAvoidReach(mut movestate: libc::c_int) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_RESET_AVOID_REACH as libc::c_int as crate::stdlib::intptr_t,
        movestate,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotResetLastAvoidReach(mut movestate: libc::c_int) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_RESET_LAST_AVOID_REACH as libc::c_int
            as crate::stdlib::intptr_t,
        movestate,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotReachabilityArea(
    mut origin: *mut crate::src::qcommon::q_shared::vec_t,
    mut testground: libc::c_int,
) -> libc::c_int {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_REACHABILITY_AREA as libc::c_int as crate::stdlib::intptr_t,
        origin,
        testground,
    ) as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotMovementViewTarget(
    mut movestate: libc::c_int,
    mut goal: *mut libc::c_void,
    mut travelflags: libc::c_int,
    mut lookahead: libc::c_float,
    mut target: *mut crate::src::qcommon::q_shared::vec_t,
) -> libc::c_int {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_MOVEMENT_VIEW_TARGET as libc::c_int as crate::stdlib::intptr_t,
        movestate,
        goal,
        travelflags,
        PASSFLOAT(lookahead),
        target,
    ) as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotPredictVisiblePosition(
    mut origin: *mut crate::src::qcommon::q_shared::vec_t,
    mut areanum: libc::c_int,
    mut goal: *mut libc::c_void,
    mut travelflags: libc::c_int,
    mut target: *mut crate::src::qcommon::q_shared::vec_t,
) -> libc::c_int {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_PREDICT_VISIBLE_POSITION as libc::c_int
            as crate::stdlib::intptr_t,
        origin,
        areanum,
        goal,
        travelflags,
        target,
    ) as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotAllocMoveState() -> libc::c_int {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_ALLOC_MOVE_STATE as libc::c_int as crate::stdlib::intptr_t,
    ) as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotFreeMoveState(mut handle: libc::c_int) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_FREE_MOVE_STATE as libc::c_int as crate::stdlib::intptr_t,
        handle,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotInitMoveState(
    mut handle: libc::c_int,
    mut initmove: *mut libc::c_void,
) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_INIT_MOVE_STATE as libc::c_int as crate::stdlib::intptr_t,
        handle,
        initmove,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotChooseBestFightWeapon(
    mut weaponstate: libc::c_int,
    mut inventory: *mut libc::c_int,
) -> libc::c_int {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_CHOOSE_BEST_FIGHT_WEAPON as libc::c_int
            as crate::stdlib::intptr_t,
        weaponstate,
        inventory,
    ) as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotGetWeaponInfo(
    mut weaponstate: libc::c_int,
    mut weapon: libc::c_int,
    mut weaponinfo: *mut libc::c_void,
) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_GET_WEAPON_INFO as libc::c_int as crate::stdlib::intptr_t,
        weaponstate,
        weapon,
        weaponinfo,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotLoadWeaponWeights(
    mut weaponstate: libc::c_int,
    mut filename: *mut libc::c_char,
) -> libc::c_int {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_LOAD_WEAPON_WEIGHTS as libc::c_int as crate::stdlib::intptr_t,
        weaponstate,
        filename,
    ) as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotAllocWeaponState() -> libc::c_int {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_ALLOC_WEAPON_STATE as libc::c_int as crate::stdlib::intptr_t,
    ) as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotFreeWeaponState(mut weaponstate: libc::c_int) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_FREE_WEAPON_STATE as libc::c_int as crate::stdlib::intptr_t,
        weaponstate,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_BotResetWeaponState(mut weaponstate: libc::c_int) {
    syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_RESET_WEAPON_STATE as libc::c_int as crate::stdlib::intptr_t,
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
    mut numranks: libc::c_int,
    mut ranks: *mut libc::c_float,
    mut parent1: *mut libc::c_int,
    mut parent2: *mut libc::c_int,
    mut child: *mut libc::c_int,
) -> libc::c_int {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_AI_GENETIC_PARENTS_AND_CHILD_SELECTION as libc::c_int
            as crate::stdlib::intptr_t,
        numranks,
        ranks,
        parent1,
        parent2,
        child,
    ) as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn trap_PC_LoadSource(mut filename: *const libc::c_char) -> libc::c_int {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_PC_LOAD_SOURCE as libc::c_int as crate::stdlib::intptr_t,
        filename,
    ) as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn trap_PC_FreeSource(mut handle: libc::c_int) -> libc::c_int {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_PC_FREE_SOURCE as libc::c_int as crate::stdlib::intptr_t,
        handle,
    ) as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn trap_PC_ReadToken(
    mut handle: libc::c_int,
    mut pc_token: *mut crate::src::qcommon::q_shared::pc_token_t,
) -> libc::c_int {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_PC_READ_TOKEN as libc::c_int as crate::stdlib::intptr_t,
        handle,
        pc_token,
    ) as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn trap_PC_SourceFileAndLine(
    mut handle: libc::c_int,
    mut filename: *mut libc::c_char,
    mut line: *mut libc::c_int,
) -> libc::c_int {
    return syscall.expect("non-null function pointer")(
        crate::g_public_h::BOTLIB_PC_SOURCE_FILE_AND_LINE as libc::c_int as crate::stdlib::intptr_t,
        handle,
        filename,
        line,
    ) as libc::c_int;
}
unsafe extern "C" fn run_static_initializers() {
    syscall = ::std::mem::transmute::<
        libc::intptr_t,
        Option<unsafe extern "C" fn(_: crate::stdlib::intptr_t, _: ...) -> crate::stdlib::intptr_t>,
    >(-(1 as libc::c_int) as libc::intptr_t)
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];

use ::libc;

pub mod qcommon_h {

    #[inline]

    pub unsafe extern "C" fn _vmf(mut x: crate::stdlib::intptr_t) -> f32 {
        let mut fi: crate::src::qcommon::q_shared::floatint_t =
            crate::src::qcommon::q_shared::floatint_t { f: 0. };
        fi.i = x as i32;
        return fi.f;
    }

    // _QCOMMON_H_
    // flags for sv_allowDownload and cl_allowDownload
}

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

pub use crate::stdlib::intptr_t;

pub use crate::be_aas_h::aas_altroutegoal_s;
pub use crate::be_aas_h::aas_areainfo_s;
pub use crate::be_aas_h::aas_clientmove_s;
pub use crate::be_aas_h::aas_entityinfo_s;
pub use crate::be_aas_h::aas_predictroute_s;
pub use crate::be_aas_h::C2RustUnnamed_0;
pub use crate::botlib_h::aas_export_s;
pub use crate::botlib_h::aas_export_t;
pub use crate::botlib_h::ai_export_s;
pub use crate::botlib_h::ai_export_t;
pub use crate::botlib_h::bot_entitystate_s;
pub use crate::botlib_h::bot_entitystate_t;
pub use crate::botlib_h::bot_input_s;
pub use crate::botlib_h::bot_input_t;
pub use crate::botlib_h::botlib_export_s;
pub use crate::botlib_h::botlib_export_t;
pub use crate::botlib_h::ea_export_s;
pub use crate::botlib_h::ea_export_t;
pub use crate::g_public_h::entityShared_t;
pub use crate::g_public_h::sharedEntity_t;
pub use crate::g_public_h::BOTAI_START_FRAME;
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
pub use crate::g_public_h::GAME_CLIENT_BEGIN;
pub use crate::g_public_h::GAME_CLIENT_COMMAND;
pub use crate::g_public_h::GAME_CLIENT_CONNECT;
pub use crate::g_public_h::GAME_CLIENT_DISCONNECT;
pub use crate::g_public_h::GAME_CLIENT_THINK;
pub use crate::g_public_h::GAME_CLIENT_USERINFO_CHANGED;
pub use crate::g_public_h::GAME_CONSOLE_COMMAND;
pub use crate::g_public_h::GAME_INIT;
pub use crate::g_public_h::GAME_RUN_FRAME;
pub use crate::g_public_h::GAME_SHUTDOWN;
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
pub use crate::qcommon_h::msg_t;
pub use crate::qcommon_h::netadr_t;
pub use crate::qcommon_h::netadrtype_t;
pub use crate::qcommon_h::netchan_t;
pub use crate::qcommon_h::netsrc_t;
pub use crate::qcommon_h::vmInterpret_t;
pub use crate::qcommon_h::vm_t;
pub use crate::qcommon_h::NA_BAD;
pub use crate::qcommon_h::NA_BOT;
pub use crate::qcommon_h::NA_BROADCAST;
pub use crate::qcommon_h::NA_IP;
pub use crate::qcommon_h::NA_IP6;
pub use crate::qcommon_h::NA_LOOPBACK;
pub use crate::qcommon_h::NA_MULTICAST6;
pub use crate::qcommon_h::NA_UNSPEC;
pub use crate::qcommon_h::NS_CLIENT;
pub use crate::qcommon_h::NS_SERVER;
pub use crate::qcommon_h::TRAP_ANGLEVECTORS;
pub use crate::qcommon_h::TRAP_ATAN2;
pub use crate::qcommon_h::TRAP_CEIL;
pub use crate::qcommon_h::TRAP_COS;
pub use crate::qcommon_h::TRAP_FLOOR;
pub use crate::qcommon_h::TRAP_MATRIXMULTIPLY;
pub use crate::qcommon_h::TRAP_MEMCPY;
pub use crate::qcommon_h::TRAP_MEMSET;
pub use crate::qcommon_h::TRAP_PERPENDICULARVECTOR;
pub use crate::qcommon_h::TRAP_SIN;
pub use crate::qcommon_h::TRAP_SQRT;
pub use crate::qcommon_h::TRAP_STRNCPY;
pub use crate::qcommon_h::TRAP_TESTPRINTFLOAT;
pub use crate::qcommon_h::TRAP_TESTPRINTINT;
pub use crate::qcommon_h::VMI_BYTECODE;
pub use crate::qcommon_h::VMI_COMPILED;
pub use crate::qcommon_h::VMI_NATIVE;
pub use crate::server_h::challenge_t;
pub use crate::server_h::clientSnapshot_t;
pub use crate::server_h::clientState_t;
pub use crate::server_h::client_s;
pub use crate::server_h::client_t;
pub use crate::server_h::netchan_buffer_s;
pub use crate::server_h::netchan_buffer_t;
pub use crate::server_h::serverState_t;
pub use crate::server_h::serverStatic_t;
pub use crate::server_h::server_t;
pub use crate::server_h::svEntity_s;
pub use crate::server_h::svEntity_t;
pub use crate::server_h::voipServerPacket_s;
pub use crate::server_h::voipServerPacket_t;
pub use crate::server_h::worldSector_s;
pub use crate::server_h::CS_ACTIVE;
pub use crate::server_h::CS_CONNECTED;
pub use crate::server_h::CS_FREE;
pub use crate::server_h::CS_PRIMED;
pub use crate::server_h::CS_ZOMBIE;
pub use crate::server_h::SS_DEAD;
pub use crate::server_h::SS_GAME;
pub use crate::server_h::SS_LOADING;
pub use crate::src::asm::snapvector::qsnapvectorsse;
pub use crate::src::botlib::be_ai_chat::bot_consolemessage_s;
pub use crate::src::botlib::be_ai_chat::bot_match_s;
pub use crate::src::botlib::be_ai_goal::bot_goal_s;
pub use crate::src::botlib::be_ai_move::bot_initmove_s;
pub use crate::src::botlib::be_ai_move::bot_moveresult_s;
pub use crate::src::botlib::be_ai_weap::weaponinfo_s;

pub use crate::src::qcommon::cmd::Cbuf_ExecuteText;
pub use crate::src::qcommon::cmd::Cmd_Argc;
pub use crate::src::qcommon::cmd::Cmd_ArgvBuffer;
pub use crate::src::qcommon::common::Com_Error;
pub use crate::src::qcommon::common::Com_Milliseconds;
pub use crate::src::qcommon::common::Com_Printf;
pub use crate::src::qcommon::common::Com_RealTime;
pub use crate::src::qcommon::cvar::Cvar_Get;
pub use crate::src::qcommon::cvar::Cvar_InfoString;
pub use crate::src::qcommon::cvar::Cvar_Register;
pub use crate::src::qcommon::cvar::Cvar_SetSafe;
pub use crate::src::qcommon::cvar::Cvar_Update;
pub use crate::src::qcommon::cvar::Cvar_VariableIntegerValue;
pub use crate::src::qcommon::cvar::Cvar_VariableStringBuffer;
pub use crate::src::qcommon::cvar::Cvar_VariableValue;
pub use crate::src::qcommon::files::FS_FCloseFile;
pub use crate::src::qcommon::files::FS_FOpenFileByMode;
pub use crate::src::qcommon::files::FS_GetFileList;
pub use crate::src::qcommon::files::FS_Read;
pub use crate::src::qcommon::files::FS_Seek;
pub use crate::src::qcommon::files::FS_Write;
pub use crate::src::qcommon::q_math::vec3_origin;
pub use crate::src::qcommon::q_math::AngleVectors;
pub use crate::src::qcommon::q_math::MatrixMultiply;
pub use crate::src::qcommon::q_math::PerpendicularVector;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::clipHandle_t;
pub use crate::src::qcommon::q_shared::cplane_s;
pub use crate::src::qcommon::q_shared::cplane_t;
pub use crate::src::qcommon::q_shared::cvarHandle_t;
pub use crate::src::qcommon::q_shared::cvar_s;
pub use crate::src::qcommon::q_shared::cvar_t;
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
pub use crate::src::qcommon::q_shared::COM_Parse;
pub use crate::src::qcommon::q_shared::Q_strncpyz;
pub use crate::src::qcommon::q_shared::ERR_DISCONNECT;
pub use crate::src::qcommon::q_shared::ERR_DROP;
pub use crate::src::qcommon::q_shared::ERR_FATAL;
pub use crate::src::qcommon::q_shared::ERR_NEED_CD;
pub use crate::src::qcommon::q_shared::ERR_SERVERDISCONNECT;
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
pub use crate::src::qcommon::vm::VM_ArgPtr;
pub use crate::src::qcommon::vm::VM_Call;
pub use crate::src::qcommon::vm::VM_Create;
pub use crate::src::qcommon::vm::VM_Free;
pub use crate::src::qcommon::vm::VM_Restart;
pub use crate::src::server::sv_bot::BotImport_DebugPolygonCreate;
pub use crate::src::server::sv_bot::BotImport_DebugPolygonDelete;
pub use crate::src::server::sv_bot::SV_BotAllocateClient;
pub use crate::src::server::sv_bot::SV_BotFreeClient;
pub use crate::src::server::sv_bot::SV_BotGetConsoleMessage;
pub use crate::src::server::sv_bot::SV_BotGetSnapshotEntity;
pub use crate::src::server::sv_bot::SV_BotLibSetup;
pub use crate::src::server::sv_bot::SV_BotLibShutdown;
pub use crate::src::server::sv_client::SV_ClientThink;
pub use crate::src::server::sv_client::SV_DropClient;
pub use crate::src::server::sv_game::qcommon_h::_vmf;
pub use crate::src::server::sv_game::stdlib_h::atoi;
pub use crate::src::server::sv_init::SV_GetConfigstring;
pub use crate::src::server::sv_init::SV_GetUserinfo;
pub use crate::src::server::sv_init::SV_SetConfigstring;
pub use crate::src::server::sv_init::SV_SetUserinfo;
pub use crate::src::server::sv_main::gvm;
pub use crate::src::server::sv_main::sv;
pub use crate::src::server::sv_main::sv_maxclients;
pub use crate::src::server::sv_main::svs;
pub use crate::src::server::sv_main::SV_SendServerCommand;
pub use crate::src::server::sv_world::SV_AreaEntities;
pub use crate::src::server::sv_world::SV_ClipHandleForEntity;
pub use crate::src::server::sv_world::SV_LinkEntity;
pub use crate::src::server::sv_world::SV_PointContents;
pub use crate::src::server::sv_world::SV_Trace;
pub use crate::src::server::sv_world::SV_UnlinkEntity;
pub use crate::src::sys::sys_unix::Sys_Milliseconds;

pub use crate::vm_local_h::vm_s;
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
// sv_game.c -- interface to the game dll
#[no_mangle]

pub static mut botlib_export: *mut botlib_export_t = std::ptr::null_mut();
// these functions must be used instead of pointer arithmetic, because
// the game allocates gentities with private information after the server shared part
#[no_mangle]

pub unsafe extern "C" fn SV_NumForGentity(mut ent: *mut sharedEntity_t) -> i32 {
    let mut num: i32 = 0;
    num = ((ent as *mut byte).offset_from(sv.gentities as *mut byte) as isize
        / sv.gentitySize as isize) as i32;
    return num;
}
#[no_mangle]

pub unsafe extern "C" fn SV_GentityNum(mut num: i32) -> *mut sharedEntity_t {
    let mut ent: *mut sharedEntity_t = 0 as *mut sharedEntity_t;
    ent =
        (sv.gentities as *mut byte).offset((sv.gentitySize * num) as isize) as *mut sharedEntity_t;
    return ent;
}
#[no_mangle]

pub unsafe extern "C" fn SV_GameClientNum(mut num: i32) -> *mut playerState_t {
    let mut ps: *mut playerState_t = 0 as *mut playerState_t;
    ps = (sv.gameClients as *mut byte).offset((sv.gameClientSize * num) as isize)
        as *mut playerState_t;
    return ps;
}
#[no_mangle]

pub unsafe extern "C" fn SV_SvEntityForGentity(mut gEnt: *mut sharedEntity_t) -> *mut svEntity_t {
    if gEnt.is_null() || (*gEnt).s.number < 0 as i32 || (*gEnt).s.number >= (1 as i32) << 10 as i32
    {
        Com_Error(
            ERR_DROP as i32,
            b"SV_SvEntityForGentity: bad gEnt\x00" as *const u8 as *const libc::c_char,
        );
    }
    return &mut *sv.svEntities.as_mut_ptr().offset((*gEnt).s.number as isize) as *mut svEntity_t;
}
#[no_mangle]

pub unsafe extern "C" fn SV_GEntityForSvEntity(mut svEnt: *mut svEntity_t) -> *mut sharedEntity_t {
    let mut num: i32 = 0;
    num = svEnt.offset_from(sv.svEntities.as_mut_ptr()) as isize as i32;
    return SV_GentityNum(num);
}
/*
===============
SV_GameSendServerCommand

Sends a command string to a client
===============
*/
#[no_mangle]

pub unsafe extern "C" fn SV_GameSendServerCommand(
    mut clientNum: i32,
    mut text: *const libc::c_char,
) {
    if clientNum == -(1 as i32) {
        SV_SendServerCommand(
            0 as *mut client_t as *mut client_s,
            b"%s\x00" as *const u8 as *const libc::c_char,
            text,
        );
    } else {
        if clientNum < 0 as i32 || clientNum >= (*sv_maxclients).integer {
            return;
        }
        SV_SendServerCommand(
            svs.clients.offset(clientNum as isize) as *mut client_s,
            b"%s\x00" as *const u8 as *const libc::c_char,
            text,
        );
    };
}
/*
===============
SV_GameDropClient

Disconnects the client with a message
===============
*/
#[no_mangle]

pub unsafe extern "C" fn SV_GameDropClient(mut clientNum: i32, mut reason: *const libc::c_char) {
    if clientNum < 0 as i32 || clientNum >= (*sv_maxclients).integer {
        return;
    }
    SV_DropClient(
        svs.clients.offset(clientNum as isize) as *mut client_s,
        reason,
    );
}
/*
=================
SV_SetBrushModel

sets mins and maxs for inline bmodels
=================
*/
#[no_mangle]

pub unsafe extern "C" fn SV_SetBrushModel(
    mut ent: *mut sharedEntity_t,
    mut name: *const libc::c_char,
) {
    let mut h: clipHandle_t = 0; // we don't know exactly what is in the brushes
    let mut mins: vec3_t = [0.; 3];
    let mut maxs: vec3_t = [0.; 3];
    if name.is_null() {
        Com_Error(
            ERR_DROP as i32,
            b"SV_SetBrushModel: NULL\x00" as *const u8 as *const libc::c_char,
        );
    }
    if *name.offset(0 as i32 as isize) as i32 != '*' as i32 {
        Com_Error(
            ERR_DROP as i32,
            b"SV_SetBrushModel: %s isn\'t a brush model\x00" as *const u8 as *const libc::c_char,
            name,
        );
    }
    (*ent).s.modelindex = atoi(name.offset(1 as i32 as isize));
    h = crate::src::qcommon::cm_load::CM_InlineModel((*ent).s.modelindex);
    crate::src::qcommon::cm_load::CM_ModelBounds(h, mins.as_mut_ptr(), maxs.as_mut_ptr());
    (*ent).r.mins[0 as i32 as usize] = mins[0 as i32 as usize];
    (*ent).r.mins[1 as i32 as usize] = mins[1 as i32 as usize];
    (*ent).r.mins[2 as i32 as usize] = mins[2 as i32 as usize];
    (*ent).r.maxs[0 as i32 as usize] = maxs[0 as i32 as usize];
    (*ent).r.maxs[1 as i32 as usize] = maxs[1 as i32 as usize];
    (*ent).r.maxs[2 as i32 as usize] = maxs[2 as i32 as usize];
    (*ent).r.bmodel = qtrue;
    (*ent).r.contents = -(1 as i32);
    SV_LinkEntity(ent as *mut sharedEntity_t);
    // FIXME: remove
}
/*
=================
SV_inPVS

Also checks portalareas so that doors block sight
=================
*/
#[no_mangle]

pub unsafe extern "C" fn SV_inPVS(mut p1: *const vec_t, mut p2: *const vec_t) -> qboolean {
    let mut leafnum: i32 = 0; // a door blocks sight
    let mut cluster: i32 = 0;
    let mut area1: i32 = 0;
    let mut area2: i32 = 0;
    let mut mask: *mut byte = 0 as *mut byte;
    leafnum = crate::src::qcommon::cm_test::CM_PointLeafnum(p1);
    cluster = crate::src::qcommon::cm_load::CM_LeafCluster(leafnum);
    area1 = crate::src::qcommon::cm_load::CM_LeafArea(leafnum);
    mask = crate::src::qcommon::cm_test::CM_ClusterPVS(cluster);
    leafnum = crate::src::qcommon::cm_test::CM_PointLeafnum(p2);
    cluster = crate::src::qcommon::cm_load::CM_LeafCluster(leafnum);
    area2 = crate::src::qcommon::cm_load::CM_LeafArea(leafnum);
    if !mask.is_null()
        && *mask.offset((cluster >> 3 as i32) as isize) as i32 & (1 as i32) << (cluster & 7 as i32)
            == 0
    {
        return qfalse;
    }
    if crate::src::qcommon::cm_test::CM_AreasConnected(area1, area2) as u64 == 0 {
        return qfalse;
    }
    return qtrue;
}
/*
=================
SV_inPVSIgnorePortals

Does NOT check portalareas
=================
*/
#[no_mangle]

pub unsafe extern "C" fn SV_inPVSIgnorePortals(
    mut p1: *const vec_t,
    mut p2: *const vec_t,
) -> qboolean {
    let mut leafnum: i32 = 0;
    let mut cluster: i32 = 0;
    let mut mask: *mut byte = 0 as *mut byte;
    leafnum = crate::src::qcommon::cm_test::CM_PointLeafnum(p1);
    cluster = crate::src::qcommon::cm_load::CM_LeafCluster(leafnum);
    mask = crate::src::qcommon::cm_test::CM_ClusterPVS(cluster);
    leafnum = crate::src::qcommon::cm_test::CM_PointLeafnum(p2);
    cluster = crate::src::qcommon::cm_load::CM_LeafCluster(leafnum);
    if !mask.is_null()
        && *mask.offset((cluster >> 3 as i32) as isize) as i32 & (1 as i32) << (cluster & 7 as i32)
            == 0
    {
        return qfalse;
    }
    return qtrue;
}
/*
========================
SV_AdjustAreaPortalState
========================
*/
#[no_mangle]

pub unsafe extern "C" fn SV_AdjustAreaPortalState(
    mut ent: *mut sharedEntity_t,
    mut open: qboolean,
) {
    let mut svEnt: *mut svEntity_t = 0 as *mut svEntity_t;
    svEnt = SV_SvEntityForGentity(ent);
    if (*svEnt).areanum2 == -(1 as i32) {
        return;
    }
    crate::src::qcommon::cm_test::CM_AdjustAreaPortalState(
        (*svEnt).areanum,
        (*svEnt).areanum2,
        open,
    );
}
/*
==================
SV_EntityContact
==================
*/
#[no_mangle]

pub unsafe extern "C" fn SV_EntityContact(
    mut mins: *mut vec_t,
    mut maxs: *mut vec_t,
    mut gEnt: *const sharedEntity_t,
    mut capsule: i32,
) -> qboolean {
    let mut origin: *const f32 = 0 as *const f32;
    let mut angles: *const f32 = 0 as *const f32;
    let mut ch: clipHandle_t = 0;
    let mut trace: trace_t = trace_t {
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
    // check for exact collision
    origin = (*gEnt).r.currentOrigin.as_ptr();
    angles = (*gEnt).r.currentAngles.as_ptr();
    ch = SV_ClipHandleForEntity(gEnt as *const sharedEntity_t);
    crate::src::qcommon::cm_trace::CM_TransformedBoxTrace(
        &mut trace as *mut _ as *mut trace_t,
        vec3_origin.as_mut_ptr() as *const vec_t,
        vec3_origin.as_mut_ptr() as *const vec_t,
        mins,
        maxs,
        ch,
        -(1 as i32),
        origin,
        angles,
        capsule,
    );
    return trace.startsolid;
}
/*
===============
SV_GetServerinfo

===============
*/
#[no_mangle]

pub unsafe extern "C" fn SV_GetServerinfo(mut buffer: *mut libc::c_char, mut bufferSize: i32) {
    if bufferSize < 1 as i32 {
        Com_Error(
            ERR_DROP as i32,
            b"SV_GetServerinfo: bufferSize == %i\x00" as *const u8 as *const libc::c_char,
            bufferSize,
        );
    }
    Q_strncpyz(buffer, Cvar_InfoString(0x4 as i32), bufferSize);
}
/*
===============
SV_LocateGameData

===============
*/
#[no_mangle]

pub unsafe extern "C" fn SV_LocateGameData(
    mut gEnts: *mut sharedEntity_t,
    mut numGEntities: i32,
    mut sizeofGEntity_t: i32,
    mut clients: *mut playerState_t,
    mut sizeofGameClient: i32,
) {
    sv.gentities = gEnts;
    sv.gentitySize = sizeofGEntity_t;
    sv.num_entities = numGEntities;
    sv.gameClients = clients;
    sv.gameClientSize = sizeofGameClient;
}
/*
===============
SV_GetUsercmd

===============
*/
#[no_mangle]

pub unsafe extern "C" fn SV_GetUsercmd(mut clientNum: i32, mut cmd: *mut usercmd_t) {
    if clientNum < 0 as i32 || clientNum >= (*sv_maxclients).integer {
        Com_Error(
            ERR_DROP as i32,
            b"SV_GetUsercmd: bad clientNum:%i\x00" as *const u8 as *const libc::c_char,
            clientNum,
        );
    }
    *cmd = (*svs.clients.offset(clientNum as isize)).lastUsercmd;
}
//==============================================

unsafe extern "C" fn FloatAsInt(mut f: f32) -> i32 {
    let mut fi: floatint_t = floatint_t { f: 0. };
    fi.f = f;
    return fi.i;
}
/*
====================
SV_GameSystemCalls

The module is making a system call
====================
*/
#[no_mangle]

pub unsafe extern "C" fn SV_GameSystemCalls(mut args: *mut intptr_t) -> intptr_t {
    match *args.offset(0 as i32 as isize) {
        0 => {
            Com_Printf(
                b"%s\x00" as *const u8 as *const libc::c_char,
                VM_ArgPtr(*args.offset(1 as i32 as isize)) as *const libc::c_char,
            );
            return 0 as i32 as intptr_t;
        }
        1 => {
            Com_Error(
                ERR_DROP as i32,
                b"%s\x00" as *const u8 as *const libc::c_char,
                VM_ArgPtr(*args.offset(1 as i32 as isize)) as *const libc::c_char,
            );
        }
        2 => return Sys_Milliseconds() as intptr_t,
        3 => {
            Cvar_Register(
                VM_ArgPtr(*args.offset(1 as i32 as isize)) as *mut vmCvar_t as *mut vmCvar_t,
                VM_ArgPtr(*args.offset(2 as i32 as isize)) as *const libc::c_char,
                VM_ArgPtr(*args.offset(3 as i32 as isize)) as *const libc::c_char,
                *args.offset(4 as i32 as isize) as i32,
            );
            return 0 as i32 as intptr_t;
        }
        4 => {
            Cvar_Update(
                VM_ArgPtr(*args.offset(1 as i32 as isize)) as *mut vmCvar_t as *mut vmCvar_t
            );
            return 0 as i32 as intptr_t;
        }
        5 => {
            Cvar_SetSafe(
                VM_ArgPtr(*args.offset(1 as i32 as isize)) as *const libc::c_char,
                VM_ArgPtr(*args.offset(2 as i32 as isize)) as *const libc::c_char,
            );
            return 0 as i32 as intptr_t;
        }
        6 => {
            return Cvar_VariableIntegerValue(
                VM_ArgPtr(*args.offset(1 as i32 as isize)) as *const libc::c_char
            ) as intptr_t
        }
        7 => {
            Cvar_VariableStringBuffer(
                VM_ArgPtr(*args.offset(1 as i32 as isize)) as *const libc::c_char,
                VM_ArgPtr(*args.offset(2 as i32 as isize)) as *mut libc::c_char,
                *args.offset(3 as i32 as isize) as i32,
            );
            return 0 as i32 as intptr_t;
        }
        8 => return Cmd_Argc() as intptr_t,
        9 => {
            Cmd_ArgvBuffer(
                *args.offset(1 as i32 as isize) as i32,
                VM_ArgPtr(*args.offset(2 as i32 as isize)) as *mut libc::c_char,
                *args.offset(3 as i32 as isize) as i32,
            );
            return 0 as i32 as intptr_t;
        }
        14 => {
            Cbuf_ExecuteText(
                *args.offset(1 as i32 as isize) as i32,
                VM_ArgPtr(*args.offset(2 as i32 as isize)) as *const libc::c_char,
            );
            return 0 as i32 as intptr_t;
        }
        10 => {
            return FS_FOpenFileByMode(
                VM_ArgPtr(*args.offset(1 as i32 as isize)) as *const libc::c_char,
                VM_ArgPtr(*args.offset(2 as i32 as isize)) as *mut fileHandle_t,
                *args.offset(3 as i32 as isize) as fsMode_t,
            ) as intptr_t
        }
        11 => {
            FS_Read(
                VM_ArgPtr(*args.offset(1 as i32 as isize)),
                *args.offset(2 as i32 as isize) as i32,
                *args.offset(3 as i32 as isize) as fileHandle_t,
            );
            return 0 as i32 as intptr_t;
        }
        12 => {
            FS_Write(
                VM_ArgPtr(*args.offset(1 as i32 as isize)),
                *args.offset(2 as i32 as isize) as i32,
                *args.offset(3 as i32 as isize) as fileHandle_t,
            );
            return 0 as i32 as intptr_t;
        }
        13 => {
            FS_FCloseFile(*args.offset(1 as i32 as isize) as fileHandle_t);
            return 0 as i32 as intptr_t;
        }
        38 => {
            return FS_GetFileList(
                VM_ArgPtr(*args.offset(1 as i32 as isize)) as *const libc::c_char,
                VM_ArgPtr(*args.offset(2 as i32 as isize)) as *const libc::c_char,
                VM_ArgPtr(*args.offset(3 as i32 as isize)) as *mut libc::c_char,
                *args.offset(4 as i32 as isize) as i32,
            ) as intptr_t
        }
        45 => {
            return FS_Seek(
                *args.offset(1 as i32 as isize) as fileHandle_t,
                *args.offset(2 as i32 as isize),
                *args.offset(3 as i32 as isize) as i32,
            ) as intptr_t
        }
        15 => {
            SV_LocateGameData(
                VM_ArgPtr(*args.offset(1 as i32 as isize)) as *mut sharedEntity_t,
                *args.offset(2 as i32 as isize) as i32,
                *args.offset(3 as i32 as isize) as i32,
                VM_ArgPtr(*args.offset(4 as i32 as isize)) as *mut playerState_t,
                *args.offset(5 as i32 as isize) as i32,
            );
            return 0 as i32 as intptr_t;
        }
        16 => {
            SV_GameDropClient(
                *args.offset(1 as i32 as isize) as i32,
                VM_ArgPtr(*args.offset(2 as i32 as isize)) as *const libc::c_char,
            );
            return 0 as i32 as intptr_t;
        }
        17 => {
            SV_GameSendServerCommand(
                *args.offset(1 as i32 as isize) as i32,
                VM_ArgPtr(*args.offset(2 as i32 as isize)) as *const libc::c_char,
            );
            return 0 as i32 as intptr_t;
        }
        30 => {
            SV_LinkEntity(
                VM_ArgPtr(*args.offset(1 as i32 as isize)) as *mut sharedEntity_t
                    as *mut sharedEntity_t,
            );
            return 0 as i32 as intptr_t;
        }
        31 => {
            SV_UnlinkEntity(
                VM_ArgPtr(*args.offset(1 as i32 as isize)) as *mut sharedEntity_t
                    as *mut sharedEntity_t,
            );
            return 0 as i32 as intptr_t;
        }
        32 => {
            return SV_AreaEntities(
                VM_ArgPtr(*args.offset(1 as i32 as isize)) as *const vec_t,
                VM_ArgPtr(*args.offset(2 as i32 as isize)) as *const vec_t,
                VM_ArgPtr(*args.offset(3 as i32 as isize)) as *mut i32,
                *args.offset(4 as i32 as isize) as i32,
            ) as intptr_t
        }
        33 => {
            return SV_EntityContact(
                VM_ArgPtr(*args.offset(1 as i32 as isize)) as *mut vec_t,
                VM_ArgPtr(*args.offset(2 as i32 as isize)) as *mut vec_t,
                VM_ArgPtr(*args.offset(3 as i32 as isize)) as *const sharedEntity_t,
                qfalse as i32,
            ) as intptr_t
        }
        44 => {
            return SV_EntityContact(
                VM_ArgPtr(*args.offset(1 as i32 as isize)) as *mut vec_t,
                VM_ArgPtr(*args.offset(2 as i32 as isize)) as *mut vec_t,
                VM_ArgPtr(*args.offset(3 as i32 as isize)) as *const sharedEntity_t,
                qtrue as i32,
            ) as intptr_t
        }
        24 => {
            SV_Trace(
                VM_ArgPtr(*args.offset(1 as i32 as isize)) as *mut trace_t as *mut trace_t,
                VM_ArgPtr(*args.offset(2 as i32 as isize)) as *const vec_t,
                VM_ArgPtr(*args.offset(3 as i32 as isize)) as *mut vec_t,
                VM_ArgPtr(*args.offset(4 as i32 as isize)) as *mut vec_t,
                VM_ArgPtr(*args.offset(5 as i32 as isize)) as *const vec_t,
                *args.offset(6 as i32 as isize) as i32,
                *args.offset(7 as i32 as isize) as i32,
                qfalse as i32,
            );
            return 0 as i32 as intptr_t;
        }
        43 => {
            SV_Trace(
                VM_ArgPtr(*args.offset(1 as i32 as isize)) as *mut trace_t as *mut trace_t,
                VM_ArgPtr(*args.offset(2 as i32 as isize)) as *const vec_t,
                VM_ArgPtr(*args.offset(3 as i32 as isize)) as *mut vec_t,
                VM_ArgPtr(*args.offset(4 as i32 as isize)) as *mut vec_t,
                VM_ArgPtr(*args.offset(5 as i32 as isize)) as *const vec_t,
                *args.offset(6 as i32 as isize) as i32,
                *args.offset(7 as i32 as isize) as i32,
                qtrue as i32,
            );
            return 0 as i32 as intptr_t;
        }
        25 => {
            return SV_PointContents(
                VM_ArgPtr(*args.offset(1 as i32 as isize)) as *const vec_t,
                *args.offset(2 as i32 as isize) as i32,
            ) as intptr_t
        }
        23 => {
            SV_SetBrushModel(
                VM_ArgPtr(*args.offset(1 as i32 as isize)) as *mut sharedEntity_t,
                VM_ArgPtr(*args.offset(2 as i32 as isize)) as *const libc::c_char,
            );
            return 0 as i32 as intptr_t;
        }
        26 => {
            return SV_inPVS(
                VM_ArgPtr(*args.offset(1 as i32 as isize)) as *const vec_t,
                VM_ArgPtr(*args.offset(2 as i32 as isize)) as *const vec_t,
            ) as intptr_t
        }
        27 => {
            return SV_inPVSIgnorePortals(
                VM_ArgPtr(*args.offset(1 as i32 as isize)) as *const vec_t,
                VM_ArgPtr(*args.offset(2 as i32 as isize)) as *const vec_t,
            ) as intptr_t
        }
        18 => {
            SV_SetConfigstring(
                *args.offset(1 as i32 as isize) as i32,
                VM_ArgPtr(*args.offset(2 as i32 as isize)) as *const libc::c_char,
            );
            return 0 as i32 as intptr_t;
        }
        19 => {
            SV_GetConfigstring(
                *args.offset(1 as i32 as isize) as i32,
                VM_ArgPtr(*args.offset(2 as i32 as isize)) as *mut libc::c_char,
                *args.offset(3 as i32 as isize) as i32,
            );
            return 0 as i32 as intptr_t;
        }
        21 => {
            SV_SetUserinfo(
                *args.offset(1 as i32 as isize) as i32,
                VM_ArgPtr(*args.offset(2 as i32 as isize)) as *const libc::c_char,
            );
            return 0 as i32 as intptr_t;
        }
        20 => {
            SV_GetUserinfo(
                *args.offset(1 as i32 as isize) as i32,
                VM_ArgPtr(*args.offset(2 as i32 as isize)) as *mut libc::c_char,
                *args.offset(3 as i32 as isize) as i32,
            );
            return 0 as i32 as intptr_t;
        }
        22 => {
            SV_GetServerinfo(
                VM_ArgPtr(*args.offset(1 as i32 as isize)) as *mut libc::c_char,
                *args.offset(2 as i32 as isize) as i32,
            );
            return 0 as i32 as intptr_t;
        }
        28 => {
            SV_AdjustAreaPortalState(
                VM_ArgPtr(*args.offset(1 as i32 as isize)) as *mut sharedEntity_t,
                *args.offset(2 as i32 as isize) as qboolean,
            );
            return 0 as i32 as intptr_t;
        }
        29 => {
            return crate::src::qcommon::cm_test::CM_AreasConnected(
                *args.offset(1 as i32 as isize) as i32,
                *args.offset(2 as i32 as isize) as i32,
            ) as intptr_t
        }
        34 => return SV_BotAllocateClient() as intptr_t,
        35 => {
            SV_BotFreeClient(*args.offset(1 as i32 as isize) as i32);
            return 0 as i32 as intptr_t;
        }
        36 => {
            SV_GetUsercmd(
                *args.offset(1 as i32 as isize) as i32,
                VM_ArgPtr(*args.offset(2 as i32 as isize)) as *mut usercmd_t,
            );
            return 0 as i32 as intptr_t;
        }
        37 => {
            let mut s: *const libc::c_char = 0 as *const libc::c_char;
            s = COM_Parse(&mut sv.entityParsePoint);
            Q_strncpyz(
                VM_ArgPtr(*args.offset(1 as i32 as isize)) as *mut libc::c_char,
                s,
                *args.offset(2 as i32 as isize) as i32,
            );
            if sv.entityParsePoint.is_null() && *s.offset(0 as i32 as isize) == 0 {
                return qfalse as i32 as intptr_t;
            } else {
                return qtrue as i32 as intptr_t;
            }
        }
        39 => {
            return BotImport_DebugPolygonCreate(
                *args.offset(1 as i32 as isize) as i32,
                *args.offset(2 as i32 as isize) as i32,
                VM_ArgPtr(*args.offset(3 as i32 as isize)) as *mut vec3_t,
            ) as intptr_t
        }
        40 => {
            BotImport_DebugPolygonDelete(*args.offset(1 as i32 as isize) as i32);
            return 0 as i32 as intptr_t;
        }
        41 => {
            return Com_RealTime(
                VM_ArgPtr(*args.offset(1 as i32 as isize)) as *mut qtime_t as *mut qtime_s
            ) as intptr_t
        }
        42 => {
            qsnapvectorsse(VM_ArgPtr(*args.offset(1 as i32 as isize)) as *mut vec_t);
            return 0 as i32 as intptr_t;
        }
        200 => {
            //====================================
            return SV_BotLibSetup() as intptr_t;
        }
        201 => return SV_BotLibShutdown() as intptr_t,
        202 => {
            return (*botlib_export)
                .BotLibVarSet
                .expect("non-null function pointer")(
                VM_ArgPtr(*args.offset(1 as i32 as isize)) as *const libc::c_char,
                VM_ArgPtr(*args.offset(2 as i32 as isize)) as *const libc::c_char,
            ) as intptr_t
        }
        203 => {
            return (*botlib_export)
                .BotLibVarGet
                .expect("non-null function pointer")(
                VM_ArgPtr(*args.offset(1 as i32 as isize)) as *const libc::c_char,
                VM_ArgPtr(*args.offset(2 as i32 as isize)) as *mut libc::c_char,
                *args.offset(3 as i32 as isize) as i32,
            ) as intptr_t
        }
        204 => {
            return (*botlib_export)
                .PC_AddGlobalDefine
                .expect("non-null function pointer")(VM_ArgPtr(
                *args.offset(1 as i32 as isize),
            )
                as *mut libc::c_char) as intptr_t
        }
        578 => {
            return (*botlib_export)
                .PC_LoadSourceHandle
                .expect("non-null function pointer")(VM_ArgPtr(
                *args.offset(1 as i32 as isize),
            )
                as *const libc::c_char) as intptr_t
        }
        579 => {
            return (*botlib_export)
                .PC_FreeSourceHandle
                .expect("non-null function pointer")(
                *args.offset(1 as i32 as isize) as i32
            ) as intptr_t
        }
        580 => {
            return (*botlib_export)
                .PC_ReadTokenHandle
                .expect("non-null function pointer")(
                *args.offset(1 as i32 as isize) as i32,
                VM_ArgPtr(*args.offset(2 as i32 as isize)) as *mut pc_token_t,
            ) as intptr_t
        }
        581 => {
            return (*botlib_export)
                .PC_SourceFileAndLine
                .expect("non-null function pointer")(
                *args.offset(1 as i32 as isize) as i32,
                VM_ArgPtr(*args.offset(2 as i32 as isize)) as *mut libc::c_char,
                VM_ArgPtr(*args.offset(3 as i32 as isize)) as *mut i32,
            ) as intptr_t
        }
        205 => {
            return (*botlib_export)
                .BotLibStartFrame
                .expect("non-null function pointer")(_vmf(
                *args.offset(1 as i32 as isize),
            )) as intptr_t
        }
        206 => {
            return (*botlib_export)
                .BotLibLoadMap
                .expect("non-null function pointer")(VM_ArgPtr(
                *args.offset(1 as i32 as isize),
            )
                as *const libc::c_char) as intptr_t
        }
        207 => {
            return (*botlib_export)
                .BotLibUpdateEntity
                .expect("non-null function pointer")(
                *args.offset(1 as i32 as isize) as i32,
                VM_ArgPtr(*args.offset(2 as i32 as isize)) as *mut bot_entitystate_t,
            ) as intptr_t
        }
        208 => {
            return (*botlib_export).Test.expect("non-null function pointer")(
                *args.offset(1 as i32 as isize) as i32,
                VM_ArgPtr(*args.offset(2 as i32 as isize)) as *mut libc::c_char,
                VM_ArgPtr(*args.offset(3 as i32 as isize)) as *mut vec_t,
                VM_ArgPtr(*args.offset(4 as i32 as isize)) as *mut vec_t,
            ) as intptr_t
        }
        209 => {
            return SV_BotGetSnapshotEntity(
                *args.offset(1 as i32 as isize) as i32,
                *args.offset(2 as i32 as isize) as i32,
            ) as intptr_t
        }
        210 => {
            return SV_BotGetConsoleMessage(
                *args.offset(1 as i32 as isize) as i32,
                VM_ArgPtr(*args.offset(2 as i32 as isize)) as *mut libc::c_char,
                *args.offset(3 as i32 as isize) as i32,
            ) as intptr_t
        }
        211 => {
            let mut clientNum: i32 = *args.offset(1 as i32 as isize) as i32;
            if clientNum >= 0 as i32 && clientNum < (*sv_maxclients).integer {
                SV_ClientThink(
                    &mut *svs.clients.offset(clientNum as isize) as *mut _ as *mut client_s,
                    VM_ArgPtr(*args.offset(2 as i32 as isize)) as *mut usercmd_t as *mut usercmd_s,
                );
            }
            return 0 as i32 as intptr_t;
        }
        301 => {
            return (*botlib_export)
                .aas
                .AAS_BBoxAreas
                .expect("non-null function pointer")(
                VM_ArgPtr(*args.offset(1 as i32 as isize)) as *mut vec_t,
                VM_ArgPtr(*args.offset(2 as i32 as isize)) as *mut vec_t,
                VM_ArgPtr(*args.offset(3 as i32 as isize)) as *mut i32,
                *args.offset(4 as i32 as isize) as i32,
            ) as intptr_t
        }
        302 => {
            return (*botlib_export)
                .aas
                .AAS_AreaInfo
                .expect("non-null function pointer")(
                *args.offset(1 as i32 as isize) as i32,
                VM_ArgPtr(*args.offset(2 as i32 as isize)) as *mut aas_areainfo_s,
            ) as intptr_t
        }
        575 => {
            return (*botlib_export)
                .aas
                .AAS_AlternativeRouteGoals
                .expect("non-null function pointer")(
                VM_ArgPtr(*args.offset(1 as i32 as isize)) as *mut vec_t,
                *args.offset(2 as i32 as isize) as i32,
                VM_ArgPtr(*args.offset(3 as i32 as isize)) as *mut vec_t,
                *args.offset(4 as i32 as isize) as i32,
                *args.offset(5 as i32 as isize) as i32,
                VM_ArgPtr(*args.offset(6 as i32 as isize)) as *mut aas_altroutegoal_s,
                *args.offset(7 as i32 as isize) as i32,
                *args.offset(8 as i32 as isize) as i32,
            ) as intptr_t
        }
        303 => {
            (*botlib_export)
                .aas
                .AAS_EntityInfo
                .expect("non-null function pointer")(
                *args.offset(1 as i32 as isize) as i32,
                VM_ArgPtr(*args.offset(2 as i32 as isize)) as *mut aas_entityinfo_s,
            );
            return 0 as i32 as intptr_t;
        }
        304 => {
            return (*botlib_export)
                .aas
                .AAS_Initialized
                .expect("non-null function pointer")() as intptr_t
        }
        305 => {
            (*botlib_export)
                .aas
                .AAS_PresenceTypeBoundingBox
                .expect("non-null function pointer")(
                *args.offset(1 as i32 as isize) as i32,
                VM_ArgPtr(*args.offset(2 as i32 as isize)) as *mut vec_t,
                VM_ArgPtr(*args.offset(3 as i32 as isize)) as *mut vec_t,
            );
            return 0 as i32 as intptr_t;
        }
        306 => {
            return FloatAsInt((*botlib_export)
                .aas
                .AAS_Time
                .expect("non-null function pointer")()) as intptr_t
        }
        307 => {
            return (*botlib_export)
                .aas
                .AAS_PointAreaNum
                .expect("non-null function pointer")(VM_ArgPtr(
                *args.offset(1 as i32 as isize),
            ) as *mut vec_t) as intptr_t
        }
        577 => {
            return (*botlib_export)
                .aas
                .AAS_PointReachabilityAreaIndex
                .expect("non-null function pointer")(VM_ArgPtr(
                *args.offset(1 as i32 as isize),
            ) as *mut vec_t) as intptr_t
        }
        308 => {
            return (*botlib_export)
                .aas
                .AAS_TraceAreas
                .expect("non-null function pointer")(
                VM_ArgPtr(*args.offset(1 as i32 as isize)) as *mut vec_t,
                VM_ArgPtr(*args.offset(2 as i32 as isize)) as *mut vec_t,
                VM_ArgPtr(*args.offset(3 as i32 as isize)) as *mut i32,
                VM_ArgPtr(*args.offset(4 as i32 as isize)) as *mut vec3_t,
                *args.offset(5 as i32 as isize) as i32,
            ) as intptr_t
        }
        309 => {
            return (*botlib_export)
                .aas
                .AAS_PointContents
                .expect("non-null function pointer")(VM_ArgPtr(
                *args.offset(1 as i32 as isize),
            ) as *mut vec_t) as intptr_t
        }
        310 => {
            return (*botlib_export)
                .aas
                .AAS_NextBSPEntity
                .expect("non-null function pointer")(
                *args.offset(1 as i32 as isize) as i32
            ) as intptr_t
        }
        311 => {
            return (*botlib_export)
                .aas
                .AAS_ValueForBSPEpairKey
                .expect("non-null function pointer")(
                *args.offset(1 as i32 as isize) as i32,
                VM_ArgPtr(*args.offset(2 as i32 as isize)) as *mut libc::c_char,
                VM_ArgPtr(*args.offset(3 as i32 as isize)) as *mut libc::c_char,
                *args.offset(4 as i32 as isize) as i32,
            ) as intptr_t
        }
        312 => {
            return (*botlib_export)
                .aas
                .AAS_VectorForBSPEpairKey
                .expect("non-null function pointer")(
                *args.offset(1 as i32 as isize) as i32,
                VM_ArgPtr(*args.offset(2 as i32 as isize)) as *mut libc::c_char,
                VM_ArgPtr(*args.offset(3 as i32 as isize)) as *mut vec_t,
            ) as intptr_t
        }
        313 => {
            return (*botlib_export)
                .aas
                .AAS_FloatForBSPEpairKey
                .expect("non-null function pointer")(
                *args.offset(1 as i32 as isize) as i32,
                VM_ArgPtr(*args.offset(2 as i32 as isize)) as *mut libc::c_char,
                VM_ArgPtr(*args.offset(3 as i32 as isize)) as *mut f32,
            ) as intptr_t
        }
        314 => {
            return (*botlib_export)
                .aas
                .AAS_IntForBSPEpairKey
                .expect("non-null function pointer")(
                *args.offset(1 as i32 as isize) as i32,
                VM_ArgPtr(*args.offset(2 as i32 as isize)) as *mut libc::c_char,
                VM_ArgPtr(*args.offset(3 as i32 as isize)) as *mut i32,
            ) as intptr_t
        }
        315 => {
            return (*botlib_export)
                .aas
                .AAS_AreaReachability
                .expect("non-null function pointer")(
                *args.offset(1 as i32 as isize) as i32
            ) as intptr_t
        }
        316 => {
            return (*botlib_export)
                .aas
                .AAS_AreaTravelTimeToGoalArea
                .expect("non-null function pointer")(
                *args.offset(1 as i32 as isize) as i32,
                VM_ArgPtr(*args.offset(2 as i32 as isize)) as *mut vec_t,
                *args.offset(3 as i32 as isize) as i32,
                *args.offset(4 as i32 as isize) as i32,
            ) as intptr_t
        }
        300 => {
            return (*botlib_export)
                .aas
                .AAS_EnableRoutingArea
                .expect("non-null function pointer")(
                *args.offset(1 as i32 as isize) as i32,
                *args.offset(2 as i32 as isize) as i32,
            ) as intptr_t
        }
        576 => {
            return (*botlib_export)
                .aas
                .AAS_PredictRoute
                .expect("non-null function pointer")(
                VM_ArgPtr(*args.offset(1 as i32 as isize)) as *mut aas_predictroute_s,
                *args.offset(2 as i32 as isize) as i32,
                VM_ArgPtr(*args.offset(3 as i32 as isize)) as *mut vec_t,
                *args.offset(4 as i32 as isize) as i32,
                *args.offset(5 as i32 as isize) as i32,
                *args.offset(6 as i32 as isize) as i32,
                *args.offset(7 as i32 as isize) as i32,
                *args.offset(8 as i32 as isize) as i32,
                *args.offset(9 as i32 as isize) as i32,
                *args.offset(10 as i32 as isize) as i32,
                *args.offset(11 as i32 as isize) as i32,
            ) as intptr_t
        }
        317 => {
            return (*botlib_export)
                .aas
                .AAS_Swimming
                .expect("non-null function pointer")(VM_ArgPtr(
                *args.offset(1 as i32 as isize),
            ) as *mut vec_t) as intptr_t
        }
        318 => {
            return (*botlib_export)
                .aas
                .AAS_PredictClientMovement
                .expect("non-null function pointer")(
                VM_ArgPtr(*args.offset(1 as i32 as isize)) as *mut aas_clientmove_s,
                *args.offset(2 as i32 as isize) as i32,
                VM_ArgPtr(*args.offset(3 as i32 as isize)) as *mut vec_t,
                *args.offset(4 as i32 as isize) as i32,
                *args.offset(5 as i32 as isize) as i32,
                VM_ArgPtr(*args.offset(6 as i32 as isize)) as *mut vec_t,
                VM_ArgPtr(*args.offset(7 as i32 as isize)) as *mut vec_t,
                *args.offset(8 as i32 as isize) as i32,
                *args.offset(9 as i32 as isize) as i32,
                _vmf(*args.offset(10 as i32 as isize)),
                *args.offset(11 as i32 as isize) as i32,
                *args.offset(12 as i32 as isize) as i32,
                *args.offset(13 as i32 as isize) as i32,
            ) as intptr_t
        }
        400 => {
            (*botlib_export)
                .ea
                .EA_Say
                .expect("non-null function pointer")(
                *args.offset(1 as i32 as isize) as i32,
                VM_ArgPtr(*args.offset(2 as i32 as isize)) as *mut libc::c_char,
            );
            return 0 as i32 as intptr_t;
        }
        401 => {
            (*botlib_export)
                .ea
                .EA_SayTeam
                .expect("non-null function pointer")(
                *args.offset(1 as i32 as isize) as i32,
                VM_ArgPtr(*args.offset(2 as i32 as isize)) as *mut libc::c_char,
            );
            return 0 as i32 as intptr_t;
        }
        402 => {
            (*botlib_export)
                .ea
                .EA_Command
                .expect("non-null function pointer")(
                *args.offset(1 as i32 as isize) as i32,
                VM_ArgPtr(*args.offset(2 as i32 as isize)) as *mut libc::c_char,
            );
            return 0 as i32 as intptr_t;
        }
        403 => {
            (*botlib_export)
                .ea
                .EA_Action
                .expect("non-null function pointer")(
                *args.offset(1 as i32 as isize) as i32,
                *args.offset(2 as i32 as isize) as i32,
            );
            return 0 as i32 as intptr_t;
        }
        404 => {
            (*botlib_export)
                .ea
                .EA_Gesture
                .expect("non-null function pointer")(
                *args.offset(1 as i32 as isize) as i32
            );
            return 0 as i32 as intptr_t;
        }
        405 => {
            (*botlib_export)
                .ea
                .EA_Talk
                .expect("non-null function pointer")(
                *args.offset(1 as i32 as isize) as i32
            );
            return 0 as i32 as intptr_t;
        }
        406 => {
            (*botlib_export)
                .ea
                .EA_Attack
                .expect("non-null function pointer")(
                *args.offset(1 as i32 as isize) as i32
            );
            return 0 as i32 as intptr_t;
        }
        407 => {
            (*botlib_export)
                .ea
                .EA_Use
                .expect("non-null function pointer")(
                *args.offset(1 as i32 as isize) as i32
            );
            return 0 as i32 as intptr_t;
        }
        408 => {
            (*botlib_export)
                .ea
                .EA_Respawn
                .expect("non-null function pointer")(
                *args.offset(1 as i32 as isize) as i32
            );
            return 0 as i32 as intptr_t;
        }
        409 => {
            (*botlib_export)
                .ea
                .EA_Crouch
                .expect("non-null function pointer")(
                *args.offset(1 as i32 as isize) as i32
            );
            return 0 as i32 as intptr_t;
        }
        410 => {
            (*botlib_export)
                .ea
                .EA_MoveUp
                .expect("non-null function pointer")(
                *args.offset(1 as i32 as isize) as i32
            );
            return 0 as i32 as intptr_t;
        }
        411 => {
            (*botlib_export)
                .ea
                .EA_MoveDown
                .expect("non-null function pointer")(
                *args.offset(1 as i32 as isize) as i32
            );
            return 0 as i32 as intptr_t;
        }
        412 => {
            (*botlib_export)
                .ea
                .EA_MoveForward
                .expect("non-null function pointer")(
                *args.offset(1 as i32 as isize) as i32
            );
            return 0 as i32 as intptr_t;
        }
        413 => {
            (*botlib_export)
                .ea
                .EA_MoveBack
                .expect("non-null function pointer")(
                *args.offset(1 as i32 as isize) as i32
            );
            return 0 as i32 as intptr_t;
        }
        414 => {
            (*botlib_export)
                .ea
                .EA_MoveLeft
                .expect("non-null function pointer")(
                *args.offset(1 as i32 as isize) as i32
            );
            return 0 as i32 as intptr_t;
        }
        415 => {
            (*botlib_export)
                .ea
                .EA_MoveRight
                .expect("non-null function pointer")(
                *args.offset(1 as i32 as isize) as i32
            );
            return 0 as i32 as intptr_t;
        }
        416 => {
            (*botlib_export)
                .ea
                .EA_SelectWeapon
                .expect("non-null function pointer")(
                *args.offset(1 as i32 as isize) as i32,
                *args.offset(2 as i32 as isize) as i32,
            );
            return 0 as i32 as intptr_t;
        }
        417 => {
            (*botlib_export)
                .ea
                .EA_Jump
                .expect("non-null function pointer")(
                *args.offset(1 as i32 as isize) as i32
            );
            return 0 as i32 as intptr_t;
        }
        418 => {
            (*botlib_export)
                .ea
                .EA_DelayedJump
                .expect("non-null function pointer")(
                *args.offset(1 as i32 as isize) as i32
            );
            return 0 as i32 as intptr_t;
        }
        419 => {
            (*botlib_export)
                .ea
                .EA_Move
                .expect("non-null function pointer")(
                *args.offset(1 as i32 as isize) as i32,
                VM_ArgPtr(*args.offset(2 as i32 as isize)) as *mut vec_t,
                _vmf(*args.offset(3 as i32 as isize)),
            );
            return 0 as i32 as intptr_t;
        }
        420 => {
            (*botlib_export)
                .ea
                .EA_View
                .expect("non-null function pointer")(
                *args.offset(1 as i32 as isize) as i32,
                VM_ArgPtr(*args.offset(2 as i32 as isize)) as *mut vec_t,
            );
            return 0 as i32 as intptr_t;
        }
        421 => {
            (*botlib_export)
                .ea
                .EA_EndRegular
                .expect("non-null function pointer")(
                *args.offset(1 as i32 as isize) as i32,
                _vmf(*args.offset(2 as i32 as isize)),
            );
            return 0 as i32 as intptr_t;
        }
        422 => {
            (*botlib_export)
                .ea
                .EA_GetInput
                .expect("non-null function pointer")(
                *args.offset(1 as i32 as isize) as i32,
                _vmf(*args.offset(2 as i32 as isize)),
                VM_ArgPtr(*args.offset(3 as i32 as isize)) as *mut bot_input_t,
            );
            return 0 as i32 as intptr_t;
        }
        423 => {
            (*botlib_export)
                .ea
                .EA_ResetInput
                .expect("non-null function pointer")(
                *args.offset(1 as i32 as isize) as i32
            );
            return 0 as i32 as intptr_t;
        }
        500 => {
            return (*botlib_export)
                .ai
                .BotLoadCharacter
                .expect("non-null function pointer")(
                VM_ArgPtr(*args.offset(1 as i32 as isize)) as *mut libc::c_char,
                _vmf(*args.offset(2 as i32 as isize)),
            ) as intptr_t
        }
        501 => {
            (*botlib_export)
                .ai
                .BotFreeCharacter
                .expect("non-null function pointer")(
                *args.offset(1 as i32 as isize) as i32
            );
            return 0 as i32 as intptr_t;
        }
        502 => {
            return FloatAsInt((*botlib_export)
                .ai
                .Characteristic_Float
                .expect("non-null function pointer")(
                *args.offset(1 as i32 as isize) as i32,
                *args.offset(2 as i32 as isize) as i32,
            )) as intptr_t
        }
        503 => {
            return FloatAsInt((*botlib_export)
                .ai
                .Characteristic_BFloat
                .expect("non-null function pointer")(
                *args.offset(1 as i32 as isize) as i32,
                *args.offset(2 as i32 as isize) as i32,
                _vmf(*args.offset(3 as i32 as isize)),
                _vmf(*args.offset(4 as i32 as isize)),
            )) as intptr_t
        }
        504 => {
            return (*botlib_export)
                .ai
                .Characteristic_Integer
                .expect("non-null function pointer")(
                *args.offset(1 as i32 as isize) as i32,
                *args.offset(2 as i32 as isize) as i32,
            ) as intptr_t
        }
        505 => {
            return (*botlib_export)
                .ai
                .Characteristic_BInteger
                .expect("non-null function pointer")(
                *args.offset(1 as i32 as isize) as i32,
                *args.offset(2 as i32 as isize) as i32,
                *args.offset(3 as i32 as isize) as i32,
                *args.offset(4 as i32 as isize) as i32,
            ) as intptr_t
        }
        506 => {
            (*botlib_export)
                .ai
                .Characteristic_String
                .expect("non-null function pointer")(
                *args.offset(1 as i32 as isize) as i32,
                *args.offset(2 as i32 as isize) as i32,
                VM_ArgPtr(*args.offset(3 as i32 as isize)) as *mut libc::c_char,
                *args.offset(4 as i32 as isize) as i32,
            );
            return 0 as i32 as intptr_t;
        }
        507 => {
            return (*botlib_export)
                .ai
                .BotAllocChatState
                .expect("non-null function pointer")() as intptr_t
        }
        508 => {
            (*botlib_export)
                .ai
                .BotFreeChatState
                .expect("non-null function pointer")(
                *args.offset(1 as i32 as isize) as i32
            );
            return 0 as i32 as intptr_t;
        }
        509 => {
            (*botlib_export)
                .ai
                .BotQueueConsoleMessage
                .expect("non-null function pointer")(
                *args.offset(1 as i32 as isize) as i32,
                *args.offset(2 as i32 as isize) as i32,
                VM_ArgPtr(*args.offset(3 as i32 as isize)) as *mut libc::c_char,
            );
            return 0 as i32 as intptr_t;
        }
        510 => {
            (*botlib_export)
                .ai
                .BotRemoveConsoleMessage
                .expect("non-null function pointer")(
                *args.offset(1 as i32 as isize) as i32,
                *args.offset(2 as i32 as isize) as i32,
            );
            return 0 as i32 as intptr_t;
        }
        511 => {
            return (*botlib_export)
                .ai
                .BotNextConsoleMessage
                .expect("non-null function pointer")(
                *args.offset(1 as i32 as isize) as i32,
                VM_ArgPtr(*args.offset(2 as i32 as isize)) as *mut bot_consolemessage_s,
            ) as intptr_t
        }
        512 => {
            return (*botlib_export)
                .ai
                .BotNumConsoleMessages
                .expect("non-null function pointer")(
                *args.offset(1 as i32 as isize) as i32
            ) as intptr_t
        }
        513 => {
            (*botlib_export)
                .ai
                .BotInitialChat
                .expect("non-null function pointer")(
                *args.offset(1 as i32 as isize) as i32,
                VM_ArgPtr(*args.offset(2 as i32 as isize)) as *mut libc::c_char,
                *args.offset(3 as i32 as isize) as i32,
                VM_ArgPtr(*args.offset(4 as i32 as isize)) as *mut libc::c_char,
                VM_ArgPtr(*args.offset(5 as i32 as isize)) as *mut libc::c_char,
                VM_ArgPtr(*args.offset(6 as i32 as isize)) as *mut libc::c_char,
                VM_ArgPtr(*args.offset(7 as i32 as isize)) as *mut libc::c_char,
                VM_ArgPtr(*args.offset(8 as i32 as isize)) as *mut libc::c_char,
                VM_ArgPtr(*args.offset(9 as i32 as isize)) as *mut libc::c_char,
                VM_ArgPtr(*args.offset(10 as i32 as isize)) as *mut libc::c_char,
                VM_ArgPtr(*args.offset(11 as i32 as isize)) as *mut libc::c_char,
            );
            return 0 as i32 as intptr_t;
        }
        569 => {
            return (*botlib_export)
                .ai
                .BotNumInitialChats
                .expect("non-null function pointer")(
                *args.offset(1 as i32 as isize) as i32,
                VM_ArgPtr(*args.offset(2 as i32 as isize)) as *mut libc::c_char,
            ) as intptr_t
        }
        514 => {
            return (*botlib_export)
                .ai
                .BotReplyChat
                .expect("non-null function pointer")(
                *args.offset(1 as i32 as isize) as i32,
                VM_ArgPtr(*args.offset(2 as i32 as isize)) as *mut libc::c_char,
                *args.offset(3 as i32 as isize) as i32,
                *args.offset(4 as i32 as isize) as i32,
                VM_ArgPtr(*args.offset(5 as i32 as isize)) as *mut libc::c_char,
                VM_ArgPtr(*args.offset(6 as i32 as isize)) as *mut libc::c_char,
                VM_ArgPtr(*args.offset(7 as i32 as isize)) as *mut libc::c_char,
                VM_ArgPtr(*args.offset(8 as i32 as isize)) as *mut libc::c_char,
                VM_ArgPtr(*args.offset(9 as i32 as isize)) as *mut libc::c_char,
                VM_ArgPtr(*args.offset(10 as i32 as isize)) as *mut libc::c_char,
                VM_ArgPtr(*args.offset(11 as i32 as isize)) as *mut libc::c_char,
                VM_ArgPtr(*args.offset(12 as i32 as isize)) as *mut libc::c_char,
            ) as intptr_t
        }
        515 => {
            return (*botlib_export)
                .ai
                .BotChatLength
                .expect("non-null function pointer")(
                *args.offset(1 as i32 as isize) as i32
            ) as intptr_t
        }
        516 => {
            (*botlib_export)
                .ai
                .BotEnterChat
                .expect("non-null function pointer")(
                *args.offset(1 as i32 as isize) as i32,
                *args.offset(2 as i32 as isize) as i32,
                *args.offset(3 as i32 as isize) as i32,
            );
            return 0 as i32 as intptr_t;
        }
        570 => {
            (*botlib_export)
                .ai
                .BotGetChatMessage
                .expect("non-null function pointer")(
                *args.offset(1 as i32 as isize) as i32,
                VM_ArgPtr(*args.offset(2 as i32 as isize)) as *mut libc::c_char,
                *args.offset(3 as i32 as isize) as i32,
            );
            return 0 as i32 as intptr_t;
        }
        517 => {
            return (*botlib_export)
                .ai
                .StringContains
                .expect("non-null function pointer")(
                VM_ArgPtr(*args.offset(1 as i32 as isize)) as *mut libc::c_char,
                VM_ArgPtr(*args.offset(2 as i32 as isize)) as *mut libc::c_char,
                *args.offset(3 as i32 as isize) as i32,
            ) as intptr_t
        }
        518 => {
            return (*botlib_export)
                .ai
                .BotFindMatch
                .expect("non-null function pointer")(
                VM_ArgPtr(*args.offset(1 as i32 as isize)) as *mut libc::c_char,
                VM_ArgPtr(*args.offset(2 as i32 as isize)) as *mut bot_match_s,
                *args.offset(3 as i32 as isize) as usize,
            ) as intptr_t
        }
        519 => {
            (*botlib_export)
                .ai
                .BotMatchVariable
                .expect("non-null function pointer")(
                VM_ArgPtr(*args.offset(1 as i32 as isize)) as *mut bot_match_s,
                *args.offset(2 as i32 as isize) as i32,
                VM_ArgPtr(*args.offset(3 as i32 as isize)) as *mut libc::c_char,
                *args.offset(4 as i32 as isize) as i32,
            );
            return 0 as i32 as intptr_t;
        }
        520 => {
            (*botlib_export)
                .ai
                .UnifyWhiteSpaces
                .expect("non-null function pointer")(VM_ArgPtr(
                *args.offset(1 as i32 as isize),
            )
                as *mut libc::c_char);
            return 0 as i32 as intptr_t;
        }
        521 => {
            (*botlib_export)
                .ai
                .BotReplaceSynonyms
                .expect("non-null function pointer")(
                VM_ArgPtr(*args.offset(1 as i32 as isize)) as *mut libc::c_char,
                *args.offset(2 as i32 as isize) as usize,
            );
            return 0 as i32 as intptr_t;
        }
        522 => {
            return (*botlib_export)
                .ai
                .BotLoadChatFile
                .expect("non-null function pointer")(
                *args.offset(1 as i32 as isize) as i32,
                VM_ArgPtr(*args.offset(2 as i32 as isize)) as *mut libc::c_char,
                VM_ArgPtr(*args.offset(3 as i32 as isize)) as *mut libc::c_char,
            ) as intptr_t
        }
        523 => {
            (*botlib_export)
                .ai
                .BotSetChatGender
                .expect("non-null function pointer")(
                *args.offset(1 as i32 as isize) as i32,
                *args.offset(2 as i32 as isize) as i32,
            );
            return 0 as i32 as intptr_t;
        }
        524 => {
            (*botlib_export)
                .ai
                .BotSetChatName
                .expect("non-null function pointer")(
                *args.offset(1 as i32 as isize) as i32,
                VM_ArgPtr(*args.offset(2 as i32 as isize)) as *mut libc::c_char,
                *args.offset(3 as i32 as isize) as i32,
            );
            return 0 as i32 as intptr_t;
        }
        525 => {
            (*botlib_export)
                .ai
                .BotResetGoalState
                .expect("non-null function pointer")(
                *args.offset(1 as i32 as isize) as i32
            );
            return 0 as i32 as intptr_t;
        }
        526 => {
            (*botlib_export)
                .ai
                .BotResetAvoidGoals
                .expect("non-null function pointer")(
                *args.offset(1 as i32 as isize) as i32
            );
            return 0 as i32 as intptr_t;
        }
        571 => {
            (*botlib_export)
                .ai
                .BotRemoveFromAvoidGoals
                .expect("non-null function pointer")(
                *args.offset(1 as i32 as isize) as i32,
                *args.offset(2 as i32 as isize) as i32,
            );
            return 0 as i32 as intptr_t;
        }
        527 => {
            (*botlib_export)
                .ai
                .BotPushGoal
                .expect("non-null function pointer")(
                *args.offset(1 as i32 as isize) as i32,
                VM_ArgPtr(*args.offset(2 as i32 as isize)) as *mut bot_goal_s,
            );
            return 0 as i32 as intptr_t;
        }
        528 => {
            (*botlib_export)
                .ai
                .BotPopGoal
                .expect("non-null function pointer")(
                *args.offset(1 as i32 as isize) as i32
            );
            return 0 as i32 as intptr_t;
        }
        529 => {
            (*botlib_export)
                .ai
                .BotEmptyGoalStack
                .expect("non-null function pointer")(
                *args.offset(1 as i32 as isize) as i32
            );
            return 0 as i32 as intptr_t;
        }
        530 => {
            (*botlib_export)
                .ai
                .BotDumpAvoidGoals
                .expect("non-null function pointer")(
                *args.offset(1 as i32 as isize) as i32
            );
            return 0 as i32 as intptr_t;
        }
        531 => {
            (*botlib_export)
                .ai
                .BotDumpGoalStack
                .expect("non-null function pointer")(
                *args.offset(1 as i32 as isize) as i32
            );
            return 0 as i32 as intptr_t;
        }
        532 => {
            (*botlib_export)
                .ai
                .BotGoalName
                .expect("non-null function pointer")(
                *args.offset(1 as i32 as isize) as i32,
                VM_ArgPtr(*args.offset(2 as i32 as isize)) as *mut libc::c_char,
                *args.offset(3 as i32 as isize) as i32,
            );
            return 0 as i32 as intptr_t;
        }
        533 => {
            return (*botlib_export)
                .ai
                .BotGetTopGoal
                .expect("non-null function pointer")(
                *args.offset(1 as i32 as isize) as i32,
                VM_ArgPtr(*args.offset(2 as i32 as isize)) as *mut bot_goal_s,
            ) as intptr_t
        }
        534 => {
            return (*botlib_export)
                .ai
                .BotGetSecondGoal
                .expect("non-null function pointer")(
                *args.offset(1 as i32 as isize) as i32,
                VM_ArgPtr(*args.offset(2 as i32 as isize)) as *mut bot_goal_s,
            ) as intptr_t
        }
        535 => {
            return (*botlib_export)
                .ai
                .BotChooseLTGItem
                .expect("non-null function pointer")(
                *args.offset(1 as i32 as isize) as i32,
                VM_ArgPtr(*args.offset(2 as i32 as isize)) as *mut vec_t,
                VM_ArgPtr(*args.offset(3 as i32 as isize)) as *mut i32,
                *args.offset(4 as i32 as isize) as i32,
            ) as intptr_t
        }
        536 => {
            return (*botlib_export)
                .ai
                .BotChooseNBGItem
                .expect("non-null function pointer")(
                *args.offset(1 as i32 as isize) as i32,
                VM_ArgPtr(*args.offset(2 as i32 as isize)) as *mut vec_t,
                VM_ArgPtr(*args.offset(3 as i32 as isize)) as *mut i32,
                *args.offset(4 as i32 as isize) as i32,
                VM_ArgPtr(*args.offset(5 as i32 as isize)) as *mut bot_goal_s,
                _vmf(*args.offset(6 as i32 as isize)),
            ) as intptr_t
        }
        537 => {
            return (*botlib_export)
                .ai
                .BotTouchingGoal
                .expect("non-null function pointer")(
                VM_ArgPtr(*args.offset(1 as i32 as isize)) as *mut vec_t,
                VM_ArgPtr(*args.offset(2 as i32 as isize)) as *mut bot_goal_s,
            ) as intptr_t
        }
        538 => {
            return (*botlib_export)
                .ai
                .BotItemGoalInVisButNotVisible
                .expect("non-null function pointer")(
                *args.offset(1 as i32 as isize) as i32,
                VM_ArgPtr(*args.offset(2 as i32 as isize)) as *mut vec_t,
                VM_ArgPtr(*args.offset(3 as i32 as isize)) as *mut vec_t,
                VM_ArgPtr(*args.offset(4 as i32 as isize)) as *mut bot_goal_s,
            ) as intptr_t
        }
        539 => {
            return (*botlib_export)
                .ai
                .BotGetLevelItemGoal
                .expect("non-null function pointer")(
                *args.offset(1 as i32 as isize) as i32,
                VM_ArgPtr(*args.offset(2 as i32 as isize)) as *mut libc::c_char,
                VM_ArgPtr(*args.offset(3 as i32 as isize)) as *mut bot_goal_s,
            ) as intptr_t
        }
        567 => {
            return (*botlib_export)
                .ai
                .BotGetNextCampSpotGoal
                .expect("non-null function pointer")(
                *args.offset(1 as i32 as isize) as i32,
                VM_ArgPtr(*args.offset(2 as i32 as isize)) as *mut bot_goal_s,
            ) as intptr_t
        }
        568 => {
            return (*botlib_export)
                .ai
                .BotGetMapLocationGoal
                .expect("non-null function pointer")(
                VM_ArgPtr(*args.offset(1 as i32 as isize)) as *mut libc::c_char,
                VM_ArgPtr(*args.offset(2 as i32 as isize)) as *mut bot_goal_s,
            ) as intptr_t
        }
        540 => {
            return FloatAsInt((*botlib_export)
                .ai
                .BotAvoidGoalTime
                .expect("non-null function pointer")(
                *args.offset(1 as i32 as isize) as i32,
                *args.offset(2 as i32 as isize) as i32,
            )) as intptr_t
        }
        573 => {
            (*botlib_export)
                .ai
                .BotSetAvoidGoalTime
                .expect("non-null function pointer")(
                *args.offset(1 as i32 as isize) as i32,
                *args.offset(2 as i32 as isize) as i32,
                _vmf(*args.offset(3 as i32 as isize)),
            );
            return 0 as i32 as intptr_t;
        }
        541 => {
            (*botlib_export)
                .ai
                .BotInitLevelItems
                .expect("non-null function pointer")();
            return 0 as i32 as intptr_t;
        }
        542 => {
            (*botlib_export)
                .ai
                .BotUpdateEntityItems
                .expect("non-null function pointer")();
            return 0 as i32 as intptr_t;
        }
        543 => {
            return (*botlib_export)
                .ai
                .BotLoadItemWeights
                .expect("non-null function pointer")(
                *args.offset(1 as i32 as isize) as i32,
                VM_ArgPtr(*args.offset(2 as i32 as isize)) as *mut libc::c_char,
            ) as intptr_t
        }
        544 => {
            (*botlib_export)
                .ai
                .BotFreeItemWeights
                .expect("non-null function pointer")(
                *args.offset(1 as i32 as isize) as i32
            );
            return 0 as i32 as intptr_t;
        }
        565 => {
            (*botlib_export)
                .ai
                .BotInterbreedGoalFuzzyLogic
                .expect("non-null function pointer")(
                *args.offset(1 as i32 as isize) as i32,
                *args.offset(2 as i32 as isize) as i32,
                *args.offset(3 as i32 as isize) as i32,
            );
            return 0 as i32 as intptr_t;
        }
        545 => {
            (*botlib_export)
                .ai
                .BotSaveGoalFuzzyLogic
                .expect("non-null function pointer")(
                *args.offset(1 as i32 as isize) as i32,
                VM_ArgPtr(*args.offset(2 as i32 as isize)) as *mut libc::c_char,
            );
            return 0 as i32 as intptr_t;
        }
        566 => {
            (*botlib_export)
                .ai
                .BotMutateGoalFuzzyLogic
                .expect("non-null function pointer")(
                *args.offset(1 as i32 as isize) as i32,
                _vmf(*args.offset(2 as i32 as isize)),
            );
            return 0 as i32 as intptr_t;
        }
        546 => {
            return (*botlib_export)
                .ai
                .BotAllocGoalState
                .expect("non-null function pointer")(
                *args.offset(1 as i32 as isize) as i32
            ) as intptr_t
        }
        547 => {
            (*botlib_export)
                .ai
                .BotFreeGoalState
                .expect("non-null function pointer")(
                *args.offset(1 as i32 as isize) as i32
            );
            return 0 as i32 as intptr_t;
        }
        548 => {
            (*botlib_export)
                .ai
                .BotResetMoveState
                .expect("non-null function pointer")(
                *args.offset(1 as i32 as isize) as i32
            );
            return 0 as i32 as intptr_t;
        }
        574 => {
            (*botlib_export)
                .ai
                .BotAddAvoidSpot
                .expect("non-null function pointer")(
                *args.offset(1 as i32 as isize) as i32,
                VM_ArgPtr(*args.offset(2 as i32 as isize)) as *mut vec_t,
                _vmf(*args.offset(3 as i32 as isize)),
                *args.offset(4 as i32 as isize) as i32,
            );
            return 0 as i32 as intptr_t;
        }
        549 => {
            (*botlib_export)
                .ai
                .BotMoveToGoal
                .expect("non-null function pointer")(
                VM_ArgPtr(*args.offset(1 as i32 as isize)) as *mut bot_moveresult_s,
                *args.offset(2 as i32 as isize) as i32,
                VM_ArgPtr(*args.offset(3 as i32 as isize)) as *mut bot_goal_s,
                *args.offset(4 as i32 as isize) as i32,
            );
            return 0 as i32 as intptr_t;
        }
        550 => {
            return (*botlib_export)
                .ai
                .BotMoveInDirection
                .expect("non-null function pointer")(
                *args.offset(1 as i32 as isize) as i32,
                VM_ArgPtr(*args.offset(2 as i32 as isize)) as *mut vec_t,
                _vmf(*args.offset(3 as i32 as isize)),
                *args.offset(4 as i32 as isize) as i32,
            ) as intptr_t
        }
        551 => {
            (*botlib_export)
                .ai
                .BotResetAvoidReach
                .expect("non-null function pointer")(
                *args.offset(1 as i32 as isize) as i32
            );
            return 0 as i32 as intptr_t;
        }
        552 => {
            (*botlib_export)
                .ai
                .BotResetLastAvoidReach
                .expect("non-null function pointer")(
                *args.offset(1 as i32 as isize) as i32
            );
            return 0 as i32 as intptr_t;
        }
        553 => {
            return (*botlib_export)
                .ai
                .BotReachabilityArea
                .expect("non-null function pointer")(
                VM_ArgPtr(*args.offset(1 as i32 as isize)) as *mut vec_t,
                *args.offset(2 as i32 as isize) as i32,
            ) as intptr_t
        }
        554 => {
            return (*botlib_export)
                .ai
                .BotMovementViewTarget
                .expect("non-null function pointer")(
                *args.offset(1 as i32 as isize) as i32,
                VM_ArgPtr(*args.offset(2 as i32 as isize)) as *mut bot_goal_s,
                *args.offset(3 as i32 as isize) as i32,
                _vmf(*args.offset(4 as i32 as isize)),
                VM_ArgPtr(*args.offset(5 as i32 as isize)) as *mut vec_t,
            ) as intptr_t
        }
        572 => {
            return (*botlib_export)
                .ai
                .BotPredictVisiblePosition
                .expect("non-null function pointer")(
                VM_ArgPtr(*args.offset(1 as i32 as isize)) as *mut vec_t,
                *args.offset(2 as i32 as isize) as i32,
                VM_ArgPtr(*args.offset(3 as i32 as isize)) as *mut bot_goal_s,
                *args.offset(4 as i32 as isize) as i32,
                VM_ArgPtr(*args.offset(5 as i32 as isize)) as *mut vec_t,
            ) as intptr_t
        }
        555 => {
            return (*botlib_export)
                .ai
                .BotAllocMoveState
                .expect("non-null function pointer")() as intptr_t
        }
        556 => {
            (*botlib_export)
                .ai
                .BotFreeMoveState
                .expect("non-null function pointer")(
                *args.offset(1 as i32 as isize) as i32
            );
            return 0 as i32 as intptr_t;
        }
        557 => {
            (*botlib_export)
                .ai
                .BotInitMoveState
                .expect("non-null function pointer")(
                *args.offset(1 as i32 as isize) as i32,
                VM_ArgPtr(*args.offset(2 as i32 as isize)) as *mut bot_initmove_s,
            );
            return 0 as i32 as intptr_t;
        }
        558 => {
            return (*botlib_export)
                .ai
                .BotChooseBestFightWeapon
                .expect("non-null function pointer")(
                *args.offset(1 as i32 as isize) as i32,
                VM_ArgPtr(*args.offset(2 as i32 as isize)) as *mut i32,
            ) as intptr_t
        }
        559 => {
            (*botlib_export)
                .ai
                .BotGetWeaponInfo
                .expect("non-null function pointer")(
                *args.offset(1 as i32 as isize) as i32,
                *args.offset(2 as i32 as isize) as i32,
                VM_ArgPtr(*args.offset(3 as i32 as isize)) as *mut weaponinfo_s,
            );
            return 0 as i32 as intptr_t;
        }
        560 => {
            return (*botlib_export)
                .ai
                .BotLoadWeaponWeights
                .expect("non-null function pointer")(
                *args.offset(1 as i32 as isize) as i32,
                VM_ArgPtr(*args.offset(2 as i32 as isize)) as *mut libc::c_char,
            ) as intptr_t
        }
        561 => {
            return (*botlib_export)
                .ai
                .BotAllocWeaponState
                .expect("non-null function pointer")() as intptr_t
        }
        562 => {
            (*botlib_export)
                .ai
                .BotFreeWeaponState
                .expect("non-null function pointer")(
                *args.offset(1 as i32 as isize) as i32
            );
            return 0 as i32 as intptr_t;
        }
        563 => {
            (*botlib_export)
                .ai
                .BotResetWeaponState
                .expect("non-null function pointer")(
                *args.offset(1 as i32 as isize) as i32
            );
            return 0 as i32 as intptr_t;
        }
        564 => {
            return (*botlib_export)
                .ai
                .GeneticParentsAndChildSelection
                .expect("non-null function pointer")(
                *args.offset(1 as i32 as isize) as i32,
                VM_ArgPtr(*args.offset(2 as i32 as isize)) as *mut f32,
                VM_ArgPtr(*args.offset(3 as i32 as isize)) as *mut i32,
                VM_ArgPtr(*args.offset(4 as i32 as isize)) as *mut i32,
                VM_ArgPtr(*args.offset(5 as i32 as isize)) as *mut i32,
            ) as intptr_t
        }
        100 => {
            crate::stdlib::memset(
                VM_ArgPtr(*args.offset(1 as i32 as isize)),
                *args.offset(2 as i32 as isize) as i32,
                *args.offset(3 as i32 as isize) as usize,
            );
            return 0 as i32 as intptr_t;
        }
        101 => {
            crate::stdlib::memcpy(
                VM_ArgPtr(*args.offset(1 as i32 as isize)),
                VM_ArgPtr(*args.offset(2 as i32 as isize)),
                *args.offset(3 as i32 as isize) as usize,
            );
            return 0 as i32 as intptr_t;
        }
        102 => {
            crate::stdlib::strncpy(
                VM_ArgPtr(*args.offset(1 as i32 as isize)) as *mut libc::c_char,
                VM_ArgPtr(*args.offset(2 as i32 as isize)) as *const libc::c_char,
                *args.offset(3 as i32 as isize) as usize,
            );
            return *args.offset(1 as i32 as isize);
        }
        103 => {
            return FloatAsInt(
                crate::stdlib::sin(_vmf(*args.offset(1 as i32 as isize)) as f64) as f32,
            ) as intptr_t
        }
        104 => {
            return FloatAsInt(
                crate::stdlib::cos(_vmf(*args.offset(1 as i32 as isize)) as f64) as f32,
            ) as intptr_t
        }
        105 => {
            return FloatAsInt(crate::stdlib::atan2(
                _vmf(*args.offset(1 as i32 as isize)) as f64,
                _vmf(*args.offset(2 as i32 as isize)) as f64,
            ) as f32) as intptr_t
        }
        106 => {
            return FloatAsInt(
                crate::stdlib::sqrt(_vmf(*args.offset(1 as i32 as isize)) as f64) as f32,
            ) as intptr_t
        }
        107 => {
            MatrixMultiply(
                VM_ArgPtr(*args.offset(1 as i32 as isize)) as *mut [f32; 3],
                VM_ArgPtr(*args.offset(2 as i32 as isize)) as *mut [f32; 3],
                VM_ArgPtr(*args.offset(3 as i32 as isize)) as *mut [f32; 3],
            );
            return 0 as i32 as intptr_t;
        }
        108 => {
            AngleVectors(
                VM_ArgPtr(*args.offset(1 as i32 as isize)) as *const vec_t,
                VM_ArgPtr(*args.offset(2 as i32 as isize)) as *mut vec_t,
                VM_ArgPtr(*args.offset(3 as i32 as isize)) as *mut vec_t,
                VM_ArgPtr(*args.offset(4 as i32 as isize)) as *mut vec_t,
            );
            return 0 as i32 as intptr_t;
        }
        109 => {
            PerpendicularVector(
                VM_ArgPtr(*args.offset(1 as i32 as isize)) as *mut vec_t,
                VM_ArgPtr(*args.offset(2 as i32 as isize)) as *const vec_t,
            );
            return 0 as i32 as intptr_t;
        }
        110 => {
            return FloatAsInt(
                crate::stdlib::floor(_vmf(*args.offset(1 as i32 as isize)) as f64) as f32,
            ) as intptr_t
        }
        111 => {
            return FloatAsInt(
                crate::stdlib::ceil(_vmf(*args.offset(1 as i32 as isize)) as f64) as f32,
            ) as intptr_t
        }
        _ => {
            Com_Error(
                ERR_DROP as i32,
                b"Bad game system trap: %ld\x00" as *const u8 as *const libc::c_char,
                *args.offset(0 as i32 as isize),
            );
        }
    };
}
/*
===============
SV_ShutdownGameProgs

Called every time a map changes
===============
*/
#[no_mangle]

pub unsafe extern "C" fn SV_ShutdownGameProgs() {
    if gvm.is_null() {
        return;
    }
    VM_Call(gvm, GAME_SHUTDOWN as i32, qfalse as i32);
    VM_Free(gvm);
    gvm = 0 as *mut vm_t;
}
/*
==================
SV_InitGameVM

Called for both a full init and a restart
==================
*/

unsafe extern "C" fn SV_InitGameVM(mut restart: qboolean) {
    let mut i: i32 = 0;
    // start the entity parsing at the beginning
    sv.entityParsePoint = crate::src::qcommon::cm_load::CM_EntityString();
    // clear all gentity pointers that might still be set from
    // a previous level
    // https://zerowing.idsoftware.com/bugzilla/show_bug.cgi?id=522
    //   now done before GAME_INIT call
    i = 0 as i32;
    while i < (*sv_maxclients).integer {
        let ref mut fresh0 = (*svs.clients.offset(i as isize)).gentity;
        *fresh0 = 0 as *mut sharedEntity_t;
        i += 1
    }
    // use the current msec count for a random seed
    // init for this gamestate
    VM_Call(
        gvm,
        GAME_INIT as i32,
        sv.time,
        Com_Milliseconds(),
        restart as u32,
    );
}
/*
===================
SV_RestartGameProgs

Called on a map_restart, but not on a normal map change
===================
*/
#[no_mangle]

pub unsafe extern "C" fn SV_RestartGameProgs() {
    if gvm.is_null() {
        return;
    }
    VM_Call(gvm, GAME_SHUTDOWN as i32, qtrue as i32);
    // do a restart instead of a free
    gvm = VM_Restart(gvm, qtrue);
    if gvm.is_null() {
        Com_Error(
            ERR_FATAL as i32,
            b"VM_Restart on game failed\x00" as *const u8 as *const libc::c_char,
        );
    }
    SV_InitGameVM(qtrue);
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
// server.h
//=============================================================================
// !!! MUST NOT CHANGE, SERVER AND
// GAME BOTH REFERENCE !!!
// for delta compression of initial sighting
// if -1, use headnode instead
// if all the clusters don't fit in clusternums
// used to prevent double adding from portal views
// no map loaded
// spawning level entities
// actively running
// if true, send configstring changes during SS_LOADING
// changes each server start
// serverId before a map_restart
// the feed key that we use to compute the pure checksum strings
// https://zerowing.idsoftware.com/bugzilla/show_bug.cgi?id=475
// the serverId associated with the current checksumFeed (always <= serverId)
// incremented for each snapshot built
// <= 1000 / sv_frame->value
// when time > nextFrameTime, process world
// used during game VM init
// the game virtual machine will update these on init and changes
// current number, <= MAX_GENTITIES
// will be > sizeof(playerState_t) due to game private data
// portalarea visibility bits
// into the circular sv_packet_entities[]
// the entities MUST be in increasing state number
// order, otherwise the delta compression will fail
// time the message was transmitted
// time the message was acked
// used to rate drop packets
// can be reused for a new connection
// client has been disconnected, but don't reuse
// connection for a couple seconds
// has been assigned to a client_t, but no gamestate yet
// gamestate has been sent, but client hasn't sent a usercmd
// client is fully in game
// valid command string for SV_Netchan_Encode
// name, etc
// last added reliable message, not necessarily sent or acknowledged yet
// last acknowledged reliable message
// last sent reliable message, not necessarily acknowledged yet
// netchan->outgoingSequence of gamestate
// for delta compression
// reliable client message sequence
// SV_GentityNum(clientnum)
// extracted from userinfo, high bits masked
// downloading
// if not empty string, we are downloading
// file being downloaded
// total bytes (can't use EOF because of paks)
// bytes sent
// last block we sent to the client, awaiting ack
// current block number
// last block we xmited
// the buffers for the download blocks
// We have sent the EOF block
// time we last got an ack from the client
// frame last client usercmd message
// svs.time when another reliable command will be allowed
// svs.time when packet was last received
// svs.time when connection started
// svs.time of last sent snapshot
// true if nextSnapshotTime was set based on rate instead of snapshotMsec
// must timeout a few frames in a row so debugging doesn't break
// updates can be delta'd from here
// bytes / second
// requests a snapshot every snapshotMsec unless rate choked
// TTimo - additional flag to distinguish between a bad pure checksum, and no cp command at all
// TTimo
// queuing outgoing fragmented messages to send them properly, without udp packet bursts
// in case large fragmented messages are stacking up
// buffer them into this queue, and hand them out to netchan as needed
//=============================================================================
// MAX_CHALLENGES is made large to prevent a denial
// of service attack that could cycle all of them
// out before legitimate users connected
// Allow a certain amount of challenges to have the same IP address
// to make it a bit harder to DOS one single IP address from connecting
// while not allowing a single ip to grab all challenge resources
// challenge number coming from the client
// time the last packet was sent to the autherize server
// time the challenge response was sent to client
// time the adr was first used, for authorize timeout checks
// this structure will be cleared only when the game dll changes
// sv_init has completed
// will be strictly increasing across level changes
// ^= SNAPFLAG_SERVERCOUNT every SV_SpawnServer()
// [sv_maxclients->integer];
// sv_maxclients->integer*PACKET_BACKUP*MAX_SNAPSHOT_ENTITIES
// next snapshotEntities to use
// [numSnapshotEntities]
// to prevent invalid IPs from connecting
// for rcon return messages
// authorize server address
// next svs.time that server should do dns lookup for master server
// Structure for managing bans
// For a CIDR-Notation type suffix
//=============================================================================
// persistant server info across maps
// cleared each map
// game virtual machine
//===========================================================
//
// sv_main.c
//
//
// sv_init.c
//
//
// sv_client.c
//
//
// sv_ccmds.c
//
//
// sv_snapshot.c
//
//
// sv_game.c
//
/*
===============
SV_InitGameProgs

Called on a normal map change, not on a map_restart
===============
*/
#[no_mangle]

pub unsafe extern "C" fn SV_InitGameProgs() {
    let mut var: *mut cvar_t = 0 as *mut cvar_t;
    //FIXME these are temp while I make bots run in vm
    extern "C" {
        #[no_mangle]
        pub static mut bot_enable: i32;
    }
    var = Cvar_Get(
        b"bot_enable\x00" as *const u8 as *const libc::c_char,
        b"1\x00" as *const u8 as *const libc::c_char,
        0x20 as i32,
    ) as *mut cvar_s;
    if !var.is_null() {
        bot_enable = (*var).integer
    } else {
        bot_enable = 0 as i32
    }
    // load the dll or bytecode
    gvm = VM_Create(
        b"qagame\x00" as *const u8 as *const libc::c_char,
        Some(SV_GameSystemCalls as unsafe extern "C" fn(_: *mut intptr_t) -> intptr_t),
        Cvar_VariableValue(b"vm_game\x00" as *const u8 as *const libc::c_char) as vmInterpret_t,
    );
    if gvm.is_null() {
        Com_Error(
            ERR_FATAL as i32,
            b"VM_Create on game failed\x00" as *const u8 as *const libc::c_char,
        );
    }
    SV_InitGameVM(qfalse);
}
/*
====================
SV_GameCommand

See if the current console command is claimed by the game
====================
*/
#[no_mangle]

pub unsafe extern "C" fn SV_GameCommand() -> qboolean {
    if sv.state as u32 != SS_GAME as i32 as u32 {
        return qfalse;
    }
    return VM_Call(gvm, GAME_CONSOLE_COMMAND as i32) as qboolean;
}

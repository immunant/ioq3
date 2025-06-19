#[repr(C)]
#[derive(Copy, Clone)]
pub struct snapshot_t {
    pub snapFlags: i32,
    pub ping: i32,
    pub serverTime: i32,
    pub areamask: [crate::src::qcommon::q_shared::byte; 32],
    pub ps: crate::src::qcommon::q_shared::playerState_t,
    pub numEntities: i32,
    pub entities: [crate::src::qcommon::q_shared::entityState_t; 256],
    pub numServerCommands: i32,
    pub serverCommandSequence: i32,
}
pub const CG_PRINT: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 0;
pub const CG_ERROR: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 1;
pub const CG_MILLISECONDS: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 2;
pub const CG_CVAR_REGISTER: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 3;
pub const CG_CVAR_UPDATE: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 4;
pub const CG_CVAR_SET: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 5;
pub const CG_CVAR_VARIABLESTRINGBUFFER: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 6;
pub const CG_ARGC: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 7;
pub const CG_ARGV: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 8;
pub const CG_ARGS: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 9;
pub const CG_FS_FOPENFILE: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 10;
pub const CG_FS_READ: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 11;
pub const CG_FS_WRITE: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 12;
pub const CG_FS_FCLOSEFILE: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 13;
pub const CG_SENDCONSOLECOMMAND: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 14;
pub const CG_ADDCOMMAND: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 15;
pub const CG_SENDCLIENTCOMMAND: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 16;
pub const CG_UPDATESCREEN: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 17;
pub const CG_CM_LOADMAP: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 18;
pub const CG_CM_NUMINLINEMODELS: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 19;
pub const CG_CM_INLINEMODEL: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 20;
pub const CG_CM_LOADMODEL: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 21;
pub const CG_CM_TEMPBOXMODEL: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 22;
pub const CG_CM_POINTCONTENTS: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 23;
pub const CG_CM_TRANSFORMEDPOINTCONTENTS: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 24;
pub const CG_CM_BOXTRACE: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 25;
pub const CG_CM_TRANSFORMEDBOXTRACE: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 26;
pub const CG_CM_MARKFRAGMENTS: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 27;
pub const CG_S_STARTSOUND: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 28;
pub const CG_S_STARTLOCALSOUND: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 29;
pub const CG_S_CLEARLOOPINGSOUNDS: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 30;
pub const CG_S_ADDLOOPINGSOUND: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 31;
pub const CG_S_UPDATEENTITYPOSITION: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 32;
pub const CG_S_RESPATIALIZE: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 33;
pub const CG_S_REGISTERSOUND: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 34;
pub const CG_S_STARTBACKGROUNDTRACK: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 35;
pub const CG_R_LOADWORLDMAP: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 36;
pub const CG_R_REGISTERMODEL: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 37;
pub const CG_R_REGISTERSKIN: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 38;
pub const CG_R_REGISTERSHADER: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 39;
pub const CG_R_CLEARSCENE: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 40;
pub const CG_R_ADDREFENTITYTOSCENE: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 41;
pub const CG_R_ADDPOLYTOSCENE: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 42;
pub const CG_R_ADDLIGHTTOSCENE: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 43;
pub const CG_R_RENDERSCENE: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 44;
pub const CG_R_SETCOLOR: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 45;
pub const CG_R_DRAWSTRETCHPIC: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 46;
pub const CG_R_MODELBOUNDS: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 47;
pub const CG_R_LERPTAG: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 48;
pub const CG_GETGLCONFIG: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 49;
pub const CG_GETGAMESTATE: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 50;
pub const CG_GETCURRENTSNAPSHOTNUMBER: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 51;
pub const CG_GETSNAPSHOT: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 52;
pub const CG_GETSERVERCOMMAND: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 53;
pub const CG_GETCURRENTCMDNUMBER: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 54;
pub const CG_GETUSERCMD: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 55;
pub const CG_SETUSERCMDVALUE: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 56;
pub const CG_R_REGISTERSHADERNOMIP: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 57;
pub const CG_MEMORY_REMAINING: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 58;
pub const CG_R_REGISTERFONT: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 59;
pub const CG_KEY_ISDOWN: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 60;
pub const CG_KEY_GETCATCHER: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 61;
pub const CG_KEY_SETCATCHER: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 62;
pub const CG_KEY_GETKEY: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 63;
pub const CG_PC_ADD_GLOBAL_DEFINE: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 64;
pub const CG_PC_LOAD_SOURCE: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 65;
pub const CG_PC_FREE_SOURCE: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 66;
pub const CG_PC_READ_TOKEN: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 67;
pub const CG_PC_SOURCE_FILE_AND_LINE: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 68;
pub const CG_S_STOPBACKGROUNDTRACK: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 69;
pub const CG_REAL_TIME: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 70;
pub const CG_SNAPVECTOR: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 71;
pub const CG_REMOVECOMMAND: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 72;
pub const CG_R_LIGHTFORPOINT: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 73;
pub const CG_CIN_PLAYCINEMATIC: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 74;
pub const CG_CIN_STOPCINEMATIC: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 75;
pub const CG_CIN_RUNCINEMATIC: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 76;
pub const CG_CIN_DRAWCINEMATIC: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 77;
pub const CG_CIN_SETEXTENTS: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 78;
pub const CG_R_REMAP_SHADER: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 79;
pub const CG_S_ADDREALLOOPINGSOUND: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 80;
pub const CG_S_STOPLOOPINGSOUND: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 81;
pub const CG_CM_TEMPCAPSULEMODEL: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 82;
pub const CG_CM_CAPSULETRACE: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 83;
pub const CG_CM_TRANSFORMEDCAPSULETRACE: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 84;
pub const CG_R_ADDADDITIVELIGHTTOSCENE: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 85;
pub const CG_GET_ENTITY_TOKEN: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 86;
pub const CG_R_ADDPOLYSTOSCENE: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 87;
pub const CG_R_INPVS: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 88;
// 1.32
pub const CG_FS_SEEK: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 89;
/*
    CG_LOADCAMERA,
    CG_STARTCAMERA,
    CG_GETCAMERAINFO,
*/
pub const CG_MEMSET: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 100;
pub const CG_MEMCPY: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 101;
pub const CG_STRNCPY: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 102;
pub const CG_SIN: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 103;
pub const CG_COS: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 104;
pub const CG_ATAN2: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 105;
pub const CG_SQRT: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 106;
pub const CG_FLOOR: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 107;
pub const CG_CEIL: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 108;
pub const CG_TESTPRINTINT: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 109;
pub const CG_TESTPRINTFLOAT: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 110;
pub const CG_ACOS: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 111;
pub const CG_INIT: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 0;
//	void CG_Init( int serverMessageNum, int serverCommandSequence, int clientNum )

// called when the level loads or when the renderer is restarted

// all media should be registered at this time

// cgame will display loading status by calling SCR_Update, which

// will call CG_DrawInformation during the loading process

// reliableCommandSequence will be 0 on fresh loads, but higher for

// demos, tourney restarts, or vid_restarts
pub const CG_SHUTDOWN: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 1;
//	void (*CG_Shutdown)( void );

// opportunity to flush and close any open files
pub const CG_CONSOLE_COMMAND: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 2;
//	qboolean (*CG_ConsoleCommand)( void );

// a console command has been issued locally that is not recognized by the

// main game system.

// use Cmd_Argc() / Cmd_Argv() to read the command, return qfalse if the

// command is not known to the game
pub const CG_DRAW_ACTIVE_FRAME: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 3;
//	void (*CG_DrawActiveFrame)( int serverTime, stereoFrame_t stereoView, qboolean demoPlayback );

// Generates and draws a game scene and status information at the given time.

// If demoPlayback is set, local movement prediction will not be enabled
pub const CG_CROSSHAIR_PLAYER: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 4;
//	int (*CG_CrosshairPlayer)( void );
pub const CG_LAST_ATTACKER: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 5;
//	int (*CG_LastAttacker)( void );
pub const CG_KEY_EVENT: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 6;
//	void	(*CG_KeyEvent)( int key, qboolean down );
pub const CG_MOUSE_EVENT: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 7;
//	void (*CG_EventHandling)(int type);

//	void	(*CG_MouseEvent)( int dx, int dy );
pub const CG_EVENT_HANDLING: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 8;

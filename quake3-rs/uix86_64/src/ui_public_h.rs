#[repr(C)]
#[derive(Copy, Clone)]
pub struct uiClientState_t {
    pub connState: crate::src::qcommon::q_shared::connstate_t,
    pub connectPacketCount: libc::c_int,
    pub clientNum: libc::c_int,
    pub servername: [libc::c_char; 1024],
    pub updateInfoString: [libc::c_char; 1024],
    pub messageString: [libc::c_char; 1024],
}
pub const UI_ERROR: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 0;
pub const UI_PRINT: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 1;
pub const UI_MILLISECONDS: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 2;
pub const UI_CVAR_SET: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 3;
pub const UI_CVAR_VARIABLEVALUE: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 4;
pub const UI_CVAR_VARIABLESTRINGBUFFER: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 5;
pub const UI_CVAR_SETVALUE: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 6;
pub const UI_CVAR_RESET: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 7;
pub const UI_CVAR_CREATE: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 8;
pub const UI_CVAR_INFOSTRINGBUFFER: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 9;
pub const UI_ARGC: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 10;
pub const UI_ARGV: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 11;
pub const UI_CMD_EXECUTETEXT: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 12;
pub const UI_FS_FOPENFILE: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 13;
pub const UI_FS_READ: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 14;
pub const UI_FS_WRITE: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 15;
pub const UI_FS_FCLOSEFILE: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 16;
pub const UI_FS_GETFILELIST: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 17;
pub const UI_R_REGISTERMODEL: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 18;
pub const UI_R_REGISTERSKIN: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 19;
pub const UI_R_REGISTERSHADERNOMIP: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 20;
pub const UI_R_CLEARSCENE: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 21;
pub const UI_R_ADDREFENTITYTOSCENE: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 22;
pub const UI_R_ADDPOLYTOSCENE: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 23;
pub const UI_R_ADDLIGHTTOSCENE: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 24;
pub const UI_R_RENDERSCENE: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 25;
pub const UI_R_SETCOLOR: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 26;
pub const UI_R_DRAWSTRETCHPIC: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 27;
pub const UI_UPDATESCREEN: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 28;
pub const UI_CM_LERPTAG: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 29;
pub const UI_CM_LOADMODEL: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 30;
pub const UI_S_REGISTERSOUND: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 31;
pub const UI_S_STARTLOCALSOUND: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 32;
pub const UI_KEY_KEYNUMTOSTRINGBUF: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 33;
pub const UI_KEY_GETBINDINGBUF: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 34;
pub const UI_KEY_SETBINDING: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 35;
pub const UI_KEY_ISDOWN: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 36;
pub const UI_KEY_GETOVERSTRIKEMODE: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 37;
pub const UI_KEY_SETOVERSTRIKEMODE: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 38;
pub const UI_KEY_CLEARSTATES: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 39;
pub const UI_KEY_GETCATCHER: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 40;
pub const UI_KEY_SETCATCHER: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 41;
pub const UI_GETCLIPBOARDDATA: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 42;
pub const UI_GETGLCONFIG: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 43;
pub const UI_GETCLIENTSTATE: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 44;
pub const UI_GETCONFIGSTRING: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 45;
pub const UI_LAN_GETPINGQUEUECOUNT: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 46;
pub const UI_LAN_CLEARPING: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 47;
pub const UI_LAN_GETPING: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 48;
pub const UI_LAN_GETPINGINFO: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 49;
pub const UI_CVAR_REGISTER: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 50;
pub const UI_CVAR_UPDATE: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 51;
pub const UI_MEMORY_REMAINING: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 52;
pub const UI_GET_CDKEY: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 53;
pub const UI_SET_CDKEY: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 54;
pub const UI_R_REGISTERFONT: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 55;
pub const UI_R_MODELBOUNDS: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 56;
pub const UI_PC_ADD_GLOBAL_DEFINE: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 57;
pub const UI_PC_LOAD_SOURCE: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 58;
pub const UI_PC_FREE_SOURCE: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 59;
pub const UI_PC_READ_TOKEN: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 60;
pub const UI_PC_SOURCE_FILE_AND_LINE: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 61;
pub const UI_S_STOPBACKGROUNDTRACK: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 62;
pub const UI_S_STARTBACKGROUNDTRACK: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 63;
pub const UI_REAL_TIME: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 64;
pub const UI_LAN_GETSERVERCOUNT: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 65;
pub const UI_LAN_GETSERVERADDRESSSTRING: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 66;
pub const UI_LAN_GETSERVERINFO: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 67;
pub const UI_LAN_MARKSERVERVISIBLE: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 68;
pub const UI_LAN_UPDATEVISIBLEPINGS: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 69;
pub const UI_LAN_RESETPINGS: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 70;
pub const UI_LAN_LOADCACHEDSERVERS: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 71;
pub const UI_LAN_SAVECACHEDSERVERS: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 72;
pub const UI_LAN_ADDSERVER: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 73;
pub const UI_LAN_REMOVESERVER: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 74;
pub const UI_CIN_PLAYCINEMATIC: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 75;
pub const UI_CIN_STOPCINEMATIC: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 76;
pub const UI_CIN_RUNCINEMATIC: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 77;
pub const UI_CIN_DRAWCINEMATIC: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 78;
pub const UI_CIN_SETEXTENTS: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 79;
pub const UI_R_REMAP_SHADER: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 80;
pub const UI_VERIFY_CDKEY: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 81;
pub const UI_LAN_SERVERSTATUS: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 82;
pub const UI_LAN_GETSERVERPING: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 83;
pub const UI_LAN_SERVERISVISIBLE: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 84;
pub const UI_LAN_COMPARESERVERS: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 85;
// 1.32
pub const UI_FS_SEEK: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 86;
pub const UI_SET_PBCLSTATUS: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 87;
pub const UI_MEMSET: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 100;
pub const UI_MEMCPY: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 101;
pub const UI_STRNCPY: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 102;
pub const UI_SIN: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 103;
pub const UI_COS: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 104;
pub const UI_ATAN2: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 105;
pub const UI_SQRT: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 106;
pub const UI_FLOOR: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 107;
pub const UI_CEIL: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 108;
pub type uiMenuCommand_t = libc::c_uint;
pub const UIMENU_NONE: uiMenuCommand_t = 0;
pub const UIMENU_MAIN: uiMenuCommand_t = 1;
pub const UIMENU_INGAME: uiMenuCommand_t = 2;
pub const UIMENU_NEED_CD: uiMenuCommand_t = 3;
pub const UIMENU_BAD_CD_KEY: uiMenuCommand_t = 4;
pub const UIMENU_TEAM: uiMenuCommand_t = 5;
pub const UIMENU_POSTGAME: uiMenuCommand_t = 6;
pub const UI_GETAPIVERSION: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 0;
// system reserved
pub const UI_INIT: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 1;
//	void	UI_Init( void );
pub const UI_SHUTDOWN: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 2;
//	void	UI_Shutdown( void );
pub const UI_KEY_EVENT: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 3;
//	void	UI_KeyEvent( int key );
pub const UI_MOUSE_EVENT: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 4;
//	void	UI_MouseEvent( int dx, int dy );
pub const UI_REFRESH: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 5;
//	void	UI_Refresh( int time );
pub const UI_IS_FULLSCREEN: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 6;
//	qboolean UI_IsFullscreen( void );
pub const UI_SET_ACTIVE_MENU: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 7;
//	void	UI_SetActiveMenu( uiMenuCommand_t menu );
pub const UI_CONSOLE_COMMAND: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 8;
//	qboolean UI_ConsoleCommand( int realTime );
pub const UI_DRAW_CONNECT_SCREEN: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 9;
// if !overlay, the background will be drawn, otherwise it will be

// overlayed over whatever the cgame has drawn.

// a GetClientState syscall will be made to get the current strings

//	void	UI_DrawConnectScreen( qboolean overlay );
pub const UI_HASUNIQUECDKEY: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 10;

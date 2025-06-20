use ::libc;

pub use crate::internal::__builtin_va_list;
pub use crate::internal::__va_list_tag;
pub use crate::stdarg_h::va_list;

pub use crate::src::q3_ui::ui_addbots::UI_AddBots_Cache;
pub use crate::src::q3_ui::ui_cdkey::UI_CDKeyMenu_Cache;
pub use crate::src::q3_ui::ui_cdkey::UI_CDKeyMenu_f;
pub use crate::src::q3_ui::ui_cinematics::UI_CinematicsMenu_Cache;
pub use crate::src::q3_ui::ui_cinematics::UI_CinematicsMenu_f;
pub use crate::src::q3_ui::ui_confirm::ConfirmMenu_Cache;
pub use crate::src::q3_ui::ui_confirm::UI_ConfirmMenu;
pub use crate::src::q3_ui::ui_controls2::Controls_Cache;
pub use crate::src::q3_ui::ui_demo2::Demos_Cache;
pub use crate::src::q3_ui::ui_display::UI_DisplayOptionsMenu_Cache;
pub use crate::src::q3_ui::ui_gameinfo::UI_InitGameinfo;
pub use crate::src::q3_ui::ui_gameinfo::UI_SPUnlockMedals_f;
pub use crate::src::q3_ui::ui_gameinfo::UI_SPUnlock_f;
pub use crate::src::q3_ui::ui_ingame::InGame_Cache;
pub use crate::src::q3_ui::ui_ingame::UI_InGameMenu;
pub use crate::src::q3_ui::ui_main::UI_RegisterCvars;
pub use crate::src::q3_ui::ui_main::UI_UpdateCvars;
pub use crate::src::q3_ui::ui_menu::MainMenu_Cache;
pub use crate::src::q3_ui::ui_menu::UI_MainMenu;
pub use crate::src::q3_ui::ui_mods::UI_ModsMenu_Cache;
pub use crate::src::q3_ui::ui_network::UI_NetworkOptionsMenu_Cache;
pub use crate::src::q3_ui::ui_playermodel::PlayerModel_Cache;
pub use crate::src::q3_ui::ui_playersettings::PlayerSettings_Cache;
pub use crate::src::q3_ui::ui_preferences::Preferences_Cache;
pub use crate::src::q3_ui::ui_qmenu::menu_in_sound;
pub use crate::src::q3_ui::ui_qmenu::menu_move_sound;
pub use crate::src::q3_ui::ui_qmenu::menu_null_sound;
pub use crate::src::q3_ui::ui_qmenu::menu_out_sound;
pub use crate::src::q3_ui::ui_qmenu::Menu_Cache;
pub use crate::src::q3_ui::ui_qmenu::Menu_DefaultKey;
pub use crate::src::q3_ui::ui_qmenu::Menu_Draw;
pub use crate::src::q3_ui::ui_qmenu::Menu_SetCursor;
pub use crate::src::q3_ui::ui_removebots::UI_RemoveBots_Cache;
pub use crate::src::q3_ui::ui_serverinfo::ServerInfo_Cache;
pub use crate::src::q3_ui::ui_servers2::ArenaServers_Cache;
pub use crate::src::q3_ui::ui_setup::UI_SetupMenu_Cache;
pub use crate::src::q3_ui::ui_sound::UI_SoundOptionsMenu_Cache;
pub use crate::src::q3_ui::ui_specifyserver::SpecifyServer_Cache;
pub use crate::src::q3_ui::ui_splevel::UI_SPLevelMenu_Cache;
pub use crate::src::q3_ui::ui_splevel::UI_SPLevelMenu_f;
pub use crate::src::q3_ui::ui_sppostgame::UI_SPPostgameMenu_Cache;
pub use crate::src::q3_ui::ui_sppostgame::UI_SPPostgameMenu_f;
pub use crate::src::q3_ui::ui_spskill::UI_SPSkillMenu_Cache;
pub use crate::src::q3_ui::ui_startserver::ServerOptions_Cache;
pub use crate::src::q3_ui::ui_startserver::StartServer_Cache;
pub use crate::src::q3_ui::ui_startserver::UI_BotSelectMenu_Cache;
pub use crate::src::q3_ui::ui_team::TeamMain_Cache;
pub use crate::src::q3_ui::ui_teamorders::UI_TeamOrdersMenu_f;
pub use crate::src::q3_ui::ui_video::DriverInfo_Cache;
pub use crate::src::q3_ui::ui_video::GraphicsOptions_Cache;
pub use crate::src::qcommon::q_math::colorBlack;
pub use crate::src::qcommon::q_math::colorWhite;
pub use crate::src::qcommon::q_math::g_color_table;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qhandle_t;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::sfxHandle_t;
pub use crate::src::qcommon::q_shared::vec4_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::qcommon::q_shared::C2RustUnnamed_0;
pub use crate::src::qcommon::q_shared::Q_IsColorString;
pub use crate::src::qcommon::q_shared::Q_stricmp;
pub use crate::src::qcommon::q_shared::Q_strncpyz;
pub use crate::src::qcommon::q_shared::CHAN_ANNOUNCER;
pub use crate::src::qcommon::q_shared::CHAN_AUTO;
pub use crate::src::qcommon::q_shared::CHAN_BODY;
pub use crate::src::qcommon::q_shared::CHAN_ITEM;
pub use crate::src::qcommon::q_shared::CHAN_LOCAL;
pub use crate::src::qcommon::q_shared::CHAN_LOCAL_SOUND;
pub use crate::src::qcommon::q_shared::CHAN_VOICE;
pub use crate::src::qcommon::q_shared::CHAN_WEAPON;
pub use crate::src::qcommon::q_shared::EXEC_APPEND;
pub use crate::src::qcommon::q_shared::EXEC_INSERT;
pub use crate::src::qcommon::q_shared::EXEC_NOW;
pub use crate::src::ui::ui_syscalls::trap_Argv;
pub use crate::src::ui::ui_syscalls::trap_Cmd_ExecuteText;
pub use crate::src::ui::ui_syscalls::trap_Cvar_Set;
pub use crate::src::ui::ui_syscalls::trap_Cvar_VariableStringBuffer;
pub use crate::src::ui::ui_syscalls::trap_Error;
pub use crate::src::ui::ui_syscalls::trap_GetGlconfig;
pub use crate::src::ui::ui_syscalls::trap_Key_ClearStates;
pub use crate::src::ui::ui_syscalls::trap_Key_GetCatcher;
pub use crate::src::ui::ui_syscalls::trap_Key_SetCatcher;
pub use crate::src::ui::ui_syscalls::trap_Print;
pub use crate::src::ui::ui_syscalls::trap_R_DrawStretchPic;
pub use crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip;
pub use crate::src::ui::ui_syscalls::trap_R_SetColor;
pub use crate::src::ui::ui_syscalls::trap_S_StartLocalSound;
pub use crate::src::ui::ui_syscalls::trap_UpdateScreen;

pub use crate::tr_types_h::glDriverType_t;
pub use crate::tr_types_h::glHardwareType_t;
pub use crate::tr_types_h::glconfig_t;
pub use crate::tr_types_h::textureCompression_t;
pub use crate::tr_types_h::GLDRV_ICD;
pub use crate::tr_types_h::GLDRV_STANDALONE;
pub use crate::tr_types_h::GLDRV_VOODOO;
pub use crate::tr_types_h::GLHW_3DFX_2D3D;
pub use crate::tr_types_h::GLHW_GENERIC;
pub use crate::tr_types_h::GLHW_PERMEDIA2;
pub use crate::tr_types_h::GLHW_RAGEPRO;
pub use crate::tr_types_h::GLHW_RIVA128;
pub use crate::tr_types_h::TC_NONE;
pub use crate::tr_types_h::TC_S3TC;
pub use crate::tr_types_h::TC_S3TC_ARB;
pub use crate::ui_local_h::_tag_menuframework;
pub use crate::ui_local_h::menucommon_s;
pub use crate::ui_local_h::menuframework_s;
pub use crate::ui_local_h::uiStatic_t;
pub use crate::ui_public_h::uiMenuCommand_t;
pub use crate::ui_public_h::UIMENU_BAD_CD_KEY;
pub use crate::ui_public_h::UIMENU_INGAME;
pub use crate::ui_public_h::UIMENU_MAIN;
pub use crate::ui_public_h::UIMENU_NEED_CD;
pub use crate::ui_public_h::UIMENU_NONE;
pub use crate::ui_public_h::UIMENU_POSTGAME;
pub use crate::ui_public_h::UIMENU_TEAM;
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
/* *********************************************************************
    UI_ATOMS.C

    User interface building blocks and support functions.
**********************************************************************/
#[no_mangle]

pub static mut uis: uiStatic_t = uiStatic_t {
    frametime: 0,
    realtime: 0,
    cursorx: 0,
    cursory: 0,
    menusp: 0,
    activemenu: std::ptr::null_mut(),
    stack: [std::ptr::null_mut(); 8],
    glconfig: glconfig_t {
        renderer_string: [0; 1024],
        vendor_string: [0; 1024],
        version_string: [0; 1024],
        extensions_string: [0; 8192],
        maxTextureSize: 0,
        numTextureUnits: 0,
        colorBits: 0,
        depthBits: 0,
        stencilBits: 0,
        driverType: GLDRV_ICD,
        hardwareType: GLHW_GENERIC,
        deviceSupportsGamma: qfalse,
        textureCompression: TC_NONE,
        textureEnvAddAvailable: qfalse,
        vidWidth: 0,
        vidHeight: 0,
        windowAspect: 0.,
        displayFrequency: 0,
        isFullscreen: qfalse,
        stereoEnabled: qfalse,
        smpActive: qfalse,
    },
    debug: qfalse,
    whiteShader: 0,
    menuBackShader: 0,
    menuBackNoLogoShader: 0,
    charset: 0,
    charsetProp: 0,
    charsetPropGlow: 0,
    charsetPropB: 0,
    cursor: 0,
    rb_on: 0,
    rb_off: 0,
    xscale: 0.,
    yscale: 0.,
    bias: 0.,
    demoversion: qfalse,
    firstdraw: qfalse,
};
#[no_mangle]

pub static mut m_entersound: qboolean = qfalse;
// after a frame, so caching won't disrupt the sound
#[no_mangle]

pub unsafe extern "C" fn Com_Error(
    mut _level: i32,
    mut error: *const libc::c_char,
    mut args: ...
) -> ! {
    let mut argptr: ::std::ffi::VaListImpl;
    let mut text: [libc::c_char; 1024] = [0; 1024];
    argptr = args.clone();
    crate::stdlib::vsnprintf(
        text.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as usize,
        error,
        argptr.as_va_list(),
    );
    trap_Error(text.as_mut_ptr());
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
// q_shared.h -- included first by ALL program modules.
// A user mod should never modify this file
// Heartbeat for dpmaster protocol. You shouldn't change this unless you know what you're doing
// When com_gamename is LEGACY_MASTER_GAMENAME, use quake3 master protocol.
// You shouldn't change this unless you know what you're doing
// number of supported master servers
// standard demo extension
//Ignore __attribute__ on non-gcc platforms
/* *********************************************************************
 VM Considerations

 The VM can not use the standard system headers because we aren't really
 using the compiler they were meant for.  We use bg_lib.h which contains
 prototypes for the functions we define for our own use in bg_lib.c.

 When writing mods, please add needed headers HERE, do not start including
 stuff like <stdio.h> in the various .c files that make up each of the VMs
 since you will be including system headers files can will have issues.

 Remember, if you use a C library function that is not defined in bg_lib.c,
 you will have to add your own version for support in the VM.

**********************************************************************/
//=============================================================
// expand constants before stringifying them
// angle indexes
// up / down
// left / right
// fall over
// the game guarantees that no string from the network will ever
// exceed MAX_STRING_CHARS
// max length of a string passed to Cmd_TokenizeString
// max tokens resulting from Cmd_TokenizeString
// max length of an individual token
// used for system info key only
// max length of a quake game pathname
// max length of a client name
// parameters for command buffer stuffing
// don't return until completed, a VM should NEVER use this,
// because some commands might cause the VM to be unloaded...
// insert at current position, but don't run yet
// add to end of the command buffer (normal case)
//
// these aren't needed by any of the VMs.  put in another header?
//
// bit vector of area visibility
// print levels from renderer (FIXME: set up for game / cgame?)
// only print when "developer 1"
// parameters to the main Error routine
// exit the entire game with a popup window
// print to console and disconnect from game
// don't kill server
// client disconnected from the server
// pop up the need-cd dialog
// font rendering values used by ui and cgame
// default
// default
/*
==============================================================

MATHLIB

==============================================================
*/
// all drawing is done to a 640*480 virtual screen size
// and will be automatically scaled to the real resolution
// ^[0-9a-zA-Z]
/*
// if your system does not have lrintf() and round() you can try this block. Please also open a bug report at bugzilla.icculus.org
// or write a mail to the ioq3 mailing list.
#else
  #define Q_ftol(v) ((long) (v))
  #define Q_round(v) do { if((v) < 0) (v) -= 0.5f; else (v) += 0.5f; (v) = Q_ftol((v)); } while(0)
  #define Q_SnapVector(vec) \
    do\
    {\
        vec3_t *temp = (vec);\
        \
        Q_round((*temp)[0]);\
        Q_round((*temp)[1]);\
        Q_round((*temp)[2]);\
    } while(0)
#endif
*/
// reciprocal square root
// this isn't a real cheap function to call!
// just in case you don't want to use the macros
// fast vector normalize routine that does not check to make sure
// that length != 0, nor does it return length, uses rsqrt approximation
// returns vector length
// perpendicular vector could be replaced by this
//int	PlaneTypeForNormal (vec3_t normal);
//=============================================
//int		COM_ParseInfos( char *buf, int max, char infos[][MAX_INFO_STRING] );
//token types
// string
// literal
// number
// name
// punctuation
// data is an in/out parm, returns a parsed out token
// mode parm for FS_FOpenFile
//=============================================
// portable case insensitive compare
// buffer size safe library replacements
// strlen that discounts Quake color sequences
// removes color sequences from string
// Count the number of char tocount encountered in string
//=============================================
// 64-bit integers for global rankings interface
// implemented as a struct for qvm compatibility
//=============================================
/*
short	BigShort(short l);
short	LittleShort(short l);
int		BigLong (int l);
int		LittleLong (int l);
qint64  BigLong64 (qint64 l);
qint64  LittleLong64 (qint64 l);
float	BigFloat (const float *l);
float	LittleFloat (const float *l);

void	Swap_Init (void);
*/
//=============================================
//
// key / value info strings
//
// this is only here so the functions in q_shared.c and bg_*.c can link
#[no_mangle]

pub unsafe extern "C" fn Com_Printf(mut msg: *const libc::c_char, mut args: ...) {
    let mut argptr: ::std::ffi::VaListImpl;
    let mut text: [libc::c_char; 1024] = [0; 1024];
    argptr = args.clone();
    crate::stdlib::vsnprintf(
        text.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as usize,
        msg,
        argptr.as_va_list(),
    );
    trap_Print(text.as_mut_ptr());
}
/*
=================
UI_ClampCvar
=================
*/
#[no_mangle]

pub unsafe extern "C" fn UI_ClampCvar(mut min: f32, mut max: f32, mut value: f32) -> f32 {
    if value < min {
        return min;
    }
    if value > max {
        return max;
    }
    return value;
}
/*
=================
UI_StartDemoLoop
=================
*/
#[no_mangle]

pub unsafe extern "C" fn UI_StartDemoLoop() {
    trap_Cmd_ExecuteText(
        EXEC_APPEND as i32,
        b"d1\n\x00" as *const u8 as *const libc::c_char,
    );
}
/*
=================
UI_PushMenu
=================
*/
#[no_mangle]

pub unsafe extern "C" fn UI_PushMenu(mut menu: *mut menuframework_s) {
    let mut i: i32 = 0;
    let mut item: *mut menucommon_s = std::ptr::null_mut();
    // avoid stacking menus invoked by hotkeys
    i = 0 as i32;
    while i < uis.menusp {
        if uis.stack[i as usize] == menu {
            uis.menusp = i;
            break;
        } else {
            i += 1
        }
    }
    if i == uis.menusp {
        if uis.menusp >= 8 as i32 {
            trap_Error(b"UI_PushMenu: menu stack overflow\x00" as *const u8 as *const libc::c_char);
        }
        let fresh0 = uis.menusp;
        uis.menusp = uis.menusp + 1;
        uis.stack[fresh0 as usize] = menu
    }
    uis.activemenu = menu;
    // default cursor position
    (*menu).cursor = 0 as i32;
    (*menu).cursor_prev = 0 as i32;
    m_entersound = qtrue;
    trap_Key_SetCatcher(0x2 as i32);
    // force first available item to have focus
    i = 0 as i32;
    while i < (*menu).nitems {
        item = (*menu).items[i as usize] as *mut menucommon_s;
        if (*item).flags & (0x2000 as i32 as u32 | 0x800 as i32 as u32 | 0x4000 as i32 as u32) == 0
        {
            (*menu).cursor_prev = -(1 as i32);
            Menu_SetCursor(menu as *mut _tag_menuframework, i);
            break;
        } else {
            i += 1
        }
    }
    uis.firstdraw = qtrue;
}
/*
=================
UI_PopMenu
=================
*/
#[no_mangle]

pub unsafe extern "C" fn UI_PopMenu() {
    trap_S_StartLocalSound(menu_out_sound, CHAN_LOCAL_SOUND as i32);
    uis.menusp -= 1;
    if uis.menusp < 0 as i32 {
        trap_Error(b"UI_PopMenu: menu stack underflow\x00" as *const u8 as *const libc::c_char);
    }
    if uis.menusp != 0 {
        uis.activemenu = uis.stack[(uis.menusp - 1 as i32) as usize];
        uis.firstdraw = qtrue
    } else {
        UI_ForceMenuOff();
    };
}
#[no_mangle]

pub unsafe extern "C" fn UI_ForceMenuOff() {
    uis.menusp = 0 as i32;
    uis.activemenu = std::ptr::null_mut();
    trap_Key_SetCatcher(trap_Key_GetCatcher() & !(0x2 as i32));
    trap_Key_ClearStates();
    trap_Cvar_Set(
        b"cl_paused\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
    );
}
/*
=================
UI_LerpColor
=================
*/
#[no_mangle]

pub unsafe extern "C" fn UI_LerpColor(
    mut a: *mut vec_t,
    mut b: *mut vec_t,
    mut c: *mut vec_t,
    mut t: f32,
) {
    let mut i: i32 = 0;
    // lerp and clamp each component
    i = 0 as i32;
    while i < 4 as i32 {
        *c.offset(i as isize) =
            *a.offset(i as isize) + t * (*b.offset(i as isize) - *a.offset(i as isize));
        if *c.offset(i as isize) < 0 as i32 as f32 {
            *c.offset(i as isize) = 0 as i32 as vec_t
        } else if *c.offset(i as isize) as f64 > 1.0f64 {
            *c.offset(i as isize) = 1.0f64 as vec_t
        }
        i += 1
    }
}
/*
=================
UI_DrawProportionalString2
=================
*/

static mut propMap: [[i32; 3]; 128] = [
    [0 as i32, 0 as i32, -(1 as i32)],
    [0 as i32, 0 as i32, -(1 as i32)],
    [0 as i32, 0 as i32, -(1 as i32)],
    [0 as i32, 0 as i32, -(1 as i32)],
    [0 as i32, 0 as i32, -(1 as i32)],
    [0 as i32, 0 as i32, -(1 as i32)],
    [0 as i32, 0 as i32, -(1 as i32)],
    [0 as i32, 0 as i32, -(1 as i32)],
    [0 as i32, 0 as i32, -(1 as i32)],
    [0 as i32, 0 as i32, -(1 as i32)],
    [0 as i32, 0 as i32, -(1 as i32)],
    [0 as i32, 0 as i32, -(1 as i32)],
    [0 as i32, 0 as i32, -(1 as i32)],
    [0 as i32, 0 as i32, -(1 as i32)],
    [0 as i32, 0 as i32, -(1 as i32)],
    [0 as i32, 0 as i32, -(1 as i32)],
    [0 as i32, 0 as i32, -(1 as i32)],
    [0 as i32, 0 as i32, -(1 as i32)],
    [0 as i32, 0 as i32, -(1 as i32)],
    [0 as i32, 0 as i32, -(1 as i32)],
    [0 as i32, 0 as i32, -(1 as i32)],
    [0 as i32, 0 as i32, -(1 as i32)],
    [0 as i32, 0 as i32, -(1 as i32)],
    [0 as i32, 0 as i32, -(1 as i32)],
    [0 as i32, 0 as i32, -(1 as i32)],
    [0 as i32, 0 as i32, -(1 as i32)],
    [0 as i32, 0 as i32, -(1 as i32)],
    [0 as i32, 0 as i32, -(1 as i32)],
    [0 as i32, 0 as i32, -(1 as i32)],
    [0 as i32, 0 as i32, -(1 as i32)],
    [0 as i32, 0 as i32, -(1 as i32)],
    [0 as i32, 0 as i32, -(1 as i32)],
    [0 as i32, 0 as i32, 8 as i32],
    [11 as i32, 122 as i32, 7 as i32],
    [154 as i32, 181 as i32, 14 as i32],
    [55 as i32, 122 as i32, 17 as i32],
    [79 as i32, 122 as i32, 18 as i32],
    [101 as i32, 122 as i32, 23 as i32],
    [153 as i32, 122 as i32, 18 as i32],
    [9 as i32, 93 as i32, 7 as i32],
    [207 as i32, 122 as i32, 8 as i32],
    [230 as i32, 122 as i32, 9 as i32],
    [177 as i32, 122 as i32, 18 as i32],
    [30 as i32, 152 as i32, 18 as i32],
    [85 as i32, 181 as i32, 7 as i32],
    [34 as i32, 93 as i32, 11 as i32],
    [110 as i32, 181 as i32, 6 as i32],
    [130 as i32, 152 as i32, 14 as i32],
    [22 as i32, 64 as i32, 17 as i32],
    [41 as i32, 64 as i32, 12 as i32],
    [58 as i32, 64 as i32, 17 as i32],
    [78 as i32, 64 as i32, 18 as i32],
    [98 as i32, 64 as i32, 19 as i32],
    [120 as i32, 64 as i32, 18 as i32],
    [141 as i32, 64 as i32, 18 as i32],
    [204 as i32, 64 as i32, 16 as i32],
    [162 as i32, 64 as i32, 17 as i32],
    [182 as i32, 64 as i32, 18 as i32],
    [59 as i32, 181 as i32, 7 as i32],
    [35 as i32, 181 as i32, 7 as i32],
    [203 as i32, 152 as i32, 14 as i32],
    [56 as i32, 93 as i32, 14 as i32],
    [228 as i32, 152 as i32, 14 as i32],
    [177 as i32, 181 as i32, 18 as i32],
    [28 as i32, 122 as i32, 22 as i32],
    [5 as i32, 4 as i32, 18 as i32],
    [27 as i32, 4 as i32, 18 as i32],
    [48 as i32, 4 as i32, 18 as i32],
    [69 as i32, 4 as i32, 17 as i32],
    [90 as i32, 4 as i32, 13 as i32],
    [106 as i32, 4 as i32, 13 as i32],
    [121 as i32, 4 as i32, 18 as i32],
    [143 as i32, 4 as i32, 17 as i32],
    [164 as i32, 4 as i32, 8 as i32],
    [175 as i32, 4 as i32, 16 as i32],
    [195 as i32, 4 as i32, 18 as i32],
    [216 as i32, 4 as i32, 12 as i32],
    [230 as i32, 4 as i32, 23 as i32],
    [6 as i32, 34 as i32, 18 as i32],
    [27 as i32, 34 as i32, 18 as i32],
    [48 as i32, 34 as i32, 18 as i32],
    [68 as i32, 34 as i32, 18 as i32],
    [90 as i32, 34 as i32, 17 as i32],
    [110 as i32, 34 as i32, 18 as i32],
    [130 as i32, 34 as i32, 14 as i32],
    [146 as i32, 34 as i32, 18 as i32],
    [166 as i32, 34 as i32, 19 as i32],
    [185 as i32, 34 as i32, 29 as i32],
    [215 as i32, 34 as i32, 18 as i32],
    [234 as i32, 34 as i32, 18 as i32],
    [5 as i32, 64 as i32, 14 as i32],
    [60 as i32, 152 as i32, 7 as i32],
    [106 as i32, 151 as i32, 13 as i32],
    [83 as i32, 152 as i32, 7 as i32],
    [128 as i32, 122 as i32, 17 as i32],
    [4 as i32, 152 as i32, 21 as i32],
    [134 as i32, 181 as i32, 5 as i32],
    [5 as i32, 4 as i32, 18 as i32],
    [27 as i32, 4 as i32, 18 as i32],
    [48 as i32, 4 as i32, 18 as i32],
    [69 as i32, 4 as i32, 17 as i32],
    [90 as i32, 4 as i32, 13 as i32],
    [106 as i32, 4 as i32, 13 as i32],
    [121 as i32, 4 as i32, 18 as i32],
    [143 as i32, 4 as i32, 17 as i32],
    [164 as i32, 4 as i32, 8 as i32],
    [175 as i32, 4 as i32, 16 as i32],
    [195 as i32, 4 as i32, 18 as i32],
    [216 as i32, 4 as i32, 12 as i32],
    [230 as i32, 4 as i32, 23 as i32],
    [6 as i32, 34 as i32, 18 as i32],
    [27 as i32, 34 as i32, 18 as i32],
    [48 as i32, 34 as i32, 18 as i32],
    [68 as i32, 34 as i32, 18 as i32],
    [90 as i32, 34 as i32, 17 as i32],
    [110 as i32, 34 as i32, 18 as i32],
    [130 as i32, 34 as i32, 14 as i32],
    [146 as i32, 34 as i32, 18 as i32],
    [166 as i32, 34 as i32, 19 as i32],
    [185 as i32, 34 as i32, 29 as i32],
    [215 as i32, 34 as i32, 18 as i32],
    [234 as i32, 34 as i32, 18 as i32],
    [5 as i32, 64 as i32, 14 as i32],
    [153 as i32, 152 as i32, 13 as i32],
    [11 as i32, 181 as i32, 5 as i32],
    [180 as i32, 152 as i32, 13 as i32],
    [79 as i32, 93 as i32, 17 as i32],
    [0 as i32, 0 as i32, -(1 as i32)],
];

static mut propMapB: [[i32; 3]; 26] = [
    [11 as i32, 12 as i32, 33 as i32],
    [49 as i32, 12 as i32, 31 as i32],
    [85 as i32, 12 as i32, 31 as i32],
    [120 as i32, 12 as i32, 30 as i32],
    [156 as i32, 12 as i32, 21 as i32],
    [183 as i32, 12 as i32, 21 as i32],
    [207 as i32, 12 as i32, 32 as i32],
    [13 as i32, 55 as i32, 30 as i32],
    [49 as i32, 55 as i32, 13 as i32],
    [66 as i32, 55 as i32, 29 as i32],
    [101 as i32, 55 as i32, 31 as i32],
    [135 as i32, 55 as i32, 21 as i32],
    [158 as i32, 55 as i32, 40 as i32],
    [204 as i32, 55 as i32, 32 as i32],
    [12 as i32, 97 as i32, 31 as i32],
    [48 as i32, 97 as i32, 31 as i32],
    [82 as i32, 97 as i32, 30 as i32],
    [118 as i32, 97 as i32, 30 as i32],
    [153 as i32, 97 as i32, 30 as i32],
    [185 as i32, 97 as i32, 25 as i32],
    [213 as i32, 97 as i32, 30 as i32],
    [11 as i32, 139 as i32, 32 as i32],
    [42 as i32, 139 as i32, 51 as i32],
    [93 as i32, 139 as i32, 32 as i32],
    [126 as i32, 139 as i32, 31 as i32],
    [158 as i32, 139 as i32, 25 as i32],
];
/*
=================
UI_DrawBannerString
=================
*/

unsafe extern "C" fn UI_DrawBannerString2(
    mut x: i32,
    mut y: i32,
    mut str: *const libc::c_char,
    mut color: *mut vec_t,
) {
    let mut s: *const libc::c_char = std::ptr::null();
    let mut ch: u8 = 0;
    let mut ax: f32 = 0.;
    let mut ay: f32 = 0.;
    let mut aw: f32 = 0.;
    let mut ah: f32 = 0.;
    let mut frow: f32 = 0.;
    let mut fcol: f32 = 0.;
    let mut fwidth: f32 = 0.;
    let mut fheight: f32 = 0.;
    // draw the colored text
    trap_R_SetColor(color as *const f32);
    ax = x as f32 * uis.xscale + uis.bias;
    ay = y as f32 * uis.yscale;
    s = str;
    while *s != 0 {
        ch = (*s as i32 & 127 as i32) as u8;
        if ch as i32 == ' ' as i32 {
            ax += (12 as i32 as f32 + 4 as i32 as f32) * uis.xscale
        } else if ch as i32 >= 'A' as i32 && ch as i32 <= 'Z' as i32 {
            ch = (ch as i32 - 'A' as i32) as u8;
            fcol = propMapB[ch as usize][0 as i32 as usize] as f32 / 256.0f32;
            frow = propMapB[ch as usize][1 as i32 as usize] as f32 / 256.0f32;
            fwidth = propMapB[ch as usize][2 as i32 as usize] as f32 / 256.0f32;
            fheight = 36 as i32 as f32 / 256.0f32;
            aw = propMapB[ch as usize][2 as i32 as usize] as f32 * uis.xscale;
            ah = 36 as i32 as f32 * uis.yscale;
            trap_R_DrawStretchPic(
                ax,
                ay,
                aw,
                ah,
                fcol,
                frow,
                fcol + fwidth,
                frow + fheight,
                uis.charsetPropB,
            );
            ax += aw + 4 as i32 as f32 * uis.xscale
        }
        s = s.offset(1)
    }
    trap_R_SetColor(std::ptr::null());
}
#[no_mangle]

pub unsafe extern "C" fn UI_DrawBannerString(
    mut x: i32,
    mut y: i32,
    mut str: *const libc::c_char,
    mut style: i32,
    mut color: *mut vec_t,
) {
    let mut s: *const libc::c_char = std::ptr::null();
    let mut ch: i32 = 0;
    let mut width: i32 = 0;
    let mut drawcolor: vec4_t = [0.; 4];
    // find the width of the drawn text
    s = str;
    width = 0 as i32;
    while *s != 0 {
        ch = *s as i32;
        if ch == ' ' as i32 {
            width += 12 as i32
        } else if ch >= 'A' as i32 && ch <= 'Z' as i32 {
            width += propMapB[(ch - 'A' as i32) as usize][2 as i32 as usize] + 4 as i32
        }
        s = s.offset(1)
    }
    width -= 4 as i32;
    match style & 0x7 as i32 {
        1 => x -= width / 2 as i32,
        2 => x -= width,
        0 | _ => {}
    }
    if style & 0x800 as i32 != 0 {
        drawcolor[2 as i32 as usize] = 0 as i32 as vec_t;
        drawcolor[1 as i32 as usize] = drawcolor[2 as i32 as usize];
        drawcolor[0 as i32 as usize] = drawcolor[1 as i32 as usize];
        drawcolor[3 as i32 as usize] = *color.offset(3 as i32 as isize);
        UI_DrawBannerString2(x + 2 as i32, y + 2 as i32, str, drawcolor.as_mut_ptr());
    }
    UI_DrawBannerString2(x, y, str, color);
}
#[no_mangle]

pub unsafe extern "C" fn UI_ProportionalStringWidth(mut str: *const libc::c_char) -> i32 {
    let mut s: *const libc::c_char = std::ptr::null();
    let mut ch: i32 = 0;
    let mut charWidth: i32 = 0;
    let mut width: i32 = 0;
    s = str;
    width = 0 as i32;
    while *s != 0 {
        ch = *s as i32 & 127 as i32;
        charWidth = propMap[ch as usize][2 as i32 as usize];
        if charWidth != -(1 as i32) {
            width += charWidth;
            width += 3 as i32
        }
        s = s.offset(1)
    }
    width -= 3 as i32;
    return width;
}

unsafe extern "C" fn UI_DrawProportionalString2(
    mut x: i32,
    mut y: i32,
    mut str: *const libc::c_char,
    mut color: *mut vec_t,
    mut sizeScale: f32,
    mut charset: qhandle_t,
) {
    let mut s: *const libc::c_char = std::ptr::null();
    let mut ch: u8 = 0;
    let mut ax: f32 = 0.;
    let mut ay: f32 = 0.;
    let mut aw: f32 = 0 as i32 as f32;
    let mut ah: f32 = 0.;
    let mut frow: f32 = 0.;
    let mut fcol: f32 = 0.;
    let mut fwidth: f32 = 0.;
    let mut fheight: f32 = 0.;
    // draw the colored text
    trap_R_SetColor(color as *const f32);
    ax = x as f32 * uis.xscale + uis.bias;
    ay = y as f32 * uis.yscale;
    s = str;
    while *s != 0 {
        ch = (*s as i32 & 127 as i32) as u8;
        if ch as i32 == ' ' as i32 {
            aw = 8 as i32 as f32 * uis.xscale * sizeScale
        } else if propMap[ch as usize][2 as i32 as usize] != -(1 as i32) {
            fcol = propMap[ch as usize][0 as i32 as usize] as f32 / 256.0f32;
            frow = propMap[ch as usize][1 as i32 as usize] as f32 / 256.0f32;
            fwidth = propMap[ch as usize][2 as i32 as usize] as f32 / 256.0f32;
            fheight = 27 as i32 as f32 / 256.0f32;
            aw = propMap[ch as usize][2 as i32 as usize] as f32 * uis.xscale * sizeScale;
            ah = 27 as i32 as f32 * uis.yscale * sizeScale;
            trap_R_DrawStretchPic(
                ax,
                ay,
                aw,
                ah,
                fcol,
                frow,
                fcol + fwidth,
                frow + fheight,
                charset,
            );
        }
        ax += aw + 3 as i32 as f32 * uis.xscale * sizeScale;
        s = s.offset(1)
    }
    trap_R_SetColor(std::ptr::null());
}
/*
=================
UI_ProportionalSizeScale
=================
*/
#[no_mangle]

pub unsafe extern "C" fn UI_ProportionalSizeScale(mut style: i32) -> f32 {
    if style & 0x10 as i32 != 0 {
        return 0.75f64 as f32;
    }
    return 1.00f64 as f32;
}
/*
=================
UI_DrawProportionalString
=================
*/
#[no_mangle]

pub unsafe extern "C" fn UI_DrawProportionalString(
    mut x: i32,
    mut y: i32,
    mut str: *const libc::c_char,
    mut style: i32,
    mut color: *mut vec_t,
) {
    let mut drawcolor: vec4_t = [0.; 4];
    let mut width: i32 = 0;
    let mut sizeScale: f32 = 0.;
    if str.is_null() {
        return;
    }
    sizeScale = UI_ProportionalSizeScale(style);
    match style & 0x7 as i32 {
        1 => {
            width = (UI_ProportionalStringWidth(str) as f32 * sizeScale) as i32;
            x -= width / 2 as i32
        }
        2 => {
            width = (UI_ProportionalStringWidth(str) as f32 * sizeScale) as i32;
            x -= width
        }
        0 | _ => {}
    }
    if style & 0x800 as i32 != 0 {
        drawcolor[2 as i32 as usize] = 0 as i32 as vec_t;
        drawcolor[1 as i32 as usize] = drawcolor[2 as i32 as usize];
        drawcolor[0 as i32 as usize] = drawcolor[1 as i32 as usize];
        drawcolor[3 as i32 as usize] = *color.offset(3 as i32 as isize);
        UI_DrawProportionalString2(
            x + 2 as i32,
            y + 2 as i32,
            str,
            drawcolor.as_mut_ptr(),
            sizeScale,
            uis.charsetProp,
        );
    }
    if style & 0x2000 as i32 != 0 {
        drawcolor[0 as i32 as usize] = (*color.offset(0 as i32 as isize) as f64 * 0.7f64) as vec_t;
        drawcolor[1 as i32 as usize] = (*color.offset(1 as i32 as isize) as f64 * 0.7f64) as vec_t;
        drawcolor[2 as i32 as usize] = (*color.offset(2 as i32 as isize) as f64 * 0.7f64) as vec_t;
        drawcolor[3 as i32 as usize] = *color.offset(3 as i32 as isize);
        UI_DrawProportionalString2(
            x,
            y,
            str,
            drawcolor.as_mut_ptr(),
            sizeScale,
            uis.charsetProp,
        );
        return;
    }
    if style & 0x4000 as i32 != 0 {
        drawcolor[0 as i32 as usize] = (*color.offset(0 as i32 as isize) as f64 * 0.7f64) as vec_t;
        drawcolor[1 as i32 as usize] = (*color.offset(1 as i32 as isize) as f64 * 0.7f64) as vec_t;
        drawcolor[2 as i32 as usize] = (*color.offset(2 as i32 as isize) as f64 * 0.7f64) as vec_t;
        drawcolor[3 as i32 as usize] = *color.offset(3 as i32 as isize);
        UI_DrawProportionalString2(x, y, str, color, sizeScale, uis.charsetProp);
        drawcolor[0 as i32 as usize] = *color.offset(0 as i32 as isize);
        drawcolor[1 as i32 as usize] = *color.offset(1 as i32 as isize);
        drawcolor[2 as i32 as usize] = *color.offset(2 as i32 as isize);
        drawcolor[3 as i32 as usize] =
            (0.5f64 + 0.5f64 * crate::stdlib::sin((uis.realtime / 75 as i32) as f64)) as vec_t;
        UI_DrawProportionalString2(
            x,
            y,
            str,
            drawcolor.as_mut_ptr(),
            sizeScale,
            uis.charsetPropGlow,
        );
        return;
    }
    UI_DrawProportionalString2(x, y, str, color, sizeScale, uis.charsetProp);
}
/*
=================
UI_DrawProportionalString_Wrapped
=================
*/
#[no_mangle]

pub unsafe extern "C" fn UI_DrawProportionalString_AutoWrapped(
    mut x: i32,
    mut y: i32,
    mut xmax: i32,
    mut ystep: i32,
    mut str: *const libc::c_char,
    mut style: i32,
    mut color: *mut vec_t,
) {
    let mut width: i32 = 0;
    let mut s1: *mut libc::c_char = std::ptr::null_mut();
    let mut s2: *mut libc::c_char = std::ptr::null_mut();
    let mut s3: *mut libc::c_char = std::ptr::null_mut();
    let mut c_bcp: libc::c_char = 0;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut sizeScale: f32 = 0.;
    if str.is_null() || *str.offset(0 as i32 as isize) as i32 == '\u{0}' as i32 {
        return;
    }
    sizeScale = UI_ProportionalSizeScale(style);
    Q_strncpyz(
        buf.as_mut_ptr(),
        str,
        ::std::mem::size_of::<[libc::c_char; 1024]>() as usize as i32,
    );
    s3 = buf.as_mut_ptr();
    s2 = s3;
    s1 = s2;
    loop {
        loop {
            s3 = s3.offset(1);
            if !(*s3 as i32 != ' ' as i32 && *s3 as i32 != '\u{0}' as i32) {
                break;
            }
        }
        c_bcp = *s3;
        *s3 = '\u{0}' as i32 as libc::c_char;
        width = (UI_ProportionalStringWidth(s1) as f32 * sizeScale) as i32;
        *s3 = c_bcp;
        if width > xmax {
            if s1 == s2 {
                // fuck, don't have a clean cut, we'll overflow
                s2 = s3
            }
            *s2 = '\u{0}' as i32 as libc::c_char;
            UI_DrawProportionalString(x, y, s1, style, color);
            y += ystep;
            if c_bcp as i32 == '\u{0}' as i32 {
                // that was the last word
                // we could start a new loop, but that wouldn't be much use
                // even if the word is too long, we would overflow it (see above)
                // so just print it now if needed
                s2 = s2.offset(1);
                if *s2 as i32 != '\u{0}' as i32 {
                    // if we are printing an overflowing line we have s2 == s3
                    UI_DrawProportionalString(x, y, s2, style, color);
                }
                break;
            } else {
                s2 = s2.offset(1);
                s1 = s2;
                s3 = s2
            }
        } else {
            s2 = s3;
            if !(c_bcp as i32 == '\u{0}' as i32) {
                continue;
            }
            // we reached the end
            UI_DrawProportionalString(x, y, s1, style, color);
            break;
        }
    }
}
/*
=================
UI_DrawString2
=================
*/

unsafe extern "C" fn UI_DrawString2(
    mut x: i32,
    mut y: i32,
    mut str: *const libc::c_char,
    mut color: *mut vec_t,
    mut charw: i32,
    mut charh: i32,
) {
    let mut s: *const libc::c_char = std::ptr::null(); //APSFIXME;
    let mut ch: libc::c_char = 0;
    let mut forceColor: i32 = qfalse as i32;
    let mut tempcolor: vec4_t = [0.; 4];
    let mut ax: f32 = 0.;
    let mut ay: f32 = 0.;
    let mut aw: f32 = 0.;
    let mut ah: f32 = 0.;
    let mut frow: f32 = 0.;
    let mut fcol: f32 = 0.;
    if y < -charh {
        // offscreen
        return;
    }
    // draw the colored text
    trap_R_SetColor(color as *const f32);
    ax = x as f32 * uis.xscale + uis.bias;
    ay = y as f32 * uis.yscale;
    aw = charw as f32 * uis.xscale;
    ah = charh as f32 * uis.yscale;
    s = str;
    while *s != 0 {
        if Q_IsColorString(s) as u64 != 0 {
            if forceColor == 0 {
                crate::stdlib::memcpy(
                    tempcolor.as_mut_ptr() as *mut libc::c_void,
                    g_color_table
                        [(*s.offset(1 as i32 as isize) as i32 - '0' as i32 & 0x7 as i32) as usize]
                        .as_mut_ptr() as *const libc::c_void,
                    ::std::mem::size_of::<vec4_t>() as usize,
                );
                tempcolor[3 as i32 as usize] = *color.offset(3 as i32 as isize);
                trap_R_SetColor(tempcolor.as_mut_ptr());
            }
            s = s.offset(2 as i32 as isize)
        } else {
            ch = (*s as i32 & 255 as i32) as libc::c_char;
            if ch as i32 != ' ' as i32 {
                frow = ((ch as i32 >> 4 as i32) as f64 * 0.0625f64) as f32;
                fcol = ((ch as i32 & 15 as i32) as f64 * 0.0625f64) as f32;
                trap_R_DrawStretchPic(
                    ax,
                    ay,
                    aw,
                    ah,
                    fcol,
                    frow,
                    (fcol as f64 + 0.0625f64) as f32,
                    (frow as f64 + 0.0625f64) as f32,
                    uis.charset,
                );
            }
            ax += aw;
            s = s.offset(1)
        }
    }
    trap_R_SetColor(std::ptr::null());
}
/*
=================
UI_DrawString
=================
*/
#[no_mangle]

pub unsafe extern "C" fn UI_DrawString(
    mut x: i32,
    mut y: i32,
    mut str: *const libc::c_char,
    mut style: i32,
    mut color: *mut vec_t,
) {
    let mut len: i32 = 0;
    let mut charw: i32 = 0;
    let mut charh: i32 = 0;
    let mut newcolor: vec4_t = [0.; 4];
    let mut lowlight: vec4_t = [0.; 4];
    let mut drawcolor: *mut f32 = std::ptr::null_mut();
    let mut dropcolor: vec4_t = [0.; 4];
    if str.is_null() {
        return;
    }
    if style & 0x1000 as i32 != 0 && uis.realtime / 200 as i32 & 1 as i32 != 0 {
        return;
    }
    if style & 0x10 as i32 != 0 {
        charw = 8 as i32;
        charh = 16 as i32
    } else if style & 0x40 as i32 != 0 {
        charw = 32 as i32;
        charh = 48 as i32
    } else {
        charw = 16 as i32;
        charh = 16 as i32
    }
    if style & 0x4000 as i32 != 0 {
        lowlight[0 as i32 as usize] = (0.8f64 * *color.offset(0 as i32 as isize) as f64) as vec_t;
        lowlight[1 as i32 as usize] = (0.8f64 * *color.offset(1 as i32 as isize) as f64) as vec_t;
        lowlight[2 as i32 as usize] = (0.8f64 * *color.offset(2 as i32 as isize) as f64) as vec_t;
        lowlight[3 as i32 as usize] = (0.8f64 * *color.offset(3 as i32 as isize) as f64) as vec_t;
        UI_LerpColor(
            color,
            lowlight.as_mut_ptr(),
            newcolor.as_mut_ptr(),
            (0.5f64 + 0.5f64 * crate::stdlib::sin((uis.realtime / 75 as i32) as f64)) as f32,
        );
        drawcolor = newcolor.as_mut_ptr()
    } else {
        drawcolor = color
    }
    match style & 0x7 as i32 {
        1 => {
            // center justify at x
            len = crate::stdlib::strlen(str) as i32;
            x = x - len * charw / 2 as i32
        }
        2 => {
            // right justify at x
            len = crate::stdlib::strlen(str) as i32;
            x = x - len * charw
        }
        _ => {}
    }
    if style & 0x800 as i32 != 0 {
        dropcolor[2 as i32 as usize] = 0 as i32 as vec_t;
        dropcolor[1 as i32 as usize] = dropcolor[2 as i32 as usize];
        dropcolor[0 as i32 as usize] = dropcolor[1 as i32 as usize];
        dropcolor[3 as i32 as usize] = *drawcolor.offset(3 as i32 as isize);
        UI_DrawString2(
            x + 2 as i32,
            y + 2 as i32,
            str,
            dropcolor.as_mut_ptr(),
            charw,
            charh,
        );
    }
    UI_DrawString2(x, y, str, drawcolor, charw, charh);
}
/*
=================
UI_DrawChar
=================
*/
#[no_mangle]

pub unsafe extern "C" fn UI_DrawChar(
    mut x: i32,
    mut y: i32,
    mut ch: i32,
    mut style: i32,
    mut color: *mut vec_t,
) {
    let mut buff: [libc::c_char; 2] = [0; 2];
    buff[0 as i32 as usize] = ch as libc::c_char;
    buff[1 as i32 as usize] = '\u{0}' as i32 as libc::c_char;
    UI_DrawString(x, y, buff.as_mut_ptr(), style, color);
}
#[no_mangle]

pub unsafe extern "C" fn UI_IsFullscreen() -> qboolean {
    if !uis.activemenu.is_null() && trap_Key_GetCatcher() & 0x2 as i32 != 0 {
        return (*uis.activemenu).fullscreen;
    }
    return qfalse;
}

unsafe extern "C" fn NeedCDAction(mut result: qboolean) {
    if result as u64 == 0 {
        trap_Cmd_ExecuteText(
            EXEC_APPEND as i32,
            b"quit\n\x00" as *const u8 as *const libc::c_char,
        );
    };
}

unsafe extern "C" fn NeedCDKeyAction(mut result: qboolean) {
    if result as u64 == 0 {
        trap_Cmd_ExecuteText(
            EXEC_APPEND as i32,
            b"quit\n\x00" as *const u8 as *const libc::c_char,
        );
    };
}
#[no_mangle]

pub unsafe extern "C" fn UI_SetActiveMenu(mut menu: uiMenuCommand_t) {
    // this should be the ONLY way the menu system is brought up
    // ensure minimum menu data is cached
    Menu_Cache();
    match menu as u32 {
        0 => {
            UI_ForceMenuOff();
            return;
        }
        1 => {
            UI_MainMenu();
            return;
        }
        3 => {
            UI_ConfirmMenu(
                b"Insert the CD\x00" as *const u8 as *const libc::c_char,
                None,
                Some(NeedCDAction as unsafe extern "C" fn(_: qboolean) -> ()),
            );
            return;
        }
        4 => {
            UI_ConfirmMenu(
                b"Bad CD Key\x00" as *const u8 as *const libc::c_char,
                None,
                Some(NeedCDKeyAction as unsafe extern "C" fn(_: qboolean) -> ()),
            );
            return;
        }
        2 => {
            /*
            //GRank
            UI_RankingsMenu();
            return;
            */
            trap_Cvar_Set(
                b"cl_paused\x00" as *const u8 as *const libc::c_char,
                b"1\x00" as *const u8 as *const libc::c_char,
            );
            UI_InGameMenu();
            return;
        }
        5 | 6 | _ => {}
    };
}
/*
=================
UI_KeyEvent
=================
*/
#[no_mangle]

pub unsafe extern "C" fn UI_KeyEvent(mut key: i32, mut down: i32) {
    let mut s: sfxHandle_t = 0;
    if uis.activemenu.is_null() {
        return;
    }
    if down == 0 {
        return;
    }
    if (*uis.activemenu).key.is_some() {
        s = (*uis.activemenu).key.expect("non-null function pointer")(key)
    } else {
        s = Menu_DefaultKey(uis.activemenu as *mut _tag_menuframework, key)
    }
    if s > 0 as i32 && s != menu_null_sound {
        trap_S_StartLocalSound(s, CHAN_LOCAL_SOUND as i32);
    };
}
/*
=================
UI_MouseEvent
=================
*/
#[no_mangle]

pub unsafe extern "C" fn UI_MouseEvent(mut dx: i32, mut dy: i32) {
    let mut i: i32 = 0;
    let mut bias: i32 = 0;
    let mut m: *mut menucommon_s = std::ptr::null_mut();
    if uis.activemenu.is_null() {
        return;
    }
    // convert X bias to 640 coords
    bias = (uis.bias / uis.xscale) as i32;
    // update mouse screen position
    uis.cursorx += dx;
    if uis.cursorx < -bias {
        uis.cursorx = -bias
    } else if uis.cursorx > 640 as i32 + bias {
        uis.cursorx = 640 as i32 + bias
    }
    uis.cursory += dy;
    if uis.cursory < 0 as i32 {
        uis.cursory = 0 as i32
    } else if uis.cursory > 480 as i32 {
        uis.cursory = 480 as i32
    }
    // region test the active menu items
    i = 0 as i32;
    while i < (*uis.activemenu).nitems {
        m = (*uis.activemenu).items[i as usize] as *mut menucommon_s;
        if !((*m).flags & (0x2000 as i32 as u32 | 0x4000 as i32 as u32) != 0) {
            if !(uis.cursorx < (*m).left
                || uis.cursorx > (*m).right
                || uis.cursory < (*m).top
                || uis.cursory > (*m).bottom)
            {
                // set focus to item at cursor
                if (*uis.activemenu).cursor != i {
                    Menu_SetCursor(uis.activemenu as *mut _tag_menuframework, i);
                    (*((*uis.activemenu).items[(*uis.activemenu).cursor_prev as usize]
                        as *mut menucommon_s))
                        .flags &= !(0x200 as i32 as u32);
                    if (*((*uis.activemenu).items[(*uis.activemenu).cursor as usize]
                        as *mut menucommon_s))
                        .flags
                        & 0x100000 as i32 as u32
                        == 0
                    {
                        trap_S_StartLocalSound(menu_move_sound, CHAN_LOCAL_SOUND as i32);
                    }
                }
                (*((*uis.activemenu).items[(*uis.activemenu).cursor as usize]
                    as *mut menucommon_s))
                    .flags |= 0x200 as i32 as u32;
                return;
            }
        }
        // cursor out of item bounds
        i += 1
    }
    if (*uis.activemenu).nitems > 0 as i32 {
        // out of any region
        (*((*uis.activemenu).items[(*uis.activemenu).cursor as usize] as *mut menucommon_s))
            .flags &= !(0x200 as i32 as u32)
    };
}
#[no_mangle]

pub unsafe extern "C" fn UI_Argv(mut arg: i32) -> *mut libc::c_char {
    static mut buffer: [libc::c_char; 1024] = [0; 1024];
    trap_Argv(
        arg,
        buffer.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as usize as i32,
    );
    return buffer.as_mut_ptr();
}
#[no_mangle]

pub unsafe extern "C" fn UI_Cvar_VariableString(
    mut var_name: *const libc::c_char,
) -> *mut libc::c_char {
    static mut buffer: [libc::c_char; 1024] = [0; 1024];
    trap_Cvar_VariableStringBuffer(
        var_name,
        buffer.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as usize as i32,
    );
    return buffer.as_mut_ptr();
}
/*
=================
UI_Cache
=================
*/
#[no_mangle]

pub unsafe extern "C" fn UI_Cache_f() {
    MainMenu_Cache();
    InGame_Cache();
    ConfirmMenu_Cache();
    PlayerModel_Cache();
    PlayerSettings_Cache();
    Controls_Cache();
    Demos_Cache();
    UI_CinematicsMenu_Cache();
    Preferences_Cache();
    ServerInfo_Cache();
    SpecifyServer_Cache();
    ArenaServers_Cache();
    StartServer_Cache();
    ServerOptions_Cache();
    DriverInfo_Cache();
    GraphicsOptions_Cache();
    UI_DisplayOptionsMenu_Cache();
    UI_SoundOptionsMenu_Cache();
    UI_NetworkOptionsMenu_Cache();
    UI_SPLevelMenu_Cache();
    UI_SPSkillMenu_Cache();
    UI_SPPostgameMenu_Cache();
    TeamMain_Cache();
    UI_AddBots_Cache();
    UI_RemoveBots_Cache();
    UI_SetupMenu_Cache();
    //	UI_LoadConfig_Cache();
    //	UI_SaveConfigMenu_Cache();
    UI_BotSelectMenu_Cache();
    UI_CDKeyMenu_Cache();
    UI_ModsMenu_Cache();
}
/*
=================
UI_ConsoleCommand
=================
*/
#[no_mangle]

pub unsafe extern "C" fn UI_ConsoleCommand(mut realTime: i32) -> qboolean {
    let mut cmd: *mut libc::c_char = std::ptr::null_mut();
    uis.frametime = realTime - uis.realtime;
    uis.realtime = realTime;
    cmd = UI_Argv(0 as i32);
    // ensure minimum menu data is available
    Menu_Cache();
    if Q_stricmp(cmd, b"levelselect\x00" as *const u8 as *const libc::c_char) == 0 as i32 {
        UI_SPLevelMenu_f();
        return qtrue;
    }
    if Q_stricmp(cmd, b"postgame\x00" as *const u8 as *const libc::c_char) == 0 as i32 {
        UI_SPPostgameMenu_f();
        return qtrue;
    }
    if Q_stricmp(cmd, b"ui_cache\x00" as *const u8 as *const libc::c_char) == 0 as i32 {
        UI_Cache_f();
        return qtrue;
    }
    if Q_stricmp(
        cmd,
        b"ui_cinematics\x00" as *const u8 as *const libc::c_char,
    ) == 0 as i32
    {
        UI_CinematicsMenu_f();
        return qtrue;
    }
    if Q_stricmp(
        cmd,
        b"ui_teamOrders\x00" as *const u8 as *const libc::c_char,
    ) == 0 as i32
    {
        UI_TeamOrdersMenu_f();
        return qtrue;
    }
    if Q_stricmp(cmd, b"iamacheater\x00" as *const u8 as *const libc::c_char) == 0 as i32 {
        UI_SPUnlock_f();
        return qtrue;
    }
    if Q_stricmp(cmd, b"iamamonkey\x00" as *const u8 as *const libc::c_char) == 0 as i32 {
        UI_SPUnlockMedals_f();
        return qtrue;
    }
    if Q_stricmp(cmd, b"ui_cdkey\x00" as *const u8 as *const libc::c_char) == 0 as i32 {
        UI_CDKeyMenu_f();
        return qtrue;
    }
    return qfalse;
}
/*
=================
UI_Shutdown
=================
*/
#[no_mangle]

pub unsafe extern "C" fn UI_Shutdown() {}
/*
=================
UI_Init
=================
*/
#[no_mangle]

pub unsafe extern "C" fn UI_Init() {
    UI_RegisterCvars();
    UI_InitGameinfo();
    // cache redundant calulations
    trap_GetGlconfig(&mut uis.glconfig as *mut _ as *mut glconfig_t);
    // for 640x480 virtualized screen
    uis.xscale = (uis.glconfig.vidWidth as f64 * (1.0f64 / 640.0f64)) as f32;
    uis.yscale = (uis.glconfig.vidHeight as f64 * (1.0f64 / 480.0f64)) as f32;
    if uis.glconfig.vidWidth * 480 as i32 > uis.glconfig.vidHeight * 640 as i32 {
        // wide screen
        uis.bias = (0.5f64
            * (uis.glconfig.vidWidth as f64
                - uis.glconfig.vidHeight as f64 * (640.0f64 / 480.0f64))) as f32;
        uis.xscale = uis.yscale
    } else {
        // no wide screen
        uis.bias = 0 as i32 as f32
    }
    // initialize the menu system
    Menu_Cache();
    uis.activemenu = std::ptr::null_mut();
    uis.menusp = 0 as i32;
}
/*
================
UI_AdjustFrom640

Adjusted for resolution and screen aspect ratio
================
*/
#[no_mangle]

pub unsafe extern "C" fn UI_AdjustFrom640(
    mut x: *mut f32,
    mut y: *mut f32,
    mut w: *mut f32,
    mut h: *mut f32,
) {
    // expect valid pointers
    *x = *x * uis.xscale + uis.bias;
    *y *= uis.yscale;
    *w *= uis.xscale;
    *h *= uis.yscale;
}
#[no_mangle]

pub unsafe extern "C" fn UI_DrawNamedPic(
    mut x: f32,
    mut y: f32,
    mut width: f32,
    mut height: f32,
    mut picname: *const libc::c_char,
) {
    let mut hShader: qhandle_t = 0;
    hShader = trap_R_RegisterShaderNoMip(picname);
    UI_AdjustFrom640(&mut x, &mut y, &mut width, &mut height);
    trap_R_DrawStretchPic(
        x,
        y,
        width,
        height,
        0 as i32 as f32,
        0 as i32 as f32,
        1 as i32 as f32,
        1 as i32 as f32,
        hShader,
    );
}
#[no_mangle]

pub unsafe extern "C" fn UI_DrawHandlePic(
    mut x: f32,
    mut y: f32,
    mut w: f32,
    mut h: f32,
    mut hShader: qhandle_t,
) {
    let mut s0: f32 = 0.;
    let mut s1: f32 = 0.;
    let mut t0: f32 = 0.;
    let mut t1: f32 = 0.;
    if w < 0 as i32 as f32 {
        // flip about vertical
        w = -w;
        s0 = 1 as i32 as f32;
        s1 = 0 as i32 as f32
    } else {
        s0 = 0 as i32 as f32;
        s1 = 1 as i32 as f32
    }
    if h < 0 as i32 as f32 {
        // flip about horizontal
        h = -h;
        t0 = 1 as i32 as f32;
        t1 = 0 as i32 as f32
    } else {
        t0 = 0 as i32 as f32;
        t1 = 1 as i32 as f32
    }
    UI_AdjustFrom640(&mut x, &mut y, &mut w, &mut h);
    trap_R_DrawStretchPic(x, y, w, h, s0, t0, s1, t1, hShader);
}
/*
================
UI_FillRect

Coordinates are 640*480 virtual values
=================
*/
#[no_mangle]

pub unsafe extern "C" fn UI_FillRect(
    mut x: f32,
    mut y: f32,
    mut width: f32,
    mut height: f32,
    mut color: *const f32,
) {
    trap_R_SetColor(color);
    UI_AdjustFrom640(&mut x, &mut y, &mut width, &mut height);
    trap_R_DrawStretchPic(
        x,
        y,
        width,
        height,
        0 as i32 as f32,
        0 as i32 as f32,
        0 as i32 as f32,
        0 as i32 as f32,
        uis.whiteShader,
    );
    trap_R_SetColor(std::ptr::null());
}
/*
================
UI_DrawRect

Coordinates are 640*480 virtual values
=================
*/
#[no_mangle]

pub unsafe extern "C" fn UI_DrawRect(
    mut x: f32,
    mut y: f32,
    mut width: f32,
    mut height: f32,
    mut color: *const f32,
) {
    trap_R_SetColor(color);
    UI_AdjustFrom640(&mut x, &mut y, &mut width, &mut height);
    trap_R_DrawStretchPic(
        x,
        y,
        width,
        1 as i32 as f32,
        0 as i32 as f32,
        0 as i32 as f32,
        0 as i32 as f32,
        0 as i32 as f32,
        uis.whiteShader,
    );
    trap_R_DrawStretchPic(
        x,
        y,
        1 as i32 as f32,
        height,
        0 as i32 as f32,
        0 as i32 as f32,
        0 as i32 as f32,
        0 as i32 as f32,
        uis.whiteShader,
    );
    trap_R_DrawStretchPic(
        x,
        y + height - 1 as i32 as f32,
        width,
        1 as i32 as f32,
        0 as i32 as f32,
        0 as i32 as f32,
        0 as i32 as f32,
        0 as i32 as f32,
        uis.whiteShader,
    );
    trap_R_DrawStretchPic(
        x + width - 1 as i32 as f32,
        y,
        1 as i32 as f32,
        height,
        0 as i32 as f32,
        0 as i32 as f32,
        0 as i32 as f32,
        0 as i32 as f32,
        uis.whiteShader,
    );
    trap_R_SetColor(std::ptr::null());
}
#[no_mangle]

pub unsafe extern "C" fn UI_SetColor(mut rgba: *const f32) {
    trap_R_SetColor(rgba);
}
#[no_mangle]

pub unsafe extern "C" fn UI_UpdateScreen() {
    trap_UpdateScreen();
}
/*
=================
UI_Refresh
=================
*/
#[no_mangle]

pub unsafe extern "C" fn UI_Refresh(mut realtime: i32) {
    uis.frametime = realtime - uis.realtime;
    uis.realtime = realtime;
    if trap_Key_GetCatcher() & 0x2 as i32 == 0 {
        return;
    }
    UI_UpdateCvars();
    if !uis.activemenu.is_null() {
        if (*uis.activemenu).fullscreen as u64 != 0 {
            // draw the background
            if (*uis.activemenu).showlogo as u64 != 0 {
                UI_DrawHandlePic(
                    0 as i32 as f32,
                    0 as i32 as f32,
                    640 as i32 as f32,
                    480 as i32 as f32,
                    uis.menuBackShader,
                );
            } else {
                UI_DrawHandlePic(
                    0 as i32 as f32,
                    0 as i32 as f32,
                    640 as i32 as f32,
                    480 as i32 as f32,
                    uis.menuBackNoLogoShader,
                );
            }
        }
        if (*uis.activemenu).draw.is_some() {
            (*uis.activemenu).draw.expect("non-null function pointer")();
        } else {
            Menu_Draw(uis.activemenu as *mut _tag_menuframework);
        }
        if uis.firstdraw as u64 != 0 {
            UI_MouseEvent(0 as i32, 0 as i32);
            uis.firstdraw = qfalse
        }
    }
    // draw cursor
    UI_SetColor(std::ptr::null());
    UI_DrawHandlePic(
        (uis.cursorx - 16 as i32) as f32,
        (uis.cursory - 16 as i32) as f32,
        32 as i32 as f32,
        32 as i32 as f32,
        uis.cursor,
    );
    // delay playing the enter sound until after the
    // menu has been drawn, to avoid delay while
    // caching images
    if m_entersound as u64 != 0 {
        trap_S_StartLocalSound(menu_in_sound, CHAN_LOCAL_SOUND as i32);
        m_entersound = qfalse
    };
}
#[no_mangle]

pub unsafe extern "C" fn UI_DrawTextBox(mut x: i32, mut y: i32, mut width: i32, mut lines: i32) {
    UI_FillRect(
        (x + 16 as i32 / 2 as i32) as f32,
        (y + 16 as i32 / 2 as i32) as f32,
        ((width + 1 as i32) * 16 as i32) as f32,
        ((lines + 1 as i32) * 16 as i32) as f32,
        colorBlack.as_mut_ptr(),
    );
    UI_DrawRect(
        (x + 16 as i32 / 2 as i32) as f32,
        (y + 16 as i32 / 2 as i32) as f32,
        ((width + 1 as i32) * 16 as i32) as f32,
        ((lines + 1 as i32) * 16 as i32) as f32,
        colorWhite.as_mut_ptr(),
    );
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
//NOTE: include the ui_public.h from the new UI
//redefine to old API version
//
// ui_qmenu.c
//
// edit field is only numbers
// steady focus
// pulse if focus
// only mouse input allowed
// skips drawing
// grays and disables
// disables any input
// skip default initialization
// edit field is all lower case
// edit field is all upper case
// callback notifications
//
// ui_mfield.c
//
//
// ui_menu.c
//
//
// ui_credits.c
//
//
// ui_ingame.c
//
//
// ui_confirm.c
//
//
// ui_setup.c
//
//
// ui_team.c
//
//
// ui_connect.c
//
//
// ui_controls2.c
//
//
// ui_demo2.c
//
//
// ui_cinematics.c
//
//
// ui_mods.c
//
//
// ui_cdkey.c
//
//
// ui_playermodel.c
//
//
// ui_playersettings.c
//
//
// ui_preferences.c
//
//
// ui_specifyleague.c
//
//
// ui_specifyserver.c
//
//
// ui_servers2.c
//
//
// ui_startserver.c
//
//
// ui_serverinfo.c
//
//
// ui_video.c
//
//
// ui_players.c
//
//FIXME ripped from cg_local.h
// time when ->oldFrame was exactly on
// time when ->frame will be exactly on
// may include ANIM_TOGGLEBIT
// time when the first frame of the animation will be exact
// model info
// true if legs yaw is always the same as torso yaw
// true if torso never changes yaw
// currently in use drawing parms
// animation vars
//
// ui_atoms.c
//
#[no_mangle]

pub unsafe extern "C" fn UI_CursorInRect(
    mut x: i32,
    mut y: i32,
    mut width: i32,
    mut height: i32,
) -> qboolean {
    if uis.cursorx < x || uis.cursory < y || uis.cursorx > x + width || uis.cursory > y + height {
        return qfalse;
    }
    return qtrue;
}

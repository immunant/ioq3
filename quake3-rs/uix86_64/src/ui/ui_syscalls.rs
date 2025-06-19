use ::libc;

pub use crate::stdlib::intptr_t;

pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::clipHandle_t;
pub use crate::src::qcommon::q_shared::connstate_t;
pub use crate::src::qcommon::q_shared::cvarHandle_t;
pub use crate::src::qcommon::q_shared::e_status;
pub use crate::src::qcommon::q_shared::fileHandle_t;
pub use crate::src::qcommon::q_shared::floatint_t;
pub use crate::src::qcommon::q_shared::fontInfo_t;
pub use crate::src::qcommon::q_shared::fsMode_t;
pub use crate::src::qcommon::q_shared::glyphInfo_t;
pub use crate::src::qcommon::q_shared::orientation_t;
pub use crate::src::qcommon::q_shared::pc_token_s;
pub use crate::src::qcommon::q_shared::pc_token_t;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qhandle_t;
pub use crate::src::qcommon::q_shared::qtime_s;
pub use crate::src::qcommon::q_shared::qtime_t;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::sfxHandle_t;
pub use crate::src::qcommon::q_shared::vec3_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::qcommon::q_shared::vmCvar_t;
pub use crate::src::qcommon::q_shared::C2RustUnnamed_0;
pub use crate::src::qcommon::q_shared::CA_ACTIVE;
pub use crate::src::qcommon::q_shared::CA_AUTHORIZING;
pub use crate::src::qcommon::q_shared::CA_CHALLENGING;
pub use crate::src::qcommon::q_shared::CA_CINEMATIC;
pub use crate::src::qcommon::q_shared::CA_CONNECTED;
pub use crate::src::qcommon::q_shared::CA_CONNECTING;
pub use crate::src::qcommon::q_shared::CA_DISCONNECTED;
pub use crate::src::qcommon::q_shared::CA_LOADING;
pub use crate::src::qcommon::q_shared::CA_PRIMED;
pub use crate::src::qcommon::q_shared::CA_UNINITIALIZED;
pub use crate::src::qcommon::q_shared::FMV_EOF;
pub use crate::src::qcommon::q_shared::FMV_IDLE;
pub use crate::src::qcommon::q_shared::FMV_ID_BLT;
pub use crate::src::qcommon::q_shared::FMV_ID_IDLE;
pub use crate::src::qcommon::q_shared::FMV_ID_WAIT;
pub use crate::src::qcommon::q_shared::FMV_LOOPED;
pub use crate::src::qcommon::q_shared::FMV_PLAY;
pub use crate::src::qcommon::q_shared::FS_APPEND;
pub use crate::src::qcommon::q_shared::FS_APPEND_SYNC;
pub use crate::src::qcommon::q_shared::FS_READ;
pub use crate::src::qcommon::q_shared::FS_WRITE;
pub use crate::tr_types_h::glDriverType_t;
pub use crate::tr_types_h::glHardwareType_t;
pub use crate::tr_types_h::glconfig_t;
pub use crate::tr_types_h::polyVert_t;
pub use crate::tr_types_h::refEntityType_t;
pub use crate::tr_types_h::refEntity_t;
pub use crate::tr_types_h::refdef_t;
pub use crate::tr_types_h::textureCompression_t;
pub use crate::tr_types_h::GLDRV_ICD;
pub use crate::tr_types_h::GLDRV_STANDALONE;
pub use crate::tr_types_h::GLDRV_VOODOO;
pub use crate::tr_types_h::GLHW_3DFX_2D3D;
pub use crate::tr_types_h::GLHW_GENERIC;
pub use crate::tr_types_h::GLHW_PERMEDIA2;
pub use crate::tr_types_h::GLHW_RAGEPRO;
pub use crate::tr_types_h::GLHW_RIVA128;
pub use crate::tr_types_h::RT_BEAM;
pub use crate::tr_types_h::RT_LIGHTNING;
pub use crate::tr_types_h::RT_MAX_REF_ENTITY_TYPE;
pub use crate::tr_types_h::RT_MODEL;
pub use crate::tr_types_h::RT_POLY;
pub use crate::tr_types_h::RT_PORTALSURFACE;
pub use crate::tr_types_h::RT_RAIL_CORE;
pub use crate::tr_types_h::RT_RAIL_RINGS;
pub use crate::tr_types_h::RT_SPRITE;
pub use crate::tr_types_h::TC_NONE;
pub use crate::tr_types_h::TC_S3TC;
pub use crate::tr_types_h::TC_S3TC_ARB;
pub use crate::ui_public_h::uiClientState_t;
pub use crate::ui_public_h::UI_ARGC;
pub use crate::ui_public_h::UI_ARGV;
pub use crate::ui_public_h::UI_ATAN2;
pub use crate::ui_public_h::UI_CEIL;
pub use crate::ui_public_h::UI_CIN_DRAWCINEMATIC;
pub use crate::ui_public_h::UI_CIN_PLAYCINEMATIC;
pub use crate::ui_public_h::UI_CIN_RUNCINEMATIC;
pub use crate::ui_public_h::UI_CIN_SETEXTENTS;
pub use crate::ui_public_h::UI_CIN_STOPCINEMATIC;
pub use crate::ui_public_h::UI_CMD_EXECUTETEXT;
pub use crate::ui_public_h::UI_CM_LERPTAG;
pub use crate::ui_public_h::UI_CM_LOADMODEL;
pub use crate::ui_public_h::UI_COS;
pub use crate::ui_public_h::UI_CVAR_CREATE;
pub use crate::ui_public_h::UI_CVAR_INFOSTRINGBUFFER;
pub use crate::ui_public_h::UI_CVAR_REGISTER;
pub use crate::ui_public_h::UI_CVAR_RESET;
pub use crate::ui_public_h::UI_CVAR_SET;
pub use crate::ui_public_h::UI_CVAR_SETVALUE;
pub use crate::ui_public_h::UI_CVAR_UPDATE;
pub use crate::ui_public_h::UI_CVAR_VARIABLESTRINGBUFFER;
pub use crate::ui_public_h::UI_CVAR_VARIABLEVALUE;
pub use crate::ui_public_h::UI_ERROR;
pub use crate::ui_public_h::UI_FLOOR;
pub use crate::ui_public_h::UI_FS_FCLOSEFILE;
pub use crate::ui_public_h::UI_FS_FOPENFILE;
pub use crate::ui_public_h::UI_FS_GETFILELIST;
pub use crate::ui_public_h::UI_FS_READ;
pub use crate::ui_public_h::UI_FS_SEEK;
pub use crate::ui_public_h::UI_FS_WRITE;
pub use crate::ui_public_h::UI_GETCLIENTSTATE;
pub use crate::ui_public_h::UI_GETCLIPBOARDDATA;
pub use crate::ui_public_h::UI_GETCONFIGSTRING;
pub use crate::ui_public_h::UI_GETGLCONFIG;
pub use crate::ui_public_h::UI_GET_CDKEY;
pub use crate::ui_public_h::UI_KEY_CLEARSTATES;
pub use crate::ui_public_h::UI_KEY_GETBINDINGBUF;
pub use crate::ui_public_h::UI_KEY_GETCATCHER;
pub use crate::ui_public_h::UI_KEY_GETOVERSTRIKEMODE;
pub use crate::ui_public_h::UI_KEY_ISDOWN;
pub use crate::ui_public_h::UI_KEY_KEYNUMTOSTRINGBUF;
pub use crate::ui_public_h::UI_KEY_SETBINDING;
pub use crate::ui_public_h::UI_KEY_SETCATCHER;
pub use crate::ui_public_h::UI_KEY_SETOVERSTRIKEMODE;
pub use crate::ui_public_h::UI_LAN_ADDSERVER;
pub use crate::ui_public_h::UI_LAN_CLEARPING;
pub use crate::ui_public_h::UI_LAN_COMPARESERVERS;
pub use crate::ui_public_h::UI_LAN_GETPING;
pub use crate::ui_public_h::UI_LAN_GETPINGINFO;
pub use crate::ui_public_h::UI_LAN_GETPINGQUEUECOUNT;
pub use crate::ui_public_h::UI_LAN_GETSERVERADDRESSSTRING;
pub use crate::ui_public_h::UI_LAN_GETSERVERCOUNT;
pub use crate::ui_public_h::UI_LAN_GETSERVERINFO;
pub use crate::ui_public_h::UI_LAN_GETSERVERPING;
pub use crate::ui_public_h::UI_LAN_LOADCACHEDSERVERS;
pub use crate::ui_public_h::UI_LAN_MARKSERVERVISIBLE;
pub use crate::ui_public_h::UI_LAN_REMOVESERVER;
pub use crate::ui_public_h::UI_LAN_RESETPINGS;
pub use crate::ui_public_h::UI_LAN_SAVECACHEDSERVERS;
pub use crate::ui_public_h::UI_LAN_SERVERISVISIBLE;
pub use crate::ui_public_h::UI_LAN_SERVERSTATUS;
pub use crate::ui_public_h::UI_LAN_UPDATEVISIBLEPINGS;
pub use crate::ui_public_h::UI_MEMCPY;
pub use crate::ui_public_h::UI_MEMORY_REMAINING;
pub use crate::ui_public_h::UI_MEMSET;
pub use crate::ui_public_h::UI_MILLISECONDS;
pub use crate::ui_public_h::UI_PC_ADD_GLOBAL_DEFINE;
pub use crate::ui_public_h::UI_PC_FREE_SOURCE;
pub use crate::ui_public_h::UI_PC_LOAD_SOURCE;
pub use crate::ui_public_h::UI_PC_READ_TOKEN;
pub use crate::ui_public_h::UI_PC_SOURCE_FILE_AND_LINE;
pub use crate::ui_public_h::UI_PRINT;
pub use crate::ui_public_h::UI_REAL_TIME;
pub use crate::ui_public_h::UI_R_ADDLIGHTTOSCENE;
pub use crate::ui_public_h::UI_R_ADDPOLYTOSCENE;
pub use crate::ui_public_h::UI_R_ADDREFENTITYTOSCENE;
pub use crate::ui_public_h::UI_R_CLEARSCENE;
pub use crate::ui_public_h::UI_R_DRAWSTRETCHPIC;
pub use crate::ui_public_h::UI_R_MODELBOUNDS;
pub use crate::ui_public_h::UI_R_REGISTERFONT;
pub use crate::ui_public_h::UI_R_REGISTERMODEL;
pub use crate::ui_public_h::UI_R_REGISTERSHADERNOMIP;
pub use crate::ui_public_h::UI_R_REGISTERSKIN;
pub use crate::ui_public_h::UI_R_REMAP_SHADER;
pub use crate::ui_public_h::UI_R_RENDERSCENE;
pub use crate::ui_public_h::UI_R_SETCOLOR;
pub use crate::ui_public_h::UI_SET_CDKEY;
pub use crate::ui_public_h::UI_SET_PBCLSTATUS;
pub use crate::ui_public_h::UI_SIN;
pub use crate::ui_public_h::UI_SQRT;
pub use crate::ui_public_h::UI_STRNCPY;
pub use crate::ui_public_h::UI_S_REGISTERSOUND;
pub use crate::ui_public_h::UI_S_STARTBACKGROUNDTRACK;
pub use crate::ui_public_h::UI_S_STARTLOCALSOUND;
pub use crate::ui_public_h::UI_S_STOPBACKGROUNDTRACK;
pub use crate::ui_public_h::UI_UPDATESCREEN;
pub use crate::ui_public_h::UI_VERIFY_CDKEY;

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
// syscalls.asm is included instead when building a qvm
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

pub unsafe extern "C" fn trap_Print(mut string: *const libc::c_char) {
    syscall.expect("non-null function pointer")(
        crate::ui_public_h::UI_PRINT as i32 as crate::stdlib::intptr_t,
        string,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_Error(mut string: *const libc::c_char) -> ! {
    syscall.expect("non-null function pointer")(
        crate::ui_public_h::UI_ERROR as i32 as crate::stdlib::intptr_t,
        string,
    );
    // shut up GCC warning about returning functions, because we know better
    ::libc::exit(1 as i32);
}
#[no_mangle]

pub unsafe extern "C" fn trap_Milliseconds() -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::ui_public_h::UI_MILLISECONDS as i32 as crate::stdlib::intptr_t,
    ) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn trap_Cvar_Register(
    mut cvar: *mut crate::src::qcommon::q_shared::vmCvar_t,
    mut var_name: *const libc::c_char,
    mut value: *const libc::c_char,
    mut flags: i32,
) {
    syscall.expect("non-null function pointer")(
        crate::ui_public_h::UI_CVAR_REGISTER as i32 as crate::stdlib::intptr_t,
        cvar,
        var_name,
        value,
        flags,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_Cvar_Update(mut cvar: *mut crate::src::qcommon::q_shared::vmCvar_t) {
    syscall.expect("non-null function pointer")(
        crate::ui_public_h::UI_CVAR_UPDATE as i32 as crate::stdlib::intptr_t,
        cvar,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_Cvar_Set(
    mut var_name: *const libc::c_char,
    mut value: *const libc::c_char,
) {
    syscall.expect("non-null function pointer")(
        crate::ui_public_h::UI_CVAR_SET as i32 as crate::stdlib::intptr_t,
        var_name,
        value,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_Cvar_VariableValue(mut var_name: *const libc::c_char) -> f32 {
    let mut fi: crate::src::qcommon::q_shared::floatint_t =
        crate::src::qcommon::q_shared::floatint_t { f: 0. };
    fi.i = syscall.expect("non-null function pointer")(
        crate::ui_public_h::UI_CVAR_VARIABLEVALUE as i32 as crate::stdlib::intptr_t,
        var_name,
    ) as i32;
    return fi.f;
}
#[no_mangle]

pub unsafe extern "C" fn trap_Cvar_VariableStringBuffer(
    mut var_name: *const libc::c_char,
    mut buffer: *mut libc::c_char,
    mut bufsize: i32,
) {
    syscall.expect("non-null function pointer")(
        crate::ui_public_h::UI_CVAR_VARIABLESTRINGBUFFER as i32 as crate::stdlib::intptr_t,
        var_name,
        buffer,
        bufsize,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_Cvar_SetValue(mut var_name: *const libc::c_char, mut value: f32) {
    syscall.expect("non-null function pointer")(
        crate::ui_public_h::UI_CVAR_SETVALUE as i32 as crate::stdlib::intptr_t,
        var_name,
        PASSFLOAT(value),
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_Cvar_Reset(mut name: *const libc::c_char) {
    syscall.expect("non-null function pointer")(
        crate::ui_public_h::UI_CVAR_RESET as i32 as crate::stdlib::intptr_t,
        name,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_Cvar_Create(
    mut var_name: *const libc::c_char,
    mut var_value: *const libc::c_char,
    mut flags: i32,
) {
    syscall.expect("non-null function pointer")(
        crate::ui_public_h::UI_CVAR_CREATE as i32 as crate::stdlib::intptr_t,
        var_name,
        var_value,
        flags,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_Cvar_InfoStringBuffer(
    mut bit: i32,
    mut buffer: *mut libc::c_char,
    mut bufsize: i32,
) {
    syscall.expect("non-null function pointer")(
        crate::ui_public_h::UI_CVAR_INFOSTRINGBUFFER as i32 as crate::stdlib::intptr_t,
        bit,
        buffer,
        bufsize,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_Argc() -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::ui_public_h::UI_ARGC as i32 as crate::stdlib::intptr_t,
    ) as i32;
}
// for the showing the status of a server
// to retrieve the status of server to find a player
//
// ui_spLevel.c
//
//
// ui_spArena.c
//
//
// ui_spPostgame.c
//
//
// ui_spSkill.c
//
//
// ui_syscalls.c
//
#[no_mangle]

pub unsafe extern "C" fn trap_Argv(
    mut n: i32,
    mut buffer: *mut libc::c_char,
    mut bufferLength: i32,
) {
    syscall.expect("non-null function pointer")(
        crate::ui_public_h::UI_ARGV as i32 as crate::stdlib::intptr_t,
        n,
        buffer,
        bufferLength,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_Cmd_ExecuteText(mut exec_when: i32, mut text: *const libc::c_char) {
    syscall.expect("non-null function pointer")(
        crate::ui_public_h::UI_CMD_EXECUTETEXT as i32 as crate::stdlib::intptr_t,
        exec_when,
        text,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_FS_FOpenFile(
    mut qpath: *const libc::c_char,
    mut f: *mut crate::src::qcommon::q_shared::fileHandle_t,
    mut mode: crate::src::qcommon::q_shared::fsMode_t,
) -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::ui_public_h::UI_FS_FOPENFILE as i32 as crate::stdlib::intptr_t,
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
        crate::ui_public_h::UI_FS_READ as i32 as crate::stdlib::intptr_t,
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
        crate::ui_public_h::UI_FS_WRITE as i32 as crate::stdlib::intptr_t,
        buffer,
        len,
        f,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_FS_FCloseFile(mut f: crate::src::qcommon::q_shared::fileHandle_t) {
    syscall.expect("non-null function pointer")(
        crate::ui_public_h::UI_FS_FCLOSEFILE as i32 as crate::stdlib::intptr_t,
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
        crate::ui_public_h::UI_FS_GETFILELIST as i32 as crate::stdlib::intptr_t,
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
        crate::ui_public_h::UI_FS_SEEK as i32 as crate::stdlib::intptr_t,
        f,
        offset,
        origin,
    ) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn trap_R_RegisterModel(
    mut name: *const libc::c_char,
) -> crate::src::qcommon::q_shared::qhandle_t {
    return syscall.expect("non-null function pointer")(
        crate::ui_public_h::UI_R_REGISTERMODEL as i32 as crate::stdlib::intptr_t,
        name,
    ) as crate::src::qcommon::q_shared::qhandle_t;
}
#[no_mangle]

pub unsafe extern "C" fn trap_R_RegisterSkin(
    mut name: *const libc::c_char,
) -> crate::src::qcommon::q_shared::qhandle_t {
    return syscall.expect("non-null function pointer")(
        crate::ui_public_h::UI_R_REGISTERSKIN as i32 as crate::stdlib::intptr_t,
        name,
    ) as crate::src::qcommon::q_shared::qhandle_t;
}
#[no_mangle]

pub unsafe extern "C" fn trap_R_RegisterFont(
    mut fontName: *const libc::c_char,
    mut pointSize: i32,
    mut font: *mut crate::src::qcommon::q_shared::fontInfo_t,
) {
    syscall.expect("non-null function pointer")(
        crate::ui_public_h::UI_R_REGISTERFONT as i32 as crate::stdlib::intptr_t,
        fontName,
        pointSize,
        font,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_R_RegisterShaderNoMip(
    mut name: *const libc::c_char,
) -> crate::src::qcommon::q_shared::qhandle_t {
    return syscall.expect("non-null function pointer")(
        crate::ui_public_h::UI_R_REGISTERSHADERNOMIP as i32 as crate::stdlib::intptr_t,
        name,
    ) as crate::src::qcommon::q_shared::qhandle_t;
}
#[no_mangle]

pub unsafe extern "C" fn trap_R_ClearScene() {
    syscall.expect("non-null function pointer")(
        crate::ui_public_h::UI_R_CLEARSCENE as i32 as crate::stdlib::intptr_t,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_R_AddRefEntityToScene(mut re: *const crate::tr_types_h::refEntity_t) {
    syscall.expect("non-null function pointer")(
        crate::ui_public_h::UI_R_ADDREFENTITYTOSCENE as i32 as crate::stdlib::intptr_t,
        re,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_R_AddPolyToScene(
    mut hShader: crate::src::qcommon::q_shared::qhandle_t,
    mut numVerts: i32,
    mut verts: *const crate::tr_types_h::polyVert_t,
) {
    syscall.expect("non-null function pointer")(
        crate::ui_public_h::UI_R_ADDPOLYTOSCENE as i32 as crate::stdlib::intptr_t,
        hShader,
        numVerts,
        verts,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_R_AddLightToScene(
    mut org: *const crate::src::qcommon::q_shared::vec_t,
    mut intensity: f32,
    mut r: f32,
    mut g: f32,
    mut b: f32,
) {
    syscall.expect("non-null function pointer")(
        crate::ui_public_h::UI_R_ADDLIGHTTOSCENE as i32 as crate::stdlib::intptr_t,
        org,
        PASSFLOAT(intensity),
        PASSFLOAT(r),
        PASSFLOAT(g),
        PASSFLOAT(b),
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_R_RenderScene(mut fd: *const crate::tr_types_h::refdef_t) {
    syscall.expect("non-null function pointer")(
        crate::ui_public_h::UI_R_RENDERSCENE as i32 as crate::stdlib::intptr_t,
        fd,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_R_SetColor(mut rgba: *const f32) {
    syscall.expect("non-null function pointer")(
        crate::ui_public_h::UI_R_SETCOLOR as i32 as crate::stdlib::intptr_t,
        rgba,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_R_DrawStretchPic(
    mut x: f32,
    mut y: f32,
    mut w: f32,
    mut h: f32,
    mut s1: f32,
    mut t1: f32,
    mut s2: f32,
    mut t2: f32,
    mut hShader: crate::src::qcommon::q_shared::qhandle_t,
) {
    syscall.expect("non-null function pointer")(
        crate::ui_public_h::UI_R_DRAWSTRETCHPIC as i32 as crate::stdlib::intptr_t,
        PASSFLOAT(x),
        PASSFLOAT(y),
        PASSFLOAT(w),
        PASSFLOAT(h),
        PASSFLOAT(s1),
        PASSFLOAT(t1),
        PASSFLOAT(s2),
        PASSFLOAT(t2),
        hShader,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_R_ModelBounds(
    mut model: crate::src::qcommon::q_shared::clipHandle_t,
    mut mins: *mut crate::src::qcommon::q_shared::vec_t,
    mut maxs: *mut crate::src::qcommon::q_shared::vec_t,
) {
    syscall.expect("non-null function pointer")(
        crate::ui_public_h::UI_R_MODELBOUNDS as i32 as crate::stdlib::intptr_t,
        model,
        mins,
        maxs,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_UpdateScreen() {
    syscall.expect("non-null function pointer")(
        crate::ui_public_h::UI_UPDATESCREEN as i32 as crate::stdlib::intptr_t,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_CM_LerpTag(
    mut tag: *mut crate::src::qcommon::q_shared::orientation_t,
    mut mod_0: crate::src::qcommon::q_shared::clipHandle_t,
    mut startFrame: i32,
    mut endFrame: i32,
    mut frac: f32,
    mut tagName: *const libc::c_char,
) -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::ui_public_h::UI_CM_LERPTAG as i32 as crate::stdlib::intptr_t,
        tag,
        mod_0,
        startFrame,
        endFrame,
        PASSFLOAT(frac),
        tagName,
    ) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn trap_S_StartLocalSound(
    mut sfx: crate::src::qcommon::q_shared::sfxHandle_t,
    mut channelNum: i32,
) {
    syscall.expect("non-null function pointer")(
        crate::ui_public_h::UI_S_STARTLOCALSOUND as i32 as crate::stdlib::intptr_t,
        sfx,
        channelNum,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_S_RegisterSound(
    mut sample: *const libc::c_char,
    mut compressed: crate::src::qcommon::q_shared::qboolean,
) -> crate::src::qcommon::q_shared::sfxHandle_t {
    return syscall.expect("non-null function pointer")(
        crate::ui_public_h::UI_S_REGISTERSOUND as i32 as crate::stdlib::intptr_t,
        sample,
        compressed as u32,
    ) as crate::src::qcommon::q_shared::sfxHandle_t;
}
#[no_mangle]

pub unsafe extern "C" fn trap_Key_KeynumToStringBuf(
    mut keynum: i32,
    mut buf: *mut libc::c_char,
    mut buflen: i32,
) {
    syscall.expect("non-null function pointer")(
        crate::ui_public_h::UI_KEY_KEYNUMTOSTRINGBUF as i32 as crate::stdlib::intptr_t,
        keynum,
        buf,
        buflen,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_Key_GetBindingBuf(
    mut keynum: i32,
    mut buf: *mut libc::c_char,
    mut buflen: i32,
) {
    syscall.expect("non-null function pointer")(
        crate::ui_public_h::UI_KEY_GETBINDINGBUF as i32 as crate::stdlib::intptr_t,
        keynum,
        buf,
        buflen,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_Key_SetBinding(mut keynum: i32, mut binding: *const libc::c_char) {
    syscall.expect("non-null function pointer")(
        crate::ui_public_h::UI_KEY_SETBINDING as i32 as crate::stdlib::intptr_t,
        keynum,
        binding,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_Key_IsDown(
    mut keynum: i32,
) -> crate::src::qcommon::q_shared::qboolean {
    return syscall.expect("non-null function pointer")(
        crate::ui_public_h::UI_KEY_ISDOWN as i32 as crate::stdlib::intptr_t,
        keynum,
    ) as crate::src::qcommon::q_shared::qboolean;
}
#[no_mangle]

pub unsafe extern "C" fn trap_Key_GetOverstrikeMode() -> crate::src::qcommon::q_shared::qboolean {
    return syscall.expect("non-null function pointer")(
        crate::ui_public_h::UI_KEY_GETOVERSTRIKEMODE as i32 as crate::stdlib::intptr_t,
    ) as crate::src::qcommon::q_shared::qboolean;
}
#[no_mangle]

pub unsafe extern "C" fn trap_Key_SetOverstrikeMode(
    mut state: crate::src::qcommon::q_shared::qboolean,
) {
    syscall.expect("non-null function pointer")(
        crate::ui_public_h::UI_KEY_SETOVERSTRIKEMODE as i32 as crate::stdlib::intptr_t,
        state as u32,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_Key_ClearStates() {
    syscall.expect("non-null function pointer")(
        crate::ui_public_h::UI_KEY_CLEARSTATES as i32 as crate::stdlib::intptr_t,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_Key_GetCatcher() -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::ui_public_h::UI_KEY_GETCATCHER as i32 as crate::stdlib::intptr_t,
    ) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn trap_Key_SetCatcher(mut catcher: i32) {
    syscall.expect("non-null function pointer")(
        crate::ui_public_h::UI_KEY_SETCATCHER as i32 as crate::stdlib::intptr_t,
        catcher,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_GetClipboardData(mut buf: *mut libc::c_char, mut bufsize: i32) {
    syscall.expect("non-null function pointer")(
        crate::ui_public_h::UI_GETCLIPBOARDDATA as i32 as crate::stdlib::intptr_t,
        buf,
        bufsize,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_GetClientState(mut state: *mut crate::ui_public_h::uiClientState_t) {
    syscall.expect("non-null function pointer")(
        crate::ui_public_h::UI_GETCLIENTSTATE as i32 as crate::stdlib::intptr_t,
        state,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_GetGlconfig(mut glconfig: *mut crate::tr_types_h::glconfig_t) {
    syscall.expect("non-null function pointer")(
        crate::ui_public_h::UI_GETGLCONFIG as i32 as crate::stdlib::intptr_t,
        glconfig,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_GetConfigString(
    mut index: i32,
    mut buff: *mut libc::c_char,
    mut buffsize: i32,
) -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::ui_public_h::UI_GETCONFIGSTRING as i32 as crate::stdlib::intptr_t,
        index,
        buff,
        buffsize,
    ) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn trap_LAN_GetServerCount(mut source: i32) -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::ui_public_h::UI_LAN_GETSERVERCOUNT as i32 as crate::stdlib::intptr_t,
        source,
    ) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn trap_LAN_GetServerAddressString(
    mut source: i32,
    mut n: i32,
    mut buf: *mut libc::c_char,
    mut buflen: i32,
) {
    syscall.expect("non-null function pointer")(
        crate::ui_public_h::UI_LAN_GETSERVERADDRESSSTRING as i32 as crate::stdlib::intptr_t,
        source,
        n,
        buf,
        buflen,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_LAN_GetServerInfo(
    mut source: i32,
    mut n: i32,
    mut buf: *mut libc::c_char,
    mut buflen: i32,
) {
    syscall.expect("non-null function pointer")(
        crate::ui_public_h::UI_LAN_GETSERVERINFO as i32 as crate::stdlib::intptr_t,
        source,
        n,
        buf,
        buflen,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_LAN_GetServerPing(mut source: i32, mut n: i32) -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::ui_public_h::UI_LAN_GETSERVERPING as i32 as crate::stdlib::intptr_t,
        source,
        n,
    ) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn trap_LAN_GetPingQueueCount() -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::ui_public_h::UI_LAN_GETPINGQUEUECOUNT as i32 as crate::stdlib::intptr_t,
    ) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn trap_LAN_ServerStatus(
    mut serverAddress: *const libc::c_char,
    mut serverStatus: *mut libc::c_char,
    mut maxLen: i32,
) -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::ui_public_h::UI_LAN_SERVERSTATUS as i32 as crate::stdlib::intptr_t,
        serverAddress,
        serverStatus,
        maxLen,
    ) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn trap_LAN_SaveCachedServers() {
    syscall.expect("non-null function pointer")(
        crate::ui_public_h::UI_LAN_SAVECACHEDSERVERS as i32 as crate::stdlib::intptr_t,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_LAN_LoadCachedServers() {
    syscall.expect("non-null function pointer")(
        crate::ui_public_h::UI_LAN_LOADCACHEDSERVERS as i32 as crate::stdlib::intptr_t,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_LAN_ResetPings(mut n: i32) {
    syscall.expect("non-null function pointer")(
        crate::ui_public_h::UI_LAN_RESETPINGS as i32 as crate::stdlib::intptr_t,
        n,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_LAN_ClearPing(mut n: i32) {
    syscall.expect("non-null function pointer")(
        crate::ui_public_h::UI_LAN_CLEARPING as i32 as crate::stdlib::intptr_t,
        n,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_LAN_GetPing(
    mut n: i32,
    mut buf: *mut libc::c_char,
    mut buflen: i32,
    mut pingtime: *mut i32,
) {
    syscall.expect("non-null function pointer")(
        crate::ui_public_h::UI_LAN_GETPING as i32 as crate::stdlib::intptr_t,
        n,
        buf,
        buflen,
        pingtime,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_LAN_GetPingInfo(
    mut n: i32,
    mut buf: *mut libc::c_char,
    mut buflen: i32,
) {
    syscall.expect("non-null function pointer")(
        crate::ui_public_h::UI_LAN_GETPINGINFO as i32 as crate::stdlib::intptr_t,
        n,
        buf,
        buflen,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_LAN_MarkServerVisible(
    mut source: i32,
    mut n: i32,
    mut visible: crate::src::qcommon::q_shared::qboolean,
) {
    syscall.expect("non-null function pointer")(
        crate::ui_public_h::UI_LAN_MARKSERVERVISIBLE as i32 as crate::stdlib::intptr_t,
        source,
        n,
        visible as u32,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_LAN_ServerIsVisible(mut source: i32, mut n: i32) -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::ui_public_h::UI_LAN_SERVERISVISIBLE as i32 as crate::stdlib::intptr_t,
        source,
        n,
    ) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn trap_LAN_UpdateVisiblePings(
    mut source: i32,
) -> crate::src::qcommon::q_shared::qboolean {
    return syscall.expect("non-null function pointer")(
        crate::ui_public_h::UI_LAN_UPDATEVISIBLEPINGS as i32 as crate::stdlib::intptr_t,
        source,
    ) as crate::src::qcommon::q_shared::qboolean;
}
#[no_mangle]

pub unsafe extern "C" fn trap_LAN_AddServer(
    mut source: i32,
    mut name: *const libc::c_char,
    mut addr: *const libc::c_char,
) -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::ui_public_h::UI_LAN_ADDSERVER as i32 as crate::stdlib::intptr_t,
        source,
        name,
        addr,
    ) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn trap_LAN_RemoveServer(mut source: i32, mut addr: *const libc::c_char) {
    syscall.expect("non-null function pointer")(
        crate::ui_public_h::UI_LAN_REMOVESERVER as i32 as crate::stdlib::intptr_t,
        source,
        addr,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_LAN_CompareServers(
    mut source: i32,
    mut sortKey: i32,
    mut sortDir: i32,
    mut s1: i32,
    mut s2: i32,
) -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::ui_public_h::UI_LAN_COMPARESERVERS as i32 as crate::stdlib::intptr_t,
        source,
        sortKey,
        sortDir,
        s1,
        s2,
    ) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn trap_MemoryRemaining() -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::ui_public_h::UI_MEMORY_REMAINING as i32 as crate::stdlib::intptr_t,
    ) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn trap_GetCDKey(mut buf: *mut libc::c_char, mut buflen: i32) {
    syscall.expect("non-null function pointer")(
        crate::ui_public_h::UI_GET_CDKEY as i32 as crate::stdlib::intptr_t,
        buf,
        buflen,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_SetCDKey(mut buf: *mut libc::c_char) {
    syscall.expect("non-null function pointer")(
        crate::ui_public_h::UI_SET_CDKEY as i32 as crate::stdlib::intptr_t,
        buf,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_PC_AddGlobalDefine(mut define: *mut libc::c_char) -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::ui_public_h::UI_PC_ADD_GLOBAL_DEFINE as i32 as crate::stdlib::intptr_t,
        define,
    ) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn trap_PC_LoadSource(mut filename: *const libc::c_char) -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::ui_public_h::UI_PC_LOAD_SOURCE as i32 as crate::stdlib::intptr_t,
        filename,
    ) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn trap_PC_FreeSource(mut handle: i32) -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::ui_public_h::UI_PC_FREE_SOURCE as i32 as crate::stdlib::intptr_t,
        handle,
    ) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn trap_PC_ReadToken(
    mut handle: i32,
    mut pc_token: *mut crate::src::qcommon::q_shared::pc_token_t,
) -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::ui_public_h::UI_PC_READ_TOKEN as i32 as crate::stdlib::intptr_t,
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
        crate::ui_public_h::UI_PC_SOURCE_FILE_AND_LINE as i32 as crate::stdlib::intptr_t,
        handle,
        filename,
        line,
    ) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn trap_S_StopBackgroundTrack() {
    syscall.expect("non-null function pointer")(
        crate::ui_public_h::UI_S_STOPBACKGROUNDTRACK as i32 as crate::stdlib::intptr_t,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_S_StartBackgroundTrack(
    mut intro: *const libc::c_char,
    mut loop_0: *const libc::c_char,
) {
    syscall.expect("non-null function pointer")(
        crate::ui_public_h::UI_S_STARTBACKGROUNDTRACK as i32 as crate::stdlib::intptr_t,
        intro,
        loop_0,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_RealTime(
    mut qtime: *mut crate::src::qcommon::q_shared::qtime_t,
) -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::ui_public_h::UI_REAL_TIME as i32 as crate::stdlib::intptr_t,
        qtime,
    ) as i32;
}
// this returns a handle.  arg0 is the name in the format "idlogo.roq", set arg1 to NULL, alteredstates to qfalse (do not alter gamestate)
#[no_mangle]

pub unsafe extern "C" fn trap_CIN_PlayCinematic(
    mut arg0: *const libc::c_char,
    mut xpos: i32,
    mut ypos: i32,
    mut width: i32,
    mut height: i32,
    mut bits: i32,
) -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::ui_public_h::UI_CIN_PLAYCINEMATIC as i32 as crate::stdlib::intptr_t,
        arg0,
        xpos,
        ypos,
        width,
        height,
        bits,
    ) as i32;
}
// stops playing the cinematic and ends it.  should always return FMV_EOF
// cinematics must be stopped in reverse order of when they are started
#[no_mangle]

pub unsafe extern "C" fn trap_CIN_StopCinematic(
    mut handle: i32,
) -> crate::src::qcommon::q_shared::e_status {
    return syscall.expect("non-null function pointer")(
        crate::ui_public_h::UI_CIN_STOPCINEMATIC as i32 as crate::stdlib::intptr_t,
        handle,
    ) as crate::src::qcommon::q_shared::e_status;
}
// will run a frame of the cinematic but will not draw it.  Will return FMV_EOF if the end of the cinematic has been reached.
#[no_mangle]

pub unsafe extern "C" fn trap_CIN_RunCinematic(
    mut handle: i32,
) -> crate::src::qcommon::q_shared::e_status {
    return syscall.expect("non-null function pointer")(
        crate::ui_public_h::UI_CIN_RUNCINEMATIC as i32 as crate::stdlib::intptr_t,
        handle,
    ) as crate::src::qcommon::q_shared::e_status;
}
// draws the current frame
#[no_mangle]

pub unsafe extern "C" fn trap_CIN_DrawCinematic(mut handle: i32) {
    syscall.expect("non-null function pointer")(
        crate::ui_public_h::UI_CIN_DRAWCINEMATIC as i32 as crate::stdlib::intptr_t,
        handle,
    );
}
// allows you to resize the animation dynamically
#[no_mangle]

pub unsafe extern "C" fn trap_CIN_SetExtents(
    mut handle: i32,
    mut x: i32,
    mut y: i32,
    mut w: i32,
    mut h: i32,
) {
    syscall.expect("non-null function pointer")(
        crate::ui_public_h::UI_CIN_SETEXTENTS as i32 as crate::stdlib::intptr_t,
        handle,
        x,
        y,
        w,
        h,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_R_RemapShader(
    mut oldShader: *const libc::c_char,
    mut newShader: *const libc::c_char,
    mut timeOffset: *const libc::c_char,
) {
    syscall.expect("non-null function pointer")(
        crate::ui_public_h::UI_R_REMAP_SHADER as i32 as crate::stdlib::intptr_t,
        oldShader,
        newShader,
        timeOffset,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_VerifyCDKey(
    mut key: *const libc::c_char,
    mut chksum: *const libc::c_char,
) -> crate::src::qcommon::q_shared::qboolean {
    return syscall.expect("non-null function pointer")(
        crate::ui_public_h::UI_VERIFY_CDKEY as i32 as crate::stdlib::intptr_t,
        key,
        chksum,
    ) as crate::src::qcommon::q_shared::qboolean;
}
#[no_mangle]

pub unsafe extern "C" fn trap_SetPbClStatus(mut status: i32) {
    syscall.expect("non-null function pointer")(
        crate::ui_public_h::UI_SET_PBCLSTATUS as i32 as crate::stdlib::intptr_t,
        status,
    );
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

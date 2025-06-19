use ::libc;

pub use crate::stdlib::intptr_t;

pub use crate::cg_public_h::snapshot_t;
pub use crate::cg_public_h::CG_ACOS;
pub use crate::cg_public_h::CG_ADDCOMMAND;
pub use crate::cg_public_h::CG_ARGC;
pub use crate::cg_public_h::CG_ARGS;
pub use crate::cg_public_h::CG_ARGV;
pub use crate::cg_public_h::CG_ATAN2;
pub use crate::cg_public_h::CG_CEIL;
pub use crate::cg_public_h::CG_CIN_DRAWCINEMATIC;
pub use crate::cg_public_h::CG_CIN_PLAYCINEMATIC;
pub use crate::cg_public_h::CG_CIN_RUNCINEMATIC;
pub use crate::cg_public_h::CG_CIN_SETEXTENTS;
pub use crate::cg_public_h::CG_CIN_STOPCINEMATIC;
pub use crate::cg_public_h::CG_CM_BOXTRACE;
pub use crate::cg_public_h::CG_CM_CAPSULETRACE;
pub use crate::cg_public_h::CG_CM_INLINEMODEL;
pub use crate::cg_public_h::CG_CM_LOADMAP;
pub use crate::cg_public_h::CG_CM_LOADMODEL;
pub use crate::cg_public_h::CG_CM_MARKFRAGMENTS;
pub use crate::cg_public_h::CG_CM_NUMINLINEMODELS;
pub use crate::cg_public_h::CG_CM_POINTCONTENTS;
pub use crate::cg_public_h::CG_CM_TEMPBOXMODEL;
pub use crate::cg_public_h::CG_CM_TEMPCAPSULEMODEL;
pub use crate::cg_public_h::CG_CM_TRANSFORMEDBOXTRACE;
pub use crate::cg_public_h::CG_CM_TRANSFORMEDCAPSULETRACE;
pub use crate::cg_public_h::CG_CM_TRANSFORMEDPOINTCONTENTS;
pub use crate::cg_public_h::CG_COS;
pub use crate::cg_public_h::CG_CVAR_REGISTER;
pub use crate::cg_public_h::CG_CVAR_SET;
pub use crate::cg_public_h::CG_CVAR_UPDATE;
pub use crate::cg_public_h::CG_CVAR_VARIABLESTRINGBUFFER;
pub use crate::cg_public_h::CG_ERROR;
pub use crate::cg_public_h::CG_FLOOR;
pub use crate::cg_public_h::CG_FS_FCLOSEFILE;
pub use crate::cg_public_h::CG_FS_FOPENFILE;
pub use crate::cg_public_h::CG_FS_READ;
pub use crate::cg_public_h::CG_FS_SEEK;
pub use crate::cg_public_h::CG_FS_WRITE;
pub use crate::cg_public_h::CG_GETCURRENTCMDNUMBER;
pub use crate::cg_public_h::CG_GETCURRENTSNAPSHOTNUMBER;
pub use crate::cg_public_h::CG_GETGAMESTATE;
pub use crate::cg_public_h::CG_GETGLCONFIG;
pub use crate::cg_public_h::CG_GETSERVERCOMMAND;
pub use crate::cg_public_h::CG_GETSNAPSHOT;
pub use crate::cg_public_h::CG_GETUSERCMD;
pub use crate::cg_public_h::CG_GET_ENTITY_TOKEN;
pub use crate::cg_public_h::CG_KEY_GETCATCHER;
pub use crate::cg_public_h::CG_KEY_GETKEY;
pub use crate::cg_public_h::CG_KEY_ISDOWN;
pub use crate::cg_public_h::CG_KEY_SETCATCHER;
pub use crate::cg_public_h::CG_MEMCPY;
pub use crate::cg_public_h::CG_MEMORY_REMAINING;
pub use crate::cg_public_h::CG_MEMSET;
pub use crate::cg_public_h::CG_MILLISECONDS;
pub use crate::cg_public_h::CG_PC_ADD_GLOBAL_DEFINE;
pub use crate::cg_public_h::CG_PC_FREE_SOURCE;
pub use crate::cg_public_h::CG_PC_LOAD_SOURCE;
pub use crate::cg_public_h::CG_PC_READ_TOKEN;
pub use crate::cg_public_h::CG_PC_SOURCE_FILE_AND_LINE;
pub use crate::cg_public_h::CG_PRINT;
pub use crate::cg_public_h::CG_REAL_TIME;
pub use crate::cg_public_h::CG_REMOVECOMMAND;
pub use crate::cg_public_h::CG_R_ADDADDITIVELIGHTTOSCENE;
pub use crate::cg_public_h::CG_R_ADDLIGHTTOSCENE;
pub use crate::cg_public_h::CG_R_ADDPOLYSTOSCENE;
pub use crate::cg_public_h::CG_R_ADDPOLYTOSCENE;
pub use crate::cg_public_h::CG_R_ADDREFENTITYTOSCENE;
pub use crate::cg_public_h::CG_R_CLEARSCENE;
pub use crate::cg_public_h::CG_R_DRAWSTRETCHPIC;
pub use crate::cg_public_h::CG_R_INPVS;
pub use crate::cg_public_h::CG_R_LERPTAG;
pub use crate::cg_public_h::CG_R_LIGHTFORPOINT;
pub use crate::cg_public_h::CG_R_LOADWORLDMAP;
pub use crate::cg_public_h::CG_R_MODELBOUNDS;
pub use crate::cg_public_h::CG_R_REGISTERFONT;
pub use crate::cg_public_h::CG_R_REGISTERMODEL;
pub use crate::cg_public_h::CG_R_REGISTERSHADER;
pub use crate::cg_public_h::CG_R_REGISTERSHADERNOMIP;
pub use crate::cg_public_h::CG_R_REGISTERSKIN;
pub use crate::cg_public_h::CG_R_REMAP_SHADER;
pub use crate::cg_public_h::CG_R_RENDERSCENE;
pub use crate::cg_public_h::CG_R_SETCOLOR;
pub use crate::cg_public_h::CG_SENDCLIENTCOMMAND;
pub use crate::cg_public_h::CG_SENDCONSOLECOMMAND;
pub use crate::cg_public_h::CG_SETUSERCMDVALUE;
pub use crate::cg_public_h::CG_SIN;
pub use crate::cg_public_h::CG_SNAPVECTOR;
pub use crate::cg_public_h::CG_SQRT;
pub use crate::cg_public_h::CG_STRNCPY;
pub use crate::cg_public_h::CG_S_ADDLOOPINGSOUND;
pub use crate::cg_public_h::CG_S_ADDREALLOOPINGSOUND;
pub use crate::cg_public_h::CG_S_CLEARLOOPINGSOUNDS;
pub use crate::cg_public_h::CG_S_REGISTERSOUND;
pub use crate::cg_public_h::CG_S_RESPATIALIZE;
pub use crate::cg_public_h::CG_S_STARTBACKGROUNDTRACK;
pub use crate::cg_public_h::CG_S_STARTLOCALSOUND;
pub use crate::cg_public_h::CG_S_STARTSOUND;
pub use crate::cg_public_h::CG_S_STOPBACKGROUNDTRACK;
pub use crate::cg_public_h::CG_S_STOPLOOPINGSOUND;
pub use crate::cg_public_h::CG_S_UPDATEENTITYPOSITION;
pub use crate::cg_public_h::CG_TESTPRINTFLOAT;
pub use crate::cg_public_h::CG_TESTPRINTINT;
pub use crate::cg_public_h::CG_UPDATESCREEN;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::clipHandle_t;
pub use crate::src::qcommon::q_shared::cplane_s;
pub use crate::src::qcommon::q_shared::cplane_t;
pub use crate::src::qcommon::q_shared::cvarHandle_t;
pub use crate::src::qcommon::q_shared::e_status;
pub use crate::src::qcommon::q_shared::entityState_s;
pub use crate::src::qcommon::q_shared::entityState_t;
pub use crate::src::qcommon::q_shared::fileHandle_t;
pub use crate::src::qcommon::q_shared::floatint_t;
pub use crate::src::qcommon::q_shared::fontInfo_t;
pub use crate::src::qcommon::q_shared::fsMode_t;
pub use crate::src::qcommon::q_shared::gameState_t;
pub use crate::src::qcommon::q_shared::glyphInfo_t;
pub use crate::src::qcommon::q_shared::markFragment_t;
pub use crate::src::qcommon::q_shared::orientation_t;
pub use crate::src::qcommon::q_shared::pc_token_s;
pub use crate::src::qcommon::q_shared::pc_token_t;
pub use crate::src::qcommon::q_shared::playerState_s;
pub use crate::src::qcommon::q_shared::playerState_t;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qhandle_t;
pub use crate::src::qcommon::q_shared::qtime_s;
pub use crate::src::qcommon::q_shared::qtime_t;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::sfxHandle_t;
pub use crate::src::qcommon::q_shared::trType_t;
pub use crate::src::qcommon::q_shared::trace_t;
pub use crate::src::qcommon::q_shared::trajectory_t;
pub use crate::src::qcommon::q_shared::usercmd_s;
pub use crate::src::qcommon::q_shared::usercmd_t;
pub use crate::src::qcommon::q_shared::vec3_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::qcommon::q_shared::vmCvar_t;
pub use crate::src::qcommon::q_shared::C2RustUnnamed_0;
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
pub use crate::src::qcommon::q_shared::TR_GRAVITY;
pub use crate::src::qcommon::q_shared::TR_INTERPOLATE;
pub use crate::src::qcommon::q_shared::TR_LINEAR;
pub use crate::src::qcommon::q_shared::TR_LINEAR_STOP;
pub use crate::src::qcommon::q_shared::TR_SINE;
pub use crate::src::qcommon::q_shared::TR_STATIONARY;
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
// cg_syscalls.c -- this file is only included when building a dll
// cg_syscalls.asm is included instead when building a qvm
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

pub unsafe extern "C" fn trap_Print(mut fmt: *const libc::c_char) {
    syscall.expect("non-null function pointer")(
        crate::cg_public_h::CG_PRINT as i32 as crate::stdlib::intptr_t,
        fmt,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_Error(mut fmt: *const libc::c_char) -> ! {
    syscall.expect("non-null function pointer")(
        crate::cg_public_h::CG_ERROR as i32 as crate::stdlib::intptr_t,
        fmt,
    );
    // shut up GCC warning about returning functions, because we know better
    ::libc::exit(1 as i32);
}
#[no_mangle]

pub unsafe extern "C" fn trap_Milliseconds() -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::cg_public_h::CG_MILLISECONDS as i32 as crate::stdlib::intptr_t,
    ) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn trap_Cvar_Register(
    mut vmCvar: *mut crate::src::qcommon::q_shared::vmCvar_t,
    mut varName: *const libc::c_char,
    mut defaultValue: *const libc::c_char,
    mut flags: i32,
) {
    syscall.expect("non-null function pointer")(
        crate::cg_public_h::CG_CVAR_REGISTER as i32 as crate::stdlib::intptr_t,
        vmCvar,
        varName,
        defaultValue,
        flags,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_Cvar_Update(
    mut vmCvar: *mut crate::src::qcommon::q_shared::vmCvar_t,
) {
    syscall.expect("non-null function pointer")(
        crate::cg_public_h::CG_CVAR_UPDATE as i32 as crate::stdlib::intptr_t,
        vmCvar,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_Cvar_Set(
    mut var_name: *const libc::c_char,
    mut value: *const libc::c_char,
) {
    syscall.expect("non-null function pointer")(
        crate::cg_public_h::CG_CVAR_SET as i32 as crate::stdlib::intptr_t,
        var_name,
        value,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_Cvar_VariableStringBuffer(
    mut var_name: *const libc::c_char,
    mut buffer: *mut libc::c_char,
    mut bufsize: i32,
) {
    syscall.expect("non-null function pointer")(
        crate::cg_public_h::CG_CVAR_VARIABLESTRINGBUFFER as i32 as crate::stdlib::intptr_t,
        var_name,
        buffer,
        bufsize,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_Argc() -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::cg_public_h::CG_ARGC as i32 as crate::stdlib::intptr_t,
    ) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn trap_Argv(
    mut n: i32,
    mut buffer: *mut libc::c_char,
    mut bufferLength: i32,
) {
    syscall.expect("non-null function pointer")(
        crate::cg_public_h::CG_ARGV as i32 as crate::stdlib::intptr_t,
        n,
        buffer,
        bufferLength,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_Args(mut buffer: *mut libc::c_char, mut bufferLength: i32) {
    syscall.expect("non-null function pointer")(
        crate::cg_public_h::CG_ARGS as i32 as crate::stdlib::intptr_t,
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
        crate::cg_public_h::CG_FS_FOPENFILE as i32 as crate::stdlib::intptr_t,
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
        crate::cg_public_h::CG_FS_READ as i32 as crate::stdlib::intptr_t,
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
        crate::cg_public_h::CG_FS_WRITE as i32 as crate::stdlib::intptr_t,
        buffer,
        len,
        f,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_FS_FCloseFile(mut f: crate::src::qcommon::q_shared::fileHandle_t) {
    syscall.expect("non-null function pointer")(
        crate::cg_public_h::CG_FS_FCLOSEFILE as i32 as crate::stdlib::intptr_t,
        f,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_FS_Seek(
    mut f: crate::src::qcommon::q_shared::fileHandle_t,
    mut offset: isize,
    mut origin: i32,
) -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::cg_public_h::CG_FS_SEEK as i32 as crate::stdlib::intptr_t,
        f,
        offset,
        origin,
    ) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn trap_SendConsoleCommand(mut text: *const libc::c_char) {
    syscall.expect("non-null function pointer")(
        crate::cg_public_h::CG_SENDCONSOLECOMMAND as i32 as crate::stdlib::intptr_t,
        text,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_AddCommand(mut cmdName: *const libc::c_char) {
    syscall.expect("non-null function pointer")(
        crate::cg_public_h::CG_ADDCOMMAND as i32 as crate::stdlib::intptr_t,
        cmdName,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_RemoveCommand(mut cmdName: *const libc::c_char) {
    syscall.expect("non-null function pointer")(
        crate::cg_public_h::CG_REMOVECOMMAND as i32 as crate::stdlib::intptr_t,
        cmdName,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_SendClientCommand(mut s: *const libc::c_char) {
    syscall.expect("non-null function pointer")(
        crate::cg_public_h::CG_SENDCLIENTCOMMAND as i32 as crate::stdlib::intptr_t,
        s,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_UpdateScreen() {
    syscall.expect("non-null function pointer")(
        crate::cg_public_h::CG_UPDATESCREEN as i32 as crate::stdlib::intptr_t,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_CM_LoadMap(mut mapname: *const libc::c_char) {
    syscall.expect("non-null function pointer")(
        crate::cg_public_h::CG_CM_LOADMAP as i32 as crate::stdlib::intptr_t,
        mapname,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_CM_NumInlineModels() -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::cg_public_h::CG_CM_NUMINLINEMODELS as i32 as crate::stdlib::intptr_t,
    ) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn trap_CM_InlineModel(
    mut index: i32,
) -> crate::src::qcommon::q_shared::clipHandle_t {
    return syscall.expect("non-null function pointer")(
        crate::cg_public_h::CG_CM_INLINEMODEL as i32 as crate::stdlib::intptr_t,
        index,
    ) as crate::src::qcommon::q_shared::clipHandle_t;
}
#[no_mangle]

pub unsafe extern "C" fn trap_CM_TempBoxModel(
    mut mins: *const crate::src::qcommon::q_shared::vec_t,
    mut maxs: *const crate::src::qcommon::q_shared::vec_t,
) -> crate::src::qcommon::q_shared::clipHandle_t {
    return syscall.expect("non-null function pointer")(
        crate::cg_public_h::CG_CM_TEMPBOXMODEL as i32 as crate::stdlib::intptr_t,
        mins,
        maxs,
    ) as crate::src::qcommon::q_shared::clipHandle_t;
}
#[no_mangle]

pub unsafe extern "C" fn trap_CM_TempCapsuleModel(
    mut mins: *const crate::src::qcommon::q_shared::vec_t,
    mut maxs: *const crate::src::qcommon::q_shared::vec_t,
) -> crate::src::qcommon::q_shared::clipHandle_t {
    return syscall.expect("non-null function pointer")(
        crate::cg_public_h::CG_CM_TEMPCAPSULEMODEL as i32 as crate::stdlib::intptr_t,
        mins,
        maxs,
    ) as crate::src::qcommon::q_shared::clipHandle_t;
}
#[no_mangle]

pub unsafe extern "C" fn trap_CM_PointContents(
    mut p: *const crate::src::qcommon::q_shared::vec_t,
    mut model: crate::src::qcommon::q_shared::clipHandle_t,
) -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::cg_public_h::CG_CM_POINTCONTENTS as i32 as crate::stdlib::intptr_t,
        p,
        model,
    ) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn trap_CM_TransformedPointContents(
    mut p: *const crate::src::qcommon::q_shared::vec_t,
    mut model: crate::src::qcommon::q_shared::clipHandle_t,
    mut origin: *const crate::src::qcommon::q_shared::vec_t,
    mut angles: *const crate::src::qcommon::q_shared::vec_t,
) -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::cg_public_h::CG_CM_TRANSFORMEDPOINTCONTENTS as i32 as crate::stdlib::intptr_t,
        p,
        model,
        origin,
        angles,
    ) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn trap_CM_BoxTrace(
    mut results: *mut crate::src::qcommon::q_shared::trace_t,
    mut start: *const crate::src::qcommon::q_shared::vec_t,
    mut end: *const crate::src::qcommon::q_shared::vec_t,
    mut mins: *const crate::src::qcommon::q_shared::vec_t,
    mut maxs: *const crate::src::qcommon::q_shared::vec_t,
    mut model: crate::src::qcommon::q_shared::clipHandle_t,
    mut brushmask: i32,
) {
    syscall.expect("non-null function pointer")(
        crate::cg_public_h::CG_CM_BOXTRACE as i32 as crate::stdlib::intptr_t,
        results,
        start,
        end,
        mins,
        maxs,
        model,
        brushmask,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_CM_CapsuleTrace(
    mut results: *mut crate::src::qcommon::q_shared::trace_t,
    mut start: *const crate::src::qcommon::q_shared::vec_t,
    mut end: *const crate::src::qcommon::q_shared::vec_t,
    mut mins: *const crate::src::qcommon::q_shared::vec_t,
    mut maxs: *const crate::src::qcommon::q_shared::vec_t,
    mut model: crate::src::qcommon::q_shared::clipHandle_t,
    mut brushmask: i32,
) {
    syscall.expect("non-null function pointer")(
        crate::cg_public_h::CG_CM_CAPSULETRACE as i32 as crate::stdlib::intptr_t,
        results,
        start,
        end,
        mins,
        maxs,
        model,
        brushmask,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_CM_TransformedBoxTrace(
    mut results: *mut crate::src::qcommon::q_shared::trace_t,
    mut start: *const crate::src::qcommon::q_shared::vec_t,
    mut end: *const crate::src::qcommon::q_shared::vec_t,
    mut mins: *const crate::src::qcommon::q_shared::vec_t,
    mut maxs: *const crate::src::qcommon::q_shared::vec_t,
    mut model: crate::src::qcommon::q_shared::clipHandle_t,
    mut brushmask: i32,
    mut origin: *const crate::src::qcommon::q_shared::vec_t,
    mut angles: *const crate::src::qcommon::q_shared::vec_t,
) {
    syscall.expect("non-null function pointer")(
        crate::cg_public_h::CG_CM_TRANSFORMEDBOXTRACE as i32 as crate::stdlib::intptr_t,
        results,
        start,
        end,
        mins,
        maxs,
        model,
        brushmask,
        origin,
        angles,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_CM_TransformedCapsuleTrace(
    mut results: *mut crate::src::qcommon::q_shared::trace_t,
    mut start: *const crate::src::qcommon::q_shared::vec_t,
    mut end: *const crate::src::qcommon::q_shared::vec_t,
    mut mins: *const crate::src::qcommon::q_shared::vec_t,
    mut maxs: *const crate::src::qcommon::q_shared::vec_t,
    mut model: crate::src::qcommon::q_shared::clipHandle_t,
    mut brushmask: i32,
    mut origin: *const crate::src::qcommon::q_shared::vec_t,
    mut angles: *const crate::src::qcommon::q_shared::vec_t,
) {
    syscall.expect("non-null function pointer")(
        crate::cg_public_h::CG_CM_TRANSFORMEDCAPSULETRACE as i32 as crate::stdlib::intptr_t,
        results,
        start,
        end,
        mins,
        maxs,
        model,
        brushmask,
        origin,
        angles,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_CM_MarkFragments(
    mut numPoints: i32,
    mut points: *const crate::src::qcommon::q_shared::vec3_t,
    mut projection: *const crate::src::qcommon::q_shared::vec_t,
    mut maxPoints: i32,
    mut pointBuffer: *mut crate::src::qcommon::q_shared::vec_t,
    mut maxFragments: i32,
    mut fragmentBuffer: *mut crate::src::qcommon::q_shared::markFragment_t,
) -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::cg_public_h::CG_CM_MARKFRAGMENTS as i32 as crate::stdlib::intptr_t,
        numPoints,
        points,
        projection,
        maxPoints,
        pointBuffer,
        maxFragments,
        fragmentBuffer,
    ) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn trap_S_StartSound(
    mut origin: *mut crate::src::qcommon::q_shared::vec_t,
    mut entityNum: i32,
    mut entchannel: i32,
    mut sfx: crate::src::qcommon::q_shared::sfxHandle_t,
) {
    syscall.expect("non-null function pointer")(
        crate::cg_public_h::CG_S_STARTSOUND as i32 as crate::stdlib::intptr_t,
        origin,
        entityNum,
        entchannel,
        sfx,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_S_StartLocalSound(
    mut sfx: crate::src::qcommon::q_shared::sfxHandle_t,
    mut channelNum: i32,
) {
    syscall.expect("non-null function pointer")(
        crate::cg_public_h::CG_S_STARTLOCALSOUND as i32 as crate::stdlib::intptr_t,
        sfx,
        channelNum,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_S_ClearLoopingSounds(
    mut killall: crate::src::qcommon::q_shared::qboolean,
) {
    syscall.expect("non-null function pointer")(
        crate::cg_public_h::CG_S_CLEARLOOPINGSOUNDS as i32 as crate::stdlib::intptr_t,
        killall as u32,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_S_AddLoopingSound(
    mut entityNum: i32,
    mut origin: *const crate::src::qcommon::q_shared::vec_t,
    mut velocity: *const crate::src::qcommon::q_shared::vec_t,
    mut sfx: crate::src::qcommon::q_shared::sfxHandle_t,
) {
    syscall.expect("non-null function pointer")(
        crate::cg_public_h::CG_S_ADDLOOPINGSOUND as i32 as crate::stdlib::intptr_t,
        entityNum,
        origin,
        velocity,
        sfx,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_S_AddRealLoopingSound(
    mut entityNum: i32,
    mut origin: *const crate::src::qcommon::q_shared::vec_t,
    mut velocity: *const crate::src::qcommon::q_shared::vec_t,
    mut sfx: crate::src::qcommon::q_shared::sfxHandle_t,
) {
    syscall.expect("non-null function pointer")(
        crate::cg_public_h::CG_S_ADDREALLOOPINGSOUND as i32 as crate::stdlib::intptr_t,
        entityNum,
        origin,
        velocity,
        sfx,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_S_StopLoopingSound(mut entityNum: i32) {
    syscall.expect("non-null function pointer")(
        crate::cg_public_h::CG_S_STOPLOOPINGSOUND as i32 as crate::stdlib::intptr_t,
        entityNum,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_S_UpdateEntityPosition(
    mut entityNum: i32,
    mut origin: *const crate::src::qcommon::q_shared::vec_t,
) {
    syscall.expect("non-null function pointer")(
        crate::cg_public_h::CG_S_UPDATEENTITYPOSITION as i32 as crate::stdlib::intptr_t,
        entityNum,
        origin,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_S_Respatialize(
    mut entityNum: i32,
    mut origin: *const crate::src::qcommon::q_shared::vec_t,
    mut axis: *mut crate::src::qcommon::q_shared::vec3_t,
    mut inwater: i32,
) {
    syscall.expect("non-null function pointer")(
        crate::cg_public_h::CG_S_RESPATIALIZE as i32 as crate::stdlib::intptr_t,
        entityNum,
        origin,
        axis,
        inwater,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_S_RegisterSound(
    mut sample: *const libc::c_char,
    mut compressed: crate::src::qcommon::q_shared::qboolean,
) -> crate::src::qcommon::q_shared::sfxHandle_t {
    return syscall.expect("non-null function pointer")(
        crate::cg_public_h::CG_S_REGISTERSOUND as i32 as crate::stdlib::intptr_t,
        sample,
        compressed as u32,
    ) as crate::src::qcommon::q_shared::sfxHandle_t;
}
#[no_mangle]

pub unsafe extern "C" fn trap_S_StartBackgroundTrack(
    mut intro: *const libc::c_char,
    mut loop_0: *const libc::c_char,
) {
    syscall.expect("non-null function pointer")(
        crate::cg_public_h::CG_S_STARTBACKGROUNDTRACK as i32 as crate::stdlib::intptr_t,
        intro,
        loop_0,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_R_LoadWorldMap(mut mapname: *const libc::c_char) {
    syscall.expect("non-null function pointer")(
        crate::cg_public_h::CG_R_LOADWORLDMAP as i32 as crate::stdlib::intptr_t,
        mapname,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_R_RegisterModel(
    mut name: *const libc::c_char,
) -> crate::src::qcommon::q_shared::qhandle_t {
    return syscall.expect("non-null function pointer")(
        crate::cg_public_h::CG_R_REGISTERMODEL as i32 as crate::stdlib::intptr_t,
        name,
    ) as crate::src::qcommon::q_shared::qhandle_t;
}
#[no_mangle]

pub unsafe extern "C" fn trap_R_RegisterSkin(
    mut name: *const libc::c_char,
) -> crate::src::qcommon::q_shared::qhandle_t {
    return syscall.expect("non-null function pointer")(
        crate::cg_public_h::CG_R_REGISTERSKIN as i32 as crate::stdlib::intptr_t,
        name,
    ) as crate::src::qcommon::q_shared::qhandle_t;
}
#[no_mangle]

pub unsafe extern "C" fn trap_R_RegisterShader(
    mut name: *const libc::c_char,
) -> crate::src::qcommon::q_shared::qhandle_t {
    return syscall.expect("non-null function pointer")(
        crate::cg_public_h::CG_R_REGISTERSHADER as i32 as crate::stdlib::intptr_t,
        name,
    ) as crate::src::qcommon::q_shared::qhandle_t;
}
#[no_mangle]

pub unsafe extern "C" fn trap_R_RegisterShaderNoMip(
    mut name: *const libc::c_char,
) -> crate::src::qcommon::q_shared::qhandle_t {
    return syscall.expect("non-null function pointer")(
        crate::cg_public_h::CG_R_REGISTERSHADERNOMIP as i32 as crate::stdlib::intptr_t,
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
        crate::cg_public_h::CG_R_REGISTERFONT as i32 as crate::stdlib::intptr_t,
        fontName,
        pointSize,
        font,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_R_ClearScene() {
    syscall.expect("non-null function pointer")(
        crate::cg_public_h::CG_R_CLEARSCENE as i32 as crate::stdlib::intptr_t,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_R_AddRefEntityToScene(mut re: *const crate::tr_types_h::refEntity_t) {
    syscall.expect("non-null function pointer")(
        crate::cg_public_h::CG_R_ADDREFENTITYTOSCENE as i32 as crate::stdlib::intptr_t,
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
        crate::cg_public_h::CG_R_ADDPOLYTOSCENE as i32 as crate::stdlib::intptr_t,
        hShader,
        numVerts,
        verts,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_R_AddPolysToScene(
    mut hShader: crate::src::qcommon::q_shared::qhandle_t,
    mut numVerts: i32,
    mut verts: *const crate::tr_types_h::polyVert_t,
    mut num: i32,
) {
    syscall.expect("non-null function pointer")(
        crate::cg_public_h::CG_R_ADDPOLYSTOSCENE as i32 as crate::stdlib::intptr_t,
        hShader,
        numVerts,
        verts,
        num,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_R_LightForPoint(
    mut point: *mut crate::src::qcommon::q_shared::vec_t,
    mut ambientLight: *mut crate::src::qcommon::q_shared::vec_t,
    mut directedLight: *mut crate::src::qcommon::q_shared::vec_t,
    mut lightDir: *mut crate::src::qcommon::q_shared::vec_t,
) -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::cg_public_h::CG_R_LIGHTFORPOINT as i32 as crate::stdlib::intptr_t,
        point,
        ambientLight,
        directedLight,
        lightDir,
    ) as i32;
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
        crate::cg_public_h::CG_R_ADDLIGHTTOSCENE as i32 as crate::stdlib::intptr_t,
        org,
        PASSFLOAT(intensity),
        PASSFLOAT(r),
        PASSFLOAT(g),
        PASSFLOAT(b),
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_R_AddAdditiveLightToScene(
    mut org: *const crate::src::qcommon::q_shared::vec_t,
    mut intensity: f32,
    mut r: f32,
    mut g: f32,
    mut b: f32,
) {
    syscall.expect("non-null function pointer")(
        crate::cg_public_h::CG_R_ADDADDITIVELIGHTTOSCENE as i32 as crate::stdlib::intptr_t,
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
        crate::cg_public_h::CG_R_RENDERSCENE as i32 as crate::stdlib::intptr_t,
        fd,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_R_SetColor(mut rgba: *const f32) {
    syscall.expect("non-null function pointer")(
        crate::cg_public_h::CG_R_SETCOLOR as i32 as crate::stdlib::intptr_t,
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
        crate::cg_public_h::CG_R_DRAWSTRETCHPIC as i32 as crate::stdlib::intptr_t,
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
        crate::cg_public_h::CG_R_MODELBOUNDS as i32 as crate::stdlib::intptr_t,
        model,
        mins,
        maxs,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_R_LerpTag(
    mut tag: *mut crate::src::qcommon::q_shared::orientation_t,
    mut mod_0: crate::src::qcommon::q_shared::clipHandle_t,
    mut startFrame: i32,
    mut endFrame: i32,
    mut frac: f32,
    mut tagName: *const libc::c_char,
) -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::cg_public_h::CG_R_LERPTAG as i32 as crate::stdlib::intptr_t,
        tag,
        mod_0,
        startFrame,
        endFrame,
        PASSFLOAT(frac),
        tagName,
    ) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn trap_R_RemapShader(
    mut oldShader: *const libc::c_char,
    mut newShader: *const libc::c_char,
    mut timeOffset: *const libc::c_char,
) {
    syscall.expect("non-null function pointer")(
        crate::cg_public_h::CG_R_REMAP_SHADER as i32 as crate::stdlib::intptr_t,
        oldShader,
        newShader,
        timeOffset,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_GetGlconfig(mut glconfig: *mut crate::tr_types_h::glconfig_t) {
    syscall.expect("non-null function pointer")(
        crate::cg_public_h::CG_GETGLCONFIG as i32 as crate::stdlib::intptr_t,
        glconfig,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_GetGameState(
    mut gamestate: *mut crate::src::qcommon::q_shared::gameState_t,
) {
    syscall.expect("non-null function pointer")(
        crate::cg_public_h::CG_GETGAMESTATE as i32 as crate::stdlib::intptr_t,
        gamestate,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_GetCurrentSnapshotNumber(
    mut snapshotNumber: *mut i32,
    mut serverTime: *mut i32,
) {
    syscall.expect("non-null function pointer")(
        crate::cg_public_h::CG_GETCURRENTSNAPSHOTNUMBER as i32 as crate::stdlib::intptr_t,
        snapshotNumber,
        serverTime,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_GetSnapshot(
    mut snapshotNumber: i32,
    mut snapshot: *mut crate::cg_public_h::snapshot_t,
) -> crate::src::qcommon::q_shared::qboolean {
    return syscall.expect("non-null function pointer")(
        crate::cg_public_h::CG_GETSNAPSHOT as i32 as crate::stdlib::intptr_t,
        snapshotNumber,
        snapshot,
    ) as crate::src::qcommon::q_shared::qboolean;
}
#[no_mangle]

pub unsafe extern "C" fn trap_GetServerCommand(
    mut serverCommandNumber: i32,
) -> crate::src::qcommon::q_shared::qboolean {
    return syscall.expect("non-null function pointer")(
        crate::cg_public_h::CG_GETSERVERCOMMAND as i32 as crate::stdlib::intptr_t,
        serverCommandNumber,
    ) as crate::src::qcommon::q_shared::qboolean;
}
#[no_mangle]

pub unsafe extern "C" fn trap_GetCurrentCmdNumber() -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::cg_public_h::CG_GETCURRENTCMDNUMBER as i32 as crate::stdlib::intptr_t,
    ) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn trap_GetUserCmd(
    mut cmdNumber: i32,
    mut ucmd: *mut crate::src::qcommon::q_shared::usercmd_t,
) -> crate::src::qcommon::q_shared::qboolean {
    return syscall.expect("non-null function pointer")(
        crate::cg_public_h::CG_GETUSERCMD as i32 as crate::stdlib::intptr_t,
        cmdNumber,
        ucmd,
    ) as crate::src::qcommon::q_shared::qboolean;
}
#[no_mangle]

pub unsafe extern "C" fn trap_SetUserCmdValue(mut stateValue: i32, mut sensitivityScale: f32) {
    syscall.expect("non-null function pointer")(
        crate::cg_public_h::CG_SETUSERCMDVALUE as i32 as crate::stdlib::intptr_t,
        stateValue,
        PASSFLOAT(sensitivityScale),
    );
}
#[no_mangle]

pub unsafe extern "C" fn testPrintInt(mut string: *mut libc::c_char, mut i: i32) {
    syscall.expect("non-null function pointer")(
        crate::cg_public_h::CG_TESTPRINTINT as i32 as crate::stdlib::intptr_t,
        string,
        i,
    );
}
#[no_mangle]

pub unsafe extern "C" fn testPrintFloat(mut string: *mut libc::c_char, mut f: f32) {
    syscall.expect("non-null function pointer")(
        crate::cg_public_h::CG_TESTPRINTFLOAT as i32 as crate::stdlib::intptr_t,
        string,
        PASSFLOAT(f),
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_MemoryRemaining() -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::cg_public_h::CG_MEMORY_REMAINING as i32 as crate::stdlib::intptr_t,
    ) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn trap_Key_IsDown(
    mut keynum: i32,
) -> crate::src::qcommon::q_shared::qboolean {
    return syscall.expect("non-null function pointer")(
        crate::cg_public_h::CG_KEY_ISDOWN as i32 as crate::stdlib::intptr_t,
        keynum,
    ) as crate::src::qcommon::q_shared::qboolean;
}
#[no_mangle]

pub unsafe extern "C" fn trap_Key_GetCatcher() -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::cg_public_h::CG_KEY_GETCATCHER as i32 as crate::stdlib::intptr_t,
    ) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn trap_Key_SetCatcher(mut catcher: i32) {
    syscall.expect("non-null function pointer")(
        crate::cg_public_h::CG_KEY_SETCATCHER as i32 as crate::stdlib::intptr_t,
        catcher,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_Key_GetKey(mut binding: *const libc::c_char) -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::cg_public_h::CG_KEY_GETKEY as i32 as crate::stdlib::intptr_t,
        binding,
    ) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn trap_PC_AddGlobalDefine(mut define: *mut libc::c_char) -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::cg_public_h::CG_PC_ADD_GLOBAL_DEFINE as i32 as crate::stdlib::intptr_t,
        define,
    ) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn trap_PC_LoadSource(mut filename: *const libc::c_char) -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::cg_public_h::CG_PC_LOAD_SOURCE as i32 as crate::stdlib::intptr_t,
        filename,
    ) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn trap_PC_FreeSource(mut handle: i32) -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::cg_public_h::CG_PC_FREE_SOURCE as i32 as crate::stdlib::intptr_t,
        handle,
    ) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn trap_PC_ReadToken(
    mut handle: i32,
    mut pc_token: *mut crate::src::qcommon::q_shared::pc_token_t,
) -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::cg_public_h::CG_PC_READ_TOKEN as i32 as crate::stdlib::intptr_t,
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
        crate::cg_public_h::CG_PC_SOURCE_FILE_AND_LINE as i32 as crate::stdlib::intptr_t,
        handle,
        filename,
        line,
    ) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn trap_S_StopBackgroundTrack() {
    syscall.expect("non-null function pointer")(
        crate::cg_public_h::CG_S_STOPBACKGROUNDTRACK as i32 as crate::stdlib::intptr_t,
    );
}
#[no_mangle]

pub unsafe extern "C" fn trap_RealTime(
    mut qtime: *mut crate::src::qcommon::q_shared::qtime_t,
) -> i32 {
    return syscall.expect("non-null function pointer")(
        crate::cg_public_h::CG_REAL_TIME as i32 as crate::stdlib::intptr_t,
        qtime,
    ) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn trap_SnapVector(mut v: *mut f32) {
    syscall.expect("non-null function pointer")(
        crate::cg_public_h::CG_SNAPVECTOR as i32 as crate::stdlib::intptr_t,
        v,
    );
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
        crate::cg_public_h::CG_CIN_PLAYCINEMATIC as i32 as crate::stdlib::intptr_t,
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
        crate::cg_public_h::CG_CIN_STOPCINEMATIC as i32 as crate::stdlib::intptr_t,
        handle,
    ) as crate::src::qcommon::q_shared::e_status;
}
// will run a frame of the cinematic but will not draw it.  Will return FMV_EOF if the end of the cinematic has been reached.
#[no_mangle]

pub unsafe extern "C" fn trap_CIN_RunCinematic(
    mut handle: i32,
) -> crate::src::qcommon::q_shared::e_status {
    return syscall.expect("non-null function pointer")(
        crate::cg_public_h::CG_CIN_RUNCINEMATIC as i32 as crate::stdlib::intptr_t,
        handle,
    ) as crate::src::qcommon::q_shared::e_status;
}
// draws the current frame
#[no_mangle]

pub unsafe extern "C" fn trap_CIN_DrawCinematic(mut handle: i32) {
    syscall.expect("non-null function pointer")(
        crate::cg_public_h::CG_CIN_DRAWCINEMATIC as i32 as crate::stdlib::intptr_t,
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
        crate::cg_public_h::CG_CIN_SETEXTENTS as i32 as crate::stdlib::intptr_t,
        handle,
        x,
        y,
        w,
        h,
    );
}
// The glconfig_t will not change during the life of a cgame.
// If it needs to change, the entire cgame will be restarted, because
// all the qhandle_t are then invalid.
// the gamestate should be grabbed at startup, and whenever a
// configstring changes
// cgame will poll each frame to see if a newer snapshot has arrived
// that it is interested in.  The time is returned separately so that
// snapshot latency can be calculated.
// a snapshot get can fail if the snapshot (or the entties it holds) is so
// old that it has fallen out of the client system queue
// retrieve a text command from the server stream
// the current snapshot will hold the number of the most recent command
// qfalse can be returned if the client system handled the command
// argc() / argv() can be used to examine the parameters of the command
// returns the most recent command number that can be passed to GetUserCmd
// this will always be at least one higher than the number in the current
// snapshot, and it may be quite a few higher if it is a fast computer on
// a lagged connection
// used for the weapon select and zoom
// aids for VM testing
/*
qboolean trap_loadCamera( const char *name ) {
    return syscall( CG_LOADCAMERA, name );
}

void trap_startCamera(int time) {
    syscall(CG_STARTCAMERA, time);
}

qboolean trap_getCameraInfo( int time, vec3_t *origin, vec3_t *angles) {
    return syscall( CG_GETCAMERAINFO, time, origin, angles );
}
*/
#[no_mangle]

pub unsafe extern "C" fn trap_GetEntityToken(
    mut buffer: *mut libc::c_char,
    mut bufferSize: i32,
) -> crate::src::qcommon::q_shared::qboolean {
    return syscall.expect("non-null function pointer")(
        crate::cg_public_h::CG_GET_ENTITY_TOKEN as i32 as crate::stdlib::intptr_t,
        buffer,
        bufferSize,
    ) as crate::src::qcommon::q_shared::qboolean;
}
//
// cg_view.c
//
//
// cg_drawtools.c
//
//
// cg_draw.c, cg_newDraw.c
//
//
// cg_player.c
//
//
// cg_predict.c
//
//
// cg_events.c
//
//
// cg_ents.c
//
//
// cg_weapons.c
//
// should this be in pmove?
//
// cg_marks.c
//
//
// cg_localents.c
//
//
// cg_effects.c
//
//
// cg_snapshot.c
//
//
// cg_info.c
//
//
// cg_scoreboard.c
//
//
// cg_consolecmds.c
//
//
// cg_servercmds.c
//
//
// cg_playerstate.c
//
//===============================================
//
// system traps
// These functions are how the cgame communicates with the main game system
//
// print message on the local console
// abort the game
// milliseconds should only be used for performance tuning, never
// for anything game related.  Get time from the CG_DrawActiveFrame parameter
// console variable interaction
// ServerCommand and ConsoleCommand parameter access
// filesystem access
// returns length of file
// fsOrigin_t
// add commands to the local console as if they were typed in
// for map changing, etc.  The command is not executed immediately,
// but will be executed in order the next time console commands
// are processed
// register a command name so the console can perform command completion.
// FIXME: replace this with a normal console command "defineCommand"?
// send a string to the server over the network
// force a screen update, only used during gamestate load
// model collision
// 0 = world, 1+ = bmodels
// Returns the projection of a polygon onto the solid brushes in the world
// normal sounds will have their volume dynamically changed as their entity
// moves and the listener moves
// a local sound is always played full volume
// respatialize recalculates the volumes of sound as they should be heard by the
// given entityNum and position
// returns buzz if not found
// empty name stops music
// all media should be registered during level startup to prevent
// hitches during gameplay
// returns rgb axis if not found
// returns all white if not found
// returns all white if not found
// returns all white if not found
// a scene is built up by calls to R_ClearScene and the various R_Add functions.
// Nothing is drawn until R_RenderScene is called.
// polys are intended for simple wall marks, not really for doing
// significant construction
// NULL = 1,1,1,1
#[no_mangle]

pub unsafe extern "C" fn trap_R_inPVS(
    mut p1: *const crate::src::qcommon::q_shared::vec_t,
    mut p2: *const crate::src::qcommon::q_shared::vec_t,
) -> crate::src::qcommon::q_shared::qboolean {
    return syscall.expect("non-null function pointer")(
        crate::cg_public_h::CG_R_INPVS as i32 as crate::stdlib::intptr_t,
        p1,
        p2,
    ) as crate::src::qcommon::q_shared::qboolean;
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

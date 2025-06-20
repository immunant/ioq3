use ::libc;

pub use crate::botlib_h::bot_input_s;
pub use crate::botlib_h::bot_input_t;
pub use crate::botlib_h::botlib_import_s;
pub use crate::botlib_h::botlib_import_t;
pub use crate::botlib_h::bsp_surface_s;
pub use crate::botlib_h::bsp_surface_t;
pub use crate::botlib_h::bsp_trace_s;
pub use crate::botlib_h::bsp_trace_t;
pub use crate::src::botlib::be_interface::botimport;
pub use crate::src::botlib::be_interface::botlib_globals_s;
pub use crate::src::botlib::be_interface::botlib_globals_t;
pub use crate::src::botlib::be_interface::botlibglobals;

pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::cplane_s;
pub use crate::src::qcommon::q_shared::cplane_t;
pub use crate::src::qcommon::q_shared::fileHandle_t;
pub use crate::src::qcommon::q_shared::fsMode_t;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::va;
pub use crate::src::qcommon::q_shared::vec3_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::qcommon::q_shared::FS_APPEND;
pub use crate::src::qcommon::q_shared::FS_APPEND_SYNC;
pub use crate::src::qcommon::q_shared::FS_READ;
pub use crate::src::qcommon::q_shared::FS_WRITE;

#[no_mangle]

pub static mut botinputs: *mut bot_input_t = std::ptr::null_mut();
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
/* ****************************************************************************
 * name:		be_ea.h
 *
 * desc:		elementary actions
 *
 * $Archive: /source/code/botlib/be_ea.h $
 *
 *****************************************************************************/
//ClientCommand elementary actions
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn EA_Say(mut client: i32, mut str: *mut libc::c_char) {
    botimport
        .BotClientCommand
        .expect("non-null function pointer")(
        client,
        va(
            b"say %s\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            str,
        ),
    );
}
//end of the function EA_Say
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn EA_SayTeam(mut client: i32, mut str: *mut libc::c_char) {
    botimport
        .BotClientCommand
        .expect("non-null function pointer")(
        client,
        va(
            b"say_team %s\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            str,
        ),
    );
}
//end of the function EA_SayTeam
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn EA_Tell(mut client: i32, mut clientto: i32, mut str: *mut libc::c_char) {
    botimport
        .BotClientCommand
        .expect("non-null function pointer")(
        client,
        va(
            b"tell %d, %s\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            clientto,
            str,
        ),
    );
}
//end of the function EA_SayTeam
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn EA_UseItem(mut client: i32, mut it: *mut libc::c_char) {
    botimport
        .BotClientCommand
        .expect("non-null function pointer")(
        client,
        va(
            b"use %s\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            it,
        ),
    );
}
//end of the function EA_UseItem
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn EA_DropItem(mut client: i32, mut it: *mut libc::c_char) {
    botimport
        .BotClientCommand
        .expect("non-null function pointer")(
        client,
        va(
            b"drop %s\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            it,
        ),
    );
}
//end of the function EA_DropItem
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn EA_UseInv(mut client: i32, mut inv: *mut libc::c_char) {
    botimport
        .BotClientCommand
        .expect("non-null function pointer")(
        client,
        va(
            b"invuse %s\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            inv,
        ),
    );
}
//end of the function EA_UseInv
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn EA_DropInv(mut client: i32, mut inv: *mut libc::c_char) {
    botimport
        .BotClientCommand
        .expect("non-null function pointer")(
        client,
        va(
            b"invdrop %s\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            inv,
        ),
    );
}
//end of the function EA_DropInv
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn EA_Gesture(mut client: i32) {
    let mut bi: *mut bot_input_t = 0 as *mut bot_input_t;
    bi = &mut *botinputs.offset(client as isize) as *mut bot_input_t;
    (*bi).actionflags |= 0x20000 as i32;
}
//end of the function EA_Gesture
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn EA_Command(mut client: i32, mut command: *mut libc::c_char) {
    botimport
        .BotClientCommand
        .expect("non-null function pointer")(client, command);
}
//regular elementary actions
//end of the function EA_Command
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn EA_SelectWeapon(mut client: i32, mut weapon: i32) {
    let mut bi: *mut bot_input_t = 0 as *mut bot_input_t;
    bi = &mut *botinputs.offset(client as isize) as *mut bot_input_t;
    (*bi).weapon = weapon;
}
//end of the function EA_SelectWeapon
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn EA_Attack(mut client: i32) {
    let mut bi: *mut bot_input_t = 0 as *mut bot_input_t;
    bi = &mut *botinputs.offset(client as isize) as *mut bot_input_t;
    (*bi).actionflags |= 0x1 as i32;
}
//end of the function EA_Attack
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn EA_Talk(mut client: i32) {
    let mut bi: *mut bot_input_t = 0 as *mut bot_input_t;
    bi = &mut *botinputs.offset(client as isize) as *mut bot_input_t;
    (*bi).actionflags |= 0x10000 as i32;
}
//end of the function EA_Talk
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn EA_Use(mut client: i32) {
    let mut bi: *mut bot_input_t = 0 as *mut bot_input_t;
    bi = &mut *botinputs.offset(client as isize) as *mut bot_input_t;
    (*bi).actionflags |= 0x2 as i32;
}
//end of the function EA_Use
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn EA_Respawn(mut client: i32) {
    let mut bi: *mut bot_input_t = 0 as *mut bot_input_t;
    bi = &mut *botinputs.offset(client as isize) as *mut bot_input_t;
    (*bi).actionflags |= 0x8 as i32;
}
//end of the function EA_Respawn
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn EA_Jump(mut client: i32) {
    let mut bi: *mut bot_input_t = 0 as *mut bot_input_t; //end if
    bi = &mut *botinputs.offset(client as isize) as *mut bot_input_t;
    if (*bi).actionflags & 0x10000000 as i32 != 0 {
        (*bi).actionflags &= !(0x10 as i32)
    } else {
        (*bi).actionflags |= 0x10 as i32
    };
    //end if
}
//end of the function EA_Jump
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn EA_DelayedJump(mut client: i32) {
    let mut bi: *mut bot_input_t = 0 as *mut bot_input_t; //end if
    bi = &mut *botinputs.offset(client as isize) as *mut bot_input_t;
    if (*bi).actionflags & 0x10000000 as i32 != 0 {
        (*bi).actionflags &= !(0x8000 as i32)
    } else {
        (*bi).actionflags |= 0x8000 as i32
    };
    //end if
}
//end of the function EA_DelayedJump
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn EA_Crouch(mut client: i32) {
    let mut bi: *mut bot_input_t = 0 as *mut bot_input_t;
    bi = &mut *botinputs.offset(client as isize) as *mut bot_input_t;
    (*bi).actionflags |= 0x80 as i32;
}
//end of the function EA_Crouch
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn EA_Walk(mut client: i32) {
    let mut bi: *mut bot_input_t = 0 as *mut bot_input_t;
    bi = &mut *botinputs.offset(client as isize) as *mut bot_input_t;
    (*bi).actionflags |= 0x80000 as i32;
}
//end of the function EA_Walk
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn EA_Action(mut client: i32, mut action: i32) {
    let mut bi: *mut bot_input_t = 0 as *mut bot_input_t;
    bi = &mut *botinputs.offset(client as isize) as *mut bot_input_t;
    (*bi).actionflags |= action;
}
//end of function EA_Action
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn EA_MoveUp(mut client: i32) {
    let mut bi: *mut bot_input_t = 0 as *mut bot_input_t;
    bi = &mut *botinputs.offset(client as isize) as *mut bot_input_t;
    (*bi).actionflags |= 0x20 as i32;
}
//end of the function EA_MoveUp
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn EA_MoveDown(mut client: i32) {
    let mut bi: *mut bot_input_t = 0 as *mut bot_input_t;
    bi = &mut *botinputs.offset(client as isize) as *mut bot_input_t;
    (*bi).actionflags |= 0x100 as i32;
}
//end of the function EA_MoveDown
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn EA_MoveForward(mut client: i32) {
    let mut bi: *mut bot_input_t = 0 as *mut bot_input_t;
    bi = &mut *botinputs.offset(client as isize) as *mut bot_input_t;
    (*bi).actionflags |= 0x200 as i32;
}
//end of the function EA_MoveForward
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn EA_MoveBack(mut client: i32) {
    let mut bi: *mut bot_input_t = 0 as *mut bot_input_t;
    bi = &mut *botinputs.offset(client as isize) as *mut bot_input_t;
    (*bi).actionflags |= 0x800 as i32;
}
//end of the function EA_MoveBack
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn EA_MoveLeft(mut client: i32) {
    let mut bi: *mut bot_input_t = 0 as *mut bot_input_t;
    bi = &mut *botinputs.offset(client as isize) as *mut bot_input_t;
    (*bi).actionflags |= 0x1000 as i32;
}
//end of the function EA_MoveLeft
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn EA_MoveRight(mut client: i32) {
    let mut bi: *mut bot_input_t = 0 as *mut bot_input_t;
    bi = &mut *botinputs.offset(client as isize) as *mut bot_input_t;
    (*bi).actionflags |= 0x2000 as i32;
}
//end of the function EA_MoveRight
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn EA_Move(mut client: i32, mut dir: *mut vec_t, mut speed: f32) {
    let mut bi: *mut bot_input_t = 0 as *mut bot_input_t;
    bi = &mut *botinputs.offset(client as isize) as *mut bot_input_t;
    (*bi).dir[0 as i32 as usize] = *dir.offset(0 as i32 as isize);
    (*bi).dir[1 as i32 as usize] = *dir.offset(1 as i32 as isize);
    (*bi).dir[2 as i32 as usize] = *dir.offset(2 as i32 as isize);
    //cap speed
    if speed > 400 as i32 as f32 {
        speed = 400 as i32 as f32
    } else if speed < -(400 as i32) as f32 {
        speed = -(400 as i32) as f32
    }
    (*bi).speed = speed;
}
//end of the function EA_Move
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn EA_View(mut client: i32, mut viewangles: *mut vec_t) {
    let mut bi: *mut bot_input_t = 0 as *mut bot_input_t;
    bi = &mut *botinputs.offset(client as isize) as *mut bot_input_t;
    (*bi).viewangles[0 as i32 as usize] = *viewangles.offset(0 as i32 as isize);
    (*bi).viewangles[1 as i32 as usize] = *viewangles.offset(1 as i32 as isize);
    (*bi).viewangles[2 as i32 as usize] = *viewangles.offset(2 as i32 as isize);
}
//send regular input to the server
//end of the function EA_View
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn EA_EndRegular(mut _client: i32, mut _thinktime: f32) {}
//end of the function EA_EndRegular
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn EA_GetInput(
    mut client: i32,
    mut thinktime: f32,
    mut input: *mut bot_input_t,
) {
    let mut bi: *mut bot_input_t = 0 as *mut bot_input_t;
    bi = &mut *botinputs.offset(client as isize) as *mut bot_input_t;
    (*bi).thinktime = thinktime;
    crate::stdlib::memcpy(
        input as *mut libc::c_void,
        bi as *const libc::c_void,
        ::std::mem::size_of::<bot_input_t>() as usize,
    );
}
//end of the function EA_GetInput
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn EA_ResetInput(mut client: i32) {
    let mut bi: *mut bot_input_t = 0 as *mut bot_input_t;
    let mut jumped: i32 = qfalse as i32;
    bi = &mut *botinputs.offset(client as isize) as *mut bot_input_t;
    (*bi).thinktime = 0 as i32 as f32;
    (*bi).dir[2 as i32 as usize] = 0 as i32 as vec_t;
    (*bi).dir[1 as i32 as usize] = (*bi).dir[2 as i32 as usize];
    (*bi).dir[0 as i32 as usize] = (*bi).dir[1 as i32 as usize];
    (*bi).speed = 0 as i32 as f32;
    jumped = (*bi).actionflags & 0x10 as i32;
    (*bi).actionflags = 0 as i32;
    if jumped != 0 {
        (*bi).actionflags |= 0x10000000 as i32
    };
}
//setup and shutdown routines
//end of the function EA_ResetInput
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn EA_Setup() -> i32 {
    //initialize the bot inputs
    botinputs = crate::src::botlib::l_memory::GetClearedHunkMemory(
        (botlibglobals.maxclients as usize)
            .wrapping_mul(::std::mem::size_of::<bot_input_t>() as usize),
    ) as *mut bot_input_t;
    return 0 as i32;
}
//end of the function EA_Setup
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn EA_Shutdown() {
    crate::src::botlib::l_memory::FreeMemory(botinputs as *mut libc::c_void);
    botinputs = 0 as *mut bot_input_t;
}
//end of the function EA_Shutdown

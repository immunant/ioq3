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

pub static mut botinputs: *mut crate::botlib_h::bot_input_t =
    0 as *const crate::botlib_h::bot_input_t as *mut crate::botlib_h::bot_input_t;
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

pub unsafe extern "C" fn EA_Say(mut client: libc::c_int, mut str: *mut libc::c_char) {
    crate::src::botlib::be_interface::botimport
        .BotClientCommand
        .expect("non-null function pointer")(
        client,
        crate::src::qcommon::q_shared::va(
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

pub unsafe extern "C" fn EA_SayTeam(mut client: libc::c_int, mut str: *mut libc::c_char) {
    crate::src::botlib::be_interface::botimport
        .BotClientCommand
        .expect("non-null function pointer")(
        client,
        crate::src::qcommon::q_shared::va(
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

pub unsafe extern "C" fn EA_Tell(
    mut client: libc::c_int,
    mut clientto: libc::c_int,
    mut str: *mut libc::c_char,
) {
    crate::src::botlib::be_interface::botimport
        .BotClientCommand
        .expect("non-null function pointer")(
        client,
        crate::src::qcommon::q_shared::va(
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

pub unsafe extern "C" fn EA_UseItem(mut client: libc::c_int, mut it: *mut libc::c_char) {
    crate::src::botlib::be_interface::botimport
        .BotClientCommand
        .expect("non-null function pointer")(
        client,
        crate::src::qcommon::q_shared::va(
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

pub unsafe extern "C" fn EA_DropItem(mut client: libc::c_int, mut it: *mut libc::c_char) {
    crate::src::botlib::be_interface::botimport
        .BotClientCommand
        .expect("non-null function pointer")(
        client,
        crate::src::qcommon::q_shared::va(
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

pub unsafe extern "C" fn EA_UseInv(mut client: libc::c_int, mut inv: *mut libc::c_char) {
    crate::src::botlib::be_interface::botimport
        .BotClientCommand
        .expect("non-null function pointer")(
        client,
        crate::src::qcommon::q_shared::va(
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

pub unsafe extern "C" fn EA_DropInv(mut client: libc::c_int, mut inv: *mut libc::c_char) {
    crate::src::botlib::be_interface::botimport
        .BotClientCommand
        .expect("non-null function pointer")(
        client,
        crate::src::qcommon::q_shared::va(
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

pub unsafe extern "C" fn EA_Gesture(mut client: libc::c_int) {
    let mut bi: *mut crate::botlib_h::bot_input_t = 0 as *mut crate::botlib_h::bot_input_t;
    bi = &mut *botinputs.offset(client as isize) as *mut crate::botlib_h::bot_input_t;
    (*bi).actionflags |= 0x20000 as libc::c_int;
}
//end of the function EA_Gesture
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn EA_Command(mut client: libc::c_int, mut command: *mut libc::c_char) {
    crate::src::botlib::be_interface::botimport
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

pub unsafe extern "C" fn EA_SelectWeapon(mut client: libc::c_int, mut weapon: libc::c_int) {
    let mut bi: *mut crate::botlib_h::bot_input_t = 0 as *mut crate::botlib_h::bot_input_t;
    bi = &mut *botinputs.offset(client as isize) as *mut crate::botlib_h::bot_input_t;
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

pub unsafe extern "C" fn EA_Attack(mut client: libc::c_int) {
    let mut bi: *mut crate::botlib_h::bot_input_t = 0 as *mut crate::botlib_h::bot_input_t;
    bi = &mut *botinputs.offset(client as isize) as *mut crate::botlib_h::bot_input_t;
    (*bi).actionflags |= 0x1 as libc::c_int;
}
//end of the function EA_Attack
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn EA_Talk(mut client: libc::c_int) {
    let mut bi: *mut crate::botlib_h::bot_input_t = 0 as *mut crate::botlib_h::bot_input_t;
    bi = &mut *botinputs.offset(client as isize) as *mut crate::botlib_h::bot_input_t;
    (*bi).actionflags |= 0x10000 as libc::c_int;
}
//end of the function EA_Talk
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn EA_Use(mut client: libc::c_int) {
    let mut bi: *mut crate::botlib_h::bot_input_t = 0 as *mut crate::botlib_h::bot_input_t;
    bi = &mut *botinputs.offset(client as isize) as *mut crate::botlib_h::bot_input_t;
    (*bi).actionflags |= 0x2 as libc::c_int;
}
//end of the function EA_Use
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn EA_Respawn(mut client: libc::c_int) {
    let mut bi: *mut crate::botlib_h::bot_input_t = 0 as *mut crate::botlib_h::bot_input_t;
    bi = &mut *botinputs.offset(client as isize) as *mut crate::botlib_h::bot_input_t;
    (*bi).actionflags |= 0x8 as libc::c_int;
}
//end of the function EA_Respawn
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn EA_Jump(mut client: libc::c_int) {
    let mut bi: *mut crate::botlib_h::bot_input_t = 0 as *mut crate::botlib_h::bot_input_t; //end if
    bi = &mut *botinputs.offset(client as isize) as *mut crate::botlib_h::bot_input_t;
    if (*bi).actionflags & 0x10000000 as libc::c_int != 0 {
        (*bi).actionflags &= !(0x10 as libc::c_int)
    } else {
        (*bi).actionflags |= 0x10 as libc::c_int
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

pub unsafe extern "C" fn EA_DelayedJump(mut client: libc::c_int) {
    let mut bi: *mut crate::botlib_h::bot_input_t = 0 as *mut crate::botlib_h::bot_input_t; //end if
    bi = &mut *botinputs.offset(client as isize) as *mut crate::botlib_h::bot_input_t;
    if (*bi).actionflags & 0x10000000 as libc::c_int != 0 {
        (*bi).actionflags &= !(0x8000 as libc::c_int)
    } else {
        (*bi).actionflags |= 0x8000 as libc::c_int
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

pub unsafe extern "C" fn EA_Crouch(mut client: libc::c_int) {
    let mut bi: *mut crate::botlib_h::bot_input_t = 0 as *mut crate::botlib_h::bot_input_t;
    bi = &mut *botinputs.offset(client as isize) as *mut crate::botlib_h::bot_input_t;
    (*bi).actionflags |= 0x80 as libc::c_int;
}
//end of the function EA_Crouch
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn EA_Walk(mut client: libc::c_int) {
    let mut bi: *mut crate::botlib_h::bot_input_t = 0 as *mut crate::botlib_h::bot_input_t;
    bi = &mut *botinputs.offset(client as isize) as *mut crate::botlib_h::bot_input_t;
    (*bi).actionflags |= 0x80000 as libc::c_int;
}
//end of the function EA_Walk
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn EA_Action(mut client: libc::c_int, mut action: libc::c_int) {
    let mut bi: *mut crate::botlib_h::bot_input_t = 0 as *mut crate::botlib_h::bot_input_t;
    bi = &mut *botinputs.offset(client as isize) as *mut crate::botlib_h::bot_input_t;
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

pub unsafe extern "C" fn EA_MoveUp(mut client: libc::c_int) {
    let mut bi: *mut crate::botlib_h::bot_input_t = 0 as *mut crate::botlib_h::bot_input_t;
    bi = &mut *botinputs.offset(client as isize) as *mut crate::botlib_h::bot_input_t;
    (*bi).actionflags |= 0x20 as libc::c_int;
}
//end of the function EA_MoveUp
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn EA_MoveDown(mut client: libc::c_int) {
    let mut bi: *mut crate::botlib_h::bot_input_t = 0 as *mut crate::botlib_h::bot_input_t;
    bi = &mut *botinputs.offset(client as isize) as *mut crate::botlib_h::bot_input_t;
    (*bi).actionflags |= 0x100 as libc::c_int;
}
//end of the function EA_MoveDown
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn EA_MoveForward(mut client: libc::c_int) {
    let mut bi: *mut crate::botlib_h::bot_input_t = 0 as *mut crate::botlib_h::bot_input_t;
    bi = &mut *botinputs.offset(client as isize) as *mut crate::botlib_h::bot_input_t;
    (*bi).actionflags |= 0x200 as libc::c_int;
}
//end of the function EA_MoveForward
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn EA_MoveBack(mut client: libc::c_int) {
    let mut bi: *mut crate::botlib_h::bot_input_t = 0 as *mut crate::botlib_h::bot_input_t;
    bi = &mut *botinputs.offset(client as isize) as *mut crate::botlib_h::bot_input_t;
    (*bi).actionflags |= 0x800 as libc::c_int;
}
//end of the function EA_MoveBack
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn EA_MoveLeft(mut client: libc::c_int) {
    let mut bi: *mut crate::botlib_h::bot_input_t = 0 as *mut crate::botlib_h::bot_input_t;
    bi = &mut *botinputs.offset(client as isize) as *mut crate::botlib_h::bot_input_t;
    (*bi).actionflags |= 0x1000 as libc::c_int;
}
//end of the function EA_MoveLeft
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn EA_MoveRight(mut client: libc::c_int) {
    let mut bi: *mut crate::botlib_h::bot_input_t = 0 as *mut crate::botlib_h::bot_input_t;
    bi = &mut *botinputs.offset(client as isize) as *mut crate::botlib_h::bot_input_t;
    (*bi).actionflags |= 0x2000 as libc::c_int;
}
//end of the function EA_MoveRight
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn EA_Move(
    mut client: libc::c_int,
    mut dir: *mut crate::src::qcommon::q_shared::vec_t,
    mut speed: libc::c_float,
) {
    let mut bi: *mut crate::botlib_h::bot_input_t = 0 as *mut crate::botlib_h::bot_input_t;
    bi = &mut *botinputs.offset(client as isize) as *mut crate::botlib_h::bot_input_t;
    (*bi).dir[0 as libc::c_int as usize] = *dir.offset(0 as libc::c_int as isize);
    (*bi).dir[1 as libc::c_int as usize] = *dir.offset(1 as libc::c_int as isize);
    (*bi).dir[2 as libc::c_int as usize] = *dir.offset(2 as libc::c_int as isize);
    //cap speed
    if speed > 400 as libc::c_int as libc::c_float {
        speed = 400 as libc::c_int as libc::c_float
    } else if speed < -(400 as libc::c_int) as libc::c_float {
        speed = -(400 as libc::c_int) as libc::c_float
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

pub unsafe extern "C" fn EA_View(
    mut client: libc::c_int,
    mut viewangles: *mut crate::src::qcommon::q_shared::vec_t,
) {
    let mut bi: *mut crate::botlib_h::bot_input_t = 0 as *mut crate::botlib_h::bot_input_t;
    bi = &mut *botinputs.offset(client as isize) as *mut crate::botlib_h::bot_input_t;
    (*bi).viewangles[0 as libc::c_int as usize] = *viewangles.offset(0 as libc::c_int as isize);
    (*bi).viewangles[1 as libc::c_int as usize] = *viewangles.offset(1 as libc::c_int as isize);
    (*bi).viewangles[2 as libc::c_int as usize] = *viewangles.offset(2 as libc::c_int as isize);
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

pub unsafe extern "C" fn EA_EndRegular(mut _client: libc::c_int, mut _thinktime: libc::c_float) {}
//end of the function EA_EndRegular
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn EA_GetInput(
    mut client: libc::c_int,
    mut thinktime: libc::c_float,
    mut input: *mut crate::botlib_h::bot_input_t,
) {
    let mut bi: *mut crate::botlib_h::bot_input_t = 0 as *mut crate::botlib_h::bot_input_t;
    bi = &mut *botinputs.offset(client as isize) as *mut crate::botlib_h::bot_input_t;
    (*bi).thinktime = thinktime;
    crate::stdlib::memcpy(
        input as *mut libc::c_void,
        bi as *const libc::c_void,
        ::std::mem::size_of::<crate::botlib_h::bot_input_t>() as libc::c_ulong,
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

pub unsafe extern "C" fn EA_ResetInput(mut client: libc::c_int) {
    let mut bi: *mut crate::botlib_h::bot_input_t = 0 as *mut crate::botlib_h::bot_input_t;
    let mut jumped: libc::c_int = crate::src::qcommon::q_shared::qfalse as libc::c_int;
    bi = &mut *botinputs.offset(client as isize) as *mut crate::botlib_h::bot_input_t;
    (*bi).thinktime = 0 as libc::c_int as libc::c_float;
    (*bi).dir[2 as libc::c_int as usize] = 0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
    (*bi).dir[1 as libc::c_int as usize] = (*bi).dir[2 as libc::c_int as usize];
    (*bi).dir[0 as libc::c_int as usize] = (*bi).dir[1 as libc::c_int as usize];
    (*bi).speed = 0 as libc::c_int as libc::c_float;
    jumped = (*bi).actionflags & 0x10 as libc::c_int;
    (*bi).actionflags = 0 as libc::c_int;
    if jumped != 0 {
        (*bi).actionflags |= 0x10000000 as libc::c_int
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

pub unsafe extern "C" fn EA_Setup() -> libc::c_int {
    //initialize the bot inputs
    botinputs = crate::src::botlib::l_memory::GetClearedHunkMemory(
        (crate::src::botlib::be_interface::botlibglobals.maxclients as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<crate::botlib_h::bot_input_t>() as libc::c_ulong),
    ) as *mut crate::botlib_h::bot_input_t;
    return 0 as libc::c_int;
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
    botinputs = 0 as *mut crate::botlib_h::bot_input_t;
}
//end of the function EA_Shutdown

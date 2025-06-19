use ::libc;

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

pub use crate::src::q3_ui::ui_sparena::stdlib_h::atoi;
pub use crate::src::qcommon::q_shared::va;
pub use crate::src::qcommon::q_shared::C2RustUnnamed_0;
pub use crate::src::qcommon::q_shared::Info_ValueForKey;
pub use crate::src::qcommon::q_shared::Q_stricmp;
pub use crate::src::qcommon::q_shared::EXEC_APPEND;
pub use crate::src::qcommon::q_shared::EXEC_INSERT;
pub use crate::src::qcommon::q_shared::EXEC_NOW;

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
//
// ui_spLevel.c
//
//
// ui_spArena.c
//
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
#[no_mangle]

pub unsafe extern "C" fn UI_SPArena_Start(mut arenaInfo: *const libc::c_char) {
    let mut map: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut level: i32 = 0;
    let mut n: i32 = 0;
    let mut txt: *mut libc::c_char = 0 as *mut libc::c_char;
    n = crate::src::ui::ui_syscalls::trap_Cvar_VariableValue(
        b"sv_maxclients\x00" as *const u8 as *const libc::c_char,
    ) as i32;
    if n < 8 as i32 {
        crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
            b"sv_maxclients\x00" as *const u8 as *const libc::c_char,
            8 as i32 as f32,
        );
    }
    level = atoi(Info_ValueForKey(
        arenaInfo,
        b"num\x00" as *const u8 as *const libc::c_char,
    ));
    txt = Info_ValueForKey(
        arenaInfo,
        b"special\x00" as *const u8 as *const libc::c_char,
    );
    if *txt.offset(0 as i32 as isize) != 0 {
        if Q_stricmp(txt, b"training\x00" as *const u8 as *const libc::c_char) == 0 as i32 {
            level = -(4 as i32)
        } else if Q_stricmp(txt, b"final\x00" as *const u8 as *const libc::c_char) == 0 as i32 {
            level = crate::src::q3_ui::ui_gameinfo::UI_GetNumSPTiers() * 4 as i32
        }
    }
    crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
        b"ui_spSelection\x00" as *const u8 as *const libc::c_char,
        level as f32,
    );
    map = Info_ValueForKey(arenaInfo, b"map\x00" as *const u8 as *const libc::c_char);
    crate::src::ui::ui_syscalls::trap_Cmd_ExecuteText(
        EXEC_APPEND as i32,
        va(
            b"spmap %s\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            map,
        ),
    );
}

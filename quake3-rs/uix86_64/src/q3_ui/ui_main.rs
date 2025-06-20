use ::libc;

pub use crate::stdlib::intptr_t;

pub use crate::src::qcommon::q_shared::cvarHandle_t;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::vmCvar_t;
pub use crate::src::qcommon::q_shared::C2RustUnnamed_0;

pub use crate::ui_public_h::uiMenuCommand_t;
pub use crate::ui_public_h::UIMENU_BAD_CD_KEY;
pub use crate::ui_public_h::UIMENU_INGAME;
pub use crate::ui_public_h::UIMENU_MAIN;
pub use crate::ui_public_h::UIMENU_NEED_CD;
pub use crate::ui_public_h::UIMENU_NONE;
pub use crate::ui_public_h::UIMENU_POSTGAME;
pub use crate::ui_public_h::UIMENU_TEAM;
pub use crate::ui_public_h::UI_CONSOLE_COMMAND;
pub use crate::ui_public_h::UI_DRAW_CONNECT_SCREEN;
pub use crate::ui_public_h::UI_GETAPIVERSION;
pub use crate::ui_public_h::UI_HASUNIQUECDKEY;
pub use crate::ui_public_h::UI_INIT;
pub use crate::ui_public_h::UI_IS_FULLSCREEN;
pub use crate::ui_public_h::UI_KEY_EVENT;
pub use crate::ui_public_h::UI_MOUSE_EVENT;
pub use crate::ui_public_h::UI_REFRESH;
pub use crate::ui_public_h::UI_SET_ACTIVE_MENU;
pub use crate::ui_public_h::UI_SHUTDOWN;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvarTable_t {
    pub vmCvar: *mut vmCvar_t,
    pub cvarName: *mut libc::c_char,
    pub defaultString: *mut libc::c_char,
    pub cvarFlags: i32,
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
/*
=======================================================================

USER INTERFACE MAIN

=======================================================================
*/
/*
================
vmMain

This is the only way control passes into the module.
This must be the very first function compiled into the .qvm file
================
*/
#[no_mangle]

pub unsafe extern "C" fn vmMain(
    mut command: i32,
    mut arg0: i32,
    mut arg1: i32,
    mut _arg2: i32,
    mut _arg3: i32,
    mut _arg4: i32,
    mut _arg5: i32,
    mut _arg6: i32,
    mut _arg7: i32,
    mut _arg8: i32,
    mut _arg9: i32,
    mut _arg10: i32,
    mut _arg11: i32,
) -> intptr_t {
    match command {
        0 => {
            return 4 as i32 as intptr_t;
            // change this to qfalse for mods!
        }
        1 => {
            crate::src::q3_ui::ui_atoms::UI_Init();
            return 0 as i32 as intptr_t;
        }
        2 => {
            crate::src::q3_ui::ui_atoms::UI_Shutdown();
            return 0 as i32 as intptr_t;
        }
        3 => {
            crate::src::q3_ui::ui_atoms::UI_KeyEvent(arg0, arg1);
            return 0 as i32 as intptr_t;
        }
        4 => {
            crate::src::q3_ui::ui_atoms::UI_MouseEvent(arg0, arg1);
            return 0 as i32 as intptr_t;
        }
        5 => {
            crate::src::q3_ui::ui_atoms::UI_Refresh(arg0);
            return 0 as i32 as intptr_t;
        }
        6 => return crate::src::q3_ui::ui_atoms::UI_IsFullscreen() as intptr_t,
        7 => {
            crate::src::q3_ui::ui_atoms::UI_SetActiveMenu(arg0 as uiMenuCommand_t);
            return 0 as i32 as intptr_t;
        }
        8 => return crate::src::q3_ui::ui_atoms::UI_ConsoleCommand(arg0) as intptr_t,
        9 => {
            crate::src::q3_ui::ui_connect::UI_DrawConnectScreen(arg0 as qboolean);
            return 0 as i32 as intptr_t;
        }
        10 => {
            // mod authors need to observe this
            return qtrue as i32 as intptr_t;
        }
        _ => {}
    }
    return -(1 as i32) as intptr_t;
}
#[no_mangle]

pub static mut ui_ffa_fraglimit: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]

pub static mut ui_ffa_timelimit: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]

pub static mut ui_tourney_fraglimit: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]

pub static mut ui_tourney_timelimit: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]

pub static mut ui_team_fraglimit: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]

pub static mut ui_team_timelimit: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]

pub static mut ui_team_friendly: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]

pub static mut ui_ctf_capturelimit: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]

pub static mut ui_ctf_timelimit: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]

pub static mut ui_ctf_friendly: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]

pub static mut ui_arenasFile: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]

pub static mut ui_botsFile: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]

pub static mut ui_spScores1: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]

pub static mut ui_spScores2: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]

pub static mut ui_spScores3: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]

pub static mut ui_spScores4: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]

pub static mut ui_spScores5: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]

pub static mut ui_spAwards: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]

pub static mut ui_spVideos: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]

pub static mut ui_spSkill: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]

pub static mut ui_spSelection: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]

pub static mut ui_browserMaster: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]

pub static mut ui_browserGameType: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]

pub static mut ui_browserSortKey: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]

pub static mut ui_browserShowFull: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]

pub static mut ui_browserShowEmpty: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]

pub static mut ui_brassTime: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]

pub static mut ui_drawCrosshair: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]

pub static mut ui_drawCrosshairNames: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]

pub static mut ui_marks: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]

pub static mut ui_server1: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]

pub static mut ui_server2: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]

pub static mut ui_server3: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]

pub static mut ui_server4: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]

pub static mut ui_server5: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]

pub static mut ui_server6: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]

pub static mut ui_server7: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]

pub static mut ui_server8: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]

pub static mut ui_server9: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]

pub static mut ui_server10: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]

pub static mut ui_server11: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]

pub static mut ui_server12: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]

pub static mut ui_server13: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]

pub static mut ui_server14: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]

pub static mut ui_server15: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]

pub static mut ui_server16: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]

pub static mut ui_cdkeychecked: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]

pub static mut ui_ioq3: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};

static mut cvarTable: [cvarTable_t; 49] = unsafe {
    [
        {
            let mut init = cvarTable_t {
                vmCvar: &ui_ffa_fraglimit as *const vmCvar_t as *mut vmCvar_t,
                cvarName: b"ui_ffa_fraglimit\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                defaultString: b"20\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                cvarFlags: 0x1 as i32,
            };
            init
        },
        {
            let mut init = cvarTable_t {
                vmCvar: &ui_ffa_timelimit as *const vmCvar_t as *mut vmCvar_t,
                cvarName: b"ui_ffa_timelimit\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                defaultString: b"0\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                cvarFlags: 0x1 as i32,
            };
            init
        },
        {
            let mut init = cvarTable_t {
                vmCvar: &ui_tourney_fraglimit as *const vmCvar_t as *mut vmCvar_t,
                cvarName: b"ui_tourney_fraglimit\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                defaultString: b"0\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                cvarFlags: 0x1 as i32,
            };
            init
        },
        {
            let mut init = cvarTable_t {
                vmCvar: &ui_tourney_timelimit as *const vmCvar_t as *mut vmCvar_t,
                cvarName: b"ui_tourney_timelimit\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                defaultString: b"15\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                cvarFlags: 0x1 as i32,
            };
            init
        },
        {
            let mut init = cvarTable_t {
                vmCvar: &ui_team_fraglimit as *const vmCvar_t as *mut vmCvar_t,
                cvarName: b"ui_team_fraglimit\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                defaultString: b"0\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                cvarFlags: 0x1 as i32,
            };
            init
        },
        {
            let mut init = cvarTable_t {
                vmCvar: &ui_team_timelimit as *const vmCvar_t as *mut vmCvar_t,
                cvarName: b"ui_team_timelimit\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                defaultString: b"20\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                cvarFlags: 0x1 as i32,
            };
            init
        },
        {
            let mut init = cvarTable_t {
                vmCvar: &ui_team_friendly as *const vmCvar_t as *mut vmCvar_t,
                cvarName: b"ui_team_friendly\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                defaultString: b"1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                cvarFlags: 0x1 as i32,
            };
            init
        },
        {
            let mut init = cvarTable_t {
                vmCvar: &ui_ctf_capturelimit as *const vmCvar_t as *mut vmCvar_t,
                cvarName: b"ui_ctf_capturelimit\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                defaultString: b"8\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                cvarFlags: 0x1 as i32,
            };
            init
        },
        {
            let mut init = cvarTable_t {
                vmCvar: &ui_ctf_timelimit as *const vmCvar_t as *mut vmCvar_t,
                cvarName: b"ui_ctf_timelimit\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                defaultString: b"30\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                cvarFlags: 0x1 as i32,
            };
            init
        },
        {
            let mut init = cvarTable_t {
                vmCvar: &ui_ctf_friendly as *const vmCvar_t as *mut vmCvar_t,
                cvarName: b"ui_ctf_friendly\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                defaultString: b"0\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                cvarFlags: 0x1 as i32,
            };
            init
        },
        {
            let mut init = cvarTable_t {
                vmCvar: &ui_arenasFile as *const vmCvar_t as *mut vmCvar_t,
                cvarName: b"g_arenasFile\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                defaultString: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                cvarFlags: 0x10 as i32 | 0x40 as i32,
            };
            init
        },
        {
            let mut init = cvarTable_t {
                vmCvar: &ui_botsFile as *const vmCvar_t as *mut vmCvar_t,
                cvarName: b"g_botsFile\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                defaultString: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                cvarFlags: 0x10 as i32 | 0x40 as i32,
            };
            init
        },
        {
            let mut init = cvarTable_t {
                vmCvar: &ui_spScores1 as *const vmCvar_t as *mut vmCvar_t,
                cvarName: b"g_spScores1\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                defaultString: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                cvarFlags: 0x1 as i32,
            };
            init
        },
        {
            let mut init = cvarTable_t {
                vmCvar: &ui_spScores2 as *const vmCvar_t as *mut vmCvar_t,
                cvarName: b"g_spScores2\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                defaultString: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                cvarFlags: 0x1 as i32,
            };
            init
        },
        {
            let mut init = cvarTable_t {
                vmCvar: &ui_spScores3 as *const vmCvar_t as *mut vmCvar_t,
                cvarName: b"g_spScores3\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                defaultString: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                cvarFlags: 0x1 as i32,
            };
            init
        },
        {
            let mut init = cvarTable_t {
                vmCvar: &ui_spScores4 as *const vmCvar_t as *mut vmCvar_t,
                cvarName: b"g_spScores4\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                defaultString: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                cvarFlags: 0x1 as i32,
            };
            init
        },
        {
            let mut init = cvarTable_t {
                vmCvar: &ui_spScores5 as *const vmCvar_t as *mut vmCvar_t,
                cvarName: b"g_spScores5\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                defaultString: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                cvarFlags: 0x1 as i32,
            };
            init
        },
        {
            let mut init = cvarTable_t {
                vmCvar: &ui_spAwards as *const vmCvar_t as *mut vmCvar_t,
                cvarName: b"g_spAwards\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                defaultString: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                cvarFlags: 0x1 as i32,
            };
            init
        },
        {
            let mut init = cvarTable_t {
                vmCvar: &ui_spVideos as *const vmCvar_t as *mut vmCvar_t,
                cvarName: b"g_spVideos\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                defaultString: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                cvarFlags: 0x1 as i32,
            };
            init
        },
        {
            let mut init = cvarTable_t {
                vmCvar: &ui_spSkill as *const vmCvar_t as *mut vmCvar_t,
                cvarName: b"g_spSkill\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                defaultString: b"2\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                cvarFlags: 0x1 as i32 | 0x20 as i32,
            };
            init
        },
        {
            let mut init = cvarTable_t {
                vmCvar: &ui_spSelection as *const vmCvar_t as *mut vmCvar_t,
                cvarName: b"ui_spSelection\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                defaultString: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                cvarFlags: 0x40 as i32,
            };
            init
        },
        {
            let mut init = cvarTable_t {
                vmCvar: &ui_browserMaster as *const vmCvar_t as *mut vmCvar_t,
                cvarName: b"ui_browserMaster\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                defaultString: b"1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                cvarFlags: 0x1 as i32,
            };
            init
        },
        {
            let mut init = cvarTable_t {
                vmCvar: &ui_browserGameType as *const vmCvar_t as *mut vmCvar_t,
                cvarName: b"ui_browserGameType\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                defaultString: b"0\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                cvarFlags: 0x1 as i32,
            };
            init
        },
        {
            let mut init = cvarTable_t {
                vmCvar: &ui_browserSortKey as *const vmCvar_t as *mut vmCvar_t,
                cvarName: b"ui_browserSortKey\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                defaultString: b"4\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                cvarFlags: 0x1 as i32,
            };
            init
        },
        {
            let mut init = cvarTable_t {
                vmCvar: &ui_browserShowFull as *const vmCvar_t as *mut vmCvar_t,
                cvarName: b"ui_browserShowFull\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                defaultString: b"1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                cvarFlags: 0x1 as i32,
            };
            init
        },
        {
            let mut init = cvarTable_t {
                vmCvar: &ui_browserShowEmpty as *const vmCvar_t as *mut vmCvar_t,
                cvarName: b"ui_browserShowEmpty\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                defaultString: b"1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                cvarFlags: 0x1 as i32,
            };
            init
        },
        {
            let mut init = cvarTable_t {
                vmCvar: &ui_brassTime as *const vmCvar_t as *mut vmCvar_t,
                cvarName: b"cg_brassTime\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                defaultString: b"2500\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                cvarFlags: 0x1 as i32,
            };
            init
        },
        {
            let mut init = cvarTable_t {
                vmCvar: &ui_drawCrosshair as *const vmCvar_t as *mut vmCvar_t,
                cvarName: b"cg_drawCrosshair\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                defaultString: b"4\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                cvarFlags: 0x1 as i32,
            };
            init
        },
        {
            let mut init = cvarTable_t {
                vmCvar: &ui_drawCrosshairNames as *const vmCvar_t as *mut vmCvar_t,
                cvarName: b"cg_drawCrosshairNames\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                defaultString: b"1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                cvarFlags: 0x1 as i32,
            };
            init
        },
        {
            let mut init = cvarTable_t {
                vmCvar: &ui_marks as *const vmCvar_t as *mut vmCvar_t,
                cvarName: b"cg_marks\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                defaultString: b"1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                cvarFlags: 0x1 as i32,
            };
            init
        },
        {
            let mut init = cvarTable_t {
                vmCvar: &ui_server1 as *const vmCvar_t as *mut vmCvar_t,
                cvarName: b"server1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                defaultString: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                cvarFlags: 0x1 as i32,
            };
            init
        },
        {
            let mut init = cvarTable_t {
                vmCvar: &ui_server2 as *const vmCvar_t as *mut vmCvar_t,
                cvarName: b"server2\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                defaultString: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                cvarFlags: 0x1 as i32,
            };
            init
        },
        {
            let mut init = cvarTable_t {
                vmCvar: &ui_server3 as *const vmCvar_t as *mut vmCvar_t,
                cvarName: b"server3\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                defaultString: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                cvarFlags: 0x1 as i32,
            };
            init
        },
        {
            let mut init = cvarTable_t {
                vmCvar: &ui_server4 as *const vmCvar_t as *mut vmCvar_t,
                cvarName: b"server4\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                defaultString: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                cvarFlags: 0x1 as i32,
            };
            init
        },
        {
            let mut init = cvarTable_t {
                vmCvar: &ui_server5 as *const vmCvar_t as *mut vmCvar_t,
                cvarName: b"server5\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                defaultString: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                cvarFlags: 0x1 as i32,
            };
            init
        },
        {
            let mut init = cvarTable_t {
                vmCvar: &ui_server6 as *const vmCvar_t as *mut vmCvar_t,
                cvarName: b"server6\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                defaultString: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                cvarFlags: 0x1 as i32,
            };
            init
        },
        {
            let mut init = cvarTable_t {
                vmCvar: &ui_server7 as *const vmCvar_t as *mut vmCvar_t,
                cvarName: b"server7\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                defaultString: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                cvarFlags: 0x1 as i32,
            };
            init
        },
        {
            let mut init = cvarTable_t {
                vmCvar: &ui_server8 as *const vmCvar_t as *mut vmCvar_t,
                cvarName: b"server8\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                defaultString: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                cvarFlags: 0x1 as i32,
            };
            init
        },
        {
            let mut init = cvarTable_t {
                vmCvar: &ui_server9 as *const vmCvar_t as *mut vmCvar_t,
                cvarName: b"server9\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                defaultString: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                cvarFlags: 0x1 as i32,
            };
            init
        },
        {
            let mut init = cvarTable_t {
                vmCvar: &ui_server10 as *const vmCvar_t as *mut vmCvar_t,
                cvarName: b"server10\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                defaultString: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                cvarFlags: 0x1 as i32,
            };
            init
        },
        {
            let mut init = cvarTable_t {
                vmCvar: &ui_server11 as *const vmCvar_t as *mut vmCvar_t,
                cvarName: b"server11\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                defaultString: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                cvarFlags: 0x1 as i32,
            };
            init
        },
        {
            let mut init = cvarTable_t {
                vmCvar: &ui_server12 as *const vmCvar_t as *mut vmCvar_t,
                cvarName: b"server12\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                defaultString: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                cvarFlags: 0x1 as i32,
            };
            init
        },
        {
            let mut init = cvarTable_t {
                vmCvar: &ui_server13 as *const vmCvar_t as *mut vmCvar_t,
                cvarName: b"server13\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                defaultString: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                cvarFlags: 0x1 as i32,
            };
            init
        },
        {
            let mut init = cvarTable_t {
                vmCvar: &ui_server14 as *const vmCvar_t as *mut vmCvar_t,
                cvarName: b"server14\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                defaultString: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                cvarFlags: 0x1 as i32,
            };
            init
        },
        {
            let mut init = cvarTable_t {
                vmCvar: &ui_server15 as *const vmCvar_t as *mut vmCvar_t,
                cvarName: b"server15\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                defaultString: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                cvarFlags: 0x1 as i32,
            };
            init
        },
        {
            let mut init = cvarTable_t {
                vmCvar: &ui_server16 as *const vmCvar_t as *mut vmCvar_t,
                cvarName: b"server16\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                defaultString: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                cvarFlags: 0x1 as i32,
            };
            init
        },
        {
            let mut init = cvarTable_t {
                vmCvar: &ui_cdkeychecked as *const vmCvar_t as *mut vmCvar_t,
                cvarName: b"ui_cdkeychecked\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                defaultString: b"0\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                cvarFlags: 0x40 as i32,
            };
            init
        },
        {
            let mut init = cvarTable_t {
                vmCvar: &ui_ioq3 as *const vmCvar_t as *mut vmCvar_t,
                cvarName: b"ui_ioq3\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                defaultString: b"1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                cvarFlags: 0x40 as i32,
            };
            init
        },
        {
            let mut init = cvarTable_t {
                vmCvar: std::ptr::null_mut(),
                cvarName: b"g_localTeamPref\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                defaultString: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                cvarFlags: 0 as i32,
            };
            init
        },
    ]
};
// Initialized in run_static_initializers

static mut cvarTableSize: i32 = 0;
/*
=================
UI_RegisterCvars
=================
*/
#[no_mangle]

pub unsafe extern "C" fn UI_RegisterCvars() {
    let mut i: i32 = 0;
    let mut cv: *mut cvarTable_t = std::ptr::null_mut();
    i = 0 as i32;
    cv = cvarTable.as_mut_ptr();
    while i < cvarTableSize {
        crate::src::ui::ui_syscalls::trap_Cvar_Register(
            (*cv).vmCvar as *mut vmCvar_t,
            (*cv).cvarName,
            (*cv).defaultString,
            (*cv).cvarFlags,
        );
        i += 1;
        cv = cv.offset(1)
    }
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
/*
=================
UI_UpdateCvars
=================
*/
#[no_mangle]

pub unsafe extern "C" fn UI_UpdateCvars() {
    let mut i: i32 = 0;
    let mut cv: *mut cvarTable_t = std::ptr::null_mut();
    i = 0 as i32;
    cv = cvarTable.as_mut_ptr();
    while i < cvarTableSize {
        if !(*cv).vmCvar.is_null() {
            crate::src::ui::ui_syscalls::trap_Cvar_Update((*cv).vmCvar as *mut vmCvar_t);
        }
        i += 1;
        cv = cv.offset(1)
    }
}
unsafe extern "C" fn run_static_initializers() {
    cvarTableSize = (::std::mem::size_of::<[cvarTable_t; 49]>() as usize)
        .wrapping_div(::std::mem::size_of::<cvarTable_t>() as usize) as i32
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];

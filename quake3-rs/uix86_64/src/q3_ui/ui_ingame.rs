use ::libc;

pub mod stdlib_h {
    #[inline]

    pub unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
        return ::libc::strtol(
            __nptr,
            0 as *mut libc::c_void as *mut *mut libc::c_char,
            10 as libc::c_int,
        ) as libc::c_int;
    }
}

pub use crate::src::qcommon::q_shared::connstate_t;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qhandle_t;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::sfxHandle_t;
pub use crate::src::qcommon::q_shared::vec4_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::qcommon::q_shared::C2RustUnnamed_0;
pub use crate::src::qcommon::q_shared::Info_ValueForKey;
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
pub use crate::src::qcommon::q_shared::EXEC_APPEND;
pub use crate::src::qcommon::q_shared::EXEC_INSERT;
pub use crate::src::qcommon::q_shared::EXEC_NOW;
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
pub use crate::ui_public_h::uiClientState_t;

pub use crate::bg_public_h::GT_1FCTF;
pub use crate::bg_public_h::GT_CTF;
pub use crate::bg_public_h::GT_FFA;
pub use crate::bg_public_h::GT_HARVESTER;
pub use crate::bg_public_h::GT_MAX_GAME_TYPE;
pub use crate::bg_public_h::GT_OBELISK;
pub use crate::bg_public_h::GT_SINGLE_PLAYER;
pub use crate::bg_public_h::GT_TEAM;
pub use crate::bg_public_h::GT_TOURNAMENT;
pub use crate::bg_public_h::TEAM_BLUE;
pub use crate::bg_public_h::TEAM_FREE;
pub use crate::bg_public_h::TEAM_NUM_TEAMS;
pub use crate::bg_public_h::TEAM_RED;
pub use crate::bg_public_h::TEAM_SPECTATOR;
pub use crate::src::q3_ui::ui_addbots::UI_AddBotsMenu;
pub use crate::src::q3_ui::ui_atoms::uis;
pub use crate::src::q3_ui::ui_atoms::UI_PopMenu;
pub use crate::src::q3_ui::ui_atoms::UI_PushMenu;
pub use crate::src::q3_ui::ui_confirm::UI_ConfirmMenu;
pub use crate::src::q3_ui::ui_credits::UI_CreditMenu;
pub use crate::src::q3_ui::ui_ingame::stdlib_h::atoi;
pub use crate::src::q3_ui::ui_qmenu::color_red;
pub use crate::src::q3_ui::ui_qmenu::Menu_AddItem;
pub use crate::src::q3_ui::ui_removebots::UI_RemoveBotsMenu;
pub use crate::src::q3_ui::ui_serverinfo::UI_ServerInfoMenu;
pub use crate::src::q3_ui::ui_setup::UI_SetupMenu;
pub use crate::src::q3_ui::ui_team::UI_TeamMainMenu;
pub use crate::src::q3_ui::ui_teamorders::UI_TeamOrdersMenu;
pub use crate::src::ui::ui_syscalls::trap_Cmd_ExecuteText;
pub use crate::src::ui::ui_syscalls::trap_Cvar_VariableValue;
pub use crate::src::ui::ui_syscalls::trap_GetClientState;
pub use crate::src::ui::ui_syscalls::trap_GetConfigString;
pub use crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip;

pub use crate::ui_local_h::_tag_menuframework;
pub use crate::ui_local_h::menubitmap_s;
pub use crate::ui_local_h::menucommon_s;
pub use crate::ui_local_h::menuframework_s;
pub use crate::ui_local_h::menutext_s;
pub use crate::ui_local_h::uiStatic_t;
pub use ::libc::strtol;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ingamemenu_t {
    pub menu: crate::ui_local_h::menuframework_s,
    pub frame: crate::ui_local_h::menubitmap_s,
    pub team: crate::ui_local_h::menutext_s,
    pub setup: crate::ui_local_h::menutext_s,
    pub server: crate::ui_local_h::menutext_s,
    pub leave: crate::ui_local_h::menutext_s,
    pub restart: crate::ui_local_h::menutext_s,
    pub addbots: crate::ui_local_h::menutext_s,
    pub removebots: crate::ui_local_h::menutext_s,
    pub teamorders: crate::ui_local_h::menutext_s,
    pub quit: crate::ui_local_h::menutext_s,
    pub resume: crate::ui_local_h::menutext_s,
}

static mut s_ingame: ingamemenu_t = ingamemenu_t {
    menu: crate::ui_local_h::menuframework_s {
        cursor: 0,
        cursor_prev: 0,
        nitems: 0,
        items: [0 as *const libc::c_void as *mut libc::c_void; 64],
        draw: None,
        key: None,
        wrapAround: crate::src::qcommon::q_shared::qfalse,
        fullscreen: crate::src::qcommon::q_shared::qfalse,
        showlogo: crate::src::qcommon::q_shared::qfalse,
    },
    frame: crate::ui_local_h::menubitmap_s {
        generic: crate::ui_local_h::menucommon_s {
            type_0: 0,
            name: 0 as *const libc::c_char,
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: 0 as *const crate::ui_local_h::menuframework_s
                as *mut crate::ui_local_h::menuframework_s,
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
        focuspic: 0 as *const libc::c_char as *mut libc::c_char,
        errorpic: 0 as *const libc::c_char as *mut libc::c_char,
        shader: 0,
        focusshader: 0,
        width: 0,
        height: 0,
        focuscolor: 0 as *const libc::c_float as *mut libc::c_float,
    },
    team: crate::ui_local_h::menutext_s {
        generic: crate::ui_local_h::menucommon_s {
            type_0: 0,
            name: 0 as *const libc::c_char,
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: 0 as *const crate::ui_local_h::menuframework_s
                as *mut crate::ui_local_h::menuframework_s,
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
        string: 0 as *const libc::c_char as *mut libc::c_char,
        style: 0,
        color: 0 as *const libc::c_float as *mut libc::c_float,
    },
    setup: crate::ui_local_h::menutext_s {
        generic: crate::ui_local_h::menucommon_s {
            type_0: 0,
            name: 0 as *const libc::c_char,
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: 0 as *const crate::ui_local_h::menuframework_s
                as *mut crate::ui_local_h::menuframework_s,
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
        string: 0 as *const libc::c_char as *mut libc::c_char,
        style: 0,
        color: 0 as *const libc::c_float as *mut libc::c_float,
    },
    server: crate::ui_local_h::menutext_s {
        generic: crate::ui_local_h::menucommon_s {
            type_0: 0,
            name: 0 as *const libc::c_char,
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: 0 as *const crate::ui_local_h::menuframework_s
                as *mut crate::ui_local_h::menuframework_s,
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
        string: 0 as *const libc::c_char as *mut libc::c_char,
        style: 0,
        color: 0 as *const libc::c_float as *mut libc::c_float,
    },
    leave: crate::ui_local_h::menutext_s {
        generic: crate::ui_local_h::menucommon_s {
            type_0: 0,
            name: 0 as *const libc::c_char,
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: 0 as *const crate::ui_local_h::menuframework_s
                as *mut crate::ui_local_h::menuframework_s,
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
        string: 0 as *const libc::c_char as *mut libc::c_char,
        style: 0,
        color: 0 as *const libc::c_float as *mut libc::c_float,
    },
    restart: crate::ui_local_h::menutext_s {
        generic: crate::ui_local_h::menucommon_s {
            type_0: 0,
            name: 0 as *const libc::c_char,
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: 0 as *const crate::ui_local_h::menuframework_s
                as *mut crate::ui_local_h::menuframework_s,
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
        string: 0 as *const libc::c_char as *mut libc::c_char,
        style: 0,
        color: 0 as *const libc::c_float as *mut libc::c_float,
    },
    addbots: crate::ui_local_h::menutext_s {
        generic: crate::ui_local_h::menucommon_s {
            type_0: 0,
            name: 0 as *const libc::c_char,
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: 0 as *const crate::ui_local_h::menuframework_s
                as *mut crate::ui_local_h::menuframework_s,
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
        string: 0 as *const libc::c_char as *mut libc::c_char,
        style: 0,
        color: 0 as *const libc::c_float as *mut libc::c_float,
    },
    removebots: crate::ui_local_h::menutext_s {
        generic: crate::ui_local_h::menucommon_s {
            type_0: 0,
            name: 0 as *const libc::c_char,
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: 0 as *const crate::ui_local_h::menuframework_s
                as *mut crate::ui_local_h::menuframework_s,
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
        string: 0 as *const libc::c_char as *mut libc::c_char,
        style: 0,
        color: 0 as *const libc::c_float as *mut libc::c_float,
    },
    teamorders: crate::ui_local_h::menutext_s {
        generic: crate::ui_local_h::menucommon_s {
            type_0: 0,
            name: 0 as *const libc::c_char,
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: 0 as *const crate::ui_local_h::menuframework_s
                as *mut crate::ui_local_h::menuframework_s,
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
        string: 0 as *const libc::c_char as *mut libc::c_char,
        style: 0,
        color: 0 as *const libc::c_float as *mut libc::c_float,
    },
    quit: crate::ui_local_h::menutext_s {
        generic: crate::ui_local_h::menucommon_s {
            type_0: 0,
            name: 0 as *const libc::c_char,
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: 0 as *const crate::ui_local_h::menuframework_s
                as *mut crate::ui_local_h::menuframework_s,
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
        string: 0 as *const libc::c_char as *mut libc::c_char,
        style: 0,
        color: 0 as *const libc::c_float as *mut libc::c_float,
    },
    resume: crate::ui_local_h::menutext_s {
        generic: crate::ui_local_h::menucommon_s {
            type_0: 0,
            name: 0 as *const libc::c_char,
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: 0 as *const crate::ui_local_h::menuframework_s
                as *mut crate::ui_local_h::menuframework_s,
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
        string: 0 as *const libc::c_char as *mut libc::c_char,
        style: 0,
        color: 0 as *const libc::c_float as *mut libc::c_float,
    },
};
/*
=================
InGame_RestartAction
=================
*/

unsafe extern "C" fn InGame_RestartAction(mut result: crate::src::qcommon::q_shared::qboolean) {
    if result as u64 == 0 {
        return;
    }
    crate::src::q3_ui::ui_atoms::UI_PopMenu();
    crate::src::ui::ui_syscalls::trap_Cmd_ExecuteText(
        crate::src::qcommon::q_shared::EXEC_APPEND as libc::c_int,
        b"map_restart 0\n\x00" as *const u8 as *const libc::c_char,
    );
}
/*
=================
InGame_QuitAction
=================
*/

unsafe extern "C" fn InGame_QuitAction(mut result: crate::src::qcommon::q_shared::qboolean) {
    if result as u64 == 0 {
        return;
    }
    crate::src::q3_ui::ui_atoms::UI_PopMenu();
    crate::src::q3_ui::ui_credits::UI_CreditMenu();
}
/*
=================
InGame_Event
=================
*/
#[no_mangle]

pub unsafe extern "C" fn InGame_Event(mut ptr: *mut libc::c_void, mut notification: libc::c_int) {
    if notification != 3 as libc::c_int {
        return;
    }
    match (*(ptr as *mut crate::ui_local_h::menucommon_s)).id {
        10 => {
            crate::src::q3_ui::ui_team::UI_TeamMainMenu();
        }
        13 => {
            crate::src::q3_ui::ui_setup::UI_SetupMenu();
        }
        15 => {
            crate::src::ui::ui_syscalls::trap_Cmd_ExecuteText(
                crate::src::qcommon::q_shared::EXEC_APPEND as libc::c_int,
                b"disconnect\n\x00" as *const u8 as *const libc::c_char,
            );
        }
        16 => {
            crate::src::q3_ui::ui_confirm::UI_ConfirmMenu(
                b"RESTART ARENA?\x00" as *const u8 as *const libc::c_char,
                None,
                Some(
                    InGame_RestartAction
                        as unsafe extern "C" fn(_: crate::src::qcommon::q_shared::qboolean) -> (),
                ),
            );
        }
        17 => {
            crate::src::q3_ui::ui_confirm::UI_ConfirmMenu(
                b"EXIT GAME?\x00" as *const u8 as *const libc::c_char,
                None,
                Some(
                    InGame_QuitAction
                        as unsafe extern "C" fn(_: crate::src::qcommon::q_shared::qboolean) -> (),
                ),
            );
        }
        14 => {
            crate::src::q3_ui::ui_serverinfo::UI_ServerInfoMenu();
        }
        11 => {
            crate::src::q3_ui::ui_addbots::UI_AddBotsMenu();
        }
        12 => {
            crate::src::q3_ui::ui_removebots::UI_RemoveBotsMenu();
        }
        19 => {
            crate::src::q3_ui::ui_teamorders::UI_TeamOrdersMenu();
        }
        18 => {
            crate::src::q3_ui::ui_atoms::UI_PopMenu();
        }
        _ => {}
    };
}
/*
=================
InGame_MenuInit
=================
*/
#[no_mangle]

pub unsafe extern "C" fn InGame_MenuInit() {
    let mut y: libc::c_int = 0; //142;
    let mut cs: crate::ui_public_h::uiClientState_t = crate::ui_public_h::uiClientState_t {
        connState: crate::src::qcommon::q_shared::CA_UNINITIALIZED,
        connectPacketCount: 0,
        clientNum: 0,
        servername: [0; 1024],
        updateInfoString: [0; 1024],
        messageString: [0; 1024],
    }; //118;
    let mut info: [libc::c_char; 1024] = [0; 1024]; //359;
    let mut team: libc::c_int = 0; //256;
    crate::stdlib::memset(
        &mut s_ingame as *mut ingamemenu_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<ingamemenu_t>() as libc::c_ulong,
    );
    InGame_Cache();
    s_ingame.menu.wrapAround = crate::src::qcommon::q_shared::qtrue;
    s_ingame.menu.fullscreen = crate::src::qcommon::q_shared::qfalse;
    s_ingame.frame.generic.type_0 = 6 as libc::c_int;
    s_ingame.frame.generic.flags = 0x4000 as libc::c_int as libc::c_uint;
    s_ingame.frame.generic.name = b"menu/art/addbotframe\x00" as *const u8 as *const libc::c_char;
    s_ingame.frame.generic.x = 320 as libc::c_int - 233 as libc::c_int;
    s_ingame.frame.generic.y = 240 as libc::c_int - 166 as libc::c_int;
    s_ingame.frame.width = 466 as libc::c_int;
    s_ingame.frame.height = 332 as libc::c_int;
    //y = 96;
    y = 88 as libc::c_int;
    s_ingame.team.generic.type_0 = 9 as libc::c_int;
    s_ingame.team.generic.flags =
        0x8 as libc::c_int as libc::c_uint | 0x100 as libc::c_int as libc::c_uint;
    s_ingame.team.generic.x = 320 as libc::c_int;
    s_ingame.team.generic.y = y;
    s_ingame.team.generic.id = 10 as libc::c_int;
    s_ingame.team.generic.callback =
        Some(InGame_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: libc::c_int) -> ());
    s_ingame.team.string = b"START\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_ingame.team.color = crate::src::q3_ui::ui_qmenu::color_red.as_mut_ptr();
    s_ingame.team.style = 0x1 as libc::c_int | 0x10 as libc::c_int;
    y += 28 as libc::c_int;
    s_ingame.addbots.generic.type_0 = 9 as libc::c_int;
    s_ingame.addbots.generic.flags =
        0x8 as libc::c_int as libc::c_uint | 0x100 as libc::c_int as libc::c_uint;
    s_ingame.addbots.generic.x = 320 as libc::c_int;
    s_ingame.addbots.generic.y = y;
    s_ingame.addbots.generic.id = 11 as libc::c_int;
    s_ingame.addbots.generic.callback =
        Some(InGame_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: libc::c_int) -> ());
    s_ingame.addbots.string =
        b"ADD BOTS\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_ingame.addbots.color = crate::src::q3_ui::ui_qmenu::color_red.as_mut_ptr();
    s_ingame.addbots.style = 0x1 as libc::c_int | 0x10 as libc::c_int;
    if crate::src::ui::ui_syscalls::trap_Cvar_VariableValue(
        b"sv_running\x00" as *const u8 as *const libc::c_char,
    ) == 0.
        || crate::src::ui::ui_syscalls::trap_Cvar_VariableValue(
            b"bot_enable\x00" as *const u8 as *const libc::c_char,
        ) == 0.
        || crate::src::ui::ui_syscalls::trap_Cvar_VariableValue(
            b"g_gametype\x00" as *const u8 as *const libc::c_char,
        ) == crate::bg_public_h::GT_SINGLE_PLAYER as libc::c_int as libc::c_float
    {
        s_ingame.addbots.generic.flags |= 0x2000 as libc::c_int as libc::c_uint
    }
    y += 28 as libc::c_int;
    s_ingame.removebots.generic.type_0 = 9 as libc::c_int;
    s_ingame.removebots.generic.flags =
        0x8 as libc::c_int as libc::c_uint | 0x100 as libc::c_int as libc::c_uint;
    s_ingame.removebots.generic.x = 320 as libc::c_int;
    s_ingame.removebots.generic.y = y;
    s_ingame.removebots.generic.id = 12 as libc::c_int;
    s_ingame.removebots.generic.callback =
        Some(InGame_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: libc::c_int) -> ());
    s_ingame.removebots.string =
        b"REMOVE BOTS\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_ingame.removebots.color = crate::src::q3_ui::ui_qmenu::color_red.as_mut_ptr();
    s_ingame.removebots.style = 0x1 as libc::c_int | 0x10 as libc::c_int;
    if crate::src::ui::ui_syscalls::trap_Cvar_VariableValue(
        b"sv_running\x00" as *const u8 as *const libc::c_char,
    ) == 0.
        || crate::src::ui::ui_syscalls::trap_Cvar_VariableValue(
            b"bot_enable\x00" as *const u8 as *const libc::c_char,
        ) == 0.
        || crate::src::ui::ui_syscalls::trap_Cvar_VariableValue(
            b"g_gametype\x00" as *const u8 as *const libc::c_char,
        ) == crate::bg_public_h::GT_SINGLE_PLAYER as libc::c_int as libc::c_float
    {
        s_ingame.removebots.generic.flags |= 0x2000 as libc::c_int as libc::c_uint
    }
    y += 28 as libc::c_int;
    s_ingame.teamorders.generic.type_0 = 9 as libc::c_int;
    s_ingame.teamorders.generic.flags =
        0x8 as libc::c_int as libc::c_uint | 0x100 as libc::c_int as libc::c_uint;
    s_ingame.teamorders.generic.x = 320 as libc::c_int;
    s_ingame.teamorders.generic.y = y;
    s_ingame.teamorders.generic.id = 19 as libc::c_int;
    s_ingame.teamorders.generic.callback =
        Some(InGame_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: libc::c_int) -> ());
    s_ingame.teamorders.string =
        b"TEAM ORDERS\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_ingame.teamorders.color = crate::src::q3_ui::ui_qmenu::color_red.as_mut_ptr();
    s_ingame.teamorders.style = 0x1 as libc::c_int | 0x10 as libc::c_int;
    if !(crate::src::ui::ui_syscalls::trap_Cvar_VariableValue(
        b"g_gametype\x00" as *const u8 as *const libc::c_char,
    ) >= crate::bg_public_h::GT_TEAM as libc::c_int as libc::c_float)
    {
        s_ingame.teamorders.generic.flags |= 0x2000 as libc::c_int as libc::c_uint
    } else {
        crate::src::ui::ui_syscalls::trap_GetClientState(
            &mut cs as *mut _ as *mut crate::ui_public_h::uiClientState_t,
        );
        crate::src::ui::ui_syscalls::trap_GetConfigString(
            32 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int + cs.clientNum,
            info.as_mut_ptr(),
            1024 as libc::c_int,
        );
        team = atoi(crate::src::qcommon::q_shared::Info_ValueForKey(
            info.as_mut_ptr(),
            b"t\x00" as *const u8 as *const libc::c_char,
        ));
        if team == crate::bg_public_h::TEAM_SPECTATOR as libc::c_int {
            s_ingame.teamorders.generic.flags |= 0x2000 as libc::c_int as libc::c_uint
        }
    }
    y += 28 as libc::c_int;
    s_ingame.setup.generic.type_0 = 9 as libc::c_int;
    s_ingame.setup.generic.flags =
        0x8 as libc::c_int as libc::c_uint | 0x100 as libc::c_int as libc::c_uint;
    s_ingame.setup.generic.x = 320 as libc::c_int;
    s_ingame.setup.generic.y = y;
    s_ingame.setup.generic.id = 13 as libc::c_int;
    s_ingame.setup.generic.callback =
        Some(InGame_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: libc::c_int) -> ());
    s_ingame.setup.string = b"SETUP\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_ingame.setup.color = crate::src::q3_ui::ui_qmenu::color_red.as_mut_ptr();
    s_ingame.setup.style = 0x1 as libc::c_int | 0x10 as libc::c_int;
    y += 28 as libc::c_int;
    s_ingame.server.generic.type_0 = 9 as libc::c_int;
    s_ingame.server.generic.flags =
        0x8 as libc::c_int as libc::c_uint | 0x100 as libc::c_int as libc::c_uint;
    s_ingame.server.generic.x = 320 as libc::c_int;
    s_ingame.server.generic.y = y;
    s_ingame.server.generic.id = 14 as libc::c_int;
    s_ingame.server.generic.callback =
        Some(InGame_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: libc::c_int) -> ());
    s_ingame.server.string =
        b"SERVER INFO\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_ingame.server.color = crate::src::q3_ui::ui_qmenu::color_red.as_mut_ptr();
    s_ingame.server.style = 0x1 as libc::c_int | 0x10 as libc::c_int;
    y += 28 as libc::c_int;
    s_ingame.restart.generic.type_0 = 9 as libc::c_int;
    s_ingame.restart.generic.flags =
        0x8 as libc::c_int as libc::c_uint | 0x100 as libc::c_int as libc::c_uint;
    s_ingame.restart.generic.x = 320 as libc::c_int;
    s_ingame.restart.generic.y = y;
    s_ingame.restart.generic.id = 16 as libc::c_int;
    s_ingame.restart.generic.callback =
        Some(InGame_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: libc::c_int) -> ());
    s_ingame.restart.string =
        b"RESTART ARENA\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_ingame.restart.color = crate::src::q3_ui::ui_qmenu::color_red.as_mut_ptr();
    s_ingame.restart.style = 0x1 as libc::c_int | 0x10 as libc::c_int;
    if crate::src::ui::ui_syscalls::trap_Cvar_VariableValue(
        b"sv_running\x00" as *const u8 as *const libc::c_char,
    ) == 0.
    {
        s_ingame.restart.generic.flags |= 0x2000 as libc::c_int as libc::c_uint
    }
    y += 28 as libc::c_int;
    s_ingame.resume.generic.type_0 = 9 as libc::c_int;
    s_ingame.resume.generic.flags =
        0x8 as libc::c_int as libc::c_uint | 0x100 as libc::c_int as libc::c_uint;
    s_ingame.resume.generic.x = 320 as libc::c_int;
    s_ingame.resume.generic.y = y;
    s_ingame.resume.generic.id = 18 as libc::c_int;
    s_ingame.resume.generic.callback =
        Some(InGame_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: libc::c_int) -> ());
    s_ingame.resume.string =
        b"RESUME GAME\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_ingame.resume.color = crate::src::q3_ui::ui_qmenu::color_red.as_mut_ptr();
    s_ingame.resume.style = 0x1 as libc::c_int | 0x10 as libc::c_int;
    y += 28 as libc::c_int;
    s_ingame.leave.generic.type_0 = 9 as libc::c_int;
    s_ingame.leave.generic.flags =
        0x8 as libc::c_int as libc::c_uint | 0x100 as libc::c_int as libc::c_uint;
    s_ingame.leave.generic.x = 320 as libc::c_int;
    s_ingame.leave.generic.y = y;
    s_ingame.leave.generic.id = 15 as libc::c_int;
    s_ingame.leave.generic.callback =
        Some(InGame_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: libc::c_int) -> ());
    s_ingame.leave.string =
        b"LEAVE ARENA\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_ingame.leave.color = crate::src::q3_ui::ui_qmenu::color_red.as_mut_ptr();
    s_ingame.leave.style = 0x1 as libc::c_int | 0x10 as libc::c_int;
    y += 28 as libc::c_int;
    s_ingame.quit.generic.type_0 = 9 as libc::c_int;
    s_ingame.quit.generic.flags =
        0x8 as libc::c_int as libc::c_uint | 0x100 as libc::c_int as libc::c_uint;
    s_ingame.quit.generic.x = 320 as libc::c_int;
    s_ingame.quit.generic.y = y;
    s_ingame.quit.generic.id = 17 as libc::c_int;
    s_ingame.quit.generic.callback =
        Some(InGame_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: libc::c_int) -> ());
    s_ingame.quit.string =
        b"EXIT GAME\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_ingame.quit.color = crate::src::q3_ui::ui_qmenu::color_red.as_mut_ptr();
    s_ingame.quit.style = 0x1 as libc::c_int | 0x10 as libc::c_int;
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_ingame.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut s_ingame.frame as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_ingame.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut s_ingame.team as *mut crate::ui_local_h::menutext_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_ingame.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut s_ingame.addbots as *mut crate::ui_local_h::menutext_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_ingame.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut s_ingame.removebots as *mut crate::ui_local_h::menutext_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_ingame.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut s_ingame.teamorders as *mut crate::ui_local_h::menutext_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_ingame.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut s_ingame.setup as *mut crate::ui_local_h::menutext_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_ingame.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut s_ingame.server as *mut crate::ui_local_h::menutext_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_ingame.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut s_ingame.restart as *mut crate::ui_local_h::menutext_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_ingame.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut s_ingame.resume as *mut crate::ui_local_h::menutext_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_ingame.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut s_ingame.leave as *mut crate::ui_local_h::menutext_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_ingame.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut s_ingame.quit as *mut crate::ui_local_h::menutext_s as *mut libc::c_void,
    );
}
/*
=================
InGame_Cache
=================
*/
#[no_mangle]

pub unsafe extern "C" fn InGame_Cache() {
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/addbotframe\x00" as *const u8 as *const libc::c_char,
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
/*
=================
UI_InGameMenu
=================
*/
#[no_mangle]

pub unsafe extern "C" fn UI_InGameMenu() {
    // force as top level menu
    crate::src::q3_ui::ui_atoms::uis.menusp = 0 as libc::c_int;
    // set menu cursor to a nice location
    crate::src::q3_ui::ui_atoms::uis.cursorx = 319 as libc::c_int;
    crate::src::q3_ui::ui_atoms::uis.cursory = 80 as libc::c_int;
    InGame_MenuInit();
    crate::src::q3_ui::ui_atoms::UI_PushMenu(
        &mut s_ingame.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
    );
}

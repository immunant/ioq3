use ::libc;

pub use crate::src::q3_ui::ui_atoms::uis;
pub use crate::src::q3_ui::ui_atoms::UI_AdjustFrom640;
pub use crate::src::q3_ui::ui_atoms::UI_DrawProportionalString;
pub use crate::src::q3_ui::ui_atoms::UI_DrawProportionalString_AutoWrapped;
pub use crate::src::q3_ui::ui_atoms::UI_DrawString;
pub use crate::src::q3_ui::ui_atoms::UI_PopMenu;
pub use crate::src::q3_ui::ui_atoms::UI_PushMenu;
pub use crate::src::q3_ui::ui_cdkey::UI_CDKeyMenu;
pub use crate::src::q3_ui::ui_cinematics::UI_CinematicsMenu;
pub use crate::src::q3_ui::ui_confirm::UI_ConfirmMenu;
pub use crate::src::q3_ui::ui_credits::UI_CreditMenu;
pub use crate::src::q3_ui::ui_demo2::UI_DemosMenu;
pub use crate::src::q3_ui::ui_main::ui_cdkeychecked;
pub use crate::src::q3_ui::ui_mods::UI_ModsMenu;
pub use crate::src::q3_ui::ui_qmenu::color_red;
pub use crate::src::q3_ui::ui_qmenu::menu_null_sound;
pub use crate::src::q3_ui::ui_qmenu::menu_text_color;
pub use crate::src::q3_ui::ui_qmenu::Menu_AddItem;
pub use crate::src::q3_ui::ui_qmenu::Menu_Draw;
pub use crate::src::q3_ui::ui_servers2::UI_ArenaServersMenu;
pub use crate::src::q3_ui::ui_setup::UI_SetupMenu;
pub use crate::src::q3_ui::ui_splevel::UI_SPLevelMenu;
pub use crate::src::qcommon::q_math::AnglesToAxis;
pub use crate::src::qcommon::q_math::AxisClear;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::cvarHandle_t;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qhandle_t;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::sfxHandle_t;
pub use crate::src::qcommon::q_shared::vec3_t;
pub use crate::src::qcommon::q_shared::vec4_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::qcommon::q_shared::vmCvar_t;
pub use crate::src::qcommon::q_shared::C2RustUnnamed_0;
pub use crate::src::qcommon::q_shared::Q_stricmp;
pub use crate::src::qcommon::q_shared::EXEC_APPEND;
pub use crate::src::qcommon::q_shared::EXEC_INSERT;
pub use crate::src::qcommon::q_shared::EXEC_NOW;
pub use crate::src::ui::ui_syscalls::trap_Cmd_ExecuteText;
pub use crate::src::ui::ui_syscalls::trap_Cvar_Set;
pub use crate::src::ui::ui_syscalls::trap_Cvar_VariableStringBuffer;
pub use crate::src::ui::ui_syscalls::trap_FS_GetFileList;
pub use crate::src::ui::ui_syscalls::trap_GetCDKey;
pub use crate::src::ui::ui_syscalls::trap_Key_SetCatcher;
pub use crate::src::ui::ui_syscalls::trap_R_AddRefEntityToScene;
pub use crate::src::ui::ui_syscalls::trap_R_ClearScene;
pub use crate::src::ui::ui_syscalls::trap_R_RegisterModel;
pub use crate::src::ui::ui_syscalls::trap_R_RenderScene;
pub use crate::src::ui::ui_syscalls::trap_VerifyCDKey;

pub use crate::tr_types_h::glDriverType_t;
pub use crate::tr_types_h::glHardwareType_t;
pub use crate::tr_types_h::glconfig_t;
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
pub use crate::ui_local_h::_tag_menuframework;
pub use crate::ui_local_h::menucommon_s;
pub use crate::ui_local_h::menuframework_s;
pub use crate::ui_local_h::menutext_s;
pub use crate::ui_local_h::uiStatic_t;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mainmenu_t {
    pub menu: crate::ui_local_h::menuframework_s,
    pub singleplayer: crate::ui_local_h::menutext_s,
    pub multiplayer: crate::ui_local_h::menutext_s,
    pub setup: crate::ui_local_h::menutext_s,
    pub demos: crate::ui_local_h::menutext_s,
    pub cinematics: crate::ui_local_h::menutext_s,
    pub teamArena: crate::ui_local_h::menutext_s,
    pub mods: crate::ui_local_h::menutext_s,
    pub exit: crate::ui_local_h::menutext_s,
    pub bannerModel: crate::src::qcommon::q_shared::qhandle_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct errorMessage_t {
    pub menu: crate::ui_local_h::menuframework_s,
    pub errorMessage: [libc::c_char; 4096],
}

static mut s_main: mainmenu_t = mainmenu_t {
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
    singleplayer: crate::ui_local_h::menutext_s {
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
        color: 0 as *const f32 as *mut f32,
    },
    multiplayer: crate::ui_local_h::menutext_s {
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
        color: 0 as *const f32 as *mut f32,
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
        color: 0 as *const f32 as *mut f32,
    },
    demos: crate::ui_local_h::menutext_s {
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
        color: 0 as *const f32 as *mut f32,
    },
    cinematics: crate::ui_local_h::menutext_s {
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
        color: 0 as *const f32 as *mut f32,
    },
    teamArena: crate::ui_local_h::menutext_s {
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
        color: 0 as *const f32 as *mut f32,
    },
    mods: crate::ui_local_h::menutext_s {
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
        color: 0 as *const f32 as *mut f32,
    },
    exit: crate::ui_local_h::menutext_s {
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
        color: 0 as *const f32 as *mut f32,
    },
    bannerModel: 0,
};

static mut s_errorMessage: errorMessage_t = errorMessage_t {
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
    errorMessage: [0; 4096],
};
/*
=================
MainMenu_ExitAction
=================
*/

unsafe extern "C" fn MainMenu_ExitAction(mut result: crate::src::qcommon::q_shared::qboolean) {
    if result as u64 == 0 {
        return;
    }
    crate::src::q3_ui::ui_atoms::UI_PopMenu();
    crate::src::q3_ui::ui_credits::UI_CreditMenu();
}
/*
=================
Main_MenuEvent
=================
*/
#[no_mangle]

pub unsafe extern "C" fn Main_MenuEvent(mut ptr: *mut libc::c_void, mut event: i32) {
    if event != 3 as i32 {
        return;
    }
    match (*(ptr as *mut crate::ui_local_h::menucommon_s)).id {
        10 => {
            crate::src::q3_ui::ui_splevel::UI_SPLevelMenu();
        }
        11 => {
            crate::src::q3_ui::ui_servers2::UI_ArenaServersMenu();
        }
        12 => {
            crate::src::q3_ui::ui_setup::UI_SetupMenu();
        }
        13 => {
            crate::src::q3_ui::ui_demo2::UI_DemosMenu();
        }
        14 => {
            crate::src::q3_ui::ui_cinematics::UI_CinematicsMenu();
        }
        16 => {
            crate::src::q3_ui::ui_mods::UI_ModsMenu();
        }
        15 => {
            crate::src::ui::ui_syscalls::trap_Cvar_Set(
                b"fs_game\x00" as *const u8 as *const libc::c_char,
                b"missionpack\x00" as *const u8 as *const libc::c_char,
            );
            crate::src::ui::ui_syscalls::trap_Cmd_ExecuteText(
                crate::src::qcommon::q_shared::EXEC_APPEND as i32,
                b"vid_restart;\x00" as *const u8 as *const libc::c_char,
            );
        }
        17 => {
            crate::src::q3_ui::ui_confirm::UI_ConfirmMenu(
                b"EXIT GAME?\x00" as *const u8 as *const libc::c_char,
                None,
                Some(
                    MainMenu_ExitAction
                        as unsafe extern "C" fn(_: crate::src::qcommon::q_shared::qboolean) -> (),
                ),
            );
        }
        _ => {}
    };
}
/*
===============
MainMenu_Cache
===============
*/
#[no_mangle]

pub unsafe extern "C" fn MainMenu_Cache() {
    s_main.bannerModel = crate::src::ui::ui_syscalls::trap_R_RegisterModel(
        b"models/mapobjects/banner/banner5.md3\x00" as *const u8 as *const libc::c_char,
    );
}
#[no_mangle]

pub unsafe extern "C" fn ErrorMessage_Key(
    mut _key: i32,
) -> crate::src::qcommon::q_shared::sfxHandle_t {
    crate::src::ui::ui_syscalls::trap_Cvar_Set(
        b"com_errorMessage\x00" as *const u8 as *const libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char,
    );
    UI_MainMenu();
    return crate::src::q3_ui::ui_qmenu::menu_null_sound;
}
/*
===============
Main_MenuDraw
TTimo: this function is common to the main menu and errorMessage menu
===============
*/

unsafe extern "C" fn Main_MenuDraw() {
    let mut refdef: crate::tr_types_h::refdef_t = crate::tr_types_h::refdef_t {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
        fov_x: 0.,
        fov_y: 0.,
        vieworg: [0.; 3],
        viewaxis: [[0.; 3]; 3],
        time: 0,
        rdflags: 0,
        areamask: [0; 32],
        text: [[0; 32]; 8],
    };
    let mut ent: crate::tr_types_h::refEntity_t = crate::tr_types_h::refEntity_t {
        reType: crate::tr_types_h::RT_MODEL,
        renderfx: 0,
        hModel: 0,
        lightingOrigin: [0.; 3],
        shadowPlane: 0.,
        axis: [[0.; 3]; 3],
        nonNormalizedAxes: crate::src::qcommon::q_shared::qfalse,
        origin: [0.; 3],
        frame: 0,
        oldorigin: [0.; 3],
        oldframe: 0,
        backlerp: 0.,
        skinNum: 0,
        customSkin: 0,
        customShader: 0,
        shaderRGBA: [0; 4],
        shaderTexCoord: [0.; 2],
        shaderTime: 0.,
        radius: 0.,
        rotation: 0.,
    };
    let mut origin: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut angles: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut adjust: f32 = 0.;
    let mut x: f32 = 0.;
    let mut y: f32 = 0.;
    let mut w: f32 = 0.;
    let mut h: f32 = 0.;
    let mut color: crate::src::qcommon::q_shared::vec4_t = [
        0.5f64 as crate::src::qcommon::q_shared::vec_t,
        0 as i32 as crate::src::qcommon::q_shared::vec_t,
        0 as i32 as crate::src::qcommon::q_shared::vec_t,
        1 as i32 as crate::src::qcommon::q_shared::vec_t,
    ];
    // setup the refdef
    crate::stdlib::memset(
        &mut refdef as *mut crate::tr_types_h::refdef_t as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<crate::tr_types_h::refdef_t>() as libc::c_ulong,
    ); // JDC: Kenneth asked me to stop this 1.0 * sin( (float)uis.realtime / 1000 );
    refdef.rdflags = 0x1 as i32;
    crate::src::qcommon::q_math::AxisClear(refdef.viewaxis.as_mut_ptr());
    x = 0 as i32 as f32;
    y = 0 as i32 as f32;
    w = 640 as i32 as f32;
    h = 120 as i32 as f32;
    crate::src::q3_ui::ui_atoms::UI_AdjustFrom640(&mut x, &mut y, &mut w, &mut h);
    refdef.x = x as i32;
    refdef.y = y as i32;
    refdef.width = w as i32;
    refdef.height = h as i32;
    adjust = 0 as i32 as f32;
    refdef.fov_x = 60 as i32 as f32 + adjust;
    refdef.fov_y = (19.6875f64 + adjust as f64) as f32;
    refdef.time = crate::src::q3_ui::ui_atoms::uis.realtime;
    origin[0 as i32 as usize] = 300 as i32 as crate::src::qcommon::q_shared::vec_t;
    origin[1 as i32 as usize] = 0 as i32 as crate::src::qcommon::q_shared::vec_t;
    origin[2 as i32 as usize] = -(32 as i32) as crate::src::qcommon::q_shared::vec_t;
    crate::src::ui::ui_syscalls::trap_R_ClearScene();
    // add the model
    crate::stdlib::memset(
        &mut ent as *mut crate::tr_types_h::refEntity_t as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<crate::tr_types_h::refEntity_t>() as libc::c_ulong,
    );
    adjust = (5.0f64
        * crate::stdlib::sin(
            (crate::src::q3_ui::ui_atoms::uis.realtime as f32 / 5000 as i32 as f32) as f64,
        )) as f32;
    angles[0 as i32 as usize] = 0 as i32 as crate::src::qcommon::q_shared::vec_t;
    angles[1 as i32 as usize] = 180 as i32 as f32 + adjust;
    angles[2 as i32 as usize] = 0 as i32 as crate::src::qcommon::q_shared::vec_t;
    crate::src::qcommon::q_math::AnglesToAxis(
        angles.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        ent.axis.as_mut_ptr(),
    );
    ent.hModel = s_main.bannerModel;
    ent.origin[0 as i32 as usize] = origin[0 as i32 as usize];
    ent.origin[1 as i32 as usize] = origin[1 as i32 as usize];
    ent.origin[2 as i32 as usize] = origin[2 as i32 as usize];
    ent.lightingOrigin[0 as i32 as usize] = origin[0 as i32 as usize];
    ent.lightingOrigin[1 as i32 as usize] = origin[1 as i32 as usize];
    ent.lightingOrigin[2 as i32 as usize] = origin[2 as i32 as usize];
    ent.renderfx = 0x80 as i32 | 0x40 as i32;
    ent.oldorigin[0 as i32 as usize] = ent.origin[0 as i32 as usize];
    ent.oldorigin[1 as i32 as usize] = ent.origin[1 as i32 as usize];
    ent.oldorigin[2 as i32 as usize] = ent.origin[2 as i32 as usize];
    crate::src::ui::ui_syscalls::trap_R_AddRefEntityToScene(
        &mut ent as *mut _ as *const crate::tr_types_h::refEntity_t,
    );
    crate::src::ui::ui_syscalls::trap_R_RenderScene(
        &mut refdef as *mut _ as *const crate::tr_types_h::refdef_t,
    );
    if crate::stdlib::strlen(s_errorMessage.errorMessage.as_mut_ptr()) != 0 {
        crate::src::q3_ui::ui_atoms::UI_DrawProportionalString_AutoWrapped(
            320 as i32,
            192 as i32,
            600 as i32,
            20 as i32,
            s_errorMessage.errorMessage.as_mut_ptr(),
            0x1 as i32 | 0x10 as i32 | 0x800 as i32,
            crate::src::q3_ui::ui_qmenu::menu_text_color.as_mut_ptr(),
        );
    } else {
        // standard menu drawing
        crate::src::q3_ui::ui_qmenu::Menu_Draw(
            &mut s_main.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        );
    }
    if crate::src::q3_ui::ui_atoms::uis.demoversion as u64 != 0 {
        crate::src::q3_ui::ui_atoms::UI_DrawProportionalString(
            320 as i32,
            372 as i32,
            b"DEMO      FOR MATURE AUDIENCES      DEMO\x00" as *const u8 as *const libc::c_char,
            0x1 as i32 | 0x10 as i32,
            color.as_mut_ptr(),
        );
        crate::src::q3_ui::ui_atoms::UI_DrawString(
            320 as i32,
            400 as i32,
            b"Quake III Arena(c) 1999-2000, Id Software, Inc.  All Rights Reserved\x00" as *const u8
                as *const libc::c_char,
            0x1 as i32 | 0x10 as i32,
            color.as_mut_ptr(),
        );
    } else {
        crate::src::q3_ui::ui_atoms::UI_DrawString(
            320 as i32,
            450 as i32,
            b"Quake III Arena(c) 1999-2000, Id Software, Inc.  All Rights Reserved\x00" as *const u8
                as *const libc::c_char,
            0x1 as i32 | 0x10 as i32,
            color.as_mut_ptr(),
        );
    };
}
/*
===============
UI_TeamArenaExists
===============
*/

unsafe extern "C" fn UI_TeamArenaExists() -> crate::src::qcommon::q_shared::qboolean {
    let mut numdirs: i32 = 0;
    let mut dirlist: [libc::c_char; 2048] = [0; 2048];
    let mut dirptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut descptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: i32 = 0;
    let mut dirlen: i32 = 0;
    numdirs = crate::src::ui::ui_syscalls::trap_FS_GetFileList(
        b"$modlist\x00" as *const u8 as *const libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char,
        dirlist.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 2048]>() as libc::c_ulong as i32,
    );
    dirptr = dirlist.as_mut_ptr();
    i = 0 as i32;
    while i < numdirs {
        dirlen = crate::stdlib::strlen(dirptr).wrapping_add(1 as i32 as libc::c_ulong) as i32;
        descptr = dirptr.offset(dirlen as isize);
        if crate::src::qcommon::q_shared::Q_stricmp(
            dirptr,
            b"missionpack\x00" as *const u8 as *const libc::c_char,
        ) == 0 as i32
        {
            return crate::src::qcommon::q_shared::qtrue;
        }
        dirptr = dirptr.offset(
            (dirlen as libc::c_ulong)
                .wrapping_add(crate::stdlib::strlen(descptr))
                .wrapping_add(1 as i32 as libc::c_ulong) as isize,
        );
        i += 1
    }
    return crate::src::qcommon::q_shared::qfalse;
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
===============
UI_MainMenu

The main menu only comes up when not in a game,
so make sure that the attract loop server is down
and that local cinematics are killed
===============
*/
#[no_mangle]

pub unsafe extern "C" fn UI_MainMenu() {
    let mut y: i32 = 0;
    let mut teamArena: crate::src::qcommon::q_shared::qboolean =
        crate::src::qcommon::q_shared::qfalse;
    let mut style: i32 = 0x1 as i32 | 0x800 as i32;
    crate::src::ui::ui_syscalls::trap_Cvar_Set(
        b"sv_killserver\x00" as *const u8 as *const libc::c_char,
        b"1\x00" as *const u8 as *const libc::c_char,
    );
    if crate::src::q3_ui::ui_atoms::uis.demoversion as u64 == 0
        && crate::src::q3_ui::ui_main::ui_cdkeychecked.integer == 0
    {
        let mut key: [libc::c_char; 17] = [0; 17];
        crate::src::ui::ui_syscalls::trap_GetCDKey(
            key.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong as i32,
        );
        if crate::src::ui::ui_syscalls::trap_VerifyCDKey(key.as_mut_ptr(), 0 as *const libc::c_char)
            as u32
            == crate::src::qcommon::q_shared::qfalse as i32 as u32
        {
            crate::src::q3_ui::ui_cdkey::UI_CDKeyMenu();
            return;
        }
    }
    crate::stdlib::memset(
        &mut s_main as *mut mainmenu_t as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<mainmenu_t>() as libc::c_ulong,
    );
    crate::stdlib::memset(
        &mut s_errorMessage as *mut errorMessage_t as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<errorMessage_t>() as libc::c_ulong,
    );
    // com_errorMessage would need that too
    MainMenu_Cache();
    crate::src::ui::ui_syscalls::trap_Cvar_VariableStringBuffer(
        b"com_errorMessage\x00" as *const u8 as *const libc::c_char,
        s_errorMessage.errorMessage.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong as i32,
    );
    if crate::stdlib::strlen(s_errorMessage.errorMessage.as_mut_ptr()) != 0 {
        s_errorMessage.menu.draw = Some(Main_MenuDraw as unsafe extern "C" fn() -> ());
        s_errorMessage.menu.key = Some(
            ErrorMessage_Key
                as unsafe extern "C" fn(_: i32) -> crate::src::qcommon::q_shared::sfxHandle_t,
        );
        s_errorMessage.menu.fullscreen = crate::src::qcommon::q_shared::qtrue;
        s_errorMessage.menu.wrapAround = crate::src::qcommon::q_shared::qtrue;
        s_errorMessage.menu.showlogo = crate::src::qcommon::q_shared::qtrue;
        crate::src::ui::ui_syscalls::trap_Key_SetCatcher(0x2 as i32);
        crate::src::q3_ui::ui_atoms::uis.menusp = 0 as i32;
        crate::src::q3_ui::ui_atoms::UI_PushMenu(
            &mut s_errorMessage.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        );
        return;
    }
    s_main.menu.draw = Some(Main_MenuDraw as unsafe extern "C" fn() -> ());
    s_main.menu.fullscreen = crate::src::qcommon::q_shared::qtrue;
    s_main.menu.wrapAround = crate::src::qcommon::q_shared::qtrue;
    s_main.menu.showlogo = crate::src::qcommon::q_shared::qtrue;
    y = 134 as i32;
    s_main.singleplayer.generic.type_0 = 9 as i32;
    s_main.singleplayer.generic.flags = 0x8 as i32 as u32 | 0x100 as i32 as u32;
    s_main.singleplayer.generic.x = 320 as i32;
    s_main.singleplayer.generic.y = y;
    s_main.singleplayer.generic.id = 10 as i32;
    s_main.singleplayer.generic.callback =
        Some(Main_MenuEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_main.singleplayer.string =
        b"SINGLE PLAYER\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_main.singleplayer.color = crate::src::q3_ui::ui_qmenu::color_red.as_mut_ptr();
    s_main.singleplayer.style = style;
    y += 34 as i32;
    s_main.multiplayer.generic.type_0 = 9 as i32;
    s_main.multiplayer.generic.flags = 0x8 as i32 as u32 | 0x100 as i32 as u32;
    s_main.multiplayer.generic.x = 320 as i32;
    s_main.multiplayer.generic.y = y;
    s_main.multiplayer.generic.id = 11 as i32;
    s_main.multiplayer.generic.callback =
        Some(Main_MenuEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_main.multiplayer.string =
        b"MULTIPLAYER\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_main.multiplayer.color = crate::src::q3_ui::ui_qmenu::color_red.as_mut_ptr();
    s_main.multiplayer.style = style;
    y += 34 as i32;
    s_main.setup.generic.type_0 = 9 as i32;
    s_main.setup.generic.flags = 0x8 as i32 as u32 | 0x100 as i32 as u32;
    s_main.setup.generic.x = 320 as i32;
    s_main.setup.generic.y = y;
    s_main.setup.generic.id = 12 as i32;
    s_main.setup.generic.callback =
        Some(Main_MenuEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_main.setup.string = b"SETUP\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_main.setup.color = crate::src::q3_ui::ui_qmenu::color_red.as_mut_ptr();
    s_main.setup.style = style;
    y += 34 as i32;
    s_main.demos.generic.type_0 = 9 as i32;
    s_main.demos.generic.flags = 0x8 as i32 as u32 | 0x100 as i32 as u32;
    s_main.demos.generic.x = 320 as i32;
    s_main.demos.generic.y = y;
    s_main.demos.generic.id = 13 as i32;
    s_main.demos.generic.callback =
        Some(Main_MenuEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_main.demos.string = b"DEMOS\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_main.demos.color = crate::src::q3_ui::ui_qmenu::color_red.as_mut_ptr();
    s_main.demos.style = style;
    y += 34 as i32;
    s_main.cinematics.generic.type_0 = 9 as i32;
    s_main.cinematics.generic.flags = 0x8 as i32 as u32 | 0x100 as i32 as u32;
    s_main.cinematics.generic.x = 320 as i32;
    s_main.cinematics.generic.y = y;
    s_main.cinematics.generic.id = 14 as i32;
    s_main.cinematics.generic.callback =
        Some(Main_MenuEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_main.cinematics.string =
        b"CINEMATICS\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_main.cinematics.color = crate::src::q3_ui::ui_qmenu::color_red.as_mut_ptr();
    s_main.cinematics.style = style;
    if crate::src::q3_ui::ui_atoms::uis.demoversion as u64 == 0 && UI_TeamArenaExists() as u32 != 0
    {
        teamArena = crate::src::qcommon::q_shared::qtrue;
        y += 34 as i32;
        s_main.teamArena.generic.type_0 = 9 as i32;
        s_main.teamArena.generic.flags = 0x8 as i32 as u32 | 0x100 as i32 as u32;
        s_main.teamArena.generic.x = 320 as i32;
        s_main.teamArena.generic.y = y;
        s_main.teamArena.generic.id = 15 as i32;
        s_main.teamArena.generic.callback =
            Some(Main_MenuEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
        s_main.teamArena.string =
            b"TEAM ARENA\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
        s_main.teamArena.color = crate::src::q3_ui::ui_qmenu::color_red.as_mut_ptr();
        s_main.teamArena.style = style
    }
    if crate::src::q3_ui::ui_atoms::uis.demoversion as u64 == 0 {
        y += 34 as i32;
        s_main.mods.generic.type_0 = 9 as i32;
        s_main.mods.generic.flags = 0x8 as i32 as u32 | 0x100 as i32 as u32;
        s_main.mods.generic.x = 320 as i32;
        s_main.mods.generic.y = y;
        s_main.mods.generic.id = 16 as i32;
        s_main.mods.generic.callback =
            Some(Main_MenuEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
        s_main.mods.string = b"MODS\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
        s_main.mods.color = crate::src::q3_ui::ui_qmenu::color_red.as_mut_ptr();
        s_main.mods.style = style
    }
    y += 34 as i32;
    s_main.exit.generic.type_0 = 9 as i32;
    s_main.exit.generic.flags = 0x8 as i32 as u32 | 0x100 as i32 as u32;
    s_main.exit.generic.x = 320 as i32;
    s_main.exit.generic.y = y;
    s_main.exit.generic.id = 17 as i32;
    s_main.exit.generic.callback =
        Some(Main_MenuEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_main.exit.string = b"EXIT\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_main.exit.color = crate::src::q3_ui::ui_qmenu::color_red.as_mut_ptr();
    s_main.exit.style = style;
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_main.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut s_main.singleplayer as *mut crate::ui_local_h::menutext_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_main.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut s_main.multiplayer as *mut crate::ui_local_h::menutext_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_main.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut s_main.setup as *mut crate::ui_local_h::menutext_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_main.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut s_main.demos as *mut crate::ui_local_h::menutext_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_main.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut s_main.cinematics as *mut crate::ui_local_h::menutext_s as *mut libc::c_void,
    );
    if teamArena as u64 != 0 {
        crate::src::q3_ui::ui_qmenu::Menu_AddItem(
            &mut s_main.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
            &mut s_main.teamArena as *mut crate::ui_local_h::menutext_s as *mut libc::c_void,
        );
    }
    if crate::src::q3_ui::ui_atoms::uis.demoversion as u64 == 0 {
        crate::src::q3_ui::ui_qmenu::Menu_AddItem(
            &mut s_main.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
            &mut s_main.mods as *mut crate::ui_local_h::menutext_s as *mut libc::c_void,
        );
    }
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_main.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut s_main.exit as *mut crate::ui_local_h::menutext_s as *mut libc::c_void,
    );
    crate::src::ui::ui_syscalls::trap_Key_SetCatcher(0x2 as i32);
    crate::src::q3_ui::ui_atoms::uis.menusp = 0 as i32;
    crate::src::q3_ui::ui_atoms::UI_PushMenu(
        &mut s_main.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
    );
}

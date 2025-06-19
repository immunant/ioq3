use ::libc;

pub mod stdlib_h {
    #[inline]

    pub unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> i32 {
        return ::libc::strtol(
            __nptr,
            0 as *mut libc::c_void as *mut *mut libc::c_char,
            10 as i32,
        ) as i32;
    }
}

pub use crate::src::q3_ui::ui_atoms::uis;
pub use crate::src::q3_ui::ui_atoms::UI_Argv;
pub use crate::src::q3_ui::ui_atoms::UI_PopMenu;
pub use crate::src::q3_ui::ui_atoms::UI_PushMenu;
pub use crate::src::q3_ui::ui_cinematics::stdlib_h::atoi;
pub use crate::src::q3_ui::ui_gameinfo::UI_CanShowTierVideo;
pub use crate::src::q3_ui::ui_qmenu::color_red;
pub use crate::src::q3_ui::ui_qmenu::color_white;
pub use crate::src::q3_ui::ui_qmenu::Menu_AddItem;
pub use crate::src::q3_ui::ui_qmenu::Menu_SetCursorToItem;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qhandle_t;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::sfxHandle_t;
pub use crate::src::qcommon::q_shared::va;
pub use crate::src::qcommon::q_shared::vec4_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::qcommon::q_shared::C2RustUnnamed_0;
pub use crate::src::qcommon::q_shared::EXEC_APPEND;
pub use crate::src::qcommon::q_shared::EXEC_INSERT;
pub use crate::src::qcommon::q_shared::EXEC_NOW;
pub use crate::src::ui::ui_syscalls::trap_Cmd_ExecuteText;
pub use crate::src::ui::ui_syscalls::trap_Cvar_Set;
pub use crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip;

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
pub use crate::ui_local_h::menubitmap_s;
pub use crate::ui_local_h::menucommon_s;
pub use crate::ui_local_h::menuframework_s;
pub use crate::ui_local_h::menutext_s;
pub use crate::ui_local_h::uiStatic_t;
pub use ::libc::strtol;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cinematicsMenuInfo_t {
    pub menu: crate::ui_local_h::menuframework_s,
    pub banner: crate::ui_local_h::menutext_s,
    pub framel: crate::ui_local_h::menubitmap_s,
    pub framer: crate::ui_local_h::menubitmap_s,
    pub cin_idlogo: crate::ui_local_h::menutext_s,
    pub cin_intro: crate::ui_local_h::menutext_s,
    pub cin_tier1: crate::ui_local_h::menutext_s,
    pub cin_tier2: crate::ui_local_h::menutext_s,
    pub cin_tier3: crate::ui_local_h::menutext_s,
    pub cin_tier4: crate::ui_local_h::menutext_s,
    pub cin_tier5: crate::ui_local_h::menutext_s,
    pub cin_tier6: crate::ui_local_h::menutext_s,
    pub cin_tier7: crate::ui_local_h::menutext_s,
    pub cin_end: crate::ui_local_h::menutext_s,
    pub back: crate::ui_local_h::menubitmap_s,
}

static mut cinematicsMenuInfo: cinematicsMenuInfo_t = cinematicsMenuInfo_t {
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
    banner: crate::ui_local_h::menutext_s {
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
    framel: crate::ui_local_h::menubitmap_s {
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
        focuscolor: 0 as *const f32 as *mut f32,
    },
    framer: crate::ui_local_h::menubitmap_s {
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
        focuscolor: 0 as *const f32 as *mut f32,
    },
    cin_idlogo: crate::ui_local_h::menutext_s {
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
    cin_intro: crate::ui_local_h::menutext_s {
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
    cin_tier1: crate::ui_local_h::menutext_s {
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
    cin_tier2: crate::ui_local_h::menutext_s {
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
    cin_tier3: crate::ui_local_h::menutext_s {
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
    cin_tier4: crate::ui_local_h::menutext_s {
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
    cin_tier5: crate::ui_local_h::menutext_s {
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
    cin_tier6: crate::ui_local_h::menutext_s {
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
    cin_tier7: crate::ui_local_h::menutext_s {
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
    cin_end: crate::ui_local_h::menutext_s {
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
    back: crate::ui_local_h::menubitmap_s {
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
        focuscolor: 0 as *const f32 as *mut f32,
    },
};

static mut cinematics: [*mut libc::c_char; 10] = [
    b"idlogo\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"intro\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"tier1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"tier2\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"tier3\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"tier4\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"tier5\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"tier6\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"tier7\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"end\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
];
/*
===============
UI_CinematicsMenu_BackEvent
===============
*/

unsafe extern "C" fn UI_CinematicsMenu_BackEvent(mut _ptr: *mut libc::c_void, mut event: i32) {
    if event != 3 as i32 {
        return;
    }
    crate::src::q3_ui::ui_atoms::UI_PopMenu();
}
/*
===============
UI_CinematicsMenu_Event
===============
*/

unsafe extern "C" fn UI_CinematicsMenu_Event(mut ptr: *mut libc::c_void, mut event: i32) {
    let mut n: i32 = 0;
    if event != 3 as i32 {
        return;
    }
    n = (*(ptr as *mut crate::ui_local_h::menucommon_s)).id - 11 as i32;
    crate::src::ui::ui_syscalls::trap_Cvar_Set(
        b"nextmap\x00" as *const u8 as *const libc::c_char,
        crate::src::qcommon::q_shared::va(
            b"ui_cinematics %i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            n,
        ),
    );
    if crate::src::q3_ui::ui_atoms::uis.demoversion as u32 != 0
        && (*(ptr as *mut crate::ui_local_h::menucommon_s)).id == 20 as i32
    {
        crate::src::ui::ui_syscalls::trap_Cmd_ExecuteText(
            crate::src::qcommon::q_shared::EXEC_APPEND as i32,
            b"disconnect; cinematic demoEnd.RoQ 1\n\x00" as *const u8 as *const libc::c_char,
        );
    } else {
        crate::src::ui::ui_syscalls::trap_Cmd_ExecuteText(
            crate::src::qcommon::q_shared::EXEC_APPEND as i32,
            crate::src::qcommon::q_shared::va(
                b"disconnect; cinematic %s.RoQ\n\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                cinematics[n as usize],
            ),
        );
    };
}
/*
===============
UI_CinematicsMenu_Init
===============
*/

unsafe extern "C" fn UI_CinematicsMenu_Init() {
    let mut y: i32 = 0;
    UI_CinematicsMenu_Cache();
    crate::stdlib::memset(
        &mut cinematicsMenuInfo as *mut cinematicsMenuInfo_t as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<cinematicsMenuInfo_t>() as libc::c_ulong,
    );
    cinematicsMenuInfo.menu.fullscreen = crate::src::qcommon::q_shared::qtrue;
    cinematicsMenuInfo.banner.generic.type_0 = 10 as i32;
    cinematicsMenuInfo.banner.generic.x = 320 as i32;
    cinematicsMenuInfo.banner.generic.y = 16 as i32;
    cinematicsMenuInfo.banner.string =
        b"CINEMATICS\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    cinematicsMenuInfo.banner.color = crate::src::q3_ui::ui_qmenu::color_white.as_mut_ptr();
    cinematicsMenuInfo.banner.style = 0x1 as i32;
    cinematicsMenuInfo.framel.generic.type_0 = 6 as i32;
    cinematicsMenuInfo.framel.generic.name =
        b"menu/art/frame2_l\x00" as *const u8 as *const libc::c_char;
    cinematicsMenuInfo.framel.generic.flags = 0x4000 as i32 as u32;
    cinematicsMenuInfo.framel.generic.x = 0 as i32;
    cinematicsMenuInfo.framel.generic.y = 78 as i32;
    cinematicsMenuInfo.framel.width = 256 as i32;
    cinematicsMenuInfo.framel.height = 329 as i32;
    cinematicsMenuInfo.framer.generic.type_0 = 6 as i32;
    cinematicsMenuInfo.framer.generic.name =
        b"menu/art/frame1_r\x00" as *const u8 as *const libc::c_char;
    cinematicsMenuInfo.framer.generic.flags = 0x4000 as i32 as u32;
    cinematicsMenuInfo.framer.generic.x = 376 as i32;
    cinematicsMenuInfo.framer.generic.y = 76 as i32;
    cinematicsMenuInfo.framer.width = 256 as i32;
    cinematicsMenuInfo.framer.height = 334 as i32;
    y = 100 as i32;
    cinematicsMenuInfo.cin_idlogo.generic.type_0 = 9 as i32;
    cinematicsMenuInfo.cin_idlogo.generic.flags = 0x8 as i32 as u32 | 0x100 as i32 as u32;
    cinematicsMenuInfo.cin_idlogo.generic.x = 320 as i32;
    cinematicsMenuInfo.cin_idlogo.generic.y = y;
    cinematicsMenuInfo.cin_idlogo.generic.id = 11 as i32;
    cinematicsMenuInfo.cin_idlogo.generic.callback =
        Some(UI_CinematicsMenu_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    cinematicsMenuInfo.cin_idlogo.string =
        b"ID LOGO\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    cinematicsMenuInfo.cin_idlogo.color = crate::src::q3_ui::ui_qmenu::color_red.as_mut_ptr();
    cinematicsMenuInfo.cin_idlogo.style = 0x1 as i32;
    y += 30 as i32;
    cinematicsMenuInfo.cin_intro.generic.type_0 = 9 as i32;
    cinematicsMenuInfo.cin_intro.generic.flags = 0x8 as i32 as u32 | 0x100 as i32 as u32;
    cinematicsMenuInfo.cin_intro.generic.x = 320 as i32;
    cinematicsMenuInfo.cin_intro.generic.y = y;
    cinematicsMenuInfo.cin_intro.generic.id = 12 as i32;
    cinematicsMenuInfo.cin_intro.generic.callback =
        Some(UI_CinematicsMenu_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    cinematicsMenuInfo.cin_intro.string =
        b"INTRO\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    cinematicsMenuInfo.cin_intro.color = crate::src::q3_ui::ui_qmenu::color_red.as_mut_ptr();
    cinematicsMenuInfo.cin_intro.style = 0x1 as i32;
    if crate::src::q3_ui::ui_atoms::uis.demoversion as u64 != 0 {
        cinematicsMenuInfo.cin_intro.generic.flags |= 0x2000 as i32 as u32
    }
    y += 30 as i32;
    cinematicsMenuInfo.cin_tier1.generic.type_0 = 9 as i32;
    cinematicsMenuInfo.cin_tier1.generic.flags = 0x8 as i32 as u32 | 0x100 as i32 as u32;
    cinematicsMenuInfo.cin_tier1.generic.x = 320 as i32;
    cinematicsMenuInfo.cin_tier1.generic.y = y;
    cinematicsMenuInfo.cin_tier1.generic.id = 13 as i32;
    cinematicsMenuInfo.cin_tier1.generic.callback =
        Some(UI_CinematicsMenu_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    cinematicsMenuInfo.cin_tier1.string =
        b"Tier 1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    cinematicsMenuInfo.cin_tier1.color = crate::src::q3_ui::ui_qmenu::color_red.as_mut_ptr();
    cinematicsMenuInfo.cin_tier1.style = 0x1 as i32;
    if crate::src::q3_ui::ui_gameinfo::UI_CanShowTierVideo(1 as i32) as u64 == 0 {
        cinematicsMenuInfo.cin_tier1.generic.flags |= 0x2000 as i32 as u32
    }
    y += 30 as i32;
    cinematicsMenuInfo.cin_tier2.generic.type_0 = 9 as i32;
    cinematicsMenuInfo.cin_tier2.generic.flags = 0x8 as i32 as u32 | 0x100 as i32 as u32;
    cinematicsMenuInfo.cin_tier2.generic.x = 320 as i32;
    cinematicsMenuInfo.cin_tier2.generic.y = y;
    cinematicsMenuInfo.cin_tier2.generic.id = 14 as i32;
    cinematicsMenuInfo.cin_tier2.generic.callback =
        Some(UI_CinematicsMenu_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    cinematicsMenuInfo.cin_tier2.string =
        b"Tier 2\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    cinematicsMenuInfo.cin_tier2.color = crate::src::q3_ui::ui_qmenu::color_red.as_mut_ptr();
    cinematicsMenuInfo.cin_tier2.style = 0x1 as i32;
    if crate::src::q3_ui::ui_gameinfo::UI_CanShowTierVideo(2 as i32) as u64 == 0 {
        cinematicsMenuInfo.cin_tier2.generic.flags |= 0x2000 as i32 as u32
    }
    y += 30 as i32;
    cinematicsMenuInfo.cin_tier3.generic.type_0 = 9 as i32;
    cinematicsMenuInfo.cin_tier3.generic.flags = 0x8 as i32 as u32 | 0x100 as i32 as u32;
    cinematicsMenuInfo.cin_tier3.generic.x = 320 as i32;
    cinematicsMenuInfo.cin_tier3.generic.y = y;
    cinematicsMenuInfo.cin_tier3.generic.id = 15 as i32;
    cinematicsMenuInfo.cin_tier3.generic.callback =
        Some(UI_CinematicsMenu_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    cinematicsMenuInfo.cin_tier3.string =
        b"Tier 3\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    cinematicsMenuInfo.cin_tier3.color = crate::src::q3_ui::ui_qmenu::color_red.as_mut_ptr();
    cinematicsMenuInfo.cin_tier3.style = 0x1 as i32;
    if crate::src::q3_ui::ui_gameinfo::UI_CanShowTierVideo(3 as i32) as u64 == 0 {
        cinematicsMenuInfo.cin_tier3.generic.flags |= 0x2000 as i32 as u32
    }
    y += 30 as i32;
    cinematicsMenuInfo.cin_tier4.generic.type_0 = 9 as i32;
    cinematicsMenuInfo.cin_tier4.generic.flags = 0x8 as i32 as u32 | 0x100 as i32 as u32;
    cinematicsMenuInfo.cin_tier4.generic.x = 320 as i32;
    cinematicsMenuInfo.cin_tier4.generic.y = y;
    cinematicsMenuInfo.cin_tier4.generic.id = 16 as i32;
    cinematicsMenuInfo.cin_tier4.generic.callback =
        Some(UI_CinematicsMenu_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    cinematicsMenuInfo.cin_tier4.string =
        b"Tier 4\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    cinematicsMenuInfo.cin_tier4.color = crate::src::q3_ui::ui_qmenu::color_red.as_mut_ptr();
    cinematicsMenuInfo.cin_tier4.style = 0x1 as i32;
    if crate::src::q3_ui::ui_gameinfo::UI_CanShowTierVideo(4 as i32) as u64 == 0 {
        cinematicsMenuInfo.cin_tier4.generic.flags |= 0x2000 as i32 as u32
    }
    y += 30 as i32;
    cinematicsMenuInfo.cin_tier5.generic.type_0 = 9 as i32;
    cinematicsMenuInfo.cin_tier5.generic.flags = 0x8 as i32 as u32 | 0x100 as i32 as u32;
    cinematicsMenuInfo.cin_tier5.generic.x = 320 as i32;
    cinematicsMenuInfo.cin_tier5.generic.y = y;
    cinematicsMenuInfo.cin_tier5.generic.id = 17 as i32;
    cinematicsMenuInfo.cin_tier5.generic.callback =
        Some(UI_CinematicsMenu_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    cinematicsMenuInfo.cin_tier5.string =
        b"Tier 5\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    cinematicsMenuInfo.cin_tier5.color = crate::src::q3_ui::ui_qmenu::color_red.as_mut_ptr();
    cinematicsMenuInfo.cin_tier5.style = 0x1 as i32;
    if crate::src::q3_ui::ui_gameinfo::UI_CanShowTierVideo(5 as i32) as u64 == 0 {
        cinematicsMenuInfo.cin_tier5.generic.flags |= 0x2000 as i32 as u32
    }
    y += 30 as i32;
    cinematicsMenuInfo.cin_tier6.generic.type_0 = 9 as i32;
    cinematicsMenuInfo.cin_tier6.generic.flags = 0x8 as i32 as u32 | 0x100 as i32 as u32;
    cinematicsMenuInfo.cin_tier6.generic.x = 320 as i32;
    cinematicsMenuInfo.cin_tier6.generic.y = y;
    cinematicsMenuInfo.cin_tier6.generic.id = 18 as i32;
    cinematicsMenuInfo.cin_tier6.generic.callback =
        Some(UI_CinematicsMenu_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    cinematicsMenuInfo.cin_tier6.string =
        b"Tier 6\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    cinematicsMenuInfo.cin_tier6.color = crate::src::q3_ui::ui_qmenu::color_red.as_mut_ptr();
    cinematicsMenuInfo.cin_tier6.style = 0x1 as i32;
    if crate::src::q3_ui::ui_gameinfo::UI_CanShowTierVideo(6 as i32) as u64 == 0 {
        cinematicsMenuInfo.cin_tier6.generic.flags |= 0x2000 as i32 as u32
    }
    y += 30 as i32;
    cinematicsMenuInfo.cin_tier7.generic.type_0 = 9 as i32;
    cinematicsMenuInfo.cin_tier7.generic.flags = 0x8 as i32 as u32 | 0x100 as i32 as u32;
    cinematicsMenuInfo.cin_tier7.generic.x = 320 as i32;
    cinematicsMenuInfo.cin_tier7.generic.y = y;
    cinematicsMenuInfo.cin_tier7.generic.id = 19 as i32;
    cinematicsMenuInfo.cin_tier7.generic.callback =
        Some(UI_CinematicsMenu_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    cinematicsMenuInfo.cin_tier7.string =
        b"Tier 7\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    cinematicsMenuInfo.cin_tier7.color = crate::src::q3_ui::ui_qmenu::color_red.as_mut_ptr();
    cinematicsMenuInfo.cin_tier7.style = 0x1 as i32;
    if crate::src::q3_ui::ui_gameinfo::UI_CanShowTierVideo(7 as i32) as u64 == 0 {
        cinematicsMenuInfo.cin_tier7.generic.flags |= 0x2000 as i32 as u32
    }
    y += 30 as i32;
    cinematicsMenuInfo.cin_end.generic.type_0 = 9 as i32;
    cinematicsMenuInfo.cin_end.generic.flags = 0x8 as i32 as u32 | 0x100 as i32 as u32;
    cinematicsMenuInfo.cin_end.generic.x = 320 as i32;
    cinematicsMenuInfo.cin_end.generic.y = y;
    cinematicsMenuInfo.cin_end.generic.id = 20 as i32;
    cinematicsMenuInfo.cin_end.generic.callback =
        Some(UI_CinematicsMenu_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    cinematicsMenuInfo.cin_end.string =
        b"END\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    cinematicsMenuInfo.cin_end.color = crate::src::q3_ui::ui_qmenu::color_red.as_mut_ptr();
    cinematicsMenuInfo.cin_end.style = 0x1 as i32;
    if crate::src::q3_ui::ui_gameinfo::UI_CanShowTierVideo(8 as i32) as u64 == 0 {
        cinematicsMenuInfo.cin_end.generic.flags |= 0x2000 as i32 as u32
    }
    cinematicsMenuInfo.back.generic.type_0 = 6 as i32;
    cinematicsMenuInfo.back.generic.name =
        b"menu/art/back_0\x00" as *const u8 as *const libc::c_char;
    cinematicsMenuInfo.back.generic.flags = 0x4 as i32 as u32 | 0x100 as i32 as u32;
    cinematicsMenuInfo.back.generic.id = 10 as i32;
    cinematicsMenuInfo.back.generic.callback = Some(
        UI_CinematicsMenu_BackEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> (),
    );
    cinematicsMenuInfo.back.generic.x = 0 as i32;
    cinematicsMenuInfo.back.generic.y = 480 as i32 - 64 as i32;
    cinematicsMenuInfo.back.width = 128 as i32;
    cinematicsMenuInfo.back.height = 64 as i32;
    cinematicsMenuInfo.back.focuspic =
        b"menu/art/back_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut cinematicsMenuInfo.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut cinematicsMenuInfo.banner as *mut crate::ui_local_h::menutext_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut cinematicsMenuInfo.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut cinematicsMenuInfo.framel as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut cinematicsMenuInfo.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut cinematicsMenuInfo.framer as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut cinematicsMenuInfo.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut cinematicsMenuInfo.cin_idlogo as *mut crate::ui_local_h::menutext_s
            as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut cinematicsMenuInfo.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut cinematicsMenuInfo.cin_intro as *mut crate::ui_local_h::menutext_s
            as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut cinematicsMenuInfo.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut cinematicsMenuInfo.cin_tier1 as *mut crate::ui_local_h::menutext_s
            as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut cinematicsMenuInfo.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut cinematicsMenuInfo.cin_tier2 as *mut crate::ui_local_h::menutext_s
            as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut cinematicsMenuInfo.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut cinematicsMenuInfo.cin_tier3 as *mut crate::ui_local_h::menutext_s
            as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut cinematicsMenuInfo.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut cinematicsMenuInfo.cin_tier4 as *mut crate::ui_local_h::menutext_s
            as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut cinematicsMenuInfo.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut cinematicsMenuInfo.cin_tier5 as *mut crate::ui_local_h::menutext_s
            as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut cinematicsMenuInfo.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut cinematicsMenuInfo.cin_tier6 as *mut crate::ui_local_h::menutext_s
            as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut cinematicsMenuInfo.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut cinematicsMenuInfo.cin_tier7 as *mut crate::ui_local_h::menutext_s
            as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut cinematicsMenuInfo.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut cinematicsMenuInfo.cin_end as *mut crate::ui_local_h::menutext_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut cinematicsMenuInfo.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut cinematicsMenuInfo.back as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
}
/*
=================
UI_CinematicsMenu_Cache
=================
*/
#[no_mangle]

pub unsafe extern "C" fn UI_CinematicsMenu_Cache() {
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/back_0\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/back_1\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/frame2_l\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/frame1_r\x00" as *const u8 as *const libc::c_char,
    );
}
/*
===============
UI_CinematicsMenu
===============
*/
#[no_mangle]

pub unsafe extern "C" fn UI_CinematicsMenu() {
    UI_CinematicsMenu_Init();
    crate::src::q3_ui::ui_atoms::UI_PushMenu(
        &mut cinematicsMenuInfo.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
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
/*
===============
UI_CinematicsMenu_f
===============
*/
#[no_mangle]

pub unsafe extern "C" fn UI_CinematicsMenu_f() {
    let mut n: i32 = 0;
    n = atoi(crate::src::q3_ui::ui_atoms::UI_Argv(1 as i32));
    UI_CinematicsMenu();
    crate::src::q3_ui::ui_qmenu::Menu_SetCursorToItem(
        &mut cinematicsMenuInfo.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        cinematicsMenuInfo.menu.items[(n + 3 as i32) as usize],
    );
}

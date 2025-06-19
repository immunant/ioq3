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
pub use crate::src::q3_ui::ui_atoms::UI_Cvar_VariableString;
pub use crate::src::q3_ui::ui_atoms::UI_DrawString;
pub use crate::src::q3_ui::ui_atoms::UI_PopMenu;
pub use crate::src::q3_ui::ui_atoms::UI_PushMenu;
pub use crate::src::q3_ui::ui_display::UI_DisplayOptionsMenu;
pub use crate::src::q3_ui::ui_network::UI_NetworkOptionsMenu;
pub use crate::src::q3_ui::ui_qmenu::color_red;
pub use crate::src::q3_ui::ui_qmenu::color_white;
pub use crate::src::q3_ui::ui_qmenu::text_color_normal;
pub use crate::src::q3_ui::ui_qmenu::Menu_AddItem;
pub use crate::src::q3_ui::ui_qmenu::Menu_Draw;
pub use crate::src::q3_ui::ui_qmenu::Menu_SetCursorToItem;
pub use crate::src::q3_ui::ui_sound::UI_SoundOptionsMenu;
pub use crate::src::q3_ui::ui_video::stdlib_h::atoi;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qhandle_t;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::sfxHandle_t;
pub use crate::src::qcommon::q_shared::va;
pub use crate::src::qcommon::q_shared::vec4_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::qcommon::q_shared::C2RustUnnamed_0;
pub use crate::src::qcommon::q_shared::Com_sprintf;
pub use crate::src::qcommon::q_shared::Q_stricmp;
pub use crate::src::qcommon::q_shared::Q_strncpyz;
pub use crate::src::qcommon::q_shared::EXEC_APPEND;
pub use crate::src::qcommon::q_shared::EXEC_INSERT;
pub use crate::src::qcommon::q_shared::EXEC_NOW;
pub use crate::src::ui::ui_syscalls::trap_Cmd_ExecuteText;
pub use crate::src::ui::ui_syscalls::trap_Cvar_Reset;
pub use crate::src::ui::ui_syscalls::trap_Cvar_Set;
pub use crate::src::ui::ui_syscalls::trap_Cvar_SetValue;
pub use crate::src::ui::ui_syscalls::trap_Cvar_VariableStringBuffer;
pub use crate::src::ui::ui_syscalls::trap_Cvar_VariableValue;
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
pub use crate::ui_local_h::menulist_s;
pub use crate::ui_local_h::menuslider_s;
pub use crate::ui_local_h::menutext_s;
pub use crate::ui_local_h::uiStatic_t;

pub use ::libc::strtol;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct graphicsoptions_t {
    pub menu: crate::ui_local_h::menuframework_s,
    pub banner: crate::ui_local_h::menutext_s,
    pub framel: crate::ui_local_h::menubitmap_s,
    pub framer: crate::ui_local_h::menubitmap_s,
    pub graphics: crate::ui_local_h::menutext_s,
    pub display: crate::ui_local_h::menutext_s,
    pub sound: crate::ui_local_h::menutext_s,
    pub network: crate::ui_local_h::menutext_s,
    pub list: crate::ui_local_h::menulist_s,
    pub ratio: crate::ui_local_h::menulist_s,
    pub mode: crate::ui_local_h::menulist_s,
    pub driver: crate::ui_local_h::menulist_s,
    pub tq: crate::ui_local_h::menuslider_s,
    pub fs: crate::ui_local_h::menulist_s,
    pub lighting: crate::ui_local_h::menulist_s,
    pub allow_extensions: crate::ui_local_h::menulist_s,
    pub texturebits: crate::ui_local_h::menulist_s,
    pub colordepth: crate::ui_local_h::menulist_s,
    pub geometry: crate::ui_local_h::menulist_s,
    pub filter: crate::ui_local_h::menulist_s,
    pub driverinfo: crate::ui_local_h::menutext_s,
    pub apply: crate::ui_local_h::menubitmap_s,
    pub back: crate::ui_local_h::menubitmap_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct InitialVideoOptions_s {
    pub mode: i32,
    pub fullscreen: crate::src::qcommon::q_shared::qboolean,
    pub tq: i32,
    pub lighting: i32,
    pub colordepth: i32,
    pub texturebits: i32,
    pub geometry: i32,
    pub filter: i32,
    pub driver: i32,
    pub extensions: crate::src::qcommon::q_shared::qboolean,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct driverinfo_t {
    pub menu: crate::ui_local_h::menuframework_s,
    pub banner: crate::ui_local_h::menutext_s,
    pub back: crate::ui_local_h::menubitmap_s,
    pub framel: crate::ui_local_h::menubitmap_s,
    pub framer: crate::ui_local_h::menubitmap_s,
    pub stringbuff: [libc::c_char; 1024],
    pub strings: [*mut libc::c_char; 64],
    pub numstrings: i32,
}

static mut driverinfo_artlist: [*mut libc::c_char; 5] = [
    b"menu/art/frame2_l\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"menu/art/frame1_r\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"menu/art/back_0\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"menu/art/back_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
];

static mut s_driverinfo: driverinfo_t = driverinfo_t {
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
    stringbuff: [0; 1024],
    strings: [0 as *const libc::c_char as *mut libc::c_char; 64],
    numstrings: 0,
};
/*
=================
DriverInfo_Event
=================
*/

unsafe extern "C" fn DriverInfo_Event(mut ptr: *mut libc::c_void, mut event: i32) {
    if event != 3 as i32 {
        return;
    }
    match (*(ptr as *mut crate::ui_local_h::menucommon_s)).id {
        100 => {
            crate::src::q3_ui::ui_atoms::UI_PopMenu();
        }
        _ => {}
    };
}
/*
=================
DriverInfo_MenuDraw
=================
*/

unsafe extern "C" fn DriverInfo_MenuDraw() {
    let mut i: i32 = 0;
    let mut y: i32 = 0;
    crate::src::q3_ui::ui_qmenu::Menu_Draw(
        &mut s_driverinfo.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
    );
    crate::src::q3_ui::ui_atoms::UI_DrawString(
        320 as i32,
        80 as i32,
        b"VENDOR\x00" as *const u8 as *const libc::c_char,
        0x1 as i32 | 0x10 as i32,
        crate::src::q3_ui::ui_qmenu::color_red.as_mut_ptr(),
    );
    crate::src::q3_ui::ui_atoms::UI_DrawString(
        320 as i32,
        152 as i32,
        b"PIXELFORMAT\x00" as *const u8 as *const libc::c_char,
        0x1 as i32 | 0x10 as i32,
        crate::src::q3_ui::ui_qmenu::color_red.as_mut_ptr(),
    );
    crate::src::q3_ui::ui_atoms::UI_DrawString(
        320 as i32,
        192 as i32,
        b"EXTENSIONS\x00" as *const u8 as *const libc::c_char,
        0x1 as i32 | 0x10 as i32,
        crate::src::q3_ui::ui_qmenu::color_red.as_mut_ptr(),
    );
    crate::src::q3_ui::ui_atoms::UI_DrawString(
        320 as i32,
        80 as i32 + 16 as i32,
        crate::src::q3_ui::ui_atoms::uis
            .glconfig
            .vendor_string
            .as_mut_ptr(),
        0x1 as i32 | 0x10 as i32,
        crate::src::q3_ui::ui_qmenu::text_color_normal.as_mut_ptr(),
    );
    crate::src::q3_ui::ui_atoms::UI_DrawString(
        320 as i32,
        96 as i32 + 16 as i32,
        crate::src::q3_ui::ui_atoms::uis
            .glconfig
            .version_string
            .as_mut_ptr(),
        0x1 as i32 | 0x10 as i32,
        crate::src::q3_ui::ui_qmenu::text_color_normal.as_mut_ptr(),
    );
    crate::src::q3_ui::ui_atoms::UI_DrawString(
        320 as i32,
        112 as i32 + 16 as i32,
        crate::src::q3_ui::ui_atoms::uis
            .glconfig
            .renderer_string
            .as_mut_ptr(),
        0x1 as i32 | 0x10 as i32,
        crate::src::q3_ui::ui_qmenu::text_color_normal.as_mut_ptr(),
    );
    crate::src::q3_ui::ui_atoms::UI_DrawString(
        320 as i32,
        152 as i32 + 16 as i32,
        crate::src::qcommon::q_shared::va(
            b"color(%d-bits) Z(%d-bits) stencil(%d-bits)\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            crate::src::q3_ui::ui_atoms::uis.glconfig.colorBits,
            crate::src::q3_ui::ui_atoms::uis.glconfig.depthBits,
            crate::src::q3_ui::ui_atoms::uis.glconfig.stencilBits,
        ),
        0x1 as i32 | 0x10 as i32,
        crate::src::q3_ui::ui_qmenu::text_color_normal.as_mut_ptr(),
    );
    // double column
    y = 192 as i32 + 16 as i32;
    i = 0 as i32;
    while i < s_driverinfo.numstrings / 2 as i32 {
        crate::src::q3_ui::ui_atoms::UI_DrawString(
            320 as i32 - 4 as i32,
            y,
            s_driverinfo.strings[(i * 2 as i32) as usize],
            0x2 as i32 | 0x10 as i32,
            crate::src::q3_ui::ui_qmenu::text_color_normal.as_mut_ptr(),
        );
        crate::src::q3_ui::ui_atoms::UI_DrawString(
            320 as i32 + 4 as i32,
            y,
            s_driverinfo.strings[(i * 2 as i32 + 1 as i32) as usize],
            0 as i32 | 0x10 as i32,
            crate::src::q3_ui::ui_qmenu::text_color_normal.as_mut_ptr(),
        );
        y += 16 as i32;
        i += 1
    }
    if s_driverinfo.numstrings & 1 as i32 != 0 {
        crate::src::q3_ui::ui_atoms::UI_DrawString(
            320 as i32,
            y,
            s_driverinfo.strings[(s_driverinfo.numstrings - 1 as i32) as usize],
            0x1 as i32 | 0x10 as i32,
            crate::src::q3_ui::ui_qmenu::text_color_normal.as_mut_ptr(),
        );
    };
}
/*
=================
DriverInfo_Cache
=================
*/
#[no_mangle]

pub unsafe extern "C" fn DriverInfo_Cache() {
    let mut i: i32 = 0;
    // touch all our pics
    i = 0 as i32;
    while !driverinfo_artlist[i as usize].is_null() {
        crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(driverinfo_artlist[i as usize]);
        i += 1
    }
}
/*
=================
UI_DriverInfo_Menu
=================
*/

unsafe extern "C" fn UI_DriverInfo_Menu() {
    let mut eptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: i32 = 0;
    let mut len: i32 = 0;
    // zero set all our globals
    crate::stdlib::memset(
        &mut s_driverinfo as *mut driverinfo_t as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<driverinfo_t>() as libc::c_ulong,
    );
    DriverInfo_Cache();
    s_driverinfo.menu.fullscreen = crate::src::qcommon::q_shared::qtrue;
    s_driverinfo.menu.draw = Some(DriverInfo_MenuDraw as unsafe extern "C" fn() -> ());
    s_driverinfo.banner.generic.type_0 = 10 as i32;
    s_driverinfo.banner.generic.x = 320 as i32;
    s_driverinfo.banner.generic.y = 16 as i32;
    s_driverinfo.banner.string =
        b"DRIVER INFO\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_driverinfo.banner.color = crate::src::q3_ui::ui_qmenu::color_white.as_mut_ptr();
    s_driverinfo.banner.style = 0x1 as i32;
    s_driverinfo.framel.generic.type_0 = 6 as i32;
    s_driverinfo.framel.generic.name = b"menu/art/frame2_l\x00" as *const u8 as *const libc::c_char;
    s_driverinfo.framel.generic.flags = 0x4000 as i32 as u32;
    s_driverinfo.framel.generic.x = 0 as i32;
    s_driverinfo.framel.generic.y = 78 as i32;
    s_driverinfo.framel.width = 256 as i32;
    s_driverinfo.framel.height = 329 as i32;
    s_driverinfo.framer.generic.type_0 = 6 as i32;
    s_driverinfo.framer.generic.name = b"menu/art/frame1_r\x00" as *const u8 as *const libc::c_char;
    s_driverinfo.framer.generic.flags = 0x4000 as i32 as u32;
    s_driverinfo.framer.generic.x = 376 as i32;
    s_driverinfo.framer.generic.y = 76 as i32;
    s_driverinfo.framer.width = 256 as i32;
    s_driverinfo.framer.height = 334 as i32;
    s_driverinfo.back.generic.type_0 = 6 as i32;
    s_driverinfo.back.generic.name = b"menu/art/back_0\x00" as *const u8 as *const libc::c_char;
    s_driverinfo.back.generic.flags = 0x4 as i32 as u32 | 0x100 as i32 as u32;
    s_driverinfo.back.generic.callback =
        Some(DriverInfo_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_driverinfo.back.generic.id = 100 as i32;
    s_driverinfo.back.generic.x = 0 as i32;
    s_driverinfo.back.generic.y = 480 as i32 - 64 as i32;
    s_driverinfo.back.width = 128 as i32;
    s_driverinfo.back.height = 64 as i32;
    s_driverinfo.back.focuspic =
        b"menu/art/back_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    // TTimo: overflow with particularly long GL extensions (such as the gf3)
    // https://zerowing.idsoftware.com/bugzilla/show_bug.cgi?id=399
    // NOTE: could have pushed the size of stringbuff, but the list is already out of the screen
    // (no matter what your resolution)
    crate::src::qcommon::q_shared::Q_strncpyz(
        s_driverinfo.stringbuff.as_mut_ptr(),
        crate::src::q3_ui::ui_atoms::uis
            .glconfig
            .extensions_string
            .as_mut_ptr(),
        1024 as i32,
    );
    // build null terminated extension strings
    eptr = s_driverinfo.stringbuff.as_mut_ptr();
    while s_driverinfo.numstrings < 40 as i32 && *eptr as i32 != 0 {
        while *eptr as i32 != 0 && *eptr as i32 == ' ' as i32 {
            let fresh0 = eptr;
            eptr = eptr.offset(1);
            *fresh0 = '\u{0}' as i32 as libc::c_char
        }
        // track start of valid string
        if *eptr as i32 != 0 && *eptr as i32 != ' ' as i32 {
            let fresh1 = s_driverinfo.numstrings;
            s_driverinfo.numstrings = s_driverinfo.numstrings + 1;
            s_driverinfo.strings[fresh1 as usize] = eptr
        }
        while *eptr as i32 != 0 && *eptr as i32 != ' ' as i32 {
            eptr = eptr.offset(1)
        }
    }
    // safety length strings for display
    i = 0 as i32;
    while i < s_driverinfo.numstrings {
        len = crate::stdlib::strlen(s_driverinfo.strings[i as usize]) as i32;
        if len > 32 as i32 {
            *s_driverinfo.strings[i as usize].offset((len - 1 as i32) as isize) =
                '>' as i32 as libc::c_char;
            *s_driverinfo.strings[i as usize].offset(len as isize) = '\u{0}' as i32 as libc::c_char
        }
        i += 1
    }
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_driverinfo.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut s_driverinfo.banner as *mut crate::ui_local_h::menutext_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_driverinfo.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut s_driverinfo.framel as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_driverinfo.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut s_driverinfo.framer as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_driverinfo.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut s_driverinfo.back as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_atoms::UI_PushMenu(
        &mut s_driverinfo.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
    );
}

static mut s_ivo: InitialVideoOptions_s = InitialVideoOptions_s {
    mode: 0,
    fullscreen: crate::src::qcommon::q_shared::qfalse,
    tq: 0,
    lighting: 0,
    colordepth: 0,
    texturebits: 0,
    geometry: 0,
    filter: 0,
    driver: 0,
    extensions: crate::src::qcommon::q_shared::qfalse,
};

static mut s_graphicsoptions: graphicsoptions_t = graphicsoptions_t {
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
    graphics: crate::ui_local_h::menutext_s {
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
    display: crate::ui_local_h::menutext_s {
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
    sound: crate::ui_local_h::menutext_s {
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
    network: crate::ui_local_h::menutext_s {
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
    list: crate::ui_local_h::menulist_s {
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
        oldvalue: 0,
        curvalue: 0,
        numitems: 0,
        top: 0,
        itemnames: 0 as *const *const libc::c_char as *mut *const libc::c_char,
        width: 0,
        height: 0,
        columns: 0,
        separation: 0,
    },
    ratio: crate::ui_local_h::menulist_s {
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
        oldvalue: 0,
        curvalue: 0,
        numitems: 0,
        top: 0,
        itemnames: 0 as *const *const libc::c_char as *mut *const libc::c_char,
        width: 0,
        height: 0,
        columns: 0,
        separation: 0,
    },
    mode: crate::ui_local_h::menulist_s {
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
        oldvalue: 0,
        curvalue: 0,
        numitems: 0,
        top: 0,
        itemnames: 0 as *const *const libc::c_char as *mut *const libc::c_char,
        width: 0,
        height: 0,
        columns: 0,
        separation: 0,
    },
    driver: crate::ui_local_h::menulist_s {
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
        oldvalue: 0,
        curvalue: 0,
        numitems: 0,
        top: 0,
        itemnames: 0 as *const *const libc::c_char as *mut *const libc::c_char,
        width: 0,
        height: 0,
        columns: 0,
        separation: 0,
    },
    tq: crate::ui_local_h::menuslider_s {
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
        minvalue: 0.,
        maxvalue: 0.,
        curvalue: 0.,
        range: 0.,
    },
    fs: crate::ui_local_h::menulist_s {
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
        oldvalue: 0,
        curvalue: 0,
        numitems: 0,
        top: 0,
        itemnames: 0 as *const *const libc::c_char as *mut *const libc::c_char,
        width: 0,
        height: 0,
        columns: 0,
        separation: 0,
    },
    lighting: crate::ui_local_h::menulist_s {
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
        oldvalue: 0,
        curvalue: 0,
        numitems: 0,
        top: 0,
        itemnames: 0 as *const *const libc::c_char as *mut *const libc::c_char,
        width: 0,
        height: 0,
        columns: 0,
        separation: 0,
    },
    allow_extensions: crate::ui_local_h::menulist_s {
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
        oldvalue: 0,
        curvalue: 0,
        numitems: 0,
        top: 0,
        itemnames: 0 as *const *const libc::c_char as *mut *const libc::c_char,
        width: 0,
        height: 0,
        columns: 0,
        separation: 0,
    },
    texturebits: crate::ui_local_h::menulist_s {
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
        oldvalue: 0,
        curvalue: 0,
        numitems: 0,
        top: 0,
        itemnames: 0 as *const *const libc::c_char as *mut *const libc::c_char,
        width: 0,
        height: 0,
        columns: 0,
        separation: 0,
    },
    colordepth: crate::ui_local_h::menulist_s {
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
        oldvalue: 0,
        curvalue: 0,
        numitems: 0,
        top: 0,
        itemnames: 0 as *const *const libc::c_char as *mut *const libc::c_char,
        width: 0,
        height: 0,
        columns: 0,
        separation: 0,
    },
    geometry: crate::ui_local_h::menulist_s {
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
        oldvalue: 0,
        curvalue: 0,
        numitems: 0,
        top: 0,
        itemnames: 0 as *const *const libc::c_char as *mut *const libc::c_char,
        width: 0,
        height: 0,
        columns: 0,
        separation: 0,
    },
    filter: crate::ui_local_h::menulist_s {
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
        oldvalue: 0,
        curvalue: 0,
        numitems: 0,
        top: 0,
        itemnames: 0 as *const *const libc::c_char as *mut *const libc::c_char,
        width: 0,
        height: 0,
        columns: 0,
        separation: 0,
    },
    driverinfo: crate::ui_local_h::menutext_s {
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
    apply: crate::ui_local_h::menubitmap_s {
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

static mut s_ivo_templates: [InitialVideoOptions_s; 6] = [
    {
        let mut init = InitialVideoOptions_s {
            mode: 6 as i32,
            fullscreen: crate::src::qcommon::q_shared::qtrue,
            tq: 3 as i32,
            lighting: 0 as i32,
            colordepth: 2 as i32,
            texturebits: 2 as i32,
            geometry: 2 as i32,
            filter: 1 as i32,
            driver: 0 as i32,
            extensions: crate::src::qcommon::q_shared::qtrue,
        };
        init
    },
    {
        let mut init = InitialVideoOptions_s {
            mode: 4 as i32,
            fullscreen: crate::src::qcommon::q_shared::qtrue,
            tq: 2 as i32,
            lighting: 0 as i32,
            colordepth: 2 as i32,
            texturebits: 2 as i32,
            geometry: 1 as i32,
            filter: 1 as i32,
            driver: 0 as i32,
            extensions: crate::src::qcommon::q_shared::qtrue,
        };
        init
    },
    {
        let mut init = InitialVideoOptions_s {
            mode: 3 as i32,
            fullscreen: crate::src::qcommon::q_shared::qtrue,
            tq: 2 as i32,
            lighting: 0 as i32,
            colordepth: 0 as i32,
            texturebits: 0 as i32,
            geometry: 1 as i32,
            filter: 0 as i32,
            driver: 0 as i32,
            extensions: crate::src::qcommon::q_shared::qtrue,
        };
        init
    },
    {
        let mut init = InitialVideoOptions_s {
            mode: 2 as i32,
            fullscreen: crate::src::qcommon::q_shared::qtrue,
            tq: 1 as i32,
            lighting: 0 as i32,
            colordepth: 1 as i32,
            texturebits: 0 as i32,
            geometry: 0 as i32,
            filter: 0 as i32,
            driver: 0 as i32,
            extensions: crate::src::qcommon::q_shared::qtrue,
        };
        init
    },
    {
        let mut init = InitialVideoOptions_s {
            mode: 2 as i32,
            fullscreen: crate::src::qcommon::q_shared::qtrue,
            tq: 1 as i32,
            lighting: 1 as i32,
            colordepth: 1 as i32,
            texturebits: 0 as i32,
            geometry: 0 as i32,
            filter: 0 as i32,
            driver: 0 as i32,
            extensions: crate::src::qcommon::q_shared::qtrue,
        };
        init
    },
    {
        let mut init = InitialVideoOptions_s {
            mode: 3 as i32,
            fullscreen: crate::src::qcommon::q_shared::qtrue,
            tq: 1 as i32,
            lighting: 0 as i32,
            colordepth: 0 as i32,
            texturebits: 0 as i32,
            geometry: 1 as i32,
            filter: 0 as i32,
            driver: 0 as i32,
            extensions: crate::src::qcommon::q_shared::qtrue,
        };
        init
    },
];

static mut builtinResolutions: [*const libc::c_char; 13] = [
    b"320x240\x00" as *const u8 as *const libc::c_char,
    b"400x300\x00" as *const u8 as *const libc::c_char,
    b"512x384\x00" as *const u8 as *const libc::c_char,
    b"640x480\x00" as *const u8 as *const libc::c_char,
    b"800x600\x00" as *const u8 as *const libc::c_char,
    b"960x720\x00" as *const u8 as *const libc::c_char,
    b"1024x768\x00" as *const u8 as *const libc::c_char,
    b"1152x864\x00" as *const u8 as *const libc::c_char,
    b"1280x1024\x00" as *const u8 as *const libc::c_char,
    b"1600x1200\x00" as *const u8 as *const libc::c_char,
    b"2048x1536\x00" as *const u8 as *const libc::c_char,
    b"856x480\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];

static mut knownRatios: [[*const libc::c_char; 2]; 8] = [
    [
        b"1.25:1\x00" as *const u8 as *const libc::c_char,
        b"5:4\x00" as *const u8 as *const libc::c_char,
    ],
    [
        b"1.33:1\x00" as *const u8 as *const libc::c_char,
        b"4:3\x00" as *const u8 as *const libc::c_char,
    ],
    [
        b"1.50:1\x00" as *const u8 as *const libc::c_char,
        b"3:2\x00" as *const u8 as *const libc::c_char,
    ],
    [
        b"1.56:1\x00" as *const u8 as *const libc::c_char,
        b"14:9\x00" as *const u8 as *const libc::c_char,
    ],
    [
        b"1.60:1\x00" as *const u8 as *const libc::c_char,
        b"16:10\x00" as *const u8 as *const libc::c_char,
    ],
    [
        b"1.67:1\x00" as *const u8 as *const libc::c_char,
        b"5:3\x00" as *const u8 as *const libc::c_char,
    ],
    [
        b"1.78:1\x00" as *const u8 as *const libc::c_char,
        b"16:9\x00" as *const u8 as *const libc::c_char,
    ],
    [0 as *const libc::c_char, 0 as *const libc::c_char],
];

static mut ratios: [*const libc::c_char; 32] = [0 as *const libc::c_char; 32];

static mut ratioBuf: [[libc::c_char; 8]; 32] = [[0; 8]; 32];

static mut ratioToRes: [i32; 32] = [0; 32];

static mut resToRatio: [i32; 32] = [0; 32];

static mut resbuf: [libc::c_char; 1024] = [0; 1024];

static mut detectedResolutions: [*const libc::c_char; 32] = [0 as *const libc::c_char; 32];

static mut resolutions: *mut *const libc::c_char = unsafe { builtinResolutions.as_ptr() as *mut _ };

static mut resolutionsDetected: crate::src::qcommon::q_shared::qboolean =
    crate::src::qcommon::q_shared::qfalse;
/*
=================
GraphicsOptions_FindBuiltinResolution
=================
*/

unsafe extern "C" fn GraphicsOptions_FindBuiltinResolution(mut mode: i32) -> i32 {
    let mut i: i32 = 0;
    if resolutionsDetected as u64 == 0 {
        return mode;
    }
    if mode < 0 as i32 {
        return -(1 as i32);
    }
    i = 0 as i32;
    while !builtinResolutions[i as usize].is_null() {
        if crate::src::qcommon::q_shared::Q_stricmp(
            builtinResolutions[i as usize],
            detectedResolutions[mode as usize],
        ) == 0
        {
            return i;
        }
        i += 1
    }
    return -(1 as i32);
}
/*
=================
GraphicsOptions_FindDetectedResolution
=================
*/

unsafe extern "C" fn GraphicsOptions_FindDetectedResolution(mut mode: i32) -> i32 {
    let mut i: i32 = 0;
    if resolutionsDetected as u64 == 0 {
        return mode;
    }
    if mode < 0 as i32 {
        return -(1 as i32);
    }
    i = 0 as i32;
    while !detectedResolutions[i as usize].is_null() {
        if crate::src::qcommon::q_shared::Q_stricmp(
            builtinResolutions[mode as usize],
            detectedResolutions[i as usize],
        ) == 0
        {
            return i;
        }
        i += 1
    }
    return -(1 as i32);
}
/*
=================
GraphicsOptions_GetAspectRatios
=================
*/

unsafe extern "C" fn GraphicsOptions_GetAspectRatios() {
    let mut i: i32 = 0;
    let mut r: i32 = 0;
    // build ratio list from resolutions
    r = 0 as i32;
    while !(*resolutions.offset(r as isize)).is_null() {
        let mut w: i32 = 0;
        let mut h: i32 = 0;
        let mut x: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut str: [libc::c_char; 8] = [0; 8];
        // calculate resolution's aspect ratio
        x = ::libc::strchr(*resolutions.offset(r as isize), 'x' as i32).offset(1 as i32 as isize);
        crate::src::qcommon::q_shared::Q_strncpyz(
            str.as_mut_ptr(),
            *resolutions.offset(r as isize),
            x.offset_from(*resolutions.offset(r as isize)) as libc::c_long as i32,
        );
        w = atoi(str.as_mut_ptr());
        h = atoi(x);
        crate::src::qcommon::q_shared::Com_sprintf(
            str.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong as i32,
            b"%.2f:1\x00" as *const u8 as *const libc::c_char,
            (w as f32 / h as f32) as f64,
        );
        // rename common ratios ("1.33:1" -> "4:3")
        i = 0 as i32;
        while !knownRatios[i as usize][0 as i32 as usize].is_null() {
            if crate::src::qcommon::q_shared::Q_stricmp(
                str.as_mut_ptr(),
                knownRatios[i as usize][0 as i32 as usize],
            ) == 0
            {
                crate::src::qcommon::q_shared::Q_strncpyz(
                    str.as_mut_ptr(),
                    knownRatios[i as usize][1 as i32 as usize],
                    ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong as i32,
                );
                break;
            } else {
                i += 1
            }
        }
        // add ratio to list if it is new
        // establish res/ratio relationship
        i = 0 as i32;
        while ratioBuf[i as usize][0 as i32 as usize] != 0 {
            if crate::src::qcommon::q_shared::Q_stricmp(
                str.as_mut_ptr(),
                ratioBuf[i as usize].as_mut_ptr(),
            ) == 0
            {
                break;
            }
            i += 1
        }
        if ratioBuf[i as usize][0 as i32 as usize] == 0 {
            crate::src::qcommon::q_shared::Q_strncpyz(
                ratioBuf[i as usize].as_mut_ptr(),
                str.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong as i32,
            );
            ratioToRes[i as usize] = r
        }
        ratios[r as usize] = ratioBuf[r as usize].as_mut_ptr();
        resToRatio[r as usize] = i;
        r += 1
    }
    ratios[r as usize] = 0 as *const libc::c_char;
}
/*
=================
GraphicsOptions_GetInitialVideo
=================
*/

unsafe extern "C" fn GraphicsOptions_GetInitialVideo() {
    s_ivo.colordepth = s_graphicsoptions.colordepth.curvalue;
    s_ivo.driver = s_graphicsoptions.driver.curvalue;
    s_ivo.mode = s_graphicsoptions.mode.curvalue;
    s_ivo.fullscreen = s_graphicsoptions.fs.curvalue as crate::src::qcommon::q_shared::qboolean;
    s_ivo.extensions =
        s_graphicsoptions.allow_extensions.curvalue as crate::src::qcommon::q_shared::qboolean;
    s_ivo.tq = s_graphicsoptions.tq.curvalue as i32;
    s_ivo.lighting = s_graphicsoptions.lighting.curvalue;
    s_ivo.geometry = s_graphicsoptions.geometry.curvalue;
    s_ivo.filter = s_graphicsoptions.filter.curvalue;
    s_ivo.texturebits = s_graphicsoptions.texturebits.curvalue;
}
/*
=================
GraphicsOptions_GetResolutions
=================
*/

unsafe extern "C" fn GraphicsOptions_GetResolutions() {
    crate::src::qcommon::q_shared::Q_strncpyz(
        resbuf.as_mut_ptr(),
        crate::src::q3_ui::ui_atoms::UI_Cvar_VariableString(
            b"r_availableModes\x00" as *const u8 as *const libc::c_char,
        ),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as i32,
    );
    if *resbuf.as_mut_ptr() != 0 {
        let mut s: *mut libc::c_char = resbuf.as_mut_ptr();
        let mut i: u32 = 0 as i32 as u32;
        while !s.is_null()
            && (i as libc::c_ulong)
                < (::std::mem::size_of::<[*const libc::c_char; 32]>() as libc::c_ulong)
                    .wrapping_div(::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
                    .wrapping_sub(1 as i32 as libc::c_ulong)
        {
            let fresh2 = i;
            i = i.wrapping_add(1);
            detectedResolutions[fresh2 as usize] = s;
            s = ::libc::strchr(s, ' ' as i32);
            if !s.is_null() {
                let fresh3 = s;
                s = s.offset(1);
                *fresh3 = '\u{0}' as i32 as libc::c_char
            }
        }
        detectedResolutions[i as usize] = 0 as *const libc::c_char;
        if i > 0 as i32 as u32 {
            resolutions = detectedResolutions.as_mut_ptr();
            resolutionsDetected = crate::src::qcommon::q_shared::qtrue
        }
    };
}
/*
=================
GraphicsOptions_CheckConfig
=================
*/

unsafe extern "C" fn GraphicsOptions_CheckConfig() {
    let mut i: i32 = 0;
    i = 0 as i32;
    while (i as libc::c_ulong)
        < (::std::mem::size_of::<[InitialVideoOptions_s; 6]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<InitialVideoOptions_s>() as libc::c_ulong)
            .wrapping_sub(1 as i32 as libc::c_ulong)
    {
        if !(s_ivo_templates[i as usize].colordepth != s_graphicsoptions.colordepth.curvalue) {
            if !(s_ivo_templates[i as usize].driver != s_graphicsoptions.driver.curvalue) {
                if !(GraphicsOptions_FindDetectedResolution(s_ivo_templates[i as usize].mode)
                    != s_graphicsoptions.mode.curvalue)
                {
                    if !(s_ivo_templates[i as usize].fullscreen as u32
                        != s_graphicsoptions.fs.curvalue as u32)
                    {
                        if !(s_ivo_templates[i as usize].tq as f32 != s_graphicsoptions.tq.curvalue)
                        {
                            if !(s_ivo_templates[i as usize].lighting
                                != s_graphicsoptions.lighting.curvalue)
                            {
                                if !(s_ivo_templates[i as usize].geometry
                                    != s_graphicsoptions.geometry.curvalue)
                                {
                                    if !(s_ivo_templates[i as usize].filter
                                        != s_graphicsoptions.filter.curvalue)
                                    {
                                        //		if ( s_ivo_templates[i].texturebits != s_graphicsoptions.texturebits.curvalue )
                                        //			continue;
                                        s_graphicsoptions.list.curvalue = i;
                                        return;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        i += 1
    }
    // return 'Custom' ivo template
    s_graphicsoptions.list.curvalue = (::std::mem::size_of::<[InitialVideoOptions_s; 6]>()
        as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<InitialVideoOptions_s>() as libc::c_ulong)
        .wrapping_sub(1 as i32 as libc::c_ulong) as i32;
}
/*
=================
GraphicsOptions_UpdateMenuItems
=================
*/

unsafe extern "C" fn GraphicsOptions_UpdateMenuItems() {
    if s_graphicsoptions.driver.curvalue == 1 as i32 {
        s_graphicsoptions.fs.curvalue = 1 as i32;
        s_graphicsoptions.fs.generic.flags |= 0x2000 as i32 as u32;
        s_graphicsoptions.colordepth.curvalue = 1 as i32
    } else {
        s_graphicsoptions.fs.generic.flags &= !(0x2000 as i32 as u32)
    }
    if s_graphicsoptions.fs.curvalue == 0 as i32 || s_graphicsoptions.driver.curvalue == 1 as i32 {
        s_graphicsoptions.colordepth.curvalue = 0 as i32;
        s_graphicsoptions.colordepth.generic.flags |= 0x2000 as i32 as u32
    } else {
        s_graphicsoptions.colordepth.generic.flags &= !(0x2000 as i32 as u32)
    }
    if s_graphicsoptions.allow_extensions.curvalue == 0 as i32 {
        if s_graphicsoptions.texturebits.curvalue == 0 as i32 {
            s_graphicsoptions.texturebits.curvalue = 1 as i32
        }
    }
    s_graphicsoptions.apply.generic.flags |= 0x1000 as i32 as u32 | 0x4000 as i32 as u32;
    if s_ivo.mode != s_graphicsoptions.mode.curvalue {
        s_graphicsoptions.apply.generic.flags &= !(0x1000 as i32 as u32 | 0x4000 as i32 as u32)
    }
    if s_ivo.fullscreen as u32 != s_graphicsoptions.fs.curvalue as u32 {
        s_graphicsoptions.apply.generic.flags &= !(0x1000 as i32 as u32 | 0x4000 as i32 as u32)
    }
    if s_ivo.extensions as u32 != s_graphicsoptions.allow_extensions.curvalue as u32 {
        s_graphicsoptions.apply.generic.flags &= !(0x1000 as i32 as u32 | 0x4000 as i32 as u32)
    }
    if s_ivo.tq as f32 != s_graphicsoptions.tq.curvalue {
        s_graphicsoptions.apply.generic.flags &= !(0x1000 as i32 as u32 | 0x4000 as i32 as u32)
    }
    if s_ivo.lighting != s_graphicsoptions.lighting.curvalue {
        s_graphicsoptions.apply.generic.flags &= !(0x1000 as i32 as u32 | 0x4000 as i32 as u32)
    }
    if s_ivo.colordepth != s_graphicsoptions.colordepth.curvalue {
        s_graphicsoptions.apply.generic.flags &= !(0x1000 as i32 as u32 | 0x4000 as i32 as u32)
    }
    if s_ivo.driver != s_graphicsoptions.driver.curvalue {
        s_graphicsoptions.apply.generic.flags &= !(0x1000 as i32 as u32 | 0x4000 as i32 as u32)
    }
    if s_ivo.texturebits != s_graphicsoptions.texturebits.curvalue {
        s_graphicsoptions.apply.generic.flags &= !(0x1000 as i32 as u32 | 0x4000 as i32 as u32)
    }
    if s_ivo.geometry != s_graphicsoptions.geometry.curvalue {
        s_graphicsoptions.apply.generic.flags &= !(0x1000 as i32 as u32 | 0x4000 as i32 as u32)
    }
    if s_ivo.filter != s_graphicsoptions.filter.curvalue {
        s_graphicsoptions.apply.generic.flags &= !(0x1000 as i32 as u32 | 0x4000 as i32 as u32)
    }
    GraphicsOptions_CheckConfig();
}
/*
=================
GraphicsOptions_ApplyChanges
=================
*/

unsafe extern "C" fn GraphicsOptions_ApplyChanges(
    mut _unused: *mut libc::c_void,
    mut notification: i32,
) {
    if notification != 3 as i32 {
        return;
    }
    match s_graphicsoptions.texturebits.curvalue {
        0 => {
            crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
                b"r_texturebits\x00" as *const u8 as *const libc::c_char,
                0 as i32 as f32,
            );
        }
        1 => {
            crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
                b"r_texturebits\x00" as *const u8 as *const libc::c_char,
                16 as i32 as f32,
            );
        }
        2 => {
            crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
                b"r_texturebits\x00" as *const u8 as *const libc::c_char,
                32 as i32 as f32,
            );
        }
        _ => {}
    }
    crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
        b"r_picmip\x00" as *const u8 as *const libc::c_char,
        3 as i32 as f32 - s_graphicsoptions.tq.curvalue,
    );
    crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
        b"r_allowExtensions\x00" as *const u8 as *const libc::c_char,
        s_graphicsoptions.allow_extensions.curvalue as f32,
    );
    if resolutionsDetected as u64 != 0 {
        // search for builtin mode that matches the detected mode
        let mut mode: i32 = 0;
        if s_graphicsoptions.mode.curvalue == -(1 as i32)
            || s_graphicsoptions.mode.curvalue as libc::c_ulong
                >= (::std::mem::size_of::<[*const libc::c_char; 32]>() as libc::c_ulong)
                    .wrapping_div(::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
        {
            s_graphicsoptions.mode.curvalue = 0 as i32
        }
        mode = GraphicsOptions_FindBuiltinResolution(s_graphicsoptions.mode.curvalue);
        if mode == -(1 as i32) {
            let mut w: [libc::c_char; 16] = [0; 16];
            let mut h: [libc::c_char; 16] = [0; 16];
            crate::src::qcommon::q_shared::Q_strncpyz(
                w.as_mut_ptr(),
                detectedResolutions[s_graphicsoptions.mode.curvalue as usize],
                ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong as i32,
            );
            *::libc::strchr(w.as_mut_ptr(), 'x' as i32) = 0 as i32 as libc::c_char;
            crate::src::qcommon::q_shared::Q_strncpyz(
                h.as_mut_ptr(),
                ::libc::strchr(
                    detectedResolutions[s_graphicsoptions.mode.curvalue as usize],
                    'x' as i32,
                )
                .offset(1 as i32 as isize),
                ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong as i32,
            );
            crate::src::ui::ui_syscalls::trap_Cvar_Set(
                b"r_customwidth\x00" as *const u8 as *const libc::c_char,
                w.as_mut_ptr(),
            );
            crate::src::ui::ui_syscalls::trap_Cvar_Set(
                b"r_customheight\x00" as *const u8 as *const libc::c_char,
                h.as_mut_ptr(),
            );
        }
        crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
            b"r_mode\x00" as *const u8 as *const libc::c_char,
            mode as f32,
        );
    } else {
        crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
            b"r_mode\x00" as *const u8 as *const libc::c_char,
            s_graphicsoptions.mode.curvalue as f32,
        );
    }
    crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
        b"r_fullscreen\x00" as *const u8 as *const libc::c_char,
        s_graphicsoptions.fs.curvalue as f32,
    );
    match s_graphicsoptions.colordepth.curvalue {
        0 => {
            crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
                b"r_colorbits\x00" as *const u8 as *const libc::c_char,
                0 as i32 as f32,
            );
            crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
                b"r_depthbits\x00" as *const u8 as *const libc::c_char,
                0 as i32 as f32,
            );
            crate::src::ui::ui_syscalls::trap_Cvar_Reset(
                b"r_stencilbits\x00" as *const u8 as *const libc::c_char,
            );
        }
        1 => {
            crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
                b"r_colorbits\x00" as *const u8 as *const libc::c_char,
                16 as i32 as f32,
            );
            crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
                b"r_depthbits\x00" as *const u8 as *const libc::c_char,
                16 as i32 as f32,
            );
            crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
                b"r_stencilbits\x00" as *const u8 as *const libc::c_char,
                0 as i32 as f32,
            );
        }
        2 => {
            crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
                b"r_colorbits\x00" as *const u8 as *const libc::c_char,
                32 as i32 as f32,
            );
            crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
                b"r_depthbits\x00" as *const u8 as *const libc::c_char,
                24 as i32 as f32,
            );
            crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
                b"r_stencilbits\x00" as *const u8 as *const libc::c_char,
                8 as i32 as f32,
            );
        }
        _ => {}
    }
    crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
        b"r_vertexLight\x00" as *const u8 as *const libc::c_char,
        s_graphicsoptions.lighting.curvalue as f32,
    );
    if s_graphicsoptions.geometry.curvalue == 2 as i32 {
        crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
            b"r_lodBias\x00" as *const u8 as *const libc::c_char,
            0 as i32 as f32,
        );
        crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
            b"r_subdivisions\x00" as *const u8 as *const libc::c_char,
            4 as i32 as f32,
        );
    } else if s_graphicsoptions.geometry.curvalue == 1 as i32 {
        crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
            b"r_lodBias\x00" as *const u8 as *const libc::c_char,
            1 as i32 as f32,
        );
        crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
            b"r_subdivisions\x00" as *const u8 as *const libc::c_char,
            12 as i32 as f32,
        );
    } else {
        crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
            b"r_lodBias\x00" as *const u8 as *const libc::c_char,
            1 as i32 as f32,
        );
        crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
            b"r_subdivisions\x00" as *const u8 as *const libc::c_char,
            20 as i32 as f32,
        );
    }
    if s_graphicsoptions.filter.curvalue != 0 {
        crate::src::ui::ui_syscalls::trap_Cvar_Set(
            b"r_textureMode\x00" as *const u8 as *const libc::c_char,
            b"GL_LINEAR_MIPMAP_LINEAR\x00" as *const u8 as *const libc::c_char,
        );
    } else {
        crate::src::ui::ui_syscalls::trap_Cvar_Set(
            b"r_textureMode\x00" as *const u8 as *const libc::c_char,
            b"GL_LINEAR_MIPMAP_NEAREST\x00" as *const u8 as *const libc::c_char,
        );
    }
    crate::src::ui::ui_syscalls::trap_Cmd_ExecuteText(
        crate::src::qcommon::q_shared::EXEC_APPEND as i32,
        b"vid_restart\n\x00" as *const u8 as *const libc::c_char,
    );
}
/*
=================
GraphicsOptions_Event
=================
*/

unsafe extern "C" fn GraphicsOptions_Event(mut ptr: *mut libc::c_void, mut event: i32) {
    let mut ivo: *mut InitialVideoOptions_s = 0 as *mut InitialVideoOptions_s;
    if event != 3 as i32 {
        return;
    }
    let mut current_block_28: u64;
    match (*(ptr as *mut crate::ui_local_h::menucommon_s)).id {
        110 => {
            s_graphicsoptions.mode.curvalue = ratioToRes[s_graphicsoptions.ratio.curvalue as usize];
            current_block_28 = 6581794399938936269;
        }
        104 => {
            current_block_28 = 6581794399938936269;
        }
        103 => {
            ivo = &mut *s_ivo_templates
                .as_mut_ptr()
                .offset(s_graphicsoptions.list.curvalue as isize)
                as *mut InitialVideoOptions_s;
            s_graphicsoptions.mode.curvalue = GraphicsOptions_FindDetectedResolution((*ivo).mode);
            s_graphicsoptions.ratio.curvalue = resToRatio[s_graphicsoptions.mode.curvalue as usize];
            s_graphicsoptions.tq.curvalue = (*ivo).tq as f32;
            s_graphicsoptions.lighting.curvalue = (*ivo).lighting;
            s_graphicsoptions.colordepth.curvalue = (*ivo).colordepth;
            s_graphicsoptions.texturebits.curvalue = (*ivo).texturebits;
            s_graphicsoptions.geometry.curvalue = (*ivo).geometry;
            s_graphicsoptions.filter.curvalue = (*ivo).filter;
            s_graphicsoptions.fs.curvalue = (*ivo).fullscreen as i32;
            current_block_28 = 652864300344834934;
        }
        105 => {
            UI_DriverInfo_Menu();
            current_block_28 = 652864300344834934;
        }
        101 => {
            crate::src::q3_ui::ui_atoms::UI_PopMenu();
            current_block_28 = 652864300344834934;
        }
        107 => {
            crate::src::q3_ui::ui_atoms::UI_PopMenu();
            crate::src::q3_ui::ui_display::UI_DisplayOptionsMenu();
            current_block_28 = 652864300344834934;
        }
        108 => {
            crate::src::q3_ui::ui_atoms::UI_PopMenu();
            crate::src::q3_ui::ui_sound::UI_SoundOptionsMenu();
            current_block_28 = 652864300344834934;
        }
        109 => {
            crate::src::q3_ui::ui_atoms::UI_PopMenu();
            crate::src::q3_ui::ui_network::UI_NetworkOptionsMenu();
            current_block_28 = 652864300344834934;
        }
        106 | _ => {
            current_block_28 = 652864300344834934;
        }
    }
    match current_block_28 {
        6581794399938936269 =>
        // fall through to apply mode constraints
        // clamp 3dfx video modes
        {
            if s_graphicsoptions.driver.curvalue == 1 as i32 {
                if s_graphicsoptions.mode.curvalue < 2 as i32 {
                    s_graphicsoptions.mode.curvalue = 2 as i32
                } else if s_graphicsoptions.mode.curvalue > 6 as i32 {
                    s_graphicsoptions.mode.curvalue = 6 as i32
                }
            }
            s_graphicsoptions.ratio.curvalue = resToRatio[s_graphicsoptions.mode.curvalue as usize]
        }
        _ => {}
    };
}
/*
================
GraphicsOptions_TQEvent
================
*/

unsafe extern "C" fn GraphicsOptions_TQEvent(mut _ptr: *mut libc::c_void, mut event: i32) {
    if event != 3 as i32 {
        return;
    }
    s_graphicsoptions.tq.curvalue = (s_graphicsoptions.tq.curvalue as f64 + 0.5f64) as i32 as f32;
}
/*
================
GraphicsOptions_MenuDraw
================
*/
#[no_mangle]

pub unsafe extern "C" fn GraphicsOptions_MenuDraw() {
    //APSFIX - rework this
    GraphicsOptions_UpdateMenuItems();
    crate::src::q3_ui::ui_qmenu::Menu_Draw(
        &mut s_graphicsoptions.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
    );
}
/*
=================
GraphicsOptions_SetMenuItems
=================
*/

unsafe extern "C" fn GraphicsOptions_SetMenuItems() {
    s_graphicsoptions.mode.curvalue = GraphicsOptions_FindDetectedResolution(
        crate::src::ui::ui_syscalls::trap_Cvar_VariableValue(
            b"r_mode\x00" as *const u8 as *const libc::c_char,
        ) as i32,
    );
    if s_graphicsoptions.mode.curvalue < 0 as i32 {
        if resolutionsDetected as u64 != 0 {
            let mut i: i32 = 0;
            let mut buf: [libc::c_char; 1024] = [0; 1024];
            crate::src::ui::ui_syscalls::trap_Cvar_VariableStringBuffer(
                b"r_customwidth\x00" as *const u8 as *const libc::c_char,
                buf.as_mut_ptr(),
                (::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong)
                    .wrapping_sub(2 as i32 as libc::c_ulong) as i32,
            );
            buf[crate::stdlib::strlen(buf.as_mut_ptr()).wrapping_add(1 as i32 as libc::c_ulong)
                as usize] = 0 as i32 as libc::c_char;
            buf[crate::stdlib::strlen(buf.as_mut_ptr()) as usize] = 'x' as i32 as libc::c_char;
            crate::src::ui::ui_syscalls::trap_Cvar_VariableStringBuffer(
                b"r_customheight\x00" as *const u8 as *const libc::c_char,
                buf.as_mut_ptr()
                    .offset(crate::stdlib::strlen(buf.as_mut_ptr()) as isize),
                (::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong)
                    .wrapping_sub(crate::stdlib::strlen(buf.as_mut_ptr())) as i32,
            );
            i = 0 as i32;
            while !detectedResolutions[i as usize].is_null() {
                if crate::src::qcommon::q_shared::Q_stricmp(
                    buf.as_mut_ptr(),
                    detectedResolutions[i as usize],
                ) == 0
                {
                    s_graphicsoptions.mode.curvalue = i;
                    break;
                } else {
                    i += 1
                }
            }
            if s_graphicsoptions.mode.curvalue < 0 as i32 {
                s_graphicsoptions.mode.curvalue = 0 as i32
            }
        } else {
            s_graphicsoptions.mode.curvalue = 3 as i32
        }
    }
    s_graphicsoptions.ratio.curvalue = resToRatio[s_graphicsoptions.mode.curvalue as usize];
    s_graphicsoptions.fs.curvalue = crate::src::ui::ui_syscalls::trap_Cvar_VariableValue(
        b"r_fullscreen\x00" as *const u8 as *const libc::c_char,
    ) as i32;
    s_graphicsoptions.allow_extensions.curvalue =
        crate::src::ui::ui_syscalls::trap_Cvar_VariableValue(
            b"r_allowExtensions\x00" as *const u8 as *const libc::c_char,
        ) as i32;
    s_graphicsoptions.tq.curvalue = 3 as i32 as f32
        - crate::src::ui::ui_syscalls::trap_Cvar_VariableValue(
            b"r_picmip\x00" as *const u8 as *const libc::c_char,
        );
    if s_graphicsoptions.tq.curvalue < 0 as i32 as f32 {
        s_graphicsoptions.tq.curvalue = 0 as i32 as f32
    } else if s_graphicsoptions.tq.curvalue > 3 as i32 as f32 {
        s_graphicsoptions.tq.curvalue = 3 as i32 as f32
    }
    s_graphicsoptions.lighting.curvalue = (crate::src::ui::ui_syscalls::trap_Cvar_VariableValue(
        b"r_vertexLight\x00" as *const u8 as *const libc::c_char,
    ) != 0 as i32 as f32) as i32;
    match crate::src::ui::ui_syscalls::trap_Cvar_VariableValue(
        b"r_texturebits\x00" as *const u8 as *const libc::c_char,
    ) as i32
    {
        16 => s_graphicsoptions.texturebits.curvalue = 1 as i32,
        32 => s_graphicsoptions.texturebits.curvalue = 2 as i32,
        0 | _ => s_graphicsoptions.texturebits.curvalue = 0 as i32,
    }
    if crate::src::qcommon::q_shared::Q_stricmp(
        crate::src::q3_ui::ui_atoms::UI_Cvar_VariableString(
            b"r_textureMode\x00" as *const u8 as *const libc::c_char,
        ),
        b"GL_LINEAR_MIPMAP_NEAREST\x00" as *const u8 as *const libc::c_char,
    ) == 0
    {
        s_graphicsoptions.filter.curvalue = 0 as i32
    } else {
        s_graphicsoptions.filter.curvalue = 1 as i32
    }
    if crate::src::ui::ui_syscalls::trap_Cvar_VariableValue(
        b"r_lodBias\x00" as *const u8 as *const libc::c_char,
    ) > 0 as i32 as f32
    {
        if crate::src::ui::ui_syscalls::trap_Cvar_VariableValue(
            b"r_subdivisions\x00" as *const u8 as *const libc::c_char,
        ) >= 20 as i32 as f32
        {
            s_graphicsoptions.geometry.curvalue = 0 as i32
        } else {
            s_graphicsoptions.geometry.curvalue = 1 as i32
        }
    } else {
        s_graphicsoptions.geometry.curvalue = 2 as i32
    }
    match crate::src::ui::ui_syscalls::trap_Cvar_VariableValue(
        b"r_colorbits\x00" as *const u8 as *const libc::c_char,
    ) as i32
    {
        16 => s_graphicsoptions.colordepth.curvalue = 1 as i32,
        32 => s_graphicsoptions.colordepth.curvalue = 2 as i32,
        0 | _ => s_graphicsoptions.colordepth.curvalue = 0 as i32,
    }
    if s_graphicsoptions.fs.curvalue == 0 as i32 {
        s_graphicsoptions.colordepth.curvalue = 0 as i32
    }
    if s_graphicsoptions.driver.curvalue == 1 as i32 {
        s_graphicsoptions.colordepth.curvalue = 1 as i32
    };
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
================
GraphicsOptions_MenuInit
================
*/
#[no_mangle]

pub unsafe extern "C" fn GraphicsOptions_MenuInit() {
    static mut s_driver_names: [*const libc::c_char; 3] = [
        b"Default\x00" as *const u8 as *const libc::c_char,
        b"Voodoo\x00" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    static mut tq_names: [*const libc::c_char; 4] = [
        b"Default\x00" as *const u8 as *const libc::c_char,
        b"16 bit\x00" as *const u8 as *const libc::c_char,
        b"32 bit\x00" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    static mut s_graphics_options_names: [*const libc::c_char; 7] = [
        b"Very High Quality\x00" as *const u8 as *const libc::c_char,
        b"High Quality\x00" as *const u8 as *const libc::c_char,
        b"Normal\x00" as *const u8 as *const libc::c_char,
        b"Fast\x00" as *const u8 as *const libc::c_char,
        b"Fastest\x00" as *const u8 as *const libc::c_char,
        b"Custom\x00" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    static mut lighting_names: [*const libc::c_char; 3] = [
        b"Lightmap\x00" as *const u8 as *const libc::c_char,
        b"Vertex\x00" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    static mut colordepth_names: [*const libc::c_char; 4] = [
        b"Default\x00" as *const u8 as *const libc::c_char,
        b"16 bit\x00" as *const u8 as *const libc::c_char,
        b"32 bit\x00" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    static mut filter_names: [*const libc::c_char; 3] = [
        b"Bilinear\x00" as *const u8 as *const libc::c_char,
        b"Trilinear\x00" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    static mut quality_names: [*const libc::c_char; 4] = [
        b"Low\x00" as *const u8 as *const libc::c_char,
        b"Medium\x00" as *const u8 as *const libc::c_char,
        b"High\x00" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    static mut enabled_names: [*const libc::c_char; 3] = [
        b"Off\x00" as *const u8 as *const libc::c_char,
        b"On\x00" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut y: i32 = 0;
    // zero set all our globals
    crate::stdlib::memset(
        &mut s_graphicsoptions as *mut graphicsoptions_t as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<graphicsoptions_t>() as libc::c_ulong,
    );
    GraphicsOptions_GetResolutions();
    GraphicsOptions_GetAspectRatios();
    GraphicsOptions_Cache();
    s_graphicsoptions.menu.wrapAround = crate::src::qcommon::q_shared::qtrue;
    s_graphicsoptions.menu.fullscreen = crate::src::qcommon::q_shared::qtrue;
    s_graphicsoptions.menu.draw = Some(GraphicsOptions_MenuDraw as unsafe extern "C" fn() -> ());
    s_graphicsoptions.banner.generic.type_0 = 10 as i32;
    s_graphicsoptions.banner.generic.x = 320 as i32;
    s_graphicsoptions.banner.generic.y = 16 as i32;
    s_graphicsoptions.banner.string =
        b"SYSTEM SETUP\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_graphicsoptions.banner.color = crate::src::q3_ui::ui_qmenu::color_white.as_mut_ptr();
    s_graphicsoptions.banner.style = 0x1 as i32;
    s_graphicsoptions.framel.generic.type_0 = 6 as i32;
    s_graphicsoptions.framel.generic.name =
        b"menu/art/frame2_l\x00" as *const u8 as *const libc::c_char;
    s_graphicsoptions.framel.generic.flags = 0x4000 as i32 as u32;
    s_graphicsoptions.framel.generic.x = 0 as i32;
    s_graphicsoptions.framel.generic.y = 78 as i32;
    s_graphicsoptions.framel.width = 256 as i32;
    s_graphicsoptions.framel.height = 329 as i32;
    s_graphicsoptions.framer.generic.type_0 = 6 as i32;
    s_graphicsoptions.framer.generic.name =
        b"menu/art/frame1_r\x00" as *const u8 as *const libc::c_char;
    s_graphicsoptions.framer.generic.flags = 0x4000 as i32 as u32;
    s_graphicsoptions.framer.generic.x = 376 as i32;
    s_graphicsoptions.framer.generic.y = 76 as i32;
    s_graphicsoptions.framer.width = 256 as i32;
    s_graphicsoptions.framer.height = 334 as i32;
    s_graphicsoptions.graphics.generic.type_0 = 9 as i32;
    s_graphicsoptions.graphics.generic.flags = 0x10 as i32 as u32;
    s_graphicsoptions.graphics.generic.id = 106 as i32;
    s_graphicsoptions.graphics.generic.callback =
        Some(GraphicsOptions_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_graphicsoptions.graphics.generic.x = 216 as i32;
    s_graphicsoptions.graphics.generic.y = 240 as i32 - 2 as i32 * 27 as i32;
    s_graphicsoptions.graphics.string =
        b"GRAPHICS\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_graphicsoptions.graphics.style = 0x2 as i32;
    s_graphicsoptions.graphics.color = crate::src::q3_ui::ui_qmenu::color_red.as_mut_ptr();
    s_graphicsoptions.display.generic.type_0 = 9 as i32;
    s_graphicsoptions.display.generic.flags = 0x10 as i32 as u32 | 0x100 as i32 as u32;
    s_graphicsoptions.display.generic.id = 107 as i32;
    s_graphicsoptions.display.generic.callback =
        Some(GraphicsOptions_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_graphicsoptions.display.generic.x = 216 as i32;
    s_graphicsoptions.display.generic.y = 240 as i32 - 27 as i32;
    s_graphicsoptions.display.string =
        b"DISPLAY\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_graphicsoptions.display.style = 0x2 as i32;
    s_graphicsoptions.display.color = crate::src::q3_ui::ui_qmenu::color_red.as_mut_ptr();
    s_graphicsoptions.sound.generic.type_0 = 9 as i32;
    s_graphicsoptions.sound.generic.flags = 0x10 as i32 as u32 | 0x100 as i32 as u32;
    s_graphicsoptions.sound.generic.id = 108 as i32;
    s_graphicsoptions.sound.generic.callback =
        Some(GraphicsOptions_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_graphicsoptions.sound.generic.x = 216 as i32;
    s_graphicsoptions.sound.generic.y = 240 as i32;
    s_graphicsoptions.sound.string =
        b"SOUND\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_graphicsoptions.sound.style = 0x2 as i32;
    s_graphicsoptions.sound.color = crate::src::q3_ui::ui_qmenu::color_red.as_mut_ptr();
    s_graphicsoptions.network.generic.type_0 = 9 as i32;
    s_graphicsoptions.network.generic.flags = 0x10 as i32 as u32 | 0x100 as i32 as u32;
    s_graphicsoptions.network.generic.id = 109 as i32;
    s_graphicsoptions.network.generic.callback =
        Some(GraphicsOptions_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_graphicsoptions.network.generic.x = 216 as i32;
    s_graphicsoptions.network.generic.y = 240 as i32 + 27 as i32;
    s_graphicsoptions.network.string =
        b"NETWORK\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_graphicsoptions.network.style = 0x2 as i32;
    s_graphicsoptions.network.color = crate::src::q3_ui::ui_qmenu::color_red.as_mut_ptr();
    y = 240 as i32 - 7 as i32 * (16 as i32 + 2 as i32);
    s_graphicsoptions.list.generic.type_0 = 3 as i32;
    s_graphicsoptions.list.generic.name =
        b"Graphics Settings:\x00" as *const u8 as *const libc::c_char;
    s_graphicsoptions.list.generic.flags = 0x100 as i32 as u32 | 0x2 as i32 as u32;
    s_graphicsoptions.list.generic.x = 400 as i32;
    s_graphicsoptions.list.generic.y = y;
    s_graphicsoptions.list.generic.callback =
        Some(GraphicsOptions_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_graphicsoptions.list.generic.id = 103 as i32;
    s_graphicsoptions.list.itemnames = s_graphics_options_names.as_mut_ptr();
    y += 2 as i32 * (16 as i32 + 2 as i32);
    s_graphicsoptions.driver.generic.type_0 = 3 as i32;
    s_graphicsoptions.driver.generic.name = b"GL Driver:\x00" as *const u8 as *const libc::c_char;
    s_graphicsoptions.driver.generic.flags = 0x100 as i32 as u32 | 0x2 as i32 as u32;
    s_graphicsoptions.driver.generic.x = 400 as i32;
    s_graphicsoptions.driver.generic.y = y;
    s_graphicsoptions.driver.itemnames = s_driver_names.as_mut_ptr();
    s_graphicsoptions.driver.curvalue =
        (crate::src::q3_ui::ui_atoms::uis.glconfig.driverType as u32
            == crate::tr_types_h::GLDRV_VOODOO as i32 as u32) as i32;
    y += 16 as i32 + 2 as i32;
    // references/modifies "r_allowExtensions"
    s_graphicsoptions.allow_extensions.generic.type_0 = 3 as i32;
    s_graphicsoptions.allow_extensions.generic.name =
        b"GL Extensions:\x00" as *const u8 as *const libc::c_char;
    s_graphicsoptions.allow_extensions.generic.flags = 0x100 as i32 as u32 | 0x2 as i32 as u32;
    s_graphicsoptions.allow_extensions.generic.x = 400 as i32;
    s_graphicsoptions.allow_extensions.generic.y = y;
    s_graphicsoptions.allow_extensions.itemnames = enabled_names.as_mut_ptr();
    y += 16 as i32 + 2 as i32;
    s_graphicsoptions.ratio.generic.type_0 = 3 as i32;
    s_graphicsoptions.ratio.generic.name = b"Aspect Ratio:\x00" as *const u8 as *const libc::c_char;
    s_graphicsoptions.ratio.generic.flags = 0x100 as i32 as u32 | 0x2 as i32 as u32;
    s_graphicsoptions.ratio.generic.x = 400 as i32;
    s_graphicsoptions.ratio.generic.y = y;
    s_graphicsoptions.ratio.itemnames = ratios.as_mut_ptr();
    s_graphicsoptions.ratio.generic.callback =
        Some(GraphicsOptions_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_graphicsoptions.ratio.generic.id = 110 as i32;
    y += 16 as i32 + 2 as i32;
    // references/modifies "r_mode"
    s_graphicsoptions.mode.generic.type_0 = 3 as i32;
    s_graphicsoptions.mode.generic.name = b"Resolution:\x00" as *const u8 as *const libc::c_char;
    s_graphicsoptions.mode.generic.flags = 0x100 as i32 as u32 | 0x2 as i32 as u32;
    s_graphicsoptions.mode.generic.x = 400 as i32;
    s_graphicsoptions.mode.generic.y = y;
    s_graphicsoptions.mode.itemnames = resolutions;
    s_graphicsoptions.mode.generic.callback =
        Some(GraphicsOptions_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_graphicsoptions.mode.generic.id = 104 as i32;
    y += 16 as i32 + 2 as i32;
    // references "r_colorbits"
    s_graphicsoptions.colordepth.generic.type_0 = 3 as i32;
    s_graphicsoptions.colordepth.generic.name =
        b"Color Depth:\x00" as *const u8 as *const libc::c_char;
    s_graphicsoptions.colordepth.generic.flags = 0x100 as i32 as u32 | 0x2 as i32 as u32;
    s_graphicsoptions.colordepth.generic.x = 400 as i32;
    s_graphicsoptions.colordepth.generic.y = y;
    s_graphicsoptions.colordepth.itemnames = colordepth_names.as_mut_ptr();
    y += 16 as i32 + 2 as i32;
    // references/modifies "r_fullscreen"
    s_graphicsoptions.fs.generic.type_0 = 3 as i32;
    s_graphicsoptions.fs.generic.name = b"Fullscreen:\x00" as *const u8 as *const libc::c_char;
    s_graphicsoptions.fs.generic.flags = 0x100 as i32 as u32 | 0x2 as i32 as u32;
    s_graphicsoptions.fs.generic.x = 400 as i32;
    s_graphicsoptions.fs.generic.y = y;
    s_graphicsoptions.fs.itemnames = enabled_names.as_mut_ptr();
    y += 16 as i32 + 2 as i32;
    // references/modifies "r_vertexLight"
    s_graphicsoptions.lighting.generic.type_0 = 3 as i32;
    s_graphicsoptions.lighting.generic.name = b"Lighting:\x00" as *const u8 as *const libc::c_char;
    s_graphicsoptions.lighting.generic.flags = 0x100 as i32 as u32 | 0x2 as i32 as u32;
    s_graphicsoptions.lighting.generic.x = 400 as i32;
    s_graphicsoptions.lighting.generic.y = y;
    s_graphicsoptions.lighting.itemnames = lighting_names.as_mut_ptr();
    y += 16 as i32 + 2 as i32;
    // references/modifies "r_lodBias" & "subdivisions"
    s_graphicsoptions.geometry.generic.type_0 = 3 as i32;
    s_graphicsoptions.geometry.generic.name =
        b"Geometric Detail:\x00" as *const u8 as *const libc::c_char;
    s_graphicsoptions.geometry.generic.flags = 0x100 as i32 as u32 | 0x2 as i32 as u32;
    s_graphicsoptions.geometry.generic.x = 400 as i32;
    s_graphicsoptions.geometry.generic.y = y;
    s_graphicsoptions.geometry.itemnames = quality_names.as_mut_ptr();
    y += 16 as i32 + 2 as i32;
    // references/modifies "r_picmip"
    s_graphicsoptions.tq.generic.type_0 = 1 as i32;
    s_graphicsoptions.tq.generic.name = b"Texture Detail:\x00" as *const u8 as *const libc::c_char;
    s_graphicsoptions.tq.generic.flags = 0x100 as i32 as u32 | 0x2 as i32 as u32;
    s_graphicsoptions.tq.generic.x = 400 as i32;
    s_graphicsoptions.tq.generic.y = y;
    s_graphicsoptions.tq.minvalue = 0 as i32 as f32;
    s_graphicsoptions.tq.maxvalue = 3 as i32 as f32;
    s_graphicsoptions.tq.generic.callback =
        Some(GraphicsOptions_TQEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    y += 16 as i32 + 2 as i32;
    // references/modifies "r_textureBits"
    s_graphicsoptions.texturebits.generic.type_0 = 3 as i32;
    s_graphicsoptions.texturebits.generic.name =
        b"Texture Quality:\x00" as *const u8 as *const libc::c_char;
    s_graphicsoptions.texturebits.generic.flags = 0x100 as i32 as u32 | 0x2 as i32 as u32;
    s_graphicsoptions.texturebits.generic.x = 400 as i32;
    s_graphicsoptions.texturebits.generic.y = y;
    s_graphicsoptions.texturebits.itemnames = tq_names.as_mut_ptr();
    y += 16 as i32 + 2 as i32;
    // references/modifies "r_textureMode"
    s_graphicsoptions.filter.generic.type_0 = 3 as i32;
    s_graphicsoptions.filter.generic.name =
        b"Texture Filter:\x00" as *const u8 as *const libc::c_char;
    s_graphicsoptions.filter.generic.flags = 0x100 as i32 as u32 | 0x2 as i32 as u32;
    s_graphicsoptions.filter.generic.x = 400 as i32;
    s_graphicsoptions.filter.generic.y = y;
    s_graphicsoptions.filter.itemnames = filter_names.as_mut_ptr();
    y += 2 as i32 * 16 as i32;
    s_graphicsoptions.driverinfo.generic.type_0 = 9 as i32;
    s_graphicsoptions.driverinfo.generic.flags = 0x8 as i32 as u32 | 0x100 as i32 as u32;
    s_graphicsoptions.driverinfo.generic.callback =
        Some(GraphicsOptions_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_graphicsoptions.driverinfo.generic.id = 105 as i32;
    s_graphicsoptions.driverinfo.generic.x = 320 as i32;
    s_graphicsoptions.driverinfo.generic.y = y;
    s_graphicsoptions.driverinfo.string =
        b"Driver Info\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_graphicsoptions.driverinfo.style = 0x1 as i32 | 0x10 as i32;
    s_graphicsoptions.driverinfo.color = crate::src::q3_ui::ui_qmenu::color_red.as_mut_ptr();
    s_graphicsoptions.back.generic.type_0 = 6 as i32;
    s_graphicsoptions.back.generic.name =
        b"menu/art/back_0\x00" as *const u8 as *const libc::c_char;
    s_graphicsoptions.back.generic.flags = 0x4 as i32 as u32 | 0x100 as i32 as u32;
    s_graphicsoptions.back.generic.callback =
        Some(GraphicsOptions_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_graphicsoptions.back.generic.id = 101 as i32;
    s_graphicsoptions.back.generic.x = 0 as i32;
    s_graphicsoptions.back.generic.y = 480 as i32 - 64 as i32;
    s_graphicsoptions.back.width = 128 as i32;
    s_graphicsoptions.back.height = 64 as i32;
    s_graphicsoptions.back.focuspic =
        b"menu/art/back_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_graphicsoptions.apply.generic.type_0 = 6 as i32;
    s_graphicsoptions.apply.generic.name =
        b"menu/art/accept_0\x00" as *const u8 as *const libc::c_char;
    s_graphicsoptions.apply.generic.flags =
        0x10 as i32 as u32 | 0x100 as i32 as u32 | 0x1000 as i32 as u32 | 0x4000 as i32 as u32;
    s_graphicsoptions.apply.generic.callback = Some(
        GraphicsOptions_ApplyChanges as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> (),
    );
    s_graphicsoptions.apply.generic.x = 640 as i32;
    s_graphicsoptions.apply.generic.y = 480 as i32 - 64 as i32;
    s_graphicsoptions.apply.width = 128 as i32;
    s_graphicsoptions.apply.height = 64 as i32;
    s_graphicsoptions.apply.focuspic =
        b"menu/art/accept_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_graphicsoptions.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut s_graphicsoptions.banner as *mut crate::ui_local_h::menutext_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_graphicsoptions.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut s_graphicsoptions.framel as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_graphicsoptions.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut s_graphicsoptions.framer as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_graphicsoptions.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut s_graphicsoptions.graphics as *mut crate::ui_local_h::menutext_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_graphicsoptions.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut s_graphicsoptions.display as *mut crate::ui_local_h::menutext_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_graphicsoptions.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut s_graphicsoptions.sound as *mut crate::ui_local_h::menutext_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_graphicsoptions.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut s_graphicsoptions.network as *mut crate::ui_local_h::menutext_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_graphicsoptions.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut s_graphicsoptions.list as *mut crate::ui_local_h::menulist_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_graphicsoptions.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut s_graphicsoptions.driver as *mut crate::ui_local_h::menulist_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_graphicsoptions.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut s_graphicsoptions.allow_extensions as *mut crate::ui_local_h::menulist_s
            as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_graphicsoptions.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut s_graphicsoptions.ratio as *mut crate::ui_local_h::menulist_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_graphicsoptions.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut s_graphicsoptions.mode as *mut crate::ui_local_h::menulist_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_graphicsoptions.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut s_graphicsoptions.colordepth as *mut crate::ui_local_h::menulist_s
            as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_graphicsoptions.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut s_graphicsoptions.fs as *mut crate::ui_local_h::menulist_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_graphicsoptions.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut s_graphicsoptions.lighting as *mut crate::ui_local_h::menulist_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_graphicsoptions.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut s_graphicsoptions.geometry as *mut crate::ui_local_h::menulist_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_graphicsoptions.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut s_graphicsoptions.tq as *mut crate::ui_local_h::menuslider_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_graphicsoptions.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut s_graphicsoptions.texturebits as *mut crate::ui_local_h::menulist_s
            as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_graphicsoptions.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut s_graphicsoptions.filter as *mut crate::ui_local_h::menulist_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_graphicsoptions.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut s_graphicsoptions.driverinfo as *mut crate::ui_local_h::menutext_s
            as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_graphicsoptions.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut s_graphicsoptions.back as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_graphicsoptions.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut s_graphicsoptions.apply as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    GraphicsOptions_SetMenuItems();
    GraphicsOptions_GetInitialVideo();
    if crate::src::q3_ui::ui_atoms::uis.glconfig.driverType as u32
        == crate::tr_types_h::GLDRV_ICD as i32 as u32
        && crate::src::q3_ui::ui_atoms::uis.glconfig.hardwareType as u32
            == crate::tr_types_h::GLHW_3DFX_2D3D as i32 as u32
    {
        s_graphicsoptions.driver.generic.flags |= 0x1000 as i32 as u32 | 0x4000 as i32 as u32
    };
}
/*
=================
GraphicsOptions_Cache
=================
*/
#[no_mangle]

pub unsafe extern "C" fn GraphicsOptions_Cache() {
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/frame2_l\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/frame1_r\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/back_0\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/back_1\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/accept_0\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/accept_1\x00" as *const u8 as *const libc::c_char,
    );
}
//
// ui_video.c
//
/*
=================
UI_GraphicsOptionsMenu
=================
*/
#[no_mangle]

pub unsafe extern "C" fn UI_GraphicsOptionsMenu() {
    GraphicsOptions_MenuInit();
    crate::src::q3_ui::ui_atoms::UI_PushMenu(
        &mut s_graphicsoptions.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
    );
    crate::src::q3_ui::ui_qmenu::Menu_SetCursorToItem(
        &mut s_graphicsoptions.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut s_graphicsoptions.graphics as *mut crate::ui_local_h::menutext_s as *mut libc::c_void,
    );
}

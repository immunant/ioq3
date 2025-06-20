use ::libc;

pub use crate::src::q3_ui::ui_atoms::UI_PopMenu;
pub use crate::src::q3_ui::ui_atoms::UI_PushMenu;
pub use crate::src::q3_ui::ui_display::UI_DisplayOptionsMenu;
pub use crate::src::q3_ui::ui_network::UI_NetworkOptionsMenu;
pub use crate::src::q3_ui::ui_qmenu::color_red;
pub use crate::src::q3_ui::ui_qmenu::color_white;
pub use crate::src::q3_ui::ui_qmenu::Menu_AddItem;
pub use crate::src::q3_ui::ui_sound::UI_SoundOptionsMenu;
pub use crate::src::q3_ui::ui_video::UI_GraphicsOptionsMenu;
pub use crate::src::qcommon::q_shared::connstate_t;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qhandle_t;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::sfxHandle_t;
pub use crate::src::qcommon::q_shared::vec4_t;
pub use crate::src::qcommon::q_shared::vec_t;
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
pub use crate::src::ui::ui_syscalls::trap_GetClientState;
pub use crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip;

pub use crate::ui_local_h::_tag_menuframework;
pub use crate::ui_local_h::menubitmap_s;
pub use crate::ui_local_h::menucommon_s;
pub use crate::ui_local_h::menuframework_s;
pub use crate::ui_local_h::menutext_s;
pub use crate::ui_public_h::uiClientState_t;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct optionsmenu_t {
    pub menu: menuframework_s,
    pub banner: menutext_s,
    pub framel: menubitmap_s,
    pub framer: menubitmap_s,
    pub graphics: menutext_s,
    pub display: menutext_s,
    pub sound: menutext_s,
    pub network: menutext_s,
    pub back: menubitmap_s,
}

static mut s_options: optionsmenu_t = optionsmenu_t {
    menu: menuframework_s {
        cursor: 0,
        cursor_prev: 0,
        nitems: 0,
        items: [std::ptr::null_mut(); 64],
        draw: None,
        key: None,
        wrapAround: qfalse,
        fullscreen: qfalse,
        showlogo: qfalse,
    },
    banner: menutext_s {
        generic: menucommon_s {
            type_0: 0,
            name: 0 as *const libc::c_char,
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: std::ptr::null_mut(),
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
        string: std::ptr::null_mut(),
        style: 0,
        color: std::ptr::null_mut(),
    },
    framel: menubitmap_s {
        generic: menucommon_s {
            type_0: 0,
            name: 0 as *const libc::c_char,
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: std::ptr::null_mut(),
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
        focuspic: std::ptr::null_mut(),
        errorpic: std::ptr::null_mut(),
        shader: 0,
        focusshader: 0,
        width: 0,
        height: 0,
        focuscolor: std::ptr::null_mut(),
    },
    framer: menubitmap_s {
        generic: menucommon_s {
            type_0: 0,
            name: 0 as *const libc::c_char,
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: std::ptr::null_mut(),
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
        focuspic: std::ptr::null_mut(),
        errorpic: std::ptr::null_mut(),
        shader: 0,
        focusshader: 0,
        width: 0,
        height: 0,
        focuscolor: std::ptr::null_mut(),
    },
    graphics: menutext_s {
        generic: menucommon_s {
            type_0: 0,
            name: 0 as *const libc::c_char,
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: std::ptr::null_mut(),
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
        string: std::ptr::null_mut(),
        style: 0,
        color: std::ptr::null_mut(),
    },
    display: menutext_s {
        generic: menucommon_s {
            type_0: 0,
            name: 0 as *const libc::c_char,
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: std::ptr::null_mut(),
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
        string: std::ptr::null_mut(),
        style: 0,
        color: std::ptr::null_mut(),
    },
    sound: menutext_s {
        generic: menucommon_s {
            type_0: 0,
            name: 0 as *const libc::c_char,
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: std::ptr::null_mut(),
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
        string: std::ptr::null_mut(),
        style: 0,
        color: std::ptr::null_mut(),
    },
    network: menutext_s {
        generic: menucommon_s {
            type_0: 0,
            name: 0 as *const libc::c_char,
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: std::ptr::null_mut(),
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
        string: std::ptr::null_mut(),
        style: 0,
        color: std::ptr::null_mut(),
    },
    back: menubitmap_s {
        generic: menucommon_s {
            type_0: 0,
            name: 0 as *const libc::c_char,
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: std::ptr::null_mut(),
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
        focuspic: std::ptr::null_mut(),
        errorpic: std::ptr::null_mut(),
        shader: 0,
        focusshader: 0,
        width: 0,
        height: 0,
        focuscolor: std::ptr::null_mut(),
    },
};
/*
=================
Options_Event
=================
*/

unsafe extern "C" fn Options_Event(mut ptr: *mut libc::c_void, mut event: i32) {
    if event != 3 as i32 {
        return;
    }
    match (*(ptr as *mut menucommon_s)).id {
        10 => {
            UI_GraphicsOptionsMenu();
        }
        11 => {
            UI_DisplayOptionsMenu();
        }
        12 => {
            UI_SoundOptionsMenu();
        }
        13 => {
            UI_NetworkOptionsMenu();
        }
        14 => {
            UI_PopMenu();
        }
        _ => {}
    };
}
/*
===============
SystemConfig_Cache
===============
*/
#[no_mangle]

pub unsafe extern "C" fn SystemConfig_Cache() {
    trap_R_RegisterShaderNoMip(b"menu/art/frame2_l\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/frame1_r\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/back_0\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/back_1\x00" as *const u8 as *const libc::c_char);
}
/*
===============
Options_MenuInit
===============
*/
#[no_mangle]

pub unsafe extern "C" fn Options_MenuInit() {
    let mut y: i32 = 0;
    let mut cstate: uiClientState_t = uiClientState_t {
        connState: CA_UNINITIALIZED,
        connectPacketCount: 0,
        clientNum: 0,
        servername: [0; 1024],
        updateInfoString: [0; 1024],
        messageString: [0; 1024],
    };
    crate::stdlib::memset(
        &mut s_options as *mut optionsmenu_t as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<optionsmenu_t>() as usize,
    );
    SystemConfig_Cache();
    s_options.menu.wrapAround = qtrue;
    trap_GetClientState(&mut cstate as *mut _ as *mut uiClientState_t);
    if cstate.connState as u32 >= CA_CONNECTED as i32 as u32 {
        s_options.menu.fullscreen = qfalse
    } else {
        s_options.menu.fullscreen = qtrue
    }
    s_options.banner.generic.type_0 = 10 as i32;
    s_options.banner.generic.flags = 0x8 as i32 as u32;
    s_options.banner.generic.x = 320 as i32;
    s_options.banner.generic.y = 16 as i32;
    s_options.banner.string =
        b"SYSTEM SETUP\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_options.banner.color = color_white.as_mut_ptr();
    s_options.banner.style = 0x1 as i32;
    s_options.framel.generic.type_0 = 6 as i32;
    s_options.framel.generic.name = b"menu/art/frame2_l\x00" as *const u8 as *const libc::c_char;
    s_options.framel.generic.flags = 0x4000 as i32 as u32;
    s_options.framel.generic.x = 8 as i32;
    s_options.framel.generic.y = 76 as i32;
    s_options.framel.width = 256 as i32;
    s_options.framel.height = 334 as i32;
    s_options.framer.generic.type_0 = 6 as i32;
    s_options.framer.generic.name = b"menu/art/frame1_r\x00" as *const u8 as *const libc::c_char;
    s_options.framer.generic.flags = 0x4000 as i32 as u32;
    s_options.framer.generic.x = 376 as i32;
    s_options.framer.generic.y = 76 as i32;
    s_options.framer.width = 256 as i32;
    s_options.framer.height = 334 as i32;
    y = 168 as i32;
    s_options.graphics.generic.type_0 = 9 as i32;
    s_options.graphics.generic.flags = 0x8 as i32 as u32 | 0x100 as i32 as u32;
    s_options.graphics.generic.callback =
        Some(Options_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_options.graphics.generic.id = 10 as i32;
    s_options.graphics.generic.x = 320 as i32;
    s_options.graphics.generic.y = y;
    s_options.graphics.string =
        b"GRAPHICS\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_options.graphics.color = color_red.as_mut_ptr();
    s_options.graphics.style = 0x1 as i32;
    y += 34 as i32;
    s_options.display.generic.type_0 = 9 as i32;
    s_options.display.generic.flags = 0x8 as i32 as u32 | 0x100 as i32 as u32;
    s_options.display.generic.callback =
        Some(Options_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_options.display.generic.id = 11 as i32;
    s_options.display.generic.x = 320 as i32;
    s_options.display.generic.y = y;
    s_options.display.string =
        b"DISPLAY\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_options.display.color = color_red.as_mut_ptr();
    s_options.display.style = 0x1 as i32;
    y += 34 as i32;
    s_options.sound.generic.type_0 = 9 as i32;
    s_options.sound.generic.flags = 0x8 as i32 as u32 | 0x100 as i32 as u32;
    s_options.sound.generic.callback =
        Some(Options_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_options.sound.generic.id = 12 as i32;
    s_options.sound.generic.x = 320 as i32;
    s_options.sound.generic.y = y;
    s_options.sound.string = b"SOUND\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_options.sound.color = color_red.as_mut_ptr();
    s_options.sound.style = 0x1 as i32;
    y += 34 as i32;
    s_options.network.generic.type_0 = 9 as i32;
    s_options.network.generic.flags = 0x8 as i32 as u32 | 0x100 as i32 as u32;
    s_options.network.generic.callback =
        Some(Options_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_options.network.generic.id = 13 as i32;
    s_options.network.generic.x = 320 as i32;
    s_options.network.generic.y = y;
    s_options.network.string =
        b"NETWORK\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_options.network.color = color_red.as_mut_ptr();
    s_options.network.style = 0x1 as i32;
    s_options.back.generic.type_0 = 6 as i32;
    s_options.back.generic.name = b"menu/art/back_0\x00" as *const u8 as *const libc::c_char;
    s_options.back.generic.flags = 0x4 as i32 as u32 | 0x100 as i32 as u32;
    s_options.back.generic.callback =
        Some(Options_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_options.back.generic.id = 14 as i32;
    s_options.back.generic.x = 0 as i32;
    s_options.back.generic.y = 480 as i32 - 64 as i32;
    s_options.back.width = 128 as i32;
    s_options.back.height = 64 as i32;
    s_options.back.focuspic =
        b"menu/art/back_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    Menu_AddItem(
        &mut s_options.menu as *mut _ as *mut _tag_menuframework,
        &mut s_options.banner as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_options.menu as *mut _ as *mut _tag_menuframework,
        &mut s_options.framel as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_options.menu as *mut _ as *mut _tag_menuframework,
        &mut s_options.framer as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_options.menu as *mut _ as *mut _tag_menuframework,
        &mut s_options.graphics as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_options.menu as *mut _ as *mut _tag_menuframework,
        &mut s_options.display as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_options.menu as *mut _ as *mut _tag_menuframework,
        &mut s_options.sound as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_options.menu as *mut _ as *mut _tag_menuframework,
        &mut s_options.network as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_options.menu as *mut _ as *mut _tag_menuframework,
        &mut s_options.back as *mut menubitmap_s as *mut libc::c_void,
    );
}
/*
===============
UI_SystemConfigMenu
===============
*/
#[no_mangle]

pub unsafe extern "C" fn UI_SystemConfigMenu() {
    Options_MenuInit();
    UI_PushMenu(&mut s_options.menu as *mut _ as *mut _tag_menuframework);
}

use ::libc;

pub use crate::src::q3_ui::ui_atoms::UI_PopMenu;
pub use crate::src::q3_ui::ui_atoms::UI_PushMenu;
pub use crate::src::q3_ui::ui_qmenu::color_white;
pub use crate::src::q3_ui::ui_qmenu::Menu_AddItem;
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
pub use crate::src::qcommon::q_shared::EXEC_APPEND;
pub use crate::src::qcommon::q_shared::EXEC_INSERT;
pub use crate::src::qcommon::q_shared::EXEC_NOW;
pub use crate::src::ui::ui_syscalls::trap_Cmd_ExecuteText;
pub use crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip;

pub use crate::ui_local_h::_tag_menuframework;
pub use crate::ui_local_h::menubitmap_s;
pub use crate::ui_local_h::menucommon_s;
pub use crate::ui_local_h::menufield_s;
pub use crate::ui_local_h::menuframework_s;
pub use crate::ui_local_h::menutext_s;
pub use crate::ui_local_h::mfield_t;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct specifyserver_t {
    pub menu: menuframework_s,
    pub banner: menutext_s,
    pub framel: menubitmap_s,
    pub framer: menubitmap_s,
    pub domain: menufield_s,
    pub port: menufield_s,
    pub go: menubitmap_s,
    pub back: menubitmap_s,
}

static mut specifyserver_artlist: [*mut libc::c_char; 7] = [
    b"menu/art/frame2_l\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"menu/art/frame1_r\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"menu/art/back_0\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"menu/art/back_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"menu/art/fight_0\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"menu/art/fight_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
];

static mut s_specifyserver: specifyserver_t = specifyserver_t {
    menu: menuframework_s {
        cursor: 0,
        cursor_prev: 0,
        nitems: 0,
        items: [0 as *const libc::c_void as *mut libc::c_void; 64],
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
            parent: 0 as *const menuframework_s
                as *mut menuframework_s,
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
            parent: 0 as *const menuframework_s
                as *mut menuframework_s,
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
            parent: 0 as *const menuframework_s
                as *mut menuframework_s,
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
    domain: menufield_s {
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
            parent: 0 as *const menuframework_s
                as *mut menuframework_s,
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
        field: mfield_t {
            cursor: 0,
            scroll: 0,
            widthInChars: 0,
            buffer: [0; 256],
            maxchars: 0,
        },
    },
    port: menufield_s {
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
            parent: 0 as *const menuframework_s
                as *mut menuframework_s,
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
        field: mfield_t {
            cursor: 0,
            scroll: 0,
            widthInChars: 0,
            buffer: [0; 256],
            maxchars: 0,
        },
    },
    go: menubitmap_s {
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
            parent: 0 as *const menuframework_s
                as *mut menuframework_s,
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
            parent: 0 as *const menuframework_s
                as *mut menuframework_s,
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
/*
=================
SpecifyServer_Event
=================
*/

unsafe extern "C" fn SpecifyServer_Event(mut ptr: *mut libc::c_void, mut event: i32) {
    let mut buff: [libc::c_char; 256] = [0; 256];
    match (*(ptr as *mut menucommon_s)).id {
        103 => {
            if !(event != 3 as i32) {
                if s_specifyserver.domain.field.buffer[0 as i32 as usize] != 0 {
                    libc::strcpy(
                        buff.as_mut_ptr(),
                        s_specifyserver.domain.field.buffer.as_mut_ptr(),
                    );
                    if s_specifyserver.port.field.buffer[0 as i32 as usize] != 0 {
                        Com_sprintf(
                            buff.as_mut_ptr()
                                .offset(crate::stdlib::strlen(buff.as_mut_ptr()) as isize),
                            128 as i32,
                            b":%s\x00" as *const u8 as *const libc::c_char,
                            s_specifyserver.port.field.buffer.as_mut_ptr(),
                        );
                    }
                    trap_Cmd_ExecuteText(
                        EXEC_APPEND as i32,
                        va(
                            b"connect %s\n\x00" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                            buff.as_mut_ptr(),
                        ),
                    );
                }
            }
        }
        102 => {
            if !(event != 3 as i32) {
                UI_PopMenu();
            }
        }
        _ => {}
    };
}
/*
=================
SpecifyServer_MenuInit
=================
*/
#[no_mangle]

pub unsafe extern "C" fn SpecifyServer_MenuInit() {
    // zero set all our globals
    crate::stdlib::memset(
        &mut s_specifyserver as *mut specifyserver_t as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<specifyserver_t>() as libc::c_ulong,
    );
    SpecifyServer_Cache();
    s_specifyserver.menu.wrapAround = qtrue;
    s_specifyserver.menu.fullscreen = qtrue;
    s_specifyserver.banner.generic.type_0 = 10 as i32;
    s_specifyserver.banner.generic.x = 320 as i32;
    s_specifyserver.banner.generic.y = 16 as i32;
    s_specifyserver.banner.string =
        b"SPECIFY SERVER\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_specifyserver.banner.color = color_white.as_mut_ptr();
    s_specifyserver.banner.style = 0x1 as i32;
    s_specifyserver.framel.generic.type_0 = 6 as i32;
    s_specifyserver.framel.generic.name =
        b"menu/art/frame2_l\x00" as *const u8 as *const libc::c_char;
    s_specifyserver.framel.generic.flags = 0x4000 as i32 as u32;
    s_specifyserver.framel.generic.x = 0 as i32;
    s_specifyserver.framel.generic.y = 78 as i32;
    s_specifyserver.framel.width = 256 as i32;
    s_specifyserver.framel.height = 329 as i32;
    s_specifyserver.framer.generic.type_0 = 6 as i32;
    s_specifyserver.framer.generic.name =
        b"menu/art/frame1_r\x00" as *const u8 as *const libc::c_char;
    s_specifyserver.framer.generic.flags = 0x4000 as i32 as u32;
    s_specifyserver.framer.generic.x = 376 as i32;
    s_specifyserver.framer.generic.y = 76 as i32;
    s_specifyserver.framer.width = 256 as i32;
    s_specifyserver.framer.height = 334 as i32;
    s_specifyserver.domain.generic.type_0 = 4 as i32;
    s_specifyserver.domain.generic.name = b"Address:\x00" as *const u8 as *const libc::c_char;
    s_specifyserver.domain.generic.flags = 0x100 as i32 as u32 | 0x2 as i32 as u32;
    s_specifyserver.domain.generic.x = 206 as i32;
    s_specifyserver.domain.generic.y = 220 as i32;
    s_specifyserver.domain.field.widthInChars = 38 as i32;
    s_specifyserver.domain.field.maxchars = 80 as i32;
    s_specifyserver.port.generic.type_0 = 4 as i32;
    s_specifyserver.port.generic.name = b"Port:\x00" as *const u8 as *const libc::c_char;
    s_specifyserver.port.generic.flags =
        0x100 as i32 as u32 | 0x2 as i32 as u32 | 0x20 as i32 as u32;
    s_specifyserver.port.generic.x = 206 as i32;
    s_specifyserver.port.generic.y = 250 as i32;
    s_specifyserver.port.field.widthInChars = 6 as i32;
    s_specifyserver.port.field.maxchars = 5 as i32;
    s_specifyserver.go.generic.type_0 = 6 as i32;
    s_specifyserver.go.generic.name = b"menu/art/fight_0\x00" as *const u8 as *const libc::c_char;
    s_specifyserver.go.generic.flags = 0x10 as i32 as u32 | 0x100 as i32 as u32;
    s_specifyserver.go.generic.callback =
        Some(SpecifyServer_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_specifyserver.go.generic.id = 103 as i32;
    s_specifyserver.go.generic.x = 640 as i32;
    s_specifyserver.go.generic.y = 480 as i32 - 64 as i32;
    s_specifyserver.go.width = 128 as i32;
    s_specifyserver.go.height = 64 as i32;
    s_specifyserver.go.focuspic =
        b"menu/art/fight_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_specifyserver.back.generic.type_0 = 6 as i32;
    s_specifyserver.back.generic.name = b"menu/art/back_0\x00" as *const u8 as *const libc::c_char;
    s_specifyserver.back.generic.flags = 0x4 as i32 as u32 | 0x100 as i32 as u32;
    s_specifyserver.back.generic.callback =
        Some(SpecifyServer_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_specifyserver.back.generic.id = 102 as i32;
    s_specifyserver.back.generic.x = 0 as i32;
    s_specifyserver.back.generic.y = 480 as i32 - 64 as i32;
    s_specifyserver.back.width = 128 as i32;
    s_specifyserver.back.height = 64 as i32;
    s_specifyserver.back.focuspic =
        b"menu/art/back_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    Menu_AddItem(
        &mut s_specifyserver.menu as *mut _ as *mut _tag_menuframework,
        &mut s_specifyserver.banner as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_specifyserver.menu as *mut _ as *mut _tag_menuframework,
        &mut s_specifyserver.framel as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_specifyserver.menu as *mut _ as *mut _tag_menuframework,
        &mut s_specifyserver.framer as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_specifyserver.menu as *mut _ as *mut _tag_menuframework,
        &mut s_specifyserver.domain as *mut menufield_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_specifyserver.menu as *mut _ as *mut _tag_menuframework,
        &mut s_specifyserver.port as *mut menufield_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_specifyserver.menu as *mut _ as *mut _tag_menuframework,
        &mut s_specifyserver.go as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_specifyserver.menu as *mut _ as *mut _tag_menuframework,
        &mut s_specifyserver.back as *mut menubitmap_s as *mut libc::c_void,
    );
    Com_sprintf(
        s_specifyserver.port.field.buffer.as_mut_ptr(),
        6 as i32,
        b"%i\x00" as *const u8 as *const libc::c_char,
        27960 as i32,
    );
}
/*
=================
SpecifyServer_Cache
=================
*/
#[no_mangle]

pub unsafe extern "C" fn SpecifyServer_Cache() {
    let mut i: i32 = 0;
    // touch all our pics
    i = 0 as i32;
    while !specifyserver_artlist[i as usize].is_null() {
        trap_R_RegisterShaderNoMip(specifyserver_artlist[i as usize]);
        i += 1
    }
}
//
// ui_specifyserver.c
//
/*
=================
UI_SpecifyServerMenu
=================
*/
#[no_mangle]

pub unsafe extern "C" fn UI_SpecifyServerMenu() {
    SpecifyServer_MenuInit();
    UI_PushMenu(
        &mut s_specifyserver.menu as *mut _ as *mut _tag_menuframework,
    );
}

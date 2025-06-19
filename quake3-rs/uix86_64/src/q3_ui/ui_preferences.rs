use ::libc;

pub use crate::src::q3_ui::ui_atoms::UI_DrawChar;
pub use crate::src::q3_ui::ui_atoms::UI_DrawHandlePic;
pub use crate::src::q3_ui::ui_atoms::UI_DrawString;
pub use crate::src::q3_ui::ui_atoms::UI_FillRect;
pub use crate::src::q3_ui::ui_atoms::UI_PopMenu;
pub use crate::src::q3_ui::ui_atoms::UI_PushMenu;
pub use crate::src::q3_ui::ui_qmenu::color_white;
pub use crate::src::q3_ui::ui_qmenu::listbar_color;
pub use crate::src::q3_ui::ui_qmenu::text_color_disabled;
pub use crate::src::q3_ui::ui_qmenu::text_color_highlight;
pub use crate::src::q3_ui::ui_qmenu::text_color_normal;
pub use crate::src::q3_ui::ui_qmenu::Menu_AddItem;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qhandle_t;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::sfxHandle_t;
pub use crate::src::qcommon::q_shared::va;
pub use crate::src::qcommon::q_shared::vec4_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::qcommon::q_shared::Com_Clamp;
pub use crate::src::ui::ui_syscalls::trap_Cvar_Reset;
pub use crate::src::ui::ui_syscalls::trap_Cvar_SetValue;
pub use crate::src::ui::ui_syscalls::trap_Cvar_VariableValue;
pub use crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip;

pub use crate::ui_local_h::_tag_menuframework;
pub use crate::ui_local_h::menubitmap_s;
pub use crate::ui_local_h::menucommon_s;
pub use crate::ui_local_h::menuframework_s;
pub use crate::ui_local_h::menulist_s;
pub use crate::ui_local_h::menuradiobutton_s;
pub use crate::ui_local_h::menutext_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct preferences_t {
    pub menu: crate::ui_local_h::menuframework_s,
    pub banner: crate::ui_local_h::menutext_s,
    pub framel: crate::ui_local_h::menubitmap_s,
    pub framer: crate::ui_local_h::menubitmap_s,
    pub crosshair: crate::ui_local_h::menulist_s,
    pub simpleitems: crate::ui_local_h::menuradiobutton_s,
    pub brass: crate::ui_local_h::menuradiobutton_s,
    pub wallmarks: crate::ui_local_h::menuradiobutton_s,
    pub dynamiclights: crate::ui_local_h::menuradiobutton_s,
    pub identifytarget: crate::ui_local_h::menuradiobutton_s,
    pub highqualitysky: crate::ui_local_h::menuradiobutton_s,
    pub synceveryframe: crate::ui_local_h::menuradiobutton_s,
    pub forcemodel: crate::ui_local_h::menuradiobutton_s,
    pub drawteamoverlay: crate::ui_local_h::menulist_s,
    pub allowdownload: crate::ui_local_h::menuradiobutton_s,
    pub back: crate::ui_local_h::menubitmap_s,
    pub crosshairShader: [crate::src::qcommon::q_shared::qhandle_t; 10],
}

static mut s_preferences: preferences_t = preferences_t {
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
    crosshair: crate::ui_local_h::menulist_s {
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
    simpleitems: crate::ui_local_h::menuradiobutton_s {
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
        curvalue: 0,
    },
    brass: crate::ui_local_h::menuradiobutton_s {
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
        curvalue: 0,
    },
    wallmarks: crate::ui_local_h::menuradiobutton_s {
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
        curvalue: 0,
    },
    dynamiclights: crate::ui_local_h::menuradiobutton_s {
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
        curvalue: 0,
    },
    identifytarget: crate::ui_local_h::menuradiobutton_s {
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
        curvalue: 0,
    },
    highqualitysky: crate::ui_local_h::menuradiobutton_s {
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
        curvalue: 0,
    },
    synceveryframe: crate::ui_local_h::menuradiobutton_s {
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
        curvalue: 0,
    },
    forcemodel: crate::ui_local_h::menuradiobutton_s {
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
        curvalue: 0,
    },
    drawteamoverlay: crate::ui_local_h::menulist_s {
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
    allowdownload: crate::ui_local_h::menuradiobutton_s {
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
        curvalue: 0,
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
    crosshairShader: [0; 10],
};

static mut teamoverlay_names: [*const libc::c_char; 5] = [
    b"off\x00" as *const u8 as *const libc::c_char,
    b"upper right\x00" as *const u8 as *const libc::c_char,
    b"lower right\x00" as *const u8 as *const libc::c_char,
    b"lower left\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];

unsafe extern "C" fn Preferences_SetMenuItems() {
    s_preferences.crosshair.curvalue = crate::src::ui::ui_syscalls::trap_Cvar_VariableValue(
        b"cg_drawCrosshair\x00" as *const u8 as *const libc::c_char,
    ) as i32
        % 10 as i32;
    s_preferences.simpleitems.curvalue = (crate::src::ui::ui_syscalls::trap_Cvar_VariableValue(
        b"cg_simpleItems\x00" as *const u8 as *const libc::c_char,
    ) != 0 as i32 as f32) as i32;
    s_preferences.brass.curvalue = (crate::src::ui::ui_syscalls::trap_Cvar_VariableValue(
        b"cg_brassTime\x00" as *const u8 as *const libc::c_char,
    ) != 0 as i32 as f32) as i32;
    s_preferences.wallmarks.curvalue = (crate::src::ui::ui_syscalls::trap_Cvar_VariableValue(
        b"cg_marks\x00" as *const u8 as *const libc::c_char,
    ) != 0 as i32 as f32) as i32;
    s_preferences.identifytarget.curvalue = (crate::src::ui::ui_syscalls::trap_Cvar_VariableValue(
        b"cg_drawCrosshairNames\x00" as *const u8 as *const libc::c_char,
    ) != 0 as i32 as f32) as i32;
    s_preferences.dynamiclights.curvalue = (crate::src::ui::ui_syscalls::trap_Cvar_VariableValue(
        b"r_dynamiclight\x00" as *const u8 as *const libc::c_char,
    ) != 0 as i32 as f32) as i32;
    s_preferences.highqualitysky.curvalue = (crate::src::ui::ui_syscalls::trap_Cvar_VariableValue(
        b"r_fastsky\x00" as *const u8 as *const libc::c_char,
    ) == 0 as i32 as f32) as i32;
    s_preferences.synceveryframe.curvalue = (crate::src::ui::ui_syscalls::trap_Cvar_VariableValue(
        b"r_finish\x00" as *const u8 as *const libc::c_char,
    ) != 0 as i32 as f32) as i32;
    s_preferences.forcemodel.curvalue = (crate::src::ui::ui_syscalls::trap_Cvar_VariableValue(
        b"cg_forcemodel\x00" as *const u8 as *const libc::c_char,
    ) != 0 as i32 as f32) as i32;
    s_preferences.drawteamoverlay.curvalue = crate::src::qcommon::q_shared::Com_Clamp(
        0 as i32 as f32,
        3 as i32 as f32,
        crate::src::ui::ui_syscalls::trap_Cvar_VariableValue(
            b"cg_drawTeamOverlay\x00" as *const u8 as *const libc::c_char,
        ),
    ) as i32;
    s_preferences.allowdownload.curvalue = (crate::src::ui::ui_syscalls::trap_Cvar_VariableValue(
        b"cl_allowDownload\x00" as *const u8 as *const libc::c_char,
    ) != 0 as i32 as f32) as i32;
}

unsafe extern "C" fn Preferences_Event(mut ptr: *mut libc::c_void, mut notification: i32) {
    if notification != 3 as i32 {
        return;
    }
    match (*(ptr as *mut crate::ui_local_h::menucommon_s)).id {
        127 => {
            crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
                b"cg_drawCrosshair\x00" as *const u8 as *const libc::c_char,
                s_preferences.crosshair.curvalue as f32,
            );
        }
        128 => {
            crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
                b"cg_simpleItems\x00" as *const u8 as *const libc::c_char,
                s_preferences.simpleitems.curvalue as f32,
            );
        }
        129 => {
            crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
                b"r_fastsky\x00" as *const u8 as *const libc::c_char,
                (s_preferences.highqualitysky.curvalue == 0) as i32 as f32,
            );
        }
        130 => {
            if s_preferences.brass.curvalue != 0 {
                crate::src::ui::ui_syscalls::trap_Cvar_Reset(
                    b"cg_brassTime\x00" as *const u8 as *const libc::c_char,
                );
            } else {
                crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
                    b"cg_brassTime\x00" as *const u8 as *const libc::c_char,
                    0 as i32 as f32,
                );
            }
        }
        131 => {
            crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
                b"cg_marks\x00" as *const u8 as *const libc::c_char,
                s_preferences.wallmarks.curvalue as f32,
            );
        }
        132 => {
            crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
                b"r_dynamiclight\x00" as *const u8 as *const libc::c_char,
                s_preferences.dynamiclights.curvalue as f32,
            );
        }
        133 => {
            crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
                b"cg_drawCrosshairNames\x00" as *const u8 as *const libc::c_char,
                s_preferences.identifytarget.curvalue as f32,
            );
        }
        134 => {
            crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
                b"r_finish\x00" as *const u8 as *const libc::c_char,
                s_preferences.synceveryframe.curvalue as f32,
            );
        }
        135 => {
            crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
                b"cg_forcemodel\x00" as *const u8 as *const libc::c_char,
                s_preferences.forcemodel.curvalue as f32,
            );
        }
        136 => {
            crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
                b"cg_drawTeamOverlay\x00" as *const u8 as *const libc::c_char,
                s_preferences.drawteamoverlay.curvalue as f32,
            );
        }
        137 => {
            crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
                b"cl_allowDownload\x00" as *const u8 as *const libc::c_char,
                s_preferences.allowdownload.curvalue as f32,
            );
            crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
                b"sv_allowDownload\x00" as *const u8 as *const libc::c_char,
                s_preferences.allowdownload.curvalue as f32,
            );
        }
        138 => {
            crate::src::q3_ui::ui_atoms::UI_PopMenu();
        }
        _ => {}
    };
}
/*
=================
Crosshair_Draw
=================
*/

unsafe extern "C" fn Crosshair_Draw(mut self_0: *mut libc::c_void) {
    let mut s: *mut crate::ui_local_h::menulist_s = 0 as *mut crate::ui_local_h::menulist_s;
    let mut color: *mut f32 = 0 as *mut f32;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut style: i32 = 0;
    let mut focus: crate::src::qcommon::q_shared::qboolean = crate::src::qcommon::q_shared::qfalse;
    s = self_0 as *mut crate::ui_local_h::menulist_s;
    x = (*s).generic.x;
    y = (*s).generic.y;
    style = 0x10 as i32;
    focus = ((*(*s).generic.parent).cursor == (*s).generic.menuPosition) as i32
        as crate::src::qcommon::q_shared::qboolean;
    if (*s).generic.flags & 0x2000 as i32 as u32 != 0 {
        color = crate::src::q3_ui::ui_qmenu::text_color_disabled.as_mut_ptr()
    } else if focus as u64 != 0 {
        color = crate::src::q3_ui::ui_qmenu::text_color_highlight.as_mut_ptr();
        style |= 0x4000 as i32
    } else if (*s).generic.flags & 0x1 as i32 as u32 != 0 {
        color = crate::src::q3_ui::ui_qmenu::text_color_highlight.as_mut_ptr();
        style |= 0x1000 as i32
    } else {
        color = crate::src::q3_ui::ui_qmenu::text_color_normal.as_mut_ptr()
    }
    if focus as u64 != 0 {
        // draw cursor
        crate::src::q3_ui::ui_atoms::UI_FillRect(
            (*s).generic.left as f32,
            (*s).generic.top as f32,
            ((*s).generic.right - (*s).generic.left + 1 as i32) as f32,
            ((*s).generic.bottom - (*s).generic.top + 1 as i32) as f32,
            crate::src::q3_ui::ui_qmenu::listbar_color.as_mut_ptr(),
        );
        crate::src::q3_ui::ui_atoms::UI_DrawChar(
            x,
            y,
            13 as i32,
            0x1 as i32 | 0x1000 as i32 | 0x10 as i32,
            color,
        );
    }
    crate::src::q3_ui::ui_atoms::UI_DrawString(
        x - 8 as i32,
        y,
        (*s).generic.name,
        style | 0x2 as i32,
        color,
    );
    if (*s).curvalue == 0 {
        return;
    }
    crate::src::q3_ui::ui_atoms::UI_DrawHandlePic(
        (x + 8 as i32) as f32,
        (y - 4 as i32) as f32,
        24 as i32 as f32,
        24 as i32 as f32,
        s_preferences.crosshairShader[(*s).curvalue as usize],
    );
}

unsafe extern "C" fn Preferences_MenuInit() {
    let mut y: i32 = 0;
    crate::stdlib::memset(
        &mut s_preferences as *mut preferences_t as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<preferences_t>() as libc::c_ulong,
    );
    Preferences_Cache();
    s_preferences.menu.wrapAround = crate::src::qcommon::q_shared::qtrue;
    s_preferences.menu.fullscreen = crate::src::qcommon::q_shared::qtrue;
    s_preferences.banner.generic.type_0 = 10 as i32;
    s_preferences.banner.generic.x = 320 as i32;
    s_preferences.banner.generic.y = 16 as i32;
    s_preferences.banner.string =
        b"GAME OPTIONS\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_preferences.banner.color = crate::src::q3_ui::ui_qmenu::color_white.as_mut_ptr();
    s_preferences.banner.style = 0x1 as i32;
    s_preferences.framel.generic.type_0 = 6 as i32;
    s_preferences.framel.generic.name =
        b"menu/art/frame2_l\x00" as *const u8 as *const libc::c_char;
    s_preferences.framel.generic.flags = 0x4000 as i32 as u32;
    s_preferences.framel.generic.x = 0 as i32;
    s_preferences.framel.generic.y = 78 as i32;
    s_preferences.framel.width = 256 as i32;
    s_preferences.framel.height = 329 as i32;
    s_preferences.framer.generic.type_0 = 6 as i32;
    s_preferences.framer.generic.name =
        b"menu/art/frame1_r\x00" as *const u8 as *const libc::c_char;
    s_preferences.framer.generic.flags = 0x4000 as i32 as u32;
    s_preferences.framer.generic.x = 376 as i32;
    s_preferences.framer.generic.y = 76 as i32;
    s_preferences.framer.width = 256 as i32;
    s_preferences.framer.height = 334 as i32;
    y = 144 as i32;
    s_preferences.crosshair.generic.type_0 = 3 as i32;
    s_preferences.crosshair.generic.flags =
        0x100 as i32 as u32 | 0x2 as i32 as u32 | 0x8000 as i32 as u32 | 0x10000 as i32 as u32;
    s_preferences.crosshair.generic.x = 360 as i32;
    s_preferences.crosshair.generic.y = y;
    s_preferences.crosshair.generic.name = b"Crosshair:\x00" as *const u8 as *const libc::c_char;
    s_preferences.crosshair.generic.callback =
        Some(Preferences_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_preferences.crosshair.generic.ownerdraw =
        Some(Crosshair_Draw as unsafe extern "C" fn(_: *mut libc::c_void) -> ());
    s_preferences.crosshair.generic.id = 127 as i32;
    s_preferences.crosshair.generic.top = y - 4 as i32;
    s_preferences.crosshair.generic.bottom = y + 20 as i32;
    s_preferences.crosshair.generic.left = (360 as i32 as libc::c_ulong).wrapping_sub(
        crate::stdlib::strlen(s_preferences.crosshair.generic.name)
            .wrapping_add(1 as i32 as libc::c_ulong)
            .wrapping_mul(8 as i32 as libc::c_ulong),
    ) as i32;
    s_preferences.crosshair.generic.right = 360 as i32 + 48 as i32;
    s_preferences.crosshair.numitems = 10 as i32;
    y += 16 as i32 + 2 as i32 + 4 as i32;
    s_preferences.simpleitems.generic.type_0 = 5 as i32;
    s_preferences.simpleitems.generic.name =
        b"Simple Items:\x00" as *const u8 as *const libc::c_char;
    s_preferences.simpleitems.generic.flags = 0x100 as i32 as u32 | 0x2 as i32 as u32;
    s_preferences.simpleitems.generic.callback =
        Some(Preferences_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_preferences.simpleitems.generic.id = 128 as i32;
    s_preferences.simpleitems.generic.x = 360 as i32;
    s_preferences.simpleitems.generic.y = y;
    y += 16 as i32;
    s_preferences.wallmarks.generic.type_0 = 5 as i32;
    s_preferences.wallmarks.generic.name =
        b"Marks on Walls:\x00" as *const u8 as *const libc::c_char;
    s_preferences.wallmarks.generic.flags = 0x100 as i32 as u32 | 0x2 as i32 as u32;
    s_preferences.wallmarks.generic.callback =
        Some(Preferences_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_preferences.wallmarks.generic.id = 131 as i32;
    s_preferences.wallmarks.generic.x = 360 as i32;
    s_preferences.wallmarks.generic.y = y;
    y += 16 as i32 + 2 as i32;
    s_preferences.brass.generic.type_0 = 5 as i32;
    s_preferences.brass.generic.name = b"Ejecting Brass:\x00" as *const u8 as *const libc::c_char;
    s_preferences.brass.generic.flags = 0x100 as i32 as u32 | 0x2 as i32 as u32;
    s_preferences.brass.generic.callback =
        Some(Preferences_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_preferences.brass.generic.id = 130 as i32;
    s_preferences.brass.generic.x = 360 as i32;
    s_preferences.brass.generic.y = y;
    y += 16 as i32 + 2 as i32;
    s_preferences.dynamiclights.generic.type_0 = 5 as i32;
    s_preferences.dynamiclights.generic.name =
        b"Dynamic Lights:\x00" as *const u8 as *const libc::c_char;
    s_preferences.dynamiclights.generic.flags = 0x100 as i32 as u32 | 0x2 as i32 as u32;
    s_preferences.dynamiclights.generic.callback =
        Some(Preferences_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_preferences.dynamiclights.generic.id = 132 as i32;
    s_preferences.dynamiclights.generic.x = 360 as i32;
    s_preferences.dynamiclights.generic.y = y;
    y += 16 as i32 + 2 as i32;
    s_preferences.identifytarget.generic.type_0 = 5 as i32;
    s_preferences.identifytarget.generic.name =
        b"Identify Target:\x00" as *const u8 as *const libc::c_char;
    s_preferences.identifytarget.generic.flags = 0x100 as i32 as u32 | 0x2 as i32 as u32;
    s_preferences.identifytarget.generic.callback =
        Some(Preferences_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_preferences.identifytarget.generic.id = 133 as i32;
    s_preferences.identifytarget.generic.x = 360 as i32;
    s_preferences.identifytarget.generic.y = y;
    y += 16 as i32 + 2 as i32;
    s_preferences.highqualitysky.generic.type_0 = 5 as i32;
    s_preferences.highqualitysky.generic.name =
        b"High Quality Sky:\x00" as *const u8 as *const libc::c_char;
    s_preferences.highqualitysky.generic.flags = 0x100 as i32 as u32 | 0x2 as i32 as u32;
    s_preferences.highqualitysky.generic.callback =
        Some(Preferences_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_preferences.highqualitysky.generic.id = 129 as i32;
    s_preferences.highqualitysky.generic.x = 360 as i32;
    s_preferences.highqualitysky.generic.y = y;
    y += 16 as i32 + 2 as i32;
    s_preferences.synceveryframe.generic.type_0 = 5 as i32;
    s_preferences.synceveryframe.generic.name =
        b"Sync Every Frame:\x00" as *const u8 as *const libc::c_char;
    s_preferences.synceveryframe.generic.flags = 0x100 as i32 as u32 | 0x2 as i32 as u32;
    s_preferences.synceveryframe.generic.callback =
        Some(Preferences_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_preferences.synceveryframe.generic.id = 134 as i32;
    s_preferences.synceveryframe.generic.x = 360 as i32;
    s_preferences.synceveryframe.generic.y = y;
    y += 16 as i32 + 2 as i32;
    s_preferences.forcemodel.generic.type_0 = 5 as i32;
    s_preferences.forcemodel.generic.name =
        b"Force Player Models:\x00" as *const u8 as *const libc::c_char;
    s_preferences.forcemodel.generic.flags = 0x100 as i32 as u32 | 0x2 as i32 as u32;
    s_preferences.forcemodel.generic.callback =
        Some(Preferences_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_preferences.forcemodel.generic.id = 135 as i32;
    s_preferences.forcemodel.generic.x = 360 as i32;
    s_preferences.forcemodel.generic.y = y;
    y += 16 as i32 + 2 as i32;
    s_preferences.drawteamoverlay.generic.type_0 = 3 as i32;
    s_preferences.drawteamoverlay.generic.name =
        b"Draw Team Overlay:\x00" as *const u8 as *const libc::c_char;
    s_preferences.drawteamoverlay.generic.flags = 0x100 as i32 as u32 | 0x2 as i32 as u32;
    s_preferences.drawteamoverlay.generic.callback =
        Some(Preferences_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_preferences.drawteamoverlay.generic.id = 136 as i32;
    s_preferences.drawteamoverlay.generic.x = 360 as i32;
    s_preferences.drawteamoverlay.generic.y = y;
    s_preferences.drawteamoverlay.itemnames = teamoverlay_names.as_mut_ptr();
    y += 16 as i32 + 2 as i32;
    s_preferences.allowdownload.generic.type_0 = 5 as i32;
    s_preferences.allowdownload.generic.name =
        b"Automatic Downloading:\x00" as *const u8 as *const libc::c_char;
    s_preferences.allowdownload.generic.flags = 0x100 as i32 as u32 | 0x2 as i32 as u32;
    s_preferences.allowdownload.generic.callback =
        Some(Preferences_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_preferences.allowdownload.generic.id = 137 as i32;
    s_preferences.allowdownload.generic.x = 360 as i32;
    s_preferences.allowdownload.generic.y = y;
    s_preferences.back.generic.type_0 = 6 as i32;
    s_preferences.back.generic.name = b"menu/art/back_0\x00" as *const u8 as *const libc::c_char;
    s_preferences.back.generic.flags = 0x4 as i32 as u32 | 0x100 as i32 as u32;
    s_preferences.back.generic.callback =
        Some(Preferences_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_preferences.back.generic.id = 138 as i32;
    s_preferences.back.generic.x = 0 as i32;
    s_preferences.back.generic.y = 480 as i32 - 64 as i32;
    s_preferences.back.width = 128 as i32;
    s_preferences.back.height = 64 as i32;
    s_preferences.back.focuspic =
        b"menu/art/back_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_preferences.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut s_preferences.banner as *mut crate::ui_local_h::menutext_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_preferences.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut s_preferences.framel as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_preferences.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut s_preferences.framer as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_preferences.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut s_preferences.crosshair as *mut crate::ui_local_h::menulist_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_preferences.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut s_preferences.simpleitems as *mut crate::ui_local_h::menuradiobutton_s
            as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_preferences.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut s_preferences.wallmarks as *mut crate::ui_local_h::menuradiobutton_s
            as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_preferences.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut s_preferences.brass as *mut crate::ui_local_h::menuradiobutton_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_preferences.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut s_preferences.dynamiclights as *mut crate::ui_local_h::menuradiobutton_s
            as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_preferences.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut s_preferences.identifytarget as *mut crate::ui_local_h::menuradiobutton_s
            as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_preferences.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut s_preferences.highqualitysky as *mut crate::ui_local_h::menuradiobutton_s
            as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_preferences.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut s_preferences.synceveryframe as *mut crate::ui_local_h::menuradiobutton_s
            as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_preferences.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut s_preferences.forcemodel as *mut crate::ui_local_h::menuradiobutton_s
            as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_preferences.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut s_preferences.drawteamoverlay as *mut crate::ui_local_h::menulist_s
            as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_preferences.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut s_preferences.allowdownload as *mut crate::ui_local_h::menuradiobutton_s
            as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_preferences.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut s_preferences.back as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    Preferences_SetMenuItems();
}
/*
===============
Preferences_Cache
===============
*/
#[no_mangle]

pub unsafe extern "C" fn Preferences_Cache() {
    let mut n: i32 = 0;
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
    n = 0 as i32;
    while n < 10 as i32 {
        s_preferences.crosshairShader[n as usize] =
            crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
                crate::src::qcommon::q_shared::va(
                    b"gfx/2d/crosshair%c\x00" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    'a' as i32 + n,
                ),
            );
        n += 1
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
/*
===============
UI_PreferencesMenu
===============
*/
#[no_mangle]

pub unsafe extern "C" fn UI_PreferencesMenu() {
    Preferences_MenuInit();
    crate::src::q3_ui::ui_atoms::UI_PushMenu(
        &mut s_preferences.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
    );
}

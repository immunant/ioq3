use ::libc;

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
pub use crate::ui_public_h::uiClientState_t;

pub use crate::keycodes_h::K_ALT;
pub use crate::keycodes_h::K_AUX1;
pub use crate::keycodes_h::K_AUX10;
pub use crate::keycodes_h::K_AUX11;
pub use crate::keycodes_h::K_AUX12;
pub use crate::keycodes_h::K_AUX13;
pub use crate::keycodes_h::K_AUX14;
pub use crate::keycodes_h::K_AUX15;
pub use crate::keycodes_h::K_AUX16;
pub use crate::keycodes_h::K_AUX2;
pub use crate::keycodes_h::K_AUX3;
pub use crate::keycodes_h::K_AUX4;
pub use crate::keycodes_h::K_AUX5;
pub use crate::keycodes_h::K_AUX6;
pub use crate::keycodes_h::K_AUX7;
pub use crate::keycodes_h::K_AUX8;
pub use crate::keycodes_h::K_AUX9;
pub use crate::keycodes_h::K_BACKSPACE;
pub use crate::keycodes_h::K_BREAK;
pub use crate::keycodes_h::K_CAPSLOCK;
pub use crate::keycodes_h::K_COMMAND;
pub use crate::keycodes_h::K_COMPOSE;
pub use crate::keycodes_h::K_CONSOLE;
pub use crate::keycodes_h::K_CTRL;
pub use crate::keycodes_h::K_DEL;
pub use crate::keycodes_h::K_DOWNARROW;
pub use crate::keycodes_h::K_END;
pub use crate::keycodes_h::K_ENTER;
pub use crate::keycodes_h::K_ESCAPE;
pub use crate::keycodes_h::K_EURO;
pub use crate::keycodes_h::K_F1;
pub use crate::keycodes_h::K_F10;
pub use crate::keycodes_h::K_F11;
pub use crate::keycodes_h::K_F12;
pub use crate::keycodes_h::K_F13;
pub use crate::keycodes_h::K_F14;
pub use crate::keycodes_h::K_F15;
pub use crate::keycodes_h::K_F2;
pub use crate::keycodes_h::K_F3;
pub use crate::keycodes_h::K_F4;
pub use crate::keycodes_h::K_F5;
pub use crate::keycodes_h::K_F6;
pub use crate::keycodes_h::K_F7;
pub use crate::keycodes_h::K_F8;
pub use crate::keycodes_h::K_F9;
pub use crate::keycodes_h::K_HELP;
pub use crate::keycodes_h::K_HOME;
pub use crate::keycodes_h::K_INS;
pub use crate::keycodes_h::K_JOY1;
pub use crate::keycodes_h::K_JOY10;
pub use crate::keycodes_h::K_JOY11;
pub use crate::keycodes_h::K_JOY12;
pub use crate::keycodes_h::K_JOY13;
pub use crate::keycodes_h::K_JOY14;
pub use crate::keycodes_h::K_JOY15;
pub use crate::keycodes_h::K_JOY16;
pub use crate::keycodes_h::K_JOY17;
pub use crate::keycodes_h::K_JOY18;
pub use crate::keycodes_h::K_JOY19;
pub use crate::keycodes_h::K_JOY2;
pub use crate::keycodes_h::K_JOY20;
pub use crate::keycodes_h::K_JOY21;
pub use crate::keycodes_h::K_JOY22;
pub use crate::keycodes_h::K_JOY23;
pub use crate::keycodes_h::K_JOY24;
pub use crate::keycodes_h::K_JOY25;
pub use crate::keycodes_h::K_JOY26;
pub use crate::keycodes_h::K_JOY27;
pub use crate::keycodes_h::K_JOY28;
pub use crate::keycodes_h::K_JOY29;
pub use crate::keycodes_h::K_JOY3;
pub use crate::keycodes_h::K_JOY30;
pub use crate::keycodes_h::K_JOY31;
pub use crate::keycodes_h::K_JOY32;
pub use crate::keycodes_h::K_JOY4;
pub use crate::keycodes_h::K_JOY5;
pub use crate::keycodes_h::K_JOY6;
pub use crate::keycodes_h::K_JOY7;
pub use crate::keycodes_h::K_JOY8;
pub use crate::keycodes_h::K_JOY9;
pub use crate::keycodes_h::K_KP_5;
pub use crate::keycodes_h::K_KP_DEL;
pub use crate::keycodes_h::K_KP_DOWNARROW;
pub use crate::keycodes_h::K_KP_END;
pub use crate::keycodes_h::K_KP_ENTER;
pub use crate::keycodes_h::K_KP_EQUALS;
pub use crate::keycodes_h::K_KP_HOME;
pub use crate::keycodes_h::K_KP_INS;
pub use crate::keycodes_h::K_KP_LEFTARROW;
pub use crate::keycodes_h::K_KP_MINUS;
pub use crate::keycodes_h::K_KP_NUMLOCK;
pub use crate::keycodes_h::K_KP_PGDN;
pub use crate::keycodes_h::K_KP_PGUP;
pub use crate::keycodes_h::K_KP_PLUS;
pub use crate::keycodes_h::K_KP_RIGHTARROW;
pub use crate::keycodes_h::K_KP_SLASH;
pub use crate::keycodes_h::K_KP_STAR;
pub use crate::keycodes_h::K_KP_UPARROW;
pub use crate::keycodes_h::K_LEFTARROW;
pub use crate::keycodes_h::K_MENU;
pub use crate::keycodes_h::K_MODE;
pub use crate::keycodes_h::K_MOUSE1;
pub use crate::keycodes_h::K_MOUSE2;
pub use crate::keycodes_h::K_MOUSE3;
pub use crate::keycodes_h::K_MOUSE4;
pub use crate::keycodes_h::K_MOUSE5;
pub use crate::keycodes_h::K_MWHEELDOWN;
pub use crate::keycodes_h::K_MWHEELUP;
pub use crate::keycodes_h::K_PAD0_A;
pub use crate::keycodes_h::K_PAD0_B;
pub use crate::keycodes_h::K_PAD0_BACK;
pub use crate::keycodes_h::K_PAD0_DPAD_DOWN;
pub use crate::keycodes_h::K_PAD0_DPAD_LEFT;
pub use crate::keycodes_h::K_PAD0_DPAD_RIGHT;
pub use crate::keycodes_h::K_PAD0_DPAD_UP;
pub use crate::keycodes_h::K_PAD0_GUIDE;
pub use crate::keycodes_h::K_PAD0_LEFTSHOULDER;
pub use crate::keycodes_h::K_PAD0_LEFTSTICK_CLICK;
pub use crate::keycodes_h::K_PAD0_LEFTSTICK_DOWN;
pub use crate::keycodes_h::K_PAD0_LEFTSTICK_LEFT;
pub use crate::keycodes_h::K_PAD0_LEFTSTICK_RIGHT;
pub use crate::keycodes_h::K_PAD0_LEFTSTICK_UP;
pub use crate::keycodes_h::K_PAD0_LEFTTRIGGER;
pub use crate::keycodes_h::K_PAD0_RIGHTSHOULDER;
pub use crate::keycodes_h::K_PAD0_RIGHTSTICK_CLICK;
pub use crate::keycodes_h::K_PAD0_RIGHTSTICK_DOWN;
pub use crate::keycodes_h::K_PAD0_RIGHTSTICK_LEFT;
pub use crate::keycodes_h::K_PAD0_RIGHTSTICK_RIGHT;
pub use crate::keycodes_h::K_PAD0_RIGHTSTICK_UP;
pub use crate::keycodes_h::K_PAD0_RIGHTTRIGGER;
pub use crate::keycodes_h::K_PAD0_START;
pub use crate::keycodes_h::K_PAD0_X;
pub use crate::keycodes_h::K_PAD0_Y;
pub use crate::keycodes_h::K_PAUSE;
pub use crate::keycodes_h::K_PGDN;
pub use crate::keycodes_h::K_PGUP;
pub use crate::keycodes_h::K_POWER;
pub use crate::keycodes_h::K_PRINT;
pub use crate::keycodes_h::K_RIGHTARROW;
pub use crate::keycodes_h::K_SCROLLOCK;
pub use crate::keycodes_h::K_SHIFT;
pub use crate::keycodes_h::K_SPACE;
pub use crate::keycodes_h::K_SUPER;
pub use crate::keycodes_h::K_SYSREQ;
pub use crate::keycodes_h::K_TAB;
pub use crate::keycodes_h::K_UNDO;
pub use crate::keycodes_h::K_UPARROW;
pub use crate::keycodes_h::K_WORLD_0;
pub use crate::keycodes_h::K_WORLD_1;
pub use crate::keycodes_h::K_WORLD_10;
pub use crate::keycodes_h::K_WORLD_11;
pub use crate::keycodes_h::K_WORLD_12;
pub use crate::keycodes_h::K_WORLD_13;
pub use crate::keycodes_h::K_WORLD_14;
pub use crate::keycodes_h::K_WORLD_15;
pub use crate::keycodes_h::K_WORLD_16;
pub use crate::keycodes_h::K_WORLD_17;
pub use crate::keycodes_h::K_WORLD_18;
pub use crate::keycodes_h::K_WORLD_19;
pub use crate::keycodes_h::K_WORLD_2;
pub use crate::keycodes_h::K_WORLD_20;
pub use crate::keycodes_h::K_WORLD_21;
pub use crate::keycodes_h::K_WORLD_22;
pub use crate::keycodes_h::K_WORLD_23;
pub use crate::keycodes_h::K_WORLD_24;
pub use crate::keycodes_h::K_WORLD_25;
pub use crate::keycodes_h::K_WORLD_26;
pub use crate::keycodes_h::K_WORLD_27;
pub use crate::keycodes_h::K_WORLD_28;
pub use crate::keycodes_h::K_WORLD_29;
pub use crate::keycodes_h::K_WORLD_3;
pub use crate::keycodes_h::K_WORLD_30;
pub use crate::keycodes_h::K_WORLD_31;
pub use crate::keycodes_h::K_WORLD_32;
pub use crate::keycodes_h::K_WORLD_33;
pub use crate::keycodes_h::K_WORLD_34;
pub use crate::keycodes_h::K_WORLD_35;
pub use crate::keycodes_h::K_WORLD_36;
pub use crate::keycodes_h::K_WORLD_37;
pub use crate::keycodes_h::K_WORLD_38;
pub use crate::keycodes_h::K_WORLD_39;
pub use crate::keycodes_h::K_WORLD_4;
pub use crate::keycodes_h::K_WORLD_40;
pub use crate::keycodes_h::K_WORLD_41;
pub use crate::keycodes_h::K_WORLD_42;
pub use crate::keycodes_h::K_WORLD_43;
pub use crate::keycodes_h::K_WORLD_44;
pub use crate::keycodes_h::K_WORLD_45;
pub use crate::keycodes_h::K_WORLD_46;
pub use crate::keycodes_h::K_WORLD_47;
pub use crate::keycodes_h::K_WORLD_48;
pub use crate::keycodes_h::K_WORLD_49;
pub use crate::keycodes_h::K_WORLD_5;
pub use crate::keycodes_h::K_WORLD_50;
pub use crate::keycodes_h::K_WORLD_51;
pub use crate::keycodes_h::K_WORLD_52;
pub use crate::keycodes_h::K_WORLD_53;
pub use crate::keycodes_h::K_WORLD_54;
pub use crate::keycodes_h::K_WORLD_55;
pub use crate::keycodes_h::K_WORLD_56;
pub use crate::keycodes_h::K_WORLD_57;
pub use crate::keycodes_h::K_WORLD_58;
pub use crate::keycodes_h::K_WORLD_59;
pub use crate::keycodes_h::K_WORLD_6;
pub use crate::keycodes_h::K_WORLD_60;
pub use crate::keycodes_h::K_WORLD_61;
pub use crate::keycodes_h::K_WORLD_62;
pub use crate::keycodes_h::K_WORLD_63;
pub use crate::keycodes_h::K_WORLD_64;
pub use crate::keycodes_h::K_WORLD_65;
pub use crate::keycodes_h::K_WORLD_66;
pub use crate::keycodes_h::K_WORLD_67;
pub use crate::keycodes_h::K_WORLD_68;
pub use crate::keycodes_h::K_WORLD_69;
pub use crate::keycodes_h::K_WORLD_7;
pub use crate::keycodes_h::K_WORLD_70;
pub use crate::keycodes_h::K_WORLD_71;
pub use crate::keycodes_h::K_WORLD_72;
pub use crate::keycodes_h::K_WORLD_73;
pub use crate::keycodes_h::K_WORLD_74;
pub use crate::keycodes_h::K_WORLD_75;
pub use crate::keycodes_h::K_WORLD_76;
pub use crate::keycodes_h::K_WORLD_77;
pub use crate::keycodes_h::K_WORLD_78;
pub use crate::keycodes_h::K_WORLD_79;
pub use crate::keycodes_h::K_WORLD_8;
pub use crate::keycodes_h::K_WORLD_80;
pub use crate::keycodes_h::K_WORLD_81;
pub use crate::keycodes_h::K_WORLD_82;
pub use crate::keycodes_h::K_WORLD_83;
pub use crate::keycodes_h::K_WORLD_84;
pub use crate::keycodes_h::K_WORLD_85;
pub use crate::keycodes_h::K_WORLD_86;
pub use crate::keycodes_h::K_WORLD_87;
pub use crate::keycodes_h::K_WORLD_88;
pub use crate::keycodes_h::K_WORLD_89;
pub use crate::keycodes_h::K_WORLD_9;
pub use crate::keycodes_h::K_WORLD_90;
pub use crate::keycodes_h::K_WORLD_91;
pub use crate::keycodes_h::K_WORLD_92;
pub use crate::keycodes_h::K_WORLD_93;
pub use crate::keycodes_h::K_WORLD_94;
pub use crate::keycodes_h::K_WORLD_95;
pub use crate::keycodes_h::MAX_KEYS;
pub use crate::src::q3_ui::ui_atoms::UI_DrawNamedPic;
pub use crate::src::q3_ui::ui_atoms::UI_DrawProportionalString;
pub use crate::src::q3_ui::ui_atoms::UI_PopMenu;
pub use crate::src::q3_ui::ui_atoms::UI_ProportionalStringWidth;
pub use crate::src::q3_ui::ui_atoms::UI_PushMenu;
pub use crate::src::q3_ui::ui_qmenu::color_red;
pub use crate::src::q3_ui::ui_qmenu::Menu_AddItem;
pub use crate::src::q3_ui::ui_qmenu::Menu_DefaultKey;
pub use crate::src::q3_ui::ui_qmenu::Menu_Draw;
pub use crate::src::q3_ui::ui_qmenu::Menu_SetCursorToItem;
pub use crate::src::qcommon::q_shared::C2RustUnnamed_0;
pub use crate::src::ui::ui_syscalls::trap_GetClientState;
pub use crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip;

pub use crate::ui_local_h::_tag_menuframework;
pub use crate::ui_local_h::menucommon_s;
pub use crate::ui_local_h::menuframework_s;
pub use crate::ui_local_h::menutext_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct confirmMenu_t {
    pub menu: menuframework_s,
    pub no: menutext_s,
    pub yes: menutext_s,
    pub slashX: i32,
    pub question: *const libc::c_char,
    pub draw: Option<unsafe extern "C" fn() -> ()>,
    pub action: Option<unsafe extern "C" fn(_: qboolean) -> ()>,
    pub style: i32,
    pub lines: *mut *const libc::c_char,
}

static mut s_confirm: confirmMenu_t = confirmMenu_t {
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
    no: menutext_s {
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
    yes: menutext_s {
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
    slashX: 0,
    question: 0 as *const libc::c_char,
    draw: None,
    action: None,
    style: 0,
    lines: 0 as *const *const libc::c_char as *mut *const libc::c_char,
};
/*
=================
ConfirmMenu_Event
=================
*/

unsafe extern "C" fn ConfirmMenu_Event(mut ptr: *mut libc::c_void, mut event: i32) {
    let mut result: qboolean = qfalse;
    if event != 3 as i32 {
        return;
    }
    UI_PopMenu();
    if (*(ptr as *mut menucommon_s)).id == 10 as i32 {
        result = qfalse
    } else {
        result = qtrue
    }
    if s_confirm.action.is_some() {
        s_confirm.action.expect("non-null function pointer")(result);
    };
}
/*
=================
ConfirmMenu_Key
=================
*/

unsafe extern "C" fn ConfirmMenu_Key(mut key: i32) -> sfxHandle_t {
    match key {
        163 | 134 | 165 | 135 => key = K_TAB as i32,
        110 | 78 => {
            ConfirmMenu_Event(
                &mut s_confirm.no as *mut menutext_s as *mut libc::c_void,
                3 as i32,
            );
        }
        121 | 89 => {
            ConfirmMenu_Event(
                &mut s_confirm.yes as *mut menutext_s as *mut libc::c_void,
                3 as i32,
            );
        }
        _ => {}
    }
    return Menu_DefaultKey(
        &mut s_confirm.menu as *mut _ as *mut _tag_menuframework,
        key,
    );
}
/*
=================
MessaheMenu_Draw
=================
*/

unsafe extern "C" fn MessageMenu_Draw() {
    let mut i: i32 = 0;
    let mut y: i32 = 0;
    UI_DrawNamedPic(
        142 as i32 as f32,
        118 as i32 as f32,
        359 as i32 as f32,
        256 as i32 as f32,
        b"menu/art/cut_frame\x00" as *const u8 as *const libc::c_char,
    );
    y = 188 as i32;
    i = 0 as i32;
    while !(*s_confirm.lines.offset(i as isize)).is_null() {
        UI_DrawProportionalString(
            320 as i32,
            y,
            *s_confirm.lines.offset(i as isize),
            s_confirm.style,
            color_red.as_mut_ptr(),
        );
        y += 18 as i32;
        i += 1
    }
    Menu_Draw(
        &mut s_confirm.menu as *mut _ as *mut _tag_menuframework,
    );
    if s_confirm.draw.is_some() {
        s_confirm.draw.expect("non-null function pointer")();
    };
}
/*
=================
ConfirmMenu_Draw
=================
*/

unsafe extern "C" fn ConfirmMenu_Draw() {
    UI_DrawNamedPic(
        142 as i32 as f32,
        118 as i32 as f32,
        359 as i32 as f32,
        256 as i32 as f32,
        b"menu/art/cut_frame\x00" as *const u8 as *const libc::c_char,
    );
    UI_DrawProportionalString(
        320 as i32,
        204 as i32,
        s_confirm.question,
        s_confirm.style,
        color_red.as_mut_ptr(),
    );
    UI_DrawProportionalString(
        s_confirm.slashX,
        265 as i32,
        b"/\x00" as *const u8 as *const libc::c_char,
        0 as i32 | 0x2000 as i32,
        color_red.as_mut_ptr(),
    );
    Menu_Draw(
        &mut s_confirm.menu as *mut _ as *mut _tag_menuframework,
    );
    if s_confirm.draw.is_some() {
        s_confirm.draw.expect("non-null function pointer")();
    };
}
/*
=================
ConfirmMenu_Cache
=================
*/
#[no_mangle]

pub unsafe extern "C" fn ConfirmMenu_Cache() {
    trap_R_RegisterShaderNoMip(
        b"menu/art/cut_frame\x00" as *const u8 as *const libc::c_char,
    );
}
/*
=================
UI_ConfirmMenu_Stlye
=================
*/
#[no_mangle]

pub unsafe extern "C" fn UI_ConfirmMenu_Style(
    mut question: *const libc::c_char,
    mut style: i32,
    mut draw: Option<unsafe extern "C" fn() -> ()>,
    mut action: Option<unsafe extern "C" fn(_: qboolean) -> ()>,
) {
    let mut cstate: uiClientState_t = uiClientState_t {
        connState: CA_UNINITIALIZED,
        connectPacketCount: 0,
        clientNum: 0,
        servername: [0; 1024],
        updateInfoString: [0; 1024],
        messageString: [0; 1024],
    };
    let mut n1: i32 = 0;
    let mut n2: i32 = 0;
    let mut n3: i32 = 0;
    let mut l1: i32 = 0;
    let mut l2: i32 = 0;
    let mut l3: i32 = 0;
    // zero set all our globals
    crate::stdlib::memset(
        &mut s_confirm as *mut confirmMenu_t as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<confirmMenu_t>() as libc::c_ulong,
    );
    ConfirmMenu_Cache();
    n1 = UI_ProportionalStringWidth(
        b"YES/NO\x00" as *const u8 as *const libc::c_char,
    );
    n2 = UI_ProportionalStringWidth(
        b"YES\x00" as *const u8 as *const libc::c_char,
    ) + 3 as i32;
    n3 = UI_ProportionalStringWidth(
        b"/\x00" as *const u8 as *const libc::c_char,
    ) + 3 as i32;
    l1 = 320 as i32 - n1 / 2 as i32;
    l2 = l1 + n2;
    l3 = l2 + n3;
    s_confirm.slashX = l2;
    s_confirm.question = question;
    s_confirm.draw = draw;
    s_confirm.action = action;
    s_confirm.style = style;
    s_confirm.menu.draw = Some(ConfirmMenu_Draw as unsafe extern "C" fn() -> ());
    s_confirm.menu.key = Some(
        ConfirmMenu_Key
            as unsafe extern "C" fn(_: i32) -> sfxHandle_t,
    );
    s_confirm.menu.wrapAround = qtrue;
    trap_GetClientState(
        &mut cstate as *mut _ as *mut uiClientState_t,
    );
    if cstate.connState as u32 >= CA_CONNECTED as i32 as u32 {
        s_confirm.menu.fullscreen = qfalse
    } else {
        s_confirm.menu.fullscreen = qtrue
    }
    s_confirm.yes.generic.type_0 = 9 as i32;
    s_confirm.yes.generic.flags = 0x4 as i32 as u32 | 0x100 as i32 as u32;
    s_confirm.yes.generic.callback =
        Some(ConfirmMenu_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_confirm.yes.generic.id = 11 as i32;
    s_confirm.yes.generic.x = l1;
    s_confirm.yes.generic.y = 264 as i32;
    s_confirm.yes.string = b"YES\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_confirm.yes.color = color_red.as_mut_ptr();
    s_confirm.yes.style = 0 as i32;
    s_confirm.no.generic.type_0 = 9 as i32;
    s_confirm.no.generic.flags = 0x4 as i32 as u32 | 0x100 as i32 as u32;
    s_confirm.no.generic.callback =
        Some(ConfirmMenu_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_confirm.no.generic.id = 10 as i32;
    s_confirm.no.generic.x = l3;
    s_confirm.no.generic.y = 264 as i32;
    s_confirm.no.string = b"NO\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_confirm.no.color = color_red.as_mut_ptr();
    s_confirm.no.style = 0 as i32;
    Menu_AddItem(
        &mut s_confirm.menu as *mut _ as *mut _tag_menuframework,
        &mut s_confirm.yes as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_confirm.menu as *mut _ as *mut _tag_menuframework,
        &mut s_confirm.no as *mut menutext_s as *mut libc::c_void,
    );
    UI_PushMenu(
        &mut s_confirm.menu as *mut _ as *mut _tag_menuframework,
    );
    Menu_SetCursorToItem(
        &mut s_confirm.menu as *mut _ as *mut _tag_menuframework,
        &mut s_confirm.no as *mut menutext_s as *mut libc::c_void,
    );
}
/*
=================
UI_ConfirmMenu
=================
*/
#[no_mangle]

pub unsafe extern "C" fn UI_ConfirmMenu(
    mut question: *const libc::c_char,
    mut draw: Option<unsafe extern "C" fn() -> ()>,
    mut action: Option<unsafe extern "C" fn(_: qboolean) -> ()>,
) {
    UI_ConfirmMenu_Style(question, 0x1 as i32 | 0x2000 as i32, draw, action);
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
/*
=================
UI_Message
hacked over from Confirm stuff
=================
*/
#[no_mangle]

pub unsafe extern "C" fn UI_Message(mut lines: *mut *const libc::c_char) {
    let mut cstate: uiClientState_t = uiClientState_t {
        connState: CA_UNINITIALIZED,
        connectPacketCount: 0,
        clientNum: 0,
        servername: [0; 1024],
        updateInfoString: [0; 1024],
        messageString: [0; 1024],
    };
    let mut n1: i32 = 0;
    let mut l1: i32 = 0;
    // zero set all our globals
    crate::stdlib::memset(
        &mut s_confirm as *mut confirmMenu_t as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<confirmMenu_t>() as libc::c_ulong,
    );
    ConfirmMenu_Cache();
    n1 = UI_ProportionalStringWidth(
        b"OK\x00" as *const u8 as *const libc::c_char,
    );
    l1 = 320 as i32 - n1 / 2 as i32;
    s_confirm.lines = lines;
    s_confirm.style = 0x1 as i32 | 0x2000 as i32 | 0x10 as i32;
    s_confirm.menu.draw = Some(MessageMenu_Draw as unsafe extern "C" fn() -> ());
    s_confirm.menu.key = Some(
        ConfirmMenu_Key
            as unsafe extern "C" fn(_: i32) -> sfxHandle_t,
    );
    s_confirm.menu.wrapAround = qtrue;
    trap_GetClientState(
        &mut cstate as *mut _ as *mut uiClientState_t,
    );
    if cstate.connState as u32 >= CA_CONNECTED as i32 as u32 {
        s_confirm.menu.fullscreen = qfalse
    } else {
        s_confirm.menu.fullscreen = qtrue
    }
    s_confirm.yes.generic.type_0 = 9 as i32;
    s_confirm.yes.generic.flags = 0x4 as i32 as u32 | 0x100 as i32 as u32;
    s_confirm.yes.generic.callback =
        Some(ConfirmMenu_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_confirm.yes.generic.id = 11 as i32;
    s_confirm.yes.generic.x = l1;
    s_confirm.yes.generic.y = 280 as i32;
    s_confirm.yes.string = b"OK\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_confirm.yes.color = color_red.as_mut_ptr();
    s_confirm.yes.style = 0 as i32;
    Menu_AddItem(
        &mut s_confirm.menu as *mut _ as *mut _tag_menuframework,
        &mut s_confirm.yes as *mut menutext_s as *mut libc::c_void,
    );
    UI_PushMenu(
        &mut s_confirm.menu as *mut _ as *mut _tag_menuframework,
    );
    Menu_SetCursorToItem(
        &mut s_confirm.menu as *mut _ as *mut _tag_menuframework,
        &mut s_confirm.yes as *mut menutext_s as *mut libc::c_void,
    );
}

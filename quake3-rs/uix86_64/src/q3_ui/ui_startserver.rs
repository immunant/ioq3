use ::libc;

pub mod stdlib_h {

    #[inline]

    pub unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> i32 {
        return libc::strtol(
            __nptr,
            std::ptr::null_mut() as *mut *mut libc::c_char,
            10 as i32,
        ) as i32;
    }
}

pub use crate::stddef_h::size_t;

pub use crate::bg_public_h::GT_1FCTF;
pub use crate::bg_public_h::GT_CTF;
pub use crate::bg_public_h::GT_FFA;
pub use crate::bg_public_h::GT_HARVESTER;
pub use crate::bg_public_h::GT_MAX_GAME_TYPE;
pub use crate::bg_public_h::GT_OBELISK;
pub use crate::bg_public_h::GT_SINGLE_PLAYER;
pub use crate::bg_public_h::GT_TEAM;
pub use crate::bg_public_h::GT_TOURNAMENT;
pub use crate::src::q3_ui::ui_atoms::UI_Cvar_VariableString;
pub use crate::src::q3_ui::ui_atoms::UI_DrawChar;
pub use crate::src::q3_ui::ui_atoms::UI_DrawHandlePic;
pub use crate::src::q3_ui::ui_atoms::UI_DrawString;
pub use crate::src::q3_ui::ui_atoms::UI_FillRect;
pub use crate::src::q3_ui::ui_atoms::UI_PopMenu;
pub use crate::src::q3_ui::ui_atoms::UI_PushMenu;
pub use crate::src::q3_ui::ui_gameinfo::UI_GetArenaInfoByMap;
pub use crate::src::q3_ui::ui_gameinfo::UI_GetArenaInfoByNumber;
pub use crate::src::q3_ui::ui_gameinfo::UI_GetBotInfoByName;
pub use crate::src::q3_ui::ui_gameinfo::UI_GetBotInfoByNumber;
pub use crate::src::q3_ui::ui_gameinfo::UI_GetNumArenas;
pub use crate::src::q3_ui::ui_gameinfo::UI_GetNumBots;
pub use crate::src::q3_ui::ui_qmenu::color_orange;
pub use crate::src::q3_ui::ui_qmenu::color_red;
pub use crate::src::q3_ui::ui_qmenu::color_white;
pub use crate::src::q3_ui::ui_qmenu::listbar_color;
pub use crate::src::q3_ui::ui_qmenu::text_color_disabled;
pub use crate::src::q3_ui::ui_qmenu::text_color_highlight;
pub use crate::src::q3_ui::ui_qmenu::text_color_normal;
pub use crate::src::q3_ui::ui_qmenu::Bitmap_Draw;
pub use crate::src::q3_ui::ui_qmenu::Menu_AddItem;
pub use crate::src::q3_ui::ui_startserver::stdlib_h::atoi;
pub use crate::src::qcommon::q_math::colorBlack;
pub use crate::src::qcommon::q_math::colorRed;
pub use crate::src::qcommon::q_math::colorWhite;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qhandle_t;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::sfxHandle_t;
pub use crate::src::qcommon::q_shared::va;
pub use crate::src::qcommon::q_shared::vec4_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::qcommon::q_shared::C2RustUnnamed_0;
pub use crate::src::qcommon::q_shared::COM_ParseExt;
pub use crate::src::qcommon::q_shared::Com_Clamp;
pub use crate::src::qcommon::q_shared::Com_sprintf;
pub use crate::src::qcommon::q_shared::Info_ValueForKey;
pub use crate::src::qcommon::q_shared::Q_CleanStr;
pub use crate::src::qcommon::q_shared::Q_stricmp;
pub use crate::src::qcommon::q_shared::Q_strncpyz;
pub use crate::src::qcommon::q_shared::Q_strupr;
pub use crate::src::qcommon::q_shared::EXEC_APPEND;
pub use crate::src::qcommon::q_shared::EXEC_INSERT;
pub use crate::src::qcommon::q_shared::EXEC_NOW;
pub use crate::src::ui::ui_syscalls::trap_Cmd_ExecuteText;
pub use crate::src::ui::ui_syscalls::trap_Cvar_Set;
pub use crate::src::ui::ui_syscalls::trap_Cvar_SetValue;
pub use crate::src::ui::ui_syscalls::trap_Cvar_VariableStringBuffer;
pub use crate::src::ui::ui_syscalls::trap_Cvar_VariableValue;
pub use crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip;
pub use crate::stdlib::__compar_fn_t;

pub use crate::stdlib::qsort;

pub use crate::ui_local_h::_tag_menuframework;
pub use crate::ui_local_h::menubitmap_s;
pub use crate::ui_local_h::menucommon_s;
pub use crate::ui_local_h::menufield_s;
pub use crate::ui_local_h::menuframework_s;
pub use crate::ui_local_h::menulist_s;
pub use crate::ui_local_h::menuradiobutton_s;
pub use crate::ui_local_h::menutext_s;
pub use crate::ui_local_h::mfield_t;

pub use ::libc::strtol;
extern "C" {
    // use ui_servers2.c definition
    #[no_mangle]
    pub static mut punkbuster_items: [*const libc::c_char; 0];
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct startserver_t {
    pub menu: menuframework_s,
    pub banner: menutext_s,
    pub framel: menubitmap_s,
    pub framer: menubitmap_s,
    pub gametype: menulist_s,
    pub mappics: [menubitmap_s; 4],
    pub mapbuttons: [menubitmap_s; 4],
    pub arrows: menubitmap_s,
    pub prevpage: menubitmap_s,
    pub nextpage: menubitmap_s,
    pub back: menubitmap_s,
    pub next: menubitmap_s,
    pub mapname: menutext_s,
    pub item_null: menubitmap_s,
    pub multiplayer: qboolean,
    pub currentmap: i32,
    pub nummaps: i32,
    pub page: i32,
    pub maxpages: i32,
    pub maplist: [i32; 1024],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct serveroptions_t {
    pub menu: menuframework_s,
    pub banner: menutext_s,
    pub mappic: menubitmap_s,
    pub picframe: menubitmap_s,
    pub dedicated: menulist_s,
    pub timelimit: menufield_s,
    pub fraglimit: menufield_s,
    pub flaglimit: menufield_s,
    pub friendlyfire: menuradiobutton_s,
    pub hostname: menufield_s,
    pub pure_0: menuradiobutton_s,
    pub botSkill: menulist_s,
    pub player0: menutext_s,
    pub playerType: [menulist_s; 12],
    pub playerName: [menutext_s; 12],
    pub playerTeam: [menulist_s; 12],
    pub go: menubitmap_s,
    pub next: menubitmap_s,
    pub back: menubitmap_s,
    pub multiplayer: qboolean,
    pub gametype: i32,
    pub mapnamebuffer: [libc::c_char; 32],
    pub playerNameBuffers: [[libc::c_char; 16]; 12],
    pub newBot: qboolean,
    pub newBotIndex: i32,
    pub newBotName: [libc::c_char; 16],
    pub punkbuster: menulist_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct botSelectInfo_t {
    pub menu: menuframework_s,
    pub banner: menutext_s,
    pub pics: [menubitmap_s; 16],
    pub picbuttons: [menubitmap_s; 16],
    pub picnames: [menutext_s; 16],
    pub arrows: menubitmap_s,
    pub left: menubitmap_s,
    pub right: menubitmap_s,
    pub go: menubitmap_s,
    pub back: menubitmap_s,
    pub numBots: i32,
    pub modelpage: i32,
    pub numpages: i32,
    pub selectedmodel: i32,
    pub sortedBotNums: [i32; 1024],
    pub boticons: [[libc::c_char; 64]; 16],
    pub botnames: [[libc::c_char; 16]; 16],
}

static mut s_startserver: startserver_t = startserver_t {
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
            name: std::ptr::null(),
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
            name: std::ptr::null(),
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
            name: std::ptr::null(),
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
    gametype: menulist_s {
        generic: menucommon_s {
            type_0: 0,
            name: std::ptr::null(),
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
        oldvalue: 0,
        curvalue: 0,
        numitems: 0,
        top: 0,
        itemnames: std::ptr::null_mut(),
        width: 0,
        height: 0,
        columns: 0,
        separation: 0,
    },
    mappics: [menubitmap_s {
        generic: menucommon_s {
            type_0: 0,
            name: std::ptr::null(),
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
    }; 4],
    mapbuttons: [menubitmap_s {
        generic: menucommon_s {
            type_0: 0,
            name: std::ptr::null(),
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
    }; 4],
    arrows: menubitmap_s {
        generic: menucommon_s {
            type_0: 0,
            name: std::ptr::null(),
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
    prevpage: menubitmap_s {
        generic: menucommon_s {
            type_0: 0,
            name: std::ptr::null(),
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
    nextpage: menubitmap_s {
        generic: menucommon_s {
            type_0: 0,
            name: std::ptr::null(),
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
    back: menubitmap_s {
        generic: menucommon_s {
            type_0: 0,
            name: std::ptr::null(),
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
    next: menubitmap_s {
        generic: menucommon_s {
            type_0: 0,
            name: std::ptr::null(),
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
    mapname: menutext_s {
        generic: menucommon_s {
            type_0: 0,
            name: std::ptr::null(),
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
    item_null: menubitmap_s {
        generic: menucommon_s {
            type_0: 0,
            name: std::ptr::null(),
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
    multiplayer: qfalse,
    currentmap: 0,
    nummaps: 0,
    page: 0,
    maxpages: 0,
    maplist: [0; 1024],
};

static mut gametype_items: [*const libc::c_char; 5] = [
    b"Free For All\x00" as *const u8 as *const libc::c_char,
    b"Team Deathmatch\x00" as *const u8 as *const libc::c_char,
    b"Tournament\x00" as *const u8 as *const libc::c_char,
    b"Capture the Flag\x00" as *const u8 as *const libc::c_char,
    std::ptr::null(),
];

static mut gametype_remap: [i32; 4] = [
    GT_FFA as i32,
    GT_TEAM as i32,
    GT_TOURNAMENT as i32,
    GT_CTF as i32,
];

static mut gametype_remap2: [i32; 5] = [0 as i32, 2 as i32, 0 as i32, 1 as i32, 3 as i32];
/*
=================
GametypeBits
=================
*/

unsafe extern "C" fn GametypeBits(mut string: *mut libc::c_char) -> i32 {
    let mut bits: i32 = 0;
    let mut p: *mut libc::c_char = std::ptr::null_mut();
    let mut token: *mut libc::c_char = std::ptr::null_mut();
    bits = 0 as i32;
    p = string;
    loop {
        token = COM_ParseExt(&mut p, qfalse);
        if *token.offset(0 as i32 as isize) == 0 {
            break;
        }
        if Q_stricmp(token, b"ffa\x00" as *const u8 as *const libc::c_char) == 0 as i32 {
            bits |= (1 as i32) << GT_FFA as i32
        } else if Q_stricmp(token, b"tourney\x00" as *const u8 as *const libc::c_char) == 0 as i32 {
            bits |= (1 as i32) << GT_TOURNAMENT as i32
        } else if Q_stricmp(token, b"single\x00" as *const u8 as *const libc::c_char) == 0 as i32 {
            bits |= (1 as i32) << GT_SINGLE_PLAYER as i32
        } else if Q_stricmp(token, b"team\x00" as *const u8 as *const libc::c_char) == 0 as i32 {
            bits |= (1 as i32) << GT_TEAM as i32
        } else {
            if !(Q_stricmp(token, b"ctf\x00" as *const u8 as *const libc::c_char) == 0 as i32) {
                continue;
            }
            bits |= (1 as i32) << GT_CTF as i32
        }
    }
    return bits;
}
/*
=================
StartServer_Update
=================
*/

unsafe extern "C" fn StartServer_Update() {
    let mut i: i32 = 0;
    let mut top: i32 = 0;
    static mut picname: [[libc::c_char; 64]; 4] = [[0; 64]; 4];
    let mut info: *const libc::c_char = std::ptr::null();
    let mut mapname: [libc::c_char; 16] = [0; 16];
    top = s_startserver.page * 4 as i32;
    i = 0 as i32;
    while i < 4 as i32 {
        if top + i >= s_startserver.nummaps {
            break;
        }
        info = UI_GetArenaInfoByNumber(s_startserver.maplist[(top + i) as usize]);
        Q_strncpyz(
            mapname.as_mut_ptr(),
            Info_ValueForKey(info, b"map\x00" as *const u8 as *const libc::c_char),
            16 as i32,
        );
        Q_strupr(mapname.as_mut_ptr());
        Com_sprintf(
            picname[i as usize].as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 64]>() as usize as i32,
            b"levelshots/%s\x00" as *const u8 as *const libc::c_char,
            mapname.as_mut_ptr(),
        );
        s_startserver.mappics[i as usize].generic.flags &= !(0x40 as i32 as u32);
        s_startserver.mappics[i as usize].generic.name = picname[i as usize].as_mut_ptr();
        s_startserver.mappics[i as usize].shader = 0 as i32;
        // reset
        s_startserver.mapbuttons[i as usize].generic.flags |= 0x100 as i32 as u32;
        s_startserver.mapbuttons[i as usize].generic.flags &= !(0x4000 as i32 as u32);
        i += 1
    }
    while i < 4 as i32 {
        s_startserver.mappics[i as usize].generic.flags &= !(0x40 as i32 as u32);
        s_startserver.mappics[i as usize].generic.name = std::ptr::null();
        s_startserver.mappics[i as usize].shader = 0 as i32;
        // disable
        s_startserver.mapbuttons[i as usize].generic.flags &= !(0x100 as i32 as u32);
        s_startserver.mapbuttons[i as usize].generic.flags |= 0x4000 as i32 as u32;
        i += 1
    }
    // no servers to start
    if s_startserver.nummaps == 0 {
        s_startserver.next.generic.flags |= 0x4000 as i32 as u32;
        // set the map name
        libc::strcpy(
            s_startserver.mapname.string,
            b"NO MAPS FOUND\x00" as *const u8 as *const libc::c_char,
        );
    } else {
        // set the highlight
        s_startserver.next.generic.flags &= !(0x4000 as i32 as u32);
        i = s_startserver.currentmap - top;
        if i >= 0 as i32 && i < 4 as i32 {
            s_startserver.mappics[i as usize].generic.flags |= 0x40 as i32 as u32;
            s_startserver.mapbuttons[i as usize].generic.flags &= !(0x100 as i32 as u32)
        }
        // set the map name
        info = UI_GetArenaInfoByNumber(s_startserver.maplist[s_startserver.currentmap as usize]);
        Q_strncpyz(
            s_startserver.mapname.string,
            Info_ValueForKey(info, b"map\x00" as *const u8 as *const libc::c_char),
            16 as i32,
        );
    }
    Q_strupr(s_startserver.mapname.string);
}
/*
=================
StartServer_MapEvent
=================
*/

unsafe extern "C" fn StartServer_MapEvent(mut ptr: *mut libc::c_void, mut event: i32) {
    if event != 3 as i32 {
        return;
    }
    s_startserver.currentmap =
        s_startserver.page * 4 as i32 + ((*(ptr as *mut menucommon_s)).id - 11 as i32);
    StartServer_Update();
}
/*
=================
StartServer_GametypeEvent
=================
*/

unsafe extern "C" fn StartServer_GametypeEvent(mut _ptr: *mut libc::c_void, mut event: i32) {
    let mut i: i32 = 0;
    let mut count: i32 = 0;
    let mut gamebits: i32 = 0;
    let mut matchbits: i32 = 0;
    let mut info: *const libc::c_char = std::ptr::null();
    if event != 3 as i32 {
        return;
    }
    count = UI_GetNumArenas();
    s_startserver.nummaps = 0 as i32;
    matchbits = (1 as i32) << gametype_remap[s_startserver.gametype.curvalue as usize];
    if gametype_remap[s_startserver.gametype.curvalue as usize] == GT_FFA as i32 {
        matchbits |= (1 as i32) << GT_SINGLE_PLAYER as i32
    }
    i = 0 as i32;
    while i < count {
        info = UI_GetArenaInfoByNumber(i);
        gamebits = GametypeBits(Info_ValueForKey(
            info,
            b"type\x00" as *const u8 as *const libc::c_char,
        ));
        if !(gamebits & matchbits == 0) {
            s_startserver.maplist[s_startserver.nummaps as usize] = i;
            s_startserver.nummaps += 1
        }
        i += 1
    }
    s_startserver.maxpages = (s_startserver.nummaps + 4 as i32 - 1 as i32) / 4 as i32;
    s_startserver.page = 0 as i32;
    s_startserver.currentmap = 0 as i32;
    StartServer_Update();
}
/*
=================
StartServer_MenuEvent
=================
*/

unsafe extern "C" fn StartServer_MenuEvent(mut ptr: *mut libc::c_void, mut event: i32) {
    if event != 3 as i32 {
        return;
    }
    match (*(ptr as *mut menucommon_s)).id {
        15 => {
            if s_startserver.page > 0 as i32 {
                s_startserver.page -= 1;
                StartServer_Update();
            }
        }
        16 => {
            if s_startserver.page < s_startserver.maxpages - 1 as i32 {
                s_startserver.page += 1;
                StartServer_Update();
            }
        }
        18 => {
            trap_Cvar_SetValue(
                b"g_gameType\x00" as *const u8 as *const libc::c_char,
                gametype_remap[s_startserver.gametype.curvalue as usize] as f32,
            );
            UI_ServerOptionsMenu(s_startserver.multiplayer);
        }
        17 => {
            UI_PopMenu();
        }
        _ => {}
    };
}
/*
===============
StartServer_LevelshotDraw
===============
*/

unsafe extern "C" fn StartServer_LevelshotDraw(mut self_0: *mut libc::c_void) {
    let mut b: *mut menubitmap_s = std::ptr::null_mut();
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut w: i32 = 0;
    let mut h: i32 = 0;
    let mut n: i32 = 0;
    let mut info: *const libc::c_char = std::ptr::null();
    let mut mapname: [libc::c_char; 16] = [0; 16];
    b = self_0 as *mut menubitmap_s;
    if (*b).generic.name.is_null() {
        return;
    }
    if !(*b).generic.name.is_null() && (*b).shader == 0 {
        (*b).shader = trap_R_RegisterShaderNoMip((*b).generic.name);
        if (*b).shader == 0 && !(*b).errorpic.is_null() {
            (*b).shader = trap_R_RegisterShaderNoMip((*b).errorpic)
        }
    }
    if !(*b).focuspic.is_null() && (*b).focusshader == 0 {
        (*b).focusshader = trap_R_RegisterShaderNoMip((*b).focuspic)
    }
    x = (*b).generic.x;
    y = (*b).generic.y;
    w = (*b).width;
    h = (*b).height;
    if (*b).shader != 0 {
        UI_DrawHandlePic(x as f32, y as f32, w as f32, h as f32, (*b).shader);
    }
    x = (*b).generic.x;
    y = (*b).generic.y + (*b).height;
    UI_FillRect(
        x as f32,
        y as f32,
        (*b).width as f32,
        28 as i32 as f32,
        colorBlack.as_mut_ptr(),
    );
    x += (*b).width / 2 as i32;
    y += 4 as i32;
    n = s_startserver.page * 4 as i32 + (*b).generic.id - 11 as i32;
    info = UI_GetArenaInfoByNumber(s_startserver.maplist[n as usize]);
    Q_strncpyz(
        mapname.as_mut_ptr(),
        Info_ValueForKey(info, b"map\x00" as *const u8 as *const libc::c_char),
        16 as i32,
    );
    Q_strupr(mapname.as_mut_ptr());
    UI_DrawString(
        x,
        y,
        mapname.as_mut_ptr(),
        0x1 as i32 | 0x10 as i32,
        color_orange.as_mut_ptr(),
    );
    x = (*b).generic.x;
    y = (*b).generic.y;
    w = (*b).width;
    h = (*b).height + 28 as i32;
    if (*b).generic.flags & 0x40 as i32 as u32 != 0 {
        UI_DrawHandlePic(x as f32, y as f32, w as f32, h as f32, (*b).focusshader);
    };
}
/*
=================
StartServer_MenuInit
=================
*/

unsafe extern "C" fn StartServer_MenuInit() {
    let mut i: i32 = 0;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    static mut mapnamebuffer: [libc::c_char; 64] = [0; 64];
    // zero set all our globals
    crate::stdlib::memset(
        &mut s_startserver as *mut startserver_t as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<startserver_t>() as usize,
    );
    StartServer_Cache();
    s_startserver.menu.wrapAround = qtrue;
    s_startserver.menu.fullscreen = qtrue;
    s_startserver.banner.generic.type_0 = 10 as i32;
    s_startserver.banner.generic.x = 320 as i32;
    s_startserver.banner.generic.y = 16 as i32;
    s_startserver.banner.string =
        b"GAME SERVER\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_startserver.banner.color = color_white.as_mut_ptr();
    s_startserver.banner.style = 0x1 as i32;
    s_startserver.framel.generic.type_0 = 6 as i32;
    s_startserver.framel.generic.name =
        b"menu/art/frame2_l\x00" as *const u8 as *const libc::c_char;
    s_startserver.framel.generic.flags = 0x4000 as i32 as u32;
    s_startserver.framel.generic.x = 0 as i32;
    s_startserver.framel.generic.y = 78 as i32;
    s_startserver.framel.width = 256 as i32;
    s_startserver.framel.height = 329 as i32;
    s_startserver.framer.generic.type_0 = 6 as i32;
    s_startserver.framer.generic.name =
        b"menu/art/frame1_r\x00" as *const u8 as *const libc::c_char;
    s_startserver.framer.generic.flags = 0x4000 as i32 as u32;
    s_startserver.framer.generic.x = 376 as i32;
    s_startserver.framer.generic.y = 76 as i32;
    s_startserver.framer.width = 256 as i32;
    s_startserver.framer.height = 334 as i32;
    s_startserver.gametype.generic.type_0 = 3 as i32;
    s_startserver.gametype.generic.name = b"Game Type:\x00" as *const u8 as *const libc::c_char;
    s_startserver.gametype.generic.flags = 0x100 as i32 as u32 | 0x2 as i32 as u32;
    s_startserver.gametype.generic.callback =
        Some(StartServer_GametypeEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_startserver.gametype.generic.id = 10 as i32;
    s_startserver.gametype.generic.x = 320 as i32 - 24 as i32;
    s_startserver.gametype.generic.y = 368 as i32;
    s_startserver.gametype.itemnames = gametype_items.as_mut_ptr();
    i = 0 as i32;
    while i < 4 as i32 {
        x = i % 2 as i32 * (128 as i32 + 8 as i32) + 188 as i32;
        y = i / 2 as i32 * (128 as i32 + 8 as i32) + 96 as i32;
        s_startserver.mappics[i as usize].generic.type_0 = 6 as i32;
        s_startserver.mappics[i as usize].generic.flags = 0x4 as i32 as u32 | 0x4000 as i32 as u32;
        s_startserver.mappics[i as usize].generic.x = x;
        s_startserver.mappics[i as usize].generic.y = y;
        s_startserver.mappics[i as usize].generic.id = 11 as i32 + i;
        s_startserver.mappics[i as usize].width = 128 as i32;
        s_startserver.mappics[i as usize].height = 96 as i32;
        s_startserver.mappics[i as usize].focuspic =
            b"menu/art/maps_selected\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
        s_startserver.mappics[i as usize].errorpic =
            b"menu/art/unknownmap\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
        s_startserver.mappics[i as usize].generic.ownerdraw =
            Some(StartServer_LevelshotDraw as unsafe extern "C" fn(_: *mut libc::c_void) -> ());
        s_startserver.mapbuttons[i as usize].generic.type_0 = 6 as i32;
        s_startserver.mapbuttons[i as usize].generic.flags =
            0x4 as i32 as u32 | 0x100 as i32 as u32 | 0x8000 as i32 as u32;
        s_startserver.mapbuttons[i as usize].generic.id = 11 as i32 + i;
        s_startserver.mapbuttons[i as usize].generic.callback =
            Some(StartServer_MapEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
        s_startserver.mapbuttons[i as usize].generic.x = x - 30 as i32;
        s_startserver.mapbuttons[i as usize].generic.y = y - 32 as i32;
        s_startserver.mapbuttons[i as usize].width = 256 as i32;
        s_startserver.mapbuttons[i as usize].height = 248 as i32;
        s_startserver.mapbuttons[i as usize].generic.left = x;
        s_startserver.mapbuttons[i as usize].generic.top = y;
        s_startserver.mapbuttons[i as usize].generic.right = x + 128 as i32;
        s_startserver.mapbuttons[i as usize].generic.bottom = y + 128 as i32;
        s_startserver.mapbuttons[i as usize].focuspic =
            b"menu/art/maps_select\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
        i += 1
    }
    s_startserver.arrows.generic.type_0 = 6 as i32;
    s_startserver.arrows.generic.name =
        b"menu/art/gs_arrows_0\x00" as *const u8 as *const libc::c_char;
    s_startserver.arrows.generic.flags = 0x4000 as i32 as u32;
    s_startserver.arrows.generic.x = 260 as i32;
    s_startserver.arrows.generic.y = 400 as i32;
    s_startserver.arrows.width = 128 as i32;
    s_startserver.arrows.height = 32 as i32;
    s_startserver.prevpage.generic.type_0 = 6 as i32;
    s_startserver.prevpage.generic.flags = 0x4 as i32 as u32 | 0x100 as i32 as u32;
    s_startserver.prevpage.generic.callback =
        Some(StartServer_MenuEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_startserver.prevpage.generic.id = 15 as i32;
    s_startserver.prevpage.generic.x = 260 as i32;
    s_startserver.prevpage.generic.y = 400 as i32;
    s_startserver.prevpage.width = 64 as i32;
    s_startserver.prevpage.height = 32 as i32;
    s_startserver.prevpage.focuspic =
        b"menu/art/gs_arrows_l\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_startserver.nextpage.generic.type_0 = 6 as i32;
    s_startserver.nextpage.generic.flags = 0x4 as i32 as u32 | 0x100 as i32 as u32;
    s_startserver.nextpage.generic.callback =
        Some(StartServer_MenuEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_startserver.nextpage.generic.id = 16 as i32;
    s_startserver.nextpage.generic.x = 321 as i32;
    s_startserver.nextpage.generic.y = 400 as i32;
    s_startserver.nextpage.width = 64 as i32;
    s_startserver.nextpage.height = 32 as i32;
    s_startserver.nextpage.focuspic =
        b"menu/art/gs_arrows_r\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_startserver.mapname.generic.type_0 = 9 as i32;
    s_startserver.mapname.generic.flags = 0x8 as i32 as u32 | 0x4000 as i32 as u32;
    s_startserver.mapname.generic.x = 320 as i32;
    s_startserver.mapname.generic.y = 440 as i32;
    s_startserver.mapname.string = mapnamebuffer.as_mut_ptr();
    s_startserver.mapname.style = 0x1 as i32 | 0x20 as i32;
    s_startserver.mapname.color = text_color_normal.as_mut_ptr();
    s_startserver.back.generic.type_0 = 6 as i32;
    s_startserver.back.generic.name = b"menu/art/back_0\x00" as *const u8 as *const libc::c_char;
    s_startserver.back.generic.flags = 0x4 as i32 as u32 | 0x100 as i32 as u32;
    s_startserver.back.generic.callback =
        Some(StartServer_MenuEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_startserver.back.generic.id = 17 as i32;
    s_startserver.back.generic.x = 0 as i32;
    s_startserver.back.generic.y = 480 as i32 - 64 as i32;
    s_startserver.back.width = 128 as i32;
    s_startserver.back.height = 64 as i32;
    s_startserver.back.focuspic =
        b"menu/art/back_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_startserver.next.generic.type_0 = 6 as i32;
    s_startserver.next.generic.name = b"menu/art/next_0\x00" as *const u8 as *const libc::c_char;
    s_startserver.next.generic.flags = 0x10 as i32 as u32 | 0x100 as i32 as u32;
    s_startserver.next.generic.callback =
        Some(StartServer_MenuEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_startserver.next.generic.id = 18 as i32;
    s_startserver.next.generic.x = 640 as i32;
    s_startserver.next.generic.y = 480 as i32 - 64 as i32;
    s_startserver.next.width = 128 as i32;
    s_startserver.next.height = 64 as i32;
    s_startserver.next.focuspic =
        b"menu/art/next_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_startserver.item_null.generic.type_0 = 6 as i32;
    s_startserver.item_null.generic.flags =
        0x4 as i32 as u32 | 0x800 as i32 as u32 | 0x100000 as i32 as u32;
    s_startserver.item_null.generic.x = 0 as i32;
    s_startserver.item_null.generic.y = 0 as i32;
    s_startserver.item_null.width = 640 as i32;
    s_startserver.item_null.height = 480 as i32;
    Menu_AddItem(
        &mut s_startserver.menu as *mut _ as *mut _tag_menuframework,
        &mut s_startserver.banner as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_startserver.menu as *mut _ as *mut _tag_menuframework,
        &mut s_startserver.framel as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_startserver.menu as *mut _ as *mut _tag_menuframework,
        &mut s_startserver.framer as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_startserver.menu as *mut _ as *mut _tag_menuframework,
        &mut s_startserver.gametype as *mut menulist_s as *mut libc::c_void,
    );
    i = 0 as i32;
    while i < 4 as i32 {
        Menu_AddItem(
            &mut s_startserver.menu as *mut _ as *mut _tag_menuframework,
            &mut *s_startserver.mappics.as_mut_ptr().offset(i as isize) as *mut menubitmap_s
                as *mut libc::c_void,
        );
        Menu_AddItem(
            &mut s_startserver.menu as *mut _ as *mut _tag_menuframework,
            &mut *s_startserver.mapbuttons.as_mut_ptr().offset(i as isize) as *mut menubitmap_s
                as *mut libc::c_void,
        );
        i += 1
    }
    Menu_AddItem(
        &mut s_startserver.menu as *mut _ as *mut _tag_menuframework,
        &mut s_startserver.arrows as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_startserver.menu as *mut _ as *mut _tag_menuframework,
        &mut s_startserver.prevpage as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_startserver.menu as *mut _ as *mut _tag_menuframework,
        &mut s_startserver.nextpage as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_startserver.menu as *mut _ as *mut _tag_menuframework,
        &mut s_startserver.back as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_startserver.menu as *mut _ as *mut _tag_menuframework,
        &mut s_startserver.next as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_startserver.menu as *mut _ as *mut _tag_menuframework,
        &mut s_startserver.mapname as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_startserver.menu as *mut _ as *mut _tag_menuframework,
        &mut s_startserver.item_null as *mut menubitmap_s as *mut libc::c_void,
    );
    StartServer_GametypeEvent(std::ptr::null_mut(), 3 as i32);
}
/*
=================
StartServer_Cache
=================
*/
#[no_mangle]

pub unsafe extern "C" fn StartServer_Cache() {
    let mut i: i32 = 0;
    let mut info: *const libc::c_char = std::ptr::null();
    let mut precache: qboolean = qfalse;
    let mut picname: [libc::c_char; 64] = [0; 64];
    let mut mapname: [libc::c_char; 16] = [0; 16];
    trap_R_RegisterShaderNoMip(b"menu/art/back_0\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/back_1\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/next_0\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/next_1\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/frame2_l\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/frame1_r\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/maps_select\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/maps_selected\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/unknownmap\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/gs_arrows_0\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/gs_arrows_l\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/gs_arrows_r\x00" as *const u8 as *const libc::c_char);
    precache = trap_Cvar_VariableValue(b"com_buildscript\x00" as *const u8 as *const libc::c_char)
        as qboolean;
    if precache as u64 != 0 {
        i = 0 as i32;
        while i < UI_GetNumArenas() {
            info = UI_GetArenaInfoByNumber(i);
            Q_strncpyz(
                mapname.as_mut_ptr(),
                Info_ValueForKey(info, b"map\x00" as *const u8 as *const libc::c_char),
                16 as i32,
            );
            Q_strupr(mapname.as_mut_ptr());
            Com_sprintf(
                picname.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 64]>() as usize as i32,
                b"levelshots/%s\x00" as *const u8 as *const libc::c_char,
                mapname.as_mut_ptr(),
            );
            trap_R_RegisterShaderNoMip(picname.as_mut_ptr());
            i += 1
        }
    };
}
/*
=================
UI_StartServerMenu
=================
*/
#[no_mangle]

pub unsafe extern "C" fn UI_StartServerMenu(mut multiplayer: qboolean) {
    StartServer_MenuInit();
    s_startserver.multiplayer = multiplayer;
    UI_PushMenu(&mut s_startserver.menu as *mut _ as *mut _tag_menuframework);
}

static mut s_serveroptions: serveroptions_t = serveroptions_t {
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
            name: std::ptr::null(),
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
    mappic: menubitmap_s {
        generic: menucommon_s {
            type_0: 0,
            name: std::ptr::null(),
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
    picframe: menubitmap_s {
        generic: menucommon_s {
            type_0: 0,
            name: std::ptr::null(),
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
    dedicated: menulist_s {
        generic: menucommon_s {
            type_0: 0,
            name: std::ptr::null(),
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
        oldvalue: 0,
        curvalue: 0,
        numitems: 0,
        top: 0,
        itemnames: std::ptr::null_mut(),
        width: 0,
        height: 0,
        columns: 0,
        separation: 0,
    },
    timelimit: menufield_s {
        generic: menucommon_s {
            type_0: 0,
            name: std::ptr::null(),
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
        field: mfield_t {
            cursor: 0,
            scroll: 0,
            widthInChars: 0,
            buffer: [0; 256],
            maxchars: 0,
        },
    },
    fraglimit: menufield_s {
        generic: menucommon_s {
            type_0: 0,
            name: std::ptr::null(),
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
        field: mfield_t {
            cursor: 0,
            scroll: 0,
            widthInChars: 0,
            buffer: [0; 256],
            maxchars: 0,
        },
    },
    flaglimit: menufield_s {
        generic: menucommon_s {
            type_0: 0,
            name: std::ptr::null(),
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
        field: mfield_t {
            cursor: 0,
            scroll: 0,
            widthInChars: 0,
            buffer: [0; 256],
            maxchars: 0,
        },
    },
    friendlyfire: menuradiobutton_s {
        generic: menucommon_s {
            type_0: 0,
            name: std::ptr::null(),
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
        curvalue: 0,
    },
    hostname: menufield_s {
        generic: menucommon_s {
            type_0: 0,
            name: std::ptr::null(),
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
        field: mfield_t {
            cursor: 0,
            scroll: 0,
            widthInChars: 0,
            buffer: [0; 256],
            maxchars: 0,
        },
    },
    pure_0: menuradiobutton_s {
        generic: menucommon_s {
            type_0: 0,
            name: std::ptr::null(),
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
        curvalue: 0,
    },
    botSkill: menulist_s {
        generic: menucommon_s {
            type_0: 0,
            name: std::ptr::null(),
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
        oldvalue: 0,
        curvalue: 0,
        numitems: 0,
        top: 0,
        itemnames: std::ptr::null_mut(),
        width: 0,
        height: 0,
        columns: 0,
        separation: 0,
    },
    player0: menutext_s {
        generic: menucommon_s {
            type_0: 0,
            name: std::ptr::null(),
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
    playerType: [menulist_s {
        generic: menucommon_s {
            type_0: 0,
            name: std::ptr::null(),
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
        oldvalue: 0,
        curvalue: 0,
        numitems: 0,
        top: 0,
        itemnames: std::ptr::null_mut(),
        width: 0,
        height: 0,
        columns: 0,
        separation: 0,
    }; 12],
    playerName: [menutext_s {
        generic: menucommon_s {
            type_0: 0,
            name: std::ptr::null(),
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
    }; 12],
    playerTeam: [menulist_s {
        generic: menucommon_s {
            type_0: 0,
            name: std::ptr::null(),
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
        oldvalue: 0,
        curvalue: 0,
        numitems: 0,
        top: 0,
        itemnames: std::ptr::null_mut(),
        width: 0,
        height: 0,
        columns: 0,
        separation: 0,
    }; 12],
    go: menubitmap_s {
        generic: menucommon_s {
            type_0: 0,
            name: std::ptr::null(),
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
    next: menubitmap_s {
        generic: menucommon_s {
            type_0: 0,
            name: std::ptr::null(),
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
    back: menubitmap_s {
        generic: menucommon_s {
            type_0: 0,
            name: std::ptr::null(),
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
    multiplayer: qfalse,
    gametype: 0,
    mapnamebuffer: [0; 32],
    playerNameBuffers: [[0; 16]; 12],
    newBot: qfalse,
    newBotIndex: 0,
    newBotName: [0; 16],
    punkbuster: menulist_s {
        generic: menucommon_s {
            type_0: 0,
            name: std::ptr::null(),
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
        oldvalue: 0,
        curvalue: 0,
        numitems: 0,
        top: 0,
        itemnames: std::ptr::null_mut(),
        width: 0,
        height: 0,
        columns: 0,
        separation: 0,
    },
};

static mut dedicated_list: [*const libc::c_char; 4] = [
    b"No\x00" as *const u8 as *const libc::c_char,
    b"LAN\x00" as *const u8 as *const libc::c_char,
    b"Internet\x00" as *const u8 as *const libc::c_char,
    std::ptr::null(),
];

static mut playerType_list: [*const libc::c_char; 4] = [
    b"Open\x00" as *const u8 as *const libc::c_char,
    b"Bot\x00" as *const u8 as *const libc::c_char,
    b"----\x00" as *const u8 as *const libc::c_char,
    std::ptr::null(),
];

static mut playerTeam_list: [*const libc::c_char; 3] = [
    b"Blue\x00" as *const u8 as *const libc::c_char,
    b"Red\x00" as *const u8 as *const libc::c_char,
    std::ptr::null(),
];

static mut botSkill_list: [*const libc::c_char; 6] = [
    b"I Can Win\x00" as *const u8 as *const libc::c_char,
    b"Bring It On\x00" as *const u8 as *const libc::c_char,
    b"Hurt Me Plenty\x00" as *const u8 as *const libc::c_char,
    b"Hardcore\x00" as *const u8 as *const libc::c_char,
    b"Nightmare!\x00" as *const u8 as *const libc::c_char,
    std::ptr::null(),
];
/*
=================
BotAlreadySelected
=================
*/

unsafe extern "C" fn BotAlreadySelected(mut checkName: *const libc::c_char) -> qboolean {
    let mut n: i32 = 0;
    n = 1 as i32;
    while n < 12 as i32 {
        if !(s_serveroptions.playerType[n as usize].curvalue != 1 as i32) {
            if !(s_serveroptions.gametype >= GT_TEAM as i32
                && s_serveroptions.playerTeam[n as usize].curvalue
                    != s_serveroptions.playerTeam[s_serveroptions.newBotIndex as usize].curvalue)
            {
                if Q_stricmp(
                    checkName,
                    s_serveroptions.playerNameBuffers[n as usize].as_mut_ptr(),
                ) == 0 as i32
                {
                    return qtrue;
                }
            }
        }
        n += 1
    }
    return qfalse;
}
/*
=================
ServerOptions_Start
=================
*/

unsafe extern "C" fn ServerOptions_Start() {
    let mut timelimit: i32 = 0;
    let mut fraglimit: i32 = 0;
    let mut maxclients: i32 = 0;
    let mut dedicated: i32 = 0;
    let mut friendlyfire: i32 = 0;
    let mut flaglimit: i32 = 0;
    let mut pure_0: i32 = 0;
    let mut skill: i32 = 0;
    let mut n: i32 = 0;
    let mut buf: [libc::c_char; 64] = [0; 64];
    let mut info: *const libc::c_char = std::ptr::null();
    timelimit = atoi(s_serveroptions.timelimit.field.buffer.as_mut_ptr());
    fraglimit = atoi(s_serveroptions.fraglimit.field.buffer.as_mut_ptr());
    flaglimit = atoi(s_serveroptions.flaglimit.field.buffer.as_mut_ptr());
    dedicated = s_serveroptions.dedicated.curvalue;
    friendlyfire = s_serveroptions.friendlyfire.curvalue;
    pure_0 = s_serveroptions.pure_0.curvalue;
    skill = s_serveroptions.botSkill.curvalue + 1 as i32;
    //set maxclients
    n = 0 as i32;
    maxclients = 0 as i32;
    while n < 12 as i32 {
        if !(s_serveroptions.playerType[n as usize].curvalue == 2 as i32) {
            if !(s_serveroptions.playerType[n as usize].curvalue == 1 as i32
                && s_serveroptions.playerNameBuffers[n as usize][0 as i32 as usize] as i32
                    == 0 as i32)
            {
                maxclients += 1
            }
        }
        n += 1
    }
    match s_serveroptions.gametype {
        1 => {
            trap_Cvar_SetValue(
                b"ui_tourney_fraglimit\x00" as *const u8 as *const libc::c_char,
                fraglimit as f32,
            );
            trap_Cvar_SetValue(
                b"ui_tourney_timelimit\x00" as *const u8 as *const libc::c_char,
                timelimit as f32,
            );
        }
        3 => {
            trap_Cvar_SetValue(
                b"ui_team_fraglimit\x00" as *const u8 as *const libc::c_char,
                fraglimit as f32,
            );
            trap_Cvar_SetValue(
                b"ui_team_timelimit\x00" as *const u8 as *const libc::c_char,
                timelimit as f32,
            );
            trap_Cvar_SetValue(
                b"ui_team_friendly\x00" as *const u8 as *const libc::c_char,
                friendlyfire as f32,
            );
        }
        4 => {
            trap_Cvar_SetValue(
                b"ui_ctf_capturelimit\x00" as *const u8 as *const libc::c_char,
                flaglimit as f32,
            );
            trap_Cvar_SetValue(
                b"ui_ctf_timelimit\x00" as *const u8 as *const libc::c_char,
                timelimit as f32,
            );
            trap_Cvar_SetValue(
                b"ui_ctf_friendly\x00" as *const u8 as *const libc::c_char,
                friendlyfire as f32,
            );
        }
        0 | _ => {
            trap_Cvar_SetValue(
                b"ui_ffa_fraglimit\x00" as *const u8 as *const libc::c_char,
                fraglimit as f32,
            );
            trap_Cvar_SetValue(
                b"ui_ffa_timelimit\x00" as *const u8 as *const libc::c_char,
                timelimit as f32,
            );
        }
    }
    trap_Cvar_SetValue(
        b"sv_maxclients\x00" as *const u8 as *const libc::c_char,
        Com_Clamp(0 as i32 as f32, 12 as i32 as f32, maxclients as f32),
    );
    trap_Cvar_SetValue(
        b"dedicated\x00" as *const u8 as *const libc::c_char,
        Com_Clamp(0 as i32 as f32, 2 as i32 as f32, dedicated as f32),
    );
    trap_Cvar_SetValue(
        b"timelimit\x00" as *const u8 as *const libc::c_char,
        Com_Clamp(0 as i32 as f32, timelimit as f32, timelimit as f32),
    );
    trap_Cvar_SetValue(
        b"fraglimit\x00" as *const u8 as *const libc::c_char,
        Com_Clamp(0 as i32 as f32, fraglimit as f32, fraglimit as f32),
    );
    trap_Cvar_SetValue(
        b"capturelimit\x00" as *const u8 as *const libc::c_char,
        Com_Clamp(0 as i32 as f32, flaglimit as f32, flaglimit as f32),
    );
    trap_Cvar_SetValue(
        b"g_friendlyfire\x00" as *const u8 as *const libc::c_char,
        friendlyfire as f32,
    );
    trap_Cvar_SetValue(
        b"sv_pure\x00" as *const u8 as *const libc::c_char,
        pure_0 as f32,
    );
    trap_Cvar_Set(
        b"sv_hostname\x00" as *const u8 as *const libc::c_char,
        s_serveroptions.hostname.field.buffer.as_mut_ptr(),
    );
    trap_Cvar_SetValue(
        b"sv_punkbuster\x00" as *const u8 as *const libc::c_char,
        s_serveroptions.punkbuster.curvalue as f32,
    );
    // the wait commands will allow the dedicated to take effect
    info = UI_GetArenaInfoByNumber(s_startserver.maplist[s_startserver.currentmap as usize]);
    trap_Cmd_ExecuteText(
        EXEC_APPEND as i32,
        va(
            b"wait ; wait ; map %s\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            Info_ValueForKey(info, b"map\x00" as *const u8 as *const libc::c_char),
        ),
    );
    // add bots
    trap_Cmd_ExecuteText(
        EXEC_APPEND as i32,
        b"wait 3\n\x00" as *const u8 as *const libc::c_char,
    );
    n = 1 as i32;
    while n < 12 as i32 {
        if !(s_serveroptions.playerType[n as usize].curvalue != 1 as i32) {
            if !(s_serveroptions.playerNameBuffers[n as usize][0 as i32 as usize] as i32
                == 0 as i32)
            {
                if !(s_serveroptions.playerNameBuffers[n as usize][0 as i32 as usize] as i32
                    == '-' as i32)
                {
                    if s_serveroptions.gametype >= GT_TEAM as i32 {
                        Com_sprintf(
                            buf.as_mut_ptr(),
                            ::std::mem::size_of::<[libc::c_char; 64]>() as usize as i32,
                            b"addbot %s %i %s\n\x00" as *const u8 as *const libc::c_char,
                            s_serveroptions.playerNameBuffers[n as usize].as_mut_ptr(),
                            skill,
                            playerTeam_list
                                [s_serveroptions.playerTeam[n as usize].curvalue as usize],
                        );
                    } else {
                        Com_sprintf(
                            buf.as_mut_ptr(),
                            ::std::mem::size_of::<[libc::c_char; 64]>() as usize as i32,
                            b"addbot %s %i\n\x00" as *const u8 as *const libc::c_char,
                            s_serveroptions.playerNameBuffers[n as usize].as_mut_ptr(),
                            skill,
                        );
                    }
                    trap_Cmd_ExecuteText(EXEC_APPEND as i32, buf.as_mut_ptr());
                }
            }
        }
        n += 1
    }
    // set player's team
    if dedicated == 0 as i32 && s_serveroptions.gametype >= GT_TEAM as i32 {
        // send team command for vanilla q3 game qvm
        trap_Cmd_ExecuteText(
            EXEC_APPEND as i32,
            va(
                b"wait 5; team %s\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                playerTeam_list[s_serveroptions.playerTeam[0 as i32 as usize].curvalue as usize],
            ),
        );
        // set g_localTeamPref for ioq3 game qvm
        trap_Cvar_Set(
            b"g_localTeamPref\x00" as *const u8 as *const libc::c_char,
            playerTeam_list[s_serveroptions.playerTeam[0 as i32 as usize].curvalue as usize],
        );
    };
}
/*
=================
ServerOptions_InitPlayerItems
=================
*/

unsafe extern "C" fn ServerOptions_InitPlayerItems() {
    let mut n: i32 = 0;
    let mut v: i32 = 0;
    // init types
    if s_serveroptions.multiplayer as u64 != 0 {
        v = 0 as i32
    // open
    } else {
        v = 1 as i32
        // bot
    }
    n = 0 as i32;
    while n < 12 as i32 {
        s_serveroptions.playerType[n as usize].curvalue = v;
        n += 1
    }
    if s_serveroptions.multiplayer as u32 != 0 && s_serveroptions.gametype < GT_TEAM as i32 {
        n = 8 as i32;
        while n < 12 as i32 {
            s_serveroptions.playerType[n as usize].curvalue = 2 as i32;
            n += 1
        }
    }
    // if not a dedicated server, first slot is reserved for the human on the server
    if s_serveroptions.dedicated.curvalue == 0 as i32 {
        // human
        s_serveroptions.playerType[0 as i32 as usize].generic.flags |= 0x4000 as i32 as u32;
        s_serveroptions.playerType[0 as i32 as usize].curvalue = 0 as i32;
        trap_Cvar_VariableStringBuffer(
            b"name\x00" as *const u8 as *const libc::c_char,
            s_serveroptions.playerNameBuffers[0 as i32 as usize].as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 16]>() as usize as i32,
        );
        Q_CleanStr(s_serveroptions.playerNameBuffers[0 as i32 as usize].as_mut_ptr());
    }
    // init teams
    if s_serveroptions.gametype >= GT_TEAM as i32 {
        n = 0 as i32;
        while n < 12 as i32 / 2 as i32 {
            s_serveroptions.playerTeam[n as usize].curvalue = 0 as i32;
            n += 1
        }
        while n < 12 as i32 {
            s_serveroptions.playerTeam[n as usize].curvalue = 1 as i32;
            n += 1
        }
    } else {
        n = 0 as i32;
        while n < 12 as i32 {
            s_serveroptions.playerTeam[n as usize].generic.flags |=
                0x4000 as i32 as u32 | 0x1000 as i32 as u32;
            n += 1
        }
    };
}
/*
=================
ServerOptions_SetPlayerItems
=================
*/

unsafe extern "C" fn ServerOptions_SetPlayerItems() {
    let mut start: i32 = 0;
    let mut n: i32 = 0;
    // types
    //	for( n = 0; n < PLAYER_SLOTS; n++ ) {
    //		if( (!s_serveroptions.multiplayer) && (n > 0) && (s_serveroptions.playerType[n].curvalue == 0) ) {
    //			s_serveroptions.playerType[n].curvalue = 1;
    //		}
    //	}
    // names
    if s_serveroptions.dedicated.curvalue == 0 as i32 {
        s_serveroptions.player0.string =
            b"Human\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
        s_serveroptions.playerName[0 as i32 as usize].generic.flags &= !(0x1000 as i32 as u32);
        start = 1 as i32
    } else {
        s_serveroptions.player0.string =
            b"Open\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
        start = 0 as i32
    }
    n = start;
    while n < 12 as i32 {
        if s_serveroptions.playerType[n as usize].curvalue == 1 as i32 {
            s_serveroptions.playerName[n as usize].generic.flags &=
                !(0x4000 as i32 as u32 | 0x1000 as i32 as u32)
        } else {
            s_serveroptions.playerName[n as usize].generic.flags |=
                0x4000 as i32 as u32 | 0x1000 as i32 as u32
        }
        n += 1
    }
    // teams
    if s_serveroptions.gametype < GT_TEAM as i32 {
        return;
    }
    n = start;
    while n < 12 as i32 {
        if s_serveroptions.playerType[n as usize].curvalue == 2 as i32 {
            s_serveroptions.playerTeam[n as usize].generic.flags |=
                0x4000 as i32 as u32 | 0x1000 as i32 as u32
        } else {
            s_serveroptions.playerTeam[n as usize].generic.flags &=
                !(0x4000 as i32 as u32 | 0x1000 as i32 as u32)
        }
        n += 1
    }
}
/*
=================
ServerOptions_Event
=================
*/

unsafe extern "C" fn ServerOptions_Event(mut ptr: *mut libc::c_void, mut event: i32) {
    match (*(ptr as *mut menucommon_s)).id {
        20 => {
            //if( event != QM_ACTIVATED && event != QM_LOSTFOCUS) {
            //	return;
            //}
            if !(event != 3 as i32) {
                ServerOptions_SetPlayerItems();
            }
        }
        21 | 22 => {
            ServerOptions_SetPlayerItems();
        }
        23 => {
            if !(event != 3 as i32) {
                ServerOptions_Start();
            }
        }
        18 => {
            // empty case
        }
        24 => {
            if !(event != 3 as i32) {
                UI_PopMenu();
            }
        }
        _ => {}
    };
}

unsafe extern "C" fn ServerOptions_PlayerNameEvent(mut ptr: *mut libc::c_void, mut event: i32) {
    let mut n: i32 = 0;
    if event != 3 as i32 {
        return;
    }
    n = (*(ptr as *mut menutext_s)).generic.id;
    s_serveroptions.newBotIndex = n;
    UI_BotSelectMenu(s_serveroptions.playerNameBuffers[n as usize].as_mut_ptr());
}
/*
=================
ServerOptions_StatusBar
=================
*/

unsafe extern "C" fn ServerOptions_StatusBar(mut ptr: *mut libc::c_void) {
    match (*(ptr as *mut menucommon_s)).id {
        _ => {}
    }
    UI_DrawString(
        320 as i32,
        440 as i32,
        b"0 = NO LIMIT\x00" as *const u8 as *const libc::c_char,
        0x1 as i32 | 0x10 as i32,
        colorWhite.as_mut_ptr(),
    );
}
/*
===============
ServerOptions_LevelshotDraw
===============
*/

unsafe extern "C" fn ServerOptions_LevelshotDraw(mut self_0: *mut libc::c_void) {
    let mut b: *mut menubitmap_s = std::ptr::null_mut();
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    // strange place for this, but it works
    if s_serveroptions.newBot as u64 != 0 {
        Q_strncpyz(
            s_serveroptions.playerNameBuffers[s_serveroptions.newBotIndex as usize].as_mut_ptr(),
            s_serveroptions.newBotName.as_mut_ptr(),
            16 as i32,
        ); // skip the first slot, reserved for a human
        s_serveroptions.newBot = qfalse
    }
    b = self_0 as *mut menubitmap_s;
    Bitmap_Draw(b as *mut menubitmap_s);
    x = (*b).generic.x;
    y = (*b).generic.y + (*b).height;
    UI_FillRect(
        x as f32,
        y as f32,
        (*b).width as f32,
        40 as i32 as f32,
        colorBlack.as_mut_ptr(),
    );
    x += (*b).width / 2 as i32;
    y += 4 as i32;
    UI_DrawString(
        x,
        y,
        s_serveroptions.mapnamebuffer.as_mut_ptr(),
        0x1 as i32 | 0x10 as i32,
        color_orange.as_mut_ptr(),
    );
    y += 16 as i32;
    UI_DrawString(
        x,
        y,
        gametype_items[gametype_remap2[s_serveroptions.gametype as usize] as usize],
        0x1 as i32 | 0x10 as i32,
        color_orange.as_mut_ptr(),
    );
}

unsafe extern "C" fn ServerOptions_InitBotNames() {
    let mut count: i32 = 0;
    let mut n: i32 = 0;
    let mut arenaInfo: *const libc::c_char = std::ptr::null();
    let mut botInfo: *const libc::c_char = std::ptr::null();
    let mut p: *mut libc::c_char = std::ptr::null_mut();
    let mut bot: *mut libc::c_char = std::ptr::null_mut();
    let mut bots: [libc::c_char; 1024] = [0; 1024];
    if s_serveroptions.gametype >= GT_TEAM as i32 {
        Q_strncpyz(
            s_serveroptions.playerNameBuffers[1 as i32 as usize].as_mut_ptr(),
            b"grunt\x00" as *const u8 as *const libc::c_char,
            16 as i32,
        );
        Q_strncpyz(
            s_serveroptions.playerNameBuffers[2 as i32 as usize].as_mut_ptr(),
            b"major\x00" as *const u8 as *const libc::c_char,
            16 as i32,
        );
        if s_serveroptions.gametype == GT_TEAM as i32 {
            Q_strncpyz(
                s_serveroptions.playerNameBuffers[3 as i32 as usize].as_mut_ptr(),
                b"visor\x00" as *const u8 as *const libc::c_char,
                16 as i32,
            );
        } else {
            s_serveroptions.playerType[3 as i32 as usize].curvalue = 2 as i32
        }
        s_serveroptions.playerType[4 as i32 as usize].curvalue = 2 as i32;
        s_serveroptions.playerType[5 as i32 as usize].curvalue = 2 as i32;
        Q_strncpyz(
            s_serveroptions.playerNameBuffers[6 as i32 as usize].as_mut_ptr(),
            b"sarge\x00" as *const u8 as *const libc::c_char,
            16 as i32,
        );
        Q_strncpyz(
            s_serveroptions.playerNameBuffers[7 as i32 as usize].as_mut_ptr(),
            b"grunt\x00" as *const u8 as *const libc::c_char,
            16 as i32,
        );
        Q_strncpyz(
            s_serveroptions.playerNameBuffers[8 as i32 as usize].as_mut_ptr(),
            b"major\x00" as *const u8 as *const libc::c_char,
            16 as i32,
        );
        if s_serveroptions.gametype == GT_TEAM as i32 {
            Q_strncpyz(
                s_serveroptions.playerNameBuffers[9 as i32 as usize].as_mut_ptr(),
                b"visor\x00" as *const u8 as *const libc::c_char,
                16 as i32,
            );
        } else {
            s_serveroptions.playerType[9 as i32 as usize].curvalue = 2 as i32
        }
        s_serveroptions.playerType[10 as i32 as usize].curvalue = 2 as i32;
        s_serveroptions.playerType[11 as i32 as usize].curvalue = 2 as i32;
        return;
    }
    count = 1 as i32;
    // get info for this map
    arenaInfo = UI_GetArenaInfoByMap(s_serveroptions.mapnamebuffer.as_mut_ptr());
    // get the bot info - we'll seed with them if any are listed
    Q_strncpyz(
        bots.as_mut_ptr(),
        Info_ValueForKey(arenaInfo, b"bots\x00" as *const u8 as *const libc::c_char),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as usize as i32,
    );
    p = &mut *bots.as_mut_ptr().offset(0 as i32 as isize) as *mut libc::c_char;
    while *p as i32 != 0 && count < 12 as i32 {
        //skip spaces
        while *p as i32 != 0 && *p as i32 == ' ' as i32 {
            p = p.offset(1)
        }
        if *p == 0 {
            break;
        }
        // mark start of bot name
        bot = p;
        // skip until space of null
        while *p as i32 != 0 && *p as i32 != ' ' as i32 {
            p = p.offset(1)
        }
        if *p != 0 {
            let fresh0 = p;
            p = p.offset(1);
            *fresh0 = 0 as i32 as libc::c_char
        }
        botInfo = UI_GetBotInfoByName(bot);
        if botInfo.is_null() {
            botInfo = UI_GetBotInfoByNumber(count)
        }
        bot = Info_ValueForKey(botInfo, b"name\x00" as *const u8 as *const libc::c_char);
        Q_strncpyz(
            s_serveroptions.playerNameBuffers[count as usize].as_mut_ptr(),
            bot,
            ::std::mem::size_of::<[libc::c_char; 16]>() as usize as i32,
        );
        count += 1
    }
    // set the rest of the bot slots to "---"
    n = count;
    while n < 12 as i32 {
        libc::strcpy(
            s_serveroptions.playerNameBuffers[n as usize].as_mut_ptr(),
            b"--------\x00" as *const u8 as *const libc::c_char,
        );
        n += 1
    }
    // pad up to #8 as open slots
    while count < 8 as i32 {
        s_serveroptions.playerType[count as usize].curvalue = 0 as i32;
        count += 1
    }
    // close off the rest by default
    while count < 12 as i32 {
        if s_serveroptions.playerType[count as usize].curvalue == 1 as i32 {
            s_serveroptions.playerType[count as usize].curvalue = 2 as i32
        }
        count += 1
    }
}
/*
=================
ServerOptions_SetMenuItems
=================
*/

unsafe extern "C" fn ServerOptions_SetMenuItems() {
    static mut picname: [libc::c_char; 64] = [0; 64];
    let mut mapname: [libc::c_char; 16] = [0; 16];
    let mut info: *const libc::c_char = std::ptr::null();
    match s_serveroptions.gametype {
        1 => {
            Com_sprintf(
                s_serveroptions.fraglimit.field.buffer.as_mut_ptr(),
                4 as i32,
                b"%i\x00" as *const u8 as *const libc::c_char,
                Com_Clamp(
                    0 as i32 as f32,
                    999 as i32 as f32,
                    trap_Cvar_VariableValue(
                        b"ui_tourney_fraglimit\x00" as *const u8 as *const libc::c_char,
                    ),
                ) as i32,
            );
            Com_sprintf(
                s_serveroptions.timelimit.field.buffer.as_mut_ptr(),
                4 as i32,
                b"%i\x00" as *const u8 as *const libc::c_char,
                Com_Clamp(
                    0 as i32 as f32,
                    999 as i32 as f32,
                    trap_Cvar_VariableValue(
                        b"ui_tourney_timelimit\x00" as *const u8 as *const libc::c_char,
                    ),
                ) as i32,
            );
        }
        3 => {
            Com_sprintf(
                s_serveroptions.fraglimit.field.buffer.as_mut_ptr(),
                4 as i32,
                b"%i\x00" as *const u8 as *const libc::c_char,
                Com_Clamp(
                    0 as i32 as f32,
                    999 as i32 as f32,
                    trap_Cvar_VariableValue(
                        b"ui_team_fraglimit\x00" as *const u8 as *const libc::c_char,
                    ),
                ) as i32,
            );
            Com_sprintf(
                s_serveroptions.timelimit.field.buffer.as_mut_ptr(),
                4 as i32,
                b"%i\x00" as *const u8 as *const libc::c_char,
                Com_Clamp(
                    0 as i32 as f32,
                    999 as i32 as f32,
                    trap_Cvar_VariableValue(
                        b"ui_team_timelimit\x00" as *const u8 as *const libc::c_char,
                    ),
                ) as i32,
            );
            s_serveroptions.friendlyfire.curvalue = Com_Clamp(
                0 as i32 as f32,
                1 as i32 as f32,
                trap_Cvar_VariableValue(
                    b"ui_team_friendly\x00" as *const u8 as *const libc::c_char,
                ),
            ) as i32
        }
        4 => {
            Com_sprintf(
                s_serveroptions.flaglimit.field.buffer.as_mut_ptr(),
                4 as i32,
                b"%i\x00" as *const u8 as *const libc::c_char,
                Com_Clamp(
                    0 as i32 as f32,
                    100 as i32 as f32,
                    trap_Cvar_VariableValue(
                        b"ui_ctf_capturelimit\x00" as *const u8 as *const libc::c_char,
                    ),
                ) as i32,
            );
            Com_sprintf(
                s_serveroptions.timelimit.field.buffer.as_mut_ptr(),
                4 as i32,
                b"%i\x00" as *const u8 as *const libc::c_char,
                Com_Clamp(
                    0 as i32 as f32,
                    999 as i32 as f32,
                    trap_Cvar_VariableValue(
                        b"ui_ctf_timelimit\x00" as *const u8 as *const libc::c_char,
                    ),
                ) as i32,
            );
            s_serveroptions.friendlyfire.curvalue = Com_Clamp(
                0 as i32 as f32,
                1 as i32 as f32,
                trap_Cvar_VariableValue(b"ui_ctf_friendly\x00" as *const u8 as *const libc::c_char),
            ) as i32
        }
        0 | _ => {
            Com_sprintf(
                s_serveroptions.fraglimit.field.buffer.as_mut_ptr(),
                4 as i32,
                b"%i\x00" as *const u8 as *const libc::c_char,
                Com_Clamp(
                    0 as i32 as f32,
                    999 as i32 as f32,
                    trap_Cvar_VariableValue(
                        b"ui_ffa_fraglimit\x00" as *const u8 as *const libc::c_char,
                    ),
                ) as i32,
            );
            Com_sprintf(
                s_serveroptions.timelimit.field.buffer.as_mut_ptr(),
                4 as i32,
                b"%i\x00" as *const u8 as *const libc::c_char,
                Com_Clamp(
                    0 as i32 as f32,
                    999 as i32 as f32,
                    trap_Cvar_VariableValue(
                        b"ui_ffa_timelimit\x00" as *const u8 as *const libc::c_char,
                    ),
                ) as i32,
            );
        }
    }
    Q_strncpyz(
        s_serveroptions.hostname.field.buffer.as_mut_ptr(),
        UI_Cvar_VariableString(b"sv_hostname\x00" as *const u8 as *const libc::c_char),
        ::std::mem::size_of::<[libc::c_char; 256]>() as usize as i32,
    );
    s_serveroptions.pure_0.curvalue = Com_Clamp(
        0 as i32 as f32,
        1 as i32 as f32,
        trap_Cvar_VariableValue(b"sv_pure\x00" as *const u8 as *const libc::c_char),
    ) as i32;
    // set the map pic
    info = UI_GetArenaInfoByNumber(s_startserver.maplist[s_startserver.currentmap as usize]);
    Q_strncpyz(
        mapname.as_mut_ptr(),
        Info_ValueForKey(info, b"map\x00" as *const u8 as *const libc::c_char),
        16 as i32,
    );
    Q_strupr(mapname.as_mut_ptr());
    Com_sprintf(
        picname.as_mut_ptr(),
        64 as i32,
        b"levelshots/%s\x00" as *const u8 as *const libc::c_char,
        mapname.as_mut_ptr(),
    );
    s_serveroptions.mappic.generic.name = picname.as_mut_ptr();
    // set the map name
    libc::strcpy(
        s_serveroptions.mapnamebuffer.as_mut_ptr(),
        s_startserver.mapname.string,
    );
    Q_strupr(s_serveroptions.mapnamebuffer.as_mut_ptr());
    // get the player selections initialized
    ServerOptions_InitPlayerItems();
    ServerOptions_SetPlayerItems();
    // seed bot names
    ServerOptions_InitBotNames();
    ServerOptions_SetPlayerItems();
}
/*
=================
PlayerName_Draw
=================
*/

unsafe extern "C" fn PlayerName_Draw(mut item: *mut libc::c_void) {
    let mut s: *mut menutext_s = std::ptr::null_mut();
    let mut color: *mut f32 = std::ptr::null_mut();
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut style: i32 = 0;
    let mut focus: qboolean = qfalse;
    s = item as *mut menutext_s;
    x = (*s).generic.x;
    y = (*s).generic.y;
    style = 0x10 as i32;
    focus = ((*(*s).generic.parent).cursor == (*s).generic.menuPosition) as i32 as qboolean;
    if (*s).generic.flags & 0x2000 as i32 as u32 != 0 {
        color = text_color_disabled.as_mut_ptr()
    } else if focus as u64 != 0 {
        color = text_color_highlight.as_mut_ptr();
        style |= 0x4000 as i32
    } else if (*s).generic.flags & 0x1 as i32 as u32 != 0 {
        color = text_color_highlight.as_mut_ptr();
        style |= 0x1000 as i32
    } else {
        color = text_color_normal.as_mut_ptr()
    }
    if focus as u64 != 0 {
        // draw cursor
        UI_FillRect(
            (*s).generic.left as f32,
            (*s).generic.top as f32,
            ((*s).generic.right - (*s).generic.left + 1 as i32) as f32,
            ((*s).generic.bottom - (*s).generic.top + 1 as i32) as f32,
            listbar_color.as_mut_ptr(),
        );
        UI_DrawChar(
            x,
            y,
            13 as i32,
            0x1 as i32 | 0x1000 as i32 | 0x10 as i32,
            color,
        );
    }
    UI_DrawString(
        x - 8 as i32,
        y,
        (*s).generic.name,
        style | 0x2 as i32,
        color,
    );
    UI_DrawString(x + 8 as i32, y, (*s).string, style | 0 as i32, color);
}

unsafe extern "C" fn ServerOptions_MenuInit(mut multiplayer: qboolean) {
    let mut y: i32 = 0;
    let mut n: i32 = 0;
    crate::stdlib::memset(
        &mut s_serveroptions as *mut serveroptions_t as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<serveroptions_t>() as usize,
    );
    s_serveroptions.multiplayer = multiplayer;
    s_serveroptions.gametype = Com_Clamp(
        0 as i32 as f32,
        (::std::mem::size_of::<[i32; 5]>() as usize)
            .wrapping_div(::std::mem::size_of::<i32>() as usize)
            .wrapping_sub(1 as i32 as usize) as f32,
        trap_Cvar_VariableValue(b"g_gametype\x00" as *const u8 as *const libc::c_char),
    ) as i32;
    s_serveroptions.punkbuster.curvalue = Com_Clamp(
        0 as i32 as f32,
        1 as i32 as f32,
        trap_Cvar_VariableValue(b"sv_punkbuster\x00" as *const u8 as *const libc::c_char),
    ) as i32;
    ServerOptions_Cache();
    s_serveroptions.menu.wrapAround = qtrue;
    s_serveroptions.menu.fullscreen = qtrue;
    s_serveroptions.banner.generic.type_0 = 10 as i32;
    s_serveroptions.banner.generic.x = 320 as i32;
    s_serveroptions.banner.generic.y = 16 as i32;
    s_serveroptions.banner.string =
        b"GAME SERVER\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_serveroptions.banner.color = color_white.as_mut_ptr();
    s_serveroptions.banner.style = 0x1 as i32;
    s_serveroptions.mappic.generic.type_0 = 6 as i32;
    s_serveroptions.mappic.generic.flags = 0x4 as i32 as u32 | 0x4000 as i32 as u32;
    s_serveroptions.mappic.generic.x = 352 as i32;
    s_serveroptions.mappic.generic.y = 80 as i32;
    s_serveroptions.mappic.width = 160 as i32;
    s_serveroptions.mappic.height = 120 as i32;
    s_serveroptions.mappic.errorpic =
        b"menu/art/unknownmap\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_serveroptions.mappic.generic.ownerdraw =
        Some(ServerOptions_LevelshotDraw as unsafe extern "C" fn(_: *mut libc::c_void) -> ());
    s_serveroptions.picframe.generic.type_0 = 6 as i32;
    s_serveroptions.picframe.generic.flags =
        0x4 as i32 as u32 | 0x4000 as i32 as u32 | 0x40 as i32 as u32;
    s_serveroptions.picframe.generic.x = 352 as i32 - 38 as i32;
    s_serveroptions.picframe.generic.y = 80 as i32 - 40 as i32;
    s_serveroptions.picframe.width = 320 as i32;
    s_serveroptions.picframe.height = 320 as i32;
    s_serveroptions.picframe.focuspic =
        b"menu/art/maps_select\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    y = 272 as i32;
    if s_serveroptions.gametype != GT_CTF as i32 {
        s_serveroptions.fraglimit.generic.type_0 = 4 as i32;
        s_serveroptions.fraglimit.generic.name =
            b"Frag Limit:\x00" as *const u8 as *const libc::c_char;
        s_serveroptions.fraglimit.generic.flags =
            0x20 as i32 as u32 | 0x100 as i32 as u32 | 0x2 as i32 as u32;
        s_serveroptions.fraglimit.generic.x = 456 as i32;
        s_serveroptions.fraglimit.generic.y = y;
        s_serveroptions.fraglimit.generic.statusbar =
            Some(ServerOptions_StatusBar as unsafe extern "C" fn(_: *mut libc::c_void) -> ());
        s_serveroptions.fraglimit.field.widthInChars = 3 as i32;
        s_serveroptions.fraglimit.field.maxchars = 3 as i32
    } else {
        s_serveroptions.flaglimit.generic.type_0 = 4 as i32;
        s_serveroptions.flaglimit.generic.name =
            b"Capture Limit:\x00" as *const u8 as *const libc::c_char;
        s_serveroptions.flaglimit.generic.flags =
            0x20 as i32 as u32 | 0x100 as i32 as u32 | 0x2 as i32 as u32;
        s_serveroptions.flaglimit.generic.x = 456 as i32;
        s_serveroptions.flaglimit.generic.y = y;
        s_serveroptions.flaglimit.generic.statusbar =
            Some(ServerOptions_StatusBar as unsafe extern "C" fn(_: *mut libc::c_void) -> ());
        s_serveroptions.flaglimit.field.widthInChars = 3 as i32;
        s_serveroptions.flaglimit.field.maxchars = 3 as i32
    }
    y += 16 as i32 + 2 as i32;
    s_serveroptions.timelimit.generic.type_0 = 4 as i32;
    s_serveroptions.timelimit.generic.name = b"Time Limit:\x00" as *const u8 as *const libc::c_char;
    s_serveroptions.timelimit.generic.flags =
        0x20 as i32 as u32 | 0x100 as i32 as u32 | 0x2 as i32 as u32;
    s_serveroptions.timelimit.generic.x = 456 as i32;
    s_serveroptions.timelimit.generic.y = y;
    s_serveroptions.timelimit.generic.statusbar =
        Some(ServerOptions_StatusBar as unsafe extern "C" fn(_: *mut libc::c_void) -> ());
    s_serveroptions.timelimit.field.widthInChars = 3 as i32;
    s_serveroptions.timelimit.field.maxchars = 3 as i32;
    if s_serveroptions.gametype >= GT_TEAM as i32 {
        y += 16 as i32 + 2 as i32;
        s_serveroptions.friendlyfire.generic.type_0 = 5 as i32;
        s_serveroptions.friendlyfire.generic.flags = 0x100 as i32 as u32 | 0x2 as i32 as u32;
        s_serveroptions.friendlyfire.generic.x = 456 as i32;
        s_serveroptions.friendlyfire.generic.y = y;
        s_serveroptions.friendlyfire.generic.name =
            b"Friendly Fire:\x00" as *const u8 as *const libc::c_char
    }
    y += 16 as i32 + 2 as i32;
    s_serveroptions.pure_0.generic.type_0 = 5 as i32;
    s_serveroptions.pure_0.generic.flags = 0x100 as i32 as u32 | 0x2 as i32 as u32;
    s_serveroptions.pure_0.generic.x = 456 as i32;
    s_serveroptions.pure_0.generic.y = y;
    s_serveroptions.pure_0.generic.name = b"Pure Server:\x00" as *const u8 as *const libc::c_char;
    if s_serveroptions.multiplayer as u64 != 0 {
        y += 16 as i32 + 2 as i32;
        s_serveroptions.dedicated.generic.type_0 = 3 as i32;
        s_serveroptions.dedicated.generic.id = 22 as i32;
        s_serveroptions.dedicated.generic.flags = 0x100 as i32 as u32 | 0x2 as i32 as u32;
        s_serveroptions.dedicated.generic.callback =
            Some(ServerOptions_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
        s_serveroptions.dedicated.generic.x = 456 as i32;
        s_serveroptions.dedicated.generic.y = y;
        s_serveroptions.dedicated.generic.name =
            b"Dedicated:\x00" as *const u8 as *const libc::c_char;
        s_serveroptions.dedicated.itemnames = dedicated_list.as_mut_ptr()
    }
    if s_serveroptions.multiplayer as u64 != 0 {
        y += 16 as i32 + 2 as i32;
        s_serveroptions.hostname.generic.type_0 = 4 as i32;
        s_serveroptions.hostname.generic.name =
            b"Hostname:\x00" as *const u8 as *const libc::c_char;
        s_serveroptions.hostname.generic.flags = 0x2 as i32 as u32;
        s_serveroptions.hostname.generic.x = 456 as i32;
        s_serveroptions.hostname.generic.y = y;
        s_serveroptions.hostname.field.widthInChars = 18 as i32;
        s_serveroptions.hostname.field.maxchars = 64 as i32
    }
    y += 16 as i32 + 2 as i32;
    s_serveroptions.punkbuster.generic.type_0 = 3 as i32;
    s_serveroptions.punkbuster.generic.name =
        b"Punkbuster:\x00" as *const u8 as *const libc::c_char;
    s_serveroptions.punkbuster.generic.flags = 0x100 as i32 as u32 | 0x2 as i32 as u32;
    s_serveroptions.punkbuster.generic.id = 0 as i32;
    s_serveroptions.punkbuster.generic.x = 456 as i32;
    s_serveroptions.punkbuster.generic.y = y;
    s_serveroptions.punkbuster.itemnames = punkbuster_items.as_mut_ptr();
    y = 80 as i32;
    s_serveroptions.botSkill.generic.type_0 = 3 as i32;
    s_serveroptions.botSkill.generic.flags = 0x100 as i32 as u32 | 0x2 as i32 as u32;
    s_serveroptions.botSkill.generic.name = b"Bot Skill:\x00" as *const u8 as *const libc::c_char;
    s_serveroptions.botSkill.generic.x = (32 as i32 as usize).wrapping_add(
        crate::stdlib::strlen(s_serveroptions.botSkill.generic.name)
            .wrapping_add(2 as i32 as usize)
            .wrapping_mul(8 as i32 as usize),
    ) as i32;
    s_serveroptions.botSkill.generic.y = y;
    s_serveroptions.botSkill.itemnames = botSkill_list.as_mut_ptr();
    s_serveroptions.botSkill.curvalue = 1 as i32;
    y += 2 as i32 * 16 as i32;
    s_serveroptions.player0.generic.type_0 = 7 as i32;
    s_serveroptions.player0.generic.flags = 0x2 as i32 as u32;
    s_serveroptions.player0.generic.x = 32 as i32 + 8 as i32;
    s_serveroptions.player0.generic.y = y;
    s_serveroptions.player0.color = color_orange.as_mut_ptr();
    s_serveroptions.player0.style = 0 as i32 | 0x10 as i32;
    n = 0 as i32;
    while n < 12 as i32 {
        s_serveroptions.playerType[n as usize].generic.type_0 = 3 as i32;
        s_serveroptions.playerType[n as usize].generic.flags = 0x2 as i32 as u32;
        s_serveroptions.playerType[n as usize].generic.id = 20 as i32;
        s_serveroptions.playerType[n as usize].generic.callback =
            Some(ServerOptions_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
        s_serveroptions.playerType[n as usize].generic.x = 32 as i32;
        s_serveroptions.playerType[n as usize].generic.y = y;
        s_serveroptions.playerType[n as usize].itemnames = playerType_list.as_mut_ptr();
        s_serveroptions.playerName[n as usize].generic.type_0 = 7 as i32;
        s_serveroptions.playerName[n as usize].generic.flags = 0x2 as i32 as u32;
        s_serveroptions.playerName[n as usize].generic.x = 96 as i32;
        s_serveroptions.playerName[n as usize].generic.y = y;
        s_serveroptions.playerName[n as usize].generic.callback = Some(
            ServerOptions_PlayerNameEvent
                as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> (),
        );
        s_serveroptions.playerName[n as usize].generic.id = n;
        s_serveroptions.playerName[n as usize].generic.ownerdraw =
            Some(PlayerName_Draw as unsafe extern "C" fn(_: *mut libc::c_void) -> ());
        s_serveroptions.playerName[n as usize].color = color_orange.as_mut_ptr();
        s_serveroptions.playerName[n as usize].style = 0x10 as i32;
        s_serveroptions.playerName[n as usize].string =
            s_serveroptions.playerNameBuffers[n as usize].as_mut_ptr();
        s_serveroptions.playerName[n as usize].generic.top =
            s_serveroptions.playerName[n as usize].generic.y;
        s_serveroptions.playerName[n as usize].generic.bottom =
            s_serveroptions.playerName[n as usize].generic.y + 16 as i32;
        s_serveroptions.playerName[n as usize].generic.left =
            s_serveroptions.playerName[n as usize].generic.x - 16 as i32 / 2 as i32;
        s_serveroptions.playerName[n as usize].generic.right =
            s_serveroptions.playerName[n as usize].generic.x + 16 as i32 * 8 as i32;
        s_serveroptions.playerTeam[n as usize].generic.type_0 = 3 as i32;
        s_serveroptions.playerTeam[n as usize].generic.flags = 0x2 as i32 as u32;
        s_serveroptions.playerTeam[n as usize].generic.x = 240 as i32;
        s_serveroptions.playerTeam[n as usize].generic.y = y;
        s_serveroptions.playerTeam[n as usize].itemnames = playerTeam_list.as_mut_ptr();
        y += 16 as i32 + 4 as i32;
        n += 1
    }
    s_serveroptions.back.generic.type_0 = 6 as i32;
    s_serveroptions.back.generic.name = b"menu/art/back_0\x00" as *const u8 as *const libc::c_char;
    s_serveroptions.back.generic.flags = 0x4 as i32 as u32 | 0x100 as i32 as u32;
    s_serveroptions.back.generic.callback =
        Some(ServerOptions_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_serveroptions.back.generic.id = 24 as i32;
    s_serveroptions.back.generic.x = 0 as i32;
    s_serveroptions.back.generic.y = 480 as i32 - 64 as i32;
    s_serveroptions.back.width = 128 as i32;
    s_serveroptions.back.height = 64 as i32;
    s_serveroptions.back.focuspic =
        b"menu/art/back_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_serveroptions.next.generic.type_0 = 6 as i32;
    s_serveroptions.next.generic.name = b"menu/art/next_0\x00" as *const u8 as *const libc::c_char;
    s_serveroptions.next.generic.flags = 0x10 as i32 as u32
        | 0x100 as i32 as u32
        | 0x4000 as i32 as u32
        | 0x2000 as i32 as u32
        | 0x1000 as i32 as u32;
    s_serveroptions.next.generic.callback =
        Some(ServerOptions_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_serveroptions.next.generic.id = 18 as i32;
    s_serveroptions.next.generic.x = 640 as i32;
    s_serveroptions.next.generic.y = 480 as i32 - 64 as i32 - 72 as i32;
    s_serveroptions.next.generic.statusbar =
        Some(ServerOptions_StatusBar as unsafe extern "C" fn(_: *mut libc::c_void) -> ());
    s_serveroptions.next.width = 128 as i32;
    s_serveroptions.next.height = 64 as i32;
    s_serveroptions.next.focuspic =
        b"menu/art/next_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_serveroptions.go.generic.type_0 = 6 as i32;
    s_serveroptions.go.generic.name = b"menu/art/fight_0\x00" as *const u8 as *const libc::c_char;
    s_serveroptions.go.generic.flags = 0x10 as i32 as u32 | 0x100 as i32 as u32;
    s_serveroptions.go.generic.callback =
        Some(ServerOptions_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_serveroptions.go.generic.id = 23 as i32;
    s_serveroptions.go.generic.x = 640 as i32;
    s_serveroptions.go.generic.y = 480 as i32 - 64 as i32;
    s_serveroptions.go.width = 128 as i32;
    s_serveroptions.go.height = 64 as i32;
    s_serveroptions.go.focuspic =
        b"menu/art/fight_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    Menu_AddItem(
        &mut s_serveroptions.menu as *mut _ as *mut _tag_menuframework,
        &mut s_serveroptions.banner as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_serveroptions.menu as *mut _ as *mut _tag_menuframework,
        &mut s_serveroptions.mappic as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_serveroptions.menu as *mut _ as *mut _tag_menuframework,
        &mut s_serveroptions.picframe as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_serveroptions.menu as *mut _ as *mut _tag_menuframework,
        &mut s_serveroptions.botSkill as *mut menulist_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_serveroptions.menu as *mut _ as *mut _tag_menuframework,
        &mut s_serveroptions.player0 as *mut menutext_s as *mut libc::c_void,
    );
    n = 0 as i32;
    while n < 12 as i32 {
        if n != 0 as i32 {
            Menu_AddItem(
                &mut s_serveroptions.menu as *mut _ as *mut _tag_menuframework,
                &mut *s_serveroptions.playerType.as_mut_ptr().offset(n as isize) as *mut menulist_s
                    as *mut libc::c_void,
            );
        }
        Menu_AddItem(
            &mut s_serveroptions.menu as *mut _ as *mut _tag_menuframework,
            &mut *s_serveroptions.playerName.as_mut_ptr().offset(n as isize) as *mut menutext_s
                as *mut libc::c_void,
        );
        if s_serveroptions.gametype >= GT_TEAM as i32 {
            Menu_AddItem(
                &mut s_serveroptions.menu as *mut _ as *mut _tag_menuframework,
                &mut *s_serveroptions.playerTeam.as_mut_ptr().offset(n as isize) as *mut menulist_s
                    as *mut libc::c_void,
            );
        }
        n += 1
    }
    if s_serveroptions.gametype != GT_CTF as i32 {
        Menu_AddItem(
            &mut s_serveroptions.menu as *mut _ as *mut _tag_menuframework,
            &mut s_serveroptions.fraglimit as *mut menufield_s as *mut libc::c_void,
        );
    } else {
        Menu_AddItem(
            &mut s_serveroptions.menu as *mut _ as *mut _tag_menuframework,
            &mut s_serveroptions.flaglimit as *mut menufield_s as *mut libc::c_void,
        );
    }
    Menu_AddItem(
        &mut s_serveroptions.menu as *mut _ as *mut _tag_menuframework,
        &mut s_serveroptions.timelimit as *mut menufield_s as *mut libc::c_void,
    );
    if s_serveroptions.gametype >= GT_TEAM as i32 {
        Menu_AddItem(
            &mut s_serveroptions.menu as *mut _ as *mut _tag_menuframework,
            &mut s_serveroptions.friendlyfire as *mut menuradiobutton_s as *mut libc::c_void,
        );
    }
    Menu_AddItem(
        &mut s_serveroptions.menu as *mut _ as *mut _tag_menuframework,
        &mut s_serveroptions.pure_0 as *mut menuradiobutton_s as *mut libc::c_void,
    );
    if s_serveroptions.multiplayer as u64 != 0 {
        Menu_AddItem(
            &mut s_serveroptions.menu as *mut _ as *mut _tag_menuframework,
            &mut s_serveroptions.dedicated as *mut menulist_s as *mut libc::c_void,
        );
    }
    if s_serveroptions.multiplayer as u64 != 0 {
        Menu_AddItem(
            &mut s_serveroptions.menu as *mut _ as *mut _tag_menuframework,
            &mut s_serveroptions.hostname as *mut menufield_s as *mut libc::c_void,
        );
    }
    Menu_AddItem(
        &mut s_serveroptions.menu as *mut _ as *mut _tag_menuframework,
        &mut s_serveroptions.back as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_serveroptions.menu as *mut _ as *mut _tag_menuframework,
        &mut s_serveroptions.next as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_serveroptions.menu as *mut _ as *mut _tag_menuframework,
        &mut s_serveroptions.go as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_serveroptions.menu as *mut _ as *mut _tag_menuframework,
        &mut s_serveroptions.punkbuster as *mut menulist_s as *mut libc::c_void,
    );
    ServerOptions_SetMenuItems();
}
/*
=================
ServerOptions_Cache
=================
*/
#[no_mangle]

pub unsafe extern "C" fn ServerOptions_Cache() {
    trap_R_RegisterShaderNoMip(b"menu/art/back_0\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/back_1\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/fight_0\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/fight_1\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/maps_select\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/unknownmap\x00" as *const u8 as *const libc::c_char);
}
/*
=================
UI_ServerOptionsMenu
=================
*/

unsafe extern "C" fn UI_ServerOptionsMenu(mut multiplayer: qboolean) {
    ServerOptions_MenuInit(multiplayer);
    UI_PushMenu(&mut s_serveroptions.menu as *mut _ as *mut _tag_menuframework);
}

static mut botSelectInfo: botSelectInfo_t = botSelectInfo_t {
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
            name: std::ptr::null(),
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
    pics: [menubitmap_s {
        generic: menucommon_s {
            type_0: 0,
            name: std::ptr::null(),
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
    }; 16],
    picbuttons: [menubitmap_s {
        generic: menucommon_s {
            type_0: 0,
            name: std::ptr::null(),
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
    }; 16],
    picnames: [menutext_s {
        generic: menucommon_s {
            type_0: 0,
            name: std::ptr::null(),
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
    }; 16],
    arrows: menubitmap_s {
        generic: menucommon_s {
            type_0: 0,
            name: std::ptr::null(),
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
    left: menubitmap_s {
        generic: menucommon_s {
            type_0: 0,
            name: std::ptr::null(),
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
    right: menubitmap_s {
        generic: menucommon_s {
            type_0: 0,
            name: std::ptr::null(),
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
    go: menubitmap_s {
        generic: menucommon_s {
            type_0: 0,
            name: std::ptr::null(),
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
    back: menubitmap_s {
        generic: menucommon_s {
            type_0: 0,
            name: std::ptr::null(),
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
    numBots: 0,
    modelpage: 0,
    numpages: 0,
    selectedmodel: 0,
    sortedBotNums: [0; 1024],
    boticons: [[0; 64]; 16],
    botnames: [[0; 16]; 16],
};
/*
=================
UI_BotSelectMenu_SortCompare
=================
*/

unsafe extern "C" fn UI_BotSelectMenu_SortCompare(
    mut arg1: *const libc::c_void,
    mut arg2: *const libc::c_void,
) -> i32 {
    let mut num1: i32 = 0;
    let mut num2: i32 = 0;
    let mut info1: *const libc::c_char = std::ptr::null();
    let mut info2: *const libc::c_char = std::ptr::null();
    let mut name1: *const libc::c_char = std::ptr::null();
    let mut name2: *const libc::c_char = std::ptr::null();
    num1 = *(arg1 as *mut i32);
    num2 = *(arg2 as *mut i32);
    info1 = UI_GetBotInfoByNumber(num1);
    info2 = UI_GetBotInfoByNumber(num2);
    name1 = Info_ValueForKey(info1, b"name\x00" as *const u8 as *const libc::c_char);
    name2 = Info_ValueForKey(info2, b"name\x00" as *const u8 as *const libc::c_char);
    return Q_stricmp(name1, name2);
}
/*
=================
UI_BotSelectMenu_BuildList
=================
*/

unsafe extern "C" fn UI_BotSelectMenu_BuildList() {
    let mut n: i32 = 0;
    botSelectInfo.modelpage = 0 as i32;
    botSelectInfo.numBots = UI_GetNumBots();
    botSelectInfo.numpages = botSelectInfo.numBots / (4 as i32 * 4 as i32);
    if botSelectInfo.numBots % (4 as i32 * 4 as i32) != 0 {
        botSelectInfo.numpages += 1
    }
    // initialize the array
    n = 0 as i32;
    while n < botSelectInfo.numBots {
        botSelectInfo.sortedBotNums[n as usize] = n;
        n += 1
    }
    // now sort it
    qsort(
        botSelectInfo.sortedBotNums.as_mut_ptr() as *mut libc::c_void,
        botSelectInfo.numBots as size_t,
        ::std::mem::size_of::<i32>() as usize,
        Some(
            UI_BotSelectMenu_SortCompare
                as unsafe extern "C" fn(_: *const libc::c_void, _: *const libc::c_void) -> i32,
        ),
    );
}
/*
=================
ServerPlayerIcon
=================
*/

unsafe extern "C" fn ServerPlayerIcon(
    mut modelAndSkin: *const libc::c_char,
    mut iconName: *mut libc::c_char,
    mut iconNameMaxSize: i32,
) {
    let mut skin: *mut libc::c_char = std::ptr::null_mut();
    let mut model: [libc::c_char; 64] = [0; 64];
    Q_strncpyz(
        model.as_mut_ptr(),
        modelAndSkin,
        ::std::mem::size_of::<[libc::c_char; 64]>() as usize as i32,
    );
    skin = libc::strrchr(model.as_mut_ptr(), '/' as i32);
    if !skin.is_null() {
        let fresh1 = skin;
        skin = skin.offset(1);
        *fresh1 = '\u{0}' as i32 as libc::c_char
    } else {
        skin = b"default\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    }
    Com_sprintf(
        iconName,
        iconNameMaxSize,
        b"models/players/%s/icon_%s.tga\x00" as *const u8 as *const libc::c_char,
        model.as_mut_ptr(),
        skin,
    );
    if trap_R_RegisterShaderNoMip(iconName) == 0
        && Q_stricmp(skin, b"default\x00" as *const u8 as *const libc::c_char) != 0 as i32
    {
        Com_sprintf(
            iconName,
            iconNameMaxSize,
            b"models/players/%s/icon_default.tga\x00" as *const u8 as *const libc::c_char,
            model.as_mut_ptr(),
        );
    };
}
/*
=================
UI_BotSelectMenu_UpdateGrid
=================
*/

unsafe extern "C" fn UI_BotSelectMenu_UpdateGrid() {
    let mut info: *const libc::c_char = std::ptr::null();
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    j = botSelectInfo.modelpage * (4 as i32 * 4 as i32);
    i = 0 as i32;
    while i < 4 as i32 * 4 as i32 {
        if j < botSelectInfo.numBots {
            info = UI_GetBotInfoByNumber(botSelectInfo.sortedBotNums[j as usize]);
            ServerPlayerIcon(
                Info_ValueForKey(info, b"model\x00" as *const u8 as *const libc::c_char),
                botSelectInfo.boticons[i as usize].as_mut_ptr(),
                64 as i32,
            );
            Q_strncpyz(
                botSelectInfo.botnames[i as usize].as_mut_ptr(),
                Info_ValueForKey(info, b"name\x00" as *const u8 as *const libc::c_char),
                16 as i32,
            );
            Q_CleanStr(botSelectInfo.botnames[i as usize].as_mut_ptr());
            botSelectInfo.pics[i as usize].generic.name =
                botSelectInfo.boticons[i as usize].as_mut_ptr();
            if BotAlreadySelected(botSelectInfo.botnames[i as usize].as_mut_ptr()) as u64 != 0 {
                botSelectInfo.picnames[i as usize].color = color_red.as_mut_ptr()
            } else {
                botSelectInfo.picnames[i as usize].color = color_orange.as_mut_ptr()
            }
            botSelectInfo.picbuttons[i as usize].generic.flags &= !(0x4000 as i32 as u32)
        } else {
            // dead slot
            botSelectInfo.pics[i as usize].generic.name = std::ptr::null();
            botSelectInfo.picbuttons[i as usize].generic.flags |= 0x4000 as i32 as u32;
            botSelectInfo.botnames[i as usize][0 as i32 as usize] = 0 as i32 as libc::c_char
        }
        botSelectInfo.pics[i as usize].generic.flags &= !(0x40 as i32 as u32);
        botSelectInfo.pics[i as usize].shader = 0 as i32;
        botSelectInfo.picbuttons[i as usize].generic.flags |= 0x100 as i32 as u32;
        i += 1;
        j += 1
    }
    // set selected model
    i = botSelectInfo.selectedmodel % (4 as i32 * 4 as i32);
    botSelectInfo.pics[i as usize].generic.flags |= 0x40 as i32 as u32;
    botSelectInfo.picbuttons[i as usize].generic.flags &= !(0x100 as i32 as u32);
    if botSelectInfo.numpages > 1 as i32 {
        if botSelectInfo.modelpage > 0 as i32 {
            botSelectInfo.left.generic.flags &= !(0x4000 as i32 as u32)
        } else {
            botSelectInfo.left.generic.flags |= 0x4000 as i32 as u32
        }
        if botSelectInfo.modelpage < botSelectInfo.numpages - 1 as i32 {
            botSelectInfo.right.generic.flags &= !(0x4000 as i32 as u32)
        } else {
            botSelectInfo.right.generic.flags |= 0x4000 as i32 as u32
        }
    } else {
        // hide left/right markers
        botSelectInfo.left.generic.flags |= 0x4000 as i32 as u32;
        botSelectInfo.right.generic.flags |= 0x4000 as i32 as u32
    };
}
/*
=================
UI_BotSelectMenu_Default
=================
*/

unsafe extern "C" fn UI_BotSelectMenu_Default(mut bot: *mut libc::c_char) {
    let mut botInfo: *const libc::c_char = std::ptr::null();
    let mut test: *const libc::c_char = std::ptr::null();
    let mut n: i32 = 0;
    let mut i: i32 = 0;
    n = 0 as i32;
    while n < botSelectInfo.numBots {
        botInfo = UI_GetBotInfoByNumber(n);
        test = Info_ValueForKey(botInfo, b"name\x00" as *const u8 as *const libc::c_char);
        if Q_stricmp(bot, test) == 0 as i32 {
            break;
        }
        n += 1
    }
    if n == botSelectInfo.numBots {
        botSelectInfo.selectedmodel = 0 as i32;
        return;
    }
    i = 0 as i32;
    while i < botSelectInfo.numBots {
        if botSelectInfo.sortedBotNums[i as usize] == n {
            break;
        }
        i += 1
    }
    if i == botSelectInfo.numBots {
        botSelectInfo.selectedmodel = 0 as i32;
        return;
    }
    botSelectInfo.selectedmodel = i;
}
/*
=================
UI_BotSelectMenu_LeftEvent
=================
*/

unsafe extern "C" fn UI_BotSelectMenu_LeftEvent(mut _ptr: *mut libc::c_void, mut event: i32) {
    if event != 3 as i32 {
        return;
    }
    if botSelectInfo.modelpage > 0 as i32 {
        botSelectInfo.modelpage -= 1;
        botSelectInfo.selectedmodel = botSelectInfo.modelpage * (4 as i32 * 4 as i32);
        UI_BotSelectMenu_UpdateGrid();
    };
}
/*
=================
UI_BotSelectMenu_RightEvent
=================
*/

unsafe extern "C" fn UI_BotSelectMenu_RightEvent(mut _ptr: *mut libc::c_void, mut event: i32) {
    if event != 3 as i32 {
        return;
    }
    if botSelectInfo.modelpage < botSelectInfo.numpages - 1 as i32 {
        botSelectInfo.modelpage += 1;
        botSelectInfo.selectedmodel = botSelectInfo.modelpage * (4 as i32 * 4 as i32);
        UI_BotSelectMenu_UpdateGrid();
    };
}
/*
=================
UI_BotSelectMenu_BotEvent
=================
*/

unsafe extern "C" fn UI_BotSelectMenu_BotEvent(mut ptr: *mut libc::c_void, mut event: i32) {
    let mut i: i32 = 0;
    if event != 3 as i32 {
        return;
    }
    i = 0 as i32;
    while i < 4 as i32 * 4 as i32 {
        botSelectInfo.pics[i as usize].generic.flags &= !(0x40 as i32 as u32);
        botSelectInfo.picbuttons[i as usize].generic.flags |= 0x100 as i32 as u32;
        i += 1
    }
    // set selected
    i = (*(ptr as *mut menucommon_s)).id;
    botSelectInfo.pics[i as usize].generic.flags |= 0x40 as i32 as u32;
    botSelectInfo.picbuttons[i as usize].generic.flags &= !(0x100 as i32 as u32);
    botSelectInfo.selectedmodel = botSelectInfo.modelpage * (4 as i32 * 4 as i32) + i;
}
/*
=================
UI_BotSelectMenu_BackEvent
=================
*/

unsafe extern "C" fn UI_BotSelectMenu_BackEvent(mut _ptr: *mut libc::c_void, mut event: i32) {
    if event != 3 as i32 {
        return;
    }
    UI_PopMenu();
}
/*
=================
UI_BotSelectMenu_SelectEvent
=================
*/

unsafe extern "C" fn UI_BotSelectMenu_SelectEvent(mut _ptr: *mut libc::c_void, mut event: i32) {
    if event != 3 as i32 {
        return;
    }
    UI_PopMenu();
    s_serveroptions.newBot = qtrue;
    Q_strncpyz(
        s_serveroptions.newBotName.as_mut_ptr(),
        botSelectInfo.botnames[(botSelectInfo.selectedmodel % (4 as i32 * 4 as i32)) as usize]
            .as_mut_ptr(),
        16 as i32,
    );
}
/*
=================
UI_BotSelectMenu_Cache
=================
*/
#[no_mangle]

pub unsafe extern "C" fn UI_BotSelectMenu_Cache() {
    trap_R_RegisterShaderNoMip(b"menu/art/back_0\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/back_1\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/accept_0\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/accept_1\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(
        b"menu/art/opponents_select\x00" as *const u8 as *const libc::c_char,
    );
    trap_R_RegisterShaderNoMip(
        b"menu/art/opponents_selected\x00" as *const u8 as *const libc::c_char,
    );
    trap_R_RegisterShaderNoMip(b"menu/art/gs_arrows_0\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/gs_arrows_l\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/gs_arrows_r\x00" as *const u8 as *const libc::c_char);
}

unsafe extern "C" fn UI_BotSelectMenu_Init(mut bot: *mut libc::c_char) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    crate::stdlib::memset(
        &mut botSelectInfo as *mut botSelectInfo_t as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<botSelectInfo_t>() as usize,
    );
    botSelectInfo.menu.wrapAround = qtrue;
    botSelectInfo.menu.fullscreen = qtrue;
    UI_BotSelectMenu_Cache();
    botSelectInfo.banner.generic.type_0 = 10 as i32;
    botSelectInfo.banner.generic.x = 320 as i32;
    botSelectInfo.banner.generic.y = 16 as i32;
    botSelectInfo.banner.string =
        b"SELECT BOT\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    botSelectInfo.banner.color = color_white.as_mut_ptr();
    botSelectInfo.banner.style = 0x1 as i32;
    y = 80 as i32;
    i = 0 as i32;
    k = 0 as i32;
    while i < 4 as i32 {
        x = 180 as i32;
        j = 0 as i32;
        while j < 4 as i32 {
            botSelectInfo.pics[k as usize].generic.type_0 = 6 as i32;
            botSelectInfo.pics[k as usize].generic.flags = 0x4 as i32 as u32 | 0x4000 as i32 as u32;
            botSelectInfo.pics[k as usize].generic.x = x;
            botSelectInfo.pics[k as usize].generic.y = y;
            botSelectInfo.pics[k as usize].generic.name =
                botSelectInfo.boticons[k as usize].as_mut_ptr();
            botSelectInfo.pics[k as usize].width = 64 as i32;
            botSelectInfo.pics[k as usize].height = 64 as i32;
            botSelectInfo.pics[k as usize].focuspic =
                b"menu/art/opponents_selected\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char;
            botSelectInfo.pics[k as usize].focuscolor = colorRed.as_mut_ptr();
            botSelectInfo.picbuttons[k as usize].generic.type_0 = 6 as i32;
            botSelectInfo.picbuttons[k as usize].generic.flags =
                0x4 as i32 as u32 | 0x8000 as i32 as u32 | 0x100 as i32 as u32;
            botSelectInfo.picbuttons[k as usize].generic.callback = Some(
                UI_BotSelectMenu_BotEvent
                    as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> (),
            );
            botSelectInfo.picbuttons[k as usize].generic.id = k;
            botSelectInfo.picbuttons[k as usize].generic.x = x - 16 as i32;
            botSelectInfo.picbuttons[k as usize].generic.y = y - 16 as i32;
            botSelectInfo.picbuttons[k as usize].generic.left = x;
            botSelectInfo.picbuttons[k as usize].generic.top = y;
            botSelectInfo.picbuttons[k as usize].generic.right = x + 64 as i32;
            botSelectInfo.picbuttons[k as usize].generic.bottom = y + 64 as i32;
            botSelectInfo.picbuttons[k as usize].width = 128 as i32;
            botSelectInfo.picbuttons[k as usize].height = 128 as i32;
            botSelectInfo.picbuttons[k as usize].focuspic =
                b"menu/art/opponents_select\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char;
            botSelectInfo.picbuttons[k as usize].focuscolor = colorRed.as_mut_ptr();
            botSelectInfo.picnames[k as usize].generic.type_0 = 7 as i32;
            botSelectInfo.picnames[k as usize].generic.flags = 0x2 as i32 as u32;
            botSelectInfo.picnames[k as usize].generic.x = x + 32 as i32;
            botSelectInfo.picnames[k as usize].generic.y = y + 64 as i32;
            botSelectInfo.picnames[k as usize].string =
                botSelectInfo.botnames[k as usize].as_mut_ptr();
            botSelectInfo.picnames[k as usize].color = color_orange.as_mut_ptr();
            botSelectInfo.picnames[k as usize].style = 0x1 as i32 | 0x10 as i32;
            x += 64 as i32 + 6 as i32;
            j += 1;
            k += 1
        }
        y += 64 as i32 + 16 as i32 + 6 as i32;
        i += 1
    }
    botSelectInfo.arrows.generic.type_0 = 6 as i32;
    botSelectInfo.arrows.generic.name =
        b"menu/art/gs_arrows_0\x00" as *const u8 as *const libc::c_char;
    botSelectInfo.arrows.generic.flags = 0x4000 as i32 as u32;
    botSelectInfo.arrows.generic.x = 260 as i32;
    botSelectInfo.arrows.generic.y = 440 as i32;
    botSelectInfo.arrows.width = 128 as i32;
    botSelectInfo.arrows.height = 32 as i32;
    botSelectInfo.left.generic.type_0 = 6 as i32;
    botSelectInfo.left.generic.flags = 0x4 as i32 as u32 | 0x100 as i32 as u32;
    botSelectInfo.left.generic.callback = Some(
        UI_BotSelectMenu_LeftEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> (),
    );
    botSelectInfo.left.generic.x = 260 as i32;
    botSelectInfo.left.generic.y = 440 as i32;
    botSelectInfo.left.width = 64 as i32;
    botSelectInfo.left.height = 32 as i32;
    botSelectInfo.left.focuspic =
        b"menu/art/gs_arrows_l\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    botSelectInfo.right.generic.type_0 = 6 as i32;
    botSelectInfo.right.generic.flags = 0x4 as i32 as u32 | 0x100 as i32 as u32;
    botSelectInfo.right.generic.callback = Some(
        UI_BotSelectMenu_RightEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> (),
    );
    botSelectInfo.right.generic.x = 321 as i32;
    botSelectInfo.right.generic.y = 440 as i32;
    botSelectInfo.right.width = 64 as i32;
    botSelectInfo.right.height = 32 as i32;
    botSelectInfo.right.focuspic =
        b"menu/art/gs_arrows_r\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    botSelectInfo.back.generic.type_0 = 6 as i32;
    botSelectInfo.back.generic.name = b"menu/art/back_0\x00" as *const u8 as *const libc::c_char;
    botSelectInfo.back.generic.flags = 0x4 as i32 as u32 | 0x100 as i32 as u32;
    botSelectInfo.back.generic.callback = Some(
        UI_BotSelectMenu_BackEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> (),
    );
    botSelectInfo.back.generic.x = 0 as i32;
    botSelectInfo.back.generic.y = 480 as i32 - 64 as i32;
    botSelectInfo.back.width = 128 as i32;
    botSelectInfo.back.height = 64 as i32;
    botSelectInfo.back.focuspic =
        b"menu/art/back_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    botSelectInfo.go.generic.type_0 = 6 as i32;
    botSelectInfo.go.generic.name = b"menu/art/accept_0\x00" as *const u8 as *const libc::c_char;
    botSelectInfo.go.generic.flags = 0x10 as i32 as u32 | 0x100 as i32 as u32;
    botSelectInfo.go.generic.callback = Some(
        UI_BotSelectMenu_SelectEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> (),
    );
    botSelectInfo.go.generic.x = 640 as i32;
    botSelectInfo.go.generic.y = 480 as i32 - 64 as i32;
    botSelectInfo.go.width = 128 as i32;
    botSelectInfo.go.height = 64 as i32;
    botSelectInfo.go.focuspic =
        b"menu/art/accept_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    Menu_AddItem(
        &mut botSelectInfo.menu as *mut _ as *mut _tag_menuframework,
        &mut botSelectInfo.banner as *mut menutext_s as *mut libc::c_void,
    );
    i = 0 as i32;
    while i < 4 as i32 * 4 as i32 {
        Menu_AddItem(
            &mut botSelectInfo.menu as *mut _ as *mut _tag_menuframework,
            &mut *botSelectInfo.pics.as_mut_ptr().offset(i as isize) as *mut menubitmap_s
                as *mut libc::c_void,
        );
        Menu_AddItem(
            &mut botSelectInfo.menu as *mut _ as *mut _tag_menuframework,
            &mut *botSelectInfo.picbuttons.as_mut_ptr().offset(i as isize) as *mut menubitmap_s
                as *mut libc::c_void,
        );
        Menu_AddItem(
            &mut botSelectInfo.menu as *mut _ as *mut _tag_menuframework,
            &mut *botSelectInfo.picnames.as_mut_ptr().offset(i as isize) as *mut menutext_s
                as *mut libc::c_void,
        );
        i += 1
    }
    Menu_AddItem(
        &mut botSelectInfo.menu as *mut _ as *mut _tag_menuframework,
        &mut botSelectInfo.arrows as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut botSelectInfo.menu as *mut _ as *mut _tag_menuframework,
        &mut botSelectInfo.left as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut botSelectInfo.menu as *mut _ as *mut _tag_menuframework,
        &mut botSelectInfo.right as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut botSelectInfo.menu as *mut _ as *mut _tag_menuframework,
        &mut botSelectInfo.back as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut botSelectInfo.menu as *mut _ as *mut _tag_menuframework,
        &mut botSelectInfo.go as *mut menubitmap_s as *mut libc::c_void,
    );
    UI_BotSelectMenu_BuildList();
    UI_BotSelectMenu_Default(bot);
    botSelectInfo.modelpage = botSelectInfo.selectedmodel / (4 as i32 * 4 as i32);
    UI_BotSelectMenu_UpdateGrid();
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
/*
=================
UI_BotSelectMenu
=================
*/
#[no_mangle]

pub unsafe extern "C" fn UI_BotSelectMenu(mut bot: *mut libc::c_char) {
    UI_BotSelectMenu_Init(bot);
    UI_PushMenu(&mut botSelectInfo.menu as *mut _ as *mut _tag_menuframework);
}

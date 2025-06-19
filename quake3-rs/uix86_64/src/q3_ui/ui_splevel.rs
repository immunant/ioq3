use ::libc;

pub mod stdlib_h {
    #[inline]

    pub unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> i32 {
        return libc::strtol(
            __nptr,
            0 as *mut libc::c_void as *mut *mut libc::c_char,
            10 as i32,
        ) as i32;
    }
}

pub use crate::src::q3_ui::ui_atoms::uis;
pub use crate::src::q3_ui::ui_atoms::UI_DrawHandlePic;
pub use crate::src::q3_ui::ui_atoms::UI_DrawProportionalString;
pub use crate::src::q3_ui::ui_atoms::UI_DrawString;
pub use crate::src::q3_ui::ui_atoms::UI_FillRect;
pub use crate::src::q3_ui::ui_atoms::UI_PopMenu;
pub use crate::src::q3_ui::ui_atoms::UI_PushMenu;
pub use crate::src::q3_ui::ui_confirm::UI_ConfirmMenu;
pub use crate::src::q3_ui::ui_gameinfo::UI_GetArenaInfoByNumber;
pub use crate::src::q3_ui::ui_gameinfo::UI_GetAwardLevel;
pub use crate::src::q3_ui::ui_gameinfo::UI_GetBestScore;
pub use crate::src::q3_ui::ui_gameinfo::UI_GetBotInfoByName;
pub use crate::src::q3_ui::ui_gameinfo::UI_GetBotInfoByNumber;
pub use crate::src::q3_ui::ui_gameinfo::UI_GetCurrentGame;
pub use crate::src::q3_ui::ui_gameinfo::UI_GetNumSPArenas;
pub use crate::src::q3_ui::ui_gameinfo::UI_GetNumSPTiers;
pub use crate::src::q3_ui::ui_gameinfo::UI_GetSpecialArenaInfo;
pub use crate::src::q3_ui::ui_gameinfo::UI_NewGame;
pub use crate::src::q3_ui::ui_playersettings::UI_PlayerSettingsMenu;
pub use crate::src::q3_ui::ui_qmenu::color_black;
pub use crate::src::q3_ui::ui_qmenu::color_orange;
pub use crate::src::q3_ui::ui_qmenu::color_red;
pub use crate::src::q3_ui::ui_qmenu::color_white;
pub use crate::src::q3_ui::ui_qmenu::color_yellow;
pub use crate::src::q3_ui::ui_qmenu::Bitmap_Init;
pub use crate::src::q3_ui::ui_qmenu::Menu_AddItem;
pub use crate::src::q3_ui::ui_qmenu::Menu_Draw;
pub use crate::src::q3_ui::ui_qmenu::Menu_ItemAtCursor;
pub use crate::src::q3_ui::ui_qmenu::Menu_SetCursorToItem;
pub use crate::src::q3_ui::ui_sppostgame::ui_medalPicNames;
pub use crate::src::q3_ui::ui_sppostgame::ui_medalSounds;
pub use crate::src::q3_ui::ui_spskill::UI_SPSkillMenu;
pub use crate::src::q3_ui::ui_startserver::UI_StartServerMenu;
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
pub use crate::src::qcommon::q_shared::Info_ValueForKey;
pub use crate::src::qcommon::q_shared::Q_CleanStr;
pub use crate::src::qcommon::q_shared::Q_stricmp;
pub use crate::src::qcommon::q_shared::Q_strncpyz;
pub use crate::src::qcommon::q_shared::Q_strupr;
pub use crate::src::qcommon::q_shared::CHAN_ANNOUNCER;
pub use crate::src::qcommon::q_shared::CHAN_AUTO;
pub use crate::src::qcommon::q_shared::CHAN_BODY;
pub use crate::src::qcommon::q_shared::CHAN_ITEM;
pub use crate::src::qcommon::q_shared::CHAN_LOCAL;
pub use crate::src::qcommon::q_shared::CHAN_LOCAL_SOUND;
pub use crate::src::qcommon::q_shared::CHAN_VOICE;
pub use crate::src::qcommon::q_shared::CHAN_WEAPON;
pub use crate::src::ui::ui_syscalls::trap_Cvar_Set;
pub use crate::src::ui::ui_syscalls::trap_Cvar_SetValue;
pub use crate::src::ui::ui_syscalls::trap_Cvar_VariableStringBuffer;
pub use crate::src::ui::ui_syscalls::trap_Cvar_VariableValue;
pub use crate::src::ui::ui_syscalls::trap_Key_SetCatcher;
pub use crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip;
pub use crate::src::ui::ui_syscalls::trap_R_SetColor;
pub use crate::src::ui::ui_syscalls::trap_S_RegisterSound;
pub use crate::src::ui::ui_syscalls::trap_S_StartLocalSound;

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
pub use crate::ui_local_h::AWARD_ACCURACY;
pub use crate::ui_local_h::AWARD_EXCELLENT;
pub use crate::ui_local_h::AWARD_FRAGS;
pub use crate::ui_local_h::AWARD_GAUNTLET;
pub use crate::ui_local_h::AWARD_IMPRESSIVE;
pub use crate::ui_local_h::AWARD_PERFECT;

pub use crate::src::q3_ui::ui_splevel::stdlib_h::atoi;

pub use ::libc::strtol;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct levelMenuInfo_t {
    pub menu: menuframework_s,
    pub item_banner: menutext_s,
    pub item_leftarrow: menubitmap_s,
    pub item_maps: [menubitmap_s; 4],
    pub item_rightarrow: menubitmap_s,
    pub item_player: menubitmap_s,
    pub item_awards: [menubitmap_s; 6],
    pub item_back: menubitmap_s,
    pub item_reset: menubitmap_s,
    pub item_custom: menubitmap_s,
    pub item_next: menubitmap_s,
    pub item_null: menubitmap_s,
    pub reinit: qboolean,
    pub selectedArenaInfo: *const libc::c_char,
    pub numMaps: i32,
    pub levelPicNames: [[libc::c_char; 64]; 4],
    pub levelNames: [[libc::c_char; 16]; 4],
    pub levelScores: [i32; 4],
    pub levelScoresSkill: [i32; 4],
    pub levelSelectedPic: qhandle_t,
    pub levelFocusPic: qhandle_t,
    pub levelCompletePic: [qhandle_t; 5],
    pub playerModel: [libc::c_char; 64],
    pub playerPicName: [libc::c_char; 64],
    pub awardLevels: [i32; 6],
    pub awardSounds: [sfxHandle_t; 6],
    pub numBots: i32,
    pub botPics: [qhandle_t; 7],
    pub botNames: [[libc::c_char; 10]; 7],
}

static mut levelMenuInfo: levelMenuInfo_t = levelMenuInfo_t {
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
    item_banner: menutext_s {
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
            parent: 0 as *const menuframework_s as *mut menuframework_s,
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
    item_leftarrow: menubitmap_s {
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
            parent: 0 as *const menuframework_s as *mut menuframework_s,
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
    item_maps: [menubitmap_s {
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
            parent: 0 as *const menuframework_s as *mut menuframework_s,
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
    }; 4],
    item_rightarrow: menubitmap_s {
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
            parent: 0 as *const menuframework_s as *mut menuframework_s,
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
    item_player: menubitmap_s {
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
            parent: 0 as *const menuframework_s as *mut menuframework_s,
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
    item_awards: [menubitmap_s {
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
            parent: 0 as *const menuframework_s as *mut menuframework_s,
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
    }; 6],
    item_back: menubitmap_s {
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
            parent: 0 as *const menuframework_s as *mut menuframework_s,
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
    item_reset: menubitmap_s {
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
            parent: 0 as *const menuframework_s as *mut menuframework_s,
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
    item_custom: menubitmap_s {
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
            parent: 0 as *const menuframework_s as *mut menuframework_s,
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
    item_next: menubitmap_s {
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
            parent: 0 as *const menuframework_s as *mut menuframework_s,
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
    item_null: menubitmap_s {
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
            parent: 0 as *const menuframework_s as *mut menuframework_s,
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
    reinit: qfalse,
    selectedArenaInfo: 0 as *const libc::c_char,
    numMaps: 0,
    levelPicNames: [[0; 64]; 4],
    levelNames: [[0; 16]; 4],
    levelScores: [0; 4],
    levelScoresSkill: [0; 4],
    levelSelectedPic: 0,
    levelFocusPic: 0,
    levelCompletePic: [0; 5],
    playerModel: [0; 64],
    playerPicName: [0; 64],
    awardLevels: [0; 6],
    awardSounds: [0; 6],
    numBots: 0,
    botPics: [0; 7],
    botNames: [[0; 10]; 7],
};

static mut selectedArenaSet: i32 = 0;

static mut selectedArena: i32 = 0;

static mut currentSet: i32 = 0;

static mut currentGame: i32 = 0;

static mut trainingTier: i32 = 0;

static mut finalTier: i32 = 0;

static mut minTier: i32 = 0;

static mut maxTier: i32 = 0;
/*
=================
PlayerIcon
=================
*/

unsafe extern "C" fn PlayerIcon(
    mut modelAndSkin: *const libc::c_char,
    mut iconName: *mut libc::c_char,
    mut iconNameMaxSize: i32,
) {
    let mut skin: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut model: [libc::c_char; 64] = [0; 64];
    Q_strncpyz(
        model.as_mut_ptr(),
        modelAndSkin,
        ::std::mem::size_of::<[libc::c_char; 64]>() as usize as i32,
    );
    skin = libc::strrchr(model.as_mut_ptr(), '/' as i32);
    if !skin.is_null() {
        let fresh0 = skin;
        skin = skin.offset(1);
        *fresh0 = '\u{0}' as i32 as libc::c_char
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
PlayerIconhandle
=================
*/

unsafe extern "C" fn PlayerIconHandle(mut modelAndSkin: *const libc::c_char) -> qhandle_t {
    let mut iconName: [libc::c_char; 64] = [0; 64];
    PlayerIcon(
        modelAndSkin,
        iconName.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as usize as i32,
    );
    return trap_R_RegisterShaderNoMip(iconName.as_mut_ptr());
}
/*
=================
UI_SPLevelMenu_SetBots
=================
*/

unsafe extern "C" fn UI_SPLevelMenu_SetBots() {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut bot: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut botInfo: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut bots: [libc::c_char; 1024] = [0; 1024];
    levelMenuInfo.numBots = 0 as i32;
    if selectedArenaSet > currentSet {
        return;
    }
    Q_strncpyz(
        bots.as_mut_ptr(),
        Info_ValueForKey(
            levelMenuInfo.selectedArenaInfo,
            b"bots\x00" as *const u8 as *const libc::c_char,
        ),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as usize as i32,
    );
    p = &mut *bots.as_mut_ptr().offset(0 as i32 as isize) as *mut libc::c_char;
    while *p as i32 != 0 && levelMenuInfo.numBots < 7 as i32 {
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
            let fresh1 = p;
            p = p.offset(1);
            *fresh1 = 0 as i32 as libc::c_char
        }
        botInfo = UI_GetBotInfoByName(bot);
        if botInfo.is_null() {
            botInfo = UI_GetBotInfoByNumber(levelMenuInfo.numBots)
        }
        if !botInfo.is_null() {
            levelMenuInfo.botPics[levelMenuInfo.numBots as usize] = PlayerIconHandle(
                Info_ValueForKey(botInfo, b"model\x00" as *const u8 as *const libc::c_char),
            );
            Q_strncpyz(
                levelMenuInfo.botNames[levelMenuInfo.numBots as usize].as_mut_ptr(),
                Info_ValueForKey(botInfo, b"name\x00" as *const u8 as *const libc::c_char),
                10 as i32,
            );
        } else {
            levelMenuInfo.botPics[levelMenuInfo.numBots as usize] = 0 as i32;
            Q_strncpyz(
                levelMenuInfo.botNames[levelMenuInfo.numBots as usize].as_mut_ptr(),
                bot,
                10 as i32,
            );
        }
        Q_CleanStr(levelMenuInfo.botNames[levelMenuInfo.numBots as usize].as_mut_ptr());
        levelMenuInfo.numBots += 1
    }
}
/*
=================
UI_SPLevelMenu_SetMenuItems
=================
*/

unsafe extern "C" fn UI_SPLevelMenu_SetMenuArena(
    mut n: i32,
    mut level: i32,
    mut arenaInfo: *const libc::c_char,
) {
    let mut map: [libc::c_char; 64] = [0; 64];
    Q_strncpyz(
        map.as_mut_ptr(),
        Info_ValueForKey(arenaInfo, b"map\x00" as *const u8 as *const libc::c_char),
        ::std::mem::size_of::<[libc::c_char; 64]>() as usize as i32,
    );
    Q_strncpyz(
        levelMenuInfo.levelNames[n as usize].as_mut_ptr(),
        map.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 16]>() as usize as i32,
    );
    Q_strupr(levelMenuInfo.levelNames[n as usize].as_mut_ptr());
    UI_GetBestScore(
        level,
        &mut *levelMenuInfo.levelScores.as_mut_ptr().offset(n as isize),
        &mut *levelMenuInfo
            .levelScoresSkill
            .as_mut_ptr()
            .offset(n as isize),
    );
    if levelMenuInfo.levelScores[n as usize] > 8 as i32 {
        levelMenuInfo.levelScores[n as usize] = 8 as i32
    }
    Com_sprintf(
        levelMenuInfo.levelPicNames[n as usize].as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as usize as i32,
        b"levelshots/%s.tga\x00" as *const u8 as *const libc::c_char,
        map.as_mut_ptr(),
    );
    if trap_R_RegisterShaderNoMip(levelMenuInfo.levelPicNames[n as usize].as_mut_ptr()) == 0 {
        libc::strcpy(
            levelMenuInfo.levelPicNames[n as usize].as_mut_ptr(),
            b"menu/art/unknownmap\x00" as *const u8 as *const libc::c_char,
        );
    }
    levelMenuInfo.item_maps[n as usize].shader = 0 as i32;
    if selectedArenaSet > currentSet {
        levelMenuInfo.item_maps[n as usize].generic.flags |= 0x2000 as i32 as u32
    } else {
        levelMenuInfo.item_maps[n as usize].generic.flags &= !(0x2000 as i32 as u32)
    }
    levelMenuInfo.item_maps[n as usize].generic.flags &= !(0x4000 as i32 as u32);
}

unsafe extern "C" fn UI_SPLevelMenu_SetMenuItems() {
    let mut n: i32 = 0;
    let mut level: i32 = 0;
    let mut arenaInfo: *const libc::c_char = 0 as *const libc::c_char;
    if selectedArenaSet > currentSet {
        selectedArena = -(1 as i32)
    } else if selectedArena == -(1 as i32) {
        selectedArena = 0 as i32
    }
    if selectedArenaSet == trainingTier || selectedArenaSet == finalTier {
        selectedArena = 0 as i32
    }
    if selectedArena != -(1 as i32) {
        trap_Cvar_SetValue(
            b"ui_spSelection\x00" as *const u8 as *const libc::c_char,
            (selectedArenaSet * 4 as i32 + selectedArena) as f32,
        );
    }
    if selectedArenaSet == trainingTier {
        arenaInfo = UI_GetSpecialArenaInfo(b"training\x00" as *const u8 as *const libc::c_char);
        level = atoi(Info_ValueForKey(
            arenaInfo,
            b"num\x00" as *const u8 as *const libc::c_char,
        ));
        UI_SPLevelMenu_SetMenuArena(0 as i32, level, arenaInfo);
        levelMenuInfo.selectedArenaInfo = arenaInfo;
        levelMenuInfo.item_maps[0 as i32 as usize].generic.x = 256 as i32;
        Bitmap_Init(
            &mut *levelMenuInfo
                .item_maps
                .as_mut_ptr()
                .offset(0 as i32 as isize) as *mut _ as *mut menubitmap_s,
        );
        levelMenuInfo.item_maps[0 as i32 as usize].generic.bottom += 32 as i32;
        levelMenuInfo.numMaps = 1 as i32;
        levelMenuInfo.item_maps[1 as i32 as usize].generic.flags |= 0x4000 as i32 as u32;
        levelMenuInfo.item_maps[2 as i32 as usize].generic.flags |= 0x4000 as i32 as u32;
        levelMenuInfo.item_maps[3 as i32 as usize].generic.flags |= 0x4000 as i32 as u32;
        levelMenuInfo.levelPicNames[1 as i32 as usize][0 as i32 as usize] =
            0 as i32 as libc::c_char;
        levelMenuInfo.levelPicNames[2 as i32 as usize][0 as i32 as usize] =
            0 as i32 as libc::c_char;
        levelMenuInfo.levelPicNames[3 as i32 as usize][0 as i32 as usize] =
            0 as i32 as libc::c_char;
        levelMenuInfo.item_maps[1 as i32 as usize].shader = 0 as i32;
        levelMenuInfo.item_maps[2 as i32 as usize].shader = 0 as i32;
        levelMenuInfo.item_maps[3 as i32 as usize].shader = 0 as i32
    } else if selectedArenaSet == finalTier {
        arenaInfo = UI_GetSpecialArenaInfo(b"final\x00" as *const u8 as *const libc::c_char);
        level = atoi(Info_ValueForKey(
            arenaInfo,
            b"num\x00" as *const u8 as *const libc::c_char,
        ));
        UI_SPLevelMenu_SetMenuArena(0 as i32, level, arenaInfo);
        levelMenuInfo.selectedArenaInfo = arenaInfo;
        levelMenuInfo.item_maps[0 as i32 as usize].generic.x = 256 as i32;
        Bitmap_Init(
            &mut *levelMenuInfo
                .item_maps
                .as_mut_ptr()
                .offset(0 as i32 as isize) as *mut _ as *mut menubitmap_s,
        );
        levelMenuInfo.item_maps[0 as i32 as usize].generic.bottom += 32 as i32;
        levelMenuInfo.numMaps = 1 as i32;
        levelMenuInfo.item_maps[1 as i32 as usize].generic.flags |= 0x4000 as i32 as u32;
        levelMenuInfo.item_maps[2 as i32 as usize].generic.flags |= 0x4000 as i32 as u32;
        levelMenuInfo.item_maps[3 as i32 as usize].generic.flags |= 0x4000 as i32 as u32;
        levelMenuInfo.levelPicNames[1 as i32 as usize][0 as i32 as usize] =
            0 as i32 as libc::c_char;
        levelMenuInfo.levelPicNames[2 as i32 as usize][0 as i32 as usize] =
            0 as i32 as libc::c_char;
        levelMenuInfo.levelPicNames[3 as i32 as usize][0 as i32 as usize] =
            0 as i32 as libc::c_char;
        levelMenuInfo.item_maps[1 as i32 as usize].shader = 0 as i32;
        levelMenuInfo.item_maps[2 as i32 as usize].shader = 0 as i32;
        levelMenuInfo.item_maps[3 as i32 as usize].shader = 0 as i32
    } else {
        levelMenuInfo.item_maps[0 as i32 as usize].generic.x = 46 as i32;
        Bitmap_Init(
            &mut *levelMenuInfo
                .item_maps
                .as_mut_ptr()
                .offset(0 as i32 as isize) as *mut _ as *mut menubitmap_s,
        );
        levelMenuInfo.item_maps[0 as i32 as usize].generic.bottom += 18 as i32;
        levelMenuInfo.numMaps = 4 as i32;
        n = 0 as i32;
        while n < 4 as i32 {
            level = selectedArenaSet * 4 as i32 + n;
            arenaInfo = UI_GetArenaInfoByNumber(level);
            UI_SPLevelMenu_SetMenuArena(n, level, arenaInfo);
            n += 1
        }
        if selectedArena != -(1 as i32) {
            levelMenuInfo.selectedArenaInfo =
                UI_GetArenaInfoByNumber(selectedArenaSet * 4 as i32 + selectedArena)
        }
    }
    // enable/disable arrows when they are valid/invalid
    if selectedArenaSet == minTier {
        levelMenuInfo.item_leftarrow.generic.flags |= 0x4000 as i32 as u32 | 0x1000 as i32 as u32
    } else {
        levelMenuInfo.item_leftarrow.generic.flags &= !(0x4000 as i32 as u32 | 0x1000 as i32 as u32)
    }
    if selectedArenaSet == maxTier {
        levelMenuInfo.item_rightarrow.generic.flags |= 0x4000 as i32 as u32 | 0x1000 as i32 as u32
    } else {
        levelMenuInfo.item_rightarrow.generic.flags &=
            !(0x4000 as i32 as u32 | 0x1000 as i32 as u32)
    }
    UI_SPLevelMenu_SetBots();
}
/*
=================
UI_SPLevelMenu_ResetEvent
=================
*/

unsafe extern "C" fn UI_SPLevelMenu_ResetDraw() {
    UI_DrawProportionalString(
        640 as i32 / 2 as i32,
        356 as i32 + 27 as i32 * 0 as i32,
        b"WARNING: This resets all of the\x00" as *const u8 as *const libc::c_char,
        0x1 as i32 | 0x10 as i32,
        color_yellow.as_mut_ptr(),
    );
    UI_DrawProportionalString(
        640 as i32 / 2 as i32,
        356 as i32 + 27 as i32 * 1 as i32,
        b"single player game variables.\x00" as *const u8 as *const libc::c_char,
        0x1 as i32 | 0x10 as i32,
        color_yellow.as_mut_ptr(),
    );
    UI_DrawProportionalString(
        640 as i32 / 2 as i32,
        356 as i32 + 27 as i32 * 2 as i32,
        b"Do this only if you want to\x00" as *const u8 as *const libc::c_char,
        0x1 as i32 | 0x10 as i32,
        color_yellow.as_mut_ptr(),
    );
    UI_DrawProportionalString(
        640 as i32 / 2 as i32,
        356 as i32 + 27 as i32 * 3 as i32,
        b"start over from the beginning.\x00" as *const u8 as *const libc::c_char,
        0x1 as i32 | 0x10 as i32,
        color_yellow.as_mut_ptr(),
    );
}

unsafe extern "C" fn UI_SPLevelMenu_ResetAction(mut result: qboolean) {
    if result as u64 == 0 {
        return;
    }
    // clear game variables
    UI_NewGame();
    if !UI_GetSpecialArenaInfo(b"training\x00" as *const u8 as *const libc::c_char).is_null() {
        trap_Cvar_SetValue(
            b"ui_spSelection\x00" as *const u8 as *const libc::c_char,
            -(4 as i32) as f32,
        );
    } else {
        trap_Cvar_SetValue(
            b"ui_spSelection\x00" as *const u8 as *const libc::c_char,
            0 as i32 as f32,
        );
    }
    // make the level select menu re-initialize
    UI_PopMenu();
    UI_SPLevelMenu();
}

unsafe extern "C" fn UI_SPLevelMenu_ResetEvent(mut _ptr: *mut libc::c_void, mut event: i32) {
    if event != 3 as i32 {
        return;
    }
    UI_ConfirmMenu(
        b"RESET GAME?\x00" as *const u8 as *const libc::c_char,
        Some(UI_SPLevelMenu_ResetDraw as unsafe extern "C" fn() -> ()),
        Some(UI_SPLevelMenu_ResetAction as unsafe extern "C" fn(_: qboolean) -> ()),
    );
}
/*
=================
UI_SPLevelMenu_LevelEvent
=================
*/

unsafe extern "C" fn UI_SPLevelMenu_LevelEvent(mut ptr: *mut libc::c_void, mut notification: i32) {
    if notification != 3 as i32 {
        return;
    }
    if selectedArenaSet == trainingTier || selectedArenaSet == finalTier {
        return;
    }
    selectedArena = (*(ptr as *mut menucommon_s)).id - 11 as i32;
    levelMenuInfo.selectedArenaInfo =
        UI_GetArenaInfoByNumber(selectedArenaSet * 4 as i32 + selectedArena);
    UI_SPLevelMenu_SetBots();
    trap_Cvar_SetValue(
        b"ui_spSelection\x00" as *const u8 as *const libc::c_char,
        (selectedArenaSet * 4 as i32 + selectedArena) as f32,
    );
}
/*
=================
UI_SPLevelMenu_LeftArrowEvent
=================
*/

unsafe extern "C" fn UI_SPLevelMenu_LeftArrowEvent(
    mut _ptr: *mut libc::c_void,
    mut notification: i32,
) {
    if notification != 3 as i32 {
        return;
    }
    if selectedArenaSet == minTier {
        return;
    }
    selectedArenaSet -= 1;
    UI_SPLevelMenu_SetMenuItems();
}
/*
=================
UI_SPLevelMenu_RightArrowEvent
=================
*/

unsafe extern "C" fn UI_SPLevelMenu_RightArrowEvent(
    mut _ptr: *mut libc::c_void,
    mut notification: i32,
) {
    if notification != 3 as i32 {
        return;
    }
    if selectedArenaSet == maxTier {
        return;
    }
    selectedArenaSet += 1;
    UI_SPLevelMenu_SetMenuItems();
}
/*
=================
UI_SPLevelMenu_PlayerEvent
=================
*/

unsafe extern "C" fn UI_SPLevelMenu_PlayerEvent(
    mut _ptr: *mut libc::c_void,
    mut notification: i32,
) {
    if notification != 3 as i32 {
        return;
    }
    UI_PlayerSettingsMenu();
}
/*
=================
UI_SPLevelMenu_AwardEvent
=================
*/

unsafe extern "C" fn UI_SPLevelMenu_AwardEvent(mut ptr: *mut libc::c_void, mut notification: i32) {
    let mut n: i32 = 0;
    if notification != 3 as i32 {
        return;
    }
    n = (*(ptr as *mut menucommon_s)).id - 17 as i32;
    trap_S_StartLocalSound(levelMenuInfo.awardSounds[n as usize], CHAN_ANNOUNCER as i32);
}
/*
=================
UI_SPLevelMenu_NextEvent
=================
*/

unsafe extern "C" fn UI_SPLevelMenu_NextEvent(mut _ptr: *mut libc::c_void, mut notification: i32) {
    if notification != 3 as i32 {
        return;
    }
    if selectedArenaSet > currentSet {
        return;
    }
    if selectedArena == -(1 as i32) {
        selectedArena = 0 as i32
    }
    UI_SPSkillMenu(levelMenuInfo.selectedArenaInfo);
}
/*
=================
UI_SPLevelMenu_BackEvent
=================
*/

unsafe extern "C" fn UI_SPLevelMenu_BackEvent(mut _ptr: *mut libc::c_void, mut notification: i32) {
    if notification != 3 as i32 {
        return;
    }
    if selectedArena == -(1 as i32) {
        selectedArena = 0 as i32
    }
    UI_PopMenu();
}
/*
=================
UI_SPLevelMenu_CustomEvent
=================
*/

unsafe extern "C" fn UI_SPLevelMenu_CustomEvent(
    mut _ptr: *mut libc::c_void,
    mut notification: i32,
) {
    if notification != 3 as i32 {
        return;
    }
    UI_StartServerMenu(qfalse);
}
/*
=================
UI_SPLevelMenu_MenuDraw
=================
*/

unsafe extern "C" fn UI_SPLevelMenu_MenuDraw() {
    let mut n: i32 = 0;
    let mut i: i32 = 0;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut color: vec4_t = [0.; 4];
    let mut level: i32 = 0;
    //	int				fraglimit;
    let mut pad: i32 = 0;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut string: [libc::c_char; 64] = [0; 64];
    if levelMenuInfo.reinit as u64 != 0 {
        UI_PopMenu();
        UI_SPLevelMenu();
        return;
    }
    // draw player name
    trap_Cvar_VariableStringBuffer(
        b"name\x00" as *const u8 as *const libc::c_char,
        string.as_mut_ptr(),
        32 as i32,
    );
    Q_CleanStr(string.as_mut_ptr());
    UI_DrawProportionalString(
        320 as i32,
        314 as i32,
        string.as_mut_ptr(),
        0x1 as i32 | 0x10 as i32,
        color_orange.as_mut_ptr(),
    );
    // check for model changes
    trap_Cvar_VariableStringBuffer(
        b"model\x00" as *const u8 as *const libc::c_char,
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as usize as i32,
    );
    if Q_stricmp(buf.as_mut_ptr(), levelMenuInfo.playerModel.as_mut_ptr()) != 0 as i32 {
        Q_strncpyz(
            levelMenuInfo.playerModel.as_mut_ptr(),
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 64]>() as usize as i32,
        );
        PlayerIcon(
            levelMenuInfo.playerModel.as_mut_ptr(),
            levelMenuInfo.playerPicName.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 64]>() as usize as i32,
        );
        levelMenuInfo.item_player.shader = 0 as i32
    }
    // standard menu drawing
    Menu_Draw(&mut levelMenuInfo.menu as *mut _ as *mut _tag_menuframework);
    // draw player award levels
    y = 314 as i32 + 26 as i32;
    i = 0 as i32;
    n = 0 as i32;
    while n < 6 as i32 {
        level = levelMenuInfo.awardLevels[n as usize];
        if level > 0 as i32 {
            if i & 1 as i32 != 0 {
                x = 224 as i32 - (i - 1 as i32) / 2 as i32 * (48 as i32 + 16 as i32)
            } else {
                x = 368 as i32 + i / 2 as i32 * (48 as i32 + 16 as i32)
            }
            i += 1;
            if !(level == 1 as i32) {
                if level >= 1000000 as i32 {
                    Com_sprintf(
                        string.as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 64]>() as usize as i32,
                        b"%im\x00" as *const u8 as *const libc::c_char,
                        level / 1000000 as i32,
                    );
                } else if level >= 1000 as i32 {
                    Com_sprintf(
                        string.as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 64]>() as usize as i32,
                        b"%ik\x00" as *const u8 as *const libc::c_char,
                        level / 1000 as i32,
                    );
                } else {
                    Com_sprintf(
                        string.as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 64]>() as usize as i32,
                        b"%i\x00" as *const u8 as *const libc::c_char,
                        level,
                    );
                }
                UI_DrawString(
                    x + 24 as i32,
                    y + 48 as i32,
                    string.as_mut_ptr(),
                    0x1 as i32,
                    color_yellow.as_mut_ptr(),
                );
            }
        }
        n += 1
    }
    UI_DrawProportionalString(
        18 as i32,
        38 as i32,
        va(
            b"Tier %i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            selectedArenaSet + 1 as i32,
        ),
        0 as i32 | 0x10 as i32,
        color_orange.as_mut_ptr(),
    );
    n = 0 as i32;
    while n < levelMenuInfo.numMaps {
        x = levelMenuInfo.item_maps[n as usize].generic.x;
        y = levelMenuInfo.item_maps[n as usize].generic.y;
        UI_FillRect(
            x as f32,
            (y + 96 as i32) as f32,
            128 as i32 as f32,
            18 as i32 as f32,
            color_black.as_mut_ptr(),
        );
        n += 1
    }
    if selectedArenaSet > currentSet {
        UI_DrawProportionalString(
            320 as i32,
            216 as i32,
            b"ACCESS DENIED\x00" as *const u8 as *const libc::c_char,
            0x1 as i32 | 0x20 as i32,
            color_red.as_mut_ptr(),
        );
        return;
    }
    // show levelshots for levels of current tier
    color[0 as i32 as usize] = color_white[0 as i32 as usize];
    color[1 as i32 as usize] = color_white[1 as i32 as usize];
    color[2 as i32 as usize] = color_white[2 as i32 as usize];
    color[3 as i32 as usize] = color_white[3 as i32 as usize];
    color[3 as i32 as usize] =
        (0.5f64 + 0.5f64 * crate::stdlib::sin((uis.realtime / 75 as i32) as f64)) as vec_t;
    n = 0 as i32;
    while n < levelMenuInfo.numMaps {
        x = levelMenuInfo.item_maps[n as usize].generic.x;
        y = levelMenuInfo.item_maps[n as usize].generic.y;
        UI_DrawString(
            x + 64 as i32,
            y + 96 as i32,
            levelMenuInfo.levelNames[n as usize].as_mut_ptr(),
            0x1 as i32 | 0x10 as i32,
            color_orange.as_mut_ptr(),
        );
        if levelMenuInfo.levelScores[n as usize] == 1 as i32 {
            UI_DrawHandlePic(
                x as f32,
                y as f32,
                128 as i32 as f32,
                96 as i32 as f32,
                levelMenuInfo.levelCompletePic
                    [(levelMenuInfo.levelScoresSkill[n as usize] - 1 as i32) as usize],
            );
        }
        if n == selectedArena {
            if Menu_ItemAtCursor(&mut levelMenuInfo.menu as *mut _ as *mut _tag_menuframework)
                == &mut *levelMenuInfo.item_maps.as_mut_ptr().offset(n as isize)
                    as *mut menubitmap_s as *mut libc::c_void
            {
                trap_R_SetColor(color.as_mut_ptr());
            }
            UI_DrawHandlePic(
                (x - 1 as i32) as f32,
                (y - 1 as i32) as f32,
                130 as i32 as f32,
                (130 as i32 - 14 as i32) as f32,
                levelMenuInfo.levelSelectedPic,
            );
            trap_R_SetColor(0 as *const f32);
        } else if Menu_ItemAtCursor(&mut levelMenuInfo.menu as *mut _ as *mut _tag_menuframework)
            == &mut *levelMenuInfo.item_maps.as_mut_ptr().offset(n as isize) as *mut menubitmap_s
                as *mut libc::c_void
        {
            trap_R_SetColor(color.as_mut_ptr());
            UI_DrawHandlePic(
                (x - 31 as i32) as f32,
                (y - 30 as i32) as f32,
                256 as i32 as f32,
                (256 as i32 - 27 as i32) as f32,
                levelMenuInfo.levelFocusPic,
            );
            trap_R_SetColor(0 as *const f32);
        }
        n += 1
    }
    // show map name and long name of selected level
    y = 192 as i32;
    Q_strncpyz(
        buf.as_mut_ptr(),
        Info_ValueForKey(
            levelMenuInfo.selectedArenaInfo,
            b"map\x00" as *const u8 as *const libc::c_char,
        ),
        20 as i32,
    );
    Q_strupr(buf.as_mut_ptr());
    Com_sprintf(
        string.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as usize as i32,
        b"%s: %s\x00" as *const u8 as *const libc::c_char,
        buf.as_mut_ptr(),
        Info_ValueForKey(
            levelMenuInfo.selectedArenaInfo,
            b"longname\x00" as *const u8 as *const libc::c_char,
        ),
    );
    UI_DrawProportionalString(
        320 as i32,
        y,
        string.as_mut_ptr(),
        0x1 as i32 | 0x10 as i32,
        color_orange.as_mut_ptr(),
    );
    //	fraglimit = atoi( Info_ValueForKey( levelMenuInfo.selectedArenaInfo, "fraglimit" ) );
    //	UI_DrawString( 18, 212, va("Frags %i", fraglimit) , UI_LEFT|UI_SMALLFONT, color_orange );
    // draw bot opponents
    y += 24 as i32;
    pad = (7 as i32 - levelMenuInfo.numBots) * (64 as i32 + 26 as i32) / 2 as i32;
    n = 0 as i32;
    while n < levelMenuInfo.numBots {
        x = 18 as i32 + pad + (64 as i32 + 26 as i32) * n;
        if levelMenuInfo.botPics[n as usize] != 0 {
            UI_DrawHandlePic(
                x as f32,
                y as f32,
                64 as i32 as f32,
                64 as i32 as f32,
                levelMenuInfo.botPics[n as usize],
            );
        } else {
            UI_FillRect(
                x as f32,
                y as f32,
                64 as i32 as f32,
                64 as i32 as f32,
                color_black.as_mut_ptr(),
            );
            UI_DrawProportionalString(
                x + 22 as i32,
                y + 18 as i32,
                b"?\x00" as *const u8 as *const libc::c_char,
                0x20 as i32,
                color_orange.as_mut_ptr(),
            );
        }
        UI_DrawString(
            x,
            y + 64 as i32,
            levelMenuInfo.botNames[n as usize].as_mut_ptr(),
            0x10 as i32 | 0 as i32,
            color_orange.as_mut_ptr(),
        );
        n += 1
    }
}
/*
=================
UI_SPLevelMenu_Cache
=================
*/
#[no_mangle]

pub unsafe extern "C" fn UI_SPLevelMenu_Cache() {
    let mut n: i32 = 0;
    trap_R_RegisterShaderNoMip(b"menu/art/maps_select\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/maps_selected\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/narrow_0\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/narrow_1\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/unknownmap\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/level_complete1\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/level_complete2\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/level_complete3\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/level_complete4\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/level_complete5\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/back_0\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/back_1\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/fight_0\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/fight_1\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/reset_0\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/reset_1\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/skirmish_0\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/skirmish_1\x00" as *const u8 as *const libc::c_char);
    n = 0 as i32;
    while n < 6 as i32 {
        trap_R_RegisterShaderNoMip(*ui_medalPicNames.as_mut_ptr().offset(n as isize));
        levelMenuInfo.awardSounds[n as usize] =
            trap_S_RegisterSound(*ui_medalSounds.as_mut_ptr().offset(n as isize), qfalse);
        n += 1
    }
    levelMenuInfo.levelSelectedPic = trap_R_RegisterShaderNoMip(
        b"menu/art/maps_selected\x00" as *const u8 as *const libc::c_char,
    );
    levelMenuInfo.levelFocusPic =
        trap_R_RegisterShaderNoMip(b"menu/art/maps_select\x00" as *const u8 as *const libc::c_char);
    levelMenuInfo.levelCompletePic[0 as i32 as usize] = trap_R_RegisterShaderNoMip(
        b"menu/art/level_complete1\x00" as *const u8 as *const libc::c_char,
    );
    levelMenuInfo.levelCompletePic[1 as i32 as usize] = trap_R_RegisterShaderNoMip(
        b"menu/art/level_complete2\x00" as *const u8 as *const libc::c_char,
    );
    levelMenuInfo.levelCompletePic[2 as i32 as usize] = trap_R_RegisterShaderNoMip(
        b"menu/art/level_complete3\x00" as *const u8 as *const libc::c_char,
    );
    levelMenuInfo.levelCompletePic[3 as i32 as usize] = trap_R_RegisterShaderNoMip(
        b"menu/art/level_complete4\x00" as *const u8 as *const libc::c_char,
    );
    levelMenuInfo.levelCompletePic[4 as i32 as usize] = trap_R_RegisterShaderNoMip(
        b"menu/art/level_complete5\x00" as *const u8 as *const libc::c_char,
    );
}
/*
=================
UI_SPLevelMenu_Init
=================
*/

unsafe extern "C" fn UI_SPLevelMenu_Init() {
    let mut skill: i32 = 0;
    let mut n: i32 = 0;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut count: i32 = 0;
    let mut buf: [libc::c_char; 64] = [0; 64];
    skill = trap_Cvar_VariableValue(b"g_spSkill\x00" as *const u8 as *const libc::c_char) as i32;
    if skill < 1 as i32 || skill > 5 as i32 {
        trap_Cvar_Set(
            b"g_spSkill\x00" as *const u8 as *const libc::c_char,
            b"2\x00" as *const u8 as *const libc::c_char,
        );
    }
    crate::stdlib::memset(
        &mut levelMenuInfo as *mut levelMenuInfo_t as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<levelMenuInfo_t>() as usize,
    );
    levelMenuInfo.menu.fullscreen = qtrue;
    levelMenuInfo.menu.wrapAround = qtrue;
    levelMenuInfo.menu.draw = Some(UI_SPLevelMenu_MenuDraw as unsafe extern "C" fn() -> ());
    UI_SPLevelMenu_Cache();
    levelMenuInfo.item_banner.generic.type_0 = 10 as i32;
    levelMenuInfo.item_banner.generic.x = 320 as i32;
    levelMenuInfo.item_banner.generic.y = 16 as i32;
    levelMenuInfo.item_banner.string =
        b"CHOOSE LEVEL\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    levelMenuInfo.item_banner.color = color_red.as_mut_ptr();
    levelMenuInfo.item_banner.style = 0x1 as i32;
    levelMenuInfo.item_leftarrow.generic.type_0 = 6 as i32;
    levelMenuInfo.item_leftarrow.generic.name =
        b"menu/art/narrow_0\x00" as *const u8 as *const libc::c_char;
    levelMenuInfo.item_leftarrow.generic.flags = 0x4 as i32 as u32 | 0x100 as i32 as u32;
    levelMenuInfo.item_leftarrow.generic.x = 18 as i32;
    levelMenuInfo.item_leftarrow.generic.y = 64 as i32;
    levelMenuInfo.item_leftarrow.generic.callback = Some(
        UI_SPLevelMenu_LeftArrowEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> (),
    );
    levelMenuInfo.item_leftarrow.generic.id = 10 as i32;
    levelMenuInfo.item_leftarrow.width = 16 as i32;
    levelMenuInfo.item_leftarrow.height = 114 as i32;
    levelMenuInfo.item_leftarrow.focuspic =
        b"menu/art/narrow_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    levelMenuInfo.item_maps[0 as i32 as usize].generic.type_0 = 6 as i32;
    levelMenuInfo.item_maps[0 as i32 as usize].generic.name =
        levelMenuInfo.levelPicNames[0 as i32 as usize].as_mut_ptr();
    levelMenuInfo.item_maps[0 as i32 as usize].generic.flags = 0x4 as i32 as u32;
    levelMenuInfo.item_maps[0 as i32 as usize].generic.x = 46 as i32;
    levelMenuInfo.item_maps[0 as i32 as usize].generic.y = 64 as i32;
    levelMenuInfo.item_maps[0 as i32 as usize].generic.id = 11 as i32;
    levelMenuInfo.item_maps[0 as i32 as usize].generic.callback =
        Some(UI_SPLevelMenu_LevelEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    levelMenuInfo.item_maps[0 as i32 as usize].width = 128 as i32;
    levelMenuInfo.item_maps[0 as i32 as usize].height = 96 as i32;
    levelMenuInfo.item_maps[1 as i32 as usize].generic.type_0 = 6 as i32;
    levelMenuInfo.item_maps[1 as i32 as usize].generic.name =
        levelMenuInfo.levelPicNames[1 as i32 as usize].as_mut_ptr();
    levelMenuInfo.item_maps[1 as i32 as usize].generic.flags = 0x4 as i32 as u32;
    levelMenuInfo.item_maps[1 as i32 as usize].generic.x = 186 as i32;
    levelMenuInfo.item_maps[1 as i32 as usize].generic.y = 64 as i32;
    levelMenuInfo.item_maps[1 as i32 as usize].generic.id = 12 as i32;
    levelMenuInfo.item_maps[1 as i32 as usize].generic.callback =
        Some(UI_SPLevelMenu_LevelEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    levelMenuInfo.item_maps[1 as i32 as usize].width = 128 as i32;
    levelMenuInfo.item_maps[1 as i32 as usize].height = 96 as i32;
    levelMenuInfo.item_maps[2 as i32 as usize].generic.type_0 = 6 as i32;
    levelMenuInfo.item_maps[2 as i32 as usize].generic.name =
        levelMenuInfo.levelPicNames[2 as i32 as usize].as_mut_ptr();
    levelMenuInfo.item_maps[2 as i32 as usize].generic.flags = 0x4 as i32 as u32;
    levelMenuInfo.item_maps[2 as i32 as usize].generic.x = 326 as i32;
    levelMenuInfo.item_maps[2 as i32 as usize].generic.y = 64 as i32;
    levelMenuInfo.item_maps[2 as i32 as usize].generic.id = 13 as i32;
    levelMenuInfo.item_maps[2 as i32 as usize].generic.callback =
        Some(UI_SPLevelMenu_LevelEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    levelMenuInfo.item_maps[2 as i32 as usize].width = 128 as i32;
    levelMenuInfo.item_maps[2 as i32 as usize].height = 96 as i32;
    levelMenuInfo.item_maps[3 as i32 as usize].generic.type_0 = 6 as i32;
    levelMenuInfo.item_maps[3 as i32 as usize].generic.name =
        levelMenuInfo.levelPicNames[3 as i32 as usize].as_mut_ptr();
    levelMenuInfo.item_maps[3 as i32 as usize].generic.flags = 0x4 as i32 as u32;
    levelMenuInfo.item_maps[3 as i32 as usize].generic.x = 466 as i32;
    levelMenuInfo.item_maps[3 as i32 as usize].generic.y = 64 as i32;
    levelMenuInfo.item_maps[3 as i32 as usize].generic.id = 14 as i32;
    levelMenuInfo.item_maps[3 as i32 as usize].generic.callback =
        Some(UI_SPLevelMenu_LevelEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    levelMenuInfo.item_maps[3 as i32 as usize].width = 128 as i32;
    levelMenuInfo.item_maps[3 as i32 as usize].height = 96 as i32;
    levelMenuInfo.item_rightarrow.generic.type_0 = 6 as i32;
    levelMenuInfo.item_rightarrow.generic.name =
        b"menu/art/narrow_0\x00" as *const u8 as *const libc::c_char;
    levelMenuInfo.item_rightarrow.generic.flags = 0x4 as i32 as u32 | 0x100 as i32 as u32;
    levelMenuInfo.item_rightarrow.generic.x = 606 as i32;
    levelMenuInfo.item_rightarrow.generic.y = 64 as i32;
    levelMenuInfo.item_rightarrow.generic.callback = Some(
        UI_SPLevelMenu_RightArrowEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> (),
    );
    levelMenuInfo.item_rightarrow.generic.id = 15 as i32;
    levelMenuInfo.item_rightarrow.width = -(16 as i32);
    levelMenuInfo.item_rightarrow.height = 114 as i32;
    levelMenuInfo.item_rightarrow.focuspic =
        b"menu/art/narrow_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    trap_Cvar_VariableStringBuffer(
        b"model\x00" as *const u8 as *const libc::c_char,
        levelMenuInfo.playerModel.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as usize as i32,
    );
    PlayerIcon(
        levelMenuInfo.playerModel.as_mut_ptr(),
        levelMenuInfo.playerPicName.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as usize as i32,
    );
    levelMenuInfo.item_player.generic.type_0 = 6 as i32;
    levelMenuInfo.item_player.generic.name = levelMenuInfo.playerPicName.as_mut_ptr();
    levelMenuInfo.item_player.generic.flags = 0x4 as i32 as u32 | 0x800 as i32 as u32;
    levelMenuInfo.item_player.generic.x = 288 as i32;
    levelMenuInfo.item_player.generic.y = 314 as i32 + 26 as i32;
    levelMenuInfo.item_player.generic.id = 16 as i32;
    levelMenuInfo.item_player.generic.callback = Some(
        UI_SPLevelMenu_PlayerEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> (),
    );
    levelMenuInfo.item_player.width = 64 as i32;
    levelMenuInfo.item_player.height = 64 as i32;
    n = 0 as i32;
    while n < 6 as i32 {
        levelMenuInfo.awardLevels[n as usize] = UI_GetAwardLevel(n);
        n += 1
    }
    levelMenuInfo.awardLevels[AWARD_FRAGS as i32 as usize] =
        100 as i32 * (levelMenuInfo.awardLevels[AWARD_FRAGS as i32 as usize] / 100 as i32);
    y = 314 as i32 + 26 as i32;
    count = 0 as i32;
    n = 0 as i32;
    while n < 6 as i32 {
        if levelMenuInfo.awardLevels[n as usize] != 0 {
            if count & 1 as i32 != 0 {
                x = 224 as i32 - (count - 1 as i32) / 2 as i32 * (48 as i32 + 16 as i32)
            } else {
                x = 368 as i32 + count / 2 as i32 * (48 as i32 + 16 as i32)
            }
            levelMenuInfo.item_awards[count as usize].generic.type_0 = 6 as i32;
            levelMenuInfo.item_awards[count as usize].generic.name =
                *ui_medalPicNames.as_mut_ptr().offset(n as isize);
            levelMenuInfo.item_awards[count as usize].generic.flags =
                0x4 as i32 as u32 | 0x100000 as i32 as u32 | 0x800 as i32 as u32;
            levelMenuInfo.item_awards[count as usize].generic.x = x;
            levelMenuInfo.item_awards[count as usize].generic.y = y;
            levelMenuInfo.item_awards[count as usize].generic.id = 17 as i32 + n;
            levelMenuInfo.item_awards[count as usize].generic.callback = Some(
                UI_SPLevelMenu_AwardEvent
                    as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> (),
            );
            levelMenuInfo.item_awards[count as usize].width = 48 as i32;
            levelMenuInfo.item_awards[count as usize].height = 48 as i32;
            count += 1
        }
        n += 1
    }
    levelMenuInfo.item_back.generic.type_0 = 6 as i32;
    levelMenuInfo.item_back.generic.name =
        b"menu/art/back_0\x00" as *const u8 as *const libc::c_char;
    levelMenuInfo.item_back.generic.flags = 0x4 as i32 as u32 | 0x100 as i32 as u32;
    levelMenuInfo.item_back.generic.x = 0 as i32;
    levelMenuInfo.item_back.generic.y = 480 as i32 - 64 as i32;
    levelMenuInfo.item_back.generic.callback =
        Some(UI_SPLevelMenu_BackEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    levelMenuInfo.item_back.generic.id = 23 as i32;
    levelMenuInfo.item_back.width = 128 as i32;
    levelMenuInfo.item_back.height = 64 as i32;
    levelMenuInfo.item_back.focuspic =
        b"menu/art/back_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    levelMenuInfo.item_reset.generic.type_0 = 6 as i32;
    levelMenuInfo.item_reset.generic.name =
        b"menu/art/reset_0\x00" as *const u8 as *const libc::c_char;
    levelMenuInfo.item_reset.generic.flags = 0x4 as i32 as u32 | 0x100 as i32 as u32;
    levelMenuInfo.item_reset.generic.x = 170 as i32;
    levelMenuInfo.item_reset.generic.y = 480 as i32 - 64 as i32;
    levelMenuInfo.item_reset.generic.callback =
        Some(UI_SPLevelMenu_ResetEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    levelMenuInfo.item_reset.generic.id = 24 as i32;
    levelMenuInfo.item_reset.width = 128 as i32;
    levelMenuInfo.item_reset.height = 64 as i32;
    levelMenuInfo.item_reset.focuspic =
        b"menu/art/reset_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    levelMenuInfo.item_custom.generic.type_0 = 6 as i32;
    levelMenuInfo.item_custom.generic.name =
        b"menu/art/skirmish_0\x00" as *const u8 as *const libc::c_char;
    levelMenuInfo.item_custom.generic.flags = 0x4 as i32 as u32 | 0x100 as i32 as u32;
    levelMenuInfo.item_custom.generic.x = 342 as i32;
    levelMenuInfo.item_custom.generic.y = 480 as i32 - 64 as i32;
    levelMenuInfo.item_custom.generic.callback = Some(
        UI_SPLevelMenu_CustomEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> (),
    );
    levelMenuInfo.item_custom.generic.id = 25 as i32;
    levelMenuInfo.item_custom.width = 128 as i32;
    levelMenuInfo.item_custom.height = 64 as i32;
    levelMenuInfo.item_custom.focuspic =
        b"menu/art/skirmish_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    levelMenuInfo.item_next.generic.type_0 = 6 as i32;
    levelMenuInfo.item_next.generic.name =
        b"menu/art/fight_0\x00" as *const u8 as *const libc::c_char;
    levelMenuInfo.item_next.generic.flags = 0x10 as i32 as u32 | 0x100 as i32 as u32;
    levelMenuInfo.item_next.generic.x = 640 as i32;
    levelMenuInfo.item_next.generic.y = 480 as i32 - 64 as i32;
    levelMenuInfo.item_next.generic.callback =
        Some(UI_SPLevelMenu_NextEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    levelMenuInfo.item_next.generic.id = 26 as i32;
    levelMenuInfo.item_next.width = 128 as i32;
    levelMenuInfo.item_next.height = 64 as i32;
    levelMenuInfo.item_next.focuspic =
        b"menu/art/fight_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    levelMenuInfo.item_null.generic.type_0 = 6 as i32;
    levelMenuInfo.item_null.generic.flags =
        0x4 as i32 as u32 | 0x800 as i32 as u32 | 0x100000 as i32 as u32;
    levelMenuInfo.item_null.generic.x = 0 as i32;
    levelMenuInfo.item_null.generic.y = 0 as i32;
    levelMenuInfo.item_null.width = 640 as i32;
    levelMenuInfo.item_null.height = 480 as i32;
    Menu_AddItem(
        &mut levelMenuInfo.menu as *mut _ as *mut _tag_menuframework,
        &mut levelMenuInfo.item_banner as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut levelMenuInfo.menu as *mut _ as *mut _tag_menuframework,
        &mut levelMenuInfo.item_leftarrow as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut levelMenuInfo.menu as *mut _ as *mut _tag_menuframework,
        &mut *levelMenuInfo
            .item_maps
            .as_mut_ptr()
            .offset(0 as i32 as isize) as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut levelMenuInfo.menu as *mut _ as *mut _tag_menuframework,
        &mut *levelMenuInfo
            .item_maps
            .as_mut_ptr()
            .offset(1 as i32 as isize) as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut levelMenuInfo.menu as *mut _ as *mut _tag_menuframework,
        &mut *levelMenuInfo
            .item_maps
            .as_mut_ptr()
            .offset(2 as i32 as isize) as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut levelMenuInfo.menu as *mut _ as *mut _tag_menuframework,
        &mut *levelMenuInfo
            .item_maps
            .as_mut_ptr()
            .offset(3 as i32 as isize) as *mut menubitmap_s as *mut libc::c_void,
    );
    levelMenuInfo.item_maps[0 as i32 as usize].generic.bottom += 18 as i32;
    levelMenuInfo.item_maps[1 as i32 as usize].generic.bottom += 18 as i32;
    levelMenuInfo.item_maps[2 as i32 as usize].generic.bottom += 18 as i32;
    levelMenuInfo.item_maps[3 as i32 as usize].generic.bottom += 18 as i32;
    Menu_AddItem(
        &mut levelMenuInfo.menu as *mut _ as *mut _tag_menuframework,
        &mut levelMenuInfo.item_rightarrow as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut levelMenuInfo.menu as *mut _ as *mut _tag_menuframework,
        &mut levelMenuInfo.item_player as *mut menubitmap_s as *mut libc::c_void,
    );
    n = 0 as i32;
    while n < count {
        Menu_AddItem(
            &mut levelMenuInfo.menu as *mut _ as *mut _tag_menuframework,
            &mut *levelMenuInfo.item_awards.as_mut_ptr().offset(n as isize) as *mut menubitmap_s
                as *mut libc::c_void,
        );
        n += 1
    }
    Menu_AddItem(
        &mut levelMenuInfo.menu as *mut _ as *mut _tag_menuframework,
        &mut levelMenuInfo.item_back as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut levelMenuInfo.menu as *mut _ as *mut _tag_menuframework,
        &mut levelMenuInfo.item_reset as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut levelMenuInfo.menu as *mut _ as *mut _tag_menuframework,
        &mut levelMenuInfo.item_custom as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut levelMenuInfo.menu as *mut _ as *mut _tag_menuframework,
        &mut levelMenuInfo.item_next as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut levelMenuInfo.menu as *mut _ as *mut _tag_menuframework,
        &mut levelMenuInfo.item_null as *mut menubitmap_s as *mut libc::c_void,
    );
    trap_Cvar_VariableStringBuffer(
        b"ui_spSelection\x00" as *const u8 as *const libc::c_char,
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as usize as i32,
    );
    if *buf.as_mut_ptr() != 0 {
        n = atoi(buf.as_mut_ptr());
        selectedArenaSet = n / 4 as i32;
        selectedArena = n % 4 as i32
    } else {
        selectedArenaSet = currentSet;
        selectedArena = currentGame
    }
    UI_SPLevelMenu_SetMenuItems();
}
/*
=================
UI_SPLevelMenu
=================
*/
#[no_mangle]

pub unsafe extern "C" fn UI_SPLevelMenu() {
    let mut level: i32 = 0;
    let mut trainingLevel: i32 = 0;
    let mut arenaInfo: *const libc::c_char = 0 as *const libc::c_char;
    trainingTier = -(1 as i32);
    arenaInfo = UI_GetSpecialArenaInfo(b"training\x00" as *const u8 as *const libc::c_char);
    if !arenaInfo.is_null() {
        minTier = trainingTier;
        trainingLevel = atoi(Info_ValueForKey(
            arenaInfo,
            b"num\x00" as *const u8 as *const libc::c_char,
        ))
    } else {
        minTier = 0 as i32;
        trainingLevel = -(2 as i32)
    }
    finalTier = UI_GetNumSPTiers();
    arenaInfo = UI_GetSpecialArenaInfo(b"final\x00" as *const u8 as *const libc::c_char);
    if !arenaInfo.is_null() {
        maxTier = finalTier
    } else {
        maxTier = finalTier - 1 as i32;
        if maxTier < minTier {
            maxTier = minTier
        }
    }
    level = UI_GetCurrentGame();
    if level == -(1 as i32) {
        level = UI_GetNumSPArenas() - 1 as i32;
        if maxTier == finalTier {
            level += 1
        }
    }
    if level == trainingLevel {
        currentSet = -(1 as i32);
        currentGame = 0 as i32
    } else {
        currentSet = level / 4 as i32;
        currentGame = level % 4 as i32
    }
    UI_SPLevelMenu_Init();
    UI_PushMenu(&mut levelMenuInfo.menu as *mut _ as *mut _tag_menuframework);
    Menu_SetCursorToItem(
        &mut levelMenuInfo.menu as *mut _ as *mut _tag_menuframework,
        &mut levelMenuInfo.item_next as *mut menubitmap_s as *mut libc::c_void,
    );
}
/*
=================
UI_SPLevelMenu_f
=================
*/
#[no_mangle]

pub unsafe extern "C" fn UI_SPLevelMenu_f() {
    trap_Key_SetCatcher(0x2 as i32);
    uis.menusp = 0 as i32;
    UI_SPLevelMenu();
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
//
// ui_serverinfo.c
//
//
// ui_video.c
//
//
// ui_players.c
//
//FIXME ripped from cg_local.h
// time when ->oldFrame was exactly on
// time when ->frame will be exactly on
// may include ANIM_TOGGLEBIT
// time when the first frame of the animation will be exact
// model info
// true if legs yaw is always the same as torso yaw
// true if torso never changes yaw
// currently in use drawing parms
// animation vars
//
// ui_atoms.c
//
//
// ui_spLevel.c
//
/*
=================
UI_SPLevelMenu_ReInit
=================
*/
#[no_mangle]

pub unsafe extern "C" fn UI_SPLevelMenu_ReInit() {
    levelMenuInfo.reinit = qtrue;
}

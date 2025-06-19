#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(c_variadic)]
#![feature(const_raw_ptr_to_usize_cast)]
#![feature(ptr_offset_from)]
#![feature(register_tool)]
#![register_tool(c2rust)]

#[path = "src/bg_public_h.rs"]
pub mod bg_public_h;
#[path = "src/internal.rs"]
pub mod internal;
#[path = "src/keycodes_h.rs"]
pub mod keycodes_h;
#[path = "src/stdarg_h.rs"]
pub mod stdarg_h;
#[path = "src/stddef_h.rs"]
pub mod stddef_h;
#[path = "src/stdlib.rs"]
pub mod stdlib;
#[path = "src/tr_types_h.rs"]
pub mod tr_types_h;
#[path = "src/ui_local_h.rs"]
pub mod ui_local_h;
#[path = "src/ui_public_h.rs"]
pub mod ui_public_h;
extern crate libc;

pub mod src {
    pub mod game {
        pub mod bg_lib;
        pub mod bg_misc;
    } // mod game
    pub mod q3_ui {
        pub mod ui_addbots;
        pub mod ui_atoms;
        pub mod ui_cdkey;
        pub mod ui_cinematics;
        pub mod ui_confirm;
        pub mod ui_connect;
        pub mod ui_controls2;
        pub mod ui_credits;
        pub mod ui_demo2;
        pub mod ui_display;
        pub mod ui_gameinfo;
        pub mod ui_ingame;
        pub mod ui_loadconfig;
        pub mod ui_main;
        pub mod ui_menu;
        pub mod ui_mfield;
        pub mod ui_mods;
        pub mod ui_network;
        pub mod ui_options;
        pub mod ui_playermodel;
        pub mod ui_players;
        pub mod ui_playersettings;
        pub mod ui_preferences;
        pub mod ui_qmenu;
        pub mod ui_removebots;
        pub mod ui_saveconfig;
        pub mod ui_serverinfo;
        pub mod ui_servers2;
        pub mod ui_setup;
        pub mod ui_sound;
        pub mod ui_sparena;
        pub mod ui_specifyserver;
        pub mod ui_splevel;
        pub mod ui_sppostgame;
        pub mod ui_spskill;
        pub mod ui_startserver;
        pub mod ui_team;
        pub mod ui_teamorders;
        pub mod ui_video;
    } // mod q3_ui
    pub mod qcommon {
        pub mod q_math;
        pub mod q_shared;
    } // mod qcommon
    pub mod ui {
        pub mod ui_syscalls;
    } // mod ui
} // mod src

#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(c_variadic)]
#![feature(register_tool)]
#![register_tool(c2rust)]

#[path = "src/be_aas_h.rs"]
pub mod be_aas_h;
#[path = "src/be_ai_chat_h.rs"]
pub mod be_ai_chat_h;
#[path = "src/be_ai_goal_h.rs"]
pub mod be_ai_goal_h;
#[path = "src/be_ai_move_h.rs"]
pub mod be_ai_move_h;
#[path = "src/be_ai_weap_h.rs"]
pub mod be_ai_weap_h;
#[path = "src/bg_local_h.rs"]
pub mod bg_local_h;
#[path = "src/bg_public_h.rs"]
pub mod bg_public_h;
#[path = "src/botlib_h.rs"]
pub mod botlib_h;
#[path = "src/g_local_h.rs"]
pub mod g_local_h;
#[path = "src/g_public_h.rs"]
pub mod g_public_h;
#[path = "src/internal.rs"]
pub mod internal;
#[path = "src/stdarg_h.rs"]
pub mod stdarg_h;
#[path = "src/stddef_h.rs"]
pub mod stddef_h;
#[path = "src/stdlib.rs"]
pub mod stdlib;
extern crate libc;

pub mod src {
    pub mod game {
        pub mod ai_chat;
        pub mod ai_cmd;
        pub mod ai_dmnet;
        pub mod ai_dmq3;
        pub mod ai_main;
        pub mod ai_team;
        pub mod ai_vcmd;
        pub mod bg_lib;
        pub mod bg_misc;
        pub mod bg_pmove;
        pub mod bg_slidemove;
        pub mod g_active;
        pub mod g_arenas;
        pub mod g_bot;
        pub mod g_client;
        pub mod g_cmds;
        pub mod g_combat;
        pub mod g_items;
        pub mod g_main;
        pub mod g_mem;
        pub mod g_misc;
        pub mod g_missile;
        pub mod g_mover;
        pub mod g_session;
        pub mod g_spawn;
        pub mod g_svcmds;
        pub mod g_syscalls;
        pub mod g_target;
        pub mod g_team;
        pub mod g_trigger;
        pub mod g_utils;
        pub mod g_weapon;
    } // mod game
    pub mod qcommon {
        pub mod q_math;
        pub mod q_shared;
    } // mod qcommon
} // mod src

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

#[path = "src/bg_local_h.rs"]
pub mod bg_local_h;
#[path = "src/bg_public_h.rs"]
pub mod bg_public_h;
#[path = "src/cg_local_h.rs"]
pub mod cg_local_h;
#[path = "src/cg_public_h.rs"]
pub mod cg_public_h;
#[path = "src/internal.rs"]
pub mod internal;
#[path = "src/stdarg_h.rs"]
pub mod stdarg_h;
#[path = "src/stddef_h.rs"]
pub mod stddef_h;
#[path = "src/stdlib.rs"]
pub mod stdlib;
#[path = "src/tr_types_h.rs"]
pub mod tr_types_h;
extern crate libc;

pub mod src {
    pub mod cgame {
        pub mod cg_consolecmds;
        pub mod cg_draw;
        pub mod cg_drawtools;
        pub mod cg_effects;
        pub mod cg_ents;
        pub mod cg_event;
        pub mod cg_info;
        pub mod cg_localents;
        pub mod cg_main;
        pub mod cg_marks;
        pub mod cg_particles;
        pub mod cg_players;
        pub mod cg_playerstate;
        pub mod cg_predict;
        pub mod cg_scoreboard;
        pub mod cg_servercmds;
        pub mod cg_snapshot;
        pub mod cg_syscalls;
        pub mod cg_view;
        pub mod cg_weapons;
    } // mod cgame
    pub mod game {
        pub mod bg_lib;
        pub mod bg_misc;
        pub mod bg_pmove;
        pub mod bg_slidemove;
    } // mod game
    pub mod qcommon {
        pub mod q_math;
        pub mod q_shared;
    } // mod qcommon
} // mod src

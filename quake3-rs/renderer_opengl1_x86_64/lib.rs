#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(c_variadic)]
#![feature(const_transmute)]
#![feature(extern_types)]
#![feature(register_tool)]
#![register_tool(c2rust)]

#[path = "src/internal.rs"]
pub mod internal;
#[path = "src/iqm_h.rs"]
pub mod iqm_h;
#[path = "src/jdct_h.rs"]
pub mod jdct_h;
#[path = "src/jmemsys_h.rs"]
pub mod jmemsys_h;
#[path = "src/jmorecfg_h.rs"]
pub mod jmorecfg_h;
#[path = "src/jpegint_h.rs"]
pub mod jpegint_h;
#[path = "src/jpeglib_h.rs"]
pub mod jpeglib_h;
#[path = "src/qfiles_h.rs"]
pub mod qfiles_h;
#[path = "src/qgl_h.rs"]
pub mod qgl_h;
#[path = "src/sdl_icon_h.rs"]
pub mod sdl_icon_h;
#[path = "src/stdarg_h.rs"]
pub mod stdarg_h;
#[path = "src/stddef_h.rs"]
pub mod stddef_h;
#[path = "src/stdlib.rs"]
pub mod stdlib;
#[path = "src/tr_common_h.rs"]
pub mod tr_common_h;
#[path = "src/tr_local_h.rs"]
pub mod tr_local_h;
#[path = "src/tr_public_h.rs"]
pub mod tr_public_h;
#[path = "src/tr_types_h.rs"]
pub mod tr_types_h;
extern crate libc;

pub mod src {
    pub mod jpeg_8c {
        pub mod jaricom;
        pub mod jcapimin;
        pub mod jcapistd;
        pub mod jcarith;
        pub mod jccoefct;
        pub mod jccolor;
        pub mod jcdctmgr;
        pub mod jchuff;
        pub mod jcinit;
        pub mod jcmainct;
        pub mod jcmarker;
        pub mod jcmaster;
        pub mod jcomapi;
        pub mod jcparam;
        pub mod jcprepct;
        pub mod jcsample;
        pub mod jctrans;
        pub mod jdapimin;
        pub mod jdapistd;
        pub mod jdarith;
        pub mod jdatadst;
        pub mod jdatasrc;
        pub mod jdcoefct;
        pub mod jdcolor;
        pub mod jddctmgr;
        pub mod jdhuff;
        pub mod jdinput;
        pub mod jdmainct;
        pub mod jdmarker;
        pub mod jdmaster;
        pub mod jdmerge;
        pub mod jdpostct;
        pub mod jdsample;
        pub mod jdtrans;
        pub mod jerror;
        pub mod jfdctflt;
        pub mod jfdctfst;
        pub mod jfdctint;
        pub mod jidctflt;
        pub mod jidctfst;
        pub mod jidctint;
        pub mod jmemmgr;
        pub mod jmemnobs;
        pub mod jquant1;
        pub mod jquant2;
        pub mod jutils;
    } // mod jpeg_8c
    pub mod qcommon {
        pub mod puff;
        pub mod q_math;
        pub mod q_shared;
    } // mod qcommon
    pub mod renderercommon {
        pub mod tr_font;
        pub mod tr_image_bmp;
        pub mod tr_image_jpg;
        pub mod tr_image_pcx;
        pub mod tr_image_png;
        pub mod tr_image_tga;
        pub mod tr_noise;
    } // mod renderercommon
    pub mod renderergl1 {
        pub mod tr_altivec;
        pub mod tr_animation;
        pub mod tr_backend;
        pub mod tr_bsp;
        pub mod tr_cmds;
        pub mod tr_curve;
        pub mod tr_flares;
        pub mod tr_image;
        pub mod tr_init;
        pub mod tr_light;
        pub mod tr_main;
        pub mod tr_marks;
        pub mod tr_mesh;
        pub mod tr_model;
        pub mod tr_model_iqm;
        pub mod tr_scene;
        pub mod tr_shade;
        pub mod tr_shade_calc;
        pub mod tr_shader;
        pub mod tr_shadows;
        pub mod tr_sky;
        pub mod tr_subs;
        pub mod tr_surface;
        pub mod tr_world;
    } // mod renderergl1
    pub mod sdl {
        pub mod sdl_gamma;
        pub mod sdl_glimp;
    } // mod sdl
} // mod src

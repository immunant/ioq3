use ::libc;

pub mod sdl_icon_h {
    /* GIMP RGBA C-Source image dump (sdl_icon.h) */

    pub static mut CLIENT_WINDOW_ICON: crate::sdl_icon_h::C2RustUnnamed_152 = {
        let mut init = crate::sdl_icon_h::C2RustUnnamed_152 {
            width: 32 as i32 as u32,
            height: 32 as i32 as u32,
            bytes_per_pixel: 4 as i32 as u32,
            pixel_data: [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 119, 0, 0, 255, 119, 0,
                0, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 119, 0, 0, 255, 119, 0, 0, 255, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 119, 0, 0, 255, 119, 0, 0, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 119, 0, 0, 255, 119,
                0, 0, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 119, 0, 0, 255, 119, 0, 0, 255, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 119, 0, 0, 255, 119, 0, 0, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 136, 0, 0, 255,
                136, 0, 0, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 136, 0, 0, 255, 136, 0, 0, 255, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 136, 0, 0, 255, 136, 0, 0, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                153, 0, 0, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 136,
                0, 0, 255, 136, 0, 0, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 153, 0, 0, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 170, 0, 0, 255, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 153, 0, 0, 255, 136, 0,
                0, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                170, 0, 0, 255, 0, 0, 0, 0, 187, 0, 0, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 153, 0, 0, 255, 153, 0, 0, 255, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 187,
                0, 0, 255, 204, 0, 0, 255, 153, 0, 0, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 153, 0, 0, 255, 153, 0, 0, 255, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 153, 0, 0, 255, 204, 0, 0,
                255, 0, 0, 0, 0, 221, 0, 0, 255, 204, 0, 0, 255, 153, 0, 0, 255, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 153, 0, 0, 255, 153, 0, 0, 255, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 153, 0, 0, 255, 204, 0, 0, 255, 221, 0, 0, 255, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 153, 0, 0, 255, 204, 0, 0, 255, 221, 0, 0, 255, 221, 0,
                0, 255, 187, 0, 0, 255, 153, 0, 0, 255, 136, 0, 0, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 153, 0, 0, 255, 170, 0, 0, 255, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 170,
                0, 0, 255, 187, 0, 0, 255, 221, 0, 0, 255, 221, 0, 0, 255, 204, 0, 0, 255, 153, 0,
                0, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 170, 0, 0, 255, 204, 0, 0, 255, 204, 0, 0, 255, 204, 0, 0, 255, 204, 0, 0,
                255, 204, 0, 0, 255, 204, 0, 0, 255, 204, 0, 0, 255, 204, 0, 0, 255, 0, 0, 0, 0,
                170, 0, 0, 255, 170, 0, 0, 255, 0, 0, 0, 0, 204, 0, 0, 255, 204, 0, 0, 255, 204, 0,
                0, 255, 204, 0, 0, 255, 204, 0, 0, 255, 204, 0, 0, 255, 204, 0, 0, 255, 204, 0, 0,
                255, 170, 0, 0, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 136, 0, 0, 255, 153, 0, 0, 255, 187, 0, 0, 255, 204, 0, 0,
                255, 204, 0, 0, 255, 0, 0, 0, 0, 170, 0, 0, 255, 170, 0, 0, 255, 0, 0, 0, 0, 204,
                0, 0, 255, 204, 0, 0, 255, 187, 0, 0, 255, 153, 0, 0, 255, 136, 0, 0, 255, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 136, 0,
                0, 255, 187, 0, 0, 255, 0, 0, 0, 0, 170, 0, 0, 255, 170, 0, 0, 255, 0, 0, 0, 0,
                187, 0, 0, 255, 136, 0, 0, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 119, 0, 0,
                255, 187, 0, 0, 255, 0, 0, 0, 0, 153, 0, 0, 255, 170, 0, 0, 255, 0, 0, 0, 0, 187,
                0, 0, 255, 119, 0, 0, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 170, 0,
                0, 255, 0, 0, 0, 0, 153, 0, 0, 255, 153, 0, 0, 255, 0, 0, 0, 0, 170, 0, 0, 255, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 170, 0, 0, 255, 0, 0, 0,
                0, 136, 0, 0, 255, 153, 0, 0, 255, 0, 0, 0, 0, 170, 0, 0, 255, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 153, 0, 0, 255, 0, 0, 0, 0, 136, 0, 0, 255,
                136, 0, 0, 255, 0, 0, 0, 0, 153, 0, 0, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 153, 0, 0, 255, 0, 0, 0, 0, 119, 0, 0, 255, 136, 0, 0, 255, 0,
                0, 0, 0, 153, 0, 0, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 136, 0, 0, 255, 0, 0, 0, 0, 136, 0, 0, 255, 136, 0, 0, 255, 0, 0, 0, 0, 136, 0,
                0, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 136, 0, 0,
                255, 0, 0, 0, 0, 119, 0, 0, 255, 136, 0, 0, 255, 0, 0, 0, 0, 136, 0, 0, 255, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 119, 0, 0, 255, 0, 0, 0, 0,
                119, 0, 0, 255, 119, 0, 0, 255, 0, 0, 0, 0, 119, 0, 0, 255, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 119, 0, 0, 255, 0, 0, 0, 0, 119, 0, 0, 255,
                119, 0, 0, 255, 0, 0, 0, 0, 119, 0, 0, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 119, 0, 0, 255, 0, 0, 0, 0, 119, 0, 0, 255, 119, 0, 0, 255, 0,
                0, 0, 0, 119, 0, 0, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 102, 0, 0, 255, 102, 0, 0, 255, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                102, 0, 0, 255, 102, 0, 0, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0,
            ],
        };
        init
    };
}

pub use crate::src::jpeg_8c::jerror::C2RustUnnamed_1;
pub use crate::stddef_h::ptrdiff_t;
pub use crate::stddef_h::size_t;
pub use crate::stdlib::SDL_Color;
pub use crate::stdlib::SDL_Palette;
pub use crate::stdlib::SDL_PixelFormat;
pub use crate::stdlib::SDL_Rect;
pub use crate::stdlib::SDL_calloc;
pub use crate::stdlib::SDL_free;
pub use crate::stdlib::__compar_fn_t;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::__uint8_t;
pub use crate::stdlib::qsort;
pub use crate::stdlib::uint32_t;
pub use crate::stdlib::uint8_t;
pub use crate::stdlib::SDL_bool;
pub use crate::stdlib::Uint32;
pub use crate::stdlib::Uint8;
pub use crate::stdlib::SDL_FALSE;
pub use crate::stdlib::SDL_PIXELFORMAT_ABGR1555;
pub use crate::stdlib::SDL_PIXELFORMAT_ABGR32;
pub use crate::stdlib::SDL_PIXELFORMAT_ABGR4444;
pub use crate::stdlib::SDL_PIXELFORMAT_ABGR8888;
pub use crate::stdlib::SDL_PIXELFORMAT_ARGB1555;
pub use crate::stdlib::SDL_PIXELFORMAT_ARGB2101010;
pub use crate::stdlib::SDL_PIXELFORMAT_ARGB32;
pub use crate::stdlib::SDL_PIXELFORMAT_ARGB4444;
pub use crate::stdlib::SDL_PIXELFORMAT_ARGB8888;
pub use crate::stdlib::SDL_PIXELFORMAT_BGR24;
pub use crate::stdlib::SDL_PIXELFORMAT_BGR555;
pub use crate::stdlib::SDL_PIXELFORMAT_BGR565;
pub use crate::stdlib::SDL_PIXELFORMAT_BGR888;
pub use crate::stdlib::SDL_PIXELFORMAT_BGRA32;
pub use crate::stdlib::SDL_PIXELFORMAT_BGRA4444;
pub use crate::stdlib::SDL_PIXELFORMAT_BGRA5551;
pub use crate::stdlib::SDL_PIXELFORMAT_BGRA8888;
pub use crate::stdlib::SDL_PIXELFORMAT_BGRX8888;
pub use crate::stdlib::SDL_PIXELFORMAT_EXTERNAL_OES;
pub use crate::stdlib::SDL_PIXELFORMAT_INDEX1LSB;
pub use crate::stdlib::SDL_PIXELFORMAT_INDEX1MSB;
pub use crate::stdlib::SDL_PIXELFORMAT_INDEX4LSB;
pub use crate::stdlib::SDL_PIXELFORMAT_INDEX4MSB;
pub use crate::stdlib::SDL_PIXELFORMAT_INDEX8;
pub use crate::stdlib::SDL_PIXELFORMAT_IYUV;
pub use crate::stdlib::SDL_PIXELFORMAT_NV12;
pub use crate::stdlib::SDL_PIXELFORMAT_NV21;
pub use crate::stdlib::SDL_PIXELFORMAT_RGB24;
pub use crate::stdlib::SDL_PIXELFORMAT_RGB332;
pub use crate::stdlib::SDL_PIXELFORMAT_RGB444;
pub use crate::stdlib::SDL_PIXELFORMAT_RGB555;
pub use crate::stdlib::SDL_PIXELFORMAT_RGB565;
pub use crate::stdlib::SDL_PIXELFORMAT_RGB888;
pub use crate::stdlib::SDL_PIXELFORMAT_RGBA32;
pub use crate::stdlib::SDL_PIXELFORMAT_RGBA4444;
pub use crate::stdlib::SDL_PIXELFORMAT_RGBA5551;
pub use crate::stdlib::SDL_PIXELFORMAT_RGBA8888;
pub use crate::stdlib::SDL_PIXELFORMAT_RGBX8888;
pub use crate::stdlib::SDL_PIXELFORMAT_UNKNOWN;
pub use crate::stdlib::SDL_PIXELFORMAT_UYVY;
pub use crate::stdlib::SDL_PIXELFORMAT_YUY2;
pub use crate::stdlib::SDL_PIXELFORMAT_YV12;
pub use crate::stdlib::SDL_PIXELFORMAT_YVYU;
pub use crate::stdlib::SDL_TRUE;

pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::cvar_s;
pub use crate::src::qcommon::q_shared::cvar_t;
pub use crate::src::qcommon::q_shared::e_status;
pub use crate::src::qcommon::q_shared::h_dontcare;
pub use crate::src::qcommon::q_shared::h_high;
pub use crate::src::qcommon::q_shared::h_low;
pub use crate::src::qcommon::q_shared::ha_pref;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::va;
pub use crate::src::qcommon::q_shared::Q_strcat;
pub use crate::src::qcommon::q_shared::Q_stricmp;
pub use crate::src::qcommon::q_shared::Q_stricmpn;
pub use crate::src::qcommon::q_shared::Q_strncpyz;
pub use crate::src::qcommon::q_shared::ERR_DISCONNECT;
pub use crate::src::qcommon::q_shared::ERR_DROP;
pub use crate::src::qcommon::q_shared::ERR_FATAL;
pub use crate::src::qcommon::q_shared::ERR_NEED_CD;
pub use crate::src::qcommon::q_shared::ERR_SERVERDISCONNECT;
pub use crate::src::qcommon::q_shared::EXEC_APPEND;
pub use crate::src::qcommon::q_shared::EXEC_INSERT;
pub use crate::src::qcommon::q_shared::EXEC_NOW;
pub use crate::src::qcommon::q_shared::FMV_EOF;
pub use crate::src::qcommon::q_shared::FMV_IDLE;
pub use crate::src::qcommon::q_shared::FMV_ID_BLT;
pub use crate::src::qcommon::q_shared::FMV_ID_IDLE;
pub use crate::src::qcommon::q_shared::FMV_ID_WAIT;
pub use crate::src::qcommon::q_shared::FMV_LOOPED;
pub use crate::src::qcommon::q_shared::FMV_PLAY;
pub use crate::src::qcommon::q_shared::PRINT_ALL;
pub use crate::src::qcommon::q_shared::PRINT_DEVELOPER;
pub use crate::src::qcommon::q_shared::PRINT_ERROR;
pub use crate::src::qcommon::q_shared::PRINT_WARNING;
pub use crate::src::renderergl1::tr_subs::Com_Error;
pub use crate::stdlib::SDL_BlitMap;
pub use crate::stdlib::SDL_CreateRGBSurfaceFrom;
pub use crate::stdlib::SDL_CreateWindow;
pub use crate::stdlib::SDL_DestroyWindow;
pub use crate::stdlib::SDL_DisplayMode;
pub use crate::stdlib::SDL_FreeSurface;
pub use crate::stdlib::SDL_GLContext;
pub use crate::stdlib::SDL_GL_CreateContext;
pub use crate::stdlib::SDL_GL_DeleteContext;
pub use crate::stdlib::SDL_GL_ExtensionSupported;
pub use crate::stdlib::SDL_GL_GetAttribute;
pub use crate::stdlib::SDL_GL_GetProcAddress;
pub use crate::stdlib::SDL_GL_SetAttribute;
pub use crate::stdlib::SDL_GL_SetSwapInterval;
pub use crate::stdlib::SDL_GL_SwapWindow;
pub use crate::stdlib::SDL_GLattr;
pub use crate::stdlib::SDL_GetCurrentVideoDriver;
pub use crate::stdlib::SDL_GetDesktopDisplayMode;
pub use crate::stdlib::SDL_GetDisplayMode;
pub use crate::stdlib::SDL_GetNumDisplayModes;
pub use crate::stdlib::SDL_GetWindowDisplayIndex;
pub use crate::stdlib::SDL_GetWindowDisplayMode;
pub use crate::stdlib::SDL_GetWindowFlags;
pub use crate::stdlib::SDL_GetWindowPosition;
pub use crate::stdlib::SDL_MinimizeWindow;
pub use crate::stdlib::SDL_SetWindowBrightness;
pub use crate::stdlib::SDL_SetWindowDisplayMode;
pub use crate::stdlib::SDL_SetWindowFullscreen;
pub use crate::stdlib::SDL_SetWindowIcon;
pub use crate::stdlib::SDL_Surface;
pub use crate::stdlib::SDL_Window;
pub use crate::stdlib::SDL_GL_ACCELERATED_VISUAL;
pub use crate::stdlib::SDL_GL_ACCUM_ALPHA_SIZE;
pub use crate::stdlib::SDL_GL_ACCUM_BLUE_SIZE;
pub use crate::stdlib::SDL_GL_ACCUM_GREEN_SIZE;
pub use crate::stdlib::SDL_GL_ACCUM_RED_SIZE;
pub use crate::stdlib::SDL_GL_ALPHA_SIZE;
pub use crate::stdlib::SDL_GL_BLUE_SIZE;
pub use crate::stdlib::SDL_GL_BUFFER_SIZE;
pub use crate::stdlib::SDL_GL_CONTEXT_EGL;
pub use crate::stdlib::SDL_GL_CONTEXT_FLAGS;
pub use crate::stdlib::SDL_GL_CONTEXT_MAJOR_VERSION;
pub use crate::stdlib::SDL_GL_CONTEXT_MINOR_VERSION;
pub use crate::stdlib::SDL_GL_CONTEXT_NO_ERROR;
pub use crate::stdlib::SDL_GL_CONTEXT_PROFILE_COMPATIBILITY;
pub use crate::stdlib::SDL_GL_CONTEXT_PROFILE_CORE;
pub use crate::stdlib::SDL_GL_CONTEXT_PROFILE_ES;
pub use crate::stdlib::SDL_GL_CONTEXT_PROFILE_MASK;
pub use crate::stdlib::SDL_GL_CONTEXT_RELEASE_BEHAVIOR;
pub use crate::stdlib::SDL_GL_CONTEXT_RESET_NOTIFICATION;
pub use crate::stdlib::SDL_GL_DEPTH_SIZE;
pub use crate::stdlib::SDL_GL_DOUBLEBUFFER;
pub use crate::stdlib::SDL_GL_FRAMEBUFFER_SRGB_CAPABLE;
pub use crate::stdlib::SDL_GL_GREEN_SIZE;
pub use crate::stdlib::SDL_GL_MULTISAMPLEBUFFERS;
pub use crate::stdlib::SDL_GL_MULTISAMPLESAMPLES;
pub use crate::stdlib::SDL_GL_RED_SIZE;
pub use crate::stdlib::SDL_GL_RETAINED_BACKING;
pub use crate::stdlib::SDL_GL_SHARE_WITH_CURRENT_CONTEXT;
pub use crate::stdlib::SDL_GL_STENCIL_SIZE;
pub use crate::stdlib::SDL_GL_STEREO;
pub use crate::stdlib::SDL_WINDOW_ALLOW_HIGHDPI;
pub use crate::stdlib::SDL_WINDOW_ALWAYS_ON_TOP;
pub use crate::stdlib::SDL_WINDOW_BORDERLESS;
pub use crate::stdlib::SDL_WINDOW_FOREIGN;
pub use crate::stdlib::SDL_WINDOW_FULLSCREEN;
pub use crate::stdlib::SDL_WINDOW_FULLSCREEN_DESKTOP;
pub use crate::stdlib::SDL_WINDOW_HIDDEN;
pub use crate::stdlib::SDL_WINDOW_INPUT_FOCUS;
pub use crate::stdlib::SDL_WINDOW_INPUT_GRABBED;
pub use crate::stdlib::SDL_WINDOW_MAXIMIZED;
pub use crate::stdlib::SDL_WINDOW_MINIMIZED;
pub use crate::stdlib::SDL_WINDOW_MOUSE_CAPTURE;
pub use crate::stdlib::SDL_WINDOW_MOUSE_FOCUS;
pub use crate::stdlib::SDL_WINDOW_OPENGL;
pub use crate::stdlib::SDL_WINDOW_POPUP_MENU;
pub use crate::stdlib::SDL_WINDOW_RESIZABLE;
pub use crate::stdlib::SDL_WINDOW_SHOWN;
pub use crate::stdlib::SDL_WINDOW_SKIP_TASKBAR;
pub use crate::stdlib::SDL_WINDOW_TOOLTIP;
pub use crate::stdlib::SDL_WINDOW_UTILITY;
pub use crate::stdlib::SDL_WINDOW_VULKAN;
pub use crate::tr_public_h::refimport_t;
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

pub use crate::qgl_h::ActiveTextureproc;
pub use crate::qgl_h::AlphaFuncproc;
pub use crate::qgl_h::ArrayElementproc;
pub use crate::qgl_h::AttachShaderproc;
pub use crate::qgl_h::BeginQueryproc;
pub use crate::qgl_h::Beginproc;
pub use crate::qgl_h::BindAttribLocationproc;
pub use crate::qgl_h::BindBufferproc;
pub use crate::qgl_h::BindFramebufferproc;
pub use crate::qgl_h::BindMultiTextureEXTproc;
pub use crate::qgl_h::BindRenderbufferproc;
pub use crate::qgl_h::BindTextureproc;
pub use crate::qgl_h::BindVertexArrayproc;
pub use crate::qgl_h::BlendFuncproc;
pub use crate::qgl_h::BlitFramebufferproc;
pub use crate::qgl_h::BufferDataproc;
pub use crate::qgl_h::BufferSubDataproc;
pub use crate::qgl_h::CheckFramebufferStatusproc;
pub use crate::qgl_h::CheckNamedFramebufferStatusEXTproc;
pub use crate::qgl_h::ClearColorproc;
pub use crate::qgl_h::ClearDepthfproc;
pub use crate::qgl_h::ClearDepthproc;
pub use crate::qgl_h::ClearStencilproc;
pub use crate::qgl_h::Clearproc;
pub use crate::qgl_h::ClipPlanefproc;
pub use crate::qgl_h::ClipPlaneproc;
pub use crate::qgl_h::Color3fproc;
pub use crate::qgl_h::Color4fproc;
pub use crate::qgl_h::Color4ubvproc;
pub use crate::qgl_h::ColorMaskproc;
pub use crate::qgl_h::ColorPointerproc;
pub use crate::qgl_h::CompileShaderproc;
pub use crate::qgl_h::CompressedTexImage2Dproc;
pub use crate::qgl_h::CompressedTexSubImage2Dproc;
pub use crate::qgl_h::CompressedTextureImage2DEXTproc;
pub use crate::qgl_h::CompressedTextureSubImage2DEXTproc;
pub use crate::qgl_h::CopyTexSubImage2Dproc;
pub use crate::qgl_h::CopyTextureSubImage2DEXTproc;
pub use crate::qgl_h::CreateProgramproc;
pub use crate::qgl_h::CreateShaderproc;
pub use crate::qgl_h::CullFaceproc;
pub use crate::qgl_h::DeleteBuffersproc;
pub use crate::qgl_h::DeleteFramebuffersproc;
pub use crate::qgl_h::DeleteProgramproc;
pub use crate::qgl_h::DeleteQueriesproc;
pub use crate::qgl_h::DeleteRenderbuffersproc;
pub use crate::qgl_h::DeleteShaderproc;
pub use crate::qgl_h::DeleteTexturesproc;
pub use crate::qgl_h::DeleteVertexArraysproc;
pub use crate::qgl_h::DepthFuncproc;
pub use crate::qgl_h::DepthMaskproc;
pub use crate::qgl_h::DepthRangefproc;
pub use crate::qgl_h::DepthRangeproc;
pub use crate::qgl_h::DetachShaderproc;
pub use crate::qgl_h::DisableClientStateproc;
pub use crate::qgl_h::DisableVertexAttribArrayproc;
pub use crate::qgl_h::Disableproc;
pub use crate::qgl_h::DrawArraysproc;
pub use crate::qgl_h::DrawBufferproc;
pub use crate::qgl_h::DrawElementsproc;
pub use crate::qgl_h::EnableClientStateproc;
pub use crate::qgl_h::EnableVertexAttribArrayproc;
pub use crate::qgl_h::Enableproc;
pub use crate::qgl_h::EndQueryproc;
pub use crate::qgl_h::Endproc;
pub use crate::qgl_h::Finishproc;
pub use crate::qgl_h::Flushproc;
pub use crate::qgl_h::FramebufferRenderbufferproc;
pub use crate::qgl_h::FramebufferTexture2Dproc;
pub use crate::qgl_h::Frustumfproc;
pub use crate::qgl_h::Frustumproc;
pub use crate::qgl_h::GenBuffersproc;
pub use crate::qgl_h::GenFramebuffersproc;
pub use crate::qgl_h::GenQueriesproc;
pub use crate::qgl_h::GenRenderbuffersproc;
pub use crate::qgl_h::GenTexturesproc;
pub use crate::qgl_h::GenVertexArraysproc;
pub use crate::qgl_h::GenerateMipmapproc;
pub use crate::qgl_h::GenerateTextureMipmapEXTproc;
pub use crate::qgl_h::GetActiveUniformproc;
pub use crate::qgl_h::GetBooleanvproc;
pub use crate::qgl_h::GetErrorproc;
pub use crate::qgl_h::GetIntegervproc;
pub use crate::qgl_h::GetProgramInfoLogproc;
pub use crate::qgl_h::GetProgramivproc;
pub use crate::qgl_h::GetQueryObjectivproc;
pub use crate::qgl_h::GetQueryObjectuivproc;
pub use crate::qgl_h::GetShaderInfoLogproc;
pub use crate::qgl_h::GetShaderSourceproc;
pub use crate::qgl_h::GetShaderivproc;
pub use crate::qgl_h::GetStringiproc;
pub use crate::qgl_h::GetStringproc;
pub use crate::qgl_h::GetUniformLocationproc;
pub use crate::qgl_h::LineWidthproc;
pub use crate::qgl_h::LinkProgramproc;
pub use crate::qgl_h::LoadIdentityproc;
pub use crate::qgl_h::LoadMatrixfproc;
pub use crate::qgl_h::MatrixModeproc;
pub use crate::qgl_h::NamedFramebufferRenderbufferEXTproc;
pub use crate::qgl_h::NamedFramebufferTexture2DEXTproc;
pub use crate::qgl_h::NamedRenderbufferStorageEXTproc;
pub use crate::qgl_h::NamedRenderbufferStorageMultisampleEXTproc;
pub use crate::qgl_h::Orthofproc;
pub use crate::qgl_h::Orthoproc;
pub use crate::qgl_h::PolygonModeproc;
pub use crate::qgl_h::PolygonOffsetproc;
pub use crate::qgl_h::PopMatrixproc;
pub use crate::qgl_h::ProgramUniform1fEXTproc;
pub use crate::qgl_h::ProgramUniform1fvEXTproc;
pub use crate::qgl_h::ProgramUniform1iEXTproc;
pub use crate::qgl_h::ProgramUniform2fEXTproc;
pub use crate::qgl_h::ProgramUniform3fEXTproc;
pub use crate::qgl_h::ProgramUniform4fEXTproc;
pub use crate::qgl_h::ProgramUniformMatrix4fvEXTproc;
pub use crate::qgl_h::PushMatrixproc;
pub use crate::qgl_h::ReadPixelsproc;
pub use crate::qgl_h::RenderbufferStorageMultisampleproc;
pub use crate::qgl_h::RenderbufferStorageproc;
pub use crate::qgl_h::Scissorproc;
pub use crate::qgl_h::ShadeModelproc;
pub use crate::qgl_h::ShaderSourceproc;
pub use crate::qgl_h::StencilFuncproc;
pub use crate::qgl_h::StencilMaskproc;
pub use crate::qgl_h::StencilOpproc;
pub use crate::qgl_h::TexCoord2fproc;
pub use crate::qgl_h::TexCoord2fvproc;
pub use crate::qgl_h::TexCoordPointerproc;
pub use crate::qgl_h::TexEnvfproc;
pub use crate::qgl_h::TexImage2Dproc;
pub use crate::qgl_h::TexParameterfproc;
pub use crate::qgl_h::TexParameteriproc;
pub use crate::qgl_h::TexSubImage2Dproc;
pub use crate::qgl_h::TextureImage2DEXTproc;
pub use crate::qgl_h::TextureParameterfEXTproc;
pub use crate::qgl_h::TextureParameteriEXTproc;
pub use crate::qgl_h::TextureSubImage2DEXTproc;
pub use crate::qgl_h::Translatefproc;
pub use crate::qgl_h::Uniform1fproc;
pub use crate::qgl_h::Uniform1fvproc;
pub use crate::qgl_h::Uniform1iproc;
pub use crate::qgl_h::Uniform2fproc;
pub use crate::qgl_h::Uniform3fproc;
pub use crate::qgl_h::Uniform4fproc;
pub use crate::qgl_h::UniformMatrix4fvproc;
pub use crate::qgl_h::UseProgramproc;
pub use crate::qgl_h::ValidateProgramproc;
pub use crate::qgl_h::Vertex2fproc;
pub use crate::qgl_h::Vertex3fproc;
pub use crate::qgl_h::Vertex3fvproc;
pub use crate::qgl_h::VertexAttribPointerproc;
pub use crate::qgl_h::VertexPointerproc;
pub use crate::qgl_h::Viewportproc;
pub use crate::sdl_icon_h::C2RustUnnamed_152;
pub use crate::src::sdl::sdl_glimp::sdl_icon_h::CLIENT_WINDOW_ICON;

pub use crate::stdlib::GLbitfield;
pub use crate::stdlib::GLboolean;
pub use crate::stdlib::GLchar;
pub use crate::stdlib::GLclampd;
pub use crate::stdlib::GLclampf;
pub use crate::stdlib::GLdouble;
pub use crate::stdlib::GLenum;
pub use crate::stdlib::GLfloat;
pub use crate::stdlib::GLint;
pub use crate::stdlib::GLintptr;
pub use crate::stdlib::GLsizei;
pub use crate::stdlib::GLsizeiptr;
pub use crate::stdlib::GLubyte;
pub use crate::stdlib::GLuint;
pub use crate::stdlib::GLvoid;

pub const RSERR_INVALID_MODE: rserr_t = 2;

pub const RSERR_INVALID_FULLSCREEN: rserr_t = 1;

pub type rserr_t = u32;

pub const RSERR_UNKNOWN: rserr_t = 3;

pub const RSERR_OK: rserr_t = 0;
#[no_mangle]

pub static mut SDL_window: *mut SDL_Window = std::ptr::null_mut();

static mut SDL_glContext: SDL_GLContext = std::ptr::null_mut();
#[no_mangle]

pub static mut r_allowSoftwareGL: *mut cvar_t = std::ptr::null_mut();
// Don't abort out if a hardware visual can't be obtained
#[no_mangle]

pub static mut r_allowResize: *mut cvar_t = std::ptr::null_mut();
// make window resizable
#[no_mangle]

pub static mut r_centerWindow: *mut cvar_t = std::ptr::null_mut();
#[no_mangle]

pub static mut r_sdlDriver: *mut cvar_t = std::ptr::null_mut();
#[no_mangle]

pub static mut qglMajorVersion: i32 = 0;
#[no_mangle]

pub static mut qglMinorVersion: i32 = 0;
#[no_mangle]

pub static mut qglesMajorVersion: i32 = 0;
#[no_mangle]

pub static mut qglesMinorVersion: i32 = 0;
#[no_mangle]

pub static mut qglActiveTextureARB: Option<unsafe extern "C" fn(_: GLenum) -> ()> = None;
#[no_mangle]

pub static mut qglClientActiveTextureARB: Option<unsafe extern "C" fn(_: GLenum) -> ()> = None;
#[no_mangle]

pub static mut qglMultiTexCoord2fARB: Option<
    unsafe extern "C" fn(_: GLenum, _: GLfloat, _: GLfloat) -> (),
> = None;
#[no_mangle]

pub static mut qglLockArraysEXT: Option<unsafe extern "C" fn(_: GLint, _: GLsizei) -> ()> = None;
#[no_mangle]

pub static mut qglUnlockArraysEXT: Option<unsafe extern "C" fn() -> ()> = None;
#[no_mangle]

pub static mut qglGenTextures: Option<GenTexturesproc> = None;
#[no_mangle]

pub static mut qglBindTexture: Option<BindTextureproc> = None;
#[no_mangle]

pub static mut qglBlendFunc: Option<BlendFuncproc> = None;
#[no_mangle]

pub static mut qglClearStencil: Option<ClearStencilproc> = None;
#[no_mangle]

pub static mut qglColorMask: Option<ColorMaskproc> = None;
#[no_mangle]

pub static mut qglCopyTexSubImage2D: Option<CopyTexSubImage2Dproc> = None;
#[no_mangle]

pub static mut qglCullFace: Option<CullFaceproc> = None;
#[no_mangle]

pub static mut qglDeleteTextures: Option<DeleteTexturesproc> = None;
#[no_mangle]

pub static mut qglDepthFunc: Option<DepthFuncproc> = None;
#[no_mangle]

pub static mut qglDepthMask: Option<DepthMaskproc> = None;
#[no_mangle]

pub static mut qglDisable: Option<Disableproc> = None;
#[no_mangle]

pub static mut qglDrawArrays: Option<DrawArraysproc> = None;
#[no_mangle]

pub static mut qglDrawElements: Option<DrawElementsproc> = None;
#[no_mangle]

pub static mut qglEnable: Option<Enableproc> = None;
#[no_mangle]

pub static mut qglFinish: Option<Finishproc> = None;
#[no_mangle]

pub static mut qglFlush: Option<Flushproc> = None;
#[no_mangle]

pub static mut qglGetBooleanv: Option<GetBooleanvproc> = None;
#[no_mangle]

pub static mut qglGetError: Option<GetErrorproc> = None;
#[no_mangle]

pub static mut qglLineWidth: Option<LineWidthproc> = None;
#[no_mangle]

pub static mut qglPolygonOffset: Option<PolygonOffsetproc> = None;
#[no_mangle]

pub static mut qglReadPixels: Option<ReadPixelsproc> = None;
#[no_mangle]

pub static mut qglScissor: Option<Scissorproc> = None;
#[no_mangle]

pub static mut qglStencilFunc: Option<StencilFuncproc> = None;
#[no_mangle]

pub static mut qglClearColor: Option<ClearColorproc> = None;
#[no_mangle]

pub static mut qglClear: Option<Clearproc> = None;
#[no_mangle]

pub static mut qglStencilMask: Option<StencilMaskproc> = None;
#[no_mangle]

pub static mut qglStencilOp: Option<StencilOpproc> = None;
#[no_mangle]

pub static mut qglTexImage2D: Option<TexImage2Dproc> = None;
#[no_mangle]

pub static mut qglTexParameterf: Option<TexParameterfproc> = None;
#[no_mangle]

pub static mut qglTexParameteri: Option<TexParameteriproc> = None;
#[no_mangle]

pub static mut qglTexSubImage2D: Option<TexSubImage2Dproc> = None;
#[no_mangle]

pub static mut qglGetString: Option<GetStringproc> = None;
#[no_mangle]

pub static mut qglGetIntegerv: Option<GetIntegervproc> = None;
#[no_mangle]

pub static mut qglTranslatef: Option<Translatefproc> = None;
#[no_mangle]

pub static mut qglViewport: Option<Viewportproc> = None;
#[no_mangle]

pub static mut qglAlphaFunc: Option<AlphaFuncproc> = None;
#[no_mangle]

pub static mut qglColor4f: Option<Color4fproc> = None;
#[no_mangle]

pub static mut qglColorPointer: Option<ColorPointerproc> = None;
#[no_mangle]

pub static mut qglDisableClientState: Option<DisableClientStateproc> = None;
#[no_mangle]

pub static mut qglEnableClientState: Option<EnableClientStateproc> = None;
#[no_mangle]

pub static mut qglLoadIdentity: Option<LoadIdentityproc> = None;
#[no_mangle]

pub static mut qglLoadMatrixf: Option<LoadMatrixfproc> = None;
#[no_mangle]

pub static mut qglMatrixMode: Option<MatrixModeproc> = None;
#[no_mangle]

pub static mut qglPopMatrix: Option<PopMatrixproc> = None;
#[no_mangle]

pub static mut qglPushMatrix: Option<PushMatrixproc> = None;
#[no_mangle]

pub static mut qglShadeModel: Option<ShadeModelproc> = None;
#[no_mangle]

pub static mut qglTexCoordPointer: Option<TexCoordPointerproc> = None;
#[no_mangle]

pub static mut qglTexEnvf: Option<TexEnvfproc> = None;
#[no_mangle]

pub static mut qglVertexPointer: Option<VertexPointerproc> = None;
#[no_mangle]

pub static mut qglClearDepth: Option<ClearDepthproc> = None;
#[no_mangle]

pub static mut qglDepthRange: Option<DepthRangeproc> = None;
#[no_mangle]

pub static mut qglDrawBuffer: Option<DrawBufferproc> = None;
#[no_mangle]

pub static mut qglPolygonMode: Option<PolygonModeproc> = None;
#[no_mangle]

pub static mut qglTexCoord2f: Option<TexCoord2fproc> = None;
#[no_mangle]

pub static mut qglArrayElement: Option<ArrayElementproc> = None;
#[no_mangle]

pub static mut qglBegin: Option<Beginproc> = None;
#[no_mangle]

pub static mut qglClipPlane: Option<ClipPlaneproc> = None;
#[no_mangle]

pub static mut qglColor3f: Option<Color3fproc> = None;
#[no_mangle]

pub static mut qglColor4ubv: Option<Color4ubvproc> = None;
#[no_mangle]

pub static mut qglEnd: Option<Endproc> = None;
#[no_mangle]

pub static mut qglFrustum: Option<Frustumproc> = None;
#[no_mangle]

pub static mut qglOrtho: Option<Orthoproc> = None;
#[no_mangle]

pub static mut qglTexCoord2fv: Option<TexCoord2fvproc> = None;
#[no_mangle]

pub static mut qglVertex2f: Option<Vertex2fproc> = None;
#[no_mangle]

pub static mut qglVertex3f: Option<Vertex3fproc> = None;
#[no_mangle]

pub static mut qglVertex3fv: Option<Vertex3fvproc> = None;
#[no_mangle]

pub static mut qglClearDepthf: Option<ClearDepthfproc> = None;
#[no_mangle]

pub static mut qglDepthRangef: Option<DepthRangefproc> = None;
#[no_mangle]

pub static mut qglClipPlanef: Option<ClipPlanefproc> = None;
#[no_mangle]

pub static mut qglFrustumf: Option<Frustumfproc> = None;
#[no_mangle]

pub static mut qglOrthof: Option<Orthofproc> = None;
#[no_mangle]

pub static mut qglActiveTexture: Option<ActiveTextureproc> = None;
#[no_mangle]

pub static mut qglCompressedTexImage2D: Option<CompressedTexImage2Dproc> = None;
#[no_mangle]

pub static mut qglCompressedTexSubImage2D: Option<CompressedTexSubImage2Dproc> = None;
#[no_mangle]

pub static mut qglBufferSubData: Option<BufferSubDataproc> = None;
#[no_mangle]

pub static mut qglBindBuffer: Option<BindBufferproc> = None;
#[no_mangle]

pub static mut qglDeleteBuffers: Option<DeleteBuffersproc> = None;
#[no_mangle]

pub static mut qglGenBuffers: Option<GenBuffersproc> = None;
#[no_mangle]

pub static mut qglBufferData: Option<BufferDataproc> = None;
#[no_mangle]

pub static mut qglCompileShader: Option<CompileShaderproc> = None;
#[no_mangle]

pub static mut qglGetShaderSource: Option<GetShaderSourceproc> = None;
#[no_mangle]

pub static mut qglCreateShader: Option<CreateShaderproc> = None;
#[no_mangle]

pub static mut qglCreateProgram: Option<CreateProgramproc> = None;
#[no_mangle]

pub static mut qglVertexAttribPointer: Option<VertexAttribPointerproc> = None;
#[no_mangle]

pub static mut qglBindAttribLocation: Option<BindAttribLocationproc> = None;
#[no_mangle]

pub static mut qglAttachShader: Option<AttachShaderproc> = None;
#[no_mangle]

pub static mut qglDeleteProgram: Option<DeleteProgramproc> = None;
#[no_mangle]

pub static mut qglDeleteShader: Option<DeleteShaderproc> = None;
#[no_mangle]

pub static mut qglDetachShader: Option<DetachShaderproc> = None;
#[no_mangle]

pub static mut qglDisableVertexAttribArray: Option<DisableVertexAttribArrayproc> = None;
#[no_mangle]

pub static mut qglEnableVertexAttribArray: Option<EnableVertexAttribArrayproc> = None;
#[no_mangle]

pub static mut qglGetActiveUniform: Option<GetActiveUniformproc> = None;
#[no_mangle]

pub static mut qglGetProgramiv: Option<GetProgramivproc> = None;
#[no_mangle]

pub static mut qglGetProgramInfoLog: Option<GetProgramInfoLogproc> = None;
#[no_mangle]

pub static mut qglGetShaderiv: Option<GetShaderivproc> = None;
#[no_mangle]

pub static mut qglGetShaderInfoLog: Option<GetShaderInfoLogproc> = None;
#[no_mangle]

pub static mut qglValidateProgram: Option<ValidateProgramproc> = None;
#[no_mangle]

pub static mut qglGetUniformLocation: Option<GetUniformLocationproc> = None;
#[no_mangle]

pub static mut qglLinkProgram: Option<LinkProgramproc> = None;
#[no_mangle]

pub static mut qglShaderSource: Option<ShaderSourceproc> = None;
#[no_mangle]

pub static mut qglUseProgram: Option<UseProgramproc> = None;
#[no_mangle]

pub static mut qglUniform1f: Option<Uniform1fproc> = None;
#[no_mangle]

pub static mut qglUniform2f: Option<Uniform2fproc> = None;
#[no_mangle]

pub static mut qglUniform3f: Option<Uniform3fproc> = None;
#[no_mangle]

pub static mut qglUniform4f: Option<Uniform4fproc> = None;
#[no_mangle]

pub static mut qglUniform1i: Option<Uniform1iproc> = None;
#[no_mangle]

pub static mut qglUniform1fv: Option<Uniform1fvproc> = None;
#[no_mangle]

pub static mut qglUniformMatrix4fv: Option<UniformMatrix4fvproc> = None;
#[no_mangle]

pub static mut qglGetStringi: Option<GetStringiproc> = None;
#[no_mangle]

pub static mut qglGetQueryObjectuiv: Option<GetQueryObjectuivproc> = None;
#[no_mangle]

pub static mut qglGenQueries: Option<GenQueriesproc> = None;
#[no_mangle]

pub static mut qglDeleteQueries: Option<DeleteQueriesproc> = None;
#[no_mangle]

pub static mut qglBeginQuery: Option<BeginQueryproc> = None;
#[no_mangle]

pub static mut qglEndQuery: Option<EndQueryproc> = None;
#[no_mangle]

pub static mut qglGetQueryObjectiv: Option<GetQueryObjectivproc> = None;
#[no_mangle]

pub static mut qglCheckFramebufferStatus: Option<CheckFramebufferStatusproc> = None;
#[no_mangle]

pub static mut qglDeleteRenderbuffers: Option<DeleteRenderbuffersproc> = None;
#[no_mangle]

pub static mut qglGenRenderbuffers: Option<GenRenderbuffersproc> = None;
#[no_mangle]

pub static mut qglDeleteFramebuffers: Option<DeleteFramebuffersproc> = None;
#[no_mangle]

pub static mut qglGenFramebuffers: Option<GenFramebuffersproc> = None;
#[no_mangle]

pub static mut qglRenderbufferStorage: Option<RenderbufferStorageproc> = None;
#[no_mangle]

pub static mut qglFramebufferTexture2D: Option<FramebufferTexture2Dproc> = None;
#[no_mangle]

pub static mut qglFramebufferRenderbuffer: Option<FramebufferRenderbufferproc> = None;
#[no_mangle]

pub static mut qglGenerateMipmap: Option<GenerateMipmapproc> = None;
#[no_mangle]

pub static mut qglBlitFramebuffer: Option<BlitFramebufferproc> = None;
#[no_mangle]

pub static mut qglRenderbufferStorageMultisample: Option<RenderbufferStorageMultisampleproc> = None;
#[no_mangle]

pub static mut qglBindFramebuffer: Option<BindFramebufferproc> = None;
#[no_mangle]

pub static mut qglBindRenderbuffer: Option<BindRenderbufferproc> = None;
#[no_mangle]

pub static mut qglGenVertexArrays: Option<GenVertexArraysproc> = None;
#[no_mangle]

pub static mut qglDeleteVertexArrays: Option<DeleteVertexArraysproc> = None;
#[no_mangle]

pub static mut qglBindVertexArray: Option<BindVertexArrayproc> = None;
#[no_mangle]

pub static mut qglNamedFramebufferTexture2DEXT: Option<NamedFramebufferTexture2DEXTproc> = None;
#[no_mangle]

pub static mut qglTextureImage2DEXT: Option<TextureImage2DEXTproc> = None;
#[no_mangle]

pub static mut qglBindMultiTextureEXT: Option<BindMultiTextureEXTproc> = None;
#[no_mangle]

pub static mut qglTextureParameterfEXT: Option<TextureParameterfEXTproc> = None;
#[no_mangle]

pub static mut qglTextureParameteriEXT: Option<TextureParameteriEXTproc> = None;
#[no_mangle]

pub static mut qglTextureSubImage2DEXT: Option<TextureSubImage2DEXTproc> = None;
#[no_mangle]

pub static mut qglCopyTextureSubImage2DEXT: Option<CopyTextureSubImage2DEXTproc> = None;
#[no_mangle]

pub static mut qglCompressedTextureImage2DEXT: Option<CompressedTextureImage2DEXTproc> = None;
#[no_mangle]

pub static mut qglNamedFramebufferRenderbufferEXT: Option<NamedFramebufferRenderbufferEXTproc> =
    None;
#[no_mangle]

pub static mut qglCompressedTextureSubImage2DEXT: Option<CompressedTextureSubImage2DEXTproc> = None;
#[no_mangle]

pub static mut qglCheckNamedFramebufferStatusEXT: Option<CheckNamedFramebufferStatusEXTproc> = None;
#[no_mangle]

pub static mut qglNamedRenderbufferStorageMultisampleEXT: Option<
    NamedRenderbufferStorageMultisampleEXTproc,
> = None;
#[no_mangle]

pub static mut qglNamedRenderbufferStorageEXT: Option<NamedRenderbufferStorageEXTproc> = None;
#[no_mangle]

pub static mut qglProgramUniformMatrix4fvEXT: Option<ProgramUniformMatrix4fvEXTproc> = None;
#[no_mangle]

pub static mut qglProgramUniform1fvEXT: Option<ProgramUniform1fvEXTproc> = None;
#[no_mangle]

pub static mut qglProgramUniform4fEXT: Option<ProgramUniform4fEXTproc> = None;
#[no_mangle]

pub static mut qglProgramUniform3fEXT: Option<ProgramUniform3fEXTproc> = None;
#[no_mangle]

pub static mut qglProgramUniform2fEXT: Option<ProgramUniform2fEXTproc> = None;
#[no_mangle]

pub static mut qglProgramUniform1fEXT: Option<ProgramUniform1fEXTproc> = None;
#[no_mangle]

pub static mut qglProgramUniform1iEXT: Option<ProgramUniform1iEXTproc> = None;
#[no_mangle]

pub static mut qglGenerateTextureMipmapEXT: Option<GenerateTextureMipmapEXTproc> = None;
/*
===============
GLimp_Shutdown
===============
*/
#[no_mangle]

pub unsafe extern "C" fn GLimp_Shutdown() {
    crate::src::renderergl1::tr_main::ri
        .IN_Shutdown
        .expect("non-null function pointer")();
    crate::stdlib::SDL_QuitSubSystem(0x20 as u32);
}
/*
===============
GLimp_Minimize

Minimize the game so that user is back at the desktop
===============
*/
#[no_mangle]

pub unsafe extern "C" fn GLimp_Minimize() {
    SDL_MinimizeWindow(SDL_window);
}
/*
===============
GLimp_LogComment
===============
*/
#[no_mangle]

pub unsafe extern "C" fn GLimp_LogComment(mut _comment: *mut libc::c_char) {}
/*
===============
GLimp_CompareModes
===============
*/

unsafe extern "C" fn GLimp_CompareModes(
    mut a: *const libc::c_void,
    mut b: *const libc::c_void,
) -> i32 {
    let ASPECT_EPSILON: f32 = 0.001f32;
    let mut modeA: *mut SDL_Rect = a as *mut SDL_Rect;
    let mut modeB: *mut SDL_Rect = b as *mut SDL_Rect;
    let mut aspectA: f32 = (*modeA).w as f32 / (*modeA).h as f32;
    let mut aspectB: f32 = (*modeB).w as f32 / (*modeB).h as f32;
    let mut areaA: i32 = (*modeA).w * (*modeA).h;
    let mut areaB: i32 = (*modeB).w * (*modeB).h;
    let mut aspectDiffA: f32 =
        crate::stdlib::fabs((aspectA - crate::src::renderergl1::tr_init::displayAspect) as f64)
            as f32;
    let mut aspectDiffB: f32 =
        crate::stdlib::fabs((aspectB - crate::src::renderergl1::tr_init::displayAspect) as f64)
            as f32;
    let mut aspectDiffsDiff: f32 = aspectDiffA - aspectDiffB;
    if aspectDiffsDiff > ASPECT_EPSILON {
        return 1 as i32;
    } else if aspectDiffsDiff < -ASPECT_EPSILON {
        return -(1 as i32);
    } else {
        return areaA - areaB;
    };
}
/*
===============
GLimp_DetectAvailableModes
===============
*/

unsafe extern "C" fn GLimp_DetectAvailableModes() {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut buf: [libc::c_char; 1024] = [
        0 as i32 as libc::c_char,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    let mut numSDLModes: i32 = 0;
    let mut modes: *mut SDL_Rect = std::ptr::null_mut();
    let mut numModes: i32 = 0 as i32;
    let mut windowMode: SDL_DisplayMode = SDL_DisplayMode {
        format: 0,
        w: 0,
        h: 0,
        refresh_rate: 0,
        driverdata: std::ptr::null_mut(),
    };
    let mut display: i32 = SDL_GetWindowDisplayIndex(SDL_window);
    if display < 0 as i32 {
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            PRINT_WARNING as i32,
            b"Couldn\'t get window display index, no resolutions detected: %s\n\x00" as *const u8
                as *const libc::c_char,
            crate::stdlib::SDL_GetError(),
        );
        return;
    }
    numSDLModes = SDL_GetNumDisplayModes(display);
    if SDL_GetWindowDisplayMode(SDL_window, &mut windowMode) < 0 as i32 || numSDLModes <= 0 as i32 {
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            PRINT_WARNING as i32,
            b"Couldn\'t get window display mode, no resolutions detected: %s\n\x00" as *const u8
                as *const libc::c_char,
            crate::stdlib::SDL_GetError(),
        );
        return;
    }
    modes = SDL_calloc(
        numSDLModes as size_t,
        ::std::mem::size_of::<SDL_Rect>() as usize,
    ) as *mut SDL_Rect;
    if modes.is_null() {
        crate::src::renderergl1::tr_main::ri
            .Error
            .expect("non-null function pointer")(
            ERR_FATAL as i32,
            b"Out of memory\x00" as *const u8 as *const libc::c_char,
        );
    }
    i = 0 as i32;
    while i < numSDLModes {
        let mut mode: SDL_DisplayMode = SDL_DisplayMode {
            format: 0,
            w: 0,
            h: 0,
            refresh_rate: 0,
            driverdata: std::ptr::null_mut(),
        };
        if !(SDL_GetDisplayMode(display, i, &mut mode) < 0 as i32) {
            if mode.w == 0 || mode.h == 0 {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"Display supports any resolution\n\x00" as *const u8 as *const libc::c_char,
                );
                SDL_free(modes as *mut libc::c_void);
                return;
            }
            if !(windowMode.format != mode.format) {
                // SDL can give the same resolution with different refresh rates.
                // Only list resolution once.
                j = 0 as i32;
                while j < numModes {
                    if mode.w == (*modes.offset(j as isize)).w
                        && mode.h == (*modes.offset(j as isize)).h
                    {
                        break;
                    }
                    j += 1
                }
                if !(j != numModes) {
                    (*modes.offset(numModes as isize)).w = mode.w;
                    (*modes.offset(numModes as isize)).h = mode.h;
                    numModes += 1
                }
            }
        }
        i += 1
    }
    if numModes > 1 as i32 {
        qsort(
            modes as *mut libc::c_void,
            numModes as size_t,
            ::std::mem::size_of::<SDL_Rect>() as usize,
            Some(
                GLimp_CompareModes
                    as unsafe extern "C" fn(_: *const libc::c_void, _: *const libc::c_void) -> i32,
            ),
        );
    }
    i = 0 as i32;
    while i < numModes {
        let mut newModeString: *const libc::c_char = va(
            b"%ux%u \x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            (*modes.offset(i as isize)).w,
            (*modes.offset(i as isize)).h,
        );
        if crate::stdlib::strlen(newModeString)
            < (::std::mem::size_of::<[libc::c_char; 1024]>() as usize as i32 as usize)
                .wrapping_sub(crate::stdlib::strlen(buf.as_mut_ptr()))
        {
            Q_strcat(
                buf.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 1024]>() as usize as i32,
                newModeString,
            );
        } else {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_WARNING as i32,
                b"Skipping mode %ux%u, buffer too small\n\x00" as *const u8 as *const libc::c_char,
                (*modes.offset(i as isize)).w,
                (*modes.offset(i as isize)).h,
            );
        }
        i += 1
    }
    if *buf.as_mut_ptr() != 0 {
        buf[crate::stdlib::strlen(buf.as_mut_ptr()).wrapping_sub(1 as i32 as usize) as usize] =
            0 as i32 as libc::c_char;
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            PRINT_ALL as i32,
            b"Available modes: \'%s\'\n\x00" as *const u8 as *const libc::c_char,
            buf.as_mut_ptr(),
        );
        crate::src::renderergl1::tr_main::ri
            .Cvar_Set
            .expect("non-null function pointer")(
            b"r_availableModes\x00" as *const u8 as *const libc::c_char,
            buf.as_mut_ptr(),
        );
    }
    SDL_free(modes as *mut libc::c_void);
}
/*
===============
GLimp_GetProcAddresses

Get addresses for OpenGL functions.
===============
*/

unsafe extern "C" fn GLimp_GetProcAddresses(mut fixedFunction: qboolean) -> qboolean {
    let mut success: qboolean = qtrue;
    let mut version: *const libc::c_char = 0 as *const libc::c_char;
    // OpenGL 1.0 and OpenGL ES 1.0
    qglGetString = ::std::mem::transmute::<*mut libc::c_void, Option<GetStringproc>>(
        SDL_GL_GetProcAddress(b"glGetString\x00" as *const u8 as *const libc::c_char),
    ); // ES, ES-CM, or ES-CL
    if qglGetString.is_none() {
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            PRINT_ALL as i32,
            b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
            b"glGetString\x00" as *const u8 as *const libc::c_char,
        );
        success = qfalse
    }
    if qglGetString.is_none() {
        Com_Error(
            ERR_FATAL as i32,
            b"glGetString is NULL\x00" as *const u8 as *const libc::c_char,
        );
    }
    version = qglGetString.expect("non-null function pointer")(0x1f02 as i32 as GLenum)
        as *const libc::c_char;
    if version.is_null() {
        Com_Error(
            ERR_FATAL as i32,
            b"GL_VERSION is NULL\n\x00" as *const u8 as *const libc::c_char,
        );
    }
    if Q_stricmpn(
        b"OpenGL ES\x00" as *const u8 as *const libc::c_char,
        version,
        9 as i32,
    ) == 0 as i32
    {
        let mut profile: [libc::c_char; 6] = [0; 6];
        libc::sscanf(
            version,
            b"OpenGL %5s %d.%d\x00" as *const u8 as *const libc::c_char,
            profile.as_mut_ptr(),
            &mut qglesMajorVersion as *mut i32,
            &mut qglesMinorVersion as *mut i32,
        );
        // common lite profile (no floating point) is not supported
        if Q_stricmp(
            profile.as_mut_ptr(),
            b"ES-CL\x00" as *const u8 as *const libc::c_char,
        ) == 0 as i32
        {
            qglesMajorVersion = 0 as i32;
            qglesMinorVersion = 0 as i32
        }
    } else {
        libc::sscanf(
            version,
            b"%d.%d\x00" as *const u8 as *const libc::c_char,
            &mut qglMajorVersion as *mut i32,
            &mut qglMinorVersion as *mut i32,
        );
    }
    if fixedFunction as u64 != 0 {
        if qglMajorVersion > 1 as i32 || qglMajorVersion == 1 as i32 && qglMinorVersion >= 2 as i32
        {
            qglBindTexture = ::std::mem::transmute::<*mut libc::c_void, Option<BindTextureproc>>(
                SDL_GL_GetProcAddress(b"glBindTexture\x00" as *const u8 as *const libc::c_char),
            );
            if qglBindTexture.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glBindTexture\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglBlendFunc = ::std::mem::transmute::<*mut libc::c_void, Option<BlendFuncproc>>(
                SDL_GL_GetProcAddress(b"glBlendFunc\x00" as *const u8 as *const libc::c_char),
            );
            if qglBlendFunc.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glBlendFunc\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglClearColor = ::std::mem::transmute::<*mut libc::c_void, Option<ClearColorproc>>(
                SDL_GL_GetProcAddress(b"glClearColor\x00" as *const u8 as *const libc::c_char),
            );
            if qglClearColor.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glClearColor\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglClear = ::std::mem::transmute::<*mut libc::c_void, Option<Clearproc>>(
                SDL_GL_GetProcAddress(b"glClear\x00" as *const u8 as *const libc::c_char),
            );
            if qglClear.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glClear\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglClearStencil = ::std::mem::transmute::<*mut libc::c_void, Option<ClearStencilproc>>(
                SDL_GL_GetProcAddress(b"glClearStencil\x00" as *const u8 as *const libc::c_char),
            );
            if qglClearStencil.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glClearStencil\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglColorMask = ::std::mem::transmute::<*mut libc::c_void, Option<ColorMaskproc>>(
                SDL_GL_GetProcAddress(b"glColorMask\x00" as *const u8 as *const libc::c_char),
            );
            if qglColorMask.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glColorMask\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglCopyTexSubImage2D = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<CopyTexSubImage2Dproc>,
            >(SDL_GL_GetProcAddress(
                b"glCopyTexSubImage2D\x00" as *const u8 as *const libc::c_char,
            ));
            if qglCopyTexSubImage2D.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glCopyTexSubImage2D\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglCullFace = ::std::mem::transmute::<*mut libc::c_void, Option<CullFaceproc>>(
                SDL_GL_GetProcAddress(b"glCullFace\x00" as *const u8 as *const libc::c_char),
            );
            if qglCullFace.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glCullFace\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglDeleteTextures = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<DeleteTexturesproc>,
            >(SDL_GL_GetProcAddress(
                b"glDeleteTextures\x00" as *const u8 as *const libc::c_char,
            ));
            if qglDeleteTextures.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glDeleteTextures\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglDepthFunc = ::std::mem::transmute::<*mut libc::c_void, Option<DepthFuncproc>>(
                SDL_GL_GetProcAddress(b"glDepthFunc\x00" as *const u8 as *const libc::c_char),
            );
            if qglDepthFunc.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glDepthFunc\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglDepthMask = ::std::mem::transmute::<*mut libc::c_void, Option<DepthMaskproc>>(
                SDL_GL_GetProcAddress(b"glDepthMask\x00" as *const u8 as *const libc::c_char),
            );
            if qglDepthMask.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glDepthMask\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglDisable = ::std::mem::transmute::<*mut libc::c_void, Option<Disableproc>>(
                SDL_GL_GetProcAddress(b"glDisable\x00" as *const u8 as *const libc::c_char),
            );
            if qglDisable.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glDisable\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglDrawArrays = ::std::mem::transmute::<*mut libc::c_void, Option<DrawArraysproc>>(
                SDL_GL_GetProcAddress(b"glDrawArrays\x00" as *const u8 as *const libc::c_char),
            );
            if qglDrawArrays.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glDrawArrays\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglDrawElements = ::std::mem::transmute::<*mut libc::c_void, Option<DrawElementsproc>>(
                SDL_GL_GetProcAddress(b"glDrawElements\x00" as *const u8 as *const libc::c_char),
            );
            if qglDrawElements.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glDrawElements\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglEnable = ::std::mem::transmute::<*mut libc::c_void, Option<Enableproc>>(
                SDL_GL_GetProcAddress(b"glEnable\x00" as *const u8 as *const libc::c_char),
            );
            if qglEnable.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glEnable\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglFinish = ::std::mem::transmute::<*mut libc::c_void, Option<Finishproc>>(
                SDL_GL_GetProcAddress(b"glFinish\x00" as *const u8 as *const libc::c_char),
            );
            if qglFinish.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glFinish\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglFlush = ::std::mem::transmute::<*mut libc::c_void, Option<Flushproc>>(
                SDL_GL_GetProcAddress(b"glFlush\x00" as *const u8 as *const libc::c_char),
            );
            if qglFlush.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glFlush\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglGenTextures = ::std::mem::transmute::<*mut libc::c_void, Option<GenTexturesproc>>(
                SDL_GL_GetProcAddress(b"glGenTextures\x00" as *const u8 as *const libc::c_char),
            );
            if qglGenTextures.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glGenTextures\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglGetBooleanv = ::std::mem::transmute::<*mut libc::c_void, Option<GetBooleanvproc>>(
                SDL_GL_GetProcAddress(b"glGetBooleanv\x00" as *const u8 as *const libc::c_char),
            );
            if qglGetBooleanv.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glGetBooleanv\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglGetError = ::std::mem::transmute::<*mut libc::c_void, Option<GetErrorproc>>(
                SDL_GL_GetProcAddress(b"glGetError\x00" as *const u8 as *const libc::c_char),
            );
            if qglGetError.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glGetError\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglGetIntegerv = ::std::mem::transmute::<*mut libc::c_void, Option<GetIntegervproc>>(
                SDL_GL_GetProcAddress(b"glGetIntegerv\x00" as *const u8 as *const libc::c_char),
            );
            if qglGetIntegerv.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glGetIntegerv\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglGetString = ::std::mem::transmute::<*mut libc::c_void, Option<GetStringproc>>(
                SDL_GL_GetProcAddress(b"glGetString\x00" as *const u8 as *const libc::c_char),
            );
            if qglGetString.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glGetString\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglLineWidth = ::std::mem::transmute::<*mut libc::c_void, Option<LineWidthproc>>(
                SDL_GL_GetProcAddress(b"glLineWidth\x00" as *const u8 as *const libc::c_char),
            );
            if qglLineWidth.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glLineWidth\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglPolygonOffset = ::std::mem::transmute::<*mut libc::c_void, Option<PolygonOffsetproc>>(
                SDL_GL_GetProcAddress(b"glPolygonOffset\x00" as *const u8 as *const libc::c_char),
            );
            if qglPolygonOffset.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glPolygonOffset\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglReadPixels = ::std::mem::transmute::<*mut libc::c_void, Option<ReadPixelsproc>>(
                SDL_GL_GetProcAddress(b"glReadPixels\x00" as *const u8 as *const libc::c_char),
            );
            if qglReadPixels.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glReadPixels\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglScissor = ::std::mem::transmute::<*mut libc::c_void, Option<Scissorproc>>(
                SDL_GL_GetProcAddress(b"glScissor\x00" as *const u8 as *const libc::c_char),
            );
            if qglScissor.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glScissor\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglStencilFunc = ::std::mem::transmute::<*mut libc::c_void, Option<StencilFuncproc>>(
                SDL_GL_GetProcAddress(b"glStencilFunc\x00" as *const u8 as *const libc::c_char),
            );
            if qglStencilFunc.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glStencilFunc\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglStencilMask = ::std::mem::transmute::<*mut libc::c_void, Option<StencilMaskproc>>(
                SDL_GL_GetProcAddress(b"glStencilMask\x00" as *const u8 as *const libc::c_char),
            );
            if qglStencilMask.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glStencilMask\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglStencilOp = ::std::mem::transmute::<*mut libc::c_void, Option<StencilOpproc>>(
                SDL_GL_GetProcAddress(b"glStencilOp\x00" as *const u8 as *const libc::c_char),
            );
            if qglStencilOp.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glStencilOp\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglTexImage2D = ::std::mem::transmute::<*mut libc::c_void, Option<TexImage2Dproc>>(
                SDL_GL_GetProcAddress(b"glTexImage2D\x00" as *const u8 as *const libc::c_char),
            );
            if qglTexImage2D.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glTexImage2D\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglTexParameterf = ::std::mem::transmute::<*mut libc::c_void, Option<TexParameterfproc>>(
                SDL_GL_GetProcAddress(b"glTexParameterf\x00" as *const u8 as *const libc::c_char),
            );
            if qglTexParameterf.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glTexParameterf\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglTexParameteri = ::std::mem::transmute::<*mut libc::c_void, Option<TexParameteriproc>>(
                SDL_GL_GetProcAddress(b"glTexParameteri\x00" as *const u8 as *const libc::c_char),
            );
            if qglTexParameteri.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glTexParameteri\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglTexSubImage2D = ::std::mem::transmute::<*mut libc::c_void, Option<TexSubImage2Dproc>>(
                SDL_GL_GetProcAddress(b"glTexSubImage2D\x00" as *const u8 as *const libc::c_char),
            );
            if qglTexSubImage2D.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glTexSubImage2D\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglTranslatef = ::std::mem::transmute::<*mut libc::c_void, Option<Translatefproc>>(
                SDL_GL_GetProcAddress(b"glTranslatef\x00" as *const u8 as *const libc::c_char),
            );
            if qglTranslatef.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glTranslatef\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglViewport = ::std::mem::transmute::<*mut libc::c_void, Option<Viewportproc>>(
                SDL_GL_GetProcAddress(b"glViewport\x00" as *const u8 as *const libc::c_char),
            );
            if qglViewport.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glViewport\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglAlphaFunc = ::std::mem::transmute::<*mut libc::c_void, Option<AlphaFuncproc>>(
                SDL_GL_GetProcAddress(b"glAlphaFunc\x00" as *const u8 as *const libc::c_char),
            );
            if qglAlphaFunc.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glAlphaFunc\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglColor4f = ::std::mem::transmute::<*mut libc::c_void, Option<Color4fproc>>(
                SDL_GL_GetProcAddress(b"glColor4f\x00" as *const u8 as *const libc::c_char),
            );
            if qglColor4f.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glColor4f\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglColorPointer = ::std::mem::transmute::<*mut libc::c_void, Option<ColorPointerproc>>(
                SDL_GL_GetProcAddress(b"glColorPointer\x00" as *const u8 as *const libc::c_char),
            );
            if qglColorPointer.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glColorPointer\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglDisableClientState = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<DisableClientStateproc>,
            >(SDL_GL_GetProcAddress(
                b"glDisableClientState\x00" as *const u8 as *const libc::c_char,
            ));
            if qglDisableClientState.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glDisableClientState\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglEnableClientState = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<EnableClientStateproc>,
            >(SDL_GL_GetProcAddress(
                b"glEnableClientState\x00" as *const u8 as *const libc::c_char,
            ));
            if qglEnableClientState.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glEnableClientState\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglLoadIdentity = ::std::mem::transmute::<*mut libc::c_void, Option<LoadIdentityproc>>(
                SDL_GL_GetProcAddress(b"glLoadIdentity\x00" as *const u8 as *const libc::c_char),
            );
            if qglLoadIdentity.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glLoadIdentity\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglLoadMatrixf = ::std::mem::transmute::<*mut libc::c_void, Option<LoadMatrixfproc>>(
                SDL_GL_GetProcAddress(b"glLoadMatrixf\x00" as *const u8 as *const libc::c_char),
            );
            if qglLoadMatrixf.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glLoadMatrixf\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglMatrixMode = ::std::mem::transmute::<*mut libc::c_void, Option<MatrixModeproc>>(
                SDL_GL_GetProcAddress(b"glMatrixMode\x00" as *const u8 as *const libc::c_char),
            );
            if qglMatrixMode.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glMatrixMode\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglPopMatrix = ::std::mem::transmute::<*mut libc::c_void, Option<PopMatrixproc>>(
                SDL_GL_GetProcAddress(b"glPopMatrix\x00" as *const u8 as *const libc::c_char),
            );
            if qglPopMatrix.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glPopMatrix\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglPushMatrix = ::std::mem::transmute::<*mut libc::c_void, Option<PushMatrixproc>>(
                SDL_GL_GetProcAddress(b"glPushMatrix\x00" as *const u8 as *const libc::c_char),
            );
            if qglPushMatrix.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glPushMatrix\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglShadeModel = ::std::mem::transmute::<*mut libc::c_void, Option<ShadeModelproc>>(
                SDL_GL_GetProcAddress(b"glShadeModel\x00" as *const u8 as *const libc::c_char),
            );
            if qglShadeModel.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glShadeModel\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglTexCoordPointer = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<TexCoordPointerproc>,
            >(SDL_GL_GetProcAddress(
                b"glTexCoordPointer\x00" as *const u8 as *const libc::c_char,
            ));
            if qglTexCoordPointer.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glTexCoordPointer\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglTexEnvf = ::std::mem::transmute::<*mut libc::c_void, Option<TexEnvfproc>>(
                SDL_GL_GetProcAddress(b"glTexEnvf\x00" as *const u8 as *const libc::c_char),
            );
            if qglTexEnvf.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glTexEnvf\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglVertexPointer = ::std::mem::transmute::<*mut libc::c_void, Option<VertexPointerproc>>(
                SDL_GL_GetProcAddress(b"glVertexPointer\x00" as *const u8 as *const libc::c_char),
            );
            if qglVertexPointer.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glVertexPointer\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglClearDepth = ::std::mem::transmute::<*mut libc::c_void, Option<ClearDepthproc>>(
                SDL_GL_GetProcAddress(b"glClearDepth\x00" as *const u8 as *const libc::c_char),
            );
            if qglClearDepth.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glClearDepth\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglDepthRange = ::std::mem::transmute::<*mut libc::c_void, Option<DepthRangeproc>>(
                SDL_GL_GetProcAddress(b"glDepthRange\x00" as *const u8 as *const libc::c_char),
            );
            if qglDepthRange.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glDepthRange\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglDrawBuffer = ::std::mem::transmute::<*mut libc::c_void, Option<DrawBufferproc>>(
                SDL_GL_GetProcAddress(b"glDrawBuffer\x00" as *const u8 as *const libc::c_char),
            );
            if qglDrawBuffer.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glDrawBuffer\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglPolygonMode = ::std::mem::transmute::<*mut libc::c_void, Option<PolygonModeproc>>(
                SDL_GL_GetProcAddress(b"glPolygonMode\x00" as *const u8 as *const libc::c_char),
            );
            if qglPolygonMode.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glPolygonMode\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglArrayElement = ::std::mem::transmute::<*mut libc::c_void, Option<ArrayElementproc>>(
                SDL_GL_GetProcAddress(b"glArrayElement\x00" as *const u8 as *const libc::c_char),
            );
            if qglArrayElement.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glArrayElement\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglBegin = ::std::mem::transmute::<*mut libc::c_void, Option<Beginproc>>(
                SDL_GL_GetProcAddress(b"glBegin\x00" as *const u8 as *const libc::c_char),
            );
            if qglBegin.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glBegin\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglClipPlane = ::std::mem::transmute::<*mut libc::c_void, Option<ClipPlaneproc>>(
                SDL_GL_GetProcAddress(b"glClipPlane\x00" as *const u8 as *const libc::c_char),
            );
            if qglClipPlane.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glClipPlane\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglColor3f = ::std::mem::transmute::<*mut libc::c_void, Option<Color3fproc>>(
                SDL_GL_GetProcAddress(b"glColor3f\x00" as *const u8 as *const libc::c_char),
            );
            if qglColor3f.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glColor3f\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglColor4ubv = ::std::mem::transmute::<*mut libc::c_void, Option<Color4ubvproc>>(
                SDL_GL_GetProcAddress(b"glColor4ubv\x00" as *const u8 as *const libc::c_char),
            );
            if qglColor4ubv.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glColor4ubv\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglEnd = ::std::mem::transmute::<*mut libc::c_void, Option<Endproc>>(
                SDL_GL_GetProcAddress(b"glEnd\x00" as *const u8 as *const libc::c_char),
            );
            if qglEnd.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glEnd\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglFrustum = ::std::mem::transmute::<*mut libc::c_void, Option<Frustumproc>>(
                SDL_GL_GetProcAddress(b"glFrustum\x00" as *const u8 as *const libc::c_char),
            );
            if qglFrustum.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glFrustum\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglOrtho = ::std::mem::transmute::<*mut libc::c_void, Option<Orthoproc>>(
                SDL_GL_GetProcAddress(b"glOrtho\x00" as *const u8 as *const libc::c_char),
            );
            if qglOrtho.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glOrtho\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglTexCoord2f = ::std::mem::transmute::<*mut libc::c_void, Option<TexCoord2fproc>>(
                SDL_GL_GetProcAddress(b"glTexCoord2f\x00" as *const u8 as *const libc::c_char),
            );
            if qglTexCoord2f.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glTexCoord2f\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglTexCoord2fv = ::std::mem::transmute::<*mut libc::c_void, Option<TexCoord2fvproc>>(
                SDL_GL_GetProcAddress(b"glTexCoord2fv\x00" as *const u8 as *const libc::c_char),
            );
            if qglTexCoord2fv.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glTexCoord2fv\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglVertex2f = ::std::mem::transmute::<*mut libc::c_void, Option<Vertex2fproc>>(
                SDL_GL_GetProcAddress(b"glVertex2f\x00" as *const u8 as *const libc::c_char),
            );
            if qglVertex2f.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glVertex2f\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglVertex3f = ::std::mem::transmute::<*mut libc::c_void, Option<Vertex3fproc>>(
                SDL_GL_GetProcAddress(b"glVertex3f\x00" as *const u8 as *const libc::c_char),
            );
            if qglVertex3f.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glVertex3f\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglVertex3fv = ::std::mem::transmute::<*mut libc::c_void, Option<Vertex3fvproc>>(
                SDL_GL_GetProcAddress(b"glVertex3fv\x00" as *const u8 as *const libc::c_char),
            );
            if qglVertex3fv.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glVertex3fv\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
        } else if qglesMajorVersion == 1 as i32 && qglesMinorVersion >= 1 as i32 {
            // OpenGL ES 1.1 (2.0 is not backward compatible)
            qglBindTexture = ::std::mem::transmute::<*mut libc::c_void, Option<BindTextureproc>>(
                SDL_GL_GetProcAddress(b"glBindTexture\x00" as *const u8 as *const libc::c_char),
            );
            if qglBindTexture.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glBindTexture\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglBlendFunc = ::std::mem::transmute::<*mut libc::c_void, Option<BlendFuncproc>>(
                SDL_GL_GetProcAddress(b"glBlendFunc\x00" as *const u8 as *const libc::c_char),
            );
            if qglBlendFunc.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glBlendFunc\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglClearColor = ::std::mem::transmute::<*mut libc::c_void, Option<ClearColorproc>>(
                SDL_GL_GetProcAddress(b"glClearColor\x00" as *const u8 as *const libc::c_char),
            );
            if qglClearColor.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glClearColor\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglClear = ::std::mem::transmute::<*mut libc::c_void, Option<Clearproc>>(
                SDL_GL_GetProcAddress(b"glClear\x00" as *const u8 as *const libc::c_char),
            );
            if qglClear.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glClear\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglClearStencil = ::std::mem::transmute::<*mut libc::c_void, Option<ClearStencilproc>>(
                SDL_GL_GetProcAddress(b"glClearStencil\x00" as *const u8 as *const libc::c_char),
            );
            if qglClearStencil.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glClearStencil\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglColorMask = ::std::mem::transmute::<*mut libc::c_void, Option<ColorMaskproc>>(
                SDL_GL_GetProcAddress(b"glColorMask\x00" as *const u8 as *const libc::c_char),
            );
            if qglColorMask.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glColorMask\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglCopyTexSubImage2D = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<CopyTexSubImage2Dproc>,
            >(SDL_GL_GetProcAddress(
                b"glCopyTexSubImage2D\x00" as *const u8 as *const libc::c_char,
            ));
            if qglCopyTexSubImage2D.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glCopyTexSubImage2D\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglCullFace = ::std::mem::transmute::<*mut libc::c_void, Option<CullFaceproc>>(
                SDL_GL_GetProcAddress(b"glCullFace\x00" as *const u8 as *const libc::c_char),
            );
            if qglCullFace.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glCullFace\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglDeleteTextures = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<DeleteTexturesproc>,
            >(SDL_GL_GetProcAddress(
                b"glDeleteTextures\x00" as *const u8 as *const libc::c_char,
            ));
            if qglDeleteTextures.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glDeleteTextures\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglDepthFunc = ::std::mem::transmute::<*mut libc::c_void, Option<DepthFuncproc>>(
                SDL_GL_GetProcAddress(b"glDepthFunc\x00" as *const u8 as *const libc::c_char),
            );
            if qglDepthFunc.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glDepthFunc\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglDepthMask = ::std::mem::transmute::<*mut libc::c_void, Option<DepthMaskproc>>(
                SDL_GL_GetProcAddress(b"glDepthMask\x00" as *const u8 as *const libc::c_char),
            );
            if qglDepthMask.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glDepthMask\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglDisable = ::std::mem::transmute::<*mut libc::c_void, Option<Disableproc>>(
                SDL_GL_GetProcAddress(b"glDisable\x00" as *const u8 as *const libc::c_char),
            );
            if qglDisable.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glDisable\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglDrawArrays = ::std::mem::transmute::<*mut libc::c_void, Option<DrawArraysproc>>(
                SDL_GL_GetProcAddress(b"glDrawArrays\x00" as *const u8 as *const libc::c_char),
            );
            if qglDrawArrays.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glDrawArrays\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglDrawElements = ::std::mem::transmute::<*mut libc::c_void, Option<DrawElementsproc>>(
                SDL_GL_GetProcAddress(b"glDrawElements\x00" as *const u8 as *const libc::c_char),
            );
            if qglDrawElements.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glDrawElements\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglEnable = ::std::mem::transmute::<*mut libc::c_void, Option<Enableproc>>(
                SDL_GL_GetProcAddress(b"glEnable\x00" as *const u8 as *const libc::c_char),
            );
            if qglEnable.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glEnable\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglFinish = ::std::mem::transmute::<*mut libc::c_void, Option<Finishproc>>(
                SDL_GL_GetProcAddress(b"glFinish\x00" as *const u8 as *const libc::c_char),
            );
            if qglFinish.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glFinish\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglFlush = ::std::mem::transmute::<*mut libc::c_void, Option<Flushproc>>(
                SDL_GL_GetProcAddress(b"glFlush\x00" as *const u8 as *const libc::c_char),
            );
            if qglFlush.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glFlush\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglGenTextures = ::std::mem::transmute::<*mut libc::c_void, Option<GenTexturesproc>>(
                SDL_GL_GetProcAddress(b"glGenTextures\x00" as *const u8 as *const libc::c_char),
            );
            if qglGenTextures.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glGenTextures\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglGetBooleanv = ::std::mem::transmute::<*mut libc::c_void, Option<GetBooleanvproc>>(
                SDL_GL_GetProcAddress(b"glGetBooleanv\x00" as *const u8 as *const libc::c_char),
            );
            if qglGetBooleanv.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glGetBooleanv\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglGetError = ::std::mem::transmute::<*mut libc::c_void, Option<GetErrorproc>>(
                SDL_GL_GetProcAddress(b"glGetError\x00" as *const u8 as *const libc::c_char),
            );
            if qglGetError.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glGetError\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglGetIntegerv = ::std::mem::transmute::<*mut libc::c_void, Option<GetIntegervproc>>(
                SDL_GL_GetProcAddress(b"glGetIntegerv\x00" as *const u8 as *const libc::c_char),
            );
            if qglGetIntegerv.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glGetIntegerv\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglGetString = ::std::mem::transmute::<*mut libc::c_void, Option<GetStringproc>>(
                SDL_GL_GetProcAddress(b"glGetString\x00" as *const u8 as *const libc::c_char),
            );
            if qglGetString.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glGetString\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglLineWidth = ::std::mem::transmute::<*mut libc::c_void, Option<LineWidthproc>>(
                SDL_GL_GetProcAddress(b"glLineWidth\x00" as *const u8 as *const libc::c_char),
            );
            if qglLineWidth.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glLineWidth\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglPolygonOffset = ::std::mem::transmute::<*mut libc::c_void, Option<PolygonOffsetproc>>(
                SDL_GL_GetProcAddress(b"glPolygonOffset\x00" as *const u8 as *const libc::c_char),
            );
            if qglPolygonOffset.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glPolygonOffset\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglReadPixels = ::std::mem::transmute::<*mut libc::c_void, Option<ReadPixelsproc>>(
                SDL_GL_GetProcAddress(b"glReadPixels\x00" as *const u8 as *const libc::c_char),
            );
            if qglReadPixels.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glReadPixels\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglScissor = ::std::mem::transmute::<*mut libc::c_void, Option<Scissorproc>>(
                SDL_GL_GetProcAddress(b"glScissor\x00" as *const u8 as *const libc::c_char),
            );
            if qglScissor.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glScissor\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglStencilFunc = ::std::mem::transmute::<*mut libc::c_void, Option<StencilFuncproc>>(
                SDL_GL_GetProcAddress(b"glStencilFunc\x00" as *const u8 as *const libc::c_char),
            );
            if qglStencilFunc.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glStencilFunc\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglStencilMask = ::std::mem::transmute::<*mut libc::c_void, Option<StencilMaskproc>>(
                SDL_GL_GetProcAddress(b"glStencilMask\x00" as *const u8 as *const libc::c_char),
            );
            if qglStencilMask.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glStencilMask\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglStencilOp = ::std::mem::transmute::<*mut libc::c_void, Option<StencilOpproc>>(
                SDL_GL_GetProcAddress(b"glStencilOp\x00" as *const u8 as *const libc::c_char),
            );
            if qglStencilOp.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glStencilOp\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglTexImage2D = ::std::mem::transmute::<*mut libc::c_void, Option<TexImage2Dproc>>(
                SDL_GL_GetProcAddress(b"glTexImage2D\x00" as *const u8 as *const libc::c_char),
            );
            if qglTexImage2D.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glTexImage2D\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglTexParameterf = ::std::mem::transmute::<*mut libc::c_void, Option<TexParameterfproc>>(
                SDL_GL_GetProcAddress(b"glTexParameterf\x00" as *const u8 as *const libc::c_char),
            );
            if qglTexParameterf.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glTexParameterf\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglTexParameteri = ::std::mem::transmute::<*mut libc::c_void, Option<TexParameteriproc>>(
                SDL_GL_GetProcAddress(b"glTexParameteri\x00" as *const u8 as *const libc::c_char),
            );
            if qglTexParameteri.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glTexParameteri\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglTexSubImage2D = ::std::mem::transmute::<*mut libc::c_void, Option<TexSubImage2Dproc>>(
                SDL_GL_GetProcAddress(b"glTexSubImage2D\x00" as *const u8 as *const libc::c_char),
            );
            if qglTexSubImage2D.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glTexSubImage2D\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglTranslatef = ::std::mem::transmute::<*mut libc::c_void, Option<Translatefproc>>(
                SDL_GL_GetProcAddress(b"glTranslatef\x00" as *const u8 as *const libc::c_char),
            );
            if qglTranslatef.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glTranslatef\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglViewport = ::std::mem::transmute::<*mut libc::c_void, Option<Viewportproc>>(
                SDL_GL_GetProcAddress(b"glViewport\x00" as *const u8 as *const libc::c_char),
            );
            if qglViewport.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glViewport\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglAlphaFunc = ::std::mem::transmute::<*mut libc::c_void, Option<AlphaFuncproc>>(
                SDL_GL_GetProcAddress(b"glAlphaFunc\x00" as *const u8 as *const libc::c_char),
            );
            if qglAlphaFunc.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glAlphaFunc\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglColor4f = ::std::mem::transmute::<*mut libc::c_void, Option<Color4fproc>>(
                SDL_GL_GetProcAddress(b"glColor4f\x00" as *const u8 as *const libc::c_char),
            );
            if qglColor4f.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glColor4f\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglColorPointer = ::std::mem::transmute::<*mut libc::c_void, Option<ColorPointerproc>>(
                SDL_GL_GetProcAddress(b"glColorPointer\x00" as *const u8 as *const libc::c_char),
            );
            if qglColorPointer.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glColorPointer\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglDisableClientState = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<DisableClientStateproc>,
            >(SDL_GL_GetProcAddress(
                b"glDisableClientState\x00" as *const u8 as *const libc::c_char,
            ));
            if qglDisableClientState.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glDisableClientState\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglEnableClientState = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<EnableClientStateproc>,
            >(SDL_GL_GetProcAddress(
                b"glEnableClientState\x00" as *const u8 as *const libc::c_char,
            ));
            if qglEnableClientState.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glEnableClientState\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglLoadIdentity = ::std::mem::transmute::<*mut libc::c_void, Option<LoadIdentityproc>>(
                SDL_GL_GetProcAddress(b"glLoadIdentity\x00" as *const u8 as *const libc::c_char),
            );
            if qglLoadIdentity.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glLoadIdentity\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglLoadMatrixf = ::std::mem::transmute::<*mut libc::c_void, Option<LoadMatrixfproc>>(
                SDL_GL_GetProcAddress(b"glLoadMatrixf\x00" as *const u8 as *const libc::c_char),
            );
            if qglLoadMatrixf.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glLoadMatrixf\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglMatrixMode = ::std::mem::transmute::<*mut libc::c_void, Option<MatrixModeproc>>(
                SDL_GL_GetProcAddress(b"glMatrixMode\x00" as *const u8 as *const libc::c_char),
            );
            if qglMatrixMode.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glMatrixMode\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglPopMatrix = ::std::mem::transmute::<*mut libc::c_void, Option<PopMatrixproc>>(
                SDL_GL_GetProcAddress(b"glPopMatrix\x00" as *const u8 as *const libc::c_char),
            );
            if qglPopMatrix.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glPopMatrix\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglPushMatrix = ::std::mem::transmute::<*mut libc::c_void, Option<PushMatrixproc>>(
                SDL_GL_GetProcAddress(b"glPushMatrix\x00" as *const u8 as *const libc::c_char),
            );
            if qglPushMatrix.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glPushMatrix\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglShadeModel = ::std::mem::transmute::<*mut libc::c_void, Option<ShadeModelproc>>(
                SDL_GL_GetProcAddress(b"glShadeModel\x00" as *const u8 as *const libc::c_char),
            );
            if qglShadeModel.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glShadeModel\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglTexCoordPointer = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<TexCoordPointerproc>,
            >(SDL_GL_GetProcAddress(
                b"glTexCoordPointer\x00" as *const u8 as *const libc::c_char,
            ));
            if qglTexCoordPointer.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glTexCoordPointer\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglTexEnvf = ::std::mem::transmute::<*mut libc::c_void, Option<TexEnvfproc>>(
                SDL_GL_GetProcAddress(b"glTexEnvf\x00" as *const u8 as *const libc::c_char),
            );
            if qglTexEnvf.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glTexEnvf\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglVertexPointer = ::std::mem::transmute::<*mut libc::c_void, Option<VertexPointerproc>>(
                SDL_GL_GetProcAddress(b"glVertexPointer\x00" as *const u8 as *const libc::c_char),
            );
            if qglVertexPointer.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glVertexPointer\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglClearDepthf = ::std::mem::transmute::<*mut libc::c_void, Option<ClearDepthfproc>>(
                SDL_GL_GetProcAddress(b"glClearDepthf\x00" as *const u8 as *const libc::c_char),
            );
            if qglClearDepthf.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glClearDepthf\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglDepthRangef = ::std::mem::transmute::<*mut libc::c_void, Option<DepthRangefproc>>(
                SDL_GL_GetProcAddress(b"glDepthRangef\x00" as *const u8 as *const libc::c_char),
            );
            if qglDepthRangef.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glDepthRangef\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglClipPlanef = ::std::mem::transmute::<*mut libc::c_void, Option<ClipPlanefproc>>(
                SDL_GL_GetProcAddress(b"glClipPlanef\x00" as *const u8 as *const libc::c_char),
            );
            if qglClipPlanef.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glClipPlanef\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglFrustumf = ::std::mem::transmute::<*mut libc::c_void, Option<Frustumfproc>>(
                SDL_GL_GetProcAddress(b"glFrustumf\x00" as *const u8 as *const libc::c_char),
            );
            if qglFrustumf.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glFrustumf\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            qglOrthof = ::std::mem::transmute::<*mut libc::c_void, Option<Orthofproc>>(
                SDL_GL_GetProcAddress(b"glOrthof\x00" as *const u8 as *const libc::c_char),
            );
            if qglOrthof.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                    b"glOrthof\x00" as *const u8 as *const libc::c_char,
                );
                success = qfalse
            }
            // error so this doesn't segfault due to NULL desktop GL functions being used
            Com_Error(
                ERR_FATAL as i32,
                b"Unsupported OpenGL Version: %s\n\x00" as *const u8 as *const libc::c_char,
                version,
            );
        } else {
            Com_Error(
                ERR_FATAL as i32,
                b"Unsupported OpenGL Version (%s), OpenGL 1.2 is required\n\x00" as *const u8
                    as *const libc::c_char,
                version,
            );
        }
    } else if qglMajorVersion > 2 as i32
        || qglMajorVersion == 2 as i32 && qglMinorVersion >= 0 as i32
    {
        qglBindTexture = ::std::mem::transmute::<*mut libc::c_void, Option<BindTextureproc>>(
            SDL_GL_GetProcAddress(b"glBindTexture\x00" as *const u8 as *const libc::c_char),
        );
        if qglBindTexture.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glBindTexture\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglBlendFunc = ::std::mem::transmute::<*mut libc::c_void, Option<BlendFuncproc>>(
            SDL_GL_GetProcAddress(b"glBlendFunc\x00" as *const u8 as *const libc::c_char),
        );
        if qglBlendFunc.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glBlendFunc\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglClearColor = ::std::mem::transmute::<*mut libc::c_void, Option<ClearColorproc>>(
            SDL_GL_GetProcAddress(b"glClearColor\x00" as *const u8 as *const libc::c_char),
        );
        if qglClearColor.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glClearColor\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglClear = ::std::mem::transmute::<*mut libc::c_void, Option<Clearproc>>(
            SDL_GL_GetProcAddress(b"glClear\x00" as *const u8 as *const libc::c_char),
        );
        if qglClear.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glClear\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglClearStencil = ::std::mem::transmute::<*mut libc::c_void, Option<ClearStencilproc>>(
            SDL_GL_GetProcAddress(b"glClearStencil\x00" as *const u8 as *const libc::c_char),
        );
        if qglClearStencil.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glClearStencil\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglColorMask = ::std::mem::transmute::<*mut libc::c_void, Option<ColorMaskproc>>(
            SDL_GL_GetProcAddress(b"glColorMask\x00" as *const u8 as *const libc::c_char),
        );
        if qglColorMask.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glColorMask\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglCopyTexSubImage2D = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<CopyTexSubImage2Dproc>,
        >(SDL_GL_GetProcAddress(
            b"glCopyTexSubImage2D\x00" as *const u8 as *const libc::c_char,
        ));
        if qglCopyTexSubImage2D.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glCopyTexSubImage2D\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglCullFace = ::std::mem::transmute::<*mut libc::c_void, Option<CullFaceproc>>(
            SDL_GL_GetProcAddress(b"glCullFace\x00" as *const u8 as *const libc::c_char),
        );
        if qglCullFace.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glCullFace\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglDeleteTextures = ::std::mem::transmute::<*mut libc::c_void, Option<DeleteTexturesproc>>(
            SDL_GL_GetProcAddress(b"glDeleteTextures\x00" as *const u8 as *const libc::c_char),
        );
        if qglDeleteTextures.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glDeleteTextures\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglDepthFunc = ::std::mem::transmute::<*mut libc::c_void, Option<DepthFuncproc>>(
            SDL_GL_GetProcAddress(b"glDepthFunc\x00" as *const u8 as *const libc::c_char),
        );
        if qglDepthFunc.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glDepthFunc\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglDepthMask = ::std::mem::transmute::<*mut libc::c_void, Option<DepthMaskproc>>(
            SDL_GL_GetProcAddress(b"glDepthMask\x00" as *const u8 as *const libc::c_char),
        );
        if qglDepthMask.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glDepthMask\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglDisable = ::std::mem::transmute::<*mut libc::c_void, Option<Disableproc>>(
            SDL_GL_GetProcAddress(b"glDisable\x00" as *const u8 as *const libc::c_char),
        );
        if qglDisable.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glDisable\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglDrawArrays = ::std::mem::transmute::<*mut libc::c_void, Option<DrawArraysproc>>(
            SDL_GL_GetProcAddress(b"glDrawArrays\x00" as *const u8 as *const libc::c_char),
        );
        if qglDrawArrays.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glDrawArrays\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglDrawElements = ::std::mem::transmute::<*mut libc::c_void, Option<DrawElementsproc>>(
            SDL_GL_GetProcAddress(b"glDrawElements\x00" as *const u8 as *const libc::c_char),
        );
        if qglDrawElements.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glDrawElements\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglEnable = ::std::mem::transmute::<*mut libc::c_void, Option<Enableproc>>(
            SDL_GL_GetProcAddress(b"glEnable\x00" as *const u8 as *const libc::c_char),
        );
        if qglEnable.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glEnable\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglFinish = ::std::mem::transmute::<*mut libc::c_void, Option<Finishproc>>(
            SDL_GL_GetProcAddress(b"glFinish\x00" as *const u8 as *const libc::c_char),
        );
        if qglFinish.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glFinish\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglFlush = ::std::mem::transmute::<*mut libc::c_void, Option<Flushproc>>(
            SDL_GL_GetProcAddress(b"glFlush\x00" as *const u8 as *const libc::c_char),
        );
        if qglFlush.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glFlush\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglGenTextures = ::std::mem::transmute::<*mut libc::c_void, Option<GenTexturesproc>>(
            SDL_GL_GetProcAddress(b"glGenTextures\x00" as *const u8 as *const libc::c_char),
        );
        if qglGenTextures.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glGenTextures\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglGetBooleanv = ::std::mem::transmute::<*mut libc::c_void, Option<GetBooleanvproc>>(
            SDL_GL_GetProcAddress(b"glGetBooleanv\x00" as *const u8 as *const libc::c_char),
        );
        if qglGetBooleanv.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glGetBooleanv\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglGetError = ::std::mem::transmute::<*mut libc::c_void, Option<GetErrorproc>>(
            SDL_GL_GetProcAddress(b"glGetError\x00" as *const u8 as *const libc::c_char),
        );
        if qglGetError.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glGetError\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglGetIntegerv = ::std::mem::transmute::<*mut libc::c_void, Option<GetIntegervproc>>(
            SDL_GL_GetProcAddress(b"glGetIntegerv\x00" as *const u8 as *const libc::c_char),
        );
        if qglGetIntegerv.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glGetIntegerv\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglGetString = ::std::mem::transmute::<*mut libc::c_void, Option<GetStringproc>>(
            SDL_GL_GetProcAddress(b"glGetString\x00" as *const u8 as *const libc::c_char),
        );
        if qglGetString.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glGetString\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglLineWidth = ::std::mem::transmute::<*mut libc::c_void, Option<LineWidthproc>>(
            SDL_GL_GetProcAddress(b"glLineWidth\x00" as *const u8 as *const libc::c_char),
        );
        if qglLineWidth.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glLineWidth\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglPolygonOffset = ::std::mem::transmute::<*mut libc::c_void, Option<PolygonOffsetproc>>(
            SDL_GL_GetProcAddress(b"glPolygonOffset\x00" as *const u8 as *const libc::c_char),
        );
        if qglPolygonOffset.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glPolygonOffset\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglReadPixels = ::std::mem::transmute::<*mut libc::c_void, Option<ReadPixelsproc>>(
            SDL_GL_GetProcAddress(b"glReadPixels\x00" as *const u8 as *const libc::c_char),
        );
        if qglReadPixels.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glReadPixels\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglScissor = ::std::mem::transmute::<*mut libc::c_void, Option<Scissorproc>>(
            SDL_GL_GetProcAddress(b"glScissor\x00" as *const u8 as *const libc::c_char),
        );
        if qglScissor.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glScissor\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglStencilFunc = ::std::mem::transmute::<*mut libc::c_void, Option<StencilFuncproc>>(
            SDL_GL_GetProcAddress(b"glStencilFunc\x00" as *const u8 as *const libc::c_char),
        );
        if qglStencilFunc.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glStencilFunc\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglStencilMask = ::std::mem::transmute::<*mut libc::c_void, Option<StencilMaskproc>>(
            SDL_GL_GetProcAddress(b"glStencilMask\x00" as *const u8 as *const libc::c_char),
        );
        if qglStencilMask.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glStencilMask\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglStencilOp = ::std::mem::transmute::<*mut libc::c_void, Option<StencilOpproc>>(
            SDL_GL_GetProcAddress(b"glStencilOp\x00" as *const u8 as *const libc::c_char),
        );
        if qglStencilOp.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glStencilOp\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglTexImage2D = ::std::mem::transmute::<*mut libc::c_void, Option<TexImage2Dproc>>(
            SDL_GL_GetProcAddress(b"glTexImage2D\x00" as *const u8 as *const libc::c_char),
        );
        if qglTexImage2D.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glTexImage2D\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglTexParameterf = ::std::mem::transmute::<*mut libc::c_void, Option<TexParameterfproc>>(
            SDL_GL_GetProcAddress(b"glTexParameterf\x00" as *const u8 as *const libc::c_char),
        );
        if qglTexParameterf.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glTexParameterf\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglTexParameteri = ::std::mem::transmute::<*mut libc::c_void, Option<TexParameteriproc>>(
            SDL_GL_GetProcAddress(b"glTexParameteri\x00" as *const u8 as *const libc::c_char),
        );
        if qglTexParameteri.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glTexParameteri\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglTexSubImage2D = ::std::mem::transmute::<*mut libc::c_void, Option<TexSubImage2Dproc>>(
            SDL_GL_GetProcAddress(b"glTexSubImage2D\x00" as *const u8 as *const libc::c_char),
        );
        if qglTexSubImage2D.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glTexSubImage2D\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglTranslatef = ::std::mem::transmute::<*mut libc::c_void, Option<Translatefproc>>(
            SDL_GL_GetProcAddress(b"glTranslatef\x00" as *const u8 as *const libc::c_char),
        );
        if qglTranslatef.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glTranslatef\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglViewport = ::std::mem::transmute::<*mut libc::c_void, Option<Viewportproc>>(
            SDL_GL_GetProcAddress(b"glViewport\x00" as *const u8 as *const libc::c_char),
        );
        if qglViewport.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glViewport\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglClearDepth = ::std::mem::transmute::<*mut libc::c_void, Option<ClearDepthproc>>(
            SDL_GL_GetProcAddress(b"glClearDepth\x00" as *const u8 as *const libc::c_char),
        );
        if qglClearDepth.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glClearDepth\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglDepthRange = ::std::mem::transmute::<*mut libc::c_void, Option<DepthRangeproc>>(
            SDL_GL_GetProcAddress(b"glDepthRange\x00" as *const u8 as *const libc::c_char),
        );
        if qglDepthRange.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glDepthRange\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglDrawBuffer = ::std::mem::transmute::<*mut libc::c_void, Option<DrawBufferproc>>(
            SDL_GL_GetProcAddress(b"glDrawBuffer\x00" as *const u8 as *const libc::c_char),
        );
        if qglDrawBuffer.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glDrawBuffer\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglPolygonMode = ::std::mem::transmute::<*mut libc::c_void, Option<PolygonModeproc>>(
            SDL_GL_GetProcAddress(b"glPolygonMode\x00" as *const u8 as *const libc::c_char),
        );
        if qglPolygonMode.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glPolygonMode\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglActiveTexture = ::std::mem::transmute::<*mut libc::c_void, Option<ActiveTextureproc>>(
            SDL_GL_GetProcAddress(b"glActiveTexture\x00" as *const u8 as *const libc::c_char),
        );
        if qglActiveTexture.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glActiveTexture\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglCompressedTexImage2D = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<CompressedTexImage2Dproc>,
        >(SDL_GL_GetProcAddress(
            b"glCompressedTexImage2D\x00" as *const u8 as *const libc::c_char,
        ));
        if qglCompressedTexImage2D.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glCompressedTexImage2D\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglCompressedTexSubImage2D = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<CompressedTexSubImage2Dproc>,
        >(SDL_GL_GetProcAddress(
            b"glCompressedTexSubImage2D\x00" as *const u8 as *const libc::c_char,
        ));
        if qglCompressedTexSubImage2D.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glCompressedTexSubImage2D\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglBindBuffer = ::std::mem::transmute::<*mut libc::c_void, Option<BindBufferproc>>(
            SDL_GL_GetProcAddress(b"glBindBuffer\x00" as *const u8 as *const libc::c_char),
        );
        if qglBindBuffer.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glBindBuffer\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglDeleteBuffers = ::std::mem::transmute::<*mut libc::c_void, Option<DeleteBuffersproc>>(
            SDL_GL_GetProcAddress(b"glDeleteBuffers\x00" as *const u8 as *const libc::c_char),
        );
        if qglDeleteBuffers.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glDeleteBuffers\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglGenBuffers = ::std::mem::transmute::<*mut libc::c_void, Option<GenBuffersproc>>(
            SDL_GL_GetProcAddress(b"glGenBuffers\x00" as *const u8 as *const libc::c_char),
        );
        if qglGenBuffers.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glGenBuffers\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglBufferData = ::std::mem::transmute::<*mut libc::c_void, Option<BufferDataproc>>(
            SDL_GL_GetProcAddress(b"glBufferData\x00" as *const u8 as *const libc::c_char),
        );
        if qglBufferData.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glBufferData\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglBufferSubData = ::std::mem::transmute::<*mut libc::c_void, Option<BufferSubDataproc>>(
            SDL_GL_GetProcAddress(b"glBufferSubData\x00" as *const u8 as *const libc::c_char),
        );
        if qglBufferSubData.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glBufferSubData\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglAttachShader = ::std::mem::transmute::<*mut libc::c_void, Option<AttachShaderproc>>(
            SDL_GL_GetProcAddress(b"glAttachShader\x00" as *const u8 as *const libc::c_char),
        );
        if qglAttachShader.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glAttachShader\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglBindAttribLocation = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<BindAttribLocationproc>,
        >(SDL_GL_GetProcAddress(
            b"glBindAttribLocation\x00" as *const u8 as *const libc::c_char,
        ));
        if qglBindAttribLocation.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glBindAttribLocation\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglCompileShader = ::std::mem::transmute::<*mut libc::c_void, Option<CompileShaderproc>>(
            SDL_GL_GetProcAddress(b"glCompileShader\x00" as *const u8 as *const libc::c_char),
        );
        if qglCompileShader.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glCompileShader\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglCreateProgram = ::std::mem::transmute::<*mut libc::c_void, Option<CreateProgramproc>>(
            SDL_GL_GetProcAddress(b"glCreateProgram\x00" as *const u8 as *const libc::c_char),
        );
        if qglCreateProgram.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glCreateProgram\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglCreateShader = ::std::mem::transmute::<*mut libc::c_void, Option<CreateShaderproc>>(
            SDL_GL_GetProcAddress(b"glCreateShader\x00" as *const u8 as *const libc::c_char),
        );
        if qglCreateShader.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glCreateShader\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglDeleteProgram = ::std::mem::transmute::<*mut libc::c_void, Option<DeleteProgramproc>>(
            SDL_GL_GetProcAddress(b"glDeleteProgram\x00" as *const u8 as *const libc::c_char),
        );
        if qglDeleteProgram.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glDeleteProgram\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglDeleteShader = ::std::mem::transmute::<*mut libc::c_void, Option<DeleteShaderproc>>(
            SDL_GL_GetProcAddress(b"glDeleteShader\x00" as *const u8 as *const libc::c_char),
        );
        if qglDeleteShader.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glDeleteShader\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglDetachShader = ::std::mem::transmute::<*mut libc::c_void, Option<DetachShaderproc>>(
            SDL_GL_GetProcAddress(b"glDetachShader\x00" as *const u8 as *const libc::c_char),
        );
        if qglDetachShader.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glDetachShader\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglDisableVertexAttribArray = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<DisableVertexAttribArrayproc>,
        >(SDL_GL_GetProcAddress(
            b"glDisableVertexAttribArray\x00" as *const u8 as *const libc::c_char,
        ));
        if qglDisableVertexAttribArray.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glDisableVertexAttribArray\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglEnableVertexAttribArray = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<EnableVertexAttribArrayproc>,
        >(SDL_GL_GetProcAddress(
            b"glEnableVertexAttribArray\x00" as *const u8 as *const libc::c_char,
        ));
        if qglEnableVertexAttribArray.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glEnableVertexAttribArray\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglGetActiveUniform = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<GetActiveUniformproc>,
        >(SDL_GL_GetProcAddress(
            b"glGetActiveUniform\x00" as *const u8 as *const libc::c_char,
        ));
        if qglGetActiveUniform.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glGetActiveUniform\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglGetProgramiv = ::std::mem::transmute::<*mut libc::c_void, Option<GetProgramivproc>>(
            SDL_GL_GetProcAddress(b"glGetProgramiv\x00" as *const u8 as *const libc::c_char),
        );
        if qglGetProgramiv.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glGetProgramiv\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglGetProgramInfoLog = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<GetProgramInfoLogproc>,
        >(SDL_GL_GetProcAddress(
            b"glGetProgramInfoLog\x00" as *const u8 as *const libc::c_char,
        ));
        if qglGetProgramInfoLog.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glGetProgramInfoLog\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglGetShaderiv = ::std::mem::transmute::<*mut libc::c_void, Option<GetShaderivproc>>(
            SDL_GL_GetProcAddress(b"glGetShaderiv\x00" as *const u8 as *const libc::c_char),
        );
        if qglGetShaderiv.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glGetShaderiv\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglGetShaderInfoLog = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<GetShaderInfoLogproc>,
        >(SDL_GL_GetProcAddress(
            b"glGetShaderInfoLog\x00" as *const u8 as *const libc::c_char,
        ));
        if qglGetShaderInfoLog.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glGetShaderInfoLog\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglGetShaderSource = ::std::mem::transmute::<*mut libc::c_void, Option<GetShaderSourceproc>>(
            SDL_GL_GetProcAddress(b"glGetShaderSource\x00" as *const u8 as *const libc::c_char),
        );
        if qglGetShaderSource.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glGetShaderSource\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglGetUniformLocation = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<GetUniformLocationproc>,
        >(SDL_GL_GetProcAddress(
            b"glGetUniformLocation\x00" as *const u8 as *const libc::c_char,
        ));
        if qglGetUniformLocation.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glGetUniformLocation\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglLinkProgram = ::std::mem::transmute::<*mut libc::c_void, Option<LinkProgramproc>>(
            SDL_GL_GetProcAddress(b"glLinkProgram\x00" as *const u8 as *const libc::c_char),
        );
        if qglLinkProgram.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glLinkProgram\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglShaderSource = ::std::mem::transmute::<*mut libc::c_void, Option<ShaderSourceproc>>(
            SDL_GL_GetProcAddress(b"glShaderSource\x00" as *const u8 as *const libc::c_char),
        );
        if qglShaderSource.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glShaderSource\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglUseProgram = ::std::mem::transmute::<*mut libc::c_void, Option<UseProgramproc>>(
            SDL_GL_GetProcAddress(b"glUseProgram\x00" as *const u8 as *const libc::c_char),
        );
        if qglUseProgram.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glUseProgram\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglUniform1f = ::std::mem::transmute::<*mut libc::c_void, Option<Uniform1fproc>>(
            SDL_GL_GetProcAddress(b"glUniform1f\x00" as *const u8 as *const libc::c_char),
        );
        if qglUniform1f.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glUniform1f\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglUniform2f = ::std::mem::transmute::<*mut libc::c_void, Option<Uniform2fproc>>(
            SDL_GL_GetProcAddress(b"glUniform2f\x00" as *const u8 as *const libc::c_char),
        );
        if qglUniform2f.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glUniform2f\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglUniform3f = ::std::mem::transmute::<*mut libc::c_void, Option<Uniform3fproc>>(
            SDL_GL_GetProcAddress(b"glUniform3f\x00" as *const u8 as *const libc::c_char),
        );
        if qglUniform3f.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glUniform3f\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglUniform4f = ::std::mem::transmute::<*mut libc::c_void, Option<Uniform4fproc>>(
            SDL_GL_GetProcAddress(b"glUniform4f\x00" as *const u8 as *const libc::c_char),
        );
        if qglUniform4f.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glUniform4f\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglUniform1i = ::std::mem::transmute::<*mut libc::c_void, Option<Uniform1iproc>>(
            SDL_GL_GetProcAddress(b"glUniform1i\x00" as *const u8 as *const libc::c_char),
        );
        if qglUniform1i.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glUniform1i\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglUniform1fv = ::std::mem::transmute::<*mut libc::c_void, Option<Uniform1fvproc>>(
            SDL_GL_GetProcAddress(b"glUniform1fv\x00" as *const u8 as *const libc::c_char),
        );
        if qglUniform1fv.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glUniform1fv\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglUniformMatrix4fv = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<UniformMatrix4fvproc>,
        >(SDL_GL_GetProcAddress(
            b"glUniformMatrix4fv\x00" as *const u8 as *const libc::c_char,
        ));
        if qglUniformMatrix4fv.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glUniformMatrix4fv\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglValidateProgram = ::std::mem::transmute::<*mut libc::c_void, Option<ValidateProgramproc>>(
            SDL_GL_GetProcAddress(b"glValidateProgram\x00" as *const u8 as *const libc::c_char),
        );
        if qglValidateProgram.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glValidateProgram\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglVertexAttribPointer = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<VertexAttribPointerproc>,
        >(SDL_GL_GetProcAddress(
            b"glVertexAttribPointer\x00" as *const u8 as *const libc::c_char,
        ));
        if qglVertexAttribPointer.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glVertexAttribPointer\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
    } else if qglesMajorVersion > 2 as i32
        || qglesMajorVersion == 2 as i32 && qglesMinorVersion >= 0 as i32
    {
        qglBindTexture = ::std::mem::transmute::<*mut libc::c_void, Option<BindTextureproc>>(
            SDL_GL_GetProcAddress(b"glBindTexture\x00" as *const u8 as *const libc::c_char),
        );
        if qglBindTexture.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glBindTexture\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglBlendFunc = ::std::mem::transmute::<*mut libc::c_void, Option<BlendFuncproc>>(
            SDL_GL_GetProcAddress(b"glBlendFunc\x00" as *const u8 as *const libc::c_char),
        );
        if qglBlendFunc.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glBlendFunc\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglClearColor = ::std::mem::transmute::<*mut libc::c_void, Option<ClearColorproc>>(
            SDL_GL_GetProcAddress(b"glClearColor\x00" as *const u8 as *const libc::c_char),
        );
        if qglClearColor.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glClearColor\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglClear = ::std::mem::transmute::<*mut libc::c_void, Option<Clearproc>>(
            SDL_GL_GetProcAddress(b"glClear\x00" as *const u8 as *const libc::c_char),
        );
        if qglClear.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glClear\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglClearStencil = ::std::mem::transmute::<*mut libc::c_void, Option<ClearStencilproc>>(
            SDL_GL_GetProcAddress(b"glClearStencil\x00" as *const u8 as *const libc::c_char),
        );
        if qglClearStencil.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glClearStencil\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglColorMask = ::std::mem::transmute::<*mut libc::c_void, Option<ColorMaskproc>>(
            SDL_GL_GetProcAddress(b"glColorMask\x00" as *const u8 as *const libc::c_char),
        );
        if qglColorMask.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glColorMask\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglCopyTexSubImage2D = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<CopyTexSubImage2Dproc>,
        >(SDL_GL_GetProcAddress(
            b"glCopyTexSubImage2D\x00" as *const u8 as *const libc::c_char,
        ));
        if qglCopyTexSubImage2D.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glCopyTexSubImage2D\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglCullFace = ::std::mem::transmute::<*mut libc::c_void, Option<CullFaceproc>>(
            SDL_GL_GetProcAddress(b"glCullFace\x00" as *const u8 as *const libc::c_char),
        );
        if qglCullFace.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glCullFace\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglDeleteTextures = ::std::mem::transmute::<*mut libc::c_void, Option<DeleteTexturesproc>>(
            SDL_GL_GetProcAddress(b"glDeleteTextures\x00" as *const u8 as *const libc::c_char),
        );
        if qglDeleteTextures.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glDeleteTextures\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglDepthFunc = ::std::mem::transmute::<*mut libc::c_void, Option<DepthFuncproc>>(
            SDL_GL_GetProcAddress(b"glDepthFunc\x00" as *const u8 as *const libc::c_char),
        );
        if qglDepthFunc.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glDepthFunc\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglDepthMask = ::std::mem::transmute::<*mut libc::c_void, Option<DepthMaskproc>>(
            SDL_GL_GetProcAddress(b"glDepthMask\x00" as *const u8 as *const libc::c_char),
        );
        if qglDepthMask.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glDepthMask\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglDisable = ::std::mem::transmute::<*mut libc::c_void, Option<Disableproc>>(
            SDL_GL_GetProcAddress(b"glDisable\x00" as *const u8 as *const libc::c_char),
        );
        if qglDisable.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glDisable\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglDrawArrays = ::std::mem::transmute::<*mut libc::c_void, Option<DrawArraysproc>>(
            SDL_GL_GetProcAddress(b"glDrawArrays\x00" as *const u8 as *const libc::c_char),
        );
        if qglDrawArrays.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glDrawArrays\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglDrawElements = ::std::mem::transmute::<*mut libc::c_void, Option<DrawElementsproc>>(
            SDL_GL_GetProcAddress(b"glDrawElements\x00" as *const u8 as *const libc::c_char),
        );
        if qglDrawElements.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glDrawElements\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglEnable = ::std::mem::transmute::<*mut libc::c_void, Option<Enableproc>>(
            SDL_GL_GetProcAddress(b"glEnable\x00" as *const u8 as *const libc::c_char),
        );
        if qglEnable.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glEnable\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglFinish = ::std::mem::transmute::<*mut libc::c_void, Option<Finishproc>>(
            SDL_GL_GetProcAddress(b"glFinish\x00" as *const u8 as *const libc::c_char),
        );
        if qglFinish.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glFinish\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglFlush = ::std::mem::transmute::<*mut libc::c_void, Option<Flushproc>>(
            SDL_GL_GetProcAddress(b"glFlush\x00" as *const u8 as *const libc::c_char),
        );
        if qglFlush.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glFlush\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglGenTextures = ::std::mem::transmute::<*mut libc::c_void, Option<GenTexturesproc>>(
            SDL_GL_GetProcAddress(b"glGenTextures\x00" as *const u8 as *const libc::c_char),
        );
        if qglGenTextures.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glGenTextures\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglGetBooleanv = ::std::mem::transmute::<*mut libc::c_void, Option<GetBooleanvproc>>(
            SDL_GL_GetProcAddress(b"glGetBooleanv\x00" as *const u8 as *const libc::c_char),
        );
        if qglGetBooleanv.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glGetBooleanv\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglGetError = ::std::mem::transmute::<*mut libc::c_void, Option<GetErrorproc>>(
            SDL_GL_GetProcAddress(b"glGetError\x00" as *const u8 as *const libc::c_char),
        );
        if qglGetError.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glGetError\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglGetIntegerv = ::std::mem::transmute::<*mut libc::c_void, Option<GetIntegervproc>>(
            SDL_GL_GetProcAddress(b"glGetIntegerv\x00" as *const u8 as *const libc::c_char),
        );
        if qglGetIntegerv.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glGetIntegerv\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglGetString = ::std::mem::transmute::<*mut libc::c_void, Option<GetStringproc>>(
            SDL_GL_GetProcAddress(b"glGetString\x00" as *const u8 as *const libc::c_char),
        );
        if qglGetString.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glGetString\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglLineWidth = ::std::mem::transmute::<*mut libc::c_void, Option<LineWidthproc>>(
            SDL_GL_GetProcAddress(b"glLineWidth\x00" as *const u8 as *const libc::c_char),
        );
        if qglLineWidth.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glLineWidth\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglPolygonOffset = ::std::mem::transmute::<*mut libc::c_void, Option<PolygonOffsetproc>>(
            SDL_GL_GetProcAddress(b"glPolygonOffset\x00" as *const u8 as *const libc::c_char),
        );
        if qglPolygonOffset.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glPolygonOffset\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglReadPixels = ::std::mem::transmute::<*mut libc::c_void, Option<ReadPixelsproc>>(
            SDL_GL_GetProcAddress(b"glReadPixels\x00" as *const u8 as *const libc::c_char),
        );
        if qglReadPixels.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glReadPixels\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglScissor = ::std::mem::transmute::<*mut libc::c_void, Option<Scissorproc>>(
            SDL_GL_GetProcAddress(b"glScissor\x00" as *const u8 as *const libc::c_char),
        );
        if qglScissor.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glScissor\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglStencilFunc = ::std::mem::transmute::<*mut libc::c_void, Option<StencilFuncproc>>(
            SDL_GL_GetProcAddress(b"glStencilFunc\x00" as *const u8 as *const libc::c_char),
        );
        if qglStencilFunc.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glStencilFunc\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglStencilMask = ::std::mem::transmute::<*mut libc::c_void, Option<StencilMaskproc>>(
            SDL_GL_GetProcAddress(b"glStencilMask\x00" as *const u8 as *const libc::c_char),
        );
        if qglStencilMask.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glStencilMask\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglStencilOp = ::std::mem::transmute::<*mut libc::c_void, Option<StencilOpproc>>(
            SDL_GL_GetProcAddress(b"glStencilOp\x00" as *const u8 as *const libc::c_char),
        );
        if qglStencilOp.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glStencilOp\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglTexImage2D = ::std::mem::transmute::<*mut libc::c_void, Option<TexImage2Dproc>>(
            SDL_GL_GetProcAddress(b"glTexImage2D\x00" as *const u8 as *const libc::c_char),
        );
        if qglTexImage2D.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glTexImage2D\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglTexParameterf = ::std::mem::transmute::<*mut libc::c_void, Option<TexParameterfproc>>(
            SDL_GL_GetProcAddress(b"glTexParameterf\x00" as *const u8 as *const libc::c_char),
        );
        if qglTexParameterf.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glTexParameterf\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglTexParameteri = ::std::mem::transmute::<*mut libc::c_void, Option<TexParameteriproc>>(
            SDL_GL_GetProcAddress(b"glTexParameteri\x00" as *const u8 as *const libc::c_char),
        );
        if qglTexParameteri.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glTexParameteri\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglTexSubImage2D = ::std::mem::transmute::<*mut libc::c_void, Option<TexSubImage2Dproc>>(
            SDL_GL_GetProcAddress(b"glTexSubImage2D\x00" as *const u8 as *const libc::c_char),
        );
        if qglTexSubImage2D.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glTexSubImage2D\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglTranslatef = ::std::mem::transmute::<*mut libc::c_void, Option<Translatefproc>>(
            SDL_GL_GetProcAddress(b"glTranslatef\x00" as *const u8 as *const libc::c_char),
        );
        if qglTranslatef.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glTranslatef\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglViewport = ::std::mem::transmute::<*mut libc::c_void, Option<Viewportproc>>(
            SDL_GL_GetProcAddress(b"glViewport\x00" as *const u8 as *const libc::c_char),
        );
        if qglViewport.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glViewport\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglClearDepthf = ::std::mem::transmute::<*mut libc::c_void, Option<ClearDepthfproc>>(
            SDL_GL_GetProcAddress(b"glClearDepthf\x00" as *const u8 as *const libc::c_char),
        );
        if qglClearDepthf.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glClearDepthf\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglDepthRangef = ::std::mem::transmute::<*mut libc::c_void, Option<DepthRangefproc>>(
            SDL_GL_GetProcAddress(b"glDepthRangef\x00" as *const u8 as *const libc::c_char),
        );
        if qglDepthRangef.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glDepthRangef\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglActiveTexture = ::std::mem::transmute::<*mut libc::c_void, Option<ActiveTextureproc>>(
            SDL_GL_GetProcAddress(b"glActiveTexture\x00" as *const u8 as *const libc::c_char),
        );
        if qglActiveTexture.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glActiveTexture\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglCompressedTexImage2D = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<CompressedTexImage2Dproc>,
        >(SDL_GL_GetProcAddress(
            b"glCompressedTexImage2D\x00" as *const u8 as *const libc::c_char,
        ));
        if qglCompressedTexImage2D.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glCompressedTexImage2D\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglCompressedTexSubImage2D = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<CompressedTexSubImage2Dproc>,
        >(SDL_GL_GetProcAddress(
            b"glCompressedTexSubImage2D\x00" as *const u8 as *const libc::c_char,
        ));
        if qglCompressedTexSubImage2D.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glCompressedTexSubImage2D\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglBindBuffer = ::std::mem::transmute::<*mut libc::c_void, Option<BindBufferproc>>(
            SDL_GL_GetProcAddress(b"glBindBuffer\x00" as *const u8 as *const libc::c_char),
        );
        if qglBindBuffer.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glBindBuffer\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglDeleteBuffers = ::std::mem::transmute::<*mut libc::c_void, Option<DeleteBuffersproc>>(
            SDL_GL_GetProcAddress(b"glDeleteBuffers\x00" as *const u8 as *const libc::c_char),
        );
        if qglDeleteBuffers.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glDeleteBuffers\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglGenBuffers = ::std::mem::transmute::<*mut libc::c_void, Option<GenBuffersproc>>(
            SDL_GL_GetProcAddress(b"glGenBuffers\x00" as *const u8 as *const libc::c_char),
        );
        if qglGenBuffers.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glGenBuffers\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglBufferData = ::std::mem::transmute::<*mut libc::c_void, Option<BufferDataproc>>(
            SDL_GL_GetProcAddress(b"glBufferData\x00" as *const u8 as *const libc::c_char),
        );
        if qglBufferData.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glBufferData\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglBufferSubData = ::std::mem::transmute::<*mut libc::c_void, Option<BufferSubDataproc>>(
            SDL_GL_GetProcAddress(b"glBufferSubData\x00" as *const u8 as *const libc::c_char),
        );
        if qglBufferSubData.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glBufferSubData\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglAttachShader = ::std::mem::transmute::<*mut libc::c_void, Option<AttachShaderproc>>(
            SDL_GL_GetProcAddress(b"glAttachShader\x00" as *const u8 as *const libc::c_char),
        );
        if qglAttachShader.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glAttachShader\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglBindAttribLocation = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<BindAttribLocationproc>,
        >(SDL_GL_GetProcAddress(
            b"glBindAttribLocation\x00" as *const u8 as *const libc::c_char,
        ));
        if qglBindAttribLocation.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glBindAttribLocation\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglCompileShader = ::std::mem::transmute::<*mut libc::c_void, Option<CompileShaderproc>>(
            SDL_GL_GetProcAddress(b"glCompileShader\x00" as *const u8 as *const libc::c_char),
        );
        if qglCompileShader.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glCompileShader\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglCreateProgram = ::std::mem::transmute::<*mut libc::c_void, Option<CreateProgramproc>>(
            SDL_GL_GetProcAddress(b"glCreateProgram\x00" as *const u8 as *const libc::c_char),
        );
        if qglCreateProgram.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glCreateProgram\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglCreateShader = ::std::mem::transmute::<*mut libc::c_void, Option<CreateShaderproc>>(
            SDL_GL_GetProcAddress(b"glCreateShader\x00" as *const u8 as *const libc::c_char),
        );
        if qglCreateShader.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glCreateShader\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglDeleteProgram = ::std::mem::transmute::<*mut libc::c_void, Option<DeleteProgramproc>>(
            SDL_GL_GetProcAddress(b"glDeleteProgram\x00" as *const u8 as *const libc::c_char),
        );
        if qglDeleteProgram.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glDeleteProgram\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglDeleteShader = ::std::mem::transmute::<*mut libc::c_void, Option<DeleteShaderproc>>(
            SDL_GL_GetProcAddress(b"glDeleteShader\x00" as *const u8 as *const libc::c_char),
        );
        if qglDeleteShader.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glDeleteShader\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglDetachShader = ::std::mem::transmute::<*mut libc::c_void, Option<DetachShaderproc>>(
            SDL_GL_GetProcAddress(b"glDetachShader\x00" as *const u8 as *const libc::c_char),
        );
        if qglDetachShader.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glDetachShader\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglDisableVertexAttribArray = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<DisableVertexAttribArrayproc>,
        >(SDL_GL_GetProcAddress(
            b"glDisableVertexAttribArray\x00" as *const u8 as *const libc::c_char,
        ));
        if qglDisableVertexAttribArray.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glDisableVertexAttribArray\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglEnableVertexAttribArray = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<EnableVertexAttribArrayproc>,
        >(SDL_GL_GetProcAddress(
            b"glEnableVertexAttribArray\x00" as *const u8 as *const libc::c_char,
        ));
        if qglEnableVertexAttribArray.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glEnableVertexAttribArray\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglGetActiveUniform = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<GetActiveUniformproc>,
        >(SDL_GL_GetProcAddress(
            b"glGetActiveUniform\x00" as *const u8 as *const libc::c_char,
        ));
        if qglGetActiveUniform.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glGetActiveUniform\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglGetProgramiv = ::std::mem::transmute::<*mut libc::c_void, Option<GetProgramivproc>>(
            SDL_GL_GetProcAddress(b"glGetProgramiv\x00" as *const u8 as *const libc::c_char),
        );
        if qglGetProgramiv.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glGetProgramiv\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglGetProgramInfoLog = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<GetProgramInfoLogproc>,
        >(SDL_GL_GetProcAddress(
            b"glGetProgramInfoLog\x00" as *const u8 as *const libc::c_char,
        ));
        if qglGetProgramInfoLog.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glGetProgramInfoLog\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglGetShaderiv = ::std::mem::transmute::<*mut libc::c_void, Option<GetShaderivproc>>(
            SDL_GL_GetProcAddress(b"glGetShaderiv\x00" as *const u8 as *const libc::c_char),
        );
        if qglGetShaderiv.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glGetShaderiv\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglGetShaderInfoLog = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<GetShaderInfoLogproc>,
        >(SDL_GL_GetProcAddress(
            b"glGetShaderInfoLog\x00" as *const u8 as *const libc::c_char,
        ));
        if qglGetShaderInfoLog.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glGetShaderInfoLog\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglGetShaderSource = ::std::mem::transmute::<*mut libc::c_void, Option<GetShaderSourceproc>>(
            SDL_GL_GetProcAddress(b"glGetShaderSource\x00" as *const u8 as *const libc::c_char),
        );
        if qglGetShaderSource.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glGetShaderSource\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglGetUniformLocation = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<GetUniformLocationproc>,
        >(SDL_GL_GetProcAddress(
            b"glGetUniformLocation\x00" as *const u8 as *const libc::c_char,
        ));
        if qglGetUniformLocation.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glGetUniformLocation\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglLinkProgram = ::std::mem::transmute::<*mut libc::c_void, Option<LinkProgramproc>>(
            SDL_GL_GetProcAddress(b"glLinkProgram\x00" as *const u8 as *const libc::c_char),
        );
        if qglLinkProgram.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glLinkProgram\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglShaderSource = ::std::mem::transmute::<*mut libc::c_void, Option<ShaderSourceproc>>(
            SDL_GL_GetProcAddress(b"glShaderSource\x00" as *const u8 as *const libc::c_char),
        );
        if qglShaderSource.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glShaderSource\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglUseProgram = ::std::mem::transmute::<*mut libc::c_void, Option<UseProgramproc>>(
            SDL_GL_GetProcAddress(b"glUseProgram\x00" as *const u8 as *const libc::c_char),
        );
        if qglUseProgram.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glUseProgram\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglUniform1f = ::std::mem::transmute::<*mut libc::c_void, Option<Uniform1fproc>>(
            SDL_GL_GetProcAddress(b"glUniform1f\x00" as *const u8 as *const libc::c_char),
        );
        if qglUniform1f.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glUniform1f\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglUniform2f = ::std::mem::transmute::<*mut libc::c_void, Option<Uniform2fproc>>(
            SDL_GL_GetProcAddress(b"glUniform2f\x00" as *const u8 as *const libc::c_char),
        );
        if qglUniform2f.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glUniform2f\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglUniform3f = ::std::mem::transmute::<*mut libc::c_void, Option<Uniform3fproc>>(
            SDL_GL_GetProcAddress(b"glUniform3f\x00" as *const u8 as *const libc::c_char),
        );
        if qglUniform3f.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glUniform3f\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglUniform4f = ::std::mem::transmute::<*mut libc::c_void, Option<Uniform4fproc>>(
            SDL_GL_GetProcAddress(b"glUniform4f\x00" as *const u8 as *const libc::c_char),
        );
        if qglUniform4f.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glUniform4f\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglUniform1i = ::std::mem::transmute::<*mut libc::c_void, Option<Uniform1iproc>>(
            SDL_GL_GetProcAddress(b"glUniform1i\x00" as *const u8 as *const libc::c_char),
        );
        if qglUniform1i.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glUniform1i\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglUniform1fv = ::std::mem::transmute::<*mut libc::c_void, Option<Uniform1fvproc>>(
            SDL_GL_GetProcAddress(b"glUniform1fv\x00" as *const u8 as *const libc::c_char),
        );
        if qglUniform1fv.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glUniform1fv\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglUniformMatrix4fv = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<UniformMatrix4fvproc>,
        >(SDL_GL_GetProcAddress(
            b"glUniformMatrix4fv\x00" as *const u8 as *const libc::c_char,
        ));
        if qglUniformMatrix4fv.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glUniformMatrix4fv\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglValidateProgram = ::std::mem::transmute::<*mut libc::c_void, Option<ValidateProgramproc>>(
            SDL_GL_GetProcAddress(b"glValidateProgram\x00" as *const u8 as *const libc::c_char),
        );
        if qglValidateProgram.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glValidateProgram\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        qglVertexAttribPointer = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<VertexAttribPointerproc>,
        >(SDL_GL_GetProcAddress(
            b"glVertexAttribPointer\x00" as *const u8 as *const libc::c_char,
        ));
        if qglVertexAttribPointer.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glVertexAttribPointer\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
        // error so this doesn't segfault due to NULL desktop GL functions being used
        Com_Error(
            ERR_FATAL as i32,
            b"Unsupported OpenGL Version: %s\n\x00" as *const u8 as *const libc::c_char,
            version,
        );
    } else {
        Com_Error(
            ERR_FATAL as i32,
            b"Unsupported OpenGL Version (%s), OpenGL 2.0 is required\n\x00" as *const u8
                as *const libc::c_char,
            version,
        );
    }
    if qglMajorVersion > 3 as i32
        || qglMajorVersion == 3 as i32 && qglMinorVersion >= 0 as i32
        || (qglesMajorVersion > 3 as i32
            || qglesMajorVersion == 3 as i32 && qglesMinorVersion >= 0 as i32)
    {
        qglGetStringi = ::std::mem::transmute::<*mut libc::c_void, Option<GetStringiproc>>(
            SDL_GL_GetProcAddress(b"glGetStringi\x00" as *const u8 as *const libc::c_char),
        );
        if qglGetStringi.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const libc::c_char,
                b"glGetStringi\x00" as *const u8 as *const libc::c_char,
            );
            success = qfalse
        }
    }
    return success;
}
/*
===============
GLimp_ClearProcAddresses

Clear addresses for OpenGL functions.
===============
*/

unsafe extern "C" fn GLimp_ClearProcAddresses() {
    qglMajorVersion = 0 as i32;
    qglMinorVersion = 0 as i32;
    qglesMajorVersion = 0 as i32;
    qglesMinorVersion = 0 as i32;
    qglBindTexture = None;
    qglBlendFunc = None;
    qglClearColor = None;
    qglClear = None;
    qglClearStencil = None;
    qglColorMask = None;
    qglCopyTexSubImage2D = None;
    qglCullFace = None;
    qglDeleteTextures = None;
    qglDepthFunc = None;
    qglDepthMask = None;
    qglDisable = None;
    qglDrawArrays = None;
    qglDrawElements = None;
    qglEnable = None;
    qglFinish = None;
    qglFlush = None;
    qglGenTextures = None;
    qglGetBooleanv = None;
    qglGetError = None;
    qglGetIntegerv = None;
    qglGetString = None;
    qglLineWidth = None;
    qglPolygonOffset = None;
    qglReadPixels = None;
    qglScissor = None;
    qglStencilFunc = None;
    qglStencilMask = None;
    qglStencilOp = None;
    qglTexImage2D = None;
    qglTexParameterf = None;
    qglTexParameteri = None;
    qglTexSubImage2D = None;
    qglTranslatef = None;
    qglViewport = None;
    qglAlphaFunc = None;
    qglColor4f = None;
    qglColorPointer = None;
    qglDisableClientState = None;
    qglEnableClientState = None;
    qglLoadIdentity = None;
    qglLoadMatrixf = None;
    qglMatrixMode = None;
    qglPopMatrix = None;
    qglPushMatrix = None;
    qglShadeModel = None;
    qglTexCoordPointer = None;
    qglTexEnvf = None;
    qglVertexPointer = None;
    qglClearDepth = None;
    qglDepthRange = None;
    qglDrawBuffer = None;
    qglPolygonMode = None;
    qglArrayElement = None;
    qglBegin = None;
    qglClipPlane = None;
    qglColor3f = None;
    qglColor4ubv = None;
    qglEnd = None;
    qglFrustum = None;
    qglOrtho = None;
    qglTexCoord2f = None;
    qglTexCoord2fv = None;
    qglVertex2f = None;
    qglVertex3f = None;
    qglVertex3fv = None;
    qglClearDepthf = None;
    qglDepthRangef = None;
    qglClipPlanef = None;
    qglFrustumf = None;
    qglOrthof = None;
    qglActiveTexture = None;
    qglCompressedTexImage2D = None;
    qglCompressedTexSubImage2D = None;
    qglBindBuffer = None;
    qglDeleteBuffers = None;
    qglGenBuffers = None;
    qglBufferData = None;
    qglBufferSubData = None;
    qglAttachShader = None;
    qglBindAttribLocation = None;
    qglCompileShader = None;
    qglCreateProgram = None;
    qglCreateShader = None;
    qglDeleteProgram = None;
    qglDeleteShader = None;
    qglDetachShader = None;
    qglDisableVertexAttribArray = None;
    qglEnableVertexAttribArray = None;
    qglGetActiveUniform = None;
    qglGetProgramiv = None;
    qglGetProgramInfoLog = None;
    qglGetShaderiv = None;
    qglGetShaderInfoLog = None;
    qglGetShaderSource = None;
    qglGetUniformLocation = None;
    qglLinkProgram = None;
    qglShaderSource = None;
    qglUseProgram = None;
    qglUniform1f = None;
    qglUniform2f = None;
    qglUniform3f = None;
    qglUniform4f = None;
    qglUniform1i = None;
    qglUniform1fv = None;
    qglUniformMatrix4fv = None;
    qglValidateProgram = None;
    qglVertexAttribPointer = None;
    qglGetStringi = None;
    qglGenQueries = None;
    qglDeleteQueries = None;
    qglBeginQuery = None;
    qglEndQuery = None;
    qglGetQueryObjectiv = None;
    qglGetQueryObjectuiv = None;
    qglBindRenderbuffer = None;
    qglDeleteRenderbuffers = None;
    qglGenRenderbuffers = None;
    qglRenderbufferStorage = None;
    qglBindFramebuffer = None;
    qglDeleteFramebuffers = None;
    qglGenFramebuffers = None;
    qglCheckFramebufferStatus = None;
    qglFramebufferTexture2D = None;
    qglFramebufferRenderbuffer = None;
    qglGenerateMipmap = None;
    qglBlitFramebuffer = None;
    qglRenderbufferStorageMultisample = None;
    qglBindVertexArray = None;
    qglDeleteVertexArrays = None;
    qglGenVertexArrays = None;
    qglBindMultiTextureEXT = None;
    qglTextureParameterfEXT = None;
    qglTextureParameteriEXT = None;
    qglTextureImage2DEXT = None;
    qglTextureSubImage2DEXT = None;
    qglCopyTextureSubImage2DEXT = None;
    qglCompressedTextureImage2DEXT = None;
    qglCompressedTextureSubImage2DEXT = None;
    qglGenerateTextureMipmapEXT = None;
    qglProgramUniform1iEXT = None;
    qglProgramUniform1fEXT = None;
    qglProgramUniform2fEXT = None;
    qglProgramUniform3fEXT = None;
    qglProgramUniform4fEXT = None;
    qglProgramUniform1fvEXT = None;
    qglProgramUniformMatrix4fvEXT = None;
    qglNamedRenderbufferStorageEXT = None;
    qglNamedRenderbufferStorageMultisampleEXT = None;
    qglCheckNamedFramebufferStatusEXT = None;
    qglNamedFramebufferTexture2DEXT = None;
    qglNamedFramebufferRenderbufferEXT = None;
    qglActiveTextureARB = None;
    qglClientActiveTextureARB = None;
    qglMultiTexCoord2fARB = None;
    qglLockArraysEXT = None;
    qglUnlockArraysEXT = None;
}
/*
===============
GLimp_SetMode
===============
*/

unsafe extern "C" fn GLimp_SetMode(
    mut mode: i32,
    mut fullscreen: qboolean,
    mut noborder: qboolean,
    mut fixedFunction: qboolean,
) -> i32 {
    let mut glstring: *const libc::c_char = 0 as *const libc::c_char;
    let mut perChannelColorBits: i32 = 0;
    let mut colorBits: i32 = 0;
    let mut depthBits: i32 = 0;
    let mut stencilBits: i32 = 0;
    let mut samples: i32 = 0;
    let mut i: i32 = 0 as i32;
    let mut icon: *mut SDL_Surface = std::ptr::null_mut();
    let mut flags: Uint32 = (SDL_WINDOW_SHOWN as i32 | SDL_WINDOW_OPENGL as i32) as Uint32;
    let mut desktopMode: SDL_DisplayMode = SDL_DisplayMode {
        format: 0,
        w: 0,
        h: 0,
        refresh_rate: 0,
        driverdata: std::ptr::null_mut(),
    };
    let mut display: i32 = 0 as i32;
    let mut x: i32 = (0x1fff0000 as u32 | 0 as i32 as u32) as i32;
    let mut y: i32 = (0x1fff0000 as u32 | 0 as i32 as u32) as i32;
    crate::src::renderergl1::tr_main::ri
        .Printf
        .expect("non-null function pointer")(
        PRINT_ALL as i32,
        b"Initializing OpenGL display\n\x00" as *const u8 as *const libc::c_char,
    );
    if (*r_allowResize).integer != 0 {
        flags |= SDL_WINDOW_RESIZABLE as i32 as u32
    }
    icon = SDL_CreateRGBSurfaceFrom(
        CLIENT_WINDOW_ICON.pixel_data.as_ptr() as *mut libc::c_void,
        CLIENT_WINDOW_ICON.width as i32,
        CLIENT_WINDOW_ICON.height as i32,
        CLIENT_WINDOW_ICON
            .bytes_per_pixel
            .wrapping_mul(8 as i32 as u32) as i32,
        CLIENT_WINDOW_ICON
            .bytes_per_pixel
            .wrapping_mul(CLIENT_WINDOW_ICON.width) as i32,
        0xff as i32 as Uint32,
        0xff00 as i32 as Uint32,
        0xff0000 as i32 as Uint32,
        0xff000000 as u32,
    );
    // If a window exists, note its display index
    if !SDL_window.is_null() {
        display = SDL_GetWindowDisplayIndex(SDL_window);
        if display < 0 as i32 {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_DEVELOPER as i32,
                b"SDL_GetWindowDisplayIndex() failed: %s\n\x00" as *const u8 as *const libc::c_char,
                crate::stdlib::SDL_GetError(),
            );
        }
    }
    if display >= 0 as i32 && SDL_GetDesktopDisplayMode(display, &mut desktopMode) == 0 as i32 {
        crate::src::renderergl1::tr_init::displayAspect =
            desktopMode.w as f32 / desktopMode.h as f32;
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            PRINT_ALL as i32,
            b"Display aspect: %.3f\n\x00" as *const u8 as *const libc::c_char,
            crate::src::renderergl1::tr_init::displayAspect as f64,
        );
    } else {
        crate::stdlib::memset(
            &mut desktopMode as *mut SDL_DisplayMode as *mut libc::c_void,
            0 as i32,
            ::std::mem::size_of::<SDL_DisplayMode>() as usize,
        );
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            PRINT_ALL as i32,
            b"Cannot determine display aspect, assuming 1.333\n\x00" as *const u8
                as *const libc::c_char,
        );
    }
    crate::src::renderergl1::tr_main::ri
        .Printf
        .expect("non-null function pointer")(
        PRINT_ALL as i32,
        b"...setting mode %d:\x00" as *const u8 as *const libc::c_char,
        mode,
    );
    if mode == -(2 as i32) {
        // use desktop video resolution
        if desktopMode.h > 0 as i32 {
            crate::src::renderergl1::tr_init::glConfig.vidWidth = desktopMode.w;
            crate::src::renderergl1::tr_init::glConfig.vidHeight = desktopMode.h
        } else {
            crate::src::renderergl1::tr_init::glConfig.vidWidth = 640 as i32;
            crate::src::renderergl1::tr_init::glConfig.vidHeight = 480 as i32;
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"Cannot determine display resolution, assuming 640x480\n\x00" as *const u8
                    as *const libc::c_char,
            );
        }
        crate::src::renderergl1::tr_init::glConfig.windowAspect =
            crate::src::renderergl1::tr_init::glConfig.vidWidth as f32
                / crate::src::renderergl1::tr_init::glConfig.vidHeight as f32
    } else if crate::src::renderergl1::tr_init::R_GetModeInfo(
        &mut crate::src::renderergl1::tr_init::glConfig.vidWidth,
        &mut crate::src::renderergl1::tr_init::glConfig.vidHeight,
        &mut crate::src::renderergl1::tr_init::glConfig.windowAspect,
        mode,
    ) as u64
        == 0
    {
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            PRINT_ALL as i32,
            b" invalid mode\n\x00" as *const u8 as *const libc::c_char,
        );
        return RSERR_INVALID_MODE as i32;
    }
    crate::src::renderergl1::tr_main::ri
        .Printf
        .expect("non-null function pointer")(
        PRINT_ALL as i32,
        b" %d %d\n\x00" as *const u8 as *const libc::c_char,
        crate::src::renderergl1::tr_init::glConfig.vidWidth,
        crate::src::renderergl1::tr_init::glConfig.vidHeight,
    );
    // Center window
    if (*r_centerWindow).integer != 0 && fullscreen as u64 == 0 {
        x = desktopMode.w / 2 as i32
            - crate::src::renderergl1::tr_init::glConfig.vidWidth / 2 as i32;
        y = desktopMode.h / 2 as i32
            - crate::src::renderergl1::tr_init::glConfig.vidHeight / 2 as i32
    }
    // Destroy existing state if it exists
    if !SDL_glContext.is_null() {
        GLimp_ClearProcAddresses();
        SDL_GL_DeleteContext(SDL_glContext);
        SDL_glContext = std::ptr::null_mut()
    }
    if !SDL_window.is_null() {
        SDL_GetWindowPosition(SDL_window, &mut x, &mut y);
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            PRINT_DEVELOPER as i32,
            b"Existing window at %dx%d before being destroyed\n\x00" as *const u8
                as *const libc::c_char,
            x,
            y,
        );
        SDL_DestroyWindow(SDL_window);
        SDL_window = std::ptr::null_mut()
    }
    if fullscreen as u64 != 0 {
        flags |= SDL_WINDOW_FULLSCREEN as i32 as u32;
        crate::src::renderergl1::tr_init::glConfig.isFullscreen = qtrue
    } else {
        if noborder as u64 != 0 {
            flags |= SDL_WINDOW_BORDERLESS as i32 as u32
        }
        crate::src::renderergl1::tr_init::glConfig.isFullscreen = qfalse
    }
    colorBits = (*crate::src::renderergl1::tr_init::r_colorbits).value as i32;
    if colorBits == 0 || colorBits >= 32 as i32 {
        colorBits = 24 as i32
    }
    if (*crate::src::renderergl1::tr_init::r_depthbits).value == 0. {
        depthBits = 24 as i32
    } else {
        depthBits = (*crate::src::renderergl1::tr_init::r_depthbits).value as i32
    }
    stencilBits = (*crate::src::renderergl1::tr_init::r_stencilbits).value as i32;
    samples = (*crate::src::renderergl1::tr_init::r_ext_multisample).value as i32;
    let mut current_block_184: u64;
    i = 0 as i32;
    while i < 16 as i32 {
        let mut testColorBits: i32 = 0;
        let mut testDepthBits: i32 = 0;
        let mut testStencilBits: i32 = 0;
        let mut realColorBits: [i32; 3] = [0; 3];
        // 0 - default
        // 1 - minus colorBits
        // 2 - minus depthBits
        // 3 - minus stencil
        if i % 4 as i32 == 0 as i32 && i != 0 {
            let mut current_block_75: u64;
            // one pass, reduce
            match i / 4 as i32 {
                2 => {
                    if colorBits == 24 as i32 {
                        colorBits = 16 as i32
                    }
                    current_block_75 = 10778260831612459202;
                }
                1 => {
                    if depthBits == 24 as i32 {
                        depthBits = 16 as i32
                    } else if depthBits == 16 as i32 {
                        depthBits = 8 as i32
                    }
                    current_block_75 = 13735627286979930116;
                }
                3 => {
                    current_block_75 = 13735627286979930116;
                }
                _ => {
                    current_block_75 = 10778260831612459202;
                }
            }
            match current_block_75 {
                13735627286979930116 => {
                    if stencilBits == 24 as i32 {
                        stencilBits = 16 as i32
                    } else if stencilBits == 16 as i32 {
                        stencilBits = 8 as i32
                    }
                }
                _ => {}
            }
        }
        testColorBits = colorBits;
        testDepthBits = depthBits;
        testStencilBits = stencilBits;
        if i % 4 as i32 == 3 as i32 {
            // reduce colorBits
            if testColorBits == 24 as i32 {
                testColorBits = 16 as i32
            }
        }
        if i % 4 as i32 == 2 as i32 {
            // reduce depthBits
            if testDepthBits == 24 as i32 {
                testDepthBits = 16 as i32
            } else if testDepthBits == 16 as i32 {
                testDepthBits = 8 as i32
            }
        }
        if i % 4 as i32 == 1 as i32 {
            // reduce stencilBits
            if testStencilBits == 24 as i32 {
                testStencilBits = 16 as i32
            } else if testStencilBits == 16 as i32 {
                testStencilBits = 8 as i32
            } else {
                testStencilBits = 0 as i32
            }
        }
        if testColorBits == 24 as i32 {
            perChannelColorBits = 8 as i32
        } else {
            perChannelColorBits = 4 as i32
        }
        /* Fix for SGIs grabbing too many bits of color */
        SDL_GL_SetAttribute(SDL_GL_RED_SIZE, perChannelColorBits);
        SDL_GL_SetAttribute(SDL_GL_GREEN_SIZE, perChannelColorBits);
        SDL_GL_SetAttribute(SDL_GL_BLUE_SIZE, perChannelColorBits);
        SDL_GL_SetAttribute(SDL_GL_DEPTH_SIZE, testDepthBits);
        SDL_GL_SetAttribute(SDL_GL_STENCIL_SIZE, testStencilBits);
        SDL_GL_SetAttribute(
            SDL_GL_MULTISAMPLEBUFFERS,
            if samples != 0 { 1 as i32 } else { 0 as i32 },
        );
        SDL_GL_SetAttribute(SDL_GL_MULTISAMPLESAMPLES, samples);
        if (*crate::src::renderergl1::tr_init::r_stereoEnabled).integer != 0 {
            crate::src::renderergl1::tr_init::glConfig.stereoEnabled = qtrue;
            SDL_GL_SetAttribute(SDL_GL_STEREO, 1 as i32);
        } else {
            crate::src::renderergl1::tr_init::glConfig.stereoEnabled = qfalse;
            SDL_GL_SetAttribute(SDL_GL_STEREO, 0 as i32);
        }
        SDL_GL_SetAttribute(SDL_GL_DOUBLEBUFFER, 1 as i32);
        // if multisampling is enabled on X11, this causes create window to fail.
        SDL_window = SDL_CreateWindow(
            b"ioquake3\x00" as *const u8 as *const libc::c_char,
            x,
            y,
            crate::src::renderergl1::tr_init::glConfig.vidWidth,
            crate::src::renderergl1::tr_init::glConfig.vidHeight,
            flags,
        );
        if SDL_window.is_null() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_DEVELOPER as i32,
                b"SDL_CreateWindow failed: %s\n\x00" as *const u8 as *const libc::c_char,
                crate::stdlib::SDL_GetError(),
            );
        } else {
            if fullscreen as u64 != 0 {
                let mut mode_0: SDL_DisplayMode = SDL_DisplayMode {
                    format: 0,
                    w: 0,
                    h: 0,
                    refresh_rate: 0,
                    driverdata: std::ptr::null_mut(),
                };
                match testColorBits {
                    16 => {
                        mode_0.format = SDL_PIXELFORMAT_RGB565 as i32 as Uint32;
                        current_block_184 = 1425453989644512380;
                    }
                    24 => {
                        mode_0.format = SDL_PIXELFORMAT_RGB24 as i32 as Uint32;
                        current_block_184 = 1425453989644512380;
                    }
                    _ => {
                        crate::src::renderergl1::tr_main::ri
                            .Printf
                            .expect("non-null function pointer")(
                            PRINT_DEVELOPER as i32,
                            b"testColorBits is %d, can\'t fullscreen\n\x00" as *const u8
                                as *const libc::c_char,
                            testColorBits,
                        );
                        current_block_184 = 5597585068398118923;
                    }
                }
                match current_block_184 {
                    5597585068398118923 => {}
                    _ => {
                        mode_0.w = crate::src::renderergl1::tr_init::glConfig.vidWidth;
                        mode_0.h = crate::src::renderergl1::tr_init::glConfig.vidHeight;
                        crate::src::renderergl1::tr_init::glConfig.displayFrequency =
                            crate::src::renderergl1::tr_main::ri
                                .Cvar_VariableIntegerValue
                                .expect("non-null function pointer")(
                                b"r_displayRefresh\x00" as *const u8 as *const libc::c_char,
                            );
                        mode_0.refresh_rate =
                            crate::src::renderergl1::tr_init::glConfig.displayFrequency;
                        mode_0.driverdata = std::ptr::null_mut();
                        if SDL_SetWindowDisplayMode(SDL_window, &mut mode_0) < 0 as i32 {
                            crate::src::renderergl1::tr_main::ri
                                .Printf
                                .expect("non-null function pointer")(
                                PRINT_DEVELOPER as i32,
                                b"SDL_SetWindowDisplayMode failed: %s\n\x00" as *const u8
                                    as *const libc::c_char,
                                crate::stdlib::SDL_GetError(),
                            );
                            current_block_184 = 5597585068398118923;
                        } else {
                            current_block_184 = 2872334340672008580;
                        }
                    }
                }
            } else {
                current_block_184 = 2872334340672008580;
            }
            match current_block_184 {
                5597585068398118923 => {}
                _ => {
                    SDL_SetWindowIcon(SDL_window, icon);
                    if fixedFunction as u64 == 0 {
                        let mut profileMask: i32 = 0;
                        let mut majorVersion: i32 = 0;
                        let mut minorVersion: i32 = 0;
                        SDL_GL_GetAttribute(SDL_GL_CONTEXT_PROFILE_MASK, &mut profileMask);
                        SDL_GL_GetAttribute(SDL_GL_CONTEXT_MAJOR_VERSION, &mut majorVersion);
                        SDL_GL_GetAttribute(SDL_GL_CONTEXT_MINOR_VERSION, &mut minorVersion);
                        crate::src::renderergl1::tr_main::ri
                            .Printf
                            .expect("non-null function pointer")(
                            PRINT_ALL as i32,
                            b"Trying to get an OpenGL 3.2 core context\n\x00" as *const u8
                                as *const libc::c_char,
                        );
                        SDL_GL_SetAttribute(
                            SDL_GL_CONTEXT_PROFILE_MASK,
                            SDL_GL_CONTEXT_PROFILE_CORE as i32,
                        );
                        SDL_GL_SetAttribute(SDL_GL_CONTEXT_MAJOR_VERSION, 3 as i32);
                        SDL_GL_SetAttribute(SDL_GL_CONTEXT_MINOR_VERSION, 2 as i32);
                        SDL_glContext = SDL_GL_CreateContext(SDL_window);
                        if SDL_glContext.is_null() {
                            crate::src::renderergl1::tr_main::ri
                                .Printf
                                .expect("non-null function pointer")(
                                PRINT_ALL as i32,
                                b"SDL_GL_CreateContext failed: %s\n\x00" as *const u8
                                    as *const libc::c_char,
                                crate::stdlib::SDL_GetError(),
                            );
                            crate::src::renderergl1::tr_main::ri
                                .Printf
                                .expect("non-null function pointer")(
                                PRINT_ALL as i32,
                                b"Reverting to default context\n\x00" as *const u8
                                    as *const libc::c_char,
                            );
                            SDL_GL_SetAttribute(SDL_GL_CONTEXT_PROFILE_MASK, profileMask);
                            SDL_GL_SetAttribute(SDL_GL_CONTEXT_MAJOR_VERSION, majorVersion);
                            SDL_GL_SetAttribute(SDL_GL_CONTEXT_MINOR_VERSION, minorVersion);
                        } else {
                            let mut renderer: *const libc::c_char = 0 as *const libc::c_char;
                            crate::src::renderergl1::tr_main::ri
                                .Printf
                                .expect("non-null function pointer")(
                                PRINT_ALL as i32,
                                b"SDL_GL_CreateContext succeeded.\n\x00" as *const u8
                                    as *const libc::c_char,
                            );
                            if GLimp_GetProcAddresses(fixedFunction) as u64 != 0 {
                                renderer = qglGetString.expect("non-null function pointer")(
                                    0x1f01 as i32 as GLenum,
                                ) as *const libc::c_char
                            } else {
                                crate::src::renderergl1::tr_main::ri.Printf.expect("non-null function pointer")(PRINT_ALL
                                                                                  as
                                                                                  i32,
                                                                              b"GLimp_GetProcAddresses() failed for OpenGL 3.2 core context\n\x00"
                                                                                  as
                                                                                  *const u8
                                                                                  as
                                                                                  *const libc::c_char);
                                renderer = 0 as *const libc::c_char
                            }
                            if renderer.is_null()
                                || (!libc::strstr(
                                    renderer,
                                    b"Software Renderer\x00" as *const u8 as *const libc::c_char,
                                )
                                .is_null()
                                    || !libc::strstr(
                                        renderer,
                                        b"Software Rasterizer\x00" as *const u8
                                            as *const libc::c_char,
                                    )
                                    .is_null())
                            {
                                if !renderer.is_null() {
                                    crate::src::renderergl1::tr_main::ri
                                        .Printf
                                        .expect("non-null function pointer")(
                                        PRINT_ALL as i32,
                                        b"GL_RENDERER is %s, rejecting context\n\x00" as *const u8
                                            as *const libc::c_char,
                                        renderer,
                                    );
                                }
                                GLimp_ClearProcAddresses();
                                SDL_GL_DeleteContext(SDL_glContext);
                                SDL_glContext = std::ptr::null_mut();
                                SDL_GL_SetAttribute(SDL_GL_CONTEXT_PROFILE_MASK, profileMask);
                                SDL_GL_SetAttribute(SDL_GL_CONTEXT_MAJOR_VERSION, majorVersion);
                                SDL_GL_SetAttribute(SDL_GL_CONTEXT_MINOR_VERSION, minorVersion);
                            }
                        }
                    } else {
                        SDL_glContext = std::ptr::null_mut()
                    }
                    if SDL_glContext.is_null() {
                        SDL_glContext = SDL_GL_CreateContext(SDL_window);
                        if SDL_glContext.is_null() {
                            crate::src::renderergl1::tr_main::ri
                                .Printf
                                .expect("non-null function pointer")(
                                PRINT_DEVELOPER as i32,
                                b"SDL_GL_CreateContext failed: %s\n\x00" as *const u8
                                    as *const libc::c_char,
                                crate::stdlib::SDL_GetError(),
                            );
                            SDL_DestroyWindow(SDL_window);
                            SDL_window = std::ptr::null_mut();
                            current_block_184 = 5597585068398118923;
                        } else if GLimp_GetProcAddresses(fixedFunction) as u64 == 0 {
                            crate::src::renderergl1::tr_main::ri
                                .Printf
                                .expect("non-null function pointer")(
                                PRINT_ALL as i32,
                                b"GLimp_GetProcAddresses() failed\n\x00" as *const u8
                                    as *const libc::c_char,
                            );
                            GLimp_ClearProcAddresses();
                            SDL_GL_DeleteContext(SDL_glContext);
                            SDL_glContext = std::ptr::null_mut();
                            SDL_DestroyWindow(SDL_window);
                            SDL_window = std::ptr::null_mut();
                            current_block_184 = 5597585068398118923;
                        } else {
                            current_block_184 = 1953367063549441504;
                        }
                    } else {
                        current_block_184 = 1953367063549441504;
                    }
                    match current_block_184 {
                        5597585068398118923 => {}
                        _ => {
                            qglClearColor.expect("non-null function pointer")(
                                0 as i32 as GLclampf,
                                0 as i32 as GLclampf,
                                0 as i32 as GLclampf,
                                1 as i32 as GLclampf,
                            );
                            qglClear.expect("non-null function pointer")(
                                0x4000 as i32 as GLbitfield,
                            );
                            SDL_GL_SwapWindow(SDL_window);
                            if SDL_GL_SetSwapInterval(
                                (*crate::src::renderergl1::tr_init::r_swapInterval).integer,
                            ) == -(1 as i32)
                            {
                                crate::src::renderergl1::tr_main::ri
                                    .Printf
                                    .expect("non-null function pointer")(
                                    PRINT_DEVELOPER as i32,
                                    b"SDL_GL_SetSwapInterval failed: %s\n\x00" as *const u8
                                        as *const libc::c_char,
                                    crate::stdlib::SDL_GetError(),
                                );
                            }
                            SDL_GL_GetAttribute(
                                SDL_GL_RED_SIZE,
                                &mut *realColorBits.as_mut_ptr().offset(0 as i32 as isize),
                            );
                            SDL_GL_GetAttribute(
                                SDL_GL_GREEN_SIZE,
                                &mut *realColorBits.as_mut_ptr().offset(1 as i32 as isize),
                            );
                            SDL_GL_GetAttribute(
                                SDL_GL_BLUE_SIZE,
                                &mut *realColorBits.as_mut_ptr().offset(2 as i32 as isize),
                            );
                            SDL_GL_GetAttribute(
                                SDL_GL_DEPTH_SIZE,
                                &mut crate::src::renderergl1::tr_init::glConfig.depthBits,
                            );
                            SDL_GL_GetAttribute(
                                SDL_GL_STENCIL_SIZE,
                                &mut crate::src::renderergl1::tr_init::glConfig.stencilBits,
                            );
                            crate::src::renderergl1::tr_init::glConfig.colorBits = realColorBits
                                [0 as i32 as usize]
                                + realColorBits[1 as i32 as usize]
                                + realColorBits[2 as i32 as usize];
                            crate::src::renderergl1::tr_main::ri
                                .Printf
                                .expect("non-null function pointer")(
                                PRINT_ALL as i32,
                                b"Using %d color bits, %d depth, %d stencil display.\n\x00"
                                    as *const u8
                                    as *const libc::c_char,
                                crate::src::renderergl1::tr_init::glConfig.colorBits,
                                crate::src::renderergl1::tr_init::glConfig.depthBits,
                                crate::src::renderergl1::tr_init::glConfig.stencilBits,
                            );
                            break;
                        }
                    }
                }
            }
        }
        i += 1
    }
    SDL_FreeSurface(icon);
    if SDL_window.is_null() {
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            PRINT_ALL as i32,
            b"Couldn\'t get a visual\n\x00" as *const u8 as *const libc::c_char,
        );
        return RSERR_INVALID_MODE as i32;
    }
    GLimp_DetectAvailableModes();
    glstring = qglGetString.expect("non-null function pointer")(0x1f01 as i32 as GLenum)
        as *mut libc::c_char;
    crate::src::renderergl1::tr_main::ri
        .Printf
        .expect("non-null function pointer")(
        PRINT_ALL as i32,
        b"GL_RENDERER: %s\n\x00" as *const u8 as *const libc::c_char,
        glstring,
    );
    return RSERR_OK as i32;
}
/*
===============
GLimp_StartDriverAndSetMode
===============
*/

unsafe extern "C" fn GLimp_StartDriverAndSetMode(
    mut mode: i32,
    mut fullscreen: qboolean,
    mut noborder: qboolean,
    mut gl3Core: qboolean,
) -> qboolean {
    let mut err: rserr_t = RSERR_OK;
    if crate::stdlib::SDL_WasInit(0x20 as u32) == 0 {
        let mut driverName: *const libc::c_char = 0 as *const libc::c_char;
        if crate::stdlib::SDL_Init(0x20 as u32) != 0 as i32 {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"SDL_Init( SDL_INIT_VIDEO ) FAILED (%s)\n\x00" as *const u8 as *const libc::c_char,
                crate::stdlib::SDL_GetError(),
            );
            return qfalse;
        }
        driverName = SDL_GetCurrentVideoDriver();
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            PRINT_ALL as i32,
            b"SDL using driver \"%s\"\n\x00" as *const u8 as *const libc::c_char,
            driverName,
        );
        crate::src::renderergl1::tr_main::ri
            .Cvar_Set
            .expect("non-null function pointer")(
            b"r_sdlDriver\x00" as *const u8 as *const libc::c_char,
            driverName,
        );
    }
    if fullscreen as u32 != 0
        && crate::src::renderergl1::tr_main::ri
            .Cvar_VariableIntegerValue
            .expect("non-null function pointer")(
            b"in_nograb\x00" as *const u8 as *const libc::c_char,
        ) != 0
    {
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            PRINT_ALL as i32,
            b"Fullscreen not allowed with in_nograb 1\n\x00" as *const u8 as *const libc::c_char,
        );
        crate::src::renderergl1::tr_main::ri
            .Cvar_Set
            .expect("non-null function pointer")(
            b"r_fullscreen\x00" as *const u8 as *const libc::c_char,
            b"0\x00" as *const u8 as *const libc::c_char,
        );
        (*crate::src::renderergl1::tr_init::r_fullscreen).modified = qfalse;
        fullscreen = qfalse
    }
    err = GLimp_SetMode(mode, fullscreen, noborder, gl3Core) as rserr_t;
    match err as u32 {
        1 => {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"...WARNING: fullscreen unavailable in this mode\n\x00" as *const u8
                    as *const libc::c_char,
            );
            return qfalse;
        }
        2 => {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"...WARNING: could not set the given mode (%d)\n\x00" as *const u8
                    as *const libc::c_char,
                mode,
            );
            return qfalse;
        }
        _ => {}
    }
    return qtrue;
}
/*
===============
GLimp_InitExtensions
===============
*/

unsafe extern "C" fn GLimp_InitExtensions(mut fixedFunction: qboolean) {
    if (*crate::src::renderergl1::tr_init::r_allowExtensions).integer == 0 {
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            PRINT_ALL as i32,
            b"* IGNORING OPENGL EXTENSIONS *\n\x00" as *const u8 as *const libc::c_char,
        );
        return;
    }
    crate::src::renderergl1::tr_main::ri
        .Printf
        .expect("non-null function pointer")(
        PRINT_ALL as i32,
        b"Initializing OpenGL extensions\n\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::renderergl1::tr_init::glConfig.textureCompression = TC_NONE;
    // GL_EXT_texture_compression_s3tc
    if SDL_GL_ExtensionSupported(
        b"GL_ARB_texture_compression\x00" as *const u8 as *const libc::c_char,
    ) as u32
        != 0
        && SDL_GL_ExtensionSupported(
            b"GL_EXT_texture_compression_s3tc\x00" as *const u8 as *const libc::c_char,
        ) as u32
            != 0
    {
        if (*crate::src::renderergl1::tr_init::r_ext_compressed_textures).value != 0. {
            crate::src::renderergl1::tr_init::glConfig.textureCompression = TC_S3TC_ARB;
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"...using GL_EXT_texture_compression_s3tc\n\x00" as *const u8
                    as *const libc::c_char,
            );
        } else {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"...ignoring GL_EXT_texture_compression_s3tc\n\x00" as *const u8
                    as *const libc::c_char,
            );
        }
    } else {
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            PRINT_ALL as i32,
            b"...GL_EXT_texture_compression_s3tc not found\n\x00" as *const u8
                as *const libc::c_char,
        );
    }
    // GL_S3_s3tc ... legacy extension before GL_EXT_texture_compression_s3tc.
    if crate::src::renderergl1::tr_init::glConfig.textureCompression as u32 == TC_NONE as i32 as u32
    {
        if SDL_GL_ExtensionSupported(b"GL_S3_s3tc\x00" as *const u8 as *const libc::c_char) as u64
            != 0
        {
            if (*crate::src::renderergl1::tr_init::r_ext_compressed_textures).value != 0. {
                crate::src::renderergl1::tr_init::glConfig.textureCompression = TC_S3TC;
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"...using GL_S3_s3tc\n\x00" as *const u8 as *const libc::c_char,
                );
            } else {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"...ignoring GL_S3_s3tc\n\x00" as *const u8 as *const libc::c_char,
                );
            }
        } else {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"...GL_S3_s3tc not found\n\x00" as *const u8 as *const libc::c_char,
            );
        }
    }
    // OpenGL 1 fixed function pipeline
    if fixedFunction as u64 != 0 {
        // GL_EXT_texture_env_add
        crate::src::renderergl1::tr_init::glConfig.textureEnvAddAvailable = qfalse;
        if SDL_GL_ExtensionSupported(
            b"GL_EXT_texture_env_add\x00" as *const u8 as *const libc::c_char,
        ) as u64
            != 0
        {
            if (*crate::src::renderergl1::tr_init::r_ext_texture_env_add).integer != 0 {
                crate::src::renderergl1::tr_init::glConfig.textureEnvAddAvailable = qtrue;
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"...using GL_EXT_texture_env_add\n\x00" as *const u8 as *const libc::c_char,
                );
            } else {
                crate::src::renderergl1::tr_init::glConfig.textureEnvAddAvailable = qfalse;
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"...ignoring GL_EXT_texture_env_add\n\x00" as *const u8 as *const libc::c_char,
                );
            }
        } else {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"...GL_EXT_texture_env_add not found\n\x00" as *const u8 as *const libc::c_char,
            );
        }
        // GL_ARB_multitexture
        qglMultiTexCoord2fARB = None;
        qglActiveTextureARB = None;
        qglClientActiveTextureARB = None;
        if SDL_GL_ExtensionSupported(b"GL_ARB_multitexture\x00" as *const u8 as *const libc::c_char)
            as u64
            != 0
        {
            if (*crate::src::renderergl1::tr_init::r_ext_multitexture).value != 0. {
                qglMultiTexCoord2fARB = ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option<unsafe extern "C" fn(_: GLenum, _: GLfloat, _: GLfloat) -> ()>,
                >(SDL_GL_GetProcAddress(
                    b"glMultiTexCoord2fARB\x00" as *const u8 as *const libc::c_char,
                ));
                qglActiveTextureARB = ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option<unsafe extern "C" fn(_: GLenum) -> ()>,
                >(SDL_GL_GetProcAddress(
                    b"glActiveTextureARB\x00" as *const u8 as *const libc::c_char,
                ));
                qglClientActiveTextureARB = ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option<unsafe extern "C" fn(_: GLenum) -> ()>,
                >(SDL_GL_GetProcAddress(
                    b"glClientActiveTextureARB\x00" as *const u8 as *const libc::c_char,
                ));
                if qglActiveTextureARB.is_some() {
                    let mut glint: GLint = 0 as i32;
                    qglGetIntegerv.expect("non-null function pointer")(
                        0x84e2 as i32 as GLenum,
                        &mut glint,
                    );
                    crate::src::renderergl1::tr_init::glConfig.numTextureUnits = glint;
                    if crate::src::renderergl1::tr_init::glConfig.numTextureUnits > 1 as i32 {
                        crate::src::renderergl1::tr_main::ri
                            .Printf
                            .expect("non-null function pointer")(
                            PRINT_ALL as i32,
                            b"...using GL_ARB_multitexture\n\x00" as *const u8
                                as *const libc::c_char,
                        );
                    } else {
                        qglMultiTexCoord2fARB = None;
                        qglActiveTextureARB = None;
                        qglClientActiveTextureARB = None;
                        crate::src::renderergl1::tr_main::ri
                            .Printf
                            .expect("non-null function pointer")(
                            PRINT_ALL as i32,
                            b"...not using GL_ARB_multitexture, < 2 texture units\n\x00"
                                as *const u8 as *const libc::c_char,
                        );
                    }
                }
            } else {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"...ignoring GL_ARB_multitexture\n\x00" as *const u8 as *const libc::c_char,
                );
            }
        } else {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"...GL_ARB_multitexture not found\n\x00" as *const u8 as *const libc::c_char,
            );
        }
        // GL_EXT_compiled_vertex_array
        if SDL_GL_ExtensionSupported(
            b"GL_EXT_compiled_vertex_array\x00" as *const u8 as *const libc::c_char,
        ) as u64
            != 0
        {
            if (*crate::src::renderergl1::tr_init::r_ext_compiled_vertex_array).value != 0. {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"...using GL_EXT_compiled_vertex_array\n\x00" as *const u8
                        as *const libc::c_char,
                );
                qglLockArraysEXT = ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option<unsafe extern "C" fn(_: GLint, _: GLint) -> ()>,
                >(SDL_GL_GetProcAddress(
                    b"glLockArraysEXT\x00" as *const u8 as *const libc::c_char,
                ));
                qglUnlockArraysEXT = ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option<unsafe extern "C" fn() -> ()>,
                >(SDL_GL_GetProcAddress(
                    b"glUnlockArraysEXT\x00" as *const u8 as *const libc::c_char,
                ));
                if qglLockArraysEXT.is_none() || qglUnlockArraysEXT.is_none() {
                    crate::src::renderergl1::tr_main::ri
                        .Error
                        .expect("non-null function pointer")(
                        ERR_FATAL as i32,
                        b"bad getprocaddress\x00" as *const u8 as *const libc::c_char,
                    );
                }
            } else {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"...ignoring GL_EXT_compiled_vertex_array\n\x00" as *const u8
                        as *const libc::c_char,
                );
            }
        } else {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"...GL_EXT_compiled_vertex_array not found\n\x00" as *const u8
                    as *const libc::c_char,
            );
        }
    }
    crate::src::renderergl1::tr_init::textureFilterAnisotropic = qfalse;
    if SDL_GL_ExtensionSupported(
        b"GL_EXT_texture_filter_anisotropic\x00" as *const u8 as *const libc::c_char,
    ) as u64
        != 0
    {
        if (*crate::src::renderergl1::tr_init::r_ext_texture_filter_anisotropic).integer != 0 {
            qglGetIntegerv.expect("non-null function pointer")(
                0x84ff as i32 as GLenum,
                &mut crate::src::renderergl1::tr_init::maxAnisotropy as *mut i32 as *mut GLint,
            );
            if crate::src::renderergl1::tr_init::maxAnisotropy <= 0 as i32 {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"...GL_EXT_texture_filter_anisotropic not properly supported!\n\x00"
                        as *const u8 as *const libc::c_char,
                );
                crate::src::renderergl1::tr_init::maxAnisotropy = 0 as i32
            } else {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"...using GL_EXT_texture_filter_anisotropic (max: %i)\n\x00" as *const u8
                        as *const libc::c_char,
                    crate::src::renderergl1::tr_init::maxAnisotropy,
                );
                crate::src::renderergl1::tr_init::textureFilterAnisotropic = qtrue
            }
        } else {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"...ignoring GL_EXT_texture_filter_anisotropic\n\x00" as *const u8
                    as *const libc::c_char,
            );
        }
    } else {
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            PRINT_ALL as i32,
            b"...GL_EXT_texture_filter_anisotropic not found\n\x00" as *const u8
                as *const libc::c_char,
        );
    };
}
// 640 * 480
/*
===============
GLimp_Init

This routine is responsible for initializing the OS specific portions
of OpenGL
===============
*/
#[no_mangle]

pub unsafe extern "C" fn GLimp_Init(mut fixedFunction: qboolean) {
    let mut current_block: u64;
    crate::src::renderergl1::tr_main::ri
        .Printf
        .expect("non-null function pointer")(
        PRINT_DEVELOPER as i32,
        b"Glimp_Init( )\n\x00" as *const u8 as *const libc::c_char,
    );
    r_allowSoftwareGL = crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"r_allowSoftwareGL\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x20 as i32,
    );
    r_sdlDriver = crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"r_sdlDriver\x00" as *const u8 as *const libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char,
        0x40 as i32,
    );
    r_allowResize = crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"r_allowResize\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x1 as i32 | 0x20 as i32,
    );
    r_centerWindow = crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"r_centerWindow\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x1 as i32 | 0x20 as i32,
    );
    if crate::src::renderergl1::tr_main::ri
        .Cvar_VariableIntegerValue
        .expect("non-null function pointer")(
        b"com_abnormalExit\x00" as *const u8 as *const libc::c_char,
    ) != 0
    {
        crate::src::renderergl1::tr_main::ri
            .Cvar_Set
            .expect("non-null function pointer")(
            b"r_mode\x00" as *const u8 as *const libc::c_char,
            va(
                b"%d\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                3 as i32,
            ),
        );
        crate::src::renderergl1::tr_main::ri
            .Cvar_Set
            .expect("non-null function pointer")(
            b"r_fullscreen\x00" as *const u8 as *const libc::c_char,
            b"0\x00" as *const u8 as *const libc::c_char,
        );
        crate::src::renderergl1::tr_main::ri
            .Cvar_Set
            .expect("non-null function pointer")(
            b"r_centerWindow\x00" as *const u8 as *const libc::c_char,
            b"0\x00" as *const u8 as *const libc::c_char,
        );
        crate::src::renderergl1::tr_main::ri
            .Cvar_Set
            .expect("non-null function pointer")(
            b"com_abnormalExit\x00" as *const u8 as *const libc::c_char,
            b"0\x00" as *const u8 as *const libc::c_char,
        );
    }
    crate::src::renderergl1::tr_main::ri
        .Sys_GLimpInit
        .expect("non-null function pointer")();
    // Create the window and set up the context
    if !(GLimp_StartDriverAndSetMode(
        (*crate::src::renderergl1::tr_init::r_mode).integer,
        (*crate::src::renderergl1::tr_init::r_fullscreen).integer as qboolean,
        (*crate::src::renderergl1::tr_init::r_noborder).integer as qboolean,
        fixedFunction,
    ) as u64
        != 0)
    {
        // Try again, this time in a platform specific "safe mode"
        crate::src::renderergl1::tr_main::ri
            .Sys_GLimpSafeInit
            .expect("non-null function pointer")();
        if !(GLimp_StartDriverAndSetMode(
            (*crate::src::renderergl1::tr_init::r_mode).integer,
            (*crate::src::renderergl1::tr_init::r_fullscreen).integer as qboolean,
            qfalse,
            fixedFunction,
        ) as u64
            != 0)
        {
            // Finally, try the default screen resolution
            if (*crate::src::renderergl1::tr_init::r_mode).integer != 3 as i32 {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    PRINT_ALL as i32,
                    b"Setting r_mode %d failed, falling back on r_mode %d\n\x00" as *const u8
                        as *const libc::c_char,
                    (*crate::src::renderergl1::tr_init::r_mode).integer,
                    3 as i32,
                );
                if GLimp_StartDriverAndSetMode(3 as i32, qfalse, qfalse, fixedFunction) as u64 != 0
                {
                    current_block = 18052364713975350134;
                } else {
                    current_block = 7149356873433890176;
                }
            } else {
                current_block = 7149356873433890176;
            }
            match current_block {
                18052364713975350134 => {}
                _ => {
                    // Nothing worked, give up
                    crate::src::renderergl1::tr_main::ri
                        .Error
                        .expect("non-null function pointer")(
                        ERR_FATAL as i32,
                        b"GLimp_Init() - could not load OpenGL subsystem\x00" as *const u8
                            as *const libc::c_char,
                    );
                }
            }
        }
    }
    // These values force the UI to disable driver selection
    crate::src::renderergl1::tr_init::glConfig.driverType = GLDRV_ICD;
    crate::src::renderergl1::tr_init::glConfig.hardwareType = GLHW_GENERIC;
    // Only using SDL_SetWindowBrightness to determine if hardware gamma is supported
    crate::src::renderergl1::tr_init::glConfig.deviceSupportsGamma =
        ((*crate::src::renderergl1::tr_init::r_ignorehwgamma).integer == 0
            && SDL_SetWindowBrightness(SDL_window, 1.0f32) >= 0 as i32) as i32 as qboolean;
    // get our config strings
    Q_strncpyz(
        crate::src::renderergl1::tr_init::glConfig
            .vendor_string
            .as_mut_ptr(),
        qglGetString.expect("non-null function pointer")(0x1f00 as i32 as GLenum)
            as *mut libc::c_char,
        ::std::mem::size_of::<[libc::c_char; 1024]>() as usize as i32,
    );
    Q_strncpyz(
        crate::src::renderergl1::tr_init::glConfig
            .renderer_string
            .as_mut_ptr(),
        qglGetString.expect("non-null function pointer")(0x1f01 as i32 as GLenum)
            as *mut libc::c_char,
        ::std::mem::size_of::<[libc::c_char; 1024]>() as usize as i32,
    );
    if *crate::src::renderergl1::tr_init::glConfig
        .renderer_string
        .as_mut_ptr() as i32
        != 0
        && crate::src::renderergl1::tr_init::glConfig.renderer_string[crate::stdlib::strlen(
            crate::src::renderergl1::tr_init::glConfig
                .renderer_string
                .as_mut_ptr(),
        )
        .wrapping_sub(1 as i32 as usize)
            as usize] as i32
            == '\n' as i32
    {
        crate::src::renderergl1::tr_init::glConfig.renderer_string[crate::stdlib::strlen(
            crate::src::renderergl1::tr_init::glConfig
                .renderer_string
                .as_mut_ptr(),
        )
        .wrapping_sub(1 as i32 as usize)
            as usize] = 0 as i32 as libc::c_char
    }
    Q_strncpyz(
        crate::src::renderergl1::tr_init::glConfig
            .version_string
            .as_mut_ptr(),
        qglGetString.expect("non-null function pointer")(0x1f02 as i32 as GLenum)
            as *mut libc::c_char,
        ::std::mem::size_of::<[libc::c_char; 1024]>() as usize as i32,
    );
    // manually create extension list if using OpenGL 3
    if qglGetStringi.is_some() {
        let mut i: i32 = 0;
        let mut numExtensions: i32 = 0;
        let mut extensionLength: i32 = 0;
        let mut listLength: i32 = 0;
        let mut extension: *const libc::c_char = 0 as *const libc::c_char;
        qglGetIntegerv.expect("non-null function pointer")(
            0x821d as i32 as GLenum,
            &mut numExtensions,
        );
        listLength = 0 as i32;
        i = 0 as i32;
        while i < numExtensions {
            extension = qglGetStringi.expect("non-null function pointer")(
                0x1f03 as i32 as GLenum,
                i as GLuint,
            ) as *mut libc::c_char;
            extensionLength = crate::stdlib::strlen(extension) as i32;
            if (listLength + extensionLength + 1 as i32) as usize
                >= ::std::mem::size_of::<[libc::c_char; 8192]>() as usize
            {
                break;
            }
            if i > 0 as i32 {
                Q_strcat(
                    crate::src::renderergl1::tr_init::glConfig
                        .extensions_string
                        .as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 8192]>() as usize as i32,
                    b" \x00" as *const u8 as *const libc::c_char,
                );
                listLength += 1
            }
            Q_strcat(
                crate::src::renderergl1::tr_init::glConfig
                    .extensions_string
                    .as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 8192]>() as usize as i32,
                extension,
            );
            listLength += extensionLength;
            i += 1
        }
    } else {
        Q_strncpyz(
            crate::src::renderergl1::tr_init::glConfig
                .extensions_string
                .as_mut_ptr(),
            qglGetString.expect("non-null function pointer")(0x1f03 as i32 as GLenum)
                as *mut libc::c_char,
            ::std::mem::size_of::<[libc::c_char; 8192]>() as usize as i32,
        );
    }
    // initialize extensions
    GLimp_InitExtensions(fixedFunction);
    crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"r_availableModes\x00" as *const u8 as *const libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char,
        0x40 as i32,
    );
    // This depends on SDL_INIT_VIDEO, hence having it here
    crate::src::renderergl1::tr_main::ri
        .IN_Init
        .expect("non-null function pointer")(SDL_window as *mut libc::c_void);
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
// for color, lightmap, diffuse, and specular
// normals are swizzled, deluxe are not
// game path, including extension
// source image
// after power of two and picmip but not including clamp to MAX_TEXTURE_SIZE
// gl texture binding
// for texture usage in frame statistics
// only needed for voodoo2
// any change in the LIGHTMAP_* defines here MUST be reflected in
// R_FindShader() in tr_bsp.c
// shader is for 2D rendering
// pre-lit triangle models
// outside of TR since it shouldn't be cleared during ref re-init
// These variables should live inside glConfig but can't because of
// compatibility issues to the original ID vms.  If you release a stand-alone
// game and your mod uses tr_types.h from this build you can safely move them
// to the glconfig_t struct.
//
// cvars
//
// number of desired stencil bits
// number of desired depth bits
// number of desired color bits, only relevant for fullscreen
// number of desired texture bits
// 0 = use framebuffer depth
// 16 = use 16-bit textures
// 32 = use 32-bit textures
// all else = error
// video mode
// overrides hardware gamma capabilities
// global enable/disable of OpenGL extensions
// these control use of specific extensions
// font stuff
/*
=============================================================

IMAGE LOADERS

=============================================================
*/
/*
====================================================================

IMPLEMENTATION SPECIFIC FUNCTIONS

====================================================================
*/
/*
===============
GLimp_EndFrame

Responsible for doing a swapbuffers
===============
*/
#[no_mangle]

pub unsafe extern "C" fn GLimp_EndFrame() {
    // don't flip if drawing to front buffer
    if Q_stricmp(
        (*crate::src::renderergl1::tr_init::r_drawBuffer).string,
        b"GL_FRONT\x00" as *const u8 as *const libc::c_char,
    ) != 0 as i32
    {
        SDL_GL_SwapWindow(SDL_window);
    }
    if (*crate::src::renderergl1::tr_init::r_fullscreen).modified as u64 != 0 {
        let mut fullscreen: i32 = 0;
        let mut needToToggle: qboolean = qfalse;
        let mut sdlToggled: qboolean = qfalse;
        // Find out the current state
        fullscreen =
            (SDL_GetWindowFlags(SDL_window) & SDL_WINDOW_FULLSCREEN as i32 as u32 != 0) as i32;
        if (*crate::src::renderergl1::tr_init::r_fullscreen).integer != 0
            && crate::src::renderergl1::tr_main::ri
                .Cvar_VariableIntegerValue
                .expect("non-null function pointer")(
                b"in_nograb\x00" as *const u8 as *const libc::c_char,
            ) != 0
        {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"Fullscreen not allowed with in_nograb 1\n\x00" as *const u8
                    as *const libc::c_char,
            );
            crate::src::renderergl1::tr_main::ri
                .Cvar_Set
                .expect("non-null function pointer")(
                b"r_fullscreen\x00" as *const u8 as *const libc::c_char,
                b"0\x00" as *const u8 as *const libc::c_char,
            );
            (*crate::src::renderergl1::tr_init::r_fullscreen).modified = qfalse
        }
        // Is the state we want different from the current state?
        needToToggle = (((*crate::src::renderergl1::tr_init::r_fullscreen).integer != 0) as i32
            != fullscreen) as i32 as qboolean;
        if needToToggle as u64 != 0 {
            sdlToggled = (SDL_SetWindowFullscreen(
                SDL_window,
                (*crate::src::renderergl1::tr_init::r_fullscreen).integer as Uint32,
            ) >= 0 as i32) as i32 as qboolean;
            // SDL_WM_ToggleFullScreen didn't work, so do it the slow way
            if sdlToggled as u64 == 0 {
                crate::src::renderergl1::tr_main::ri
                    .Cmd_ExecuteText
                    .expect("non-null function pointer")(
                    EXEC_APPEND as i32,
                    b"vid_restart\n\x00" as *const u8 as *const libc::c_char,
                );
            }
            crate::src::renderergl1::tr_main::ri
                .IN_Restart
                .expect("non-null function pointer")();
        }
        (*crate::src::renderergl1::tr_init::r_fullscreen).modified = qfalse
    };
}

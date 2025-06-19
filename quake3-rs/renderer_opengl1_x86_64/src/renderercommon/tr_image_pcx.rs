use ::libc;

pub use crate::src::jpeg_8c::jerror::C2RustUnnamed_1;
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

pub use crate::tr_public_h::refimport_t;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcx_t {
    pub manufacturer: libc::c_char,
    pub version: libc::c_char,
    pub encoding: libc::c_char,
    pub bits_per_pixel: libc::c_char,
    pub xmin: u16,
    pub ymin: u16,
    pub xmax: u16,
    pub ymax: u16,
    pub hres: u16,
    pub vres: u16,
    pub palette: [u8; 48],
    pub reserved: libc::c_char,
    pub color_planes: libc::c_char,
    pub bytes_per_line: u16,
    pub palette_type: u16,
    pub hscreensize: u16,
    pub vscreensize: u16,
    pub filler: [libc::c_char; 54],
    pub data: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union C2RustUnnamed_88 {
    pub b: *mut crate::src::qcommon::q_shared::byte,
    pub v: *mut libc::c_void,
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
#[no_mangle]

pub unsafe extern "C" fn R_LoadPCX(
    mut filename: *const libc::c_char,
    mut pic: *mut *mut crate::src::qcommon::q_shared::byte,
    mut width: *mut i32,
    mut height: *mut i32,
) {
    let mut raw: C2RustUnnamed_88 = C2RustUnnamed_88 {
        b: 0 as *mut crate::src::qcommon::q_shared::byte,
    };
    let mut end: *mut crate::src::qcommon::q_shared::byte =
        0 as *mut crate::src::qcommon::q_shared::byte;
    let mut pcx: *mut pcx_t = 0 as *mut pcx_t;
    let mut len: i32 = 0;
    let mut dataByte: u8 = 0 as i32 as u8;
    let mut runLength: u8 = 0 as i32 as u8;
    let mut out: *mut crate::src::qcommon::q_shared::byte =
        0 as *mut crate::src::qcommon::q_shared::byte;
    let mut pix: *mut crate::src::qcommon::q_shared::byte =
        0 as *mut crate::src::qcommon::q_shared::byte;
    let mut w: u16 = 0;
    let mut h: u16 = 0;
    let mut pic8: *mut crate::src::qcommon::q_shared::byte =
        0 as *mut crate::src::qcommon::q_shared::byte;
    let mut palette: *mut crate::src::qcommon::q_shared::byte =
        0 as *mut crate::src::qcommon::q_shared::byte;
    let mut i: i32 = 0;
    let mut size: u32 = 0 as i32 as u32;
    if !width.is_null() {
        *width = 0 as i32
    }
    if !height.is_null() {
        *height = 0 as i32
    }
    *pic = 0 as *mut crate::src::qcommon::q_shared::byte;
    //
    // load the file
    //
    len = crate::src::renderergl1::tr_main::ri
        .FS_ReadFile
        .expect("non-null function pointer")(filename as *mut libc::c_char, &mut raw.v)
        as i32;
    if raw.b.is_null() || len < 0 as i32 {
        return;
    }
    if (len as u32 as libc::c_ulong) < ::std::mem::size_of::<pcx_t>() as libc::c_ulong {
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_ALL as i32,
            b"PCX truncated: %s\n\x00" as *const u8 as *const libc::c_char,
            filename,
        );
        crate::src::renderergl1::tr_main::ri
            .FS_FreeFile
            .expect("non-null function pointer")(raw.v);
        return;
    }
    //
    // parse the PCX file
    //
    pcx = raw.b as *mut pcx_t;
    end = raw.b.offset(len as isize);
    w = ((*pcx).xmax as i32 + 1 as i32) as u16;
    h = ((*pcx).ymax as i32 + 1 as i32) as u16;
    size = (w as i32 * h as i32) as u32;
    if (*pcx).manufacturer as i32 != 0xa as i32
        || (*pcx).version as i32 != 5 as i32
        || (*pcx).encoding as i32 != 1 as i32
        || (*pcx).color_planes as i32 != 1 as i32
        || (*pcx).bits_per_pixel as i32 != 8 as i32
        || w as i32 >= 1024 as i32
        || h as i32 >= 1024 as i32
    {
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_ALL as i32,
            b"Bad or unsupported pcx file %s (%dx%d@%d)\n\x00" as *const u8 as *const libc::c_char,
            filename,
            w as i32,
            h as i32,
            (*pcx).bits_per_pixel as i32,
        );
        return;
    }
    pic8 = crate::src::renderergl1::tr_main::ri
        .Malloc
        .expect("non-null function pointer")(size as i32)
        as *mut crate::src::qcommon::q_shared::byte;
    pix = pic8;
    raw.b = (*pcx).data.as_mut_ptr();
    // FIXME: should use bytes_per_line but original q3 didn't do that either
    while pix < pic8.offset(size as isize) {
        if runLength as i32 > 0 as i32 {
            let fresh0 = pix;
            pix = pix.offset(1);
            *fresh0 = dataByte;
            runLength = runLength.wrapping_sub(1)
        } else {
            if raw.b.offset(1 as i32 as isize) > end {
                break;
            }
            let fresh1 = raw.b;
            raw.b = raw.b.offset(1);
            dataByte = *fresh1;
            if dataByte as i32 & 0xc0 as i32 == 0xc0 as i32 {
                if raw.b.offset(1 as i32 as isize) > end {
                    break;
                }
                runLength = (dataByte as i32 & 0x3f as i32) as u8;
                let fresh2 = raw.b;
                raw.b = raw.b.offset(1);
                dataByte = *fresh2
            } else {
                runLength = 1 as i32 as u8
            }
        }
    }
    if pix < pic8.offset(size as isize) {
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_ALL as i32,
            b"PCX file truncated: %s\n\x00" as *const u8 as *const libc::c_char,
            filename,
        );
        crate::src::renderergl1::tr_main::ri
            .FS_FreeFile
            .expect("non-null function pointer")(pcx as *mut libc::c_void);
        crate::src::renderergl1::tr_main::ri
            .Free
            .expect("non-null function pointer")(pic8 as *mut libc::c_void);
    }
    if raw
        .b
        .offset_from(pcx as *mut crate::src::qcommon::q_shared::byte) as libc::c_long
        >= end.offset_from(769 as i32 as *mut crate::src::qcommon::q_shared::byte)
            as libc::c_long
        || *end.offset(-(769 as i32) as isize) as i32 != 0xc as i32
    {
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_ALL as i32,
            b"PCX missing palette: %s\n\x00" as *const u8 as *const libc::c_char,
            filename,
        );
        crate::src::renderergl1::tr_main::ri
            .FS_FreeFile
            .expect("non-null function pointer")(pcx as *mut libc::c_void);
        crate::src::renderergl1::tr_main::ri
            .Free
            .expect("non-null function pointer")(pic8 as *mut libc::c_void);
        return;
    }
    palette = end.offset(-(768 as i32 as isize));
    out = crate::src::renderergl1::tr_main::ri
        .Malloc
        .expect("non-null function pointer")(
        (4 as i32 as u32).wrapping_mul(size) as i32,
    ) as *mut crate::src::qcommon::q_shared::byte;
    pix = out;
    i = 0 as i32;
    while (i as u32) < size {
        let mut p: u8 = *pic8.offset(i as isize);
        *pix.offset(0 as i32 as isize) =
            *palette.offset((p as i32 * 3 as i32) as isize);
        *pix.offset(1 as i32 as isize) =
            *palette.offset((p as i32 * 3 as i32 + 1 as i32) as isize);
        *pix.offset(2 as i32 as isize) =
            *palette.offset((p as i32 * 3 as i32 + 2 as i32) as isize);
        *pix.offset(3 as i32 as isize) =
            255 as i32 as crate::src::qcommon::q_shared::byte;
        pix = pix.offset(4 as i32 as isize);
        i += 1
    }
    if !width.is_null() {
        *width = w as i32
    }
    if !height.is_null() {
        *height = h as i32
    }
    *pic = out;
    crate::src::renderergl1::tr_main::ri
        .FS_FreeFile
        .expect("non-null function pointer")(pcx as *mut libc::c_void);
    crate::src::renderergl1::tr_main::ri
        .Free
        .expect("non-null function pointer")(pic8 as *mut libc::c_void);
}

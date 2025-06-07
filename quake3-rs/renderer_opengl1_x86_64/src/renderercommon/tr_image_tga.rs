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
pub use crate::src::qcommon::q_shared::ERR_DISCONNECT;
pub use crate::src::qcommon::q_shared::ERR_DROP;
pub use crate::src::qcommon::q_shared::ERR_FATAL;
pub use crate::src::qcommon::q_shared::ERR_NEED_CD;
pub use crate::src::qcommon::q_shared::ERR_SERVERDISCONNECT;
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
pub union C2RustUnnamed_93 {
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
/*
========================================================================

TGA files are used for 24/32 bit images

========================================================================
*/

pub type TargaHeader = _TargaHeader;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _TargaHeader {
    pub id_length: libc::c_uchar,
    pub colormap_type: libc::c_uchar,
    pub image_type: libc::c_uchar,
    pub colormap_index: libc::c_ushort,
    pub colormap_length: libc::c_ushort,
    pub colormap_size: libc::c_uchar,
    pub x_origin: libc::c_ushort,
    pub y_origin: libc::c_ushort,
    pub width: libc::c_ushort,
    pub height: libc::c_ushort,
    pub pixel_size: libc::c_uchar,
    pub attributes: libc::c_uchar,
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

pub unsafe extern "C" fn R_LoadTGA(
    mut name: *const libc::c_char,
    mut pic: *mut *mut crate::src::qcommon::q_shared::byte,
    mut width: *mut libc::c_int,
    mut height: *mut libc::c_int,
) {
    let mut columns: libc::c_uint = 0;
    let mut rows: libc::c_uint = 0;
    let mut numPixels: libc::c_uint = 0;
    let mut pixbuf: *mut crate::src::qcommon::q_shared::byte =
        0 as *mut crate::src::qcommon::q_shared::byte;
    let mut row: libc::c_int = 0;
    let mut column: libc::c_int = 0;
    let mut buf_p: *mut crate::src::qcommon::q_shared::byte =
        0 as *mut crate::src::qcommon::q_shared::byte;
    let mut end: *mut crate::src::qcommon::q_shared::byte =
        0 as *mut crate::src::qcommon::q_shared::byte;
    let mut buffer: C2RustUnnamed_93 = C2RustUnnamed_93 {
        b: 0 as *mut crate::src::qcommon::q_shared::byte,
    };
    let mut targa_header: TargaHeader = TargaHeader {
        id_length: 0,
        colormap_type: 0,
        image_type: 0,
        colormap_index: 0,
        colormap_length: 0,
        colormap_size: 0,
        x_origin: 0,
        y_origin: 0,
        width: 0,
        height: 0,
        pixel_size: 0,
        attributes: 0,
    };
    let mut targa_rgba: *mut crate::src::qcommon::q_shared::byte =
        0 as *mut crate::src::qcommon::q_shared::byte;
    let mut length: libc::c_int = 0;
    *pic = 0 as *mut crate::src::qcommon::q_shared::byte;
    if !width.is_null() {
        *width = 0 as libc::c_int
    }
    if !height.is_null() {
        *height = 0 as libc::c_int
    }
    //
    // load the file
    //
    length = crate::src::renderergl1::tr_main::ri
        .FS_ReadFile
        .expect("non-null function pointer")(name as *mut libc::c_char, &mut buffer.v)
        as libc::c_int;
    if buffer.b.is_null() || length < 0 as libc::c_int {
        return;
    }
    if length < 18 as libc::c_int {
        crate::src::renderergl1::tr_main::ri
            .Error
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
            b"LoadTGA: header too short (%s)\x00" as *const u8 as *const libc::c_char,
            name,
        );
    }
    buf_p = buffer.b;
    end = buffer.b.offset(length as isize);
    targa_header.id_length = *buf_p.offset(0 as libc::c_int as isize);
    targa_header.colormap_type = *buf_p.offset(1 as libc::c_int as isize);
    targa_header.image_type = *buf_p.offset(2 as libc::c_int as isize);
    crate::stdlib::memcpy(
        &mut targa_header.colormap_index as *mut libc::c_ushort as *mut libc::c_void,
        &mut *buf_p.offset(3 as libc::c_int as isize) as *mut crate::src::qcommon::q_shared::byte
            as *const libc::c_void,
        2 as libc::c_int as libc::c_ulong,
    );
    crate::stdlib::memcpy(
        &mut targa_header.colormap_length as *mut libc::c_ushort as *mut libc::c_void,
        &mut *buf_p.offset(5 as libc::c_int as isize) as *mut crate::src::qcommon::q_shared::byte
            as *const libc::c_void,
        2 as libc::c_int as libc::c_ulong,
    );
    targa_header.colormap_size = *buf_p.offset(7 as libc::c_int as isize);
    crate::stdlib::memcpy(
        &mut targa_header.x_origin as *mut libc::c_ushort as *mut libc::c_void,
        &mut *buf_p.offset(8 as libc::c_int as isize) as *mut crate::src::qcommon::q_shared::byte
            as *const libc::c_void,
        2 as libc::c_int as libc::c_ulong,
    );
    crate::stdlib::memcpy(
        &mut targa_header.y_origin as *mut libc::c_ushort as *mut libc::c_void,
        &mut *buf_p.offset(10 as libc::c_int as isize) as *mut crate::src::qcommon::q_shared::byte
            as *const libc::c_void,
        2 as libc::c_int as libc::c_ulong,
    );
    crate::stdlib::memcpy(
        &mut targa_header.width as *mut libc::c_ushort as *mut libc::c_void,
        &mut *buf_p.offset(12 as libc::c_int as isize) as *mut crate::src::qcommon::q_shared::byte
            as *const libc::c_void,
        2 as libc::c_int as libc::c_ulong,
    );
    crate::stdlib::memcpy(
        &mut targa_header.height as *mut libc::c_ushort as *mut libc::c_void,
        &mut *buf_p.offset(14 as libc::c_int as isize) as *mut crate::src::qcommon::q_shared::byte
            as *const libc::c_void,
        2 as libc::c_int as libc::c_ulong,
    );
    targa_header.pixel_size = *buf_p.offset(16 as libc::c_int as isize);
    targa_header.attributes = *buf_p.offset(17 as libc::c_int as isize);
    targa_header.colormap_index = targa_header.colormap_index;
    targa_header.colormap_length = targa_header.colormap_length;
    targa_header.x_origin = targa_header.x_origin;
    targa_header.y_origin = targa_header.y_origin;
    targa_header.width = targa_header.width;
    targa_header.height = targa_header.height;
    buf_p = buf_p.offset(18 as libc::c_int as isize);
    if targa_header.image_type as libc::c_int != 2 as libc::c_int
        && targa_header.image_type as libc::c_int != 10 as libc::c_int
        && targa_header.image_type as libc::c_int != 3 as libc::c_int
    {
        crate::src::renderergl1::tr_main::ri
            .Error
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
            b"LoadTGA: Only type 2 (RGB), 3 (gray), and 10 (RGB) TGA images supported\x00"
                as *const u8 as *const libc::c_char,
        );
    }
    if targa_header.colormap_type as libc::c_int != 0 as libc::c_int {
        crate::src::renderergl1::tr_main::ri
            .Error
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
            b"LoadTGA: colormaps not supported\x00" as *const u8 as *const libc::c_char,
        );
    }
    if targa_header.pixel_size as libc::c_int != 32 as libc::c_int
        && targa_header.pixel_size as libc::c_int != 24 as libc::c_int
        && targa_header.image_type as libc::c_int != 3 as libc::c_int
    {
        crate::src::renderergl1::tr_main::ri
            .Error
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
            b"LoadTGA: Only 32 or 24 bit images supported (no colormaps)\x00" as *const u8
                as *const libc::c_char,
        );
    }
    columns = targa_header.width as libc::c_uint;
    rows = targa_header.height as libc::c_uint;
    numPixels = columns
        .wrapping_mul(rows)
        .wrapping_mul(4 as libc::c_int as libc::c_uint);
    if columns == 0
        || rows == 0
        || numPixels > 0x7fffffff as libc::c_int as libc::c_uint
        || numPixels
            .wrapping_div(columns)
            .wrapping_div(4 as libc::c_int as libc::c_uint)
            != rows
    {
        crate::src::renderergl1::tr_main::ri
            .Error
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
            b"LoadTGA: %s has an invalid image size\x00" as *const u8 as *const libc::c_char,
            name,
        );
    }
    targa_rgba = crate::src::renderergl1::tr_main::ri
        .Malloc
        .expect("non-null function pointer")(numPixels as libc::c_int)
        as *mut crate::src::qcommon::q_shared::byte;
    if targa_header.id_length as libc::c_int != 0 as libc::c_int {
        if buf_p.offset(targa_header.id_length as libc::c_int as isize) > end {
            crate::src::renderergl1::tr_main::ri
                .Error
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
                b"LoadTGA: header too short (%s)\x00" as *const u8 as *const libc::c_char,
                name,
            );
        }
        buf_p = buf_p.offset(targa_header.id_length as libc::c_int as isize)
        // skip TARGA image comment
    }
    if targa_header.image_type as libc::c_int == 2 as libc::c_int
        || targa_header.image_type as libc::c_int == 3 as libc::c_int
    {
        if buf_p.offset(
            columns
                .wrapping_mul(rows)
                .wrapping_mul(targa_header.pixel_size as libc::c_uint)
                .wrapping_div(8 as libc::c_int as libc::c_uint) as isize,
        ) > end
        {
            crate::src::renderergl1::tr_main::ri
                .Error
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
                b"LoadTGA: file truncated (%s)\x00" as *const u8 as *const libc::c_char,
                name,
            );
        }
        // Uncompressed RGB or gray scale image
        row = rows.wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_int;
        while row >= 0 as libc::c_int {
            pixbuf = targa_rgba.offset(
                (row as libc::c_uint)
                    .wrapping_mul(columns)
                    .wrapping_mul(4 as libc::c_int as libc::c_uint) as isize,
            );
            column = 0 as libc::c_int;
            while (column as libc::c_uint) < columns {
                let mut red: libc::c_uchar = 0;
                let mut green: libc::c_uchar = 0;
                let mut blue: libc::c_uchar = 0;
                let mut alphabyte: libc::c_uchar = 0;
                match targa_header.pixel_size as libc::c_int {
                    8 => {
                        let fresh0 = buf_p;
                        buf_p = buf_p.offset(1);
                        blue = *fresh0;
                        green = blue;
                        red = blue;
                        let fresh1 = pixbuf;
                        pixbuf = pixbuf.offset(1);
                        *fresh1 = red;
                        let fresh2 = pixbuf;
                        pixbuf = pixbuf.offset(1);
                        *fresh2 = green;
                        let fresh3 = pixbuf;
                        pixbuf = pixbuf.offset(1);
                        *fresh3 = blue;
                        let fresh4 = pixbuf;
                        pixbuf = pixbuf.offset(1);
                        *fresh4 = 255 as libc::c_int as crate::src::qcommon::q_shared::byte
                    }
                    24 => {
                        let fresh5 = buf_p;
                        buf_p = buf_p.offset(1);
                        blue = *fresh5;
                        let fresh6 = buf_p;
                        buf_p = buf_p.offset(1);
                        green = *fresh6;
                        let fresh7 = buf_p;
                        buf_p = buf_p.offset(1);
                        red = *fresh7;
                        let fresh8 = pixbuf;
                        pixbuf = pixbuf.offset(1);
                        *fresh8 = red;
                        let fresh9 = pixbuf;
                        pixbuf = pixbuf.offset(1);
                        *fresh9 = green;
                        let fresh10 = pixbuf;
                        pixbuf = pixbuf.offset(1);
                        *fresh10 = blue;
                        let fresh11 = pixbuf;
                        pixbuf = pixbuf.offset(1);
                        *fresh11 = 255 as libc::c_int as crate::src::qcommon::q_shared::byte
                    }
                    32 => {
                        let fresh12 = buf_p;
                        buf_p = buf_p.offset(1);
                        blue = *fresh12;
                        let fresh13 = buf_p;
                        buf_p = buf_p.offset(1);
                        green = *fresh13;
                        let fresh14 = buf_p;
                        buf_p = buf_p.offset(1);
                        red = *fresh14;
                        let fresh15 = buf_p;
                        buf_p = buf_p.offset(1);
                        alphabyte = *fresh15;
                        let fresh16 = pixbuf;
                        pixbuf = pixbuf.offset(1);
                        *fresh16 = red;
                        let fresh17 = pixbuf;
                        pixbuf = pixbuf.offset(1);
                        *fresh17 = green;
                        let fresh18 = pixbuf;
                        pixbuf = pixbuf.offset(1);
                        *fresh18 = blue;
                        let fresh19 = pixbuf;
                        pixbuf = pixbuf.offset(1);
                        *fresh19 = alphabyte
                    }
                    _ => {
                        crate::src::renderergl1::tr_main::ri
                            .Error
                            .expect("non-null function pointer")(
                            crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
                            b"LoadTGA: illegal pixel_size \'%d\' in file \'%s\'\x00" as *const u8
                                as *const libc::c_char,
                            targa_header.pixel_size as libc::c_int,
                            name,
                        );
                    }
                }
                column += 1
            }
            row -= 1
        }
    } else if targa_header.image_type as libc::c_int == 10 as libc::c_int {
        // Runlength encoded RGB images
        let mut red_0: libc::c_uchar = 0;
        let mut green_0: libc::c_uchar = 0;
        let mut blue_0: libc::c_uchar = 0;
        let mut alphabyte_0: libc::c_uchar = 0;
        let mut packetHeader: libc::c_uchar = 0;
        let mut packetSize: libc::c_uchar = 0;
        let mut j: libc::c_uchar = 0;
        row = rows.wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_int;
        while row >= 0 as libc::c_int {
            pixbuf = targa_rgba.offset(
                (row as libc::c_uint)
                    .wrapping_mul(columns)
                    .wrapping_mul(4 as libc::c_int as libc::c_uint) as isize,
            );
            column = 0 as libc::c_int;
            's_458: while (column as libc::c_uint) < columns {
                if buf_p.offset(1 as libc::c_int as isize) > end {
                    crate::src::renderergl1::tr_main::ri
                        .Error
                        .expect("non-null function pointer")(
                        crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
                        b"LoadTGA: file truncated (%s)\x00" as *const u8 as *const libc::c_char,
                        name,
                    );
                }
                let fresh20 = buf_p;
                buf_p = buf_p.offset(1);
                packetHeader = *fresh20;
                packetSize = (1 as libc::c_int
                    + (packetHeader as libc::c_int & 0x7f as libc::c_int))
                    as libc::c_uchar;
                if packetHeader as libc::c_int & 0x80 as libc::c_int != 0 {
                    // run-length packet
                    if buf_p.offset(
                        (targa_header.pixel_size as libc::c_int / 8 as libc::c_int) as isize,
                    ) > end
                    {
                        crate::src::renderergl1::tr_main::ri
                            .Error
                            .expect("non-null function pointer")(
                            crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
                            b"LoadTGA: file truncated (%s)\x00" as *const u8 as *const libc::c_char,
                            name,
                        );
                    }
                    match targa_header.pixel_size as libc::c_int {
                        24 => {
                            let fresh21 = buf_p;
                            buf_p = buf_p.offset(1);
                            blue_0 = *fresh21;
                            let fresh22 = buf_p;
                            buf_p = buf_p.offset(1);
                            green_0 = *fresh22;
                            let fresh23 = buf_p;
                            buf_p = buf_p.offset(1);
                            red_0 = *fresh23;
                            alphabyte_0 = 255 as libc::c_int as libc::c_uchar
                        }
                        32 => {
                            let fresh24 = buf_p;
                            buf_p = buf_p.offset(1);
                            blue_0 = *fresh24;
                            let fresh25 = buf_p;
                            buf_p = buf_p.offset(1);
                            green_0 = *fresh25;
                            let fresh26 = buf_p;
                            buf_p = buf_p.offset(1);
                            red_0 = *fresh26;
                            let fresh27 = buf_p;
                            buf_p = buf_p.offset(1);
                            alphabyte_0 = *fresh27
                        }
                        _ => {
                            crate::src::renderergl1::tr_main::ri
                                .Error
                                .expect("non-null function pointer")(
                                crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
                                b"LoadTGA: illegal pixel_size \'%d\' in file \'%s\'\x00"
                                    as *const u8
                                    as *const libc::c_char,
                                targa_header.pixel_size as libc::c_int,
                                name,
                            );
                        }
                    }
                    j = 0 as libc::c_int as libc::c_uchar;
                    while (j as libc::c_int) < packetSize as libc::c_int {
                        let fresh28 = pixbuf;
                        pixbuf = pixbuf.offset(1);
                        *fresh28 = red_0;
                        let fresh29 = pixbuf;
                        pixbuf = pixbuf.offset(1);
                        *fresh29 = green_0;
                        let fresh30 = pixbuf;
                        pixbuf = pixbuf.offset(1);
                        *fresh30 = blue_0;
                        let fresh31 = pixbuf;
                        pixbuf = pixbuf.offset(1);
                        *fresh31 = alphabyte_0;
                        column += 1;
                        if column as libc::c_uint == columns {
                            // run spans across rows
                            column = 0 as libc::c_int;
                            if !(row > 0 as libc::c_int) {
                                break 's_458;
                            }
                            row -= 1;
                            pixbuf = targa_rgba.offset(
                                (row as libc::c_uint)
                                    .wrapping_mul(columns)
                                    .wrapping_mul(4 as libc::c_int as libc::c_uint)
                                    as isize,
                            )
                        }
                        j = j.wrapping_add(1)
                    }
                } else {
                    // non run-length packet
                    if buf_p.offset(
                        (targa_header.pixel_size as libc::c_int / 8 as libc::c_int
                            * packetSize as libc::c_int) as isize,
                    ) > end
                    {
                        crate::src::renderergl1::tr_main::ri
                            .Error
                            .expect("non-null function pointer")(
                            crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
                            b"LoadTGA: file truncated (%s)\x00" as *const u8 as *const libc::c_char,
                            name,
                        );
                    }
                    j = 0 as libc::c_int as libc::c_uchar;
                    while (j as libc::c_int) < packetSize as libc::c_int {
                        match targa_header.pixel_size as libc::c_int {
                            24 => {
                                let fresh32 = buf_p;
                                buf_p = buf_p.offset(1);
                                blue_0 = *fresh32;
                                let fresh33 = buf_p;
                                buf_p = buf_p.offset(1);
                                green_0 = *fresh33;
                                let fresh34 = buf_p;
                                buf_p = buf_p.offset(1);
                                red_0 = *fresh34;
                                let fresh35 = pixbuf;
                                pixbuf = pixbuf.offset(1);
                                *fresh35 = red_0;
                                let fresh36 = pixbuf;
                                pixbuf = pixbuf.offset(1);
                                *fresh36 = green_0;
                                let fresh37 = pixbuf;
                                pixbuf = pixbuf.offset(1);
                                *fresh37 = blue_0;
                                let fresh38 = pixbuf;
                                pixbuf = pixbuf.offset(1);
                                *fresh38 = 255 as libc::c_int as crate::src::qcommon::q_shared::byte
                            }
                            32 => {
                                let fresh39 = buf_p;
                                buf_p = buf_p.offset(1);
                                blue_0 = *fresh39;
                                let fresh40 = buf_p;
                                buf_p = buf_p.offset(1);
                                green_0 = *fresh40;
                                let fresh41 = buf_p;
                                buf_p = buf_p.offset(1);
                                red_0 = *fresh41;
                                let fresh42 = buf_p;
                                buf_p = buf_p.offset(1);
                                alphabyte_0 = *fresh42;
                                let fresh43 = pixbuf;
                                pixbuf = pixbuf.offset(1);
                                *fresh43 = red_0;
                                let fresh44 = pixbuf;
                                pixbuf = pixbuf.offset(1);
                                *fresh44 = green_0;
                                let fresh45 = pixbuf;
                                pixbuf = pixbuf.offset(1);
                                *fresh45 = blue_0;
                                let fresh46 = pixbuf;
                                pixbuf = pixbuf.offset(1);
                                *fresh46 = alphabyte_0
                            }
                            _ => {
                                crate::src::renderergl1::tr_main::ri
                                    .Error
                                    .expect("non-null function pointer")(
                                    crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
                                    b"LoadTGA: illegal pixel_size \'%d\' in file \'%s\'\x00"
                                        as *const u8
                                        as *const libc::c_char,
                                    targa_header.pixel_size as libc::c_int,
                                    name,
                                );
                            }
                        }
                        column += 1;
                        if column as libc::c_uint == columns {
                            // pixel packet run spans across rows
                            column = 0 as libc::c_int;
                            if !(row > 0 as libc::c_int) {
                                break 's_458;
                            }
                            row -= 1;
                            pixbuf = targa_rgba.offset(
                                (row as libc::c_uint)
                                    .wrapping_mul(columns)
                                    .wrapping_mul(4 as libc::c_int as libc::c_uint)
                                    as isize,
                            )
                        }
                        j = j.wrapping_add(1)
                    }
                }
            }
            row -= 1
        }
    }
    // instead we just print a warning
    if targa_header.attributes as libc::c_int & 0x20 as libc::c_int != 0 {
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_WARNING as libc::c_int,
            b"WARNING: \'%s\' TGA file header declares top-down image, ignoring\n\x00" as *const u8
                as *const libc::c_char,
            name,
        );
    }
    if !width.is_null() {
        *width = columns as libc::c_int
    }
    if !height.is_null() {
        *height = rows as libc::c_int
    }
    *pic = targa_rgba;
    crate::src::renderergl1::tr_main::ri
        .FS_FreeFile
        .expect("non-null function pointer")(buffer.v);
}

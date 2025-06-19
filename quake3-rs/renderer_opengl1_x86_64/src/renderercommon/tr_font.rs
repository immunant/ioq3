use ::libc;

pub use crate::src::jpeg_8c::jerror::C2RustUnnamed_1;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::cvar_s;
pub use crate::src::qcommon::q_shared::cvar_t;
pub use crate::src::qcommon::q_shared::e_status;
pub use crate::src::qcommon::q_shared::fontInfo_t;
pub use crate::src::qcommon::q_shared::glyphInfo_t;
pub use crate::src::qcommon::q_shared::h_dontcare;
pub use crate::src::qcommon::q_shared::h_high;
pub use crate::src::qcommon::q_shared::h_low;
pub use crate::src::qcommon::q_shared::ha_pref;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qhandle_t;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::Com_sprintf;
pub use crate::src::qcommon::q_shared::Q_stricmp;
pub use crate::src::qcommon::q_shared::Q_strncpyz;
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
pub union poor {
    pub fred: [crate::src::qcommon::q_shared::byte; 4],
    pub ffred: f32,
}

static mut registeredFontCount: i32 = 0 as i32;

static mut registeredFont: [crate::src::qcommon::q_shared::fontInfo_t; 6] =
    [crate::src::qcommon::q_shared::fontInfo_t {
        glyphs: [crate::src::qcommon::q_shared::glyphInfo_t {
            height: 0,
            top: 0,
            bottom: 0,
            pitch: 0,
            xSkip: 0,
            imageWidth: 0,
            imageHeight: 0,
            s: 0.,
            t: 0.,
            s2: 0.,
            t2: 0.,
            glyph: 0,
            shaderName: [0; 32],
        }; 256],
        glyphScale: 0.,
        name: [0; 64],
    }; 6];

static mut fdOffset: i32 = 0;

static mut fdFile: *mut crate::src::qcommon::q_shared::byte =
    0 as *const crate::src::qcommon::q_shared::byte as *mut crate::src::qcommon::q_shared::byte;
#[no_mangle]

pub unsafe extern "C" fn readInt() -> i32 {
    let mut i: i32 = (*fdFile.offset(fdOffset as isize) as u32
        | (*fdFile.offset((fdOffset + 1 as i32) as isize) as u32) << 8 as i32
        | (*fdFile.offset((fdOffset + 2 as i32) as isize) as u32) << 16 as i32
        | (*fdFile.offset((fdOffset + 3 as i32) as isize) as u32) << 24 as i32)
        as i32;
    fdOffset += 4 as i32;
    return i;
}
#[no_mangle]

pub unsafe extern "C" fn readFloat() -> f32 {
    let mut me: poor = poor { fred: [0; 4] };
    me.fred[0 as i32 as usize] = *fdFile.offset((fdOffset + 0 as i32) as isize);
    me.fred[1 as i32 as usize] = *fdFile.offset((fdOffset + 1 as i32) as isize);
    me.fred[2 as i32 as usize] = *fdFile.offset((fdOffset + 2 as i32) as isize);
    me.fred[3 as i32 as usize] = *fdFile.offset((fdOffset + 3 as i32) as isize);
    fdOffset += 4 as i32;
    return me.ffred;
}
#[no_mangle]

pub unsafe extern "C" fn RE_RegisterFont(
    mut fontName: *const libc::c_char,
    mut pointSize: i32,
    mut font: *mut crate::src::qcommon::q_shared::fontInfo_t,
) {
    let mut faceData: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut i: i32 = 0;
    let mut len: i32 = 0;
    let mut name: [libc::c_char; 1024] = [0; 1024];
    if fontName.is_null() {
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_ALL as i32,
            b"RE_RegisterFont: called with empty name\n\x00" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if pointSize <= 0 as i32 {
        pointSize = 12 as i32
    }
    crate::src::renderergl1::tr_cmds::R_IssuePendingRenderCommands();
    if registeredFontCount >= 6 as i32 {
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_WARNING as i32,
            b"RE_RegisterFont: Too many fonts registered already.\n\x00" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    crate::src::qcommon::q_shared::Com_sprintf(
        name.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as i32,
        b"fonts/fontImage_%i.dat\x00" as *const u8 as *const libc::c_char,
        pointSize,
    );
    i = 0 as i32;
    while i < registeredFontCount {
        if crate::src::qcommon::q_shared::Q_stricmp(
            name.as_mut_ptr(),
            registeredFont[i as usize].name.as_mut_ptr(),
        ) == 0 as i32
        {
            crate::stdlib::memcpy(
                font as *mut libc::c_void,
                &mut *registeredFont.as_mut_ptr().offset(i as isize)
                    as *mut crate::src::qcommon::q_shared::fontInfo_t
                    as *const libc::c_void,
                ::std::mem::size_of::<crate::src::qcommon::q_shared::fontInfo_t>() as libc::c_ulong,
            );
            return;
        }
        i += 1
    }
    len = crate::src::renderergl1::tr_main::ri
        .FS_ReadFile
        .expect("non-null function pointer")(
        name.as_mut_ptr(), 0 as *mut *mut libc::c_void
    ) as i32;
    if len as libc::c_ulong
        == ::std::mem::size_of::<crate::src::qcommon::q_shared::fontInfo_t>() as libc::c_ulong
    {
        crate::src::renderergl1::tr_main::ri
            .FS_ReadFile
            .expect("non-null function pointer")(name.as_mut_ptr(), &mut faceData);
        fdOffset = 0 as i32;
        fdFile = faceData as *mut crate::src::qcommon::q_shared::byte;
        i = 0 as i32;
        while i < 255 as i32 - 0 as i32 + 1 as i32 {
            (*font).glyphs[i as usize].height = readInt();
            (*font).glyphs[i as usize].top = readInt();
            (*font).glyphs[i as usize].bottom = readInt();
            (*font).glyphs[i as usize].pitch = readInt();
            (*font).glyphs[i as usize].xSkip = readInt();
            (*font).glyphs[i as usize].imageWidth = readInt();
            (*font).glyphs[i as usize].imageHeight = readInt();
            (*font).glyphs[i as usize].s = readFloat();
            (*font).glyphs[i as usize].t = readFloat();
            (*font).glyphs[i as usize].s2 = readFloat();
            (*font).glyphs[i as usize].t2 = readFloat();
            (*font).glyphs[i as usize].glyph = readInt();
            crate::src::qcommon::q_shared::Q_strncpyz(
                (*font).glyphs[i as usize].shaderName.as_mut_ptr(),
                &mut *fdFile.offset(fdOffset as isize) as *mut crate::src::qcommon::q_shared::byte
                    as *const libc::c_char,
                ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong as i32,
            );
            fdOffset = (fdOffset as libc::c_ulong)
                .wrapping_add(::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong)
                as i32;
            i += 1
        }
        (*font).glyphScale = readFloat();
        crate::stdlib::memcpy(
            (*font).name.as_mut_ptr() as *mut libc::c_void,
            &mut *fdFile.offset(fdOffset as isize) as *mut crate::src::qcommon::q_shared::byte
                as *const libc::c_void,
            64 as i32 as libc::c_ulong,
        );
        //		Com_Memcpy(font, faceData, sizeof(fontInfo_t));
        crate::src::qcommon::q_shared::Q_strncpyz(
            (*font).name.as_mut_ptr(),
            name.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as i32,
        );
        i = 0 as i32;
        while i <= 255 as i32 {
            (*font).glyphs[i as usize].glyph =
                crate::src::renderergl1::tr_shader::RE_RegisterShaderNoMip(
                    (*font).glyphs[i as usize].shaderName.as_mut_ptr(),
                );
            i += 1
        }
        let fresh0 = registeredFontCount;
        registeredFontCount = registeredFontCount + 1;
        crate::stdlib::memcpy(
            &mut *registeredFont.as_mut_ptr().offset(fresh0 as isize)
                as *mut crate::src::qcommon::q_shared::fontInfo_t as *mut libc::c_void,
            font as *const libc::c_void,
            ::std::mem::size_of::<crate::src::qcommon::q_shared::fontInfo_t>() as libc::c_ulong,
        );
        crate::src::renderergl1::tr_main::ri
            .FS_FreeFile
            .expect("non-null function pointer")(faceData);
        return;
    }
    crate::src::renderergl1::tr_main::ri
        .Printf
        .expect("non-null function pointer")(
        crate::src::qcommon::q_shared::PRINT_WARNING as i32,
        b"RE_RegisterFont: FreeType code not available\n\x00" as *const u8 as *const libc::c_char,
    );
}
#[no_mangle]

pub unsafe extern "C" fn R_InitFreeType() {
    registeredFontCount = 0 as i32;
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
#[no_mangle]

pub unsafe extern "C" fn R_DoneFreeType() {
    registeredFontCount = 0 as i32;
}

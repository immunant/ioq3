use ::libc;

pub use crate::stdlib::__uint16_t;
pub use crate::stdlib::uint16_t;
pub use crate::stdlib::Uint16;

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
extern "C" {
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
    #[no_mangle]
    pub static mut SDL_window: *mut crate::stdlib::SDL_Window;
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
=================
GLimp_SetGamma
=================
*/
#[no_mangle]

pub unsafe extern "C" fn GLimp_SetGamma(
    mut red: *mut libc::c_uchar,
    mut green: *mut libc::c_uchar,
    mut blue: *mut libc::c_uchar,
) {
    let mut table: [[crate::stdlib::Uint16; 256]; 3] = [[0; 256]; 3];
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    if crate::src::renderergl1::tr_init::glConfig.deviceSupportsGamma as u64 == 0
        || (*crate::src::renderergl1::tr_init::r_ignorehwgamma).integer > 0 as libc::c_int
    {
        return;
    }
    i = 0 as libc::c_int;
    while i < 256 as libc::c_int {
        table[0 as libc::c_int as usize][i as usize] =
            ((*red.offset(i as isize) as crate::stdlib::Uint16 as libc::c_int) << 8 as libc::c_int
                | *red.offset(i as isize) as libc::c_int) as crate::stdlib::Uint16;
        table[1 as libc::c_int as usize][i as usize] =
            ((*green.offset(i as isize) as crate::stdlib::Uint16 as libc::c_int)
                << 8 as libc::c_int
                | *green.offset(i as isize) as libc::c_int) as crate::stdlib::Uint16;
        table[2 as libc::c_int as usize][i as usize] =
            ((*blue.offset(i as isize) as crate::stdlib::Uint16 as libc::c_int) << 8 as libc::c_int
                | *blue.offset(i as isize) as libc::c_int) as crate::stdlib::Uint16;
        i += 1
    }
    // enforce constantly increasing
    j = 0 as libc::c_int;
    while j < 3 as libc::c_int {
        i = 1 as libc::c_int;
        while i < 256 as libc::c_int {
            if (table[j as usize][i as usize] as libc::c_int)
                < table[j as usize][(i - 1 as libc::c_int) as usize] as libc::c_int
            {
                table[j as usize][i as usize] = table[j as usize][(i - 1 as libc::c_int) as usize]
            }
            i += 1
        }
        j += 1
    }
    if crate::stdlib::SDL_SetWindowGammaRamp(
        SDL_window,
        table[0 as libc::c_int as usize].as_mut_ptr(),
        table[1 as libc::c_int as usize].as_mut_ptr(),
        table[2 as libc::c_int as usize].as_mut_ptr(),
    ) < 0 as libc::c_int
    {
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_DEVELOPER as libc::c_int,
            b"SDL_SetWindowGammaRamp() failed: %s\n\x00" as *const u8 as *const libc::c_char,
            crate::stdlib::SDL_GetError(),
        );
    };
}

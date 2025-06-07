use ::libc;

static mut s_noise_table: [libc::c_float; 256] = [0.; 256];

static mut s_noise_perm: [libc::c_int; 256] = [0; 256];

unsafe extern "C" fn GetNoiseValue(
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut z: libc::c_int,
    mut t: libc::c_int,
) -> libc::c_float {
    let mut index: libc::c_int = s_noise_perm[(x + s_noise_perm[(y + s_noise_perm[(z + s_noise_perm
        [(t & 256 as libc::c_int - 1 as libc::c_int) as usize]
        & 256 as libc::c_int - 1 as libc::c_int)
        as usize]
        & 256 as libc::c_int - 1 as libc::c_int)
        as usize]
        & 256 as libc::c_int - 1 as libc::c_int)
        as usize];
    return s_noise_table[index as usize];
}
#[no_mangle]

pub unsafe extern "C" fn R_NoiseInit() {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 256 as libc::c_int {
        s_noise_table[i as usize] = ((::libc::rand() as libc::c_float
            / 2147483647 as libc::c_int as libc::c_float)
            as libc::c_double
            * 2.0f64
            - 1.0f64) as libc::c_float;
        s_noise_perm[i as usize] =
            (::libc::rand() as libc::c_float / 2147483647 as libc::c_int as libc::c_float
                * 255 as libc::c_int as libc::c_float) as libc::c_uchar as libc::c_int;
        i += 1
    }
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
#[no_mangle]

pub unsafe extern "C" fn R_NoiseGet4f(
    mut x: libc::c_float,
    mut y: libc::c_float,
    mut z: libc::c_float,
    mut t: libc::c_double,
) -> libc::c_float {
    let mut i: libc::c_int = 0;
    let mut ix: libc::c_int = 0;
    let mut iy: libc::c_int = 0;
    let mut iz: libc::c_int = 0;
    let mut it: libc::c_int = 0;
    let mut fx: libc::c_float = 0.;
    let mut fy: libc::c_float = 0.;
    let mut fz: libc::c_float = 0.;
    let mut ft: libc::c_float = 0.;
    let mut front: [libc::c_float; 4] = [0.; 4];
    let mut back: [libc::c_float; 4] = [0.; 4];
    let mut fvalue: libc::c_float = 0.;
    let mut bvalue: libc::c_float = 0.;
    let mut value: [libc::c_float; 2] = [0.; 2];
    let mut finalvalue: libc::c_float = 0.;
    ix = crate::stdlib::floor(x as libc::c_double) as libc::c_int;
    fx = x - ix as libc::c_float;
    iy = crate::stdlib::floor(y as libc::c_double) as libc::c_int;
    fy = y - iy as libc::c_float;
    iz = crate::stdlib::floor(z as libc::c_double) as libc::c_int;
    fz = z - iz as libc::c_float;
    it = crate::stdlib::floor(t) as libc::c_int;
    ft = (t - it as libc::c_double) as libc::c_float;
    i = 0 as libc::c_int;
    while i < 2 as libc::c_int {
        front[0 as libc::c_int as usize] = GetNoiseValue(ix, iy, iz, it + i);
        front[1 as libc::c_int as usize] = GetNoiseValue(ix + 1 as libc::c_int, iy, iz, it + i);
        front[2 as libc::c_int as usize] = GetNoiseValue(ix, iy + 1 as libc::c_int, iz, it + i);
        front[3 as libc::c_int as usize] =
            GetNoiseValue(ix + 1 as libc::c_int, iy + 1 as libc::c_int, iz, it + i);
        back[0 as libc::c_int as usize] = GetNoiseValue(ix, iy, iz + 1 as libc::c_int, it + i);
        back[1 as libc::c_int as usize] =
            GetNoiseValue(ix + 1 as libc::c_int, iy, iz + 1 as libc::c_int, it + i);
        back[2 as libc::c_int as usize] =
            GetNoiseValue(ix, iy + 1 as libc::c_int, iz + 1 as libc::c_int, it + i);
        back[3 as libc::c_int as usize] = GetNoiseValue(
            ix + 1 as libc::c_int,
            iy + 1 as libc::c_int,
            iz + 1 as libc::c_int,
            it + i,
        );
        fvalue = (front[0 as libc::c_int as usize] * (1.0f32 - fx)
            + front[1 as libc::c_int as usize] * fx)
            * (1.0f32 - fy)
            + (front[2 as libc::c_int as usize] * (1.0f32 - fx)
                + front[3 as libc::c_int as usize] * fx)
                * fy;
        bvalue = (back[0 as libc::c_int as usize] * (1.0f32 - fx)
            + back[1 as libc::c_int as usize] * fx)
            * (1.0f32 - fy)
            + (back[2 as libc::c_int as usize] * (1.0f32 - fx)
                + back[3 as libc::c_int as usize] * fx)
                * fy;
        value[i as usize] = fvalue * (1.0f32 - fz) + bvalue * fz;
        i += 1
    }
    finalvalue =
        value[0 as libc::c_int as usize] * (1.0f32 - ft) + value[1 as libc::c_int as usize] * ft;
    return finalvalue;
}

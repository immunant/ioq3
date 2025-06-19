use ::libc;

static mut s_noise_table: [f32; 256] = [0.; 256];

static mut s_noise_perm: [i32; 256] = [0; 256];

unsafe extern "C" fn GetNoiseValue(mut x: i32, mut y: i32, mut z: i32, mut t: i32) -> f32 {
    let mut index: i32 = s_noise_perm[(x + s_noise_perm[(y + s_noise_perm
        [(z + s_noise_perm[(t & 256 as i32 - 1 as i32) as usize] & 256 as i32 - 1 as i32) as usize]
        & 256 as i32 - 1 as i32) as usize]
        & 256 as i32 - 1 as i32) as usize];
    return s_noise_table[index as usize];
}
#[no_mangle]

pub unsafe extern "C" fn R_NoiseInit() {
    let mut i: i32 = 0;
    i = 0 as i32;
    while i < 256 as i32 {
        s_noise_table[i as usize] =
            ((::libc::rand() as f32 / 2147483647 as i32 as f32) as f64 * 2.0f64 - 1.0f64) as f32;
        s_noise_perm[i as usize] =
            (::libc::rand() as f32 / 2147483647 as i32 as f32 * 255 as i32 as f32) as u8 as i32;
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

pub unsafe extern "C" fn R_NoiseGet4f(mut x: f32, mut y: f32, mut z: f32, mut t: f64) -> f32 {
    let mut i: i32 = 0;
    let mut ix: i32 = 0;
    let mut iy: i32 = 0;
    let mut iz: i32 = 0;
    let mut it: i32 = 0;
    let mut fx: f32 = 0.;
    let mut fy: f32 = 0.;
    let mut fz: f32 = 0.;
    let mut ft: f32 = 0.;
    let mut front: [f32; 4] = [0.; 4];
    let mut back: [f32; 4] = [0.; 4];
    let mut fvalue: f32 = 0.;
    let mut bvalue: f32 = 0.;
    let mut value: [f32; 2] = [0.; 2];
    let mut finalvalue: f32 = 0.;
    ix = crate::stdlib::floor(x as f64) as i32;
    fx = x - ix as f32;
    iy = crate::stdlib::floor(y as f64) as i32;
    fy = y - iy as f32;
    iz = crate::stdlib::floor(z as f64) as i32;
    fz = z - iz as f32;
    it = crate::stdlib::floor(t) as i32;
    ft = (t - it as f64) as f32;
    i = 0 as i32;
    while i < 2 as i32 {
        front[0 as i32 as usize] = GetNoiseValue(ix, iy, iz, it + i);
        front[1 as i32 as usize] = GetNoiseValue(ix + 1 as i32, iy, iz, it + i);
        front[2 as i32 as usize] = GetNoiseValue(ix, iy + 1 as i32, iz, it + i);
        front[3 as i32 as usize] = GetNoiseValue(ix + 1 as i32, iy + 1 as i32, iz, it + i);
        back[0 as i32 as usize] = GetNoiseValue(ix, iy, iz + 1 as i32, it + i);
        back[1 as i32 as usize] = GetNoiseValue(ix + 1 as i32, iy, iz + 1 as i32, it + i);
        back[2 as i32 as usize] = GetNoiseValue(ix, iy + 1 as i32, iz + 1 as i32, it + i);
        back[3 as i32 as usize] =
            GetNoiseValue(ix + 1 as i32, iy + 1 as i32, iz + 1 as i32, it + i);
        fvalue = (front[0 as i32 as usize] * (1.0f32 - fx) + front[1 as i32 as usize] * fx)
            * (1.0f32 - fy)
            + (front[2 as i32 as usize] * (1.0f32 - fx) + front[3 as i32 as usize] * fx) * fy;
        bvalue = (back[0 as i32 as usize] * (1.0f32 - fx) + back[1 as i32 as usize] * fx)
            * (1.0f32 - fy)
            + (back[2 as i32 as usize] * (1.0f32 - fx) + back[3 as i32 as usize] * fx) * fy;
        value[i as usize] = fvalue * (1.0f32 - fz) + bvalue * fz;
        i += 1
    }
    finalvalue = value[0 as i32 as usize] * (1.0f32 - ft) + value[1 as i32 as usize] * ft;
    return finalvalue;
}

use ::libc;

pub mod q_shared_h {

    #[inline]

    pub unsafe extern "C" fn CrossProduct(
        mut v1: *const crate::src::qcommon::q_shared::vec_t,
        mut v2: *const crate::src::qcommon::q_shared::vec_t,
        mut cross: *mut crate::src::qcommon::q_shared::vec_t,
    ) {
        *cross.offset(0 as libc::c_int as isize) = *v1.offset(1 as libc::c_int as isize)
            * *v2.offset(2 as libc::c_int as isize)
            - *v1.offset(2 as libc::c_int as isize) * *v2.offset(1 as libc::c_int as isize);
        *cross.offset(1 as libc::c_int as isize) = *v1.offset(2 as libc::c_int as isize)
            * *v2.offset(0 as libc::c_int as isize)
            - *v1.offset(0 as libc::c_int as isize) * *v2.offset(2 as libc::c_int as isize);
        *cross.offset(2 as libc::c_int as isize) = *v1.offset(0 as libc::c_int as isize)
            * *v2.offset(1 as libc::c_int as isize)
            - *v1.offset(1 as libc::c_int as isize) * *v2.offset(0 as libc::c_int as isize);
    }
    #[inline]

    pub unsafe extern "C" fn VectorLength(
        mut v: *const crate::src::qcommon::q_shared::vec_t,
    ) -> crate::src::qcommon::q_shared::vec_t {
        return crate::stdlib::sqrt(
            (*v.offset(0 as libc::c_int as isize) * *v.offset(0 as libc::c_int as isize)
                + *v.offset(1 as libc::c_int as isize) * *v.offset(1 as libc::c_int as isize)
                + *v.offset(2 as libc::c_int as isize) * *v.offset(2 as libc::c_int as isize))
                as libc::c_double,
        ) as crate::src::qcommon::q_shared::vec_t;
    }

    // __Q_SHARED_H
}

pub use crate::src::qcommon::q_math::q_shared_h::CrossProduct;
pub use crate::src::qcommon::q_math::q_shared_h::VectorLength;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::cplane_s;
pub use crate::src::qcommon::q_shared::cplane_t;
pub use crate::src::qcommon::q_shared::floatint_t;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::vec3_t;
pub use crate::src::qcommon::q_shared::vec4_t;
pub use crate::src::qcommon::q_shared::vec_t;

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
//
// q_math.c -- stateless support routines that are included in each code module
// Some of the vector functions are static inline in q_shared.h. q3asm
// doesn't understand static functions though, so we only want them in
// one file. That's what this is about.
#[no_mangle]

pub static mut vec3_origin: crate::src::qcommon::q_shared::vec3_t = [
    0 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
    0 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
    0 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
];
#[no_mangle]

pub static mut axisDefault: [crate::src::qcommon::q_shared::vec3_t; 3] = [
    [
        1 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
        0 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
        0 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
    ],
    [
        0 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
        1 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
        0 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
    ],
    [
        0 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
        0 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
        1 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
    ],
];
#[no_mangle]

pub static mut colorBlack: crate::src::qcommon::q_shared::vec4_t = [
    0 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
    0 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
    0 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
    1 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
];
#[no_mangle]

pub static mut colorRed: crate::src::qcommon::q_shared::vec4_t = [
    1 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
    0 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
    0 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
    1 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
];
#[no_mangle]

pub static mut colorGreen: crate::src::qcommon::q_shared::vec4_t = [
    0 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
    1 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
    0 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
    1 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
];
#[no_mangle]

pub static mut colorBlue: crate::src::qcommon::q_shared::vec4_t = [
    0 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
    0 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
    1 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
    1 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
];
#[no_mangle]

pub static mut colorYellow: crate::src::qcommon::q_shared::vec4_t = [
    1 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
    1 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
    0 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
    1 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
];
#[no_mangle]

pub static mut colorMagenta: crate::src::qcommon::q_shared::vec4_t = [
    1 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
    0 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
    1 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
    1 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
];
#[no_mangle]

pub static mut colorCyan: crate::src::qcommon::q_shared::vec4_t = [
    0 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
    1 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
    1 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
    1 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
];
#[no_mangle]

pub static mut colorWhite: crate::src::qcommon::q_shared::vec4_t = [
    1 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
    1 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
    1 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
    1 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
];
#[no_mangle]

pub static mut colorLtGrey: crate::src::qcommon::q_shared::vec4_t = [
    0.75f64 as crate::src::qcommon::q_shared::vec_t,
    0.75f64 as crate::src::qcommon::q_shared::vec_t,
    0.75f64 as crate::src::qcommon::q_shared::vec_t,
    1 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
];
#[no_mangle]

pub static mut colorMdGrey: crate::src::qcommon::q_shared::vec4_t = [
    0.5f64 as crate::src::qcommon::q_shared::vec_t,
    0.5f64 as crate::src::qcommon::q_shared::vec_t,
    0.5f64 as crate::src::qcommon::q_shared::vec_t,
    1 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
];
#[no_mangle]

pub static mut colorDkGrey: crate::src::qcommon::q_shared::vec4_t = [
    0.25f64 as crate::src::qcommon::q_shared::vec_t,
    0.25f64 as crate::src::qcommon::q_shared::vec_t,
    0.25f64 as crate::src::qcommon::q_shared::vec_t,
    1 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
];
#[no_mangle]

pub static mut g_color_table: [crate::src::qcommon::q_shared::vec4_t; 8] = [
    [
        0.0f64 as crate::src::qcommon::q_shared::vec_t,
        0.0f64 as crate::src::qcommon::q_shared::vec_t,
        0.0f64 as crate::src::qcommon::q_shared::vec_t,
        1.0f64 as crate::src::qcommon::q_shared::vec_t,
    ],
    [
        1.0f64 as crate::src::qcommon::q_shared::vec_t,
        0.0f64 as crate::src::qcommon::q_shared::vec_t,
        0.0f64 as crate::src::qcommon::q_shared::vec_t,
        1.0f64 as crate::src::qcommon::q_shared::vec_t,
    ],
    [
        0.0f64 as crate::src::qcommon::q_shared::vec_t,
        1.0f64 as crate::src::qcommon::q_shared::vec_t,
        0.0f64 as crate::src::qcommon::q_shared::vec_t,
        1.0f64 as crate::src::qcommon::q_shared::vec_t,
    ],
    [
        1.0f64 as crate::src::qcommon::q_shared::vec_t,
        1.0f64 as crate::src::qcommon::q_shared::vec_t,
        0.0f64 as crate::src::qcommon::q_shared::vec_t,
        1.0f64 as crate::src::qcommon::q_shared::vec_t,
    ],
    [
        0.0f64 as crate::src::qcommon::q_shared::vec_t,
        0.0f64 as crate::src::qcommon::q_shared::vec_t,
        1.0f64 as crate::src::qcommon::q_shared::vec_t,
        1.0f64 as crate::src::qcommon::q_shared::vec_t,
    ],
    [
        0.0f64 as crate::src::qcommon::q_shared::vec_t,
        1.0f64 as crate::src::qcommon::q_shared::vec_t,
        1.0f64 as crate::src::qcommon::q_shared::vec_t,
        1.0f64 as crate::src::qcommon::q_shared::vec_t,
    ],
    [
        1.0f64 as crate::src::qcommon::q_shared::vec_t,
        0.0f64 as crate::src::qcommon::q_shared::vec_t,
        1.0f64 as crate::src::qcommon::q_shared::vec_t,
        1.0f64 as crate::src::qcommon::q_shared::vec_t,
    ],
    [
        1.0f64 as crate::src::qcommon::q_shared::vec_t,
        1.0f64 as crate::src::qcommon::q_shared::vec_t,
        1.0f64 as crate::src::qcommon::q_shared::vec_t,
        1.0f64 as crate::src::qcommon::q_shared::vec_t,
    ],
];
#[no_mangle]

pub static mut bytedirs: [crate::src::qcommon::q_shared::vec3_t; 162] = [
    [-0.525731f32, 0.000000f32, 0.850651f32],
    [-0.442863f32, 0.238856f32, 0.864188f32],
    [-0.295242f32, 0.000000f32, 0.955423f32],
    [-0.309017f32, 0.500000f32, 0.809017f32],
    [-0.162460f32, 0.262866f32, 0.951056f32],
    [0.000000f32, 0.000000f32, 1.000000f32],
    [0.000000f32, 0.850651f32, 0.525731f32],
    [-0.147621f32, 0.716567f32, 0.681718f32],
    [0.147621f32, 0.716567f32, 0.681718f32],
    [0.000000f32, 0.525731f32, 0.850651f32],
    [0.309017f32, 0.500000f32, 0.809017f32],
    [0.525731f32, 0.000000f32, 0.850651f32],
    [0.295242f32, 0.000000f32, 0.955423f32],
    [0.442863f32, 0.238856f32, 0.864188f32],
    [0.162460f32, 0.262866f32, 0.951056f32],
    [-0.681718f32, 0.147621f32, 0.716567f32],
    [-0.809017f32, 0.309017f32, 0.500000f32],
    [-0.587785f32, 0.425325f32, 0.688191f32],
    [-0.850651f32, 0.525731f32, 0.000000f32],
    [-0.864188f32, 0.442863f32, 0.238856f32],
    [-0.716567f32, 0.681718f32, 0.147621f32],
    [-0.688191f32, 0.587785f32, 0.425325f32],
    [-0.500000f32, 0.809017f32, 0.309017f32],
    [-0.238856f32, 0.864188f32, 0.442863f32],
    [-0.425325f32, 0.688191f32, 0.587785f32],
    [-0.716567f32, 0.681718f32, -0.147621f32],
    [-0.500000f32, 0.809017f32, -0.309017f32],
    [-0.525731f32, 0.850651f32, 0.000000f32],
    [0.000000f32, 0.850651f32, -0.525731f32],
    [-0.238856f32, 0.864188f32, -0.442863f32],
    [0.000000f32, 0.955423f32, -0.295242f32],
    [-0.262866f32, 0.951056f32, -0.162460f32],
    [0.000000f32, 1.000000f32, 0.000000f32],
    [0.000000f32, 0.955423f32, 0.295242f32],
    [-0.262866f32, 0.951056f32, 0.162460f32],
    [0.238856f32, 0.864188f32, 0.442863f32],
    [0.262866f32, 0.951056f32, 0.162460f32],
    [0.500000f32, 0.809017f32, 0.309017f32],
    [0.238856f32, 0.864188f32, -0.442863f32],
    [0.262866f32, 0.951056f32, -0.162460f32],
    [0.500000f32, 0.809017f32, -0.309017f32],
    [0.850651f32, 0.525731f32, 0.000000f32],
    [0.716567f32, 0.681718f32, 0.147621f32],
    [0.716567f32, 0.681718f32, -0.147621f32],
    [0.525731f32, 0.850651f32, 0.000000f32],
    [0.425325f32, 0.688191f32, 0.587785f32],
    [0.864188f32, 0.442863f32, 0.238856f32],
    [0.688191f32, 0.587785f32, 0.425325f32],
    [0.809017f32, 0.309017f32, 0.500000f32],
    [0.681718f32, 0.147621f32, 0.716567f32],
    [0.587785f32, 0.425325f32, 0.688191f32],
    [0.955423f32, 0.295242f32, 0.000000f32],
    [1.000000f32, 0.000000f32, 0.000000f32],
    [0.951056f32, 0.162460f32, 0.262866f32],
    [0.850651f32, -0.525731f32, 0.000000f32],
    [0.955423f32, -0.295242f32, 0.000000f32],
    [0.864188f32, -0.442863f32, 0.238856f32],
    [0.951056f32, -0.162460f32, 0.262866f32],
    [0.809017f32, -0.309017f32, 0.500000f32],
    [0.681718f32, -0.147621f32, 0.716567f32],
    [0.850651f32, 0.000000f32, 0.525731f32],
    [0.864188f32, 0.442863f32, -0.238856f32],
    [0.809017f32, 0.309017f32, -0.500000f32],
    [0.951056f32, 0.162460f32, -0.262866f32],
    [0.525731f32, 0.000000f32, -0.850651f32],
    [0.681718f32, 0.147621f32, -0.716567f32],
    [0.681718f32, -0.147621f32, -0.716567f32],
    [0.850651f32, 0.000000f32, -0.525731f32],
    [0.809017f32, -0.309017f32, -0.500000f32],
    [0.864188f32, -0.442863f32, -0.238856f32],
    [0.951056f32, -0.162460f32, -0.262866f32],
    [0.147621f32, 0.716567f32, -0.681718f32],
    [0.309017f32, 0.500000f32, -0.809017f32],
    [0.425325f32, 0.688191f32, -0.587785f32],
    [0.442863f32, 0.238856f32, -0.864188f32],
    [0.587785f32, 0.425325f32, -0.688191f32],
    [0.688191f32, 0.587785f32, -0.425325f32],
    [-0.147621f32, 0.716567f32, -0.681718f32],
    [-0.309017f32, 0.500000f32, -0.809017f32],
    [0.000000f32, 0.525731f32, -0.850651f32],
    [-0.525731f32, 0.000000f32, -0.850651f32],
    [-0.442863f32, 0.238856f32, -0.864188f32],
    [-0.295242f32, 0.000000f32, -0.955423f32],
    [-0.162460f32, 0.262866f32, -0.951056f32],
    [0.000000f32, 0.000000f32, -1.000000f32],
    [0.295242f32, 0.000000f32, -0.955423f32],
    [0.162460f32, 0.262866f32, -0.951056f32],
    [-0.442863f32, -0.238856f32, -0.864188f32],
    [-0.309017f32, -0.500000f32, -0.809017f32],
    [-0.162460f32, -0.262866f32, -0.951056f32],
    [0.000000f32, -0.850651f32, -0.525731f32],
    [-0.147621f32, -0.716567f32, -0.681718f32],
    [0.147621f32, -0.716567f32, -0.681718f32],
    [0.000000f32, -0.525731f32, -0.850651f32],
    [0.309017f32, -0.500000f32, -0.809017f32],
    [0.442863f32, -0.238856f32, -0.864188f32],
    [0.162460f32, -0.262866f32, -0.951056f32],
    [0.238856f32, -0.864188f32, -0.442863f32],
    [0.500000f32, -0.809017f32, -0.309017f32],
    [0.425325f32, -0.688191f32, -0.587785f32],
    [0.716567f32, -0.681718f32, -0.147621f32],
    [0.688191f32, -0.587785f32, -0.425325f32],
    [0.587785f32, -0.425325f32, -0.688191f32],
    [0.000000f32, -0.955423f32, -0.295242f32],
    [0.000000f32, -1.000000f32, 0.000000f32],
    [0.262866f32, -0.951056f32, -0.162460f32],
    [0.000000f32, -0.850651f32, 0.525731f32],
    [0.000000f32, -0.955423f32, 0.295242f32],
    [0.238856f32, -0.864188f32, 0.442863f32],
    [0.262866f32, -0.951056f32, 0.162460f32],
    [0.500000f32, -0.809017f32, 0.309017f32],
    [0.716567f32, -0.681718f32, 0.147621f32],
    [0.525731f32, -0.850651f32, 0.000000f32],
    [-0.238856f32, -0.864188f32, -0.442863f32],
    [-0.500000f32, -0.809017f32, -0.309017f32],
    [-0.262866f32, -0.951056f32, -0.162460f32],
    [-0.850651f32, -0.525731f32, 0.000000f32],
    [-0.716567f32, -0.681718f32, -0.147621f32],
    [-0.716567f32, -0.681718f32, 0.147621f32],
    [-0.525731f32, -0.850651f32, 0.000000f32],
    [-0.500000f32, -0.809017f32, 0.309017f32],
    [-0.238856f32, -0.864188f32, 0.442863f32],
    [-0.262866f32, -0.951056f32, 0.162460f32],
    [-0.864188f32, -0.442863f32, 0.238856f32],
    [-0.809017f32, -0.309017f32, 0.500000f32],
    [-0.688191f32, -0.587785f32, 0.425325f32],
    [-0.681718f32, -0.147621f32, 0.716567f32],
    [-0.442863f32, -0.238856f32, 0.864188f32],
    [-0.587785f32, -0.425325f32, 0.688191f32],
    [-0.309017f32, -0.500000f32, 0.809017f32],
    [-0.147621f32, -0.716567f32, 0.681718f32],
    [-0.425325f32, -0.688191f32, 0.587785f32],
    [-0.162460f32, -0.262866f32, 0.951056f32],
    [0.442863f32, -0.238856f32, 0.864188f32],
    [0.162460f32, -0.262866f32, 0.951056f32],
    [0.309017f32, -0.500000f32, 0.809017f32],
    [0.147621f32, -0.716567f32, 0.681718f32],
    [0.000000f32, -0.525731f32, 0.850651f32],
    [0.425325f32, -0.688191f32, 0.587785f32],
    [0.587785f32, -0.425325f32, 0.688191f32],
    [0.688191f32, -0.587785f32, 0.425325f32],
    [-0.955423f32, 0.295242f32, 0.000000f32],
    [-0.951056f32, 0.162460f32, 0.262866f32],
    [-1.000000f32, 0.000000f32, 0.000000f32],
    [-0.850651f32, 0.000000f32, 0.525731f32],
    [-0.955423f32, -0.295242f32, 0.000000f32],
    [-0.951056f32, -0.162460f32, 0.262866f32],
    [-0.864188f32, 0.442863f32, -0.238856f32],
    [-0.951056f32, 0.162460f32, -0.262866f32],
    [-0.809017f32, 0.309017f32, -0.500000f32],
    [-0.864188f32, -0.442863f32, -0.238856f32],
    [-0.951056f32, -0.162460f32, -0.262866f32],
    [-0.809017f32, -0.309017f32, -0.500000f32],
    [-0.681718f32, 0.147621f32, -0.716567f32],
    [-0.681718f32, -0.147621f32, -0.716567f32],
    [-0.850651f32, 0.000000f32, -0.525731f32],
    [-0.688191f32, 0.587785f32, -0.425325f32],
    [-0.587785f32, 0.425325f32, -0.688191f32],
    [-0.425325f32, 0.688191f32, -0.587785f32],
    [-0.425325f32, -0.688191f32, -0.587785f32],
    [-0.587785f32, -0.425325f32, -0.688191f32],
    [-0.688191f32, -0.587785f32, -0.425325f32],
];
//==============================================================
#[no_mangle]

pub unsafe extern "C" fn Q_rand(mut seed: *mut libc::c_int) -> libc::c_int {
    *seed = (69069 as libc::c_uint)
        .wrapping_mul(*seed as libc::c_uint)
        .wrapping_add(1 as libc::c_uint) as libc::c_int;
    return *seed;
}
#[no_mangle]

pub unsafe extern "C" fn Q_random(mut seed: *mut libc::c_int) -> libc::c_float {
    return (Q_rand(seed) & 0xffff as libc::c_int) as libc::c_float
        / 0x10000 as libc::c_int as libc::c_float;
}
#[no_mangle]

pub unsafe extern "C" fn Q_crandom(mut seed: *mut libc::c_int) -> libc::c_float {
    return (2.0f64 * (Q_random(seed) as libc::c_double - 0.5f64)) as libc::c_float;
}
//=======================================================
#[no_mangle]

pub unsafe extern "C" fn ClampChar(mut i: libc::c_int) -> libc::c_schar {
    if i < -(128 as libc::c_int) {
        return -(128 as libc::c_int) as libc::c_schar;
    }
    if i > 127 as libc::c_int {
        return 127 as libc::c_int as libc::c_schar;
    }
    return i as libc::c_schar;
}
#[no_mangle]

pub unsafe extern "C" fn ClampShort(mut i: libc::c_int) -> libc::c_short {
    if i < -(32768 as libc::c_int) {
        return -(32768 as libc::c_int) as libc::c_short;
    }
    if i > 0x7fff as libc::c_int {
        return 0x7fff as libc::c_int as libc::c_short;
    }
    return i as libc::c_short;
}
// this isn't a real cheap function to call!
#[no_mangle]

pub unsafe extern "C" fn DirToByte(
    mut dir: *mut crate::src::qcommon::q_shared::vec_t,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut best: libc::c_int = 0;
    let mut d: libc::c_float = 0.;
    let mut bestd: libc::c_float = 0.;
    if dir.is_null() {
        return 0 as libc::c_int;
    }
    bestd = 0 as libc::c_int as libc::c_float;
    best = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 162 as libc::c_int {
        d = *dir.offset(0 as libc::c_int as isize)
            * bytedirs[i as usize][0 as libc::c_int as usize]
            + *dir.offset(1 as libc::c_int as isize)
                * bytedirs[i as usize][1 as libc::c_int as usize]
            + *dir.offset(2 as libc::c_int as isize)
                * bytedirs[i as usize][2 as libc::c_int as usize];
        if d > bestd {
            bestd = d;
            best = i
        }
        i += 1
    }
    return best;
}
#[no_mangle]

pub unsafe extern "C" fn ByteToDir(
    mut b: libc::c_int,
    mut dir: *mut crate::src::qcommon::q_shared::vec_t,
) {
    if b < 0 as libc::c_int || b >= 162 as libc::c_int {
        *dir.offset(0 as libc::c_int as isize) = vec3_origin[0 as libc::c_int as usize];
        *dir.offset(1 as libc::c_int as isize) = vec3_origin[1 as libc::c_int as usize];
        *dir.offset(2 as libc::c_int as isize) = vec3_origin[2 as libc::c_int as usize];
        return;
    }
    *dir.offset(0 as libc::c_int as isize) = bytedirs[b as usize][0 as libc::c_int as usize];
    *dir.offset(1 as libc::c_int as isize) = bytedirs[b as usize][1 as libc::c_int as usize];
    *dir.offset(2 as libc::c_int as isize) = bytedirs[b as usize][2 as libc::c_int as usize];
}
#[no_mangle]

pub unsafe extern "C" fn ColorBytes3(
    mut r: libc::c_float,
    mut g: libc::c_float,
    mut b: libc::c_float,
) -> libc::c_uint {
    let mut i: libc::c_uint = 0;
    *(&mut i as *mut libc::c_uint as *mut crate::src::qcommon::q_shared::byte)
        .offset(0 as libc::c_int as isize) =
        (r * 255 as libc::c_int as libc::c_float) as crate::src::qcommon::q_shared::byte;
    *(&mut i as *mut libc::c_uint as *mut crate::src::qcommon::q_shared::byte)
        .offset(1 as libc::c_int as isize) =
        (g * 255 as libc::c_int as libc::c_float) as crate::src::qcommon::q_shared::byte;
    *(&mut i as *mut libc::c_uint as *mut crate::src::qcommon::q_shared::byte)
        .offset(2 as libc::c_int as isize) =
        (b * 255 as libc::c_int as libc::c_float) as crate::src::qcommon::q_shared::byte;
    return i;
}
#[no_mangle]

pub unsafe extern "C" fn ColorBytes4(
    mut r: libc::c_float,
    mut g: libc::c_float,
    mut b: libc::c_float,
    mut a: libc::c_float,
) -> libc::c_uint {
    let mut i: libc::c_uint = 0;
    *(&mut i as *mut libc::c_uint as *mut crate::src::qcommon::q_shared::byte)
        .offset(0 as libc::c_int as isize) =
        (r * 255 as libc::c_int as libc::c_float) as crate::src::qcommon::q_shared::byte;
    *(&mut i as *mut libc::c_uint as *mut crate::src::qcommon::q_shared::byte)
        .offset(1 as libc::c_int as isize) =
        (g * 255 as libc::c_int as libc::c_float) as crate::src::qcommon::q_shared::byte;
    *(&mut i as *mut libc::c_uint as *mut crate::src::qcommon::q_shared::byte)
        .offset(2 as libc::c_int as isize) =
        (b * 255 as libc::c_int as libc::c_float) as crate::src::qcommon::q_shared::byte;
    *(&mut i as *mut libc::c_uint as *mut crate::src::qcommon::q_shared::byte)
        .offset(3 as libc::c_int as isize) =
        (a * 255 as libc::c_int as libc::c_float) as crate::src::qcommon::q_shared::byte;
    return i;
}
#[no_mangle]

pub unsafe extern "C" fn NormalizeColor(
    mut in_0: *const crate::src::qcommon::q_shared::vec_t,
    mut out: *mut crate::src::qcommon::q_shared::vec_t,
) -> libc::c_float {
    let mut max: libc::c_float = 0.;
    max = *in_0.offset(0 as libc::c_int as isize);
    if *in_0.offset(1 as libc::c_int as isize) > max {
        max = *in_0.offset(1 as libc::c_int as isize)
    }
    if *in_0.offset(2 as libc::c_int as isize) > max {
        max = *in_0.offset(2 as libc::c_int as isize)
    }
    if max == 0. {
        let ref mut fresh0 = *out.offset(2 as libc::c_int as isize);
        *fresh0 = 0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
        let ref mut fresh1 = *out.offset(1 as libc::c_int as isize);
        *fresh1 = *fresh0;
        *out.offset(0 as libc::c_int as isize) = *fresh1
    } else {
        *out.offset(0 as libc::c_int as isize) = *in_0.offset(0 as libc::c_int as isize) / max;
        *out.offset(1 as libc::c_int as isize) = *in_0.offset(1 as libc::c_int as isize) / max;
        *out.offset(2 as libc::c_int as isize) = *in_0.offset(2 as libc::c_int as isize) / max
    }
    return max;
}
/*
=====================
PlaneFromPoints

Returns false if the triangle is degenrate.
The normal will point out of the clock for clockwise ordered points
=====================
*/
#[no_mangle]

pub unsafe extern "C" fn PlaneFromPoints(
    mut plane: *mut crate::src::qcommon::q_shared::vec_t,
    mut a: *const crate::src::qcommon::q_shared::vec_t,
    mut b: *const crate::src::qcommon::q_shared::vec_t,
    mut c: *const crate::src::qcommon::q_shared::vec_t,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut d1: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut d2: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    d1[0 as libc::c_int as usize] =
        *b.offset(0 as libc::c_int as isize) - *a.offset(0 as libc::c_int as isize);
    d1[1 as libc::c_int as usize] =
        *b.offset(1 as libc::c_int as isize) - *a.offset(1 as libc::c_int as isize);
    d1[2 as libc::c_int as usize] =
        *b.offset(2 as libc::c_int as isize) - *a.offset(2 as libc::c_int as isize);
    d2[0 as libc::c_int as usize] =
        *c.offset(0 as libc::c_int as isize) - *a.offset(0 as libc::c_int as isize);
    d2[1 as libc::c_int as usize] =
        *c.offset(1 as libc::c_int as isize) - *a.offset(1 as libc::c_int as isize);
    d2[2 as libc::c_int as usize] =
        *c.offset(2 as libc::c_int as isize) - *a.offset(2 as libc::c_int as isize);
    CrossProduct(
        d2.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        d1.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        plane,
    );
    if VectorNormalize(plane) == 0 as libc::c_int as libc::c_float {
        return crate::src::qcommon::q_shared::qfalse;
    }
    *plane.offset(3 as libc::c_int as isize) = *a.offset(0 as libc::c_int as isize)
        * *plane.offset(0 as libc::c_int as isize)
        + *a.offset(1 as libc::c_int as isize) * *plane.offset(1 as libc::c_int as isize)
        + *a.offset(2 as libc::c_int as isize) * *plane.offset(2 as libc::c_int as isize);
    return crate::src::qcommon::q_shared::qtrue;
}
/*
===============
RotatePointAroundVector

This is not implemented very well...
===============
*/
#[no_mangle]

pub unsafe extern "C" fn RotatePointAroundVector(
    mut dst: *mut crate::src::qcommon::q_shared::vec_t,
    mut dir: *const crate::src::qcommon::q_shared::vec_t,
    mut point: *const crate::src::qcommon::q_shared::vec_t,
    mut degrees: libc::c_float,
) {
    let mut m: [[libc::c_float; 3]; 3] = [[0.; 3]; 3];
    let mut im: [[libc::c_float; 3]; 3] = [[0.; 3]; 3];
    let mut zrot: [[libc::c_float; 3]; 3] = [[0.; 3]; 3];
    let mut tmpmat: [[libc::c_float; 3]; 3] = [[0.; 3]; 3];
    let mut rot: [[libc::c_float; 3]; 3] = [[0.; 3]; 3];
    let mut i: libc::c_int = 0;
    let mut vr: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut vup: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut vf: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut rad: libc::c_float = 0.;
    vf[0 as libc::c_int as usize] = *dir.offset(0 as libc::c_int as isize);
    vf[1 as libc::c_int as usize] = *dir.offset(1 as libc::c_int as isize);
    vf[2 as libc::c_int as usize] = *dir.offset(2 as libc::c_int as isize);
    PerpendicularVector(vr.as_mut_ptr(), dir);
    CrossProduct(
        vr.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        vf.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        vup.as_mut_ptr(),
    );
    m[0 as libc::c_int as usize][0 as libc::c_int as usize] = vr[0 as libc::c_int as usize];
    m[1 as libc::c_int as usize][0 as libc::c_int as usize] = vr[1 as libc::c_int as usize];
    m[2 as libc::c_int as usize][0 as libc::c_int as usize] = vr[2 as libc::c_int as usize];
    m[0 as libc::c_int as usize][1 as libc::c_int as usize] = vup[0 as libc::c_int as usize];
    m[1 as libc::c_int as usize][1 as libc::c_int as usize] = vup[1 as libc::c_int as usize];
    m[2 as libc::c_int as usize][1 as libc::c_int as usize] = vup[2 as libc::c_int as usize];
    m[0 as libc::c_int as usize][2 as libc::c_int as usize] = vf[0 as libc::c_int as usize];
    m[1 as libc::c_int as usize][2 as libc::c_int as usize] = vf[1 as libc::c_int as usize];
    m[2 as libc::c_int as usize][2 as libc::c_int as usize] = vf[2 as libc::c_int as usize];
    crate::stdlib::memcpy(
        im.as_mut_ptr() as *mut libc::c_void,
        m.as_mut_ptr() as *const libc::c_void,
        ::std::mem::size_of::<[[libc::c_float; 3]; 3]>() as libc::c_ulong,
    );
    im[0 as libc::c_int as usize][1 as libc::c_int as usize] =
        m[1 as libc::c_int as usize][0 as libc::c_int as usize];
    im[0 as libc::c_int as usize][2 as libc::c_int as usize] =
        m[2 as libc::c_int as usize][0 as libc::c_int as usize];
    im[1 as libc::c_int as usize][0 as libc::c_int as usize] =
        m[0 as libc::c_int as usize][1 as libc::c_int as usize];
    im[1 as libc::c_int as usize][2 as libc::c_int as usize] =
        m[2 as libc::c_int as usize][1 as libc::c_int as usize];
    im[2 as libc::c_int as usize][0 as libc::c_int as usize] =
        m[0 as libc::c_int as usize][2 as libc::c_int as usize];
    im[2 as libc::c_int as usize][1 as libc::c_int as usize] =
        m[1 as libc::c_int as usize][2 as libc::c_int as usize];
    crate::stdlib::memset(
        zrot.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[[libc::c_float; 3]; 3]>() as libc::c_ulong,
    );
    zrot[2 as libc::c_int as usize][2 as libc::c_int as usize] = 1.0f32;
    zrot[1 as libc::c_int as usize][1 as libc::c_int as usize] =
        zrot[2 as libc::c_int as usize][2 as libc::c_int as usize];
    zrot[0 as libc::c_int as usize][0 as libc::c_int as usize] =
        zrot[1 as libc::c_int as usize][1 as libc::c_int as usize];
    rad = (degrees as libc::c_double * 3.14159265358979323846f64 / 180.0f32 as libc::c_double)
        as libc::c_float;
    zrot[0 as libc::c_int as usize][0 as libc::c_int as usize] =
        crate::stdlib::cos(rad as libc::c_double) as libc::c_float;
    zrot[0 as libc::c_int as usize][1 as libc::c_int as usize] =
        crate::stdlib::sin(rad as libc::c_double) as libc::c_float;
    zrot[1 as libc::c_int as usize][0 as libc::c_int as usize] =
        -crate::stdlib::sin(rad as libc::c_double) as libc::c_float;
    zrot[1 as libc::c_int as usize][1 as libc::c_int as usize] =
        crate::stdlib::cos(rad as libc::c_double) as libc::c_float;
    MatrixMultiply(m.as_mut_ptr(), zrot.as_mut_ptr(), tmpmat.as_mut_ptr());
    MatrixMultiply(tmpmat.as_mut_ptr(), im.as_mut_ptr(), rot.as_mut_ptr());
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        *dst.offset(i as isize) = rot[i as usize][0 as libc::c_int as usize]
            * *point.offset(0 as libc::c_int as isize)
            + rot[i as usize][1 as libc::c_int as usize] * *point.offset(1 as libc::c_int as isize)
            + rot[i as usize][2 as libc::c_int as usize] * *point.offset(2 as libc::c_int as isize);
        i += 1
    }
}
/*
===============
RotateAroundDirection
===============
*/
#[no_mangle]

pub unsafe extern "C" fn RotateAroundDirection(
    mut axis: *mut crate::src::qcommon::q_shared::vec3_t,
    mut yaw: libc::c_float,
) {
    // create an arbitrary axis[1]
    PerpendicularVector(
        (*axis.offset(1 as libc::c_int as isize)).as_mut_ptr(),
        (*axis.offset(0 as libc::c_int as isize)).as_mut_ptr()
            as *const crate::src::qcommon::q_shared::vec_t,
    );
    // rotate it around axis[0] by yaw
    if yaw != 0. {
        let mut temp: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
        temp[0 as libc::c_int as usize] =
            (*axis.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize];
        temp[1 as libc::c_int as usize] =
            (*axis.offset(1 as libc::c_int as isize))[1 as libc::c_int as usize];
        temp[2 as libc::c_int as usize] =
            (*axis.offset(1 as libc::c_int as isize))[2 as libc::c_int as usize];
        RotatePointAroundVector(
            (*axis.offset(1 as libc::c_int as isize)).as_mut_ptr(),
            (*axis.offset(0 as libc::c_int as isize)).as_mut_ptr()
                as *const crate::src::qcommon::q_shared::vec_t,
            temp.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            yaw,
        );
    }
    // cross to get axis[2]
    CrossProduct(
        (*axis.offset(0 as libc::c_int as isize)).as_mut_ptr()
            as *const crate::src::qcommon::q_shared::vec_t,
        (*axis.offset(1 as libc::c_int as isize)).as_mut_ptr()
            as *const crate::src::qcommon::q_shared::vec_t,
        (*axis.offset(2 as libc::c_int as isize)).as_mut_ptr(),
    );
}
#[no_mangle]

pub unsafe extern "C" fn vectoangles(
    mut value1: *const crate::src::qcommon::q_shared::vec_t,
    mut angles: *mut crate::src::qcommon::q_shared::vec_t,
) {
    let mut forward: libc::c_float = 0.;
    let mut yaw: libc::c_float = 0.;
    let mut pitch: libc::c_float = 0.;
    if *value1.offset(1 as libc::c_int as isize) == 0 as libc::c_int as libc::c_float
        && *value1.offset(0 as libc::c_int as isize) == 0 as libc::c_int as libc::c_float
    {
        yaw = 0 as libc::c_int as libc::c_float;
        if *value1.offset(2 as libc::c_int as isize) > 0 as libc::c_int as libc::c_float {
            pitch = 90 as libc::c_int as libc::c_float
        } else {
            pitch = 270 as libc::c_int as libc::c_float
        }
    } else {
        if *value1.offset(0 as libc::c_int as isize) != 0. {
            yaw = (crate::stdlib::atan2(
                *value1.offset(1 as libc::c_int as isize) as libc::c_double,
                *value1.offset(0 as libc::c_int as isize) as libc::c_double,
            ) * 180 as libc::c_int as libc::c_double
                / 3.14159265358979323846f64) as libc::c_float
        } else if *value1.offset(1 as libc::c_int as isize) > 0 as libc::c_int as libc::c_float {
            yaw = 90 as libc::c_int as libc::c_float
        } else {
            yaw = 270 as libc::c_int as libc::c_float
        }
        if yaw < 0 as libc::c_int as libc::c_float {
            yaw += 360 as libc::c_int as libc::c_float
        }
        forward = crate::stdlib::sqrt(
            (*value1.offset(0 as libc::c_int as isize) * *value1.offset(0 as libc::c_int as isize)
                + *value1.offset(1 as libc::c_int as isize)
                    * *value1.offset(1 as libc::c_int as isize)) as libc::c_double,
        ) as libc::c_float;
        pitch = (crate::stdlib::atan2(
            *value1.offset(2 as libc::c_int as isize) as libc::c_double,
            forward as libc::c_double,
        ) * 180 as libc::c_int as libc::c_double
            / 3.14159265358979323846f64) as libc::c_float;
        if pitch < 0 as libc::c_int as libc::c_float {
            pitch += 360 as libc::c_int as libc::c_float
        }
    }
    *angles.offset(0 as libc::c_int as isize) = -pitch;
    *angles.offset(1 as libc::c_int as isize) = yaw;
    *angles.offset(2 as libc::c_int as isize) =
        0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
}
/*
=================
AnglesToAxis
=================
*/
#[no_mangle]

pub unsafe extern "C" fn AnglesToAxis(
    mut angles: *const crate::src::qcommon::q_shared::vec_t,
    mut axis: *mut crate::src::qcommon::q_shared::vec3_t,
) {
    let mut right: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    // angle vectors returns "right" instead of "y axis"
    AngleVectors(
        angles,
        (*axis.offset(0 as libc::c_int as isize)).as_mut_ptr(),
        right.as_mut_ptr(),
        (*axis.offset(2 as libc::c_int as isize)).as_mut_ptr(),
    );
    (*axis.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize] =
        vec3_origin[0 as libc::c_int as usize] - right[0 as libc::c_int as usize];
    (*axis.offset(1 as libc::c_int as isize))[1 as libc::c_int as usize] =
        vec3_origin[1 as libc::c_int as usize] - right[1 as libc::c_int as usize];
    (*axis.offset(1 as libc::c_int as isize))[2 as libc::c_int as usize] =
        vec3_origin[2 as libc::c_int as usize] - right[2 as libc::c_int as usize];
}
#[no_mangle]

pub unsafe extern "C" fn AxisClear(mut axis: *mut crate::src::qcommon::q_shared::vec3_t) {
    (*axis.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize] =
        1 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
    (*axis.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize] =
        0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
    (*axis.offset(0 as libc::c_int as isize))[2 as libc::c_int as usize] =
        0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
    (*axis.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize] =
        0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
    (*axis.offset(1 as libc::c_int as isize))[1 as libc::c_int as usize] =
        1 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
    (*axis.offset(1 as libc::c_int as isize))[2 as libc::c_int as usize] =
        0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
    (*axis.offset(2 as libc::c_int as isize))[0 as libc::c_int as usize] =
        0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
    (*axis.offset(2 as libc::c_int as isize))[1 as libc::c_int as usize] =
        0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
    (*axis.offset(2 as libc::c_int as isize))[2 as libc::c_int as usize] =
        1 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
}
#[no_mangle]

pub unsafe extern "C" fn AxisCopy(
    mut in_0: *mut crate::src::qcommon::q_shared::vec3_t,
    mut out: *mut crate::src::qcommon::q_shared::vec3_t,
) {
    (*out.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize] =
        (*in_0.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize];
    (*out.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize] =
        (*in_0.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize];
    (*out.offset(0 as libc::c_int as isize))[2 as libc::c_int as usize] =
        (*in_0.offset(0 as libc::c_int as isize))[2 as libc::c_int as usize];
    (*out.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize] =
        (*in_0.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize];
    (*out.offset(1 as libc::c_int as isize))[1 as libc::c_int as usize] =
        (*in_0.offset(1 as libc::c_int as isize))[1 as libc::c_int as usize];
    (*out.offset(1 as libc::c_int as isize))[2 as libc::c_int as usize] =
        (*in_0.offset(1 as libc::c_int as isize))[2 as libc::c_int as usize];
    (*out.offset(2 as libc::c_int as isize))[0 as libc::c_int as usize] =
        (*in_0.offset(2 as libc::c_int as isize))[0 as libc::c_int as usize];
    (*out.offset(2 as libc::c_int as isize))[1 as libc::c_int as usize] =
        (*in_0.offset(2 as libc::c_int as isize))[1 as libc::c_int as usize];
    (*out.offset(2 as libc::c_int as isize))[2 as libc::c_int as usize] =
        (*in_0.offset(2 as libc::c_int as isize))[2 as libc::c_int as usize];
}
#[no_mangle]

pub unsafe extern "C" fn ProjectPointOnPlane(
    mut dst: *mut crate::src::qcommon::q_shared::vec_t,
    mut p: *const crate::src::qcommon::q_shared::vec_t,
    mut normal: *const crate::src::qcommon::q_shared::vec_t,
) {
    let mut d: libc::c_float = 0.;
    let mut n: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut inv_denom: libc::c_float = 0.;
    inv_denom = *normal.offset(0 as libc::c_int as isize)
        * *normal.offset(0 as libc::c_int as isize)
        + *normal.offset(1 as libc::c_int as isize) * *normal.offset(1 as libc::c_int as isize)
        + *normal.offset(2 as libc::c_int as isize) * *normal.offset(2 as libc::c_int as isize);
    // zero vectors get here
    inv_denom = 1.0f32 / inv_denom;
    d = (*normal.offset(0 as libc::c_int as isize) * *p.offset(0 as libc::c_int as isize)
        + *normal.offset(1 as libc::c_int as isize) * *p.offset(1 as libc::c_int as isize)
        + *normal.offset(2 as libc::c_int as isize) * *p.offset(2 as libc::c_int as isize))
        * inv_denom;
    n[0 as libc::c_int as usize] = *normal.offset(0 as libc::c_int as isize) * inv_denom;
    n[1 as libc::c_int as usize] = *normal.offset(1 as libc::c_int as isize) * inv_denom;
    n[2 as libc::c_int as usize] = *normal.offset(2 as libc::c_int as isize) * inv_denom;
    *dst.offset(0 as libc::c_int as isize) =
        *p.offset(0 as libc::c_int as isize) - d * n[0 as libc::c_int as usize];
    *dst.offset(1 as libc::c_int as isize) =
        *p.offset(1 as libc::c_int as isize) - d * n[1 as libc::c_int as usize];
    *dst.offset(2 as libc::c_int as isize) =
        *p.offset(2 as libc::c_int as isize) - d * n[2 as libc::c_int as usize];
}
/*
================
MakeNormalVectors

Given a normalized forward vector, create two
other perpendicular vectors
================
*/
#[no_mangle]

pub unsafe extern "C" fn MakeNormalVectors(
    mut forward: *const crate::src::qcommon::q_shared::vec_t,
    mut right: *mut crate::src::qcommon::q_shared::vec_t,
    mut up: *mut crate::src::qcommon::q_shared::vec_t,
) {
    let mut d: libc::c_float = 0.;
    // this rotate and negate guarantees a vector
    // not colinear with the original
    *right.offset(1 as libc::c_int as isize) = -*forward.offset(0 as libc::c_int as isize);
    *right.offset(2 as libc::c_int as isize) = *forward.offset(1 as libc::c_int as isize);
    *right.offset(0 as libc::c_int as isize) = *forward.offset(2 as libc::c_int as isize);
    d = *right.offset(0 as libc::c_int as isize) * *forward.offset(0 as libc::c_int as isize)
        + *right.offset(1 as libc::c_int as isize) * *forward.offset(1 as libc::c_int as isize)
        + *right.offset(2 as libc::c_int as isize) * *forward.offset(2 as libc::c_int as isize);
    *right.offset(0 as libc::c_int as isize) =
        *right.offset(0 as libc::c_int as isize) + *forward.offset(0 as libc::c_int as isize) * -d;
    *right.offset(1 as libc::c_int as isize) =
        *right.offset(1 as libc::c_int as isize) + *forward.offset(1 as libc::c_int as isize) * -d;
    *right.offset(2 as libc::c_int as isize) =
        *right.offset(2 as libc::c_int as isize) + *forward.offset(2 as libc::c_int as isize) * -d;
    VectorNormalize(right);
    CrossProduct(
        right as *const crate::src::qcommon::q_shared::vec_t,
        forward,
        up,
    );
}
#[no_mangle]

pub unsafe extern "C" fn VectorRotate(
    mut in_0: *mut crate::src::qcommon::q_shared::vec_t,
    mut matrix: *mut crate::src::qcommon::q_shared::vec3_t,
    mut out: *mut crate::src::qcommon::q_shared::vec_t,
) {
    *out.offset(0 as libc::c_int as isize) = *in_0.offset(0 as libc::c_int as isize)
        * (*matrix.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize]
        + *in_0.offset(1 as libc::c_int as isize)
            * (*matrix.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize]
        + *in_0.offset(2 as libc::c_int as isize)
            * (*matrix.offset(0 as libc::c_int as isize))[2 as libc::c_int as usize];
    *out.offset(1 as libc::c_int as isize) = *in_0.offset(0 as libc::c_int as isize)
        * (*matrix.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize]
        + *in_0.offset(1 as libc::c_int as isize)
            * (*matrix.offset(1 as libc::c_int as isize))[1 as libc::c_int as usize]
        + *in_0.offset(2 as libc::c_int as isize)
            * (*matrix.offset(1 as libc::c_int as isize))[2 as libc::c_int as usize];
    *out.offset(2 as libc::c_int as isize) = *in_0.offset(0 as libc::c_int as isize)
        * (*matrix.offset(2 as libc::c_int as isize))[0 as libc::c_int as usize]
        + *in_0.offset(1 as libc::c_int as isize)
            * (*matrix.offset(2 as libc::c_int as isize))[1 as libc::c_int as usize]
        + *in_0.offset(2 as libc::c_int as isize)
            * (*matrix.offset(2 as libc::c_int as isize))[2 as libc::c_int as usize];
}
//============================================================================
/*
** float q_rsqrt( float number )
*/
#[no_mangle]

pub unsafe extern "C" fn Q_rsqrt(mut number: libc::c_float) -> libc::c_float {
    let mut t: crate::src::qcommon::q_shared::floatint_t =
        crate::src::qcommon::q_shared::floatint_t { f: 0. }; // what the fuck?
    let mut x2: libc::c_float = 0.; // 1st iteration
    let mut y: libc::c_float = 0.;
    let threehalfs: libc::c_float = 1.5f32;
    x2 = number * 0.5f32;
    t.f = number;
    t.i = 0x5f3759df as libc::c_int - (t.i >> 1 as libc::c_int);
    y = t.f;
    y = y * (threehalfs - x2 * y * y);
    //	y  = y * ( threehalfs - ( x2 * y * y ) );   // 2nd iteration, this can be removed
    return y;
}
#[no_mangle]

pub unsafe extern "C" fn Q_fabs(mut f: libc::c_float) -> libc::c_float {
    let mut fi: crate::src::qcommon::q_shared::floatint_t =
        crate::src::qcommon::q_shared::floatint_t { f: 0. };
    fi.f = f;
    fi.i &= 0x7fffffff as libc::c_int;
    return fi.f;
}
//============================================================
/*
===============
LerpAngle

===============
*/
#[no_mangle]

pub unsafe extern "C" fn LerpAngle(
    mut from: libc::c_float,
    mut to: libc::c_float,
    mut frac: libc::c_float,
) -> libc::c_float {
    let mut a: libc::c_float = 0.;
    if to - from > 180 as libc::c_int as libc::c_float {
        to -= 360 as libc::c_int as libc::c_float
    }
    if to - from < -(180 as libc::c_int) as libc::c_float {
        to += 360 as libc::c_int as libc::c_float
    }
    a = from + frac * (to - from);
    return a;
}
/*
=================
AngleSubtract

Always returns a value from -180 to 180
=================
*/
#[no_mangle]

pub unsafe extern "C" fn AngleSubtract(
    mut a1: libc::c_float,
    mut a2: libc::c_float,
) -> libc::c_float {
    let mut a: libc::c_float = 0.;
    a = a1 - a2;
    while a > 180 as libc::c_int as libc::c_float {
        a -= 360 as libc::c_int as libc::c_float
    }
    while a < -(180 as libc::c_int) as libc::c_float {
        a += 360 as libc::c_int as libc::c_float
    }
    return a;
}
#[no_mangle]

pub unsafe extern "C" fn AnglesSubtract(
    mut v1: *mut crate::src::qcommon::q_shared::vec_t,
    mut v2: *mut crate::src::qcommon::q_shared::vec_t,
    mut v3: *mut crate::src::qcommon::q_shared::vec_t,
) {
    *v3.offset(0 as libc::c_int as isize) = AngleSubtract(
        *v1.offset(0 as libc::c_int as isize),
        *v2.offset(0 as libc::c_int as isize),
    );
    *v3.offset(1 as libc::c_int as isize) = AngleSubtract(
        *v1.offset(1 as libc::c_int as isize),
        *v2.offset(1 as libc::c_int as isize),
    );
    *v3.offset(2 as libc::c_int as isize) = AngleSubtract(
        *v1.offset(2 as libc::c_int as isize),
        *v2.offset(2 as libc::c_int as isize),
    );
}
#[no_mangle]

pub unsafe extern "C" fn AngleMod(mut a: libc::c_float) -> libc::c_float {
    a = (360.0f64 / 65536 as libc::c_int as libc::c_double
        * ((a as libc::c_double * (65536 as libc::c_int as libc::c_double / 360.0f64))
            as libc::c_int
            & 65535 as libc::c_int) as libc::c_double) as libc::c_float;
    return a;
}
/*
=================
AngleNormalize360

returns angle normalized to the range [0 <= angle < 360]
=================
*/
#[no_mangle]

pub unsafe extern "C" fn AngleNormalize360(mut angle: libc::c_float) -> libc::c_float {
    return (360.0f64 / 65536 as libc::c_int as libc::c_double
        * ((angle as libc::c_double * (65536 as libc::c_int as libc::c_double / 360.0f64))
            as libc::c_int
            & 65535 as libc::c_int) as libc::c_double) as libc::c_float;
}
/*
=================
AngleNormalize180

returns angle normalized to the range [-180 < angle <= 180]
=================
*/
#[no_mangle]

pub unsafe extern "C" fn AngleNormalize180(mut angle: libc::c_float) -> libc::c_float {
    angle = AngleNormalize360(angle);
    if angle as libc::c_double > 180.0f64 {
        angle = (angle as libc::c_double - 360.0f64) as libc::c_float
    }
    return angle;
}
/*
=================
AngleDelta

returns the normalized delta from angle1 to angle2
=================
*/
#[no_mangle]

pub unsafe extern "C" fn AngleDelta(
    mut angle1: libc::c_float,
    mut angle2: libc::c_float,
) -> libc::c_float {
    return AngleNormalize180(angle1 - angle2);
}
//============================================================
/*
=================
SetPlaneSignbits
=================
*/
#[no_mangle]

pub unsafe extern "C" fn SetPlaneSignbits(mut out: *mut crate::src::qcommon::q_shared::cplane_t) {
    let mut bits: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    // for fast box on planeside test
    bits = 0 as libc::c_int;
    j = 0 as libc::c_int;
    while j < 3 as libc::c_int {
        if (*out).normal[j as usize] < 0 as libc::c_int as libc::c_float {
            bits |= (1 as libc::c_int) << j
        }
        j += 1
    }
    (*out).signbits = bits as crate::src::qcommon::q_shared::byte;
}
/*
==================
BoxOnPlaneSide

Returns 1, 2, or 1 + 2
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BoxOnPlaneSide(
    mut emins: *mut crate::src::qcommon::q_shared::vec_t,
    mut emaxs: *mut crate::src::qcommon::q_shared::vec_t,
    mut p: *mut crate::src::qcommon::q_shared::cplane_s,
) -> libc::c_int {
    let mut dist: [libc::c_float; 2] = [0.; 2];
    let mut sides: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    // fast axial cases
    if ((*p).type_0 as libc::c_int) < 3 as libc::c_int {
        if (*p).dist <= *emins.offset((*p).type_0 as isize) {
            return 1 as libc::c_int;
        }
        if (*p).dist >= *emaxs.offset((*p).type_0 as isize) {
            return 2 as libc::c_int;
        }
        return 3 as libc::c_int;
    }
    // general case
    dist[1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_float;
    dist[0 as libc::c_int as usize] = dist[1 as libc::c_int as usize];
    if ((*p).signbits as libc::c_int) < 8 as libc::c_int {
        // >= 8: default case is original code (dist[0]=dist[1]=0)
        i = 0 as libc::c_int;
        while i < 3 as libc::c_int {
            b = (*p).signbits as libc::c_int >> i & 1 as libc::c_int;
            dist[b as usize] += (*p).normal[i as usize] * *emaxs.offset(i as isize);
            dist[(b == 0) as libc::c_int as usize] +=
                (*p).normal[i as usize] * *emins.offset(i as isize);
            i += 1
        }
    }
    sides = 0 as libc::c_int;
    if dist[0 as libc::c_int as usize] >= (*p).dist {
        sides = 1 as libc::c_int
    }
    if dist[1 as libc::c_int as usize] < (*p).dist {
        sides |= 2 as libc::c_int
    }
    return sides;
}
/*
=================
RadiusFromBounds
=================
*/
#[no_mangle]

pub unsafe extern "C" fn RadiusFromBounds(
    mut mins: *const crate::src::qcommon::q_shared::vec_t,
    mut maxs: *const crate::src::qcommon::q_shared::vec_t,
) -> libc::c_float {
    let mut i: libc::c_int = 0;
    let mut corner: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut a: libc::c_float = 0.;
    let mut b: libc::c_float = 0.;
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        a = crate::stdlib::fabs(*mins.offset(i as isize) as libc::c_double) as libc::c_float;
        b = crate::stdlib::fabs(*maxs.offset(i as isize) as libc::c_double) as libc::c_float;
        corner[i as usize] = if a > b { a } else { b };
        i += 1
    }
    return VectorLength(corner.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t);
}
#[no_mangle]

pub unsafe extern "C" fn ClearBounds(
    mut mins: *mut crate::src::qcommon::q_shared::vec_t,
    mut maxs: *mut crate::src::qcommon::q_shared::vec_t,
) {
    let ref mut fresh2 = *mins.offset(2 as libc::c_int as isize);
    *fresh2 = 99999 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
    let ref mut fresh3 = *mins.offset(1 as libc::c_int as isize);
    *fresh3 = *fresh2;
    *mins.offset(0 as libc::c_int as isize) = *fresh3;
    let ref mut fresh4 = *maxs.offset(2 as libc::c_int as isize);
    *fresh4 = -(99999 as libc::c_int) as crate::src::qcommon::q_shared::vec_t;
    let ref mut fresh5 = *maxs.offset(1 as libc::c_int as isize);
    *fresh5 = *fresh4;
    *maxs.offset(0 as libc::c_int as isize) = *fresh5;
}
#[no_mangle]

pub unsafe extern "C" fn AddPointToBounds(
    mut v: *const crate::src::qcommon::q_shared::vec_t,
    mut mins: *mut crate::src::qcommon::q_shared::vec_t,
    mut maxs: *mut crate::src::qcommon::q_shared::vec_t,
) {
    if *v.offset(0 as libc::c_int as isize) < *mins.offset(0 as libc::c_int as isize) {
        *mins.offset(0 as libc::c_int as isize) = *v.offset(0 as libc::c_int as isize)
    }
    if *v.offset(0 as libc::c_int as isize) > *maxs.offset(0 as libc::c_int as isize) {
        *maxs.offset(0 as libc::c_int as isize) = *v.offset(0 as libc::c_int as isize)
    }
    if *v.offset(1 as libc::c_int as isize) < *mins.offset(1 as libc::c_int as isize) {
        *mins.offset(1 as libc::c_int as isize) = *v.offset(1 as libc::c_int as isize)
    }
    if *v.offset(1 as libc::c_int as isize) > *maxs.offset(1 as libc::c_int as isize) {
        *maxs.offset(1 as libc::c_int as isize) = *v.offset(1 as libc::c_int as isize)
    }
    if *v.offset(2 as libc::c_int as isize) < *mins.offset(2 as libc::c_int as isize) {
        *mins.offset(2 as libc::c_int as isize) = *v.offset(2 as libc::c_int as isize)
    }
    if *v.offset(2 as libc::c_int as isize) > *maxs.offset(2 as libc::c_int as isize) {
        *maxs.offset(2 as libc::c_int as isize) = *v.offset(2 as libc::c_int as isize)
    };
}
#[no_mangle]

pub unsafe extern "C" fn BoundsIntersect(
    mut mins: *const crate::src::qcommon::q_shared::vec_t,
    mut maxs: *const crate::src::qcommon::q_shared::vec_t,
    mut mins2: *const crate::src::qcommon::q_shared::vec_t,
    mut maxs2: *const crate::src::qcommon::q_shared::vec_t,
) -> crate::src::qcommon::q_shared::qboolean {
    if *maxs.offset(0 as libc::c_int as isize) < *mins2.offset(0 as libc::c_int as isize)
        || *maxs.offset(1 as libc::c_int as isize) < *mins2.offset(1 as libc::c_int as isize)
        || *maxs.offset(2 as libc::c_int as isize) < *mins2.offset(2 as libc::c_int as isize)
        || *mins.offset(0 as libc::c_int as isize) > *maxs2.offset(0 as libc::c_int as isize)
        || *mins.offset(1 as libc::c_int as isize) > *maxs2.offset(1 as libc::c_int as isize)
        || *mins.offset(2 as libc::c_int as isize) > *maxs2.offset(2 as libc::c_int as isize)
    {
        return crate::src::qcommon::q_shared::qfalse;
    }
    return crate::src::qcommon::q_shared::qtrue;
}
#[no_mangle]

pub unsafe extern "C" fn BoundsIntersectSphere(
    mut mins: *const crate::src::qcommon::q_shared::vec_t,
    mut maxs: *const crate::src::qcommon::q_shared::vec_t,
    mut origin: *const crate::src::qcommon::q_shared::vec_t,
    mut radius: crate::src::qcommon::q_shared::vec_t,
) -> crate::src::qcommon::q_shared::qboolean {
    if *origin.offset(0 as libc::c_int as isize) - radius > *maxs.offset(0 as libc::c_int as isize)
        || *origin.offset(0 as libc::c_int as isize) + radius
            < *mins.offset(0 as libc::c_int as isize)
        || *origin.offset(1 as libc::c_int as isize) - radius
            > *maxs.offset(1 as libc::c_int as isize)
        || *origin.offset(1 as libc::c_int as isize) + radius
            < *mins.offset(1 as libc::c_int as isize)
        || *origin.offset(2 as libc::c_int as isize) - radius
            > *maxs.offset(2 as libc::c_int as isize)
        || *origin.offset(2 as libc::c_int as isize) + radius
            < *mins.offset(2 as libc::c_int as isize)
    {
        return crate::src::qcommon::q_shared::qfalse;
    }
    return crate::src::qcommon::q_shared::qtrue;
}
#[no_mangle]

pub unsafe extern "C" fn BoundsIntersectPoint(
    mut mins: *const crate::src::qcommon::q_shared::vec_t,
    mut maxs: *const crate::src::qcommon::q_shared::vec_t,
    mut origin: *const crate::src::qcommon::q_shared::vec_t,
) -> crate::src::qcommon::q_shared::qboolean {
    if *origin.offset(0 as libc::c_int as isize) > *maxs.offset(0 as libc::c_int as isize)
        || *origin.offset(0 as libc::c_int as isize) < *mins.offset(0 as libc::c_int as isize)
        || *origin.offset(1 as libc::c_int as isize) > *maxs.offset(1 as libc::c_int as isize)
        || *origin.offset(1 as libc::c_int as isize) < *mins.offset(1 as libc::c_int as isize)
        || *origin.offset(2 as libc::c_int as isize) > *maxs.offset(2 as libc::c_int as isize)
        || *origin.offset(2 as libc::c_int as isize) < *mins.offset(2 as libc::c_int as isize)
    {
        return crate::src::qcommon::q_shared::qfalse;
    }
    return crate::src::qcommon::q_shared::qtrue;
}
#[no_mangle]

pub unsafe extern "C" fn VectorNormalize(
    mut v: *mut crate::src::qcommon::q_shared::vec_t,
) -> crate::src::qcommon::q_shared::vec_t {
    // NOTE: TTimo - Apple G4 altivec source uses double?
    let mut length: libc::c_float = 0.;
    let mut ilength: libc::c_float = 0.;
    length = *v.offset(0 as libc::c_int as isize) * *v.offset(0 as libc::c_int as isize)
        + *v.offset(1 as libc::c_int as isize) * *v.offset(1 as libc::c_int as isize)
        + *v.offset(2 as libc::c_int as isize) * *v.offset(2 as libc::c_int as isize);
    if length != 0. {
        /* writing it this way allows gcc to recognize that rsqrt can be used */
        ilength = 1 as libc::c_int as libc::c_float
            / crate::stdlib::sqrt(length as libc::c_double) as libc::c_float;
        /* sqrt(length) = length * (1 / sqrt(length)) */
        length *= ilength;
        let ref mut fresh6 = *v.offset(0 as libc::c_int as isize);
        *fresh6 *= ilength;
        let ref mut fresh7 = *v.offset(1 as libc::c_int as isize);
        *fresh7 *= ilength;
        let ref mut fresh8 = *v.offset(2 as libc::c_int as isize);
        *fresh8 *= ilength
    }
    return length;
}
#[no_mangle]

pub unsafe extern "C" fn VectorNormalize2(
    mut v: *const crate::src::qcommon::q_shared::vec_t,
    mut out: *mut crate::src::qcommon::q_shared::vec_t,
) -> crate::src::qcommon::q_shared::vec_t {
    let mut length: libc::c_float = 0.;
    let mut ilength: libc::c_float = 0.;
    length = *v.offset(0 as libc::c_int as isize) * *v.offset(0 as libc::c_int as isize)
        + *v.offset(1 as libc::c_int as isize) * *v.offset(1 as libc::c_int as isize)
        + *v.offset(2 as libc::c_int as isize) * *v.offset(2 as libc::c_int as isize);
    if length != 0. {
        /* writing it this way allows gcc to recognize that rsqrt can be used */
        ilength = 1 as libc::c_int as libc::c_float
            / crate::stdlib::sqrt(length as libc::c_double) as libc::c_float;
        /* sqrt(length) = length * (1 / sqrt(length)) */
        length *= ilength;
        *out.offset(0 as libc::c_int as isize) = *v.offset(0 as libc::c_int as isize) * ilength;
        *out.offset(1 as libc::c_int as isize) = *v.offset(1 as libc::c_int as isize) * ilength;
        *out.offset(2 as libc::c_int as isize) = *v.offset(2 as libc::c_int as isize) * ilength
    } else {
        let ref mut fresh9 = *out.offset(2 as libc::c_int as isize);
        *fresh9 = 0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
        let ref mut fresh10 = *out.offset(1 as libc::c_int as isize);
        *fresh10 = *fresh9;
        *out.offset(0 as libc::c_int as isize) = *fresh10
    }
    return length;
}
#[no_mangle]

pub unsafe extern "C" fn _VectorMA(
    mut veca: *const crate::src::qcommon::q_shared::vec_t,
    mut scale: libc::c_float,
    mut vecb: *const crate::src::qcommon::q_shared::vec_t,
    mut vecc: *mut crate::src::qcommon::q_shared::vec_t,
) {
    *vecc.offset(0 as libc::c_int as isize) =
        *veca.offset(0 as libc::c_int as isize) + scale * *vecb.offset(0 as libc::c_int as isize);
    *vecc.offset(1 as libc::c_int as isize) =
        *veca.offset(1 as libc::c_int as isize) + scale * *vecb.offset(1 as libc::c_int as isize);
    *vecc.offset(2 as libc::c_int as isize) =
        *veca.offset(2 as libc::c_int as isize) + scale * *vecb.offset(2 as libc::c_int as isize);
}
#[no_mangle]

pub unsafe extern "C" fn _DotProduct(
    mut v1: *const crate::src::qcommon::q_shared::vec_t,
    mut v2: *const crate::src::qcommon::q_shared::vec_t,
) -> crate::src::qcommon::q_shared::vec_t {
    return *v1.offset(0 as libc::c_int as isize) * *v2.offset(0 as libc::c_int as isize)
        + *v1.offset(1 as libc::c_int as isize) * *v2.offset(1 as libc::c_int as isize)
        + *v1.offset(2 as libc::c_int as isize) * *v2.offset(2 as libc::c_int as isize);
}
#[no_mangle]

pub unsafe extern "C" fn _VectorSubtract(
    mut veca: *const crate::src::qcommon::q_shared::vec_t,
    mut vecb: *const crate::src::qcommon::q_shared::vec_t,
    mut out: *mut crate::src::qcommon::q_shared::vec_t,
) {
    *out.offset(0 as libc::c_int as isize) =
        *veca.offset(0 as libc::c_int as isize) - *vecb.offset(0 as libc::c_int as isize);
    *out.offset(1 as libc::c_int as isize) =
        *veca.offset(1 as libc::c_int as isize) - *vecb.offset(1 as libc::c_int as isize);
    *out.offset(2 as libc::c_int as isize) =
        *veca.offset(2 as libc::c_int as isize) - *vecb.offset(2 as libc::c_int as isize);
}
#[no_mangle]

pub unsafe extern "C" fn _VectorAdd(
    mut veca: *const crate::src::qcommon::q_shared::vec_t,
    mut vecb: *const crate::src::qcommon::q_shared::vec_t,
    mut out: *mut crate::src::qcommon::q_shared::vec_t,
) {
    *out.offset(0 as libc::c_int as isize) =
        *veca.offset(0 as libc::c_int as isize) + *vecb.offset(0 as libc::c_int as isize);
    *out.offset(1 as libc::c_int as isize) =
        *veca.offset(1 as libc::c_int as isize) + *vecb.offset(1 as libc::c_int as isize);
    *out.offset(2 as libc::c_int as isize) =
        *veca.offset(2 as libc::c_int as isize) + *vecb.offset(2 as libc::c_int as isize);
}
#[no_mangle]

pub unsafe extern "C" fn _VectorCopy(
    mut in_0: *const crate::src::qcommon::q_shared::vec_t,
    mut out: *mut crate::src::qcommon::q_shared::vec_t,
) {
    *out.offset(0 as libc::c_int as isize) = *in_0.offset(0 as libc::c_int as isize);
    *out.offset(1 as libc::c_int as isize) = *in_0.offset(1 as libc::c_int as isize);
    *out.offset(2 as libc::c_int as isize) = *in_0.offset(2 as libc::c_int as isize);
}
#[no_mangle]

pub unsafe extern "C" fn _VectorScale(
    mut in_0: *const crate::src::qcommon::q_shared::vec_t,
    mut scale: crate::src::qcommon::q_shared::vec_t,
    mut out: *mut crate::src::qcommon::q_shared::vec_t,
) {
    *out.offset(0 as libc::c_int as isize) = *in_0.offset(0 as libc::c_int as isize) * scale;
    *out.offset(1 as libc::c_int as isize) = *in_0.offset(1 as libc::c_int as isize) * scale;
    *out.offset(2 as libc::c_int as isize) = *in_0.offset(2 as libc::c_int as isize) * scale;
}
#[no_mangle]

pub unsafe extern "C" fn Vector4Scale(
    mut in_0: *const crate::src::qcommon::q_shared::vec_t,
    mut scale: crate::src::qcommon::q_shared::vec_t,
    mut out: *mut crate::src::qcommon::q_shared::vec_t,
) {
    *out.offset(0 as libc::c_int as isize) = *in_0.offset(0 as libc::c_int as isize) * scale;
    *out.offset(1 as libc::c_int as isize) = *in_0.offset(1 as libc::c_int as isize) * scale;
    *out.offset(2 as libc::c_int as isize) = *in_0.offset(2 as libc::c_int as isize) * scale;
    *out.offset(3 as libc::c_int as isize) = *in_0.offset(3 as libc::c_int as isize) * scale;
}
#[no_mangle]

pub unsafe extern "C" fn Q_log2(mut val: libc::c_int) -> libc::c_int {
    let mut answer: libc::c_int = 0;
    answer = 0 as libc::c_int;
    loop {
        val >>= 1 as libc::c_int;
        if !(val != 0 as libc::c_int) {
            break;
        }
        answer += 1
    }
    return answer;
}
/*
=================
PlaneTypeForNormal
=================
*/
/*
int	PlaneTypeForNormal (vec3_t normal) {
    if ( normal[0] == 1.0 )
        return PLANE_X;
    if ( normal[1] == 1.0 )
        return PLANE_Y;
    if ( normal[2] == 1.0 )
        return PLANE_Z;

    return PLANE_NON_AXIAL;
}
*/
/*
================
MatrixMultiply
================
*/
#[no_mangle]

pub unsafe extern "C" fn MatrixMultiply(
    mut in1: *mut [libc::c_float; 3],
    mut in2: *mut [libc::c_float; 3],
    mut out: *mut [libc::c_float; 3],
) {
    (*out.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize] = (*in1
        .offset(0 as libc::c_int as isize))[0 as libc::c_int as usize]
        * (*in2.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize]
        + (*in1.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize]
            * (*in2.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize]
        + (*in1.offset(0 as libc::c_int as isize))[2 as libc::c_int as usize]
            * (*in2.offset(2 as libc::c_int as isize))[0 as libc::c_int as usize];
    (*out.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize] = (*in1
        .offset(0 as libc::c_int as isize))[0 as libc::c_int as usize]
        * (*in2.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize]
        + (*in1.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize]
            * (*in2.offset(1 as libc::c_int as isize))[1 as libc::c_int as usize]
        + (*in1.offset(0 as libc::c_int as isize))[2 as libc::c_int as usize]
            * (*in2.offset(2 as libc::c_int as isize))[1 as libc::c_int as usize];
    (*out.offset(0 as libc::c_int as isize))[2 as libc::c_int as usize] = (*in1
        .offset(0 as libc::c_int as isize))[0 as libc::c_int as usize]
        * (*in2.offset(0 as libc::c_int as isize))[2 as libc::c_int as usize]
        + (*in1.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize]
            * (*in2.offset(1 as libc::c_int as isize))[2 as libc::c_int as usize]
        + (*in1.offset(0 as libc::c_int as isize))[2 as libc::c_int as usize]
            * (*in2.offset(2 as libc::c_int as isize))[2 as libc::c_int as usize];
    (*out.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize] = (*in1
        .offset(1 as libc::c_int as isize))[0 as libc::c_int as usize]
        * (*in2.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize]
        + (*in1.offset(1 as libc::c_int as isize))[1 as libc::c_int as usize]
            * (*in2.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize]
        + (*in1.offset(1 as libc::c_int as isize))[2 as libc::c_int as usize]
            * (*in2.offset(2 as libc::c_int as isize))[0 as libc::c_int as usize];
    (*out.offset(1 as libc::c_int as isize))[1 as libc::c_int as usize] = (*in1
        .offset(1 as libc::c_int as isize))[0 as libc::c_int as usize]
        * (*in2.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize]
        + (*in1.offset(1 as libc::c_int as isize))[1 as libc::c_int as usize]
            * (*in2.offset(1 as libc::c_int as isize))[1 as libc::c_int as usize]
        + (*in1.offset(1 as libc::c_int as isize))[2 as libc::c_int as usize]
            * (*in2.offset(2 as libc::c_int as isize))[1 as libc::c_int as usize];
    (*out.offset(1 as libc::c_int as isize))[2 as libc::c_int as usize] = (*in1
        .offset(1 as libc::c_int as isize))[0 as libc::c_int as usize]
        * (*in2.offset(0 as libc::c_int as isize))[2 as libc::c_int as usize]
        + (*in1.offset(1 as libc::c_int as isize))[1 as libc::c_int as usize]
            * (*in2.offset(1 as libc::c_int as isize))[2 as libc::c_int as usize]
        + (*in1.offset(1 as libc::c_int as isize))[2 as libc::c_int as usize]
            * (*in2.offset(2 as libc::c_int as isize))[2 as libc::c_int as usize];
    (*out.offset(2 as libc::c_int as isize))[0 as libc::c_int as usize] = (*in1
        .offset(2 as libc::c_int as isize))[0 as libc::c_int as usize]
        * (*in2.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize]
        + (*in1.offset(2 as libc::c_int as isize))[1 as libc::c_int as usize]
            * (*in2.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize]
        + (*in1.offset(2 as libc::c_int as isize))[2 as libc::c_int as usize]
            * (*in2.offset(2 as libc::c_int as isize))[0 as libc::c_int as usize];
    (*out.offset(2 as libc::c_int as isize))[1 as libc::c_int as usize] = (*in1
        .offset(2 as libc::c_int as isize))[0 as libc::c_int as usize]
        * (*in2.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize]
        + (*in1.offset(2 as libc::c_int as isize))[1 as libc::c_int as usize]
            * (*in2.offset(1 as libc::c_int as isize))[1 as libc::c_int as usize]
        + (*in1.offset(2 as libc::c_int as isize))[2 as libc::c_int as usize]
            * (*in2.offset(2 as libc::c_int as isize))[1 as libc::c_int as usize];
    (*out.offset(2 as libc::c_int as isize))[2 as libc::c_int as usize] = (*in1
        .offset(2 as libc::c_int as isize))[0 as libc::c_int as usize]
        * (*in2.offset(0 as libc::c_int as isize))[2 as libc::c_int as usize]
        + (*in1.offset(2 as libc::c_int as isize))[1 as libc::c_int as usize]
            * (*in2.offset(1 as libc::c_int as isize))[2 as libc::c_int as usize]
        + (*in1.offset(2 as libc::c_int as isize))[2 as libc::c_int as usize]
            * (*in2.offset(2 as libc::c_int as isize))[2 as libc::c_int as usize];
}
#[no_mangle]

pub unsafe extern "C" fn AngleVectors(
    mut angles: *const crate::src::qcommon::q_shared::vec_t,
    mut forward: *mut crate::src::qcommon::q_shared::vec_t,
    mut right: *mut crate::src::qcommon::q_shared::vec_t,
    mut up: *mut crate::src::qcommon::q_shared::vec_t,
) {
    let mut angle: libc::c_float = 0.;
    static mut sr: libc::c_float = 0.;
    static mut sp: libc::c_float = 0.;
    static mut sy: libc::c_float = 0.;
    static mut cr: libc::c_float = 0.;
    static mut cp: libc::c_float = 0.;
    static mut cy: libc::c_float = 0.;
    // static to help MS compiler fp bugs
    angle = (*angles.offset(1 as libc::c_int as isize) as libc::c_double
        * (3.14159265358979323846f64 * 2 as libc::c_int as libc::c_double
            / 360 as libc::c_int as libc::c_double)) as libc::c_float;
    sy = crate::stdlib::sin(angle as libc::c_double) as libc::c_float;
    cy = crate::stdlib::cos(angle as libc::c_double) as libc::c_float;
    angle = (*angles.offset(0 as libc::c_int as isize) as libc::c_double
        * (3.14159265358979323846f64 * 2 as libc::c_int as libc::c_double
            / 360 as libc::c_int as libc::c_double)) as libc::c_float;
    sp = crate::stdlib::sin(angle as libc::c_double) as libc::c_float;
    cp = crate::stdlib::cos(angle as libc::c_double) as libc::c_float;
    angle = (*angles.offset(2 as libc::c_int as isize) as libc::c_double
        * (3.14159265358979323846f64 * 2 as libc::c_int as libc::c_double
            / 360 as libc::c_int as libc::c_double)) as libc::c_float;
    sr = crate::stdlib::sin(angle as libc::c_double) as libc::c_float;
    cr = crate::stdlib::cos(angle as libc::c_double) as libc::c_float;
    if !forward.is_null() {
        *forward.offset(0 as libc::c_int as isize) = cp * cy;
        *forward.offset(1 as libc::c_int as isize) = cp * sy;
        *forward.offset(2 as libc::c_int as isize) = -sp
    }
    if !right.is_null() {
        *right.offset(0 as libc::c_int as isize) =
            -(1 as libc::c_int) as libc::c_float * sr * sp * cy
                + -(1 as libc::c_int) as libc::c_float * cr * -sy;
        *right.offset(1 as libc::c_int as isize) =
            -(1 as libc::c_int) as libc::c_float * sr * sp * sy
                + -(1 as libc::c_int) as libc::c_float * cr * cy;
        *right.offset(2 as libc::c_int as isize) = -(1 as libc::c_int) as libc::c_float * sr * cp
    }
    if !up.is_null() {
        *up.offset(0 as libc::c_int as isize) = cr * sp * cy + -sr * -sy;
        *up.offset(1 as libc::c_int as isize) = cr * sp * sy + -sr * cy;
        *up.offset(2 as libc::c_int as isize) = cr * cp
    };
}
// perpendicular vector could be replaced by this
//int	PlaneTypeForNormal (vec3_t normal);
/*
** assumes "src" is normalized
*/
#[no_mangle]

pub unsafe extern "C" fn PerpendicularVector(
    mut dst: *mut crate::src::qcommon::q_shared::vec_t,
    mut src: *const crate::src::qcommon::q_shared::vec_t,
) {
    let mut pos: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut minelem: libc::c_float = 1.0f32;
    let mut tempvec: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    /*
    	** find the smallest magnitude axially aligned vector
    	*/
    pos = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        if crate::stdlib::fabs(*src.offset(i as isize) as libc::c_double)
            < minelem as libc::c_double
        {
            pos = i;
            minelem =
                crate::stdlib::fabs(*src.offset(i as isize) as libc::c_double) as libc::c_float
        }
        i += 1
    }
    tempvec[2 as libc::c_int as usize] = 0.0f32;
    tempvec[1 as libc::c_int as usize] = tempvec[2 as libc::c_int as usize];
    tempvec[0 as libc::c_int as usize] = tempvec[1 as libc::c_int as usize];
    tempvec[pos as usize] = 1.0f32;
    /*
    	** project the point onto the plane defined by src
    	*/
    ProjectPointOnPlane(
        dst,
        tempvec.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        src,
    );
    /*
    	** normalize the result
    	*/
    VectorNormalize(dst);
}
/*
================
Q_isnan

Don't pass doubles to this
================
*/
#[no_mangle]

pub unsafe extern "C" fn Q_isnan(mut x: libc::c_float) -> libc::c_int {
    let mut fi: crate::src::qcommon::q_shared::floatint_t =
        crate::src::qcommon::q_shared::floatint_t { f: 0. };
    fi.f = x;
    fi.ui &= 0x7fffffff as libc::c_int as libc::c_uint;
    fi.ui = (0x7f800000 as libc::c_int as libc::c_uint).wrapping_sub(fi.ui);
    return (fi.ui >> 31 as libc::c_int) as libc::c_int;
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
//
// q_shared.h -- included first by ALL program modules.
// A user mod should never modify this file
// Heartbeat for dpmaster protocol. You shouldn't change this unless you know what you're doing
// When com_gamename is LEGACY_MASTER_GAMENAME, use quake3 master protocol.
// You shouldn't change this unless you know what you're doing
// number of supported master servers
// standard demo extension
//Ignore __attribute__ on non-gcc platforms
/* *********************************************************************
 VM Considerations

 The VM can not use the standard system headers because we aren't really
 using the compiler they were meant for.  We use bg_lib.h which contains
 prototypes for the functions we define for our own use in bg_lib.c.

 When writing mods, please add needed headers HERE, do not start including
 stuff like <stdio.h> in the various .c files that make up each of the VMs
 since you will be including system headers files can will have issues.

 Remember, if you use a C library function that is not defined in bg_lib.c,
 you will have to add your own version for support in the VM.

**********************************************************************/
//=============================================================
// expand constants before stringifying them
// angle indexes
// up / down
// left / right
// fall over
// the game guarantees that no string from the network will ever
// exceed MAX_STRING_CHARS
// max length of a string passed to Cmd_TokenizeString
// max tokens resulting from Cmd_TokenizeString
// max length of an individual token
// used for system info key only
// max length of a quake game pathname
// max length of a client name
// parameters for command buffer stuffing
// don't return until completed, a VM should NEVER use this,
// because some commands might cause the VM to be unloaded...
// insert at current position, but don't run yet
// add to end of the command buffer (normal case)
//
// these aren't needed by any of the VMs.  put in another header?
//
// bit vector of area visibility
// print levels from renderer (FIXME: set up for game / cgame?)
// only print when "developer 1"
// parameters to the main Error routine
// exit the entire game with a popup window
// print to console and disconnect from game
// don't kill server
// client disconnected from the server
// pop up the need-cd dialog
// font rendering values used by ui and cgame
// default
// default
/*
==============================================================

MATHLIB

==============================================================
*/
// all drawing is done to a 640*480 virtual screen size
// and will be automatically scaled to the real resolution
// ^[0-9a-zA-Z]
/*
// if your system does not have lrintf() and round() you can try this block. Please also open a bug report at bugzilla.icculus.org
// or write a mail to the ioq3 mailing list.
#else
  #define Q_ftol(v) ((long) (v))
  #define Q_round(v) do { if((v) < 0) (v) -= 0.5f; else (v) += 0.5f; (v) = Q_ftol((v)); } while(0)
  #define Q_SnapVector(vec) \
    do\
    {\
        vec3_t *temp = (vec);\
        \
        Q_round((*temp)[0]);\
        Q_round((*temp)[1]);\
        Q_round((*temp)[2]);\
    } while(0)
#endif
*/
// reciprocal square root
// this isn't a real cheap function to call!
// just in case you don't want to use the macros
// fast vector normalize routine that does not check to make sure
// that length != 0, nor does it return length, uses rsqrt approximation
// returns vector length
//------------------------------------------------------------------------
/*
=====================
Q_acos

the msvc acos doesn't always return a value between -PI and PI:

int i;
i = 1065353246;
acos(*(float*) &i) == -1.#IND0

=====================
*/
#[no_mangle]

pub unsafe extern "C" fn Q_acos(mut c: libc::c_float) -> libc::c_float {
    let mut angle: libc::c_float = 0.;
    angle = crate::stdlib::acos(c as libc::c_double) as libc::c_float;
    if angle as libc::c_double > 3.14159265358979323846f64 {
        return 3.14159265358979323846f64 as libc::c_float;
    }
    if (angle as libc::c_double) < -3.14159265358979323846f64 {
        return 3.14159265358979323846f64 as libc::c_float;
    }
    return angle;
}

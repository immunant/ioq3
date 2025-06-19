#!/bin/bash

find . -name '*.rs' -print0 | xargs -0 perl -pi -e 's/libc::c_ulonglong/u64/g;s/libc::c_longlong/i64/g;s/libc::c_uint/u32/g;s/libc::c_int/i32/g;s/libc::c_ushort/u16/g;s/libc::c_short/i16/g;s/libc::c_uchar/u8/g;s/libc::c_schar/i8/g;s/libc::c_float/f32/g;s/libc::c_double/f64/g;s/libc::intptr_t/isize/g;s/libc::uintptr_t/usize/g;'

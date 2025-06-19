use ::libc;
/* *******************************************************************
 *                                                                  *
 * THIS FILE IS PART OF THE libopusfile SOFTWARE CODEC SOURCE CODE. *
 * USE, DISTRIBUTION AND REPRODUCTION OF THIS LIBRARY SOURCE IS     *
 * GOVERNED BY A BSD-STYLE SOURCE LICENSE INCLUDED WITH THIS SOURCE *
 * IN 'COPYING'. PLEASE READ THESE TERMS BEFORE DISTRIBUTING.       *
 *                                                                  *
 * THE libopusfile SOURCE CODE IS (C) COPYRIGHT 2012                *
 * by the Xiph.Org Foundation and contributors http://www.xiph.org/ *
 *                                                                  *
 ********************************************************************/
/*A version of strncasecmp() that is guaranteed to only ignore the case of
ASCII characters.*/
#[no_mangle]

pub unsafe extern "C" fn op_strncasecmp(
    mut _a: *const libc::c_char,
    mut _b: *const libc::c_char,
    mut _n: i32,
) -> i32 {
    let mut i: i32 = 0;
    i = 0 as i32;
    while i < _n {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        let mut d: i32 = 0;
        a = *_a.offset(i as isize) as i32;
        b = *_b.offset(i as isize) as i32;
        if a >= 'a' as i32 && a <= 'z' as i32 {
            a -= 'a' as i32 - 'A' as i32
        }
        if b >= 'a' as i32 && b <= 'z' as i32 {
            b -= 'a' as i32 - 'A' as i32
        }
        d = a - b;
        if d != 0 {
            return d;
        }
        i += 1
    }
    return 0 as i32;
}

use ::libc;

pub use crate::opus_types_h::opus_uint32;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::uint32_t;
/* Copyright (c) 2002-2008 Jean-Marc Valin
Copyright (c) 2007-2008 CSIRO
Copyright (c) 2007-2009 Xiph.Org Foundation
Written by Jean-Marc Valin */
/* *
   @file mathops.h
   @brief Various math functions
*/
/*
   Redistribution and use in source and binary forms, with or without
   modification, are permitted provided that the following conditions
   are met:

   - Redistributions of source code must retain the above copyright
   notice, this list of conditions and the following disclaimer.

   - Redistributions in binary form must reproduce the above copyright
   notice, this list of conditions and the following disclaimer in the
   documentation and/or other materials provided with the distribution.

   THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
   ``AS IS'' AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
   LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
   A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER
   OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
   EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
   PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
   PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
   LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
   NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
   SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
*/
/* Multiplies two 16-bit fractional values. Bit-exactness of this macro is important */
/* Copyright (c) 2002-2008 Jean-Marc Valin
Copyright (c) 2007-2008 CSIRO
Copyright (c) 2007-2009 Xiph.Org Foundation
Written by Jean-Marc Valin */
/* *
   @file mathops.h
   @brief Various math functions
*/
/*
   Redistribution and use in source and binary forms, with or without
   modification, are permitted provided that the following conditions
   are met:

   - Redistributions of source code must retain the above copyright
   notice, this list of conditions and the following disclaimer.

   - Redistributions in binary form must reproduce the above copyright
   notice, this list of conditions and the following disclaimer in the
   documentation and/or other materials provided with the distribution.

   THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
   ``AS IS'' AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
   LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
   A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER
   OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
   EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
   PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
   PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
   LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
   NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
   SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
*/
/*Compute floor(sqrt(_val)) with exact arithmetic.
This has been tested on all possible 32-bit inputs.*/
#[no_mangle]

pub unsafe extern "C" fn isqrt32(mut _val: opus_uint32) -> u32 {
    let mut b: u32 = 0;
    let mut g: u32 = 0;
    let mut bshift: i32 = 0;
    /*Uses the second method from
     http://www.azillionmonkeys.com/qed/sqroot.html
    The main idea is to search for the largest binary digit b such that
     (g+b)*(g+b) <= _val, and add it to the solution g.*/
    g = 0 as i32 as u32;
    bshift = ::std::mem::size_of::<u32>() as usize as i32 * 8 as i32
        - _val.leading_zeros() as i32
        - 1 as i32
        >> 1 as i32;
    b = (1 as u32) << bshift;
    loop {
        let mut t: opus_uint32 = 0;
        t = (g << 1 as i32).wrapping_add(b) << bshift;
        if t <= _val {
            g = g.wrapping_add(b);
            _val = (_val as u32).wrapping_sub(t) as opus_uint32 as opus_uint32
        }
        b >>= 1 as i32;
        bshift -= 1;
        if !(bshift >= 0 as i32) {
            break;
        }
    }
    return g;
}

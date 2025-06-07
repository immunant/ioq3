use ::libc;

pub use crate::stdlib::__int32_t;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::int32_t;
pub use crate::stdlib::uint32_t;

pub use crate::opus_types_h::opus_int32;
pub use crate::opus_types_h::opus_uint32;
pub use crate::src::opus_1_2_1::celt::entcode::ec_ctx;
pub use crate::src::opus_1_2_1::celt::entcode::ec_dec;
pub use crate::src::opus_1_2_1::celt::entcode::ec_enc;
pub use crate::src::opus_1_2_1::celt::entcode::ec_window;

/* When called, decay is positive and at most 11456. */

unsafe extern "C" fn ec_laplace_get_freq1(
    mut fs0: libc::c_uint,
    mut decay: libc::c_int,
) -> libc::c_uint {
    let mut ft: libc::c_uint = 0;
    ft = ((32768 as libc::c_int
        - ((1 as libc::c_int) << 0 as libc::c_int) * (2 as libc::c_int * 16 as libc::c_int))
        as libc::c_uint)
        .wrapping_sub(fs0);
    return ft.wrapping_mul((16384 as libc::c_int - decay) as libc::c_uint) >> 15 as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn ec_laplace_encode(
    mut enc: *mut crate::src::opus_1_2_1::celt::entcode::ec_enc,
    mut value: *mut libc::c_int,
    mut fs: libc::c_uint,
    mut decay: libc::c_int,
) {
    let mut fl: libc::c_uint = 0;
    let mut val: libc::c_int = *value;
    fl = 0 as libc::c_int as libc::c_uint;
    if val != 0 {
        let mut s: libc::c_int = 0;
        let mut i: libc::c_int = 0;
        s = -((val < 0 as libc::c_int) as libc::c_int);
        val = val + s ^ s;
        fl = fs;
        fs = ec_laplace_get_freq1(fs, decay);
        /* Search the decaying part of the PDF.*/
        i = 1 as libc::c_int;
        while fs > 0 as libc::c_int as libc::c_uint && i < val {
            fs = fs.wrapping_mul(2 as libc::c_int as libc::c_uint);
            fl = fl.wrapping_add(fs.wrapping_add(
                (2 as libc::c_int * ((1 as libc::c_int) << 0 as libc::c_int)) as libc::c_uint,
            ));
            fs = fs.wrapping_mul(decay as libc::c_uint) >> 15 as libc::c_int;
            i += 1
        }
        /* Everything beyond that has probability LAPLACE_MINP. */
        if fs == 0 {
            let mut di: libc::c_int = 0;
            let mut ndi_max: libc::c_int = 0;
            ndi_max = ((32768 as libc::c_int as libc::c_uint)
                .wrapping_sub(fl)
                .wrapping_add(((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint)
                .wrapping_sub(1 as libc::c_int as libc::c_uint)
                >> 0 as libc::c_int) as libc::c_int;
            ndi_max = ndi_max - s >> 1 as libc::c_int;
            di = if val - i < ndi_max - 1 as libc::c_int {
                (val) - i
            } else {
                (ndi_max) - 1 as libc::c_int
            };
            fl = fl.wrapping_add(
                ((2 as libc::c_int * di + 1 as libc::c_int + s)
                    * ((1 as libc::c_int) << 0 as libc::c_int)) as libc::c_uint,
            );
            fs = if (((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint)
                < (32768 as libc::c_int as libc::c_uint).wrapping_sub(fl)
            {
                ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint
            } else {
                (32768 as libc::c_int as libc::c_uint).wrapping_sub(fl)
            };
            *value = i + di + s ^ s
        } else {
            fs = fs.wrapping_add(((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint);
            fl = fl.wrapping_add(fs & !s as libc::c_uint)
        }
    }
    crate::src::opus_1_2_1::celt::entenc::ec_encode_bin(
        enc as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
        fl,
        fl.wrapping_add(fs),
        15 as libc::c_int as libc::c_uint,
    );
}
/* Copyright (c) 2007 CSIRO
Copyright (c) 2007-2009 Xiph.Org Foundation
Written by Jean-Marc Valin */
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
/* * Encode a value that is assumed to be the realisation of a
    Laplace-distributed random process
 @param enc Entropy encoder state
 @param value Value to encode
 @param fs Probability of 0, multiplied by 32768
 @param decay Probability of the value +/- 1, multiplied by 16384
*/
/* * Decode a value that is assumed to be the realisation of a
   Laplace-distributed random process
@param dec Entropy decoder state
@param fs Probability of 0, multiplied by 32768
@param decay Probability of the value +/- 1, multiplied by 16384
@return Value decoded
*/
#[no_mangle]

pub unsafe extern "C" fn ec_laplace_decode(
    mut dec: *mut crate::src::opus_1_2_1::celt::entcode::ec_dec,
    mut fs: libc::c_uint,
    mut decay: libc::c_int,
) -> libc::c_int {
    let mut val: libc::c_int = 0 as libc::c_int;
    let mut fl: libc::c_uint = 0;
    let mut fm: libc::c_uint = 0;
    fm = crate::src::opus_1_2_1::celt::entdec::ec_decode_bin(
        dec as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
        15 as libc::c_int as libc::c_uint,
    );
    fl = 0 as libc::c_int as libc::c_uint;
    if fm >= fs {
        val += 1;
        fl = fs;
        fs = ec_laplace_get_freq1(fs, decay)
            .wrapping_add(((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint);
        /* Search the decaying part of the PDF.*/
        while fs > ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint
            && fm >= fl.wrapping_add((2 as libc::c_int as libc::c_uint).wrapping_mul(fs))
        {
            fs = fs.wrapping_mul(2 as libc::c_int as libc::c_uint);
            fl = fl.wrapping_add(fs);
            fs = fs
                .wrapping_sub(
                    (2 as libc::c_int * ((1 as libc::c_int) << 0 as libc::c_int)) as libc::c_uint,
                )
                .wrapping_mul(decay as libc::c_uint)
                >> 15 as libc::c_int;
            fs = fs.wrapping_add(((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint);
            val += 1
        }
        /* Everything beyond that has probability LAPLACE_MINP. */
        if fs <= ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint {
            let mut di: libc::c_int = 0;
            di = (fm.wrapping_sub(fl) >> 0 as libc::c_int + 1 as libc::c_int) as libc::c_int;
            val += di;
            fl = fl.wrapping_add(
                (2 as libc::c_int * di * ((1 as libc::c_int) << 0 as libc::c_int)) as libc::c_uint,
            )
        }
        if fm < fl.wrapping_add(fs) {
            val = -val
        } else {
            fl = fl.wrapping_add(fs)
        }
    }
    crate::src::opus_1_2_1::celt::entdec::ec_dec_update(
        dec as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
        fl,
        if fl.wrapping_add(fs) < 32768 as libc::c_int as libc::c_uint {
            fl.wrapping_add(fs)
        } else {
            32768 as libc::c_int as libc::c_uint
        },
        32768 as libc::c_int as libc::c_uint,
    );
    return val;
}

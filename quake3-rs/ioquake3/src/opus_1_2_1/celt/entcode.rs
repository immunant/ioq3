// =============== BEGIN entcode_h ================
pub type ec_window = crate::opus_types_h::opus_uint32;

pub type ec_enc = crate::src::opus_1_2_1::celt::entcode::ec_ctx;

pub type ec_dec = crate::src::opus_1_2_1::celt::entcode::ec_ctx;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_ctx {
    pub buf: *mut u8,
    pub storage: crate::opus_types_h::opus_uint32,
    pub end_offs: crate::opus_types_h::opus_uint32,
    pub end_window: crate::src::opus_1_2_1::celt::entcode::ec_window,
    pub nend_bits: i32,
    pub nbits_total: i32,
    pub offs: crate::opus_types_h::opus_uint32,
    pub rng: crate::opus_types_h::opus_uint32,
    pub val: crate::opus_types_h::opus_uint32,
    pub ext: crate::opus_types_h::opus_uint32,
    pub rem: i32,
    pub error: i32,
}

pub use crate::opus_types_h::opus_uint32;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::uint32_t;

/* Copyright (c) 2001-2011 Timothy B. Terriberry
Copyright (c) 2008-2009 Xiph.Org Foundation */
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
/*OPT: ec_window must be at least 32 bits, but if you have fast arithmetic on a
larger type, you can speed up the decoder by using it here.*/
/*The number of bits to use for the range-coded part of unsigned integers.*/
/*The resolution of fractional-precision bit usage measurements, i.e.,
3 => 1/8th bits.*/
/*The entropy encoder/decoder context.
We use the same structure for both, so that common functions like ec_tell()
 can be used on either one.*/
/*Buffered input/output.*/
/*The size of the buffer.*/
/*The offset at which the last byte containing raw bits was read/written.*/
/*Bits that will be read from/written at the end.*/
/*Number of valid bits in end_window.*/
/*The total number of whole bits read/written.
This does not include partial bits currently in the range coder.*/
/*The offset at which the next range coder byte will be read/written.*/
/*The number of values in the current range.*/
/*In the decoder: the difference between the top of the current range and
 the input value, minus one.
In the encoder: the low end of the current range.*/
/*In the decoder: the saved normalization factor from ec_decode().
In the encoder: the number of oustanding carry propagating symbols.*/
/*A buffered input/output symbol, awaiting carry propagation.*/
/*Nonzero if an error occurred.*/
/*Returns the number of bits "used" by the encoded or decoded symbols so far.
This same number can be computed in either the encoder or the decoder, and is
 suitable for making coding decisions.
Return: The number of bits.
        This will always be slightly larger than the exact value (e.g., all
         rounding error is in the positive direction).*/
/*Returns the number of bits "used" by the encoded or decoded symbols so far.
This same number can be computed in either the encoder or the decoder, and is
 suitable for making coding decisions.
Return: The number of bits scaled by 2**BITRES.
        This will always be slightly larger than the exact value (e.g., all
         rounding error is in the positive direction).*/
/* Copyright (c) 2001-2011 Timothy B. Terriberry
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
/* This is a faster version of ec_tell_frac() that takes advantage
of the low (1/8 bit) resolution to use just a linear function
followed by a lookup to determine the exact transition thresholds. */
#[no_mangle]

pub unsafe extern "C" fn ec_tell_frac(
    mut _this: *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
) -> opus_uint32 {
    static mut correction: [u32; 8] = [
        35733 as i32 as u32,
        38967 as i32 as u32,
        42495 as i32 as u32,
        46340 as i32 as u32,
        50535 as i32 as u32,
        55109 as i32 as u32,
        60097 as i32 as u32,
        65535 as i32 as u32,
    ];
    let mut nbits: opus_uint32 = 0;
    let mut r: opus_uint32 = 0;
    let mut l: i32 = 0;
    let mut b: u32 = 0;
    nbits = ((*_this).nbits_total << 3 as i32) as opus_uint32;
    l = ::std::mem::size_of::<u32>() as usize as i32 * 8 as i32
        - (*_this).rng.leading_zeros() as i32;
    r = (*_this).rng >> l - 16 as i32;
    b = (r >> 12 as i32).wrapping_sub(8 as i32 as u32);
    b = b.wrapping_add((r > correction[b as usize]) as i32 as u32);
    l = ((l << 3 as i32) as u32).wrapping_add(b) as i32;
    return nbits.wrapping_sub(l as u32);
}

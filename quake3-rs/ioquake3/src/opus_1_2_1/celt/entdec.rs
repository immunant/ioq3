use ::libc;

pub mod entcode_h {
    /*OPT: ec_window must be at least 32 bits, but if you have fast arithmetic on a
    larger type, you can speed up the decoder by using it here.*/

    /*The number of bits to use for the range-coded part of unsigned integers.*/
    /*The resolution of fractional-precision bit usage measurements, i.e.,
    3 => 1/8th bits.*/
    /*The entropy encoder/decoder context.
    We use the same structure for both, so that common functions like ec_tell()
     can be used on either one.*/

    /* Tested exhaustively for all n and for 1<=d<=256 */
    #[inline]

    pub unsafe extern "C" fn celt_udiv(
        mut n: crate::opus_types_h::opus_uint32,
        mut d: crate::opus_types_h::opus_uint32,
    ) -> crate::opus_types_h::opus_uint32 {
        return n.wrapping_div(d);
    }
}
pub use crate::opus_types_h::opus_uint32;
pub use crate::src::opus_1_2_1::celt::entcode::ec_ctx;
pub use crate::src::opus_1_2_1::celt::entcode::ec_dec;
pub use crate::src::opus_1_2_1::celt::entcode::ec_window;
pub use crate::src::opus_1_2_1::celt::entdec::entcode_h::celt_udiv;
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
/*A range decoder.
This is an entropy decoder based upon \cite{Mar79}, which is itself a
 rediscovery of the FIFO arithmetic code introduced by \cite{Pas76}.
It is very similar to arithmetic encoding, except that encoding is done with
 digits in any base, instead of with bits, and so it is faster when using
 larger bases (i.e.: a byte).
The author claims an average waste of $\frac{1}{2}\log_b(2b)$ bits, where $b$
 is the base, longer than the theoretical optimum, but to my knowledge there
 is no published justification for this claim.
This only seems true when using near-infinite precision arithmetic so that
 the process is carried out with no rounding errors.

An excellent description of implementation details is available at
 http://www.arturocampos.com/ac_range.html
A recent work \cite{MNW98} which proposes several changes to arithmetic
 encoding for efficiency actually re-discovers many of the principles
 behind range encoding, and presents a good theoretical analysis of them.

End of stream is handled by writing out the smallest number of bits that
 ensures that the stream will be correctly decoded regardless of the value of
 any subsequent bits.
ec_tell() can be used to determine how many bits were needed to decode
 all the symbols thus far; other data can be packed in the remaining bits of
 the input buffer.
@PHDTHESIS{Pas76,
  author="Richard Clark Pasco",
  title="Source coding algorithms for fast data compression",
  school="Dept. of Electrical Engineering, Stanford University",
  address="Stanford, CA",
  month=May,
  year=1976
}
@INPROCEEDINGS{Mar79,
 author="Martin, G.N.N.",
 title="Range encoding: an algorithm for removing redundancy from a digitised
  message",
 booktitle="Video & Data Recording Conference",
 year=1979,
 address="Southampton",
 month=Jul
}
@ARTICLE{MNW98,
 author="Alistair Moffat and Radford Neal and Ian H. Witten",
 title="Arithmetic Coding Revisited",
 journal="{ACM} Transactions on Information Systems",
 year=1998,
 volume=16,
 number=3,
 pages="256--294",
 month=Jul,
 URL="http://www.stanford.edu/class/ee398a/handouts/papers/Moffat98ArithmCoding.pdf"
}*/

unsafe extern "C" fn ec_read_byte(
    mut _this: *mut ec_dec,
) -> i32 {
    return if (*_this).offs < (*_this).storage {
        let fresh0 = (*_this).offs;
        (*_this).offs = (*_this).offs.wrapping_add(1);
        *(*_this).buf.offset(fresh0 as isize) as i32
    } else {
        0 as i32
    };
}

unsafe extern "C" fn ec_read_byte_from_end(
    mut _this: *mut ec_dec,
) -> i32 {
    return if (*_this).end_offs < (*_this).storage {
        (*_this).end_offs = (*_this).end_offs.wrapping_add(1);
        *(*_this)
            .buf
            .offset((*_this).storage.wrapping_sub((*_this).end_offs) as isize) as i32
    } else {
        0 as i32
    };
}
/*Normalizes the contents of val and rng so that rng lies entirely in the
high-order symbol.*/

unsafe extern "C" fn ec_dec_normalize(
    mut _this: *mut ec_dec,
) {
    /*If the range is too small, rescale it and input some bits.*/
    while (*_this).rng <= (1 as u32) << 32 as i32 - 1 as i32 >> 8 as i32 {
        let mut sym: i32 = 0;
        (*_this).nbits_total += 8 as i32;
        (*_this).rng <<= 8 as i32;
        /*Use up the remaining bits from our last symbol.*/
        sym = (*_this).rem;
        /*Read the next value from the input.*/
        (*_this).rem = ec_read_byte(_this);
        /*Take the rest of the bits we need from this new symbol.*/
        sym = (sym << 8 as i32 | (*_this).rem)
            >> 8 as i32 - ((32 as i32 - 2 as i32) % 8 as i32 + 1 as i32);
        /*And subtract them from val, capped to be less than EC_CODE_TOP.*/
        (*_this).val = ((*_this).val << 8 as i32)
            .wrapping_add(((1 as u32) << 8 as i32).wrapping_sub(1 as i32 as u32) & !sym as u32)
            & ((1 as u32) << 32 as i32 - 1 as i32).wrapping_sub(1 as i32 as u32)
    }
}
#[no_mangle]

pub unsafe extern "C" fn ec_dec_init(
    mut _this: *mut ec_dec,
    mut _buf: *mut u8,
    mut _storage: opus_uint32,
) {
    (*_this).buf = _buf;
    (*_this).storage = _storage;
    (*_this).end_offs = 0 as i32 as opus_uint32;
    (*_this).end_window = 0 as i32 as ec_window;
    (*_this).nend_bits = 0 as i32;
    /*This is the offset from which ec_tell() will subtract partial bits.
    The final value after the ec_dec_normalize() call will be the same as in
     the encoder, but we have to compensate for the bits that are added there.*/
    (*_this).nbits_total = 32 as i32 + 1 as i32
        - (32 as i32 - ((32 as i32 - 2 as i32) % 8 as i32 + 1 as i32)) / 8 as i32 * 8 as i32;
    (*_this).offs = 0 as i32 as opus_uint32;
    (*_this).rng = (1 as u32) << (32 as i32 - 2 as i32) % 8 as i32 + 1 as i32;
    (*_this).rem = ec_read_byte(_this);
    (*_this).val = (*_this).rng.wrapping_sub(1 as i32 as u32).wrapping_sub(
        ((*_this).rem >> 8 as i32 - ((32 as i32 - 2 as i32) % 8 as i32 + 1 as i32)) as u32,
    );
    (*_this).error = 0 as i32;
    /*Normalize the interval.*/
    ec_dec_normalize(_this);
}
#[no_mangle]

pub unsafe extern "C" fn ec_decode(
    mut _this: *mut ec_dec,
    mut _ft: u32,
) -> u32 {
    let mut s: u32 = 0;
    (*_this).ext = celt_udiv((*_this).rng, _ft);
    s = (*_this).val.wrapping_div((*_this).ext);
    return _ft.wrapping_sub(s.wrapping_add(1 as i32 as u32).wrapping_add(
        _ft.wrapping_sub(s.wrapping_add(1 as i32 as u32))
            & -((_ft < s.wrapping_add(1 as i32 as u32)) as i32) as u32,
    ));
}
#[no_mangle]

pub unsafe extern "C" fn ec_decode_bin(
    mut _this: *mut ec_dec,
    mut _bits: u32,
) -> u32 {
    let mut s: u32 = 0;
    (*_this).ext = (*_this).rng >> _bits;
    s = (*_this).val.wrapping_div((*_this).ext);
    return ((1 as u32) << _bits).wrapping_sub(s.wrapping_add(1 as u32).wrapping_add(
        ((1 as u32) << _bits).wrapping_sub(s.wrapping_add(1 as u32))
            & -(((1 as u32) << _bits < s.wrapping_add(1 as u32)) as i32) as u32,
    ));
}
#[no_mangle]

pub unsafe extern "C" fn ec_dec_update(
    mut _this: *mut ec_dec,
    mut _fl: u32,
    mut _fh: u32,
    mut _ft: u32,
) {
    let mut s: opus_uint32 = 0;
    s = (*_this).ext.wrapping_mul(_ft.wrapping_sub(_fh));
    (*_this).val = ((*_this).val as u32).wrapping_sub(s) as opus_uint32;
    (*_this).rng = if _fl > 0 as i32 as u32 {
        (*_this).ext.wrapping_mul(_fh.wrapping_sub(_fl))
    } else {
        (*_this).rng.wrapping_sub(s)
    };
    ec_dec_normalize(_this);
}
/*The probability of having a "one" is 1/(1<<_logp).*/
#[no_mangle]

pub unsafe extern "C" fn ec_dec_bit_logp(
    mut _this: *mut ec_dec,
    mut _logp: u32,
) -> i32 {
    let mut r: opus_uint32 = 0;
    let mut d: opus_uint32 = 0;
    let mut s: opus_uint32 = 0;
    let mut ret: i32 = 0;
    r = (*_this).rng;
    d = (*_this).val;
    s = r >> _logp;
    ret = (d < s) as i32;
    if ret == 0 {
        (*_this).val = d.wrapping_sub(s)
    }
    (*_this).rng = if ret != 0 { s } else { r.wrapping_sub(s) };
    ec_dec_normalize(_this);
    return ret;
}
#[no_mangle]

pub unsafe extern "C" fn ec_dec_icdf(
    mut _this: *mut ec_dec,
    mut _icdf: *const u8,
    mut _ftb: u32,
) -> i32 {
    let mut r: opus_uint32 = 0;
    let mut d: opus_uint32 = 0;
    let mut s: opus_uint32 = 0;
    let mut t: opus_uint32 = 0;
    let mut ret: i32 = 0;
    s = (*_this).rng;
    d = (*_this).val;
    r = s >> _ftb;
    ret = -(1 as i32);
    loop {
        t = s;
        ret += 1;
        s = r.wrapping_mul(*_icdf.offset(ret as isize) as u32);
        if !(d < s) {
            break;
        }
    }
    (*_this).val = d.wrapping_sub(s);
    (*_this).rng = t.wrapping_sub(s);
    ec_dec_normalize(_this);
    return ret;
}
#[no_mangle]

pub unsafe extern "C" fn ec_dec_uint(
    mut _this: *mut ec_dec,
    mut _ft: opus_uint32,
) -> opus_uint32 {
    let mut ft: u32 = 0;
    let mut s: u32 = 0;
    let mut ftb: i32 = 0;
    /*In order to optimize EC_ILOG(), it is undefined for the value 0.*/
    _ft = _ft.wrapping_sub(1);
    ftb = ::std::mem::size_of::<u32>() as libc::c_ulong as i32 * 8 as i32
        - _ft.leading_zeros() as i32;
    if ftb > 8 as i32 {
        let mut t: opus_uint32 = 0;
        ftb -= 8 as i32;
        ft = (_ft >> ftb).wrapping_add(1 as i32 as u32);
        s = ec_decode(_this, ft);
        ec_dec_update(_this, s, s.wrapping_add(1 as i32 as u32), ft);
        t = s << ftb | ec_dec_bits(_this, ftb as u32);
        if t <= _ft {
            return t;
        }
        (*_this).error = 1 as i32;
        return _ft;
    } else {
        _ft = _ft.wrapping_add(1);
        s = ec_decode(_this, _ft);
        ec_dec_update(_this, s, s.wrapping_add(1 as i32 as u32), _ft);
        return s;
    };
}
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
/*Initializes the decoder.
_buf: The input buffer to use.
Return: 0 on success, or a negative value on error.*/
/*Calculates the cumulative frequency for the next symbol.
This can then be fed into the probability model to determine what that
 symbol is, and the additional frequency information required to advance to
 the next symbol.
This function cannot be called more than once without a corresponding call to
 ec_dec_update(), or decoding will not proceed correctly.
_ft: The total frequency of the symbols in the alphabet the next symbol was
      encoded with.
Return: A cumulative frequency representing the encoded symbol.
        If the cumulative frequency of all the symbols before the one that
         was encoded was fl, and the cumulative frequency of all the symbols
         up to and including the one encoded is fh, then the returned value
         will fall in the range [fl,fh).*/
/*Equivalent to ec_decode() with _ft==1<<_bits.*/
/*Advance the decoder past the next symbol using the frequency information the
 symbol was encoded with.
Exactly one call to ec_decode() must have been made so that all necessary
 intermediate calculations are performed.
_fl:  The cumulative frequency of all symbols that come before the symbol
       decoded.
_fh:  The cumulative frequency of all symbols up to and including the symbol
       decoded.
      Together with _fl, this defines the range [_fl,_fh) in which the value
       returned above must fall.
_ft:  The total frequency of the symbols in the alphabet the symbol decoded
       was encoded in.
      This must be the same as passed to the preceding call to ec_decode().*/
/* Decode a bit that has a 1/(1<<_logp) probability of being a one */
/*Decodes a symbol given an "inverse" CDF table.
No call to ec_dec_update() is necessary after this call.
_icdf: The "inverse" CDF, such that symbol s falls in the range
        [s>0?ft-_icdf[s-1]:0,ft-_icdf[s]), where ft=1<<_ftb.
       The values must be monotonically non-increasing, and the last value
        must be 0.
_ftb: The number of bits of precision in the cumulative distribution.
Return: The decoded symbol s.*/
/*Extracts a raw unsigned integer with a non-power-of-2 range from the stream.
The bits must have been encoded with ec_enc_uint().
No call to ec_dec_update() is necessary after this call.
_ft: The number of integers that can be decoded (one more than the max).
     This must be at least one, and no more than 2**32-1.
Return: The decoded bits.*/
/*Extracts a sequence of raw bits from the stream.
The bits must have been encoded with ec_enc_bits().
No call to ec_dec_update() is necessary after this call.
_ftb: The number of bits to extract.
      This must be between 0 and 25, inclusive.
Return: The decoded bits.*/
#[no_mangle]

pub unsafe extern "C" fn ec_dec_bits(
    mut _this: *mut ec_dec,
    mut _bits: u32,
) -> opus_uint32 {
    let mut window: ec_window = 0;
    let mut available: i32 = 0;
    let mut ret: opus_uint32 = 0;
    window = (*_this).end_window;
    available = (*_this).nend_bits;
    if (available as u32) < _bits {
        loop {
            window |= (ec_read_byte_from_end(_this)
                as ec_window)
                << available;
            available += 8 as i32;
            if !(available
                <= ::std::mem::size_of::<ec_window>()
                    as libc::c_ulong as i32
                    * 8 as i32
                    - 8 as i32)
            {
                break;
            }
        }
    }
    ret = window & ((1 as i32 as opus_uint32) << _bits).wrapping_sub(1 as u32);
    window >>= _bits;
    available = (available as u32).wrapping_sub(_bits) as i32;
    (*_this).end_window = window;
    (*_this).nend_bits = available;
    (*_this).nbits_total = ((*_this).nbits_total as u32).wrapping_add(_bits) as i32;
    return ret;
}

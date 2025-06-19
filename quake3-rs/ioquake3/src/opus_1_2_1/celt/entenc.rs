use ::libc;

pub mod entcode_h {

    #[inline]

    pub unsafe extern "C" fn celt_udiv(
        mut n: crate::opus_types_h::opus_uint32,
        mut d: crate::opus_types_h::opus_uint32,
    ) -> crate::opus_types_h::opus_uint32 {
        return n.wrapping_div(d);
    }
}

pub use crate::opus_types_h::opus_uint32;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::uint32_t;

pub use crate::src::opus_1_2_1::celt::entcode::ec_ctx;
pub use crate::src::opus_1_2_1::celt::entcode::ec_enc;
pub use crate::src::opus_1_2_1::celt::entcode::ec_window;
pub use crate::src::opus_1_2_1::celt::entenc::entcode_h::celt_udiv;

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
/*A range encoder.
See entdec.c and the references for implementation details \cite{Mar79,MNW98}.

@INPROCEEDINGS{Mar79,
 author="Martin, G.N.N.",
 title="Range encoding: an algorithm for removing redundancy from a digitised
  message",
 booktitle="Video \& Data Recording Conference",
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
 URL="http://www.stanford.edu/class/ee398/handouts/papers/Moffat98ArithmCoding.pdf"
}*/

unsafe extern "C" fn ec_write_byte(
    mut _this: *mut crate::src::opus_1_2_1::celt::entcode::ec_enc,
    mut _value: libc::c_uint,
) -> libc::c_int {
    if (*_this).offs.wrapping_add((*_this).end_offs) >= (*_this).storage {
        return -(1 as libc::c_int);
    }
    let fresh0 = (*_this).offs;
    (*_this).offs = (*_this).offs.wrapping_add(1);
    *(*_this).buf.offset(fresh0 as isize) = _value as libc::c_uchar;
    return 0 as libc::c_int;
}

unsafe extern "C" fn ec_write_byte_at_end(
    mut _this: *mut crate::src::opus_1_2_1::celt::entcode::ec_enc,
    mut _value: libc::c_uint,
) -> libc::c_int {
    if (*_this).offs.wrapping_add((*_this).end_offs) >= (*_this).storage {
        return -(1 as libc::c_int);
    }
    (*_this).end_offs = (*_this).end_offs.wrapping_add(1);
    *(*_this)
        .buf
        .offset((*_this).storage.wrapping_sub((*_this).end_offs) as isize) =
        _value as libc::c_uchar;
    return 0 as libc::c_int;
}
/*Outputs a symbol, with a carry bit.
If there is a potential to propagate a carry over several symbols, they are
 buffered until it can be determined whether or not an actual carry will
 occur.
If the counter for the buffered symbols overflows, then the stream becomes
 undecodable.
This gives a theoretical limit of a few billion symbols in a single packet on
 32-bit systems.
The alternative is to truncate the range in order to force a carry, but
 requires similar carry tracking in the decoder, needlessly slowing it down.*/

unsafe extern "C" fn ec_enc_carry_out(
    mut _this: *mut crate::src::opus_1_2_1::celt::entcode::ec_enc,
    mut _c: libc::c_int,
) {
    if _c as libc::c_uint
        != ((1 as libc::c_uint) << 8 as libc::c_int).wrapping_sub(1 as libc::c_int as libc::c_uint)
    {
        /*No further carry propagation possible, flush buffer.*/
        let mut carry: libc::c_int = 0;
        carry = _c >> 8 as libc::c_int;
        /*Don't output a byte on the first write.
        This compare should be taken care of by branch-prediction thereafter.*/
        if (*_this).rem >= 0 as libc::c_int {
            (*_this).error |= ec_write_byte(_this, ((*_this).rem + carry) as libc::c_uint)
        }
        if (*_this).ext > 0 as libc::c_int as libc::c_uint {
            let mut sym: libc::c_uint = 0;
            sym = ((1 as libc::c_uint) << 8 as libc::c_int)
                .wrapping_sub(1 as libc::c_int as libc::c_uint)
                .wrapping_add(carry as libc::c_uint)
                & ((1 as libc::c_uint) << 8 as libc::c_int)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint);
            loop {
                (*_this).error |= ec_write_byte(_this, sym);
                (*_this).ext = (*_this).ext.wrapping_sub(1);
                if !((*_this).ext > 0 as libc::c_int as libc::c_uint) {
                    break;
                }
            }
        }
        (*_this).rem = (_c as libc::c_uint
            & ((1 as libc::c_uint) << 8 as libc::c_int)
                .wrapping_sub(1 as libc::c_int as libc::c_uint))
            as libc::c_int
    } else {
        (*_this).ext = (*_this).ext.wrapping_add(1)
    };
}
#[inline]

unsafe extern "C" fn ec_enc_normalize(
    mut _this: *mut crate::src::opus_1_2_1::celt::entcode::ec_enc,
) {
    /*If the range is too small, output some bits and rescale it.*/
    while (*_this).rng
        <= (1 as libc::c_uint) << 32 as libc::c_int - 1 as libc::c_int >> 8 as libc::c_int
    {
        ec_enc_carry_out(
            _this,
            ((*_this).val >> 32 as libc::c_int - 8 as libc::c_int - 1 as libc::c_int)
                as libc::c_int,
        );
        /*Move the next-to-high-order symbol into the high-order position.*/
        (*_this).val = (*_this).val << 8 as libc::c_int
            & ((1 as libc::c_uint) << 32 as libc::c_int - 1 as libc::c_int)
                .wrapping_sub(1 as libc::c_int as libc::c_uint);
        (*_this).rng <<= 8 as libc::c_int;
        (*_this).nbits_total += 8 as libc::c_int
    }
}
#[no_mangle]

pub unsafe extern "C" fn ec_enc_init(
    mut _this: *mut crate::src::opus_1_2_1::celt::entcode::ec_enc,
    mut _buf: *mut libc::c_uchar,
    mut _size: crate::opus_types_h::opus_uint32,
) {
    (*_this).buf = _buf;
    (*_this).end_offs = 0 as libc::c_int as crate::opus_types_h::opus_uint32;
    (*_this).end_window = 0 as libc::c_int as crate::src::opus_1_2_1::celt::entcode::ec_window;
    (*_this).nend_bits = 0 as libc::c_int;
    /*This is the offset from which ec_tell() will subtract partial bits.*/
    (*_this).nbits_total = 32 as libc::c_int + 1 as libc::c_int;
    (*_this).offs = 0 as libc::c_int as crate::opus_types_h::opus_uint32;
    (*_this).rng = (1 as libc::c_uint) << 32 as libc::c_int - 1 as libc::c_int;
    (*_this).rem = -(1 as libc::c_int);
    (*_this).val = 0 as libc::c_int as crate::opus_types_h::opus_uint32;
    (*_this).ext = 0 as libc::c_int as crate::opus_types_h::opus_uint32;
    (*_this).storage = _size;
    (*_this).error = 0 as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn ec_encode(
    mut _this: *mut crate::src::opus_1_2_1::celt::entcode::ec_enc,
    mut _fl: libc::c_uint,
    mut _fh: libc::c_uint,
    mut _ft: libc::c_uint,
) {
    let mut r: crate::opus_types_h::opus_uint32 = 0;
    r = celt_udiv((*_this).rng, _ft);
    if _fl > 0 as libc::c_int as libc::c_uint {
        (*_this).val = ((*_this).val as libc::c_uint).wrapping_add(
            (*_this)
                .rng
                .wrapping_sub(r.wrapping_mul(_ft.wrapping_sub(_fl))),
        ) as crate::opus_types_h::opus_uint32
            as crate::opus_types_h::opus_uint32;
        (*_this).rng = r.wrapping_mul(_fh.wrapping_sub(_fl))
    } else {
        (*_this).rng =
            ((*_this).rng as libc::c_uint).wrapping_sub(r.wrapping_mul(_ft.wrapping_sub(_fh)))
                as crate::opus_types_h::opus_uint32 as crate::opus_types_h::opus_uint32
    }
    ec_enc_normalize(_this);
}
#[no_mangle]

pub unsafe extern "C" fn ec_encode_bin(
    mut _this: *mut crate::src::opus_1_2_1::celt::entcode::ec_enc,
    mut _fl: libc::c_uint,
    mut _fh: libc::c_uint,
    mut _bits: libc::c_uint,
) {
    let mut r: crate::opus_types_h::opus_uint32 = 0;
    r = (*_this).rng >> _bits;
    if _fl > 0 as libc::c_int as libc::c_uint {
        (*_this).val = ((*_this).val as libc::c_uint).wrapping_add(
            (*_this)
                .rng
                .wrapping_sub(r.wrapping_mul(((1 as libc::c_uint) << _bits).wrapping_sub(_fl))),
        ) as crate::opus_types_h::opus_uint32
            as crate::opus_types_h::opus_uint32;
        (*_this).rng = r.wrapping_mul(_fh.wrapping_sub(_fl))
    } else {
        (*_this).rng = ((*_this).rng as libc::c_uint)
            .wrapping_sub(r.wrapping_mul(((1 as libc::c_uint) << _bits).wrapping_sub(_fh)))
            as crate::opus_types_h::opus_uint32
            as crate::opus_types_h::opus_uint32
    }
    ec_enc_normalize(_this);
}
/*The probability of having a "one" is 1/(1<<_logp).*/
#[no_mangle]

pub unsafe extern "C" fn ec_enc_bit_logp(
    mut _this: *mut crate::src::opus_1_2_1::celt::entcode::ec_enc,
    mut _val: libc::c_int,
    mut _logp: libc::c_uint,
) {
    let mut r: crate::opus_types_h::opus_uint32 = 0;
    let mut s: crate::opus_types_h::opus_uint32 = 0;
    let mut l: crate::opus_types_h::opus_uint32 = 0;
    r = (*_this).rng;
    l = (*_this).val;
    s = r >> _logp;
    r = (r as libc::c_uint).wrapping_sub(s) as crate::opus_types_h::opus_uint32
        as crate::opus_types_h::opus_uint32;
    if _val != 0 {
        (*_this).val = l.wrapping_add(r)
    }
    (*_this).rng = if _val != 0 { s } else { r };
    ec_enc_normalize(_this);
}
#[no_mangle]

pub unsafe extern "C" fn ec_enc_icdf(
    mut _this: *mut crate::src::opus_1_2_1::celt::entcode::ec_enc,
    mut _s: libc::c_int,
    mut _icdf: *const libc::c_uchar,
    mut _ftb: libc::c_uint,
) {
    let mut r: crate::opus_types_h::opus_uint32 = 0;
    r = (*_this).rng >> _ftb;
    if _s > 0 as libc::c_int {
        (*_this).val = ((*_this).val as libc::c_uint).wrapping_add((*_this).rng.wrapping_sub(
            r.wrapping_mul(*_icdf.offset((_s - 1 as libc::c_int) as isize) as libc::c_uint),
        )) as crate::opus_types_h::opus_uint32
            as crate::opus_types_h::opus_uint32;
        (*_this).rng = r.wrapping_mul(
            (*_icdf.offset((_s - 1 as libc::c_int) as isize) as libc::c_int
                - *_icdf.offset(_s as isize) as libc::c_int) as libc::c_uint,
        )
    } else {
        (*_this).rng = ((*_this).rng as libc::c_uint)
            .wrapping_sub(r.wrapping_mul(*_icdf.offset(_s as isize) as libc::c_uint))
            as crate::opus_types_h::opus_uint32
            as crate::opus_types_h::opus_uint32
    }
    ec_enc_normalize(_this);
}
#[no_mangle]

pub unsafe extern "C" fn ec_enc_uint(
    mut _this: *mut crate::src::opus_1_2_1::celt::entcode::ec_enc,
    mut _fl: crate::opus_types_h::opus_uint32,
    mut _ft: crate::opus_types_h::opus_uint32,
) {
    let mut ft: libc::c_uint = 0;
    let mut fl: libc::c_uint = 0;
    let mut ftb: libc::c_int = 0;
    /*In order to optimize EC_ILOG(), it is undefined for the value 0.*/
    _ft = _ft.wrapping_sub(1);
    ftb = ::std::mem::size_of::<libc::c_uint>() as libc::c_ulong as libc::c_int * 8 as libc::c_int
        - _ft.leading_zeros() as i32;
    if ftb > 8 as libc::c_int {
        ftb -= 8 as libc::c_int;
        ft = (_ft >> ftb).wrapping_add(1 as libc::c_int as libc::c_uint);
        fl = _fl >> ftb;
        ec_encode(
            _this,
            fl,
            fl.wrapping_add(1 as libc::c_int as libc::c_uint),
            ft,
        );
        ec_enc_bits(
            _this,
            _fl & ((1 as libc::c_int as crate::opus_types_h::opus_uint32) << ftb)
                .wrapping_sub(1 as libc::c_uint),
            ftb as libc::c_uint,
        );
    } else {
        ec_encode(
            _this,
            _fl,
            _fl.wrapping_add(1 as libc::c_int as libc::c_uint),
            _ft.wrapping_add(1 as libc::c_int as libc::c_uint),
        );
    };
}
#[no_mangle]

pub unsafe extern "C" fn ec_enc_bits(
    mut _this: *mut crate::src::opus_1_2_1::celt::entcode::ec_enc,
    mut _fl: crate::opus_types_h::opus_uint32,
    mut _bits: libc::c_uint,
) {
    let mut window: crate::src::opus_1_2_1::celt::entcode::ec_window = 0;
    let mut used: libc::c_int = 0;
    window = (*_this).end_window;
    used = (*_this).nend_bits;
    if (used as libc::c_uint).wrapping_add(_bits)
        > (::std::mem::size_of::<crate::src::opus_1_2_1::celt::entcode::ec_window>()
            as libc::c_ulong as libc::c_int
            * 8 as libc::c_int) as libc::c_uint
    {
        loop {
            (*_this).error |= ec_write_byte_at_end(
                _this,
                window
                    & ((1 as libc::c_uint) << 8 as libc::c_int)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint),
            );
            window >>= 8 as libc::c_int;
            used -= 8 as libc::c_int;
            if !(used >= 8 as libc::c_int) {
                break;
            }
        }
    }
    window |= _fl << used;
    used = (used as libc::c_uint).wrapping_add(_bits) as libc::c_int as libc::c_int;
    (*_this).end_window = window;
    (*_this).nend_bits = used;
    (*_this).nbits_total =
        ((*_this).nbits_total as libc::c_uint).wrapping_add(_bits) as libc::c_int as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn ec_enc_patch_initial_bits(
    mut _this: *mut crate::src::opus_1_2_1::celt::entcode::ec_enc,
    mut _val: libc::c_uint,
    mut _nbits: libc::c_uint,
) {
    let mut shift: libc::c_int = 0;
    let mut mask: libc::c_uint = 0;
    shift = (8 as libc::c_int as libc::c_uint).wrapping_sub(_nbits) as libc::c_int;
    mask = ((((1 as libc::c_int) << _nbits) - 1 as libc::c_int) << shift) as libc::c_uint;
    if (*_this).offs > 0 as libc::c_int as libc::c_uint {
        /*The first byte has been finalized.*/
        *(*_this).buf.offset(0 as libc::c_int as isize) =
            (*(*_this).buf.offset(0 as libc::c_int as isize) as libc::c_uint & !mask
                | _val << shift) as libc::c_uchar
    } else if (*_this).rem >= 0 as libc::c_int {
        /*The first byte is still awaiting carry propagation.*/
        (*_this).rem = ((*_this).rem as libc::c_uint & !mask | _val << shift) as libc::c_int
    } else if (*_this).rng <= (1 as libc::c_uint) << 32 as libc::c_int - 1 as libc::c_int >> _nbits
    {
        /*The renormalization loop has never been run.*/
        (*_this).val = (*_this).val
            & !(mask << 32 as libc::c_int - 8 as libc::c_int - 1 as libc::c_int)
            | _val << 32 as libc::c_int - 8 as libc::c_int - 1 as libc::c_int + shift
    } else {
        /*The encoder hasn't even encoded _nbits of data yet.*/
        (*_this).error = -(1 as libc::c_int)
    };
}
#[no_mangle]

pub unsafe extern "C" fn ec_enc_shrink(
    mut _this: *mut crate::src::opus_1_2_1::celt::entcode::ec_enc,
    mut _size: crate::opus_types_h::opus_uint32,
) {
    crate::stdlib::memmove(
        (*_this)
            .buf
            .offset(_size as isize)
            .offset(-((*_this).end_offs as isize)) as *mut libc::c_void,
        (*_this)
            .buf
            .offset((*_this).storage as isize)
            .offset(-((*_this).end_offs as isize)) as *const libc::c_void,
        ((*_this).end_offs as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong)
            .wrapping_add(
                (0 as libc::c_int as libc::c_long
                    * (*_this)
                        .buf
                        .offset(_size as isize)
                        .offset(-((*_this).end_offs as isize))
                        .offset_from(
                            (*_this)
                                .buf
                                .offset((*_this).storage as isize)
                                .offset(-((*_this).end_offs as isize)),
                        ) as libc::c_long) as libc::c_ulong,
            ),
    );
    (*_this).storage = _size;
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
/*Initializes the encoder.
_buf:  The buffer to store output bytes in.
_size: The size of the buffer, in chars.*/
/*Encodes a symbol given its frequency information.
The frequency information must be discernable by the decoder, assuming it
 has read only the previous symbols from the stream.
It is allowable to change the frequency information, or even the entire
 source alphabet, so long as the decoder can tell from the context of the
 previously encoded information that it is supposed to do so as well.
_fl: The cumulative frequency of all symbols that come before the one to be
      encoded.
_fh: The cumulative frequency of all symbols up to and including the one to
      be encoded.
     Together with _fl, this defines the range [_fl,_fh) in which the
      decoded value will fall.
_ft: The sum of the frequencies of all the symbols*/
/*Equivalent to ec_encode() with _ft==1<<_bits.*/
/* Encode a bit that has a 1/(1<<_logp) probability of being a one */
/*Encodes a symbol given an "inverse" CDF table.
_s:    The index of the symbol to encode.
_icdf: The "inverse" CDF, such that symbol _s falls in the range
        [_s>0?ft-_icdf[_s-1]:0,ft-_icdf[_s]), where ft=1<<_ftb.
       The values must be monotonically non-increasing, and the last value
        must be 0.
_ftb: The number of bits of precision in the cumulative distribution.*/
/*Encodes a raw unsigned integer in the stream.
_fl: The integer to encode.
_ft: The number of integers that can be encoded (one more than the max).
     This must be at least one, and no more than 2**32-1.*/
/*Encodes a sequence of raw bits in the stream.
_fl:  The bits to encode.
_ftb: The number of bits to encode.
      This must be between 1 and 25, inclusive.*/
/*Overwrites a few bits at the very start of an existing stream, after they
 have already been encoded.
This makes it possible to have a few flags up front, where it is easy for
 decoders to access them without parsing the whole stream, even if their
 values are not determined until late in the encoding process, without having
 to buffer all the intermediate symbols in the encoder.
In order for this to work, at least _nbits bits must have already been
 encoded using probabilities that are an exact power of two.
The encoder can verify the number of encoded bits is sufficient, but cannot
 check this latter condition.
_val:   The bits to encode (in the least _nbits significant bits).
        They will be decoded in order from most-significant to least.
_nbits: The number of bits to overwrite.
        This must be no more than 8.*/
/*Compacts the data to fit in the target size.
This moves up the raw bits at the end of the current buffer so they are at
 the end of the new buffer size.
The caller must ensure that the amount of data that's already been written
 will fit in the new size.
_size: The number of bytes in the new buffer.
       This must be large enough to contain the bits already written, and
        must be no larger than the existing size.*/
/*Indicates that there are no more symbols to encode.
All reamining output bytes are flushed to the output buffer.
ec_enc_init() must be called before the encoder can be used again.*/
#[no_mangle]

pub unsafe extern "C" fn ec_enc_done(
    mut _this: *mut crate::src::opus_1_2_1::celt::entcode::ec_enc,
) {
    let mut window: crate::src::opus_1_2_1::celt::entcode::ec_window = 0;
    let mut used: libc::c_int = 0;
    let mut msk: crate::opus_types_h::opus_uint32 = 0;
    let mut end: crate::opus_types_h::opus_uint32 = 0;
    let mut l: libc::c_int = 0;
    /*We output the minimum number of bits that ensures that the symbols encoded
    thus far will be decoded correctly regardless of the bits that follow.*/
    l = 32 as libc::c_int
        - (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong as libc::c_int
            * 8 as libc::c_int
            - (*_this).rng.leading_zeros() as i32);
    msk = ((1 as libc::c_uint) << 32 as libc::c_int - 1 as libc::c_int)
        .wrapping_sub(1 as libc::c_int as libc::c_uint)
        >> l;
    end = (*_this).val.wrapping_add(msk) & !msk;
    if end | msk >= (*_this).val.wrapping_add((*_this).rng) {
        l += 1;
        msk >>= 1 as libc::c_int;
        end = (*_this).val.wrapping_add(msk) & !msk
    }
    while l > 0 as libc::c_int {
        ec_enc_carry_out(
            _this,
            (end >> 32 as libc::c_int - 8 as libc::c_int - 1 as libc::c_int) as libc::c_int,
        );
        end = end << 8 as libc::c_int
            & ((1 as libc::c_uint) << 32 as libc::c_int - 1 as libc::c_int)
                .wrapping_sub(1 as libc::c_int as libc::c_uint);
        l -= 8 as libc::c_int
    }
    /*If we have a buffered byte flush it into the output buffer.*/
    if (*_this).rem >= 0 as libc::c_int || (*_this).ext > 0 as libc::c_int as libc::c_uint {
        ec_enc_carry_out(_this, 0 as libc::c_int);
    }
    /*If we have buffered extra bits, flush them as well.*/
    window = (*_this).end_window;
    used = (*_this).nend_bits;
    while used >= 8 as libc::c_int {
        (*_this).error |= ec_write_byte_at_end(
            _this,
            window
                & ((1 as libc::c_uint) << 8 as libc::c_int)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
        );
        window >>= 8 as libc::c_int;
        used -= 8 as libc::c_int
    }
    /*Clear any excess space and add any remaining extra bits to the last byte.*/
    if (*_this).error == 0 {
        crate::stdlib::memset(
            (*_this).buf.offset((*_this).offs as isize) as *mut libc::c_void,
            0 as libc::c_int,
            ((*_this)
                .storage
                .wrapping_sub((*_this).offs)
                .wrapping_sub((*_this).end_offs) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong),
        );
        if used > 0 as libc::c_int {
            /*If there's no range coder data at all, give up.*/
            if (*_this).end_offs >= (*_this).storage {
                (*_this).error = -(1 as libc::c_int)
            } else {
                l = -l;
                /*If we've busted, don't add too many extra bits to the last byte; it
                would corrupt the range coder data, and that's more important.*/
                if (*_this).offs.wrapping_add((*_this).end_offs) >= (*_this).storage && l < used {
                    window &= (((1 as libc::c_int) << l) - 1 as libc::c_int) as libc::c_uint;
                    (*_this).error = -(1 as libc::c_int)
                }
                let ref mut fresh1 = *(*_this).buf.offset(
                    (*_this)
                        .storage
                        .wrapping_sub((*_this).end_offs)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint)
                        as isize,
                );
                *fresh1 = (*fresh1 as libc::c_int | window as libc::c_uchar as libc::c_int)
                    as libc::c_uchar
            }
        }
    };
}

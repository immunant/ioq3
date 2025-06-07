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

pub use crate::stdlib::__int16_t;
pub use crate::stdlib::__int32_t;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::int16_t;
pub use crate::stdlib::int32_t;
pub use crate::stdlib::uint32_t;

pub use crate::arch_h::opus_val16;
pub use crate::opus_types_h::opus_int16;
pub use crate::opus_types_h::opus_int32;
pub use crate::opus_types_h::opus_uint32;
pub use crate::src::opus_1_2_1::celt::kiss_fft::arch_fft_state;
pub use crate::src::opus_1_2_1::celt::kiss_fft::kiss_fft_state;
pub use crate::src::opus_1_2_1::celt::kiss_fft::kiss_twiddle_cpx;
pub use crate::src::opus_1_2_1::celt::mdct::mdct_lookup;
pub use crate::src::opus_1_2_1::celt::modes::OpusCustomMode;
pub use crate::src::opus_1_2_1::celt::modes::PulseCache;

pub use crate::src::opus_1_2_1::celt::entcode::ec_ctx;
pub use crate::src::opus_1_2_1::celt::entcode::ec_dec;
pub use crate::src::opus_1_2_1::celt::entcode::ec_enc;
pub use crate::src::opus_1_2_1::celt::entcode::ec_window;

pub use crate::src::opus_1_2_1::celt::rate::entcode_h::celt_udiv;
/* Copyright (c) 2007-2008 CSIRO
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

static mut LOG2_FRAC_TABLE: [libc::c_uchar; 24] = [
    0 as libc::c_int as libc::c_uchar,
    8 as libc::c_int as libc::c_uchar,
    13 as libc::c_int as libc::c_uchar,
    16 as libc::c_int as libc::c_uchar,
    19 as libc::c_int as libc::c_uchar,
    21 as libc::c_int as libc::c_uchar,
    23 as libc::c_int as libc::c_uchar,
    24 as libc::c_int as libc::c_uchar,
    26 as libc::c_int as libc::c_uchar,
    27 as libc::c_int as libc::c_uchar,
    28 as libc::c_int as libc::c_uchar,
    29 as libc::c_int as libc::c_uchar,
    30 as libc::c_int as libc::c_uchar,
    31 as libc::c_int as libc::c_uchar,
    32 as libc::c_int as libc::c_uchar,
    32 as libc::c_int as libc::c_uchar,
    33 as libc::c_int as libc::c_uchar,
    34 as libc::c_int as libc::c_uchar,
    34 as libc::c_int as libc::c_uchar,
    35 as libc::c_int as libc::c_uchar,
    36 as libc::c_int as libc::c_uchar,
    36 as libc::c_int as libc::c_uchar,
    37 as libc::c_int as libc::c_uchar,
    37 as libc::c_int as libc::c_uchar,
];
#[inline]

unsafe extern "C" fn interp_bits2pulses(
    mut m: *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode,
    mut start: libc::c_int,
    mut end: libc::c_int,
    mut skip_start: libc::c_int,
    mut bits1: *const libc::c_int,
    mut bits2: *const libc::c_int,
    mut thresh: *const libc::c_int,
    mut cap: *const libc::c_int,
    mut total: crate::opus_types_h::opus_int32,
    mut _balance: *mut crate::opus_types_h::opus_int32,
    mut skip_rsv: libc::c_int,
    mut intensity: *mut libc::c_int,
    mut intensity_rsv: libc::c_int,
    mut dual_stereo: *mut libc::c_int,
    mut dual_stereo_rsv: libc::c_int,
    mut bits: *mut libc::c_int,
    mut ebits: *mut libc::c_int,
    mut fine_priority: *mut libc::c_int,
    mut C: libc::c_int,
    mut LM: libc::c_int,
    mut ec: *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
    mut encode: libc::c_int,
    mut prev: libc::c_int,
    mut signalBandwidth: libc::c_int,
) -> libc::c_int {
    let mut psum: crate::opus_types_h::opus_int32 = 0;
    let mut lo: libc::c_int = 0;
    let mut hi: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut logM: libc::c_int = 0;
    let mut stereo: libc::c_int = 0;
    let mut codedBands: libc::c_int = -(1 as libc::c_int);
    let mut alloc_floor: libc::c_int = 0;
    let mut left: crate::opus_types_h::opus_int32 = 0;
    let mut percoeff: crate::opus_types_h::opus_int32 = 0;
    let mut done: libc::c_int = 0;
    let mut balance: crate::opus_types_h::opus_int32 = 0;
    alloc_floor = C << 3 as libc::c_int;
    stereo = (C > 1 as libc::c_int) as libc::c_int;
    logM = LM << 3 as libc::c_int;
    lo = 0 as libc::c_int;
    hi = (1 as libc::c_int) << 6 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 6 as libc::c_int {
        let mut mid: libc::c_int = lo + hi >> 1 as libc::c_int;
        psum = 0 as libc::c_int;
        done = 0 as libc::c_int;
        j = end;
        loop {
            let fresh0 = j;
            j = j - 1;
            if !(fresh0 > start) {
                break;
            }
            let mut tmp: libc::c_int =
                *bits1.offset(j as isize) + (mid * *bits2.offset(j as isize) >> 6 as libc::c_int);
            if tmp >= *thresh.offset(j as isize) || done != 0 {
                done = 1 as libc::c_int;
                /* Don't allocate more than we can actually use */
                psum += if tmp < *cap.offset(j as isize) {
                    tmp
                } else {
                    *cap.offset(j as isize)
                }
            } else if tmp >= alloc_floor {
                psum += alloc_floor
            }
        }
        if psum > total {
            hi = mid
        } else {
            lo = mid
        }
        i += 1
    }
    psum = 0 as libc::c_int;
    /*printf ("interp bisection gave %d\n", lo);*/
    done = 0 as libc::c_int;
    j = end;
    loop {
        let fresh1 = j;
        j = j - 1;
        if !(fresh1 > start) {
            break;
        }
        let mut tmp_0: libc::c_int =
            *bits1.offset(j as isize) + (lo * *bits2.offset(j as isize) >> 6 as libc::c_int);
        if tmp_0 < *thresh.offset(j as isize) && done == 0 {
            if tmp_0 >= alloc_floor {
                tmp_0 = alloc_floor
            } else {
                tmp_0 = 0 as libc::c_int
            }
        } else {
            done = 1 as libc::c_int
        }
        /* Don't allocate more than we can actually use */
        tmp_0 = if tmp_0 < *cap.offset(j as isize) {
            tmp_0
        } else {
            *cap.offset(j as isize)
        };
        *bits.offset(j as isize) = tmp_0;
        psum += tmp_0
    }
    /* Decide which bands to skip, working backwards from the end. */
    codedBands = end;
    loop {
        let mut band_width: libc::c_int = 0;
        let mut band_bits: libc::c_int = 0;
        let mut rem: libc::c_int = 0;
        j = codedBands - 1 as libc::c_int;
        /* Never skip the first band, nor a band that has been boosted by
         dynalloc.
        In the first case, we'd be coding a bit to signal we're going to waste
         all the other bits.
        In the second case, we'd be coding a bit to redistribute all the bits
         we just signaled should be cocentrated in this band. */
        if j <= skip_start {
            /* Give the bit we reserved to end skipping back. */
            total += skip_rsv;
            break;
        } else {
            /*Figure out how many left-over bits we would be adding to this band.
            This can include bits we've stolen back from higher, skipped bands.*/
            left = total - psum;
            percoeff = celt_udiv(
                left as crate::opus_types_h::opus_uint32,
                (*(*m).eBands.offset(codedBands as isize) as libc::c_int
                    - *(*m).eBands.offset(start as isize) as libc::c_int)
                    as crate::opus_types_h::opus_uint32,
            ) as crate::opus_types_h::opus_int32;
            left -= (*(*m).eBands.offset(codedBands as isize) as libc::c_int
                - *(*m).eBands.offset(start as isize) as libc::c_int)
                * percoeff;
            rem = if left
                - (*(*m).eBands.offset(j as isize) as libc::c_int
                    - *(*m).eBands.offset(start as isize) as libc::c_int)
                > 0 as libc::c_int
            {
                (left)
                    - (*(*m).eBands.offset(j as isize) as libc::c_int
                        - *(*m).eBands.offset(start as isize) as libc::c_int)
            } else {
                0 as libc::c_int
            };
            band_width = *(*m).eBands.offset(codedBands as isize) as libc::c_int
                - *(*m).eBands.offset(j as isize) as libc::c_int;
            band_bits = *bits.offset(j as isize) + percoeff * band_width + rem;
            /*Only code a skip decision if we're above the threshold for this band.
            Otherwise it is force-skipped.
            This ensures that we have enough bits to code the skip flag.*/
            if band_bits
                >= (if *thresh.offset(j as isize)
                    > alloc_floor + ((1 as libc::c_int) << 3 as libc::c_int)
                {
                    *thresh.offset(j as isize)
                } else {
                    (alloc_floor) + ((1 as libc::c_int) << 3 as libc::c_int)
                })
            {
                if encode != 0 {
                    /*This if() block is the only part of the allocation function that
                    is not a mandatory part of the bitstream: any bands we choose to
                    skip here must be explicitly signaled.*/
                    let mut depth_threshold: libc::c_int = 0;
                    /*We choose a threshold with some hysteresis to keep bands from
                    fluctuating in and out, but we try not to fold below a certain point. */
                    if codedBands > 17 as libc::c_int {
                        depth_threshold = if j < prev {
                            7 as libc::c_int
                        } else {
                            9 as libc::c_int
                        }
                    } else {
                        depth_threshold = 0 as libc::c_int
                    }
                    if codedBands <= start + 2 as libc::c_int
                        || band_bits
                            > depth_threshold * band_width << LM << 3 as libc::c_int
                                >> 4 as libc::c_int
                            && j <= signalBandwidth
                    {
                        crate::src::opus_1_2_1::celt::entenc::ec_enc_bit_logp(
                            ec as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
                            1 as libc::c_int,
                            1 as libc::c_int as libc::c_uint,
                        );
                        break;
                    } else {
                        crate::src::opus_1_2_1::celt::entenc::ec_enc_bit_logp(
                            ec as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
                            0 as libc::c_int,
                            1 as libc::c_int as libc::c_uint,
                        );
                    }
                } else if crate::src::opus_1_2_1::celt::entdec::ec_dec_bit_logp(
                    ec as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
                    1 as libc::c_int as libc::c_uint,
                ) != 0
                {
                    break;
                }
                /*We used a bit to skip this band.*/
                psum += (1 as libc::c_int) << 3 as libc::c_int;
                band_bits -= (1 as libc::c_int) << 3 as libc::c_int
            }
            /*Reclaim the bits originally allocated to this band.*/
            psum -= *bits.offset(j as isize) + intensity_rsv;
            if intensity_rsv > 0 as libc::c_int {
                intensity_rsv = LOG2_FRAC_TABLE[(j - start) as usize] as libc::c_int
            }
            psum += intensity_rsv;
            if band_bits >= alloc_floor {
                /*If we have enough for a fine energy bit per channel, use it.*/
                psum += alloc_floor;
                *bits.offset(j as isize) = alloc_floor
            } else {
                /*Otherwise this band gets nothing at all.*/
                *bits.offset(j as isize) = 0 as libc::c_int
            }
            codedBands -= 1
        }
    }
    /* Code the intensity and dual stereo parameters. */
    if intensity_rsv > 0 as libc::c_int {
        if encode != 0 {
            *intensity = if *intensity < codedBands {
                *intensity
            } else {
                codedBands
            };
            crate::src::opus_1_2_1::celt::entenc::ec_enc_uint(
                ec as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
                (*intensity - start) as crate::opus_types_h::opus_uint32,
                (codedBands + 1 as libc::c_int - start) as crate::opus_types_h::opus_uint32,
            );
        } else {
            *intensity = (start as libc::c_uint).wrapping_add(
                crate::src::opus_1_2_1::celt::entdec::ec_dec_uint(
                    ec as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
                    (codedBands + 1 as libc::c_int - start) as crate::opus_types_h::opus_uint32,
                ),
            ) as libc::c_int
        }
    } else {
        *intensity = 0 as libc::c_int
    }
    if *intensity <= start {
        total += dual_stereo_rsv;
        dual_stereo_rsv = 0 as libc::c_int
    }
    if dual_stereo_rsv > 0 as libc::c_int {
        if encode != 0 {
            crate::src::opus_1_2_1::celt::entenc::ec_enc_bit_logp(
                ec as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
                *dual_stereo,
                1 as libc::c_int as libc::c_uint,
            );
        } else {
            *dual_stereo = crate::src::opus_1_2_1::celt::entdec::ec_dec_bit_logp(
                ec as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
                1 as libc::c_int as libc::c_uint,
            )
        }
    } else {
        *dual_stereo = 0 as libc::c_int
    }
    /* Allocate the remaining bits */
    left = total - psum;
    percoeff = celt_udiv(
        left as crate::opus_types_h::opus_uint32,
        (*(*m).eBands.offset(codedBands as isize) as libc::c_int
            - *(*m).eBands.offset(start as isize) as libc::c_int)
            as crate::opus_types_h::opus_uint32,
    ) as crate::opus_types_h::opus_int32;
    left -= (*(*m).eBands.offset(codedBands as isize) as libc::c_int
        - *(*m).eBands.offset(start as isize) as libc::c_int)
        * percoeff;
    j = start;
    while j < codedBands {
        *bits.offset(j as isize) += percoeff
            * (*(*m).eBands.offset((j + 1 as libc::c_int) as isize) as libc::c_int
                - *(*m).eBands.offset(j as isize) as libc::c_int);
        j += 1
    }
    j = start;
    while j < codedBands {
        let mut tmp_1: libc::c_int = if left
            < *(*m).eBands.offset((j + 1 as libc::c_int) as isize) as libc::c_int
                - *(*m).eBands.offset(j as isize) as libc::c_int
        {
            left
        } else {
            (*(*m).eBands.offset((j + 1 as libc::c_int) as isize) as libc::c_int)
                - *(*m).eBands.offset(j as isize) as libc::c_int
        };
        *bits.offset(j as isize) += tmp_1;
        left -= tmp_1;
        j += 1
    }
    /*for (j=0;j<end;j++)printf("%d ", bits[j]);printf("\n");*/
    balance = 0 as libc::c_int;
    j = start;
    while j < codedBands {
        let mut N0: libc::c_int = 0;
        let mut N: libc::c_int = 0;
        let mut den: libc::c_int = 0;
        let mut offset: libc::c_int = 0;
        let mut NClogN: libc::c_int = 0;
        let mut excess: crate::opus_types_h::opus_int32 = 0;
        let mut bit: crate::opus_types_h::opus_int32 = 0;
        N0 = *(*m).eBands.offset((j + 1 as libc::c_int) as isize) as libc::c_int
            - *(*m).eBands.offset(j as isize) as libc::c_int;
        N = N0 << LM;
        bit = *bits.offset(j as isize) + balance;
        if N > 1 as libc::c_int {
            excess = if bit - *cap.offset(j as isize) > 0 as libc::c_int {
                (bit) - *cap.offset(j as isize)
            } else {
                0 as libc::c_int
            };
            *bits.offset(j as isize) = bit - excess;
            /* Compensate for the extra DoF in stereo */
            den = C * N
                + (if C == 2 as libc::c_int
                    && N > 2 as libc::c_int
                    && *dual_stereo == 0
                    && j < *intensity
                {
                    1 as libc::c_int
                } else {
                    0 as libc::c_int
                });
            NClogN = den * (*(*m).logN.offset(j as isize) as libc::c_int + logM);
            /* Offset for the number of fine bits by log2(N)/2 + FINE_OFFSET
            compared to their "fair share" of total/N */
            offset = (NClogN >> 1 as libc::c_int) - den * 21 as libc::c_int;
            /* N=2 is the only point that doesn't match the curve */
            if N == 2 as libc::c_int {
                offset += den << 3 as libc::c_int >> 2 as libc::c_int
            }
            /* Changing the offset for allocating the second and third
            fine energy bit */
            if *bits.offset(j as isize) + offset < (den * 2 as libc::c_int) << 3 as libc::c_int {
                offset += NClogN >> 2 as libc::c_int
            } else if *bits.offset(j as isize) + offset
                < (den * 3 as libc::c_int) << 3 as libc::c_int
            {
                offset += NClogN >> 3 as libc::c_int
            }
            /* Divide with rounding */
            *ebits.offset(j as isize) = if 0 as libc::c_int
                > *bits.offset(j as isize) + offset + (den << 3 as libc::c_int - 1 as libc::c_int)
            {
                0 as libc::c_int
            } else {
                (*bits.offset(j as isize) + offset) + (den << 3 as libc::c_int - 1 as libc::c_int)
            };
            *ebits.offset(j as isize) = (celt_udiv(
                *ebits.offset(j as isize) as crate::opus_types_h::opus_uint32,
                den as crate::opus_types_h::opus_uint32,
            ) >> 3 as libc::c_int) as libc::c_int;
            /* Make sure not to bust */
            if C * *ebits.offset(j as isize) > *bits.offset(j as isize) >> 3 as libc::c_int {
                *ebits.offset(j as isize) = *bits.offset(j as isize) >> stereo >> 3 as libc::c_int
            }
            /* More than that is useless because that's about as far as PVQ can go */
            *ebits.offset(j as isize) = if *ebits.offset(j as isize) < 8 as libc::c_int {
                *ebits.offset(j as isize)
            } else {
                8 as libc::c_int
            };
            /* If we rounded down or capped this band, make it a candidate for the
            final fine energy pass */
            *fine_priority.offset(j as isize) =
                (*ebits.offset(j as isize) * (den << 3 as libc::c_int)
                    >= *bits.offset(j as isize) + offset) as libc::c_int;
            /* Remove the allocated fine bits; the rest are assigned to PVQ */
            *bits.offset(j as isize) -= C * *ebits.offset(j as isize) << 3 as libc::c_int
        } else {
            /* For N=1, all bits go to fine energy except for a single sign bit */
            excess = if 0 as libc::c_int > bit - (C << 3 as libc::c_int) {
                0 as libc::c_int
            } else {
                (bit) - (C << 3 as libc::c_int)
            };
            *bits.offset(j as isize) = bit - excess;
            *ebits.offset(j as isize) = 0 as libc::c_int;
            *fine_priority.offset(j as isize) = 1 as libc::c_int
        }
        /* Fine energy can't take advantage of the re-balancing in
         quant_all_bands().
        Instead, do the re-balancing here.*/
        if excess > 0 as libc::c_int {
            let mut extra_fine: libc::c_int = 0;
            let mut extra_bits: libc::c_int = 0;
            extra_fine = if (excess >> stereo + 3 as libc::c_int)
                < 8 as libc::c_int - *ebits.offset(j as isize)
            {
                (excess) >> stereo + 3 as libc::c_int
            } else {
                (8 as libc::c_int) - *ebits.offset(j as isize)
            };
            *ebits.offset(j as isize) += extra_fine;
            extra_bits = extra_fine * C << 3 as libc::c_int;
            *fine_priority.offset(j as isize) = (extra_bits >= excess - balance) as libc::c_int;
            excess -= extra_bits
        }
        balance = excess;
        j += 1
    }
    /* Save any remaining bits over the cap for the rebalancing in
    quant_all_bands(). */
    *_balance = balance;
    /* The skipped bands use all their bits for fine energy. */
    while j < end {
        *ebits.offset(j as isize) = *bits.offset(j as isize) >> stereo >> 3 as libc::c_int;
        *bits.offset(j as isize) = 0 as libc::c_int;
        *fine_priority.offset(j as isize) =
            (*ebits.offset(j as isize) < 1 as libc::c_int) as libc::c_int;
        j += 1
    }
    return codedBands;
}
/* * Compute the pulse allocation, i.e. how many pulses will go in each
  * band.
 @param m mode
 @param offsets Requested increase or decrease in the number of bits for
                each band
 @param total Number of bands
 @param pulses Number of pulses per band (returned)
 @return Total number of bits allocated
*/
#[no_mangle]

pub unsafe extern "C" fn compute_allocation(
    mut m: *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode,
    mut start: libc::c_int,
    mut end: libc::c_int,
    mut offsets: *const libc::c_int,
    mut cap: *const libc::c_int,
    mut alloc_trim: libc::c_int,
    mut intensity: *mut libc::c_int,
    mut dual_stereo: *mut libc::c_int,
    mut total: crate::opus_types_h::opus_int32,
    mut balance: *mut crate::opus_types_h::opus_int32,
    mut pulses: *mut libc::c_int,
    mut ebits: *mut libc::c_int,
    mut fine_priority: *mut libc::c_int,
    mut C: libc::c_int,
    mut LM: libc::c_int,
    mut ec: *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
    mut encode: libc::c_int,
    mut prev: libc::c_int,
    mut signalBandwidth: libc::c_int,
) -> libc::c_int {
    let mut lo: libc::c_int = 0;
    let mut hi: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut codedBands: libc::c_int = 0;
    let mut skip_start: libc::c_int = 0;
    let mut skip_rsv: libc::c_int = 0;
    let mut intensity_rsv: libc::c_int = 0;
    let mut dual_stereo_rsv: libc::c_int = 0;
    let mut bits1: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut bits2: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut thresh: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut trim_offset: *mut libc::c_int = 0 as *mut libc::c_int;
    total = if total > 0 as libc::c_int {
        total
    } else {
        0 as libc::c_int
    };
    len = (*m).nbEBands;
    skip_start = start;
    /* Reserve a bit to signal the end of manually skipped bands. */
    skip_rsv = if total >= (1 as libc::c_int) << 3 as libc::c_int {
        (1 as libc::c_int) << 3 as libc::c_int
    } else {
        0 as libc::c_int
    };
    total -= skip_rsv;
    /* Reserve bits for the intensity and dual stereo parameters. */
    dual_stereo_rsv = 0 as libc::c_int;
    intensity_rsv = dual_stereo_rsv;
    if C == 2 as libc::c_int {
        intensity_rsv = LOG2_FRAC_TABLE[(end - start) as usize] as libc::c_int;
        if intensity_rsv > total {
            intensity_rsv = 0 as libc::c_int
        } else {
            total -= intensity_rsv;
            dual_stereo_rsv = if total >= (1 as libc::c_int) << 3 as libc::c_int {
                (1 as libc::c_int) << 3 as libc::c_int
            } else {
                0 as libc::c_int
            };
            total -= dual_stereo_rsv
        }
    }
    let mut fresh2 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong).wrapping_mul(len as libc::c_ulong)
            as usize,
    );
    bits1 = fresh2.as_mut_ptr() as *mut libc::c_int;
    let mut fresh3 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong).wrapping_mul(len as libc::c_ulong)
            as usize,
    );
    bits2 = fresh3.as_mut_ptr() as *mut libc::c_int;
    let mut fresh4 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong).wrapping_mul(len as libc::c_ulong)
            as usize,
    );
    thresh = fresh4.as_mut_ptr() as *mut libc::c_int;
    let mut fresh5 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong).wrapping_mul(len as libc::c_ulong)
            as usize,
    );
    trim_offset = fresh5.as_mut_ptr() as *mut libc::c_int;
    j = start;
    while j < end {
        /* Below this threshold, we're sure not to allocate any PVQ bits */
        *thresh.offset(j as isize) = if C << 3 as libc::c_int
            > (3 as libc::c_int
                * (*(*m).eBands.offset((j + 1 as libc::c_int) as isize) as libc::c_int
                    - *(*m).eBands.offset(j as isize) as libc::c_int))
                << LM
                << 3 as libc::c_int
                >> 4 as libc::c_int
        {
            (C) << 3 as libc::c_int
        } else {
            ((3 as libc::c_int
                * (*(*m).eBands.offset((j + 1 as libc::c_int) as isize) as libc::c_int
                    - *(*m).eBands.offset(j as isize) as libc::c_int))
                << LM
                << 3 as libc::c_int)
                >> 4 as libc::c_int
        };
        /* Tilt of the allocation curve */
        *trim_offset.offset(j as isize) = C
            * (*(*m).eBands.offset((j + 1 as libc::c_int) as isize) as libc::c_int
                - *(*m).eBands.offset(j as isize) as libc::c_int)
            * (alloc_trim - 5 as libc::c_int - LM)
            * (end - j - 1 as libc::c_int)
            * ((1 as libc::c_int) << LM + 3 as libc::c_int)
            >> 6 as libc::c_int;
        /* Giving less resolution to single-coefficient bands because they get
        more benefit from having one coarse value per coefficient*/
        if (*(*m).eBands.offset((j + 1 as libc::c_int) as isize) as libc::c_int
            - *(*m).eBands.offset(j as isize) as libc::c_int)
            << LM
            == 1 as libc::c_int
        {
            *trim_offset.offset(j as isize) -= C << 3 as libc::c_int
        }
        j += 1
    }
    lo = 1 as libc::c_int;
    hi = (*m).nbAllocVectors - 1 as libc::c_int;
    loop {
        let mut done: libc::c_int = 0 as libc::c_int;
        let mut psum: libc::c_int = 0 as libc::c_int;
        let mut mid: libc::c_int = lo + hi >> 1 as libc::c_int;
        j = end;
        loop {
            let fresh6 = j;
            j = j - 1;
            if !(fresh6 > start) {
                break;
            }
            let mut bitsj: libc::c_int = 0;
            let mut N: libc::c_int = *(*m).eBands.offset((j + 1 as libc::c_int) as isize)
                as libc::c_int
                - *(*m).eBands.offset(j as isize) as libc::c_int;
            bitsj = (C * N * *(*m).allocVectors.offset((mid * len + j) as isize) as libc::c_int)
                << LM
                >> 2 as libc::c_int;
            if bitsj > 0 as libc::c_int {
                bitsj = if 0 as libc::c_int > bitsj + *trim_offset.offset(j as isize) {
                    0 as libc::c_int
                } else {
                    (bitsj) + *trim_offset.offset(j as isize)
                }
            }
            bitsj += *offsets.offset(j as isize);
            if bitsj >= *thresh.offset(j as isize) || done != 0 {
                done = 1 as libc::c_int;
                /*printf ("lo = %d, hi = %d\n", lo, hi);*/
                /* Don't allocate more than we can actually use */
                psum += if bitsj < *cap.offset(j as isize) {
                    bitsj
                } else {
                    *cap.offset(j as isize)
                }
            } else if bitsj >= C << 3 as libc::c_int {
                psum += C << 3 as libc::c_int
            }
        }
        if psum > total {
            hi = mid - 1 as libc::c_int
        } else {
            lo = mid + 1 as libc::c_int
        }
        if !(lo <= hi) {
            break;
        }
    }
    let fresh7 = lo;
    lo = lo - 1;
    hi = fresh7;
    /*printf ("interp between %d and %d\n", lo, hi);*/
    j = start;
    while j < end {
        let mut bits1j: libc::c_int = 0;
        let mut bits2j: libc::c_int = 0;
        let mut N_0: libc::c_int = *(*m).eBands.offset((j + 1 as libc::c_int) as isize)
            as libc::c_int
            - *(*m).eBands.offset(j as isize) as libc::c_int;
        bits1j = (C * N_0 * *(*m).allocVectors.offset((lo * len + j) as isize) as libc::c_int)
            << LM
            >> 2 as libc::c_int;
        bits2j = if hi >= (*m).nbAllocVectors {
            *cap.offset(j as isize)
        } else {
            ((C * N_0 * *(*m).allocVectors.offset((hi * len + j) as isize) as libc::c_int) << LM)
                >> 2 as libc::c_int
        };
        if bits1j > 0 as libc::c_int {
            bits1j = if 0 as libc::c_int > bits1j + *trim_offset.offset(j as isize) {
                0 as libc::c_int
            } else {
                (bits1j) + *trim_offset.offset(j as isize)
            }
        }
        if bits2j > 0 as libc::c_int {
            bits2j = if 0 as libc::c_int > bits2j + *trim_offset.offset(j as isize) {
                0 as libc::c_int
            } else {
                (bits2j) + *trim_offset.offset(j as isize)
            }
        }
        if lo > 0 as libc::c_int {
            bits1j += *offsets.offset(j as isize)
        }
        bits2j += *offsets.offset(j as isize);
        if *offsets.offset(j as isize) > 0 as libc::c_int {
            skip_start = j
        }
        bits2j = if 0 as libc::c_int > bits2j - bits1j {
            0 as libc::c_int
        } else {
            (bits2j) - bits1j
        };
        *bits1.offset(j as isize) = bits1j;
        *bits2.offset(j as isize) = bits2j;
        j += 1
    }
    codedBands = interp_bits2pulses(
        m,
        start,
        end,
        skip_start,
        bits1,
        bits2,
        thresh,
        cap,
        total,
        balance,
        skip_rsv,
        intensity,
        intensity_rsv,
        dual_stereo,
        dual_stereo_rsv,
        pulses,
        ebits,
        fine_priority,
        C,
        LM,
        ec,
        encode,
        prev,
        signalBandwidth,
    );
    return codedBands;
}

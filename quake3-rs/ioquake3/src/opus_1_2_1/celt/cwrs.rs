pub use crate::arch_h::opus_val32;
pub use crate::opus_types_h::opus_int16;
pub use crate::opus_types_h::opus_uint32;
pub use crate::src::opus_1_2_1::celt::entcode::ec_ctx;
pub use crate::src::opus_1_2_1::celt::entcode::ec_dec;
pub use crate::src::opus_1_2_1::celt::entcode::ec_enc;
pub use crate::src::opus_1_2_1::celt::entcode::ec_window;

pub use crate::stdlib::__int16_t;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::int16_t;
pub use crate::stdlib::uint32_t;

/* Copyright (c) 2007-2008 CSIRO
Copyright (c) 2007-2009 Xiph.Org Foundation
Copyright (c) 2007-2009 Timothy B. Terriberry
Written by Timothy B. Terriberry and Jean-Marc Valin */
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
/*Although derived separately, the pulse vector coding scheme is equivalent to
 a Pyramid Vector Quantizer \cite{Fis86}.
Some additional notes about an early version appear at
 https://people.xiph.org/~tterribe/notes/cwrs.html, but the codebook ordering
 and the definitions of some terms have evolved since that was written.

The conversion from a pulse vector to an integer index (encoding) and back
 (decoding) is governed by two related functions, V(N,K) and U(N,K).

V(N,K) = the number of combinations, with replacement, of N items, taken K
 at a time, when a sign bit is added to each item taken at least once (i.e.,
 the number of N-dimensional unit pulse vectors with K pulses).
One way to compute this is via
  V(N,K) = K>0 ? sum(k=1...K,2**k*choose(N,k)*choose(K-1,k-1)) : 1,
 where choose() is the binomial function.
A table of values for N<10 and K<10 looks like:
V[10][10] = {
  {1,  0,   0,    0,    0,     0,     0,      0,      0,       0},
  {1,  2,   2,    2,    2,     2,     2,      2,      2,       2},
  {1,  4,   8,   12,   16,    20,    24,     28,     32,      36},
  {1,  6,  18,   38,   66,   102,   146,    198,    258,     326},
  {1,  8,  32,   88,  192,   360,   608,    952,   1408,    1992},
  {1, 10,  50,  170,  450,  1002,  1970,   3530,   5890,    9290},
  {1, 12,  72,  292,  912,  2364,  5336,  10836,  20256,   35436},
  {1, 14,  98,  462, 1666,  4942, 12642,  28814,  59906,  115598},
  {1, 16, 128,  688, 2816,  9424, 27008,  68464, 157184,  332688},
  {1, 18, 162,  978, 4482, 16722, 53154, 148626, 374274,  864146}
};

U(N,K) = the number of such combinations wherein N-1 objects are taken at
 most K-1 at a time.
This is given by
  U(N,K) = sum(k=0...K-1,V(N-1,k))
         = K>0 ? (V(N-1,K-1) + V(N,K-1))/2 : 0.
The latter expression also makes clear that U(N,K) is half the number of such
 combinations wherein the first object is taken at least once.
Although it may not be clear from either of these definitions, U(N,K) is the
 natural function to work with when enumerating the pulse vector codebooks,
 not V(N,K).
U(N,K) is not well-defined for N=0, but with the extension
  U(0,K) = K>0 ? 0 : 1,
 the function becomes symmetric: U(N,K) = U(K,N), with a similar table:
U[10][10] = {
  {1, 0,  0,   0,    0,    0,     0,     0,      0,      0},
  {0, 1,  1,   1,    1,    1,     1,     1,      1,      1},
  {0, 1,  3,   5,    7,    9,    11,    13,     15,     17},
  {0, 1,  5,  13,   25,   41,    61,    85,    113,    145},
  {0, 1,  7,  25,   63,  129,   231,   377,    575,    833},
  {0, 1,  9,  41,  129,  321,   681,  1289,   2241,   3649},
  {0, 1, 11,  61,  231,  681,  1683,  3653,   7183,  13073},
  {0, 1, 13,  85,  377, 1289,  3653,  8989,  19825,  40081},
  {0, 1, 15, 113,  575, 2241,  7183, 19825,  48639, 108545},
  {0, 1, 17, 145,  833, 3649, 13073, 40081, 108545, 265729}
};

With this extension, V(N,K) may be written in terms of U(N,K):
  V(N,K) = U(N,K) + U(N,K+1)
 for all N>=0, K>=0.
Thus U(N,K+1) represents the number of combinations where the first element
 is positive or zero, and U(N,K) represents the number of combinations where
 it is negative.
With a large enough table of U(N,K) values, we could write O(N) encoding
 and O(min(N*log(K),N+K)) decoding routines, but such a table would be
 prohibitively large for small embedded devices (K may be as large as 32767
 for small N, and N may be as large as 200).

Both functions obey the same recurrence relation:
  V(N,K) = V(N-1,K) + V(N,K-1) + V(N-1,K-1),
  U(N,K) = U(N-1,K) + U(N,K-1) + U(N-1,K-1),
 for all N>0, K>0, with different initial conditions at N=0 or K=0.
This allows us to construct a row of one of the tables above given the
 previous row or the next row.
Thus we can derive O(NK) encoding and decoding routines with O(K) memory
 using only addition and subtraction.

When encoding, we build up from the U(2,K) row and work our way forwards.
When decoding, we need to start at the U(N,K) row and work our way backwards,
 which requires a means of computing U(N,K).
U(N,K) may be computed from two previous values with the same N:
  U(N,K) = ((2*N-1)*U(N,K-1) - U(N,K-2))/(K-1) + U(N,K-2)
 for all N>1, and since U(N,K) is symmetric, a similar relation holds for two
 previous values with the same K:
  U(N,K>1) = ((2*K-1)*U(N-1,K) - U(N-2,K))/(N-1) + U(N-2,K)
 for all K>1.
This allows us to construct an arbitrary row of the U(N,K) table by starting
 with the first two values, which are constants.
This saves roughly 2/3 the work in our O(NK) decoding routine, but costs O(K)
 multiplications.
Similar relations can be derived for V(N,K), but are not used here.

For N>0 and K>0, U(N,K) and V(N,K) take on the form of an (N-1)-degree
 polynomial for fixed N.
The first few are
  U(1,K) = 1,
  U(2,K) = 2*K-1,
  U(3,K) = (2*K-2)*K+1,
  U(4,K) = (((4*K-6)*K+8)*K-3)/3,
  U(5,K) = ((((2*K-4)*K+10)*K-8)*K+3)/3,
 and
  V(1,K) = 2,
  V(2,K) = 4*K,
  V(3,K) = 4*K*K+2,
  V(4,K) = 8*(K*K+2)*K/3,
  V(5,K) = ((4*K*K+20)*K*K+6)/3,
 for all K>0.
This allows us to derive O(N) encoding and O(N*log(K)) decoding routines for
 small N (and indeed decoding is also O(N) for N<3).

@ARTICLE{Fis86,
  author="Thomas R. Fischer",
  title="A Pyramid Vector Quantizer",
  journal="IEEE Transactions on Information Theory",
  volume="IT-32",
  number=4,
  pages="568--583",
  month=Jul,
  year=1986
}*/
/*U(N,K) = U(K,N) := N>0?K>0?U(N-1,K)+U(N,K-1)+U(N-1,K-1):0:K>0?1:0*/
/*V(N,K) := U(N,K)+U(N,K+1) = the number of PVQ codewords for a band of size N
with K pulses allocated to it.*/
/*For each V(N,K) supported, we will access element U(min(N,K+1),max(N,K+1)).
Thus, the number of entries in row I is the larger of the maximum number of
 pulses we will ever allocate for a given N=I (K=128, or however many fit in
 32 bits, whichever is smaller), plus one, and the maximum N for which
 K=I-1 pulses fit in 32 bits.
The largest band size in an Opus Custom mode is 208.
Otherwise, we can limit things to the set of N which can be achieved by
 splitting a band from a standard Opus mode: 176, 144, 96, 88, 72, 64, 48,
 44, 36, 32, 24, 22, 18, 16, 8, 4, 2).*/

static mut CELT_PVQ_U_DATA: [crate::opus_types_h::opus_uint32; 1272] = [
    1 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    0 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    1 as i32 as crate::opus_types_h::opus_uint32,
    3 as i32 as crate::opus_types_h::opus_uint32,
    5 as i32 as crate::opus_types_h::opus_uint32,
    7 as i32 as crate::opus_types_h::opus_uint32,
    9 as i32 as crate::opus_types_h::opus_uint32,
    11 as i32 as crate::opus_types_h::opus_uint32,
    13 as i32 as crate::opus_types_h::opus_uint32,
    15 as i32 as crate::opus_types_h::opus_uint32,
    17 as i32 as crate::opus_types_h::opus_uint32,
    19 as i32 as crate::opus_types_h::opus_uint32,
    21 as i32 as crate::opus_types_h::opus_uint32,
    23 as i32 as crate::opus_types_h::opus_uint32,
    25 as i32 as crate::opus_types_h::opus_uint32,
    27 as i32 as crate::opus_types_h::opus_uint32,
    29 as i32 as crate::opus_types_h::opus_uint32,
    31 as i32 as crate::opus_types_h::opus_uint32,
    33 as i32 as crate::opus_types_h::opus_uint32,
    35 as i32 as crate::opus_types_h::opus_uint32,
    37 as i32 as crate::opus_types_h::opus_uint32,
    39 as i32 as crate::opus_types_h::opus_uint32,
    41 as i32 as crate::opus_types_h::opus_uint32,
    43 as i32 as crate::opus_types_h::opus_uint32,
    45 as i32 as crate::opus_types_h::opus_uint32,
    47 as i32 as crate::opus_types_h::opus_uint32,
    49 as i32 as crate::opus_types_h::opus_uint32,
    51 as i32 as crate::opus_types_h::opus_uint32,
    53 as i32 as crate::opus_types_h::opus_uint32,
    55 as i32 as crate::opus_types_h::opus_uint32,
    57 as i32 as crate::opus_types_h::opus_uint32,
    59 as i32 as crate::opus_types_h::opus_uint32,
    61 as i32 as crate::opus_types_h::opus_uint32,
    63 as i32 as crate::opus_types_h::opus_uint32,
    65 as i32 as crate::opus_types_h::opus_uint32,
    67 as i32 as crate::opus_types_h::opus_uint32,
    69 as i32 as crate::opus_types_h::opus_uint32,
    71 as i32 as crate::opus_types_h::opus_uint32,
    73 as i32 as crate::opus_types_h::opus_uint32,
    75 as i32 as crate::opus_types_h::opus_uint32,
    77 as i32 as crate::opus_types_h::opus_uint32,
    79 as i32 as crate::opus_types_h::opus_uint32,
    81 as i32 as crate::opus_types_h::opus_uint32,
    83 as i32 as crate::opus_types_h::opus_uint32,
    85 as i32 as crate::opus_types_h::opus_uint32,
    87 as i32 as crate::opus_types_h::opus_uint32,
    89 as i32 as crate::opus_types_h::opus_uint32,
    91 as i32 as crate::opus_types_h::opus_uint32,
    93 as i32 as crate::opus_types_h::opus_uint32,
    95 as i32 as crate::opus_types_h::opus_uint32,
    97 as i32 as crate::opus_types_h::opus_uint32,
    99 as i32 as crate::opus_types_h::opus_uint32,
    101 as i32 as crate::opus_types_h::opus_uint32,
    103 as i32 as crate::opus_types_h::opus_uint32,
    105 as i32 as crate::opus_types_h::opus_uint32,
    107 as i32 as crate::opus_types_h::opus_uint32,
    109 as i32 as crate::opus_types_h::opus_uint32,
    111 as i32 as crate::opus_types_h::opus_uint32,
    113 as i32 as crate::opus_types_h::opus_uint32,
    115 as i32 as crate::opus_types_h::opus_uint32,
    117 as i32 as crate::opus_types_h::opus_uint32,
    119 as i32 as crate::opus_types_h::opus_uint32,
    121 as i32 as crate::opus_types_h::opus_uint32,
    123 as i32 as crate::opus_types_h::opus_uint32,
    125 as i32 as crate::opus_types_h::opus_uint32,
    127 as i32 as crate::opus_types_h::opus_uint32,
    129 as i32 as crate::opus_types_h::opus_uint32,
    131 as i32 as crate::opus_types_h::opus_uint32,
    133 as i32 as crate::opus_types_h::opus_uint32,
    135 as i32 as crate::opus_types_h::opus_uint32,
    137 as i32 as crate::opus_types_h::opus_uint32,
    139 as i32 as crate::opus_types_h::opus_uint32,
    141 as i32 as crate::opus_types_h::opus_uint32,
    143 as i32 as crate::opus_types_h::opus_uint32,
    145 as i32 as crate::opus_types_h::opus_uint32,
    147 as i32 as crate::opus_types_h::opus_uint32,
    149 as i32 as crate::opus_types_h::opus_uint32,
    151 as i32 as crate::opus_types_h::opus_uint32,
    153 as i32 as crate::opus_types_h::opus_uint32,
    155 as i32 as crate::opus_types_h::opus_uint32,
    157 as i32 as crate::opus_types_h::opus_uint32,
    159 as i32 as crate::opus_types_h::opus_uint32,
    161 as i32 as crate::opus_types_h::opus_uint32,
    163 as i32 as crate::opus_types_h::opus_uint32,
    165 as i32 as crate::opus_types_h::opus_uint32,
    167 as i32 as crate::opus_types_h::opus_uint32,
    169 as i32 as crate::opus_types_h::opus_uint32,
    171 as i32 as crate::opus_types_h::opus_uint32,
    173 as i32 as crate::opus_types_h::opus_uint32,
    175 as i32 as crate::opus_types_h::opus_uint32,
    177 as i32 as crate::opus_types_h::opus_uint32,
    179 as i32 as crate::opus_types_h::opus_uint32,
    181 as i32 as crate::opus_types_h::opus_uint32,
    183 as i32 as crate::opus_types_h::opus_uint32,
    185 as i32 as crate::opus_types_h::opus_uint32,
    187 as i32 as crate::opus_types_h::opus_uint32,
    189 as i32 as crate::opus_types_h::opus_uint32,
    191 as i32 as crate::opus_types_h::opus_uint32,
    193 as i32 as crate::opus_types_h::opus_uint32,
    195 as i32 as crate::opus_types_h::opus_uint32,
    197 as i32 as crate::opus_types_h::opus_uint32,
    199 as i32 as crate::opus_types_h::opus_uint32,
    201 as i32 as crate::opus_types_h::opus_uint32,
    203 as i32 as crate::opus_types_h::opus_uint32,
    205 as i32 as crate::opus_types_h::opus_uint32,
    207 as i32 as crate::opus_types_h::opus_uint32,
    209 as i32 as crate::opus_types_h::opus_uint32,
    211 as i32 as crate::opus_types_h::opus_uint32,
    213 as i32 as crate::opus_types_h::opus_uint32,
    215 as i32 as crate::opus_types_h::opus_uint32,
    217 as i32 as crate::opus_types_h::opus_uint32,
    219 as i32 as crate::opus_types_h::opus_uint32,
    221 as i32 as crate::opus_types_h::opus_uint32,
    223 as i32 as crate::opus_types_h::opus_uint32,
    225 as i32 as crate::opus_types_h::opus_uint32,
    227 as i32 as crate::opus_types_h::opus_uint32,
    229 as i32 as crate::opus_types_h::opus_uint32,
    231 as i32 as crate::opus_types_h::opus_uint32,
    233 as i32 as crate::opus_types_h::opus_uint32,
    235 as i32 as crate::opus_types_h::opus_uint32,
    237 as i32 as crate::opus_types_h::opus_uint32,
    239 as i32 as crate::opus_types_h::opus_uint32,
    241 as i32 as crate::opus_types_h::opus_uint32,
    243 as i32 as crate::opus_types_h::opus_uint32,
    245 as i32 as crate::opus_types_h::opus_uint32,
    247 as i32 as crate::opus_types_h::opus_uint32,
    249 as i32 as crate::opus_types_h::opus_uint32,
    251 as i32 as crate::opus_types_h::opus_uint32,
    253 as i32 as crate::opus_types_h::opus_uint32,
    255 as i32 as crate::opus_types_h::opus_uint32,
    257 as i32 as crate::opus_types_h::opus_uint32,
    259 as i32 as crate::opus_types_h::opus_uint32,
    261 as i32 as crate::opus_types_h::opus_uint32,
    263 as i32 as crate::opus_types_h::opus_uint32,
    265 as i32 as crate::opus_types_h::opus_uint32,
    267 as i32 as crate::opus_types_h::opus_uint32,
    269 as i32 as crate::opus_types_h::opus_uint32,
    271 as i32 as crate::opus_types_h::opus_uint32,
    273 as i32 as crate::opus_types_h::opus_uint32,
    275 as i32 as crate::opus_types_h::opus_uint32,
    277 as i32 as crate::opus_types_h::opus_uint32,
    279 as i32 as crate::opus_types_h::opus_uint32,
    281 as i32 as crate::opus_types_h::opus_uint32,
    283 as i32 as crate::opus_types_h::opus_uint32,
    285 as i32 as crate::opus_types_h::opus_uint32,
    287 as i32 as crate::opus_types_h::opus_uint32,
    289 as i32 as crate::opus_types_h::opus_uint32,
    291 as i32 as crate::opus_types_h::opus_uint32,
    293 as i32 as crate::opus_types_h::opus_uint32,
    295 as i32 as crate::opus_types_h::opus_uint32,
    297 as i32 as crate::opus_types_h::opus_uint32,
    299 as i32 as crate::opus_types_h::opus_uint32,
    301 as i32 as crate::opus_types_h::opus_uint32,
    303 as i32 as crate::opus_types_h::opus_uint32,
    305 as i32 as crate::opus_types_h::opus_uint32,
    307 as i32 as crate::opus_types_h::opus_uint32,
    309 as i32 as crate::opus_types_h::opus_uint32,
    311 as i32 as crate::opus_types_h::opus_uint32,
    313 as i32 as crate::opus_types_h::opus_uint32,
    315 as i32 as crate::opus_types_h::opus_uint32,
    317 as i32 as crate::opus_types_h::opus_uint32,
    319 as i32 as crate::opus_types_h::opus_uint32,
    321 as i32 as crate::opus_types_h::opus_uint32,
    323 as i32 as crate::opus_types_h::opus_uint32,
    325 as i32 as crate::opus_types_h::opus_uint32,
    327 as i32 as crate::opus_types_h::opus_uint32,
    329 as i32 as crate::opus_types_h::opus_uint32,
    331 as i32 as crate::opus_types_h::opus_uint32,
    333 as i32 as crate::opus_types_h::opus_uint32,
    335 as i32 as crate::opus_types_h::opus_uint32,
    337 as i32 as crate::opus_types_h::opus_uint32,
    339 as i32 as crate::opus_types_h::opus_uint32,
    341 as i32 as crate::opus_types_h::opus_uint32,
    343 as i32 as crate::opus_types_h::opus_uint32,
    345 as i32 as crate::opus_types_h::opus_uint32,
    347 as i32 as crate::opus_types_h::opus_uint32,
    349 as i32 as crate::opus_types_h::opus_uint32,
    351 as i32 as crate::opus_types_h::opus_uint32,
    13 as i32 as crate::opus_types_h::opus_uint32,
    25 as i32 as crate::opus_types_h::opus_uint32,
    41 as i32 as crate::opus_types_h::opus_uint32,
    61 as i32 as crate::opus_types_h::opus_uint32,
    85 as i32 as crate::opus_types_h::opus_uint32,
    113 as i32 as crate::opus_types_h::opus_uint32,
    145 as i32 as crate::opus_types_h::opus_uint32,
    181 as i32 as crate::opus_types_h::opus_uint32,
    221 as i32 as crate::opus_types_h::opus_uint32,
    265 as i32 as crate::opus_types_h::opus_uint32,
    313 as i32 as crate::opus_types_h::opus_uint32,
    365 as i32 as crate::opus_types_h::opus_uint32,
    421 as i32 as crate::opus_types_h::opus_uint32,
    481 as i32 as crate::opus_types_h::opus_uint32,
    545 as i32 as crate::opus_types_h::opus_uint32,
    613 as i32 as crate::opus_types_h::opus_uint32,
    685 as i32 as crate::opus_types_h::opus_uint32,
    761 as i32 as crate::opus_types_h::opus_uint32,
    841 as i32 as crate::opus_types_h::opus_uint32,
    925 as i32 as crate::opus_types_h::opus_uint32,
    1013 as i32 as crate::opus_types_h::opus_uint32,
    1105 as i32 as crate::opus_types_h::opus_uint32,
    1201 as i32 as crate::opus_types_h::opus_uint32,
    1301 as i32 as crate::opus_types_h::opus_uint32,
    1405 as i32 as crate::opus_types_h::opus_uint32,
    1513 as i32 as crate::opus_types_h::opus_uint32,
    1625 as i32 as crate::opus_types_h::opus_uint32,
    1741 as i32 as crate::opus_types_h::opus_uint32,
    1861 as i32 as crate::opus_types_h::opus_uint32,
    1985 as i32 as crate::opus_types_h::opus_uint32,
    2113 as i32 as crate::opus_types_h::opus_uint32,
    2245 as i32 as crate::opus_types_h::opus_uint32,
    2381 as i32 as crate::opus_types_h::opus_uint32,
    2521 as i32 as crate::opus_types_h::opus_uint32,
    2665 as i32 as crate::opus_types_h::opus_uint32,
    2813 as i32 as crate::opus_types_h::opus_uint32,
    2965 as i32 as crate::opus_types_h::opus_uint32,
    3121 as i32 as crate::opus_types_h::opus_uint32,
    3281 as i32 as crate::opus_types_h::opus_uint32,
    3445 as i32 as crate::opus_types_h::opus_uint32,
    3613 as i32 as crate::opus_types_h::opus_uint32,
    3785 as i32 as crate::opus_types_h::opus_uint32,
    3961 as i32 as crate::opus_types_h::opus_uint32,
    4141 as i32 as crate::opus_types_h::opus_uint32,
    4325 as i32 as crate::opus_types_h::opus_uint32,
    4513 as i32 as crate::opus_types_h::opus_uint32,
    4705 as i32 as crate::opus_types_h::opus_uint32,
    4901 as i32 as crate::opus_types_h::opus_uint32,
    5101 as i32 as crate::opus_types_h::opus_uint32,
    5305 as i32 as crate::opus_types_h::opus_uint32,
    5513 as i32 as crate::opus_types_h::opus_uint32,
    5725 as i32 as crate::opus_types_h::opus_uint32,
    5941 as i32 as crate::opus_types_h::opus_uint32,
    6161 as i32 as crate::opus_types_h::opus_uint32,
    6385 as i32 as crate::opus_types_h::opus_uint32,
    6613 as i32 as crate::opus_types_h::opus_uint32,
    6845 as i32 as crate::opus_types_h::opus_uint32,
    7081 as i32 as crate::opus_types_h::opus_uint32,
    7321 as i32 as crate::opus_types_h::opus_uint32,
    7565 as i32 as crate::opus_types_h::opus_uint32,
    7813 as i32 as crate::opus_types_h::opus_uint32,
    8065 as i32 as crate::opus_types_h::opus_uint32,
    8321 as i32 as crate::opus_types_h::opus_uint32,
    8581 as i32 as crate::opus_types_h::opus_uint32,
    8845 as i32 as crate::opus_types_h::opus_uint32,
    9113 as i32 as crate::opus_types_h::opus_uint32,
    9385 as i32 as crate::opus_types_h::opus_uint32,
    9661 as i32 as crate::opus_types_h::opus_uint32,
    9941 as i32 as crate::opus_types_h::opus_uint32,
    10225 as i32 as crate::opus_types_h::opus_uint32,
    10513 as i32 as crate::opus_types_h::opus_uint32,
    10805 as i32 as crate::opus_types_h::opus_uint32,
    11101 as i32 as crate::opus_types_h::opus_uint32,
    11401 as i32 as crate::opus_types_h::opus_uint32,
    11705 as i32 as crate::opus_types_h::opus_uint32,
    12013 as i32 as crate::opus_types_h::opus_uint32,
    12325 as i32 as crate::opus_types_h::opus_uint32,
    12641 as i32 as crate::opus_types_h::opus_uint32,
    12961 as i32 as crate::opus_types_h::opus_uint32,
    13285 as i32 as crate::opus_types_h::opus_uint32,
    13613 as i32 as crate::opus_types_h::opus_uint32,
    13945 as i32 as crate::opus_types_h::opus_uint32,
    14281 as i32 as crate::opus_types_h::opus_uint32,
    14621 as i32 as crate::opus_types_h::opus_uint32,
    14965 as i32 as crate::opus_types_h::opus_uint32,
    15313 as i32 as crate::opus_types_h::opus_uint32,
    15665 as i32 as crate::opus_types_h::opus_uint32,
    16021 as i32 as crate::opus_types_h::opus_uint32,
    16381 as i32 as crate::opus_types_h::opus_uint32,
    16745 as i32 as crate::opus_types_h::opus_uint32,
    17113 as i32 as crate::opus_types_h::opus_uint32,
    17485 as i32 as crate::opus_types_h::opus_uint32,
    17861 as i32 as crate::opus_types_h::opus_uint32,
    18241 as i32 as crate::opus_types_h::opus_uint32,
    18625 as i32 as crate::opus_types_h::opus_uint32,
    19013 as i32 as crate::opus_types_h::opus_uint32,
    19405 as i32 as crate::opus_types_h::opus_uint32,
    19801 as i32 as crate::opus_types_h::opus_uint32,
    20201 as i32 as crate::opus_types_h::opus_uint32,
    20605 as i32 as crate::opus_types_h::opus_uint32,
    21013 as i32 as crate::opus_types_h::opus_uint32,
    21425 as i32 as crate::opus_types_h::opus_uint32,
    21841 as i32 as crate::opus_types_h::opus_uint32,
    22261 as i32 as crate::opus_types_h::opus_uint32,
    22685 as i32 as crate::opus_types_h::opus_uint32,
    23113 as i32 as crate::opus_types_h::opus_uint32,
    23545 as i32 as crate::opus_types_h::opus_uint32,
    23981 as i32 as crate::opus_types_h::opus_uint32,
    24421 as i32 as crate::opus_types_h::opus_uint32,
    24865 as i32 as crate::opus_types_h::opus_uint32,
    25313 as i32 as crate::opus_types_h::opus_uint32,
    25765 as i32 as crate::opus_types_h::opus_uint32,
    26221 as i32 as crate::opus_types_h::opus_uint32,
    26681 as i32 as crate::opus_types_h::opus_uint32,
    27145 as i32 as crate::opus_types_h::opus_uint32,
    27613 as i32 as crate::opus_types_h::opus_uint32,
    28085 as i32 as crate::opus_types_h::opus_uint32,
    28561 as i32 as crate::opus_types_h::opus_uint32,
    29041 as i32 as crate::opus_types_h::opus_uint32,
    29525 as i32 as crate::opus_types_h::opus_uint32,
    30013 as i32 as crate::opus_types_h::opus_uint32,
    30505 as i32 as crate::opus_types_h::opus_uint32,
    31001 as i32 as crate::opus_types_h::opus_uint32,
    31501 as i32 as crate::opus_types_h::opus_uint32,
    32005 as i32 as crate::opus_types_h::opus_uint32,
    32513 as i32 as crate::opus_types_h::opus_uint32,
    33025 as i32 as crate::opus_types_h::opus_uint32,
    33541 as i32 as crate::opus_types_h::opus_uint32,
    34061 as i32 as crate::opus_types_h::opus_uint32,
    34585 as i32 as crate::opus_types_h::opus_uint32,
    35113 as i32 as crate::opus_types_h::opus_uint32,
    35645 as i32 as crate::opus_types_h::opus_uint32,
    36181 as i32 as crate::opus_types_h::opus_uint32,
    36721 as i32 as crate::opus_types_h::opus_uint32,
    37265 as i32 as crate::opus_types_h::opus_uint32,
    37813 as i32 as crate::opus_types_h::opus_uint32,
    38365 as i32 as crate::opus_types_h::opus_uint32,
    38921 as i32 as crate::opus_types_h::opus_uint32,
    39481 as i32 as crate::opus_types_h::opus_uint32,
    40045 as i32 as crate::opus_types_h::opus_uint32,
    40613 as i32 as crate::opus_types_h::opus_uint32,
    41185 as i32 as crate::opus_types_h::opus_uint32,
    41761 as i32 as crate::opus_types_h::opus_uint32,
    42341 as i32 as crate::opus_types_h::opus_uint32,
    42925 as i32 as crate::opus_types_h::opus_uint32,
    43513 as i32 as crate::opus_types_h::opus_uint32,
    44105 as i32 as crate::opus_types_h::opus_uint32,
    44701 as i32 as crate::opus_types_h::opus_uint32,
    45301 as i32 as crate::opus_types_h::opus_uint32,
    45905 as i32 as crate::opus_types_h::opus_uint32,
    46513 as i32 as crate::opus_types_h::opus_uint32,
    47125 as i32 as crate::opus_types_h::opus_uint32,
    47741 as i32 as crate::opus_types_h::opus_uint32,
    48361 as i32 as crate::opus_types_h::opus_uint32,
    48985 as i32 as crate::opus_types_h::opus_uint32,
    49613 as i32 as crate::opus_types_h::opus_uint32,
    50245 as i32 as crate::opus_types_h::opus_uint32,
    50881 as i32 as crate::opus_types_h::opus_uint32,
    51521 as i32 as crate::opus_types_h::opus_uint32,
    52165 as i32 as crate::opus_types_h::opus_uint32,
    52813 as i32 as crate::opus_types_h::opus_uint32,
    53465 as i32 as crate::opus_types_h::opus_uint32,
    54121 as i32 as crate::opus_types_h::opus_uint32,
    54781 as i32 as crate::opus_types_h::opus_uint32,
    55445 as i32 as crate::opus_types_h::opus_uint32,
    56113 as i32 as crate::opus_types_h::opus_uint32,
    56785 as i32 as crate::opus_types_h::opus_uint32,
    57461 as i32 as crate::opus_types_h::opus_uint32,
    58141 as i32 as crate::opus_types_h::opus_uint32,
    58825 as i32 as crate::opus_types_h::opus_uint32,
    59513 as i32 as crate::opus_types_h::opus_uint32,
    60205 as i32 as crate::opus_types_h::opus_uint32,
    60901 as i32 as crate::opus_types_h::opus_uint32,
    61601 as i32 as crate::opus_types_h::opus_uint32,
    63 as i32 as crate::opus_types_h::opus_uint32,
    129 as i32 as crate::opus_types_h::opus_uint32,
    231 as i32 as crate::opus_types_h::opus_uint32,
    377 as i32 as crate::opus_types_h::opus_uint32,
    575 as i32 as crate::opus_types_h::opus_uint32,
    833 as i32 as crate::opus_types_h::opus_uint32,
    1159 as i32 as crate::opus_types_h::opus_uint32,
    1561 as i32 as crate::opus_types_h::opus_uint32,
    2047 as i32 as crate::opus_types_h::opus_uint32,
    2625 as i32 as crate::opus_types_h::opus_uint32,
    3303 as i32 as crate::opus_types_h::opus_uint32,
    4089 as i32 as crate::opus_types_h::opus_uint32,
    4991 as i32 as crate::opus_types_h::opus_uint32,
    6017 as i32 as crate::opus_types_h::opus_uint32,
    7175 as i32 as crate::opus_types_h::opus_uint32,
    8473 as i32 as crate::opus_types_h::opus_uint32,
    9919 as i32 as crate::opus_types_h::opus_uint32,
    11521 as i32 as crate::opus_types_h::opus_uint32,
    13287 as i32 as crate::opus_types_h::opus_uint32,
    15225 as i32 as crate::opus_types_h::opus_uint32,
    17343 as i32 as crate::opus_types_h::opus_uint32,
    19649 as i32 as crate::opus_types_h::opus_uint32,
    22151 as i32 as crate::opus_types_h::opus_uint32,
    24857 as i32 as crate::opus_types_h::opus_uint32,
    27775 as i32 as crate::opus_types_h::opus_uint32,
    30913 as i32 as crate::opus_types_h::opus_uint32,
    34279 as i32 as crate::opus_types_h::opus_uint32,
    37881 as i32 as crate::opus_types_h::opus_uint32,
    41727 as i32 as crate::opus_types_h::opus_uint32,
    45825 as i32 as crate::opus_types_h::opus_uint32,
    50183 as i32 as crate::opus_types_h::opus_uint32,
    54809 as i32 as crate::opus_types_h::opus_uint32,
    59711 as i32 as crate::opus_types_h::opus_uint32,
    64897 as i32 as crate::opus_types_h::opus_uint32,
    70375 as i32 as crate::opus_types_h::opus_uint32,
    76153 as i32 as crate::opus_types_h::opus_uint32,
    82239 as i32 as crate::opus_types_h::opus_uint32,
    88641 as i32 as crate::opus_types_h::opus_uint32,
    95367 as i32 as crate::opus_types_h::opus_uint32,
    102425 as i32 as crate::opus_types_h::opus_uint32,
    109823 as i32 as crate::opus_types_h::opus_uint32,
    117569 as i32 as crate::opus_types_h::opus_uint32,
    125671 as i32 as crate::opus_types_h::opus_uint32,
    134137 as i32 as crate::opus_types_h::opus_uint32,
    142975 as i32 as crate::opus_types_h::opus_uint32,
    152193 as i32 as crate::opus_types_h::opus_uint32,
    161799 as i32 as crate::opus_types_h::opus_uint32,
    171801 as i32 as crate::opus_types_h::opus_uint32,
    182207 as i32 as crate::opus_types_h::opus_uint32,
    193025 as i32 as crate::opus_types_h::opus_uint32,
    204263 as i32 as crate::opus_types_h::opus_uint32,
    215929 as i32 as crate::opus_types_h::opus_uint32,
    228031 as i32 as crate::opus_types_h::opus_uint32,
    240577 as i32 as crate::opus_types_h::opus_uint32,
    253575 as i32 as crate::opus_types_h::opus_uint32,
    267033 as i32 as crate::opus_types_h::opus_uint32,
    280959 as i32 as crate::opus_types_h::opus_uint32,
    295361 as i32 as crate::opus_types_h::opus_uint32,
    310247 as i32 as crate::opus_types_h::opus_uint32,
    325625 as i32 as crate::opus_types_h::opus_uint32,
    341503 as i32 as crate::opus_types_h::opus_uint32,
    357889 as i32 as crate::opus_types_h::opus_uint32,
    374791 as i32 as crate::opus_types_h::opus_uint32,
    392217 as i32 as crate::opus_types_h::opus_uint32,
    410175 as i32 as crate::opus_types_h::opus_uint32,
    428673 as i32 as crate::opus_types_h::opus_uint32,
    447719 as i32 as crate::opus_types_h::opus_uint32,
    467321 as i32 as crate::opus_types_h::opus_uint32,
    487487 as i32 as crate::opus_types_h::opus_uint32,
    508225 as i32 as crate::opus_types_h::opus_uint32,
    529543 as i32 as crate::opus_types_h::opus_uint32,
    551449 as i32 as crate::opus_types_h::opus_uint32,
    573951 as i32 as crate::opus_types_h::opus_uint32,
    597057 as i32 as crate::opus_types_h::opus_uint32,
    620775 as i32 as crate::opus_types_h::opus_uint32,
    645113 as i32 as crate::opus_types_h::opus_uint32,
    670079 as i32 as crate::opus_types_h::opus_uint32,
    695681 as i32 as crate::opus_types_h::opus_uint32,
    721927 as i32 as crate::opus_types_h::opus_uint32,
    748825 as i32 as crate::opus_types_h::opus_uint32,
    776383 as i32 as crate::opus_types_h::opus_uint32,
    804609 as i32 as crate::opus_types_h::opus_uint32,
    833511 as i32 as crate::opus_types_h::opus_uint32,
    863097 as i32 as crate::opus_types_h::opus_uint32,
    893375 as i32 as crate::opus_types_h::opus_uint32,
    924353 as i32 as crate::opus_types_h::opus_uint32,
    956039 as i32 as crate::opus_types_h::opus_uint32,
    988441 as i32 as crate::opus_types_h::opus_uint32,
    1021567 as i32 as crate::opus_types_h::opus_uint32,
    1055425 as i32 as crate::opus_types_h::opus_uint32,
    1090023 as i32 as crate::opus_types_h::opus_uint32,
    1125369 as i32 as crate::opus_types_h::opus_uint32,
    1161471 as i32 as crate::opus_types_h::opus_uint32,
    1198337 as i32 as crate::opus_types_h::opus_uint32,
    1235975 as i32 as crate::opus_types_h::opus_uint32,
    1274393 as i32 as crate::opus_types_h::opus_uint32,
    1313599 as i32 as crate::opus_types_h::opus_uint32,
    1353601 as i32 as crate::opus_types_h::opus_uint32,
    1394407 as i32 as crate::opus_types_h::opus_uint32,
    1436025 as i32 as crate::opus_types_h::opus_uint32,
    1478463 as i32 as crate::opus_types_h::opus_uint32,
    1521729 as i32 as crate::opus_types_h::opus_uint32,
    1565831 as i32 as crate::opus_types_h::opus_uint32,
    1610777 as i32 as crate::opus_types_h::opus_uint32,
    1656575 as i32 as crate::opus_types_h::opus_uint32,
    1703233 as i32 as crate::opus_types_h::opus_uint32,
    1750759 as i32 as crate::opus_types_h::opus_uint32,
    1799161 as i32 as crate::opus_types_h::opus_uint32,
    1848447 as i32 as crate::opus_types_h::opus_uint32,
    1898625 as i32 as crate::opus_types_h::opus_uint32,
    1949703 as i32 as crate::opus_types_h::opus_uint32,
    2001689 as i32 as crate::opus_types_h::opus_uint32,
    2054591 as i32 as crate::opus_types_h::opus_uint32,
    2108417 as i32 as crate::opus_types_h::opus_uint32,
    2163175 as i32 as crate::opus_types_h::opus_uint32,
    2218873 as i32 as crate::opus_types_h::opus_uint32,
    2275519 as i32 as crate::opus_types_h::opus_uint32,
    2333121 as i32 as crate::opus_types_h::opus_uint32,
    2391687 as i32 as crate::opus_types_h::opus_uint32,
    2451225 as i32 as crate::opus_types_h::opus_uint32,
    2511743 as i32 as crate::opus_types_h::opus_uint32,
    2573249 as i32 as crate::opus_types_h::opus_uint32,
    2635751 as i32 as crate::opus_types_h::opus_uint32,
    2699257 as i32 as crate::opus_types_h::opus_uint32,
    2763775 as i32 as crate::opus_types_h::opus_uint32,
    2829313 as i32 as crate::opus_types_h::opus_uint32,
    2895879 as i32 as crate::opus_types_h::opus_uint32,
    2963481 as i32 as crate::opus_types_h::opus_uint32,
    3032127 as i32 as crate::opus_types_h::opus_uint32,
    3101825 as i32 as crate::opus_types_h::opus_uint32,
    3172583 as i32 as crate::opus_types_h::opus_uint32,
    3244409 as i32 as crate::opus_types_h::opus_uint32,
    3317311 as i32 as crate::opus_types_h::opus_uint32,
    3391297 as i32 as crate::opus_types_h::opus_uint32,
    3466375 as i32 as crate::opus_types_h::opus_uint32,
    3542553 as i32 as crate::opus_types_h::opus_uint32,
    3619839 as i32 as crate::opus_types_h::opus_uint32,
    3698241 as i32 as crate::opus_types_h::opus_uint32,
    3777767 as i32 as crate::opus_types_h::opus_uint32,
    3858425 as i32 as crate::opus_types_h::opus_uint32,
    3940223 as i32 as crate::opus_types_h::opus_uint32,
    4023169 as i32 as crate::opus_types_h::opus_uint32,
    4107271 as i32 as crate::opus_types_h::opus_uint32,
    4192537 as i32 as crate::opus_types_h::opus_uint32,
    4278975 as i32 as crate::opus_types_h::opus_uint32,
    4366593 as i32 as crate::opus_types_h::opus_uint32,
    4455399 as i32 as crate::opus_types_h::opus_uint32,
    4545401 as i32 as crate::opus_types_h::opus_uint32,
    4636607 as i32 as crate::opus_types_h::opus_uint32,
    4729025 as i32 as crate::opus_types_h::opus_uint32,
    4822663 as i32 as crate::opus_types_h::opus_uint32,
    4917529 as i32 as crate::opus_types_h::opus_uint32,
    5013631 as i32 as crate::opus_types_h::opus_uint32,
    5110977 as i32 as crate::opus_types_h::opus_uint32,
    5209575 as i32 as crate::opus_types_h::opus_uint32,
    5309433 as i32 as crate::opus_types_h::opus_uint32,
    5410559 as i32 as crate::opus_types_h::opus_uint32,
    5512961 as i32 as crate::opus_types_h::opus_uint32,
    5616647 as i32 as crate::opus_types_h::opus_uint32,
    5721625 as i32 as crate::opus_types_h::opus_uint32,
    5827903 as i32 as crate::opus_types_h::opus_uint32,
    5935489 as i32 as crate::opus_types_h::opus_uint32,
    6044391 as i32 as crate::opus_types_h::opus_uint32,
    6154617 as i32 as crate::opus_types_h::opus_uint32,
    6266175 as i32 as crate::opus_types_h::opus_uint32,
    6379073 as i32 as crate::opus_types_h::opus_uint32,
    6493319 as i32 as crate::opus_types_h::opus_uint32,
    6608921 as i32 as crate::opus_types_h::opus_uint32,
    6725887 as i32 as crate::opus_types_h::opus_uint32,
    6844225 as i32 as crate::opus_types_h::opus_uint32,
    6963943 as i32 as crate::opus_types_h::opus_uint32,
    7085049 as i32 as crate::opus_types_h::opus_uint32,
    7207551 as i32 as crate::opus_types_h::opus_uint32,
    321 as i32 as crate::opus_types_h::opus_uint32,
    681 as i32 as crate::opus_types_h::opus_uint32,
    1289 as i32 as crate::opus_types_h::opus_uint32,
    2241 as i32 as crate::opus_types_h::opus_uint32,
    3649 as i32 as crate::opus_types_h::opus_uint32,
    5641 as i32 as crate::opus_types_h::opus_uint32,
    8361 as i32 as crate::opus_types_h::opus_uint32,
    11969 as i32 as crate::opus_types_h::opus_uint32,
    16641 as i32 as crate::opus_types_h::opus_uint32,
    22569 as i32 as crate::opus_types_h::opus_uint32,
    29961 as i32 as crate::opus_types_h::opus_uint32,
    39041 as i32 as crate::opus_types_h::opus_uint32,
    50049 as i32 as crate::opus_types_h::opus_uint32,
    63241 as i32 as crate::opus_types_h::opus_uint32,
    78889 as i32 as crate::opus_types_h::opus_uint32,
    97281 as i32 as crate::opus_types_h::opus_uint32,
    118721 as i32 as crate::opus_types_h::opus_uint32,
    143529 as i32 as crate::opus_types_h::opus_uint32,
    172041 as i32 as crate::opus_types_h::opus_uint32,
    204609 as i32 as crate::opus_types_h::opus_uint32,
    241601 as i32 as crate::opus_types_h::opus_uint32,
    283401 as i32 as crate::opus_types_h::opus_uint32,
    330409 as i32 as crate::opus_types_h::opus_uint32,
    383041 as i32 as crate::opus_types_h::opus_uint32,
    441729 as i32 as crate::opus_types_h::opus_uint32,
    506921 as i32 as crate::opus_types_h::opus_uint32,
    579081 as i32 as crate::opus_types_h::opus_uint32,
    658689 as i32 as crate::opus_types_h::opus_uint32,
    746241 as i32 as crate::opus_types_h::opus_uint32,
    842249 as i32 as crate::opus_types_h::opus_uint32,
    947241 as i32 as crate::opus_types_h::opus_uint32,
    1061761 as i32 as crate::opus_types_h::opus_uint32,
    1186369 as i32 as crate::opus_types_h::opus_uint32,
    1321641 as i32 as crate::opus_types_h::opus_uint32,
    1468169 as i32 as crate::opus_types_h::opus_uint32,
    1626561 as i32 as crate::opus_types_h::opus_uint32,
    1797441 as i32 as crate::opus_types_h::opus_uint32,
    1981449 as i32 as crate::opus_types_h::opus_uint32,
    2179241 as i32 as crate::opus_types_h::opus_uint32,
    2391489 as i32 as crate::opus_types_h::opus_uint32,
    2618881 as i32 as crate::opus_types_h::opus_uint32,
    2862121 as i32 as crate::opus_types_h::opus_uint32,
    3121929 as i32 as crate::opus_types_h::opus_uint32,
    3399041 as i32 as crate::opus_types_h::opus_uint32,
    3694209 as i32 as crate::opus_types_h::opus_uint32,
    4008201 as i32 as crate::opus_types_h::opus_uint32,
    4341801 as i32 as crate::opus_types_h::opus_uint32,
    4695809 as i32 as crate::opus_types_h::opus_uint32,
    5071041 as i32 as crate::opus_types_h::opus_uint32,
    5468329 as i32 as crate::opus_types_h::opus_uint32,
    5888521 as i32 as crate::opus_types_h::opus_uint32,
    6332481 as i32 as crate::opus_types_h::opus_uint32,
    6801089 as i32 as crate::opus_types_h::opus_uint32,
    7295241 as i32 as crate::opus_types_h::opus_uint32,
    7815849 as i32 as crate::opus_types_h::opus_uint32,
    8363841 as i32 as crate::opus_types_h::opus_uint32,
    8940161 as i32 as crate::opus_types_h::opus_uint32,
    9545769 as i32 as crate::opus_types_h::opus_uint32,
    10181641 as i32 as crate::opus_types_h::opus_uint32,
    10848769 as i32 as crate::opus_types_h::opus_uint32,
    11548161 as i32 as crate::opus_types_h::opus_uint32,
    12280841 as i32 as crate::opus_types_h::opus_uint32,
    13047849 as i32 as crate::opus_types_h::opus_uint32,
    13850241 as i32 as crate::opus_types_h::opus_uint32,
    14689089 as i32 as crate::opus_types_h::opus_uint32,
    15565481 as i32 as crate::opus_types_h::opus_uint32,
    16480521 as i32 as crate::opus_types_h::opus_uint32,
    17435329 as i32 as crate::opus_types_h::opus_uint32,
    18431041 as i32 as crate::opus_types_h::opus_uint32,
    19468809 as i32 as crate::opus_types_h::opus_uint32,
    20549801 as i32 as crate::opus_types_h::opus_uint32,
    21675201 as i32 as crate::opus_types_h::opus_uint32,
    22846209 as i32 as crate::opus_types_h::opus_uint32,
    24064041 as i32 as crate::opus_types_h::opus_uint32,
    25329929 as i32 as crate::opus_types_h::opus_uint32,
    26645121 as i32 as crate::opus_types_h::opus_uint32,
    28010881 as i32 as crate::opus_types_h::opus_uint32,
    29428489 as i32 as crate::opus_types_h::opus_uint32,
    30899241 as i32 as crate::opus_types_h::opus_uint32,
    32424449 as i32 as crate::opus_types_h::opus_uint32,
    34005441 as i32 as crate::opus_types_h::opus_uint32,
    35643561 as i32 as crate::opus_types_h::opus_uint32,
    37340169 as i32 as crate::opus_types_h::opus_uint32,
    39096641 as i32 as crate::opus_types_h::opus_uint32,
    40914369 as i32 as crate::opus_types_h::opus_uint32,
    42794761 as i32 as crate::opus_types_h::opus_uint32,
    44739241 as i32 as crate::opus_types_h::opus_uint32,
    46749249 as i32 as crate::opus_types_h::opus_uint32,
    48826241 as i32 as crate::opus_types_h::opus_uint32,
    50971689 as i32 as crate::opus_types_h::opus_uint32,
    53187081 as i32 as crate::opus_types_h::opus_uint32,
    55473921 as i32 as crate::opus_types_h::opus_uint32,
    57833729 as i32 as crate::opus_types_h::opus_uint32,
    60268041 as i32 as crate::opus_types_h::opus_uint32,
    62778409 as i32 as crate::opus_types_h::opus_uint32,
    65366401 as i32 as crate::opus_types_h::opus_uint32,
    68033601 as i32 as crate::opus_types_h::opus_uint32,
    70781609 as i32 as crate::opus_types_h::opus_uint32,
    73612041 as i32 as crate::opus_types_h::opus_uint32,
    76526529 as i32 as crate::opus_types_h::opus_uint32,
    79526721 as i32 as crate::opus_types_h::opus_uint32,
    82614281 as i32 as crate::opus_types_h::opus_uint32,
    85790889 as i32 as crate::opus_types_h::opus_uint32,
    89058241 as i32 as crate::opus_types_h::opus_uint32,
    92418049 as i32 as crate::opus_types_h::opus_uint32,
    95872041 as i32 as crate::opus_types_h::opus_uint32,
    99421961 as i32 as crate::opus_types_h::opus_uint32,
    103069569 as i32 as crate::opus_types_h::opus_uint32,
    106816641 as i32 as crate::opus_types_h::opus_uint32,
    110664969 as i32 as crate::opus_types_h::opus_uint32,
    114616361 as i32 as crate::opus_types_h::opus_uint32,
    118672641 as i32 as crate::opus_types_h::opus_uint32,
    122835649 as i32 as crate::opus_types_h::opus_uint32,
    127107241 as i32 as crate::opus_types_h::opus_uint32,
    131489289 as i32 as crate::opus_types_h::opus_uint32,
    135983681 as i32 as crate::opus_types_h::opus_uint32,
    140592321 as i32 as crate::opus_types_h::opus_uint32,
    145317129 as i32 as crate::opus_types_h::opus_uint32,
    150160041 as i32 as crate::opus_types_h::opus_uint32,
    155123009 as i32 as crate::opus_types_h::opus_uint32,
    160208001 as i32 as crate::opus_types_h::opus_uint32,
    165417001 as i32 as crate::opus_types_h::opus_uint32,
    170752009 as i32 as crate::opus_types_h::opus_uint32,
    176215041 as i32 as crate::opus_types_h::opus_uint32,
    181808129 as i32 as crate::opus_types_h::opus_uint32,
    187533321 as i32 as crate::opus_types_h::opus_uint32,
    193392681 as i32 as crate::opus_types_h::opus_uint32,
    199388289 as i32 as crate::opus_types_h::opus_uint32,
    205522241 as i32 as crate::opus_types_h::opus_uint32,
    211796649 as i32 as crate::opus_types_h::opus_uint32,
    218213641 as i32 as crate::opus_types_h::opus_uint32,
    224775361 as i32 as crate::opus_types_h::opus_uint32,
    231483969 as i32 as crate::opus_types_h::opus_uint32,
    238341641 as i32 as crate::opus_types_h::opus_uint32,
    245350569 as i32 as crate::opus_types_h::opus_uint32,
    252512961 as i32 as crate::opus_types_h::opus_uint32,
    259831041 as i32 as crate::opus_types_h::opus_uint32,
    267307049 as i32 as crate::opus_types_h::opus_uint32,
    274943241 as i32 as crate::opus_types_h::opus_uint32,
    282741889 as i32 as crate::opus_types_h::opus_uint32,
    290705281 as i32 as crate::opus_types_h::opus_uint32,
    298835721 as i32 as crate::opus_types_h::opus_uint32,
    307135529 as i32 as crate::opus_types_h::opus_uint32,
    315607041 as i32 as crate::opus_types_h::opus_uint32,
    324252609 as i32 as crate::opus_types_h::opus_uint32,
    333074601 as i32 as crate::opus_types_h::opus_uint32,
    342075401 as i32 as crate::opus_types_h::opus_uint32,
    351257409 as i32 as crate::opus_types_h::opus_uint32,
    360623041 as i32 as crate::opus_types_h::opus_uint32,
    370174729 as i32 as crate::opus_types_h::opus_uint32,
    379914921 as i32 as crate::opus_types_h::opus_uint32,
    389846081 as i32 as crate::opus_types_h::opus_uint32,
    399970689 as i32 as crate::opus_types_h::opus_uint32,
    410291241 as i32 as crate::opus_types_h::opus_uint32,
    420810249 as i32 as crate::opus_types_h::opus_uint32,
    431530241 as i32 as crate::opus_types_h::opus_uint32,
    442453761 as i32 as crate::opus_types_h::opus_uint32,
    453583369 as i32 as crate::opus_types_h::opus_uint32,
    464921641 as i32 as crate::opus_types_h::opus_uint32,
    476471169 as i32 as crate::opus_types_h::opus_uint32,
    488234561 as i32 as crate::opus_types_h::opus_uint32,
    500214441 as i32 as crate::opus_types_h::opus_uint32,
    512413449 as i32 as crate::opus_types_h::opus_uint32,
    524834241 as i32 as crate::opus_types_h::opus_uint32,
    537479489 as i32 as crate::opus_types_h::opus_uint32,
    550351881 as i32 as crate::opus_types_h::opus_uint32,
    563454121 as i32 as crate::opus_types_h::opus_uint32,
    576788929 as i32 as crate::opus_types_h::opus_uint32,
    590359041 as i32 as crate::opus_types_h::opus_uint32,
    604167209 as i32 as crate::opus_types_h::opus_uint32,
    618216201 as i32 as crate::opus_types_h::opus_uint32,
    632508801 as i32 as crate::opus_types_h::opus_uint32,
    1683 as i32 as crate::opus_types_h::opus_uint32,
    3653 as i32 as crate::opus_types_h::opus_uint32,
    7183 as i32 as crate::opus_types_h::opus_uint32,
    13073 as i32 as crate::opus_types_h::opus_uint32,
    22363 as i32 as crate::opus_types_h::opus_uint32,
    36365 as i32 as crate::opus_types_h::opus_uint32,
    56695 as i32 as crate::opus_types_h::opus_uint32,
    85305 as i32 as crate::opus_types_h::opus_uint32,
    124515 as i32 as crate::opus_types_h::opus_uint32,
    177045 as i32 as crate::opus_types_h::opus_uint32,
    246047 as i32 as crate::opus_types_h::opus_uint32,
    335137 as i32 as crate::opus_types_h::opus_uint32,
    448427 as i32 as crate::opus_types_h::opus_uint32,
    590557 as i32 as crate::opus_types_h::opus_uint32,
    766727 as i32 as crate::opus_types_h::opus_uint32,
    982729 as i32 as crate::opus_types_h::opus_uint32,
    1244979 as i32 as crate::opus_types_h::opus_uint32,
    1560549 as i32 as crate::opus_types_h::opus_uint32,
    1937199 as i32 as crate::opus_types_h::opus_uint32,
    2383409 as i32 as crate::opus_types_h::opus_uint32,
    2908411 as i32 as crate::opus_types_h::opus_uint32,
    3522221 as i32 as crate::opus_types_h::opus_uint32,
    4235671 as i32 as crate::opus_types_h::opus_uint32,
    5060441 as i32 as crate::opus_types_h::opus_uint32,
    6009091 as i32 as crate::opus_types_h::opus_uint32,
    7095093 as i32 as crate::opus_types_h::opus_uint32,
    8332863 as i32 as crate::opus_types_h::opus_uint32,
    9737793 as i32 as crate::opus_types_h::opus_uint32,
    11326283 as i32 as crate::opus_types_h::opus_uint32,
    13115773 as i32 as crate::opus_types_h::opus_uint32,
    15124775 as i32 as crate::opus_types_h::opus_uint32,
    17372905 as i32 as crate::opus_types_h::opus_uint32,
    19880915 as i32 as crate::opus_types_h::opus_uint32,
    22670725 as i32 as crate::opus_types_h::opus_uint32,
    25765455 as i32 as crate::opus_types_h::opus_uint32,
    29189457 as i32 as crate::opus_types_h::opus_uint32,
    32968347 as i32 as crate::opus_types_h::opus_uint32,
    37129037 as i32 as crate::opus_types_h::opus_uint32,
    41699767 as i32 as crate::opus_types_h::opus_uint32,
    46710137 as i32 as crate::opus_types_h::opus_uint32,
    52191139 as i32 as crate::opus_types_h::opus_uint32,
    58175189 as i32 as crate::opus_types_h::opus_uint32,
    64696159 as i32 as crate::opus_types_h::opus_uint32,
    71789409 as i32 as crate::opus_types_h::opus_uint32,
    79491819 as i32 as crate::opus_types_h::opus_uint32,
    87841821 as i32 as crate::opus_types_h::opus_uint32,
    96879431 as i32 as crate::opus_types_h::opus_uint32,
    106646281 as i32 as crate::opus_types_h::opus_uint32,
    117185651 as i32 as crate::opus_types_h::opus_uint32,
    128542501 as i32 as crate::opus_types_h::opus_uint32,
    140763503 as i32 as crate::opus_types_h::opus_uint32,
    153897073 as i32 as crate::opus_types_h::opus_uint32,
    167993403 as i32 as crate::opus_types_h::opus_uint32,
    183104493 as i32 as crate::opus_types_h::opus_uint32,
    199284183 as i32 as crate::opus_types_h::opus_uint32,
    216588185 as i32 as crate::opus_types_h::opus_uint32,
    235074115 as i32 as crate::opus_types_h::opus_uint32,
    254801525 as i32 as crate::opus_types_h::opus_uint32,
    275831935 as i32 as crate::opus_types_h::opus_uint32,
    298228865 as i32 as crate::opus_types_h::opus_uint32,
    322057867 as i32 as crate::opus_types_h::opus_uint32,
    347386557 as i32 as crate::opus_types_h::opus_uint32,
    374284647 as i32 as crate::opus_types_h::opus_uint32,
    402823977 as i32 as crate::opus_types_h::opus_uint32,
    433078547 as i32 as crate::opus_types_h::opus_uint32,
    465124549 as i32 as crate::opus_types_h::opus_uint32,
    499040399 as i32 as crate::opus_types_h::opus_uint32,
    534906769 as i32 as crate::opus_types_h::opus_uint32,
    572806619 as i32 as crate::opus_types_h::opus_uint32,
    612825229 as i32 as crate::opus_types_h::opus_uint32,
    655050231 as i32 as crate::opus_types_h::opus_uint32,
    699571641 as i32 as crate::opus_types_h::opus_uint32,
    746481891 as i32 as crate::opus_types_h::opus_uint32,
    795875861 as i32 as crate::opus_types_h::opus_uint32,
    847850911 as i32 as crate::opus_types_h::opus_uint32,
    902506913 as i32 as crate::opus_types_h::opus_uint32,
    959946283 as i32 as crate::opus_types_h::opus_uint32,
    1020274013 as i32 as crate::opus_types_h::opus_uint32,
    1083597703 as i32 as crate::opus_types_h::opus_uint32,
    1150027593 as i32 as crate::opus_types_h::opus_uint32,
    1219676595 as i32 as crate::opus_types_h::opus_uint32,
    1292660325 as i32 as crate::opus_types_h::opus_uint32,
    1369097135 as i32 as crate::opus_types_h::opus_uint32,
    1449108145 as i32 as crate::opus_types_h::opus_uint32,
    1532817275 as i32 as crate::opus_types_h::opus_uint32,
    1620351277 as i32 as crate::opus_types_h::opus_uint32,
    1711839767 as i32 as crate::opus_types_h::opus_uint32,
    1807415257 as i32 as crate::opus_types_h::opus_uint32,
    1907213187 as i32 as crate::opus_types_h::opus_uint32,
    2011371957 as i32 as crate::opus_types_h::opus_uint32,
    2120032959 as i32 as crate::opus_types_h::opus_uint32,
    8989 as i32 as crate::opus_types_h::opus_uint32,
    19825 as i32 as crate::opus_types_h::opus_uint32,
    40081 as i32 as crate::opus_types_h::opus_uint32,
    75517 as i32 as crate::opus_types_h::opus_uint32,
    134245 as i32 as crate::opus_types_h::opus_uint32,
    227305 as i32 as crate::opus_types_h::opus_uint32,
    369305 as i32 as crate::opus_types_h::opus_uint32,
    579125 as i32 as crate::opus_types_h::opus_uint32,
    880685 as i32 as crate::opus_types_h::opus_uint32,
    1303777 as i32 as crate::opus_types_h::opus_uint32,
    1884961 as i32 as crate::opus_types_h::opus_uint32,
    2668525 as i32 as crate::opus_types_h::opus_uint32,
    3707509 as i32 as crate::opus_types_h::opus_uint32,
    5064793 as i32 as crate::opus_types_h::opus_uint32,
    6814249 as i32 as crate::opus_types_h::opus_uint32,
    9041957 as i32 as crate::opus_types_h::opus_uint32,
    11847485 as i32 as crate::opus_types_h::opus_uint32,
    15345233 as i32 as crate::opus_types_h::opus_uint32,
    19665841 as i32 as crate::opus_types_h::opus_uint32,
    24957661 as i32 as crate::opus_types_h::opus_uint32,
    31388293 as i32 as crate::opus_types_h::opus_uint32,
    39146185 as i32 as crate::opus_types_h::opus_uint32,
    48442297 as i32 as crate::opus_types_h::opus_uint32,
    59511829 as i32 as crate::opus_types_h::opus_uint32,
    72616013 as i32 as crate::opus_types_h::opus_uint32,
    88043969 as i32 as crate::opus_types_h::opus_uint32,
    106114625 as i32 as crate::opus_types_h::opus_uint32,
    127178701 as i32 as crate::opus_types_h::opus_uint32,
    151620757 as i32 as crate::opus_types_h::opus_uint32,
    179861305 as i32 as crate::opus_types_h::opus_uint32,
    212358985 as i32 as crate::opus_types_h::opus_uint32,
    249612805 as i32 as crate::opus_types_h::opus_uint32,
    292164445 as i32 as crate::opus_types_h::opus_uint32,
    340600625 as i32 as crate::opus_types_h::opus_uint32,
    395555537 as i32 as crate::opus_types_h::opus_uint32,
    457713341 as i32 as crate::opus_types_h::opus_uint32,
    527810725 as i32 as crate::opus_types_h::opus_uint32,
    606639529 as i32 as crate::opus_types_h::opus_uint32,
    695049433 as i32 as crate::opus_types_h::opus_uint32,
    793950709 as i32 as crate::opus_types_h::opus_uint32,
    904317037 as i32 as crate::opus_types_h::opus_uint32,
    1027188385 as i32 as crate::opus_types_h::opus_uint32,
    1163673953 as i32 as crate::opus_types_h::opus_uint32,
    1314955181 as i32 as crate::opus_types_h::opus_uint32,
    1482288821 as i32 as crate::opus_types_h::opus_uint32,
    1667010073 as i32 as crate::opus_types_h::opus_uint32,
    1870535785 as i32 as crate::opus_types_h::opus_uint32,
    2094367717 as i32 as crate::opus_types_h::opus_uint32,
    48639 as i32 as crate::opus_types_h::opus_uint32,
    108545 as i32 as crate::opus_types_h::opus_uint32,
    224143 as i32 as crate::opus_types_h::opus_uint32,
    433905 as i32 as crate::opus_types_h::opus_uint32,
    795455 as i32 as crate::opus_types_h::opus_uint32,
    1392065 as i32 as crate::opus_types_h::opus_uint32,
    2340495 as i32 as crate::opus_types_h::opus_uint32,
    3800305 as i32 as crate::opus_types_h::opus_uint32,
    5984767 as i32 as crate::opus_types_h::opus_uint32,
    9173505 as i32 as crate::opus_types_h::opus_uint32,
    13726991 as i32 as crate::opus_types_h::opus_uint32,
    20103025 as i32 as crate::opus_types_h::opus_uint32,
    28875327 as i32 as crate::opus_types_h::opus_uint32,
    40754369 as i32 as crate::opus_types_h::opus_uint32,
    56610575 as i32 as crate::opus_types_h::opus_uint32,
    77500017 as i32 as crate::opus_types_h::opus_uint32,
    104692735 as i32 as crate::opus_types_h::opus_uint32,
    139703809 as i32 as crate::opus_types_h::opus_uint32,
    184327311 as i32 as crate::opus_types_h::opus_uint32,
    240673265 as i32 as crate::opus_types_h::opus_uint32,
    311207743 as i32 as crate::opus_types_h::opus_uint32,
    398796225 as i32 as crate::opus_types_h::opus_uint32,
    506750351 as i32 as crate::opus_types_h::opus_uint32,
    638878193 as i32 as crate::opus_types_h::opus_uint32,
    799538175 as i32 as crate::opus_types_h::opus_uint32,
    993696769 as i32 as crate::opus_types_h::opus_uint32,
    1226990095 as i32 as crate::opus_types_h::opus_uint32,
    1505789553 as i32 as crate::opus_types_h::opus_uint32,
    1837271615 as i32 as crate::opus_types_h::opus_uint32,
    2229491905 as u32,
    265729 as i32 as crate::opus_types_h::opus_uint32,
    598417 as i32 as crate::opus_types_h::opus_uint32,
    1256465 as i32 as crate::opus_types_h::opus_uint32,
    2485825 as i32 as crate::opus_types_h::opus_uint32,
    4673345 as i32 as crate::opus_types_h::opus_uint32,
    8405905 as i32 as crate::opus_types_h::opus_uint32,
    14546705 as i32 as crate::opus_types_h::opus_uint32,
    24331777 as i32 as crate::opus_types_h::opus_uint32,
    39490049 as i32 as crate::opus_types_h::opus_uint32,
    62390545 as i32 as crate::opus_types_h::opus_uint32,
    96220561 as i32 as crate::opus_types_h::opus_uint32,
    145198913 as i32 as crate::opus_types_h::opus_uint32,
    214828609 as i32 as crate::opus_types_h::opus_uint32,
    312193553 as i32 as crate::opus_types_h::opus_uint32,
    446304145 as i32 as crate::opus_types_h::opus_uint32,
    628496897 as i32 as crate::opus_types_h::opus_uint32,
    872893441 as i32 as crate::opus_types_h::opus_uint32,
    1196924561 as i32 as crate::opus_types_h::opus_uint32,
    1621925137 as i32 as crate::opus_types_h::opus_uint32,
    2173806145 as u32,
    1462563 as i32 as crate::opus_types_h::opus_uint32,
    3317445 as i32 as crate::opus_types_h::opus_uint32,
    7059735 as i32 as crate::opus_types_h::opus_uint32,
    14218905 as i32 as crate::opus_types_h::opus_uint32,
    27298155 as i32 as crate::opus_types_h::opus_uint32,
    50250765 as i32 as crate::opus_types_h::opus_uint32,
    89129247 as i32 as crate::opus_types_h::opus_uint32,
    152951073 as i32 as crate::opus_types_h::opus_uint32,
    254831667 as i32 as crate::opus_types_h::opus_uint32,
    413442773 as i32 as crate::opus_types_h::opus_uint32,
    654862247 as i32 as crate::opus_types_h::opus_uint32,
    1014889769 as i32 as crate::opus_types_h::opus_uint32,
    1541911931 as i32 as crate::opus_types_h::opus_uint32,
    2300409629 as u32,
    3375210671 as u32,
    8097453 as i32 as crate::opus_types_h::opus_uint32,
    18474633 as i32 as crate::opus_types_h::opus_uint32,
    39753273 as i32 as crate::opus_types_h::opus_uint32,
    81270333 as i32 as crate::opus_types_h::opus_uint32,
    158819253 as i32 as crate::opus_types_h::opus_uint32,
    298199265 as i32 as crate::opus_types_h::opus_uint32,
    540279585 as i32 as crate::opus_types_h::opus_uint32,
    948062325 as i32 as crate::opus_types_h::opus_uint32,
    1616336765 as i32 as crate::opus_types_h::opus_uint32,
    45046719 as i32 as crate::opus_types_h::opus_uint32,
    103274625 as i32 as crate::opus_types_h::opus_uint32,
    224298231 as i32 as crate::opus_types_h::opus_uint32,
    464387817 as i32 as crate::opus_types_h::opus_uint32,
    921406335 as i32 as crate::opus_types_h::opus_uint32,
    1759885185 as i32 as crate::opus_types_h::opus_uint32,
    3248227095 as u32,
    251595969 as i32 as crate::opus_types_h::opus_uint32,
    579168825 as i32 as crate::opus_types_h::opus_uint32,
    1267854873 as i32 as crate::opus_types_h::opus_uint32,
    2653649025 as u32,
    1409933619 as i32 as crate::opus_types_h::opus_uint32,
];
// Initialized in run_static_initializers

static mut CELT_PVQ_U_ROW: [*const crate::opus_types_h::opus_uint32; 15] =
    [0 as *const crate::opus_types_h::opus_uint32; 15];

unsafe extern "C" fn icwrs(mut _n: i32, mut _y: *const i32) -> crate::opus_types_h::opus_uint32 {
    let mut i: crate::opus_types_h::opus_uint32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    j = _n - 1 as i32;
    i = (*_y.offset(j as isize) < 0 as i32) as i32 as crate::opus_types_h::opus_uint32;
    k = ::libc::abs(*_y.offset(j as isize));
    loop {
        j -= 1;
        i =
            (i as u32).wrapping_add(
                *CELT_PVQ_U_ROW[if _n - j < k { (_n) - j } else { k } as usize]
                    .offset(if _n - j > k { (_n) - j } else { k } as isize),
            ) as crate::opus_types_h::opus_uint32;
        k += ::libc::abs(*_y.offset(j as isize));
        if *_y.offset(j as isize) < 0 as i32 {
            i = (i as u32).wrapping_add(
                *CELT_PVQ_U_ROW[if _n - j < k + 1 as i32 {
                    (_n) - j
                } else {
                    (k) + 1 as i32
                } as usize]
                    .offset(if _n - j > k + 1 as i32 {
                        (_n) - j
                    } else {
                        (k) + 1 as i32
                    } as isize),
            ) as crate::opus_types_h::opus_uint32
        }
        if !(j > 0 as i32) {
            break;
        }
    }
    return i;
}
#[no_mangle]

pub unsafe extern "C" fn encode_pulses(
    mut _y: *const i32,
    mut _n: i32,
    mut _k: i32,
    mut _enc: *mut crate::src::opus_1_2_1::celt::entcode::ec_enc,
) {
    crate::src::opus_1_2_1::celt::entenc::ec_enc_uint(
        _enc as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
        icwrs(_n, _y),
        (*CELT_PVQ_U_ROW[(if _n < _k { _n } else { _k }) as usize]
            .offset((if _n > _k { _n } else { _k }) as isize))
        .wrapping_add(
            *CELT_PVQ_U_ROW[(if _n < _k + 1 as i32 {
                _n
            } else {
                (_k) + 1 as i32
            }) as usize]
                .offset(
                    (if _n > _k + 1 as i32 {
                        _n
                    } else {
                        (_k) + 1 as i32
                    }) as isize,
                ),
        ),
    );
}

unsafe extern "C" fn cwrsi(
    mut _n: i32,
    mut _k: i32,
    mut _i: crate::opus_types_h::opus_uint32,
    mut _y: *mut i32,
) -> crate::arch_h::opus_val32 {
    let mut p: crate::opus_types_h::opus_uint32 = 0;
    let mut s: i32 = 0;
    let mut k0: i32 = 0;
    let mut val: crate::opus_types_h::opus_int16 = 0;
    let mut yy: crate::arch_h::opus_val32 = 0 as i32 as crate::arch_h::opus_val32;
    while _n > 2 as i32 {
        let mut q: crate::opus_types_h::opus_uint32 = 0;
        /*Lots of pulses case:*/
        if _k >= _n {
            let mut row: *const crate::opus_types_h::opus_uint32 =
                0 as *const crate::opus_types_h::opus_uint32;
            row = CELT_PVQ_U_ROW[_n as usize];
            /*Are the pulses in this dimension negative?*/
            p = *row.offset((_k + 1 as i32) as isize);
            s = -((_i >= p) as i32);
            _i = (_i as u32).wrapping_sub(p & s as u32) as crate::opus_types_h::opus_uint32;
            /*Count how many pulses were placed in this dimension.*/
            k0 = _k;
            q = *row.offset(_n as isize);
            if q > _i {
                _k = _n;
                loop {
                    _k -= 1;
                    p = *CELT_PVQ_U_ROW[_k as usize].offset(_n as isize);
                    if !(p > _i) {
                        break;
                    }
                }
            } else {
                p = *row.offset(_k as isize);
                while p > _i {
                    _k -= 1;
                    p = *row.offset(_k as isize)
                }
            }
            _i = (_i as u32).wrapping_sub(p) as crate::opus_types_h::opus_uint32;
            val = (k0 - _k + s ^ s) as crate::opus_types_h::opus_int16;
            let fresh0 = _y;
            _y = _y.offset(1);
            *fresh0 = val as i32;
            yy = yy + val as crate::arch_h::opus_val32 * val as crate::arch_h::opus_val32
        } else {
            /*Lots of dimensions case:*/
            /*Are there any pulses in this dimension at all?*/
            p = *CELT_PVQ_U_ROW[_k as usize].offset(_n as isize);
            q = *CELT_PVQ_U_ROW[(_k + 1 as i32) as usize].offset(_n as isize);
            if p <= _i && _i < q {
                _i = (_i as u32).wrapping_sub(p) as crate::opus_types_h::opus_uint32;
                let fresh1 = _y;
                _y = _y.offset(1);
                *fresh1 = 0 as i32
            } else {
                /*Are the pulses in this dimension negative?*/
                s = -((_i >= q) as i32);
                _i = (_i as u32).wrapping_sub(q & s as u32) as crate::opus_types_h::opus_uint32;
                /*Count how many pulses were placed in this dimension.*/
                k0 = _k;
                loop {
                    _k -= 1;
                    p = *CELT_PVQ_U_ROW[_k as usize].offset(_n as isize);
                    if !(p > _i) {
                        break;
                    }
                }
                _i = (_i as u32).wrapping_sub(p) as crate::opus_types_h::opus_uint32;
                val = (k0 - _k + s ^ s) as crate::opus_types_h::opus_int16;
                let fresh2 = _y;
                _y = _y.offset(1);
                *fresh2 = val as i32;
                yy = yy + val as crate::arch_h::opus_val32 * val as crate::arch_h::opus_val32
            }
        }
        _n -= 1
    }
    /*_n==2*/
    p = (2 as i32 * _k + 1 as i32) as crate::opus_types_h::opus_uint32;
    s = -((_i >= p) as i32);
    _i = (_i as u32).wrapping_sub(p & s as u32) as crate::opus_types_h::opus_uint32;
    k0 = _k;
    _k = (_i.wrapping_add(1 as i32 as u32) >> 1 as i32) as i32;
    if _k != 0 {
        _i = (_i as u32).wrapping_sub((2 as i32 * _k - 1 as i32) as u32)
            as crate::opus_types_h::opus_uint32
    }
    val = (k0 - _k + s ^ s) as crate::opus_types_h::opus_int16;
    let fresh3 = _y;
    _y = _y.offset(1);
    *fresh3 = val as i32;
    yy = yy + val as crate::arch_h::opus_val32 * val as crate::arch_h::opus_val32;
    /*_n==1*/
    s = -(_i as i32);
    val = (_k + s ^ s) as crate::opus_types_h::opus_int16;
    *_y = val as i32;
    yy = yy + val as crate::arch_h::opus_val32 * val as crate::arch_h::opus_val32;
    return yy;
}
/* Copyright (c) 2007-2008 CSIRO
Copyright (c) 2007-2009 Xiph.Org Foundation
Copyright (c) 2007-2009 Timothy B. Terriberry
Written by Timothy B. Terriberry and Jean-Marc Valin */
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
#[no_mangle]

pub unsafe extern "C" fn decode_pulses(
    mut _y: *mut i32,
    mut _n: i32,
    mut _k: i32,
    mut _dec: *mut crate::src::opus_1_2_1::celt::entcode::ec_dec,
) -> crate::arch_h::opus_val32 {
    return cwrsi(
        _n,
        _k,
        crate::src::opus_1_2_1::celt::entdec::ec_dec_uint(
            _dec as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
            (*CELT_PVQ_U_ROW[(if _n < _k { _n } else { _k }) as usize]
                .offset((if _n > _k { _n } else { _k }) as isize))
            .wrapping_add(
                *CELT_PVQ_U_ROW[(if _n < _k + 1 as i32 {
                    _n
                } else {
                    (_k) + 1 as i32
                }) as usize]
                    .offset(
                        (if _n > _k + 1 as i32 {
                            _n
                        } else {
                            (_k) + 1 as i32
                        }) as isize,
                    ),
            ),
        ),
        _y,
    );
}
unsafe extern "C" fn run_static_initializers() {
    CELT_PVQ_U_ROW = [
        CELT_PVQ_U_DATA.as_ptr().offset(0 as i32 as isize),
        CELT_PVQ_U_DATA.as_ptr().offset(176 as i32 as isize),
        CELT_PVQ_U_DATA.as_ptr().offset(351 as i32 as isize),
        CELT_PVQ_U_DATA.as_ptr().offset(525 as i32 as isize),
        CELT_PVQ_U_DATA.as_ptr().offset(698 as i32 as isize),
        CELT_PVQ_U_DATA.as_ptr().offset(870 as i32 as isize),
        CELT_PVQ_U_DATA.as_ptr().offset(1041 as i32 as isize),
        CELT_PVQ_U_DATA.as_ptr().offset(1131 as i32 as isize),
        CELT_PVQ_U_DATA.as_ptr().offset(1178 as i32 as isize),
        CELT_PVQ_U_DATA.as_ptr().offset(1207 as i32 as isize),
        CELT_PVQ_U_DATA.as_ptr().offset(1226 as i32 as isize),
        CELT_PVQ_U_DATA.as_ptr().offset(1240 as i32 as isize),
        CELT_PVQ_U_DATA.as_ptr().offset(1248 as i32 as isize),
        CELT_PVQ_U_DATA.as_ptr().offset(1254 as i32 as isize),
        CELT_PVQ_U_DATA.as_ptr().offset(1257 as i32 as isize),
    ]
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
/* SMALL_FOOTPRINT */
/* SMALL_FOOTPRINT */

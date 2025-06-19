use ::libc;
/* **********************************************************************
Copyright (c) 2006-2011, Skype Limited. All rights reserved.
Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions
are met:
- Redistributions of source code must retain the above copyright notice,
this list of conditions and the following disclaimer.
- Redistributions in binary form must reproduce the above copyright
notice, this list of conditions and the following disclaimer in the
documentation and/or other materials provided with the distribution.
- Neither the name of Internet Society, IETF or IETF Trust, nor the
names of specific contributors, may be used to endorse or promote
products derived from this software without specific prior written
permission.
THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE
LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
POSSIBILITY OF SUCH DAMAGE.
***********************************************************************/
/* *******************************************************************/
/*                    SIGNAL PROCESSING FUNCTIONS                   */
/* *******************************************************************/
/* Chirp (bw expand) LP AR filter */
/* I/O  AR filter to be expanded (without leading 1)                */
/* I    length of ar                                                */
/* I    chirp factor (typically in range (0..1) )                   */
/* compute inverse of LPC prediction gain, and                          */
/* test if LPC coefficients are stable (all poles within unit circle)   */
/* this code is based on silk_FLP_a2k()                                 */
/* O    return inverse prediction gain, energy domain               */
/* I    prediction coefficients [order]                             */
/* I    prediction order                                            */
/* O    returns residual energy                                     */
/* O    reflection coefficients (length order)                      */
/* I    autocorrelation sequence (length order+1)                   */
/* I    order                                                       */
/* O     prediction coefficients [order]                            */
/* I     reflection coefficients [order]                            */
/* I     prediction order                                           */
/* compute autocorrelation */
/* O    result (length correlationCount)                            */
/* I    input data to correlate                                     */
/* I    length of input                                             */
/* I    number of correlation taps to compute                       */
/* O    Voicing estimate: 0 voiced, 1 unvoiced                      */
/* I    Signal of length PE_FRAME_LENGTH_MS*Fs_kHz                  */
/* O    Pitch lag values [nb_subfr]                                 */
/* O    Lag Index                                                   */
/* O    Pitch contour Index                                         */
/* I/O  Normalized correlation; input: value from previous frame    */
/* I    Last lag of previous frame; set to zero is unvoiced         */
/* I    First stage threshold for lag candidates 0 - 1              */
/* I    Final threshold for lag candidates 0 - 1                    */
/* I    sample frequency (kHz)                                      */
/* I    Complexity setting, 0-2, where 2 is highest                 */
/* I    Number of 5 ms subframes                                    */
/* I    Run-time architecture                                       */
/* **********************************************************************
Copyright (c) 2006-2011, Skype Limited. All rights reserved.
Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions
are met:
- Redistributions of source code must retain the above copyright notice,
this list of conditions and the following disclaimer.
- Redistributions in binary form must reproduce the above copyright
notice, this list of conditions and the following disclaimer in the
documentation and/or other materials provided with the distribution.
- Neither the name of Internet Society, IETF or IETF Trust, nor the
names of specific contributors, may be used to endorse or promote
products derived from this software without specific prior written
permission.
THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE
LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
POSSIBILITY OF SUCH DAMAGE.
***********************************************************************/
/* Insertion sort (fast for already almost sorted arrays):  */
/* Best case:  O(n)   for an already sorted array           */
/* Worst case: O(n^2) for an inversely sorted array         */
#[no_mangle]

pub unsafe extern "C" fn silk_insertion_sort_decreasing_FLP(
    mut a: *mut f32,
    mut idx: *mut i32,
    L: i32,
    K: i32,
)
/* I    Number of correctly sorted positions                        */
{
    let mut value: f32 = 0.;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    /* Safety checks */
    /* Write start indices in index vector */
    i = 0 as i32;
    while i < K {
        *idx.offset(i as isize) = i;
        i += 1
    }
    /* Sort vector elements by value, decreasing order */
    i = 1 as i32;
    while i < K {
        value = *a.offset(i as isize);
        j = i - 1 as i32;
        while j >= 0 as i32 && value > *a.offset(j as isize) {
            /* Write index */
            *a.offset((j + 1 as i32) as isize) = *a.offset(j as isize);
            *idx.offset((j + 1 as i32) as isize) = *idx.offset(j as isize);
            j -= 1 /* Shift value */
            /* Shift index */
        } /* Write value */
        *a.offset((j + 1 as i32) as isize) = value;
        *idx.offset((j + 1 as i32) as isize) = i;
        i += 1
    }
    /* If less than L values are asked check the remaining values,      */
    /* but only spend CPU to ensure that the K first values are correct */
    i = K; /* Shift value */
    while i < L {
        value = *a.offset(i as isize);
        if value > *a.offset((K - 1 as i32) as isize) {
            j = K - 2 as i32;
            while j >= 0 as i32 && value > *a.offset(j as isize) {
                *a.offset((j + 1 as i32) as isize) = *a.offset(j as isize);
                *idx.offset((j + 1 as i32) as isize) = *idx.offset(j as isize);
                j -= 1
                /* Shift index */
            }
            /* Write index */
            *a.offset((j + 1 as i32) as isize) = value; /* Write value */
            *idx.offset((j + 1 as i32) as isize) = i
        }
        i += 1
    }
}

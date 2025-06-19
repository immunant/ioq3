pub use crate::opus_types_h::opus_int16;
pub use crate::opus_types_h::opus_int32;

pub use crate::stdlib::__int16_t;
pub use crate::stdlib::__int32_t;
pub use crate::stdlib::int16_t;
pub use crate::stdlib::int32_t;
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
/* **********************************************************
* Pitch analyser function
********************************************************** */
#[no_mangle]

pub unsafe extern "C" fn silk_decode_pitch(
    mut lagIndex: crate::opus_types_h::opus_int16,
    mut contourIndex: i8,
    mut pitch_lags: *mut i32,
    Fs_kHz: i32,
    nb_subfr: i32,
)
/* I    number of sub frames                                        */
{
    let mut lag: i32 = 0;
    let mut k: i32 = 0;
    let mut min_lag: i32 = 0;
    let mut max_lag: i32 = 0;
    let mut cbk_size: i32 = 0;
    let mut Lag_CB_ptr: *const i8 = 0 as *const i8;
    if Fs_kHz == 8 as i32 {
        if nb_subfr == 4 as i32 {
            Lag_CB_ptr = &*(*crate::src::opus_1_2_1::silk::pitch_est_tables::silk_CB_lags_stage2
                .as_ptr()
                .offset(0 as i32 as isize))
            .as_ptr()
            .offset(0 as i32 as isize) as *const i8;
            cbk_size = 11 as i32
        } else {
            Lag_CB_ptr =
                &*(*crate::src::opus_1_2_1::silk::pitch_est_tables::silk_CB_lags_stage2_10_ms
                    .as_ptr()
                    .offset(0 as i32 as isize))
                .as_ptr()
                .offset(0 as i32 as isize) as *const i8;
            cbk_size = 3 as i32
        }
    } else if nb_subfr == 4 as i32 {
        Lag_CB_ptr = &*(*crate::src::opus_1_2_1::silk::pitch_est_tables::silk_CB_lags_stage3
            .as_ptr()
            .offset(0 as i32 as isize))
        .as_ptr()
        .offset(0 as i32 as isize) as *const i8;
        cbk_size = 34 as i32
    } else {
        Lag_CB_ptr = &*(*crate::src::opus_1_2_1::silk::pitch_est_tables::silk_CB_lags_stage3_10_ms
            .as_ptr()
            .offset(0 as i32 as isize))
        .as_ptr()
        .offset(0 as i32 as isize) as *const i8;
        cbk_size = 12 as i32
    }
    min_lag = 2 as i32 as crate::opus_types_h::opus_int16 as crate::opus_types_h::opus_int32
        * Fs_kHz as crate::opus_types_h::opus_int16 as crate::opus_types_h::opus_int32;
    max_lag = 18 as i32 as crate::opus_types_h::opus_int16 as crate::opus_types_h::opus_int32
        * Fs_kHz as crate::opus_types_h::opus_int16 as crate::opus_types_h::opus_int32;
    lag = min_lag + lagIndex as i32;
    k = 0 as i32;
    while k < nb_subfr {
        *pitch_lags.offset(k as isize) =
            lag + *Lag_CB_ptr.offset((k * cbk_size + contourIndex as i32) as isize) as i32;
        *pitch_lags.offset(k as isize) = if min_lag > max_lag {
            if *pitch_lags.offset(k as isize) > min_lag {
                min_lag
            } else if *pitch_lags.offset(k as isize) < max_lag {
                max_lag
            } else {
                *pitch_lags.offset(k as isize)
            }
        } else if *pitch_lags.offset(k as isize) > max_lag {
            max_lag
        } else if *pitch_lags.offset(k as isize) < min_lag {
            min_lag
        } else {
            *pitch_lags.offset(k as isize)
        };
        k += 1
    }
}

// =============== BEGIN envelope_h ================
#[repr(C)]
#[derive(Copy, Clone)]
pub struct envelope_filter_state {
    pub ampbuf: [f32; 17],
    pub ampptr: i32,
    pub nearDC: [f32; 15],
    pub nearDC_acc: f32,
    pub nearDC_partialacc: f32,
    pub nearptr: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct envelope_band {
    pub begin: i32,
    pub end: i32,
    pub window: *mut f32,
    pub total: f32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct envelope_lookup {
    pub ch: i32,
    pub winlength: i32,
    pub searchstep: i32,
    pub minenergy: f32,
    pub mdct: crate::src::libvorbis_1_3_6::lib::mdct::mdct_lookup,
    pub mdct_win: *mut f32,
    pub band: [crate::src::libvorbis_1_3_6::lib::envelope::envelope_band; 7],
    pub filter: *mut crate::src::libvorbis_1_3_6::lib::envelope::envelope_filter_state,
    pub stretch: i32,
    pub mark: *mut i32,
    pub storage: libc::c_long,
    pub current: libc::c_long,
    pub curmark: libc::c_long,
    pub cursor: libc::c_long,
}
use ::libc;

pub mod scales_h {

    /* Segher was off (too high) by ~ .3 decibel.  Center the conversion correctly. */
    #[inline]

    pub unsafe extern "C" fn todB(mut x: *const f32) -> f32 {
        let mut ix: crate::scales_h::C2RustUnnamed_58 = crate::scales_h::C2RustUnnamed_58 { i: 0 };
        ix.f = *x;
        ix.i = ix.i & 0x7fffffff as i32 as u32;
        return ix.i as f32 * 7.17711438e-7f32 - 764.6161886f32;
    }

    /* Frequency to octave.  We arbitrarily declare 63.5 Hz to be octave
    0.0 */
    /* The bark scale equations are approximations, since the original
    table was somewhat hand rolled.  The below are chosen to have the
    best possible fit to the rolled tables, thus their somewhat odd
    appearance (these are more accurate and over a longer range than
    the oft-quoted bark equations found in the texts I have).  The
    approximations are valid from 0 - 30kHz (nyquist) or so.

    all f in Hz, z in Bark */
}

pub use crate::config_types_h::ogg_int64_t;
pub use crate::config_types_h::ogg_uint32_t;
pub use crate::ogg_h::oggpack_buffer;
pub use crate::stdlib::__int64_t;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::int64_t;
pub use crate::stdlib::uint32_t;

pub use crate::codec_h::alloc_chain;
pub use crate::codec_h::vorbis_block;
pub use crate::codec_h::vorbis_dsp_state;
pub use crate::codec_h::vorbis_info;
pub use crate::codec_internal_h::codec_setup_info;
pub use crate::codec_internal_h::private_state;
pub use crate::codec_internal_h::vorbis_info_floor;
pub use crate::codec_internal_h::vorbis_info_mapping;
pub use crate::codec_internal_h::vorbis_info_mode;
pub use crate::codec_internal_h::vorbis_info_residue;
pub use crate::codec_internal_h::vorbis_look_floor;
pub use crate::codec_internal_h::vorbis_look_residue;
pub use crate::codec_internal_h::vorbis_look_transform;
pub use crate::highlevel_h::highlevel_byblocktype;
pub use crate::highlevel_h::highlevel_encode_setup;
pub use crate::src::libvorbis_1_3_6::lib::bitrate::bitrate_manager_info;
pub use crate::src::libvorbis_1_3_6::lib::bitrate::bitrate_manager_state;
pub use crate::src::libvorbis_1_3_6::lib::codebook::codebook;
pub use crate::src::libvorbis_1_3_6::lib::codebook::static_codebook;
pub use crate::src::libvorbis_1_3_6::lib::mdct::mdct_clear;
pub use crate::src::libvorbis_1_3_6::lib::mdct::mdct_forward;
pub use crate::src::libvorbis_1_3_6::lib::mdct::mdct_init;
pub use crate::src::libvorbis_1_3_6::lib::mdct::mdct_lookup;
pub use crate::src::libvorbis_1_3_6::lib::psy::vorbis_info_psy;
pub use crate::src::libvorbis_1_3_6::lib::psy::vorbis_info_psy_global;
pub use crate::src::libvorbis_1_3_6::lib::psy::vorbis_look_psy;
pub use crate::src::libvorbis_1_3_6::lib::psy::vorbis_look_psy_global;
pub use crate::src::libvorbis_1_3_6::lib::smallft::drft_lookup;

pub use crate::scales_h::C2RustUnnamed_58;
pub use crate::src::libvorbis_1_3_6::lib::envelope::scales_h::todB;

/* *******************************************************************
*                                                                  *
* THIS FILE IS PART OF THE OggVorbis SOFTWARE CODEC SOURCE CODE.   *
* USE, DISTRIBUTION AND REPRODUCTION OF THIS LIBRARY SOURCE IS     *
* GOVERNED BY A BSD-STYLE SOURCE LICENSE INCLUDED WITH THIS SOURCE *
* IN 'COPYING'. PLEASE READ THESE TERMS BEFORE DISTRIBUTING.       *
*                                                                  *
* THE OggVorbis SOURCE CODE IS (C) COPYRIGHT 1994-2009             *
* by the Xiph.Org Foundation http://www.xiph.org/                  *
*                                                                  *
********************************************************************

function: PCM data envelope analysis

********************************************************************/
#[no_mangle]

pub unsafe extern "C" fn _ve_envelope_init(
    mut e: *mut crate::src::libvorbis_1_3_6::lib::envelope::envelope_lookup,
    mut vi: *mut crate::codec_h::vorbis_info,
) {
    let mut ci: *mut crate::codec_internal_h::codec_setup_info =
        (*vi).codec_setup as *mut crate::codec_internal_h::codec_setup_info; /* not random */
    let mut gi: *mut crate::src::libvorbis_1_3_6::lib::psy::vorbis_info_psy_global =
        &mut (*ci).psy_g_param;
    let mut ch: i32 = (*vi).channels;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    (*e).winlength = 128 as i32;
    let mut n: i32 = (*e).winlength;
    (*e).searchstep = 64 as i32;
    (*e).minenergy = (*gi).preecho_minenergy;
    (*e).ch = ch;
    (*e).storage = 128 as i32 as libc::c_long;
    (*e).cursor = (*ci).blocksizes[1 as i32 as usize] / 2 as i32 as libc::c_long;
    (*e).mdct_win = crate::stdlib::calloc(
        n as libc::c_ulong,
        ::std::mem::size_of::<f32>() as libc::c_ulong,
    ) as *mut f32;
    crate::src::libvorbis_1_3_6::lib::mdct::mdct_init(
        &mut (*e).mdct as *mut _ as *mut crate::src::libvorbis_1_3_6::lib::mdct::mdct_lookup,
        n,
    );
    i = 0 as i32;
    while i < n {
        *(*e).mdct_win.offset(i as isize) =
            crate::stdlib::sin(i as f64 / (n as f64 - 1.0f64) * 3.14159265358979323846f64) as f32;
        *(*e).mdct_win.offset(i as isize) *= *(*e).mdct_win.offset(i as isize);
        i += 1
    }
    /* magic follows */
    (*e).band[0 as i32 as usize].begin = 2 as i32;
    (*e).band[0 as i32 as usize].end = 4 as i32;
    (*e).band[1 as i32 as usize].begin = 4 as i32;
    (*e).band[1 as i32 as usize].end = 5 as i32;
    (*e).band[2 as i32 as usize].begin = 6 as i32;
    (*e).band[2 as i32 as usize].end = 6 as i32;
    (*e).band[3 as i32 as usize].begin = 9 as i32;
    (*e).band[3 as i32 as usize].end = 8 as i32;
    (*e).band[4 as i32 as usize].begin = 13 as i32;
    (*e).band[4 as i32 as usize].end = 8 as i32;
    (*e).band[5 as i32 as usize].begin = 17 as i32;
    (*e).band[5 as i32 as usize].end = 8 as i32;
    (*e).band[6 as i32 as usize].begin = 22 as i32;
    (*e).band[6 as i32 as usize].end = 8 as i32;
    j = 0 as i32;
    while j < 7 as i32 {
        n = (*e).band[j as usize].end;
        (*e).band[j as usize].window = crate::stdlib::malloc(
            (n as libc::c_ulong).wrapping_mul(::std::mem::size_of::<f32>() as libc::c_ulong),
        ) as *mut f32;
        i = 0 as i32;
        while i < n {
            *(*e).band[j as usize].window.offset(i as isize) =
                crate::stdlib::sin((i as f64 + 0.5f64) / n as f64 * 3.14159265358979323846f64)
                    as f32;
            (*e).band[j as usize].total += *(*e).band[j as usize].window.offset(i as isize);
            i += 1
        }
        (*e).band[j as usize].total = (1.0f64 / (*e).band[j as usize].total as f64) as f32;
        j += 1
    }
    (*e).filter = crate::stdlib::calloc(
        (7 as i32 * ch) as libc::c_ulong,
        ::std::mem::size_of::<crate::src::libvorbis_1_3_6::lib::envelope::envelope_filter_state>()
            as libc::c_ulong,
    ) as *mut crate::src::libvorbis_1_3_6::lib::envelope::envelope_filter_state;
    (*e).mark = crate::stdlib::calloc(
        (*e).storage as libc::c_ulong,
        ::std::mem::size_of::<i32>() as libc::c_ulong,
    ) as *mut i32;
}
#[no_mangle]

pub unsafe extern "C" fn _ve_envelope_clear(
    mut e: *mut crate::src::libvorbis_1_3_6::lib::envelope::envelope_lookup,
) {
    let mut i: i32 = 0;
    crate::src::libvorbis_1_3_6::lib::mdct::mdct_clear(
        &mut (*e).mdct as *mut _ as *mut crate::src::libvorbis_1_3_6::lib::mdct::mdct_lookup,
    );
    i = 0 as i32;
    while i < 7 as i32 {
        ::libc::free((*e).band[i as usize].window as *mut libc::c_void);
        i += 1
    }
    ::libc::free((*e).mdct_win as *mut libc::c_void);
    ::libc::free((*e).filter as *mut libc::c_void);
    ::libc::free((*e).mark as *mut libc::c_void);
    crate::stdlib::memset(
        e as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<crate::src::libvorbis_1_3_6::lib::envelope::envelope_lookup>()
            as libc::c_ulong,
    );
}
/* fairly straight threshhold-by-band based until we find something
that works better and isn't patented. */

unsafe extern "C" fn _ve_amp(
    mut ve: *mut crate::src::libvorbis_1_3_6::lib::envelope::envelope_lookup,
    mut gi: *mut crate::src::libvorbis_1_3_6::lib::psy::vorbis_info_psy_global,
    mut data: *mut f32,
    mut bands: *mut crate::src::libvorbis_1_3_6::lib::envelope::envelope_band,
    mut filters: *mut crate::src::libvorbis_1_3_6::lib::envelope::envelope_filter_state,
) -> i32 {
    let mut n: libc::c_long = (*ve).winlength as libc::c_long;
    let mut ret: i32 = 0 as i32;
    let mut i: libc::c_long = 0;
    let mut j: libc::c_long = 0;
    let mut decay: f32 = 0.;
    /* we want to have a 'minimum bar' for energy, else we're just
    basing blocks on quantization noise that outweighs the signal
    itself (for low power signals) */
    let mut minV: f32 = (*ve).minenergy;
    let mut fresh0 = ::std::vec::from_elem(
        0,
        (n as libc::c_ulong).wrapping_mul(::std::mem::size_of::<f32>() as libc::c_ulong) as usize,
    );
    let mut vec: *mut f32 = fresh0.as_mut_ptr() as *mut f32;
    /* stretch is used to gradually lengthen the number of windows
    considered prevoius-to-potential-trigger */
    let mut stretch: i32 = if (2 as i32) < (*ve).stretch / 2 as i32 {
        ((*ve).stretch) / 2 as i32
    } else {
        2 as i32
    };
    let mut penalty: f32 = (*gi).stretch_penalty - ((*ve).stretch / 2 as i32 - 2 as i32) as f32;
    if penalty < 0.0f32 {
        penalty = 0.0f32
    }
    if penalty > (*gi).stretch_penalty {
        penalty = (*gi).stretch_penalty
    }
    /*_analysis_output_always("lpcm",seq2,data,n,0,0,
    totalshift+pos*ve->searchstep);*/
    /* window and transform */
    i = 0 as i32 as libc::c_long;
    while i < n {
        *vec.offset(i as isize) = *data.offset(i as isize) * *(*ve).mdct_win.offset(i as isize);
        i += 1
    }
    crate::src::libvorbis_1_3_6::lib::mdct::mdct_forward(
        &mut (*ve).mdct as *mut _ as *mut crate::src::libvorbis_1_3_6::lib::mdct::mdct_lookup,
        vec,
        vec,
    );
    /*_analysis_output_always("mdct",seq2,vec,n/2,0,1,0); */
    /* near-DC spreading function; this has nothing to do with
    psychoacoustics, just sidelobe leakage and window size */
    let mut temp: f32 = ((*vec.offset(0 as i32 as isize) * *vec.offset(0 as i32 as isize)) as f64
        + 0.7f64 * *vec.offset(1 as i32 as isize) as f64 * *vec.offset(1 as i32 as isize) as f64
        + 0.2f64 * *vec.offset(2 as i32 as isize) as f64 * *vec.offset(2 as i32 as isize) as f64)
        as f32;
    let mut ptr: i32 = (*filters).nearptr;
    /* the accumulation is regularly refreshed from scratch to avoid
    floating point creep */
    if ptr == 0 as i32 {
        (*filters).nearDC_acc = (*filters).nearDC_partialacc + temp;
        decay = (*filters).nearDC_acc;
        (*filters).nearDC_partialacc = temp
    } else {
        (*filters).nearDC_acc += temp;
        decay = (*filters).nearDC_acc;
        (*filters).nearDC_partialacc += temp
    }
    (*filters).nearDC_acc -= (*filters).nearDC[ptr as usize];
    (*filters).nearDC[ptr as usize] = temp;
    decay = (decay as f64 * (1.0f64 / (15 as i32 + 1 as i32) as f64)) as f32;
    (*filters).nearptr += 1;
    if (*filters).nearptr >= 15 as i32 {
        (*filters).nearptr = 0 as i32
    }
    decay = (todB(&mut decay) as f64 * 0.5f64 - 15.0f32 as f64) as f32;
    /* perform spreading and limiting, also smooth the spectrum.  yes,
    the MDCT results in all real coefficients, but it still *behaves*
    like real/imaginary pairs */
    i = 0 as i32 as libc::c_long;
    while i < n / 2 as i32 as libc::c_long {
        let mut val: f32 = *vec.offset(i as isize) * *vec.offset(i as isize)
            + *vec.offset((i + 1 as i32 as libc::c_long) as isize)
                * *vec.offset((i + 1 as i32 as libc::c_long) as isize);
        val = todB(&mut val) * 0.5f32;
        if val < decay {
            val = decay
        }
        if val < minV {
            val = minV
        }
        *vec.offset((i >> 1 as i32) as isize) = val;
        decay = (decay as f64 - 8.0f64) as f32;
        i += 2 as i32 as libc::c_long
    }
    /*_analysis_output_always("spread",seq2++,vec,n/4,0,0,0);*/
    /* perform preecho/postecho triggering by band */
    j = 0 as i32 as libc::c_long;
    while j < 7 as i32 as libc::c_long {
        let mut acc: f32 = 0.0f64 as f32;
        let mut valmax: f32 = 0.;
        let mut valmin: f32 = 0.;
        /* accumulate amplitude */
        i = 0 as i32 as libc::c_long;
        while i < (*bands.offset(j as isize)).end as libc::c_long {
            acc += *vec.offset((i + (*bands.offset(j as isize)).begin as libc::c_long) as isize)
                * *(*bands.offset(j as isize)).window.offset(i as isize);
            i += 1
        }
        acc *= (*bands.offset(j as isize)).total;
        /* convert amplitude to delta */
        let mut p: i32 = 0;
        let mut this: i32 = (*filters.offset(j as isize)).ampptr;
        let mut postmax: f32 = 0.;
        let mut postmin: f32 = 0.;
        let mut premax: f32 = -99999.0f32;
        let mut premin: f32 = 99999.0f32;
        p = this;
        p -= 1;
        if p < 0 as i32 {
            p += 16 as i32 + 2 as i32 - 1 as i32
        }
        postmax = if acc < (*filters.offset(j as isize)).ampbuf[p as usize] {
            (*filters.offset(j as isize)).ampbuf[p as usize]
        } else {
            acc
        };
        postmin = if acc > (*filters.offset(j as isize)).ampbuf[p as usize] {
            (*filters.offset(j as isize)).ampbuf[p as usize]
        } else {
            acc
        };
        i = 0 as i32 as libc::c_long;
        while i < stretch as libc::c_long {
            p -= 1;
            if p < 0 as i32 {
                p += 16 as i32 + 2 as i32 - 1 as i32
            }
            premax = if premax < (*filters.offset(j as isize)).ampbuf[p as usize] {
                (*filters.offset(j as isize)).ampbuf[p as usize]
            } else {
                premax
            };
            premin = if premin > (*filters.offset(j as isize)).ampbuf[p as usize] {
                (*filters.offset(j as isize)).ampbuf[p as usize]
            } else {
                premin
            };
            i += 1
        }
        valmin = postmin - premin;
        valmax = postmax - premax;
        /*filters[j].markers[pos]=valmax;*/
        (*filters.offset(j as isize)).ampbuf[this as usize] = acc;
        let ref mut fresh1 = (*filters.offset(j as isize)).ampptr;
        *fresh1 += 1;
        if (*filters.offset(j as isize)).ampptr >= 16 as i32 + 2 as i32 - 1 as i32 {
            (*filters.offset(j as isize)).ampptr = 0 as i32
        }
        /* look at min/max, decide trigger */
        if valmax > (*gi).preecho_thresh[j as usize] + penalty {
            ret |= 1 as i32;
            ret |= 4 as i32
        }
        if valmin < (*gi).postecho_thresh[j as usize] - penalty {
            ret |= 2 as i32
        }
        j += 1
    }
    return ret;
}
#[no_mangle]

pub unsafe extern "C" fn _ve_envelope_search(
    mut v: *mut crate::codec_h::vorbis_dsp_state,
) -> libc::c_long {
    let mut vi: *mut crate::codec_h::vorbis_info = (*v).vi;
    let mut ci: *mut crate::codec_internal_h::codec_setup_info =
        (*vi).codec_setup as *mut crate::codec_internal_h::codec_setup_info;
    let mut gi: *mut crate::src::libvorbis_1_3_6::lib::psy::vorbis_info_psy_global =
        &mut (*ci).psy_g_param;
    let mut ve: *mut crate::src::libvorbis_1_3_6::lib::envelope::envelope_lookup =
        (*((*v).backend_state as *mut crate::codec_internal_h::private_state)).ve;
    let mut i: libc::c_long = 0;
    let mut j: libc::c_long = 0;
    let mut first: i32 = ((*ve).current / (*ve).searchstep as libc::c_long) as i32;
    let mut last: i32 = (*v).pcm_current / (*ve).searchstep - 4 as i32;
    if first < 0 as i32 {
        first = 0 as i32
    }
    /* make sure we have enough storage to match the PCM */
    if (last + 4 as i32 + 2 as i32) as libc::c_long > (*ve).storage {
        (*ve).storage = (last + 4 as i32 + 2 as i32) as libc::c_long; /* be sure */
        (*ve).mark = crate::stdlib::realloc(
            (*ve).mark as *mut libc::c_void,
            ((*ve).storage as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<i32>() as libc::c_ulong),
        ) as *mut i32
    }
    j = first as libc::c_long;
    while j < last as libc::c_long {
        let mut ret: i32 = 0 as i32;
        (*ve).stretch += 1;
        if (*ve).stretch > 12 as i32 * 2 as i32 {
            (*ve).stretch = 12 as i32 * 2 as i32
        }
        i = 0 as i32 as libc::c_long;
        while i < (*ve).ch as libc::c_long {
            let mut pcm: *mut f32 = (*(*v).pcm.offset(i as isize))
                .offset(((*ve).searchstep as libc::c_long * j) as isize);
            ret |= _ve_amp(
                ve,
                gi,
                pcm,
                (*ve).band.as_mut_ptr(),
                (*ve).filter.offset((i * 7 as i32 as libc::c_long) as isize),
            );
            i += 1
        }
        *(*ve).mark.offset((j + 2 as i32 as libc::c_long) as isize) = 0 as i32;
        if ret & 1 as i32 != 0 {
            *(*ve).mark.offset(j as isize) = 1 as i32;
            *(*ve).mark.offset((j + 1 as i32 as libc::c_long) as isize) = 1 as i32
        }
        if ret & 2 as i32 != 0 {
            *(*ve).mark.offset(j as isize) = 1 as i32;
            if j > 0 as i32 as libc::c_long {
                *(*ve).mark.offset((j - 1 as i32 as libc::c_long) as isize) = 1 as i32
            }
        }
        if ret & 4 as i32 != 0 {
            (*ve).stretch = -(1 as i32)
        }
        j += 1
    }
    (*ve).current = (last * (*ve).searchstep) as libc::c_long;
    let mut centerW: libc::c_long = (*v).centerW;
    let mut testW: libc::c_long = centerW
        + (*ci).blocksizes[(*v).W as usize] / 4 as i32 as libc::c_long
        + (*ci).blocksizes[1 as i32 as usize] / 2 as i32 as libc::c_long
        + (*ci).blocksizes[0 as i32 as usize] / 4 as i32 as libc::c_long;
    j = (*ve).cursor;
    while j < (*ve).current - (*ve).searchstep as libc::c_long {
        /* account for postecho
        working back one window */
        if j >= testW {
            return 1 as i32 as libc::c_long;
        }
        (*ve).cursor = j;
        if *(*ve)
            .mark
            .offset((j / (*ve).searchstep as libc::c_long) as isize)
            != 0
        {
            if j > centerW {
                (*ve).curmark = j;
                if j >= testW {
                    return 1 as i32 as libc::c_long;
                }
                return 0 as i32 as libc::c_long;
            }
        }
        j += (*ve).searchstep as libc::c_long
    }
    return -(1 as i32) as libc::c_long;
}
#[no_mangle]

pub unsafe extern "C" fn _ve_envelope_mark(mut v: *mut crate::codec_h::vorbis_dsp_state) -> i32 {
    let mut ve: *mut crate::src::libvorbis_1_3_6::lib::envelope::envelope_lookup =
        (*((*v).backend_state as *mut crate::codec_internal_h::private_state)).ve;
    let mut vi: *mut crate::codec_h::vorbis_info = (*v).vi;
    let mut ci: *mut crate::codec_internal_h::codec_setup_info =
        (*vi).codec_setup as *mut crate::codec_internal_h::codec_setup_info;
    let mut centerW: libc::c_long = (*v).centerW;
    let mut beginW: libc::c_long =
        centerW - (*ci).blocksizes[(*v).W as usize] / 4 as i32 as libc::c_long;
    let mut endW: libc::c_long =
        centerW + (*ci).blocksizes[(*v).W as usize] / 4 as i32 as libc::c_long;
    if (*v).W != 0 {
        beginW -= (*ci).blocksizes[(*v).lW as usize] / 4 as i32 as libc::c_long;
        endW += (*ci).blocksizes[(*v).nW as usize] / 4 as i32 as libc::c_long
    } else {
        beginW -= (*ci).blocksizes[0 as i32 as usize] / 4 as i32 as libc::c_long;
        endW += (*ci).blocksizes[0 as i32 as usize] / 4 as i32 as libc::c_long
    }
    if (*ve).curmark >= beginW && (*ve).curmark < endW {
        return 1 as i32;
    }
    let mut first: libc::c_long = beginW / (*ve).searchstep as libc::c_long;
    let mut last: libc::c_long = endW / (*ve).searchstep as libc::c_long;
    let mut i: libc::c_long = 0;
    i = first;
    while i < last {
        if *(*ve).mark.offset(i as isize) != 0 {
            return 1 as i32;
        }
        i += 1
    }
    return 0 as i32;
}
/* *******************************************************************
*                                                                  *
* THIS FILE IS PART OF THE OggVorbis SOFTWARE CODEC SOURCE CODE.   *
* USE, DISTRIBUTION AND REPRODUCTION OF THIS LIBRARY SOURCE IS     *
* GOVERNED BY A BSD-STYLE SOURCE LICENSE INCLUDED WITH THIS SOURCE *
* IN 'COPYING'. PLEASE READ THESE TERMS BEFORE DISTRIBUTING.       *
*                                                                  *
* THE OggVorbis SOURCE CODE IS (C) COPYRIGHT 1994-2009             *
* by the Xiph.Org Foundation http://www.xiph.org/                  *
*                                                                  *
********************************************************************

function: PCM data envelope analysis and manipulation

********************************************************************/
/* a bit less than short block */
/* one-third full block */
#[no_mangle]

pub unsafe extern "C" fn _ve_envelope_shift(
    mut e: *mut crate::src::libvorbis_1_3_6::lib::envelope::envelope_lookup,
    mut shift: libc::c_long,
) {
    let mut smallsize: i32 =
        ((*e).current / (*e).searchstep as libc::c_long + 2 as i32 as libc::c_long) as i32; /* adjust for placing marks
                                                                                            ahead of ve->current */
    let mut smallshift: i32 = (shift / (*e).searchstep as libc::c_long) as i32;
    crate::stdlib::memmove(
        (*e).mark as *mut libc::c_void,
        (*e).mark.offset(smallshift as isize) as *const libc::c_void,
        ((smallsize - smallshift) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<i32>() as libc::c_ulong),
    );
    (*e).current -= shift;
    if (*e).curmark >= 0 as i32 as libc::c_long {
        (*e).curmark -= shift
    }
    (*e).cursor -= shift;
}

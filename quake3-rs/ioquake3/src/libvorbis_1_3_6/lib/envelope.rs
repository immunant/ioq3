// =============== BEGIN envelope_h ================
#[repr(C)]
#[derive(Copy, Clone)]
pub struct envelope_filter_state {
    pub ampbuf: [libc::c_float; 17],
    pub ampptr: libc::c_int,
    pub nearDC: [libc::c_float; 15],
    pub nearDC_acc: libc::c_float,
    pub nearDC_partialacc: libc::c_float,
    pub nearptr: libc::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct envelope_band {
    pub begin: libc::c_int,
    pub end: libc::c_int,
    pub window: *mut libc::c_float,
    pub total: libc::c_float,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct envelope_lookup {
    pub ch: libc::c_int,
    pub winlength: libc::c_int,
    pub searchstep: libc::c_int,
    pub minenergy: libc::c_float,
    pub mdct: crate::src::libvorbis_1_3_6::lib::mdct::mdct_lookup,
    pub mdct_win: *mut libc::c_float,
    pub band: [crate::src::libvorbis_1_3_6::lib::envelope::envelope_band; 7],
    pub filter: *mut crate::src::libvorbis_1_3_6::lib::envelope::envelope_filter_state,
    pub stretch: libc::c_int,
    pub mark: *mut libc::c_int,
    pub storage: libc::c_long,
    pub current: libc::c_long,
    pub curmark: libc::c_long,
    pub cursor: libc::c_long,
}
use ::libc;

pub mod scales_h {

    /* Segher was off (too high) by ~ .3 decibel.  Center the conversion correctly. */
    #[inline]

    pub unsafe extern "C" fn todB(mut x: *const libc::c_float) -> libc::c_float {
        let mut ix: crate::scales_h::C2RustUnnamed_58 = crate::scales_h::C2RustUnnamed_58 { i: 0 };
        ix.f = *x;
        ix.i = ix.i & 0x7fffffff as libc::c_int as libc::c_uint;
        return ix.i as libc::c_float * 7.17711438e-7f32 - 764.6161886f32;
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
    let mut ch: libc::c_int = (*vi).channels;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    (*e).winlength = 128 as libc::c_int;
    let mut n: libc::c_int = (*e).winlength;
    (*e).searchstep = 64 as libc::c_int;
    (*e).minenergy = (*gi).preecho_minenergy;
    (*e).ch = ch;
    (*e).storage = 128 as libc::c_int as libc::c_long;
    (*e).cursor = (*ci).blocksizes[1 as libc::c_int as usize] / 2 as libc::c_int as libc::c_long;
    (*e).mdct_win = crate::stdlib::calloc(
        n as libc::c_ulong,
        ::std::mem::size_of::<libc::c_float>() as libc::c_ulong,
    ) as *mut libc::c_float;
    crate::src::libvorbis_1_3_6::lib::mdct::mdct_init(
        &mut (*e).mdct as *mut _ as *mut crate::src::libvorbis_1_3_6::lib::mdct::mdct_lookup,
        n,
    );
    i = 0 as libc::c_int;
    while i < n {
        *(*e).mdct_win.offset(i as isize) = crate::stdlib::sin(
            i as libc::c_double / (n as libc::c_double - 1.0f64) * 3.14159265358979323846f64,
        ) as libc::c_float;
        *(*e).mdct_win.offset(i as isize) *= *(*e).mdct_win.offset(i as isize);
        i += 1
    }
    /* magic follows */
    (*e).band[0 as libc::c_int as usize].begin = 2 as libc::c_int;
    (*e).band[0 as libc::c_int as usize].end = 4 as libc::c_int;
    (*e).band[1 as libc::c_int as usize].begin = 4 as libc::c_int;
    (*e).band[1 as libc::c_int as usize].end = 5 as libc::c_int;
    (*e).band[2 as libc::c_int as usize].begin = 6 as libc::c_int;
    (*e).band[2 as libc::c_int as usize].end = 6 as libc::c_int;
    (*e).band[3 as libc::c_int as usize].begin = 9 as libc::c_int;
    (*e).band[3 as libc::c_int as usize].end = 8 as libc::c_int;
    (*e).band[4 as libc::c_int as usize].begin = 13 as libc::c_int;
    (*e).band[4 as libc::c_int as usize].end = 8 as libc::c_int;
    (*e).band[5 as libc::c_int as usize].begin = 17 as libc::c_int;
    (*e).band[5 as libc::c_int as usize].end = 8 as libc::c_int;
    (*e).band[6 as libc::c_int as usize].begin = 22 as libc::c_int;
    (*e).band[6 as libc::c_int as usize].end = 8 as libc::c_int;
    j = 0 as libc::c_int;
    while j < 7 as libc::c_int {
        n = (*e).band[j as usize].end;
        (*e).band[j as usize].window = crate::stdlib::malloc(
            (n as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_float>() as libc::c_ulong),
        ) as *mut libc::c_float;
        i = 0 as libc::c_int;
        while i < n {
            *(*e).band[j as usize].window.offset(i as isize) = crate::stdlib::sin(
                (i as libc::c_double + 0.5f64) / n as libc::c_double * 3.14159265358979323846f64,
            ) as libc::c_float;
            (*e).band[j as usize].total += *(*e).band[j as usize].window.offset(i as isize);
            i += 1
        }
        (*e).band[j as usize].total =
            (1.0f64 / (*e).band[j as usize].total as libc::c_double) as libc::c_float;
        j += 1
    }
    (*e).filter = crate::stdlib::calloc(
        (7 as libc::c_int * ch) as libc::c_ulong,
        ::std::mem::size_of::<crate::src::libvorbis_1_3_6::lib::envelope::envelope_filter_state>()
            as libc::c_ulong,
    ) as *mut crate::src::libvorbis_1_3_6::lib::envelope::envelope_filter_state;
    (*e).mark = crate::stdlib::calloc(
        (*e).storage as libc::c_ulong,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn _ve_envelope_clear(
    mut e: *mut crate::src::libvorbis_1_3_6::lib::envelope::envelope_lookup,
) {
    let mut i: libc::c_int = 0;
    crate::src::libvorbis_1_3_6::lib::mdct::mdct_clear(
        &mut (*e).mdct as *mut _ as *mut crate::src::libvorbis_1_3_6::lib::mdct::mdct_lookup,
    );
    i = 0 as libc::c_int;
    while i < 7 as libc::c_int {
        ::libc::free((*e).band[i as usize].window as *mut libc::c_void);
        i += 1
    }
    ::libc::free((*e).mdct_win as *mut libc::c_void);
    ::libc::free((*e).filter as *mut libc::c_void);
    ::libc::free((*e).mark as *mut libc::c_void);
    crate::stdlib::memset(
        e as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<crate::src::libvorbis_1_3_6::lib::envelope::envelope_lookup>()
            as libc::c_ulong,
    );
}
/* fairly straight threshhold-by-band based until we find something
that works better and isn't patented. */

unsafe extern "C" fn _ve_amp(
    mut ve: *mut crate::src::libvorbis_1_3_6::lib::envelope::envelope_lookup,
    mut gi: *mut crate::src::libvorbis_1_3_6::lib::psy::vorbis_info_psy_global,
    mut data: *mut libc::c_float,
    mut bands: *mut crate::src::libvorbis_1_3_6::lib::envelope::envelope_band,
    mut filters: *mut crate::src::libvorbis_1_3_6::lib::envelope::envelope_filter_state,
) -> libc::c_int {
    let mut n: libc::c_long = (*ve).winlength as libc::c_long;
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_long = 0;
    let mut j: libc::c_long = 0;
    let mut decay: libc::c_float = 0.;
    /* we want to have a 'minimum bar' for energy, else we're just
    basing blocks on quantization noise that outweighs the signal
    itself (for low power signals) */
    let mut minV: libc::c_float = (*ve).minenergy;
    let mut fresh0 = ::std::vec::from_elem(
        0,
        (n as libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
            as usize,
    );
    let mut vec: *mut libc::c_float = fresh0.as_mut_ptr() as *mut libc::c_float;
    /* stretch is used to gradually lengthen the number of windows
    considered prevoius-to-potential-trigger */
    let mut stretch: libc::c_int = if (2 as libc::c_int) < (*ve).stretch / 2 as libc::c_int {
        ((*ve).stretch) / 2 as libc::c_int
    } else {
        2 as libc::c_int
    };
    let mut penalty: libc::c_float = (*gi).stretch_penalty
        - ((*ve).stretch / 2 as libc::c_int - 2 as libc::c_int) as libc::c_float;
    if penalty < 0.0f32 {
        penalty = 0.0f32
    }
    if penalty > (*gi).stretch_penalty {
        penalty = (*gi).stretch_penalty
    }
    /*_analysis_output_always("lpcm",seq2,data,n,0,0,
    totalshift+pos*ve->searchstep);*/
    /* window and transform */
    i = 0 as libc::c_int as libc::c_long;
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
    let mut temp: libc::c_float = ((*vec.offset(0 as libc::c_int as isize)
        * *vec.offset(0 as libc::c_int as isize))
        as libc::c_double
        + 0.7f64
            * *vec.offset(1 as libc::c_int as isize) as libc::c_double
            * *vec.offset(1 as libc::c_int as isize) as libc::c_double
        + 0.2f64
            * *vec.offset(2 as libc::c_int as isize) as libc::c_double
            * *vec.offset(2 as libc::c_int as isize) as libc::c_double)
        as libc::c_float;
    let mut ptr: libc::c_int = (*filters).nearptr;
    /* the accumulation is regularly refreshed from scratch to avoid
    floating point creep */
    if ptr == 0 as libc::c_int {
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
    decay = (decay as libc::c_double
        * (1.0f64 / (15 as libc::c_int + 1 as libc::c_int) as libc::c_double))
        as libc::c_float;
    (*filters).nearptr += 1;
    if (*filters).nearptr >= 15 as libc::c_int {
        (*filters).nearptr = 0 as libc::c_int
    }
    decay =
        (todB(&mut decay) as libc::c_double * 0.5f64 - 15.0f32 as libc::c_double) as libc::c_float;
    /* perform spreading and limiting, also smooth the spectrum.  yes,
    the MDCT results in all real coefficients, but it still *behaves*
    like real/imaginary pairs */
    i = 0 as libc::c_int as libc::c_long;
    while i < n / 2 as libc::c_int as libc::c_long {
        let mut val: libc::c_float = *vec.offset(i as isize) * *vec.offset(i as isize)
            + *vec.offset((i + 1 as libc::c_int as libc::c_long) as isize)
                * *vec.offset((i + 1 as libc::c_int as libc::c_long) as isize);
        val = todB(&mut val) * 0.5f32;
        if val < decay {
            val = decay
        }
        if val < minV {
            val = minV
        }
        *vec.offset((i >> 1 as libc::c_int) as isize) = val;
        decay = (decay as libc::c_double - 8.0f64) as libc::c_float;
        i += 2 as libc::c_int as libc::c_long
    }
    /*_analysis_output_always("spread",seq2++,vec,n/4,0,0,0);*/
    /* perform preecho/postecho triggering by band */
    j = 0 as libc::c_int as libc::c_long;
    while j < 7 as libc::c_int as libc::c_long {
        let mut acc: libc::c_float = 0.0f64 as libc::c_float;
        let mut valmax: libc::c_float = 0.;
        let mut valmin: libc::c_float = 0.;
        /* accumulate amplitude */
        i = 0 as libc::c_int as libc::c_long;
        while i < (*bands.offset(j as isize)).end as libc::c_long {
            acc += *vec.offset((i + (*bands.offset(j as isize)).begin as libc::c_long) as isize)
                * *(*bands.offset(j as isize)).window.offset(i as isize);
            i += 1
        }
        acc *= (*bands.offset(j as isize)).total;
        /* convert amplitude to delta */
        let mut p: libc::c_int = 0;
        let mut this: libc::c_int = (*filters.offset(j as isize)).ampptr;
        let mut postmax: libc::c_float = 0.;
        let mut postmin: libc::c_float = 0.;
        let mut premax: libc::c_float = -99999.0f32;
        let mut premin: libc::c_float = 99999.0f32;
        p = this;
        p -= 1;
        if p < 0 as libc::c_int {
            p += 16 as libc::c_int + 2 as libc::c_int - 1 as libc::c_int
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
        i = 0 as libc::c_int as libc::c_long;
        while i < stretch as libc::c_long {
            p -= 1;
            if p < 0 as libc::c_int {
                p += 16 as libc::c_int + 2 as libc::c_int - 1 as libc::c_int
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
        if (*filters.offset(j as isize)).ampptr
            >= 16 as libc::c_int + 2 as libc::c_int - 1 as libc::c_int
        {
            (*filters.offset(j as isize)).ampptr = 0 as libc::c_int
        }
        /* look at min/max, decide trigger */
        if valmax > (*gi).preecho_thresh[j as usize] + penalty {
            ret |= 1 as libc::c_int;
            ret |= 4 as libc::c_int
        }
        if valmin < (*gi).postecho_thresh[j as usize] - penalty {
            ret |= 2 as libc::c_int
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
    let mut first: libc::c_int = ((*ve).current / (*ve).searchstep as libc::c_long) as libc::c_int;
    let mut last: libc::c_int = (*v).pcm_current / (*ve).searchstep - 4 as libc::c_int;
    if first < 0 as libc::c_int {
        first = 0 as libc::c_int
    }
    /* make sure we have enough storage to match the PCM */
    if (last + 4 as libc::c_int + 2 as libc::c_int) as libc::c_long > (*ve).storage {
        (*ve).storage = (last + 4 as libc::c_int + 2 as libc::c_int) as libc::c_long; /* be sure */
        (*ve).mark = crate::stdlib::realloc(
            (*ve).mark as *mut libc::c_void,
            ((*ve).storage as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) as *mut libc::c_int
    }
    j = first as libc::c_long;
    while j < last as libc::c_long {
        let mut ret: libc::c_int = 0 as libc::c_int;
        (*ve).stretch += 1;
        if (*ve).stretch > 12 as libc::c_int * 2 as libc::c_int {
            (*ve).stretch = 12 as libc::c_int * 2 as libc::c_int
        }
        i = 0 as libc::c_int as libc::c_long;
        while i < (*ve).ch as libc::c_long {
            let mut pcm: *mut libc::c_float = (*(*v).pcm.offset(i as isize))
                .offset(((*ve).searchstep as libc::c_long * j) as isize);
            ret |= _ve_amp(
                ve,
                gi,
                pcm,
                (*ve).band.as_mut_ptr(),
                (*ve)
                    .filter
                    .offset((i * 7 as libc::c_int as libc::c_long) as isize),
            );
            i += 1
        }
        *(*ve)
            .mark
            .offset((j + 2 as libc::c_int as libc::c_long) as isize) = 0 as libc::c_int;
        if ret & 1 as libc::c_int != 0 {
            *(*ve).mark.offset(j as isize) = 1 as libc::c_int;
            *(*ve)
                .mark
                .offset((j + 1 as libc::c_int as libc::c_long) as isize) = 1 as libc::c_int
        }
        if ret & 2 as libc::c_int != 0 {
            *(*ve).mark.offset(j as isize) = 1 as libc::c_int;
            if j > 0 as libc::c_int as libc::c_long {
                *(*ve)
                    .mark
                    .offset((j - 1 as libc::c_int as libc::c_long) as isize) = 1 as libc::c_int
            }
        }
        if ret & 4 as libc::c_int != 0 {
            (*ve).stretch = -(1 as libc::c_int)
        }
        j += 1
    }
    (*ve).current = (last * (*ve).searchstep) as libc::c_long;
    let mut centerW: libc::c_long = (*v).centerW;
    let mut testW: libc::c_long = centerW
        + (*ci).blocksizes[(*v).W as usize] / 4 as libc::c_int as libc::c_long
        + (*ci).blocksizes[1 as libc::c_int as usize] / 2 as libc::c_int as libc::c_long
        + (*ci).blocksizes[0 as libc::c_int as usize] / 4 as libc::c_int as libc::c_long;
    j = (*ve).cursor;
    while j < (*ve).current - (*ve).searchstep as libc::c_long {
        /* account for postecho
        working back one window */
        if j >= testW {
            return 1 as libc::c_int as libc::c_long;
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
                    return 1 as libc::c_int as libc::c_long;
                }
                return 0 as libc::c_int as libc::c_long;
            }
        }
        j += (*ve).searchstep as libc::c_long
    }
    return -(1 as libc::c_int) as libc::c_long;
}
#[no_mangle]

pub unsafe extern "C" fn _ve_envelope_mark(
    mut v: *mut crate::codec_h::vorbis_dsp_state,
) -> libc::c_int {
    let mut ve: *mut crate::src::libvorbis_1_3_6::lib::envelope::envelope_lookup =
        (*((*v).backend_state as *mut crate::codec_internal_h::private_state)).ve;
    let mut vi: *mut crate::codec_h::vorbis_info = (*v).vi;
    let mut ci: *mut crate::codec_internal_h::codec_setup_info =
        (*vi).codec_setup as *mut crate::codec_internal_h::codec_setup_info;
    let mut centerW: libc::c_long = (*v).centerW;
    let mut beginW: libc::c_long =
        centerW - (*ci).blocksizes[(*v).W as usize] / 4 as libc::c_int as libc::c_long;
    let mut endW: libc::c_long =
        centerW + (*ci).blocksizes[(*v).W as usize] / 4 as libc::c_int as libc::c_long;
    if (*v).W != 0 {
        beginW -= (*ci).blocksizes[(*v).lW as usize] / 4 as libc::c_int as libc::c_long;
        endW += (*ci).blocksizes[(*v).nW as usize] / 4 as libc::c_int as libc::c_long
    } else {
        beginW -= (*ci).blocksizes[0 as libc::c_int as usize] / 4 as libc::c_int as libc::c_long;
        endW += (*ci).blocksizes[0 as libc::c_int as usize] / 4 as libc::c_int as libc::c_long
    }
    if (*ve).curmark >= beginW && (*ve).curmark < endW {
        return 1 as libc::c_int;
    }
    let mut first: libc::c_long = beginW / (*ve).searchstep as libc::c_long;
    let mut last: libc::c_long = endW / (*ve).searchstep as libc::c_long;
    let mut i: libc::c_long = 0;
    i = first;
    while i < last {
        if *(*ve).mark.offset(i as isize) != 0 {
            return 1 as libc::c_int;
        }
        i += 1
    }
    return 0 as libc::c_int;
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
    let mut smallsize: libc::c_int = ((*e).current / (*e).searchstep as libc::c_long
        + 2 as libc::c_int as libc::c_long) as libc::c_int; /* adjust for placing marks
                                                            ahead of ve->current */
    let mut smallshift: libc::c_int = (shift / (*e).searchstep as libc::c_long) as libc::c_int;
    crate::stdlib::memmove(
        (*e).mark as *mut libc::c_void,
        (*e).mark.offset(smallshift as isize) as *const libc::c_void,
        ((smallsize - smallshift) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    (*e).current -= shift;
    if (*e).curmark >= 0 as libc::c_int as libc::c_long {
        (*e).curmark -= shift
    }
    (*e).cursor -= shift;
}

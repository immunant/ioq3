use ::libc;

pub use crate::config_types_h::ogg_uint32_t;
pub use crate::stddef_h::size_t;
pub use crate::stdlib::__compar_fn_t;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::calloc;
pub use crate::stdlib::malloc;
pub use crate::stdlib::qsort;
pub use crate::stdlib::uint32_t;
pub use ::libc::free;

pub use crate::src::libvorbis_1_3_6::lib::codebook::codebook;
pub use crate::src::libvorbis_1_3_6::lib::codebook::static_codebook;

/* *******************************************************************
*                                                                  *
* THIS FILE IS PART OF THE OggVorbis SOFTWARE CODEC SOURCE CODE.   *
* USE, DISTRIBUTION AND REPRODUCTION OF THIS LIBRARY SOURCE IS     *
* GOVERNED BY A BSD-STYLE SOURCE LICENSE INCLUDED WITH THIS SOURCE *
* IN 'COPYING'. PLEASE READ THESE TERMS BEFORE DISTRIBUTING.       *
*                                                                  *
* THE OggVorbis SOURCE CODE IS (C) COPYRIGHT 1994-2015             *
* by the Xiph.Org Foundation http://www.xiph.org/                  *
*                                                                  *
********************************************************************

function: miscellaneous prototypes

********************************************************************/
/* *******************************************************************
*                                                                  *
* THIS FILE IS PART OF THE OggVorbis SOFTWARE CODEC SOURCE CODE.   *
* USE, DISTRIBUTION AND REPRODUCTION OF THIS LIBRARY SOURCE IS     *
* GOVERNED BY A BSD-STYLE SOURCE LICENSE INCLUDED WITH THIS SOURCE *
* IN 'COPYING'. PLEASE READ THESE TERMS BEFORE DISTRIBUTING.       *
*                                                                  *
* THE OggVorbis SOURCE CODE IS (C) COPYRIGHT 1994-2015             *
* by the Xiph.Org Foundation http://www.xiph.org/                  *
*                                                                  *
********************************************************************

function: basic shared codebook operations

********************************************************************/
/* *** pack/unpack helpers ******************************************/
#[no_mangle]

pub unsafe extern "C" fn ov_ilog(mut v: crate::config_types_h::ogg_uint32_t) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    ret = 0 as libc::c_int;
    while v != 0 {
        v >>= 1 as libc::c_int;
        ret += 1
    }
    return ret;
}
/* bias toward values smaller than 1. */
/* doesn't currently guard under/overflow */
#[no_mangle]

pub unsafe extern "C" fn _float32_pack(mut val: libc::c_float) -> libc::c_long {
    let mut sign: libc::c_int = 0 as libc::c_int; //+epsilon
    let mut exp: libc::c_long = 0;
    let mut mant: libc::c_long = 0;
    if val < 0 as libc::c_int as libc::c_float {
        sign = 0x80000000 as libc::c_uint as libc::c_int;
        val = -val
    }
    exp = crate::stdlib::floor(
        crate::stdlib::log(val as libc::c_double) / crate::stdlib::log(2.0f32 as libc::c_double)
            + 0.001f64,
    ) as libc::c_long;
    mant = crate::stdlib::rint(crate::stdlib::ldexp(
        val as libc::c_double,
        ((21 as libc::c_int - 1 as libc::c_int) as libc::c_long - exp) as libc::c_int,
    )) as libc::c_long;
    exp = (exp + 768 as libc::c_int as libc::c_long) << 21 as libc::c_int;
    return sign as libc::c_long | exp | mant;
}
#[no_mangle]

pub unsafe extern "C" fn _float32_unpack(mut val: libc::c_long) -> libc::c_float {
    let mut mant: libc::c_double =
        (val & 0x1fffff as libc::c_int as libc::c_long) as libc::c_double;
    let mut sign: libc::c_int = (val & 0x80000000 as libc::c_uint as libc::c_long) as libc::c_int;
    let mut exp: libc::c_long = (val & 0x7fe00000 as libc::c_long) >> 21 as libc::c_int;
    if sign != 0 {
        mant = -mant
    }
    return crate::stdlib::ldexp(
        mant,
        (exp - (21 as libc::c_int - 1 as libc::c_int) as libc::c_long
            - 768 as libc::c_int as libc::c_long) as libc::c_int,
    ) as libc::c_float;
}
/* given a list of word lengths, generate a list of codewords.  Works
for length ordered or unordered, always assigns the lowest valued
codewords first.  Extended to handle unused entries (length 0) */
#[no_mangle]

pub unsafe extern "C" fn _make_words(
    mut l: *mut libc::c_char,
    mut n: libc::c_long,
    mut sparsecount: libc::c_long,
) -> *mut crate::config_types_h::ogg_uint32_t {
    let mut i: libc::c_long = 0;
    let mut j: libc::c_long = 0;
    let mut count: libc::c_long = 0 as libc::c_int as libc::c_long;
    let mut marker: [crate::config_types_h::ogg_uint32_t; 33] = [0; 33];
    let mut r: *mut crate::config_types_h::ogg_uint32_t = crate::stdlib::malloc(
        ((if sparsecount != 0 { sparsecount } else { n }) as libc::c_ulong).wrapping_mul(
            ::std::mem::size_of::<crate::config_types_h::ogg_uint32_t>() as libc::c_ulong,
        ),
    )
        as *mut crate::config_types_h::ogg_uint32_t;
    crate::stdlib::memset(
        marker.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[crate::config_types_h::ogg_uint32_t; 33]>() as libc::c_ulong,
    );
    i = 0 as libc::c_int as libc::c_long;
    while i < n {
        let mut length: libc::c_long = *l.offset(i as isize) as libc::c_long;
        if length > 0 as libc::c_int as libc::c_long {
            let mut entry: crate::config_types_h::ogg_uint32_t = marker[length as usize];
            /* when we claim a node for an entry, we also claim the nodes
            below it (pruning off the imagined tree that may have dangled
            from it) as well as blocking the use of any nodes directly
            above for leaves */
            /* update ourself */
            if length < 32 as libc::c_int as libc::c_long && entry >> length != 0 {
                /* error condition; the lengths must specify an overpopulated tree */
                ::libc::free(r as *mut libc::c_void);
                return 0 as *mut crate::config_types_h::ogg_uint32_t;
            }
            let fresh0 = count;
            count = count + 1;
            *r.offset(fresh0 as isize) = entry;
            /* Look to see if the next shorter marker points to the node
            above. if so, update it and repeat.  */
            j = length;
            while j > 0 as libc::c_int as libc::c_long {
                if marker[j as usize] & 1 as libc::c_int as libc::c_uint != 0 {
                    /* have to jump branches */
                    if j == 1 as libc::c_int as libc::c_long {
                        marker[1 as libc::c_int as usize] =
                            marker[1 as libc::c_int as usize].wrapping_add(1)
                    } else {
                        marker[j as usize] = marker[(j - 1 as libc::c_int as libc::c_long) as usize]
                            << 1 as libc::c_int
                    }
                    break;
                /* invariant says next upper marker would already
                have been moved if it was on the same path */
                } else {
                    marker[j as usize] = marker[j as usize].wrapping_add(1);
                    j -= 1
                }
            }
            /* prune the tree; the implicit invariant says all the longer
            markers were dangling from our just-taken node.  Dangle them
            from our *new* node. */
            j = length + 1 as libc::c_int as libc::c_long;
            while j < 33 as libc::c_int as libc::c_long {
                if !(marker[j as usize] >> 1 as libc::c_int == entry) {
                    break;
                }
                entry = marker[j as usize];
                marker[j as usize] =
                    marker[(j - 1 as libc::c_int as libc::c_long) as usize] << 1 as libc::c_int;
                j += 1
            }
        } else if sparsecount == 0 as libc::c_int as libc::c_long {
            count += 1
        }
        i += 1
    }
    /* any underpopulated tree must be rejected. */
    /* Single-entry codebooks are a retconned extension to the spec.
    They have a single codeword '0' of length 1 that results in an
    underpopulated tree.  Shield that case from the underformed tree check. */
    if !(count == 1 as libc::c_int as libc::c_long
        && marker[2 as libc::c_int as usize] == 2 as libc::c_int as libc::c_uint)
    {
        i = 1 as libc::c_int as libc::c_long;
        while i < 33 as libc::c_int as libc::c_long {
            if marker[i as usize] as libc::c_ulong
                & 0xffffffff as libc::c_ulong >> 32 as libc::c_int as libc::c_long - i
                != 0
            {
                ::libc::free(r as *mut libc::c_void);
                return 0 as *mut crate::config_types_h::ogg_uint32_t;
            }
            i += 1
        }
    }
    /* bitreverse the words because our bitwise packer/unpacker is LSb
    endian */
    i = 0 as libc::c_int as libc::c_long;
    count = 0 as libc::c_int as libc::c_long;
    while i < n {
        let mut temp: crate::config_types_h::ogg_uint32_t =
            0 as libc::c_int as crate::config_types_h::ogg_uint32_t;
        j = 0 as libc::c_int as libc::c_long;
        while j < *l.offset(i as isize) as libc::c_long {
            temp <<= 1 as libc::c_int;
            temp |= *r.offset(count as isize) >> j & 1 as libc::c_int as libc::c_uint;
            j += 1
        }
        if sparsecount != 0 {
            if *l.offset(i as isize) != 0 {
                let fresh1 = count;
                count = count + 1;
                *r.offset(fresh1 as isize) = temp
            }
        } else {
            let fresh2 = count;
            count = count + 1;
            *r.offset(fresh2 as isize) = temp
        }
        i += 1
    }
    return r;
}
/* there might be a straightforward one-line way to do the below
that's portable and totally safe against roundoff, but I haven't
thought of it.  Therefore, we opt on the side of caution */
#[no_mangle]

pub unsafe extern "C" fn _book_maptype1_quantvals(
    mut b: *const crate::src::libvorbis_1_3_6::lib::codebook::static_codebook,
) -> libc::c_long {
    let mut vals: libc::c_long = 0;
    if (*b).entries < 1 as libc::c_int as libc::c_long {
        return 0 as libc::c_int as libc::c_long;
    }
    vals = crate::stdlib::floor(crate::stdlib::pow(
        (*b).entries as libc::c_float as libc::c_double,
        (1.0f32 / (*b).dim as libc::c_float) as libc::c_double,
    )) as libc::c_long;
    /* the above *should* be reliable, but we'll not assume that FP is
    ever reliable when bitstream sync is at stake; verify via integer
    means that vals really is the greatest value of dim for which
    vals^b->bim <= b->entries */
    /* treat the above as an initial guess */
    if vals < 1 as libc::c_int as libc::c_long {
        vals = 1 as libc::c_int as libc::c_long
    }
    loop {
        let mut acc: libc::c_long = 1 as libc::c_int as libc::c_long;
        let mut acc1: libc::c_long = 1 as libc::c_int as libc::c_long;
        let mut i: libc::c_int = 0;
        i = 0 as libc::c_int;
        while (i as libc::c_long) < (*b).dim {
            if (*b).entries / vals < acc {
                break;
            }
            acc *= vals;
            if (9223372036854775807 as libc::c_long / (vals + 1 as libc::c_int as libc::c_long))
                < acc1
            {
                acc1 = 9223372036854775807 as libc::c_long
            } else {
                acc1 *= vals + 1 as libc::c_int as libc::c_long
            }
            i += 1
        }
        if i as libc::c_long >= (*b).dim && acc <= (*b).entries && acc1 > (*b).entries {
            return vals;
        } else {
            if (i as libc::c_long) < (*b).dim || acc > (*b).entries {
                vals -= 1
            } else {
                vals += 1
            }
        }
    }
}
/* unpack the quantized list of values for encode/decode ***********/
/* we need to deal with two map types: in map type 1, the values are
generated algorithmically (each column of the vector counts through
the values in the quant vector). in map type 2, all the values came
in in an explicit list.  Both value lists must be unpacked */
#[no_mangle]

pub unsafe extern "C" fn _book_unquantize(
    mut b: *const crate::src::libvorbis_1_3_6::lib::codebook::static_codebook,
    mut n: libc::c_int,
    mut sparsemap: *mut libc::c_int,
) -> *mut libc::c_float {
    let mut j: libc::c_long = 0;
    let mut k: libc::c_long = 0;
    let mut count: libc::c_long = 0 as libc::c_int as libc::c_long;
    if (*b).maptype == 1 as libc::c_int || (*b).maptype == 2 as libc::c_int {
        let mut quantvals: libc::c_int = 0;
        let mut mindel: libc::c_float = _float32_unpack((*b).q_min);
        let mut delta: libc::c_float = _float32_unpack((*b).q_delta);
        let mut r: *mut libc::c_float = crate::stdlib::calloc(
            (n as libc::c_long * (*b).dim) as libc::c_ulong,
            ::std::mem::size_of::<libc::c_float>() as libc::c_ulong,
        ) as *mut libc::c_float;
        /* maptype 1 and 2 both use a quantized value vector, but
        different sizes */
        match (*b).maptype {
            1 => {
                /* most of the time, entries%dimensions == 0, but we need to be
                well defined.  We define that the possible vales at each
                scalar is values == entries/dim.  If entries%dim != 0, we'll
                have 'too few' values (values*dim<entries), which means that
                we'll have 'left over' entries; left over entries use zeroed
                values (and are wasted).  So don't generate codebooks like
                that */
                quantvals = _book_maptype1_quantvals(b) as libc::c_int;
                j = 0 as libc::c_int as libc::c_long;
                while j < (*b).entries {
                    if !sparsemap.is_null()
                        && *(*b).lengthlist.offset(j as isize) as libc::c_int != 0
                        || sparsemap.is_null()
                    {
                        let mut last: libc::c_float = 0.0f32;
                        let mut indexdiv: libc::c_int = 1 as libc::c_int;
                        k = 0 as libc::c_int as libc::c_long;
                        while k < (*b).dim {
                            let mut index: libc::c_int = (j / indexdiv as libc::c_long
                                % quantvals as libc::c_long)
                                as libc::c_int;
                            let mut val: libc::c_float =
                                *(*b).quantlist.offset(index as isize) as libc::c_float;
                            val = (crate::stdlib::fabs(val as libc::c_double)
                                * delta as libc::c_double
                                + mindel as libc::c_double
                                + last as libc::c_double)
                                as libc::c_float;
                            if (*b).q_sequencep != 0 {
                                last = val
                            }
                            if !sparsemap.is_null() {
                                *r.offset(
                                    (*sparsemap.offset(count as isize) as libc::c_long * (*b).dim
                                        + k) as isize,
                                ) = val
                            } else {
                                *r.offset((count * (*b).dim + k) as isize) = val
                            }
                            indexdiv *= quantvals;
                            k += 1
                        }
                        count += 1
                    }
                    j += 1
                }
            }
            2 => {
                j = 0 as libc::c_int as libc::c_long;
                while j < (*b).entries {
                    if !sparsemap.is_null()
                        && *(*b).lengthlist.offset(j as isize) as libc::c_int != 0
                        || sparsemap.is_null()
                    {
                        let mut last_0: libc::c_float = 0.0f32;
                        k = 0 as libc::c_int as libc::c_long;
                        while k < (*b).dim {
                            let mut val_0: libc::c_float =
                                *(*b).quantlist.offset((j * (*b).dim + k) as isize)
                                    as libc::c_float;
                            val_0 = (crate::stdlib::fabs(val_0 as libc::c_double)
                                * delta as libc::c_double
                                + mindel as libc::c_double
                                + last_0 as libc::c_double)
                                as libc::c_float;
                            if (*b).q_sequencep != 0 {
                                last_0 = val_0
                            }
                            if !sparsemap.is_null() {
                                *r.offset(
                                    (*sparsemap.offset(count as isize) as libc::c_long * (*b).dim
                                        + k) as isize,
                                ) = val_0
                            } else {
                                *r.offset((count * (*b).dim + k) as isize) = val_0
                            }
                            k += 1
                        }
                        count += 1
                    }
                    j += 1
                }
            }
            _ => {}
        }
        return r;
    }
    return 0 as *mut libc::c_float;
}
#[no_mangle]

pub unsafe extern "C" fn vorbis_staticbook_destroy(
    mut b: *mut crate::src::libvorbis_1_3_6::lib::codebook::static_codebook,
) {
    if (*b).allocedp != 0 {
        if !(*b).quantlist.is_null() {
            ::libc::free((*b).quantlist as *mut libc::c_void);
        }
        if !(*b).lengthlist.is_null() {
            ::libc::free((*b).lengthlist as *mut libc::c_void);
        }
        crate::stdlib::memset(
            b as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<crate::src::libvorbis_1_3_6::lib::codebook::static_codebook>()
                as libc::c_ulong,
        );
        ::libc::free(b as *mut libc::c_void);
    };
    /* otherwise, it is in static memory */
}
#[no_mangle]

pub unsafe extern "C" fn vorbis_book_clear(
    mut b: *mut crate::src::libvorbis_1_3_6::lib::codebook::codebook,
) {
    /* static book is not cleared; we're likely called on the lookup and
    the static codebook belongs to the info struct */
    if !(*b).valuelist.is_null() {
        ::libc::free((*b).valuelist as *mut libc::c_void);
    }
    if !(*b).codelist.is_null() {
        ::libc::free((*b).codelist as *mut libc::c_void);
    }
    if !(*b).dec_index.is_null() {
        ::libc::free((*b).dec_index as *mut libc::c_void);
    }
    if !(*b).dec_codelengths.is_null() {
        ::libc::free((*b).dec_codelengths as *mut libc::c_void);
    }
    if !(*b).dec_firsttable.is_null() {
        ::libc::free((*b).dec_firsttable as *mut libc::c_void);
    }
    crate::stdlib::memset(
        b as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<crate::src::libvorbis_1_3_6::lib::codebook::codebook>()
            as libc::c_ulong,
    );
}
#[no_mangle]

pub unsafe extern "C" fn vorbis_book_init_encode(
    mut c: *mut crate::src::libvorbis_1_3_6::lib::codebook::codebook,
    mut s: *const crate::src::libvorbis_1_3_6::lib::codebook::static_codebook,
) -> libc::c_int {
    crate::stdlib::memset(
        c as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<crate::src::libvorbis_1_3_6::lib::codebook::codebook>()
            as libc::c_ulong,
    );
    (*c).c = s;
    (*c).entries = (*s).entries;
    (*c).used_entries = (*s).entries;
    (*c).dim = (*s).dim;
    (*c).codelist = _make_words(
        (*s).lengthlist,
        (*s).entries,
        0 as libc::c_int as libc::c_long,
    );
    //c->valuelist=_book_unquantize(s,s->entries,NULL);
    (*c).quantvals = _book_maptype1_quantvals(s) as libc::c_int;
    (*c).minval = crate::stdlib::rint(_float32_unpack((*s).q_min) as libc::c_double) as libc::c_int;
    (*c).delta =
        crate::stdlib::rint(_float32_unpack((*s).q_delta) as libc::c_double) as libc::c_int;
    return 0 as libc::c_int;
}

unsafe extern "C" fn bitreverse(
    mut x: crate::config_types_h::ogg_uint32_t,
) -> crate::config_types_h::ogg_uint32_t {
    x = ((x >> 16 as libc::c_int) as libc::c_ulong & 0xffff as libc::c_ulong
        | (x << 16 as libc::c_int) as libc::c_ulong & 0xffff0000 as libc::c_ulong)
        as crate::config_types_h::ogg_uint32_t;
    x = ((x >> 8 as libc::c_int) as libc::c_ulong & 0xff00ff as libc::c_ulong
        | (x << 8 as libc::c_int) as libc::c_ulong & 0xff00ff00 as libc::c_ulong)
        as crate::config_types_h::ogg_uint32_t;
    x = ((x >> 4 as libc::c_int) as libc::c_ulong & 0xf0f0f0f as libc::c_ulong
        | (x << 4 as libc::c_int) as libc::c_ulong & 0xf0f0f0f0 as libc::c_ulong)
        as crate::config_types_h::ogg_uint32_t;
    x = ((x >> 2 as libc::c_int) as libc::c_ulong & 0x33333333 as libc::c_ulong
        | (x << 2 as libc::c_int) as libc::c_ulong & 0xcccccccc as libc::c_ulong)
        as crate::config_types_h::ogg_uint32_t;
    return ((x >> 1 as libc::c_int) as libc::c_ulong & 0x55555555 as libc::c_ulong
        | (x << 1 as libc::c_int) as libc::c_ulong & 0xaaaaaaaa as libc::c_ulong)
        as crate::config_types_h::ogg_uint32_t;
}

unsafe extern "C" fn sort32a(
    mut a: *const libc::c_void,
    mut b: *const libc::c_void,
) -> libc::c_int {
    return (**(a as *mut *mut crate::config_types_h::ogg_uint32_t)
        > **(b as *mut *mut crate::config_types_h::ogg_uint32_t)) as libc::c_int
        - ((**(a as *mut *mut crate::config_types_h::ogg_uint32_t))
            < **(b as *mut *mut crate::config_types_h::ogg_uint32_t)) as libc::c_int;
}
/* decode codebook arrangement is more heavily optimized than encode */
#[no_mangle]

pub unsafe extern "C" fn vorbis_book_init_decode(
    mut c: *mut crate::src::libvorbis_1_3_6::lib::codebook::codebook,
    mut s: *const crate::src::libvorbis_1_3_6::lib::codebook::static_codebook,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut n: libc::c_int = 0 as libc::c_int;
    let mut tabn: libc::c_int = 0;
    let mut sortindex: *mut libc::c_int = 0 as *mut libc::c_int;
    crate::stdlib::memset(
        c as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<crate::src::libvorbis_1_3_6::lib::codebook::codebook>()
            as libc::c_ulong,
    );
    /* count actually used entries and find max length */
    i = 0 as libc::c_int;
    while (i as libc::c_long) < (*s).entries {
        if *(*s).lengthlist.offset(i as isize) as libc::c_int > 0 as libc::c_int {
            n += 1
        }
        i += 1
    }
    (*c).entries = (*s).entries;
    (*c).used_entries = n as libc::c_long;
    (*c).dim = (*s).dim;
    if n > 0 as libc::c_int {
        /* two different remappings go on here.

        First, we collapse the likely sparse codebook down only to
        actually represented values/words.  This collapsing needs to be
        indexed as map-valueless books are used to encode original entry
        positions as integers.

        Second, we reorder all vectors, including the entry index above,
        by sorted bitreversed codeword to allow treeless decode. */
        /* perform sort */
        let mut codes: *mut crate::config_types_h::ogg_uint32_t =
            _make_words((*s).lengthlist, (*s).entries, (*c).used_entries);
        let mut fresh3 = ::std::vec::from_elem(
            0,
            (::std::mem::size_of::<*mut crate::config_types_h::ogg_uint32_t>() as libc::c_ulong)
                .wrapping_mul(n as libc::c_ulong) as usize,
        );
        let mut codep: *mut *mut crate::config_types_h::ogg_uint32_t =
            fresh3.as_mut_ptr() as *mut *mut crate::config_types_h::ogg_uint32_t;
        if codes.is_null() {
            vorbis_book_clear(c);
            return -(1 as libc::c_int);
        } else {
            i = 0 as libc::c_int;
            while i < n {
                *codes.offset(i as isize) = bitreverse(*codes.offset(i as isize));
                let ref mut fresh4 = *codep.offset(i as isize);
                *fresh4 = codes.offset(i as isize);
                i += 1
            }
            crate::stdlib::qsort(
                codep as *mut libc::c_void,
                n as crate::stddef_h::size_t,
                ::std::mem::size_of::<*mut crate::config_types_h::ogg_uint32_t>() as libc::c_ulong,
                Some(
                    sort32a
                        as unsafe extern "C" fn(
                            _: *const libc::c_void,
                            _: *const libc::c_void,
                        ) -> libc::c_int,
                ),
            );
            let mut fresh5 = ::std::vec::from_elem(
                0,
                (n as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                    as usize,
            );
            sortindex = fresh5.as_mut_ptr() as *mut libc::c_int;
            (*c).codelist = crate::stdlib::malloc((n as libc::c_ulong).wrapping_mul(
                ::std::mem::size_of::<crate::config_types_h::ogg_uint32_t>() as libc::c_ulong,
            )) as *mut crate::config_types_h::ogg_uint32_t;
            /* the index is a reverse index */
            i = 0 as libc::c_int;
            while i < n {
                let mut position: libc::c_int = (*codep.offset(i as isize))
                    .offset_from(codes)
                    as libc::c_long as libc::c_int;
                *sortindex.offset(position as isize) = i;
                i += 1
            }
            i = 0 as libc::c_int;
            while i < n {
                *(*c).codelist.offset(*sortindex.offset(i as isize) as isize) =
                    *codes.offset(i as isize);
                i += 1
            }
            ::libc::free(codes as *mut libc::c_void);
            (*c).valuelist = _book_unquantize(s, n, sortindex);
            (*c).dec_index = crate::stdlib::malloc(
                (n as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
            ) as *mut libc::c_int;
            n = 0 as libc::c_int;
            i = 0 as libc::c_int;
            while (i as libc::c_long) < (*s).entries {
                if *(*s).lengthlist.offset(i as isize) as libc::c_int > 0 as libc::c_int {
                    let fresh6 = n;
                    n = n + 1;
                    *(*c)
                        .dec_index
                        .offset(*sortindex.offset(fresh6 as isize) as isize) = i
                }
                i += 1
            }
            (*c).dec_codelengths = crate::stdlib::malloc(
                (n as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
            ) as *mut libc::c_char;
            (*c).dec_maxlength = 0 as libc::c_int;
            n = 0 as libc::c_int;
            i = 0 as libc::c_int;
            while (i as libc::c_long) < (*s).entries {
                if *(*s).lengthlist.offset(i as isize) as libc::c_int > 0 as libc::c_int {
                    let fresh7 = n;
                    n = n + 1;
                    *(*c)
                        .dec_codelengths
                        .offset(*sortindex.offset(fresh7 as isize) as isize) =
                        *(*s).lengthlist.offset(i as isize);
                    if *(*s).lengthlist.offset(i as isize) as libc::c_int > (*c).dec_maxlength {
                        (*c).dec_maxlength = *(*s).lengthlist.offset(i as isize) as libc::c_int
                    }
                }
                i += 1
            }
            if n == 1 as libc::c_int && (*c).dec_maxlength == 1 as libc::c_int {
                /* special case the 'single entry codebook' with a single bit
                fastpath table (that always returns entry 0 )in order to use
                unmodified decode paths. */
                (*c).dec_firsttablen = 1 as libc::c_int; /* this is magic */
                (*c).dec_firsttable = crate::stdlib::calloc(
                    2 as libc::c_int as libc::c_ulong,
                    ::std::mem::size_of::<crate::config_types_h::ogg_uint32_t>() as libc::c_ulong,
                ) as *mut crate::config_types_h::ogg_uint32_t;
                let ref mut fresh8 = *(*c).dec_firsttable.offset(1 as libc::c_int as isize);
                *fresh8 = 1 as libc::c_int as crate::config_types_h::ogg_uint32_t;
                *(*c).dec_firsttable.offset(0 as libc::c_int as isize) = *fresh8
            } else {
                (*c).dec_firsttablen =
                    ov_ilog((*c).used_entries as crate::config_types_h::ogg_uint32_t)
                        - 4 as libc::c_int;
                if (*c).dec_firsttablen < 5 as libc::c_int {
                    (*c).dec_firsttablen = 5 as libc::c_int
                }
                if (*c).dec_firsttablen > 8 as libc::c_int {
                    (*c).dec_firsttablen = 8 as libc::c_int
                }
                tabn = (1 as libc::c_int) << (*c).dec_firsttablen;
                (*c).dec_firsttable = crate::stdlib::calloc(
                    tabn as libc::c_ulong,
                    ::std::mem::size_of::<crate::config_types_h::ogg_uint32_t>() as libc::c_ulong,
                ) as *mut crate::config_types_h::ogg_uint32_t;
                i = 0 as libc::c_int;
                while i < n {
                    if *(*c).dec_codelengths.offset(i as isize) as libc::c_int
                        <= (*c).dec_firsttablen
                    {
                        let mut orig: crate::config_types_h::ogg_uint32_t =
                            bitreverse(*(*c).codelist.offset(i as isize));
                        j = 0 as libc::c_int;
                        while j
                            < (1 as libc::c_int)
                                << (*c).dec_firsttablen
                                    - *(*c).dec_codelengths.offset(i as isize) as libc::c_int
                        {
                            *(*c).dec_firsttable.offset(
                                (orig
                                    | (j << *(*c).dec_codelengths.offset(i as isize) as libc::c_int)
                                        as libc::c_uint) as isize,
                            ) = (i + 1 as libc::c_int) as crate::config_types_h::ogg_uint32_t;
                            j += 1
                        }
                    }
                    i += 1
                }
                /* now fill in 'unused' entries in the firsttable with hi/lo search
                hints for the non-direct-hits */
                let mut mask: crate::config_types_h::ogg_uint32_t = ((0xfffffffe as libc::c_ulong)
                    << 31 as libc::c_int - (*c).dec_firsttablen)
                    as crate::config_types_h::ogg_uint32_t;
                let mut lo: libc::c_long = 0 as libc::c_int as libc::c_long;
                let mut hi: libc::c_long = 0 as libc::c_int as libc::c_long;
                i = 0 as libc::c_int;
                while i < tabn {
                    let mut word: crate::config_types_h::ogg_uint32_t = (i
                        << 32 as libc::c_int - (*c).dec_firsttablen)
                        as crate::config_types_h::ogg_uint32_t;
                    if *(*c).dec_firsttable.offset(bitreverse(word) as isize)
                        == 0 as libc::c_int as libc::c_uint
                    {
                        while (lo + 1 as libc::c_int as libc::c_long) < n as libc::c_long
                            && *(*c)
                                .codelist
                                .offset((lo + 1 as libc::c_int as libc::c_long) as isize)
                                <= word
                        {
                            lo += 1
                        }
                        while hi < n as libc::c_long
                            && word >= *(*c).codelist.offset(hi as isize) & mask
                        {
                            hi += 1
                        }
                        /* we only actually have 15 bits per hint to play with here.
                        In order to overflow gracefully (nothing breaks, efficiency
                        just drops), encode as the difference from the extremes. */
                        let mut loval: libc::c_ulong = lo as libc::c_ulong;
                        let mut hival: libc::c_ulong = (n as libc::c_long - hi) as libc::c_ulong;
                        if loval > 0x7fff as libc::c_int as libc::c_ulong {
                            loval = 0x7fff as libc::c_int as libc::c_ulong
                        }
                        if hival > 0x7fff as libc::c_int as libc::c_ulong {
                            hival = 0x7fff as libc::c_int as libc::c_ulong
                        }
                        *(*c).dec_firsttable.offset(bitreverse(word) as isize) =
                            (0x80000000 as libc::c_ulong | loval << 15 as libc::c_int | hival)
                                as crate::config_types_h::ogg_uint32_t
                    }
                    i += 1
                }
            }
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn vorbis_book_codeword(
    mut book: *mut crate::src::libvorbis_1_3_6::lib::codebook::codebook,
    mut entry: libc::c_int,
) -> libc::c_long {
    if !(*book).c.is_null() {
        /* only use with encode; decode optimizations are
        allowed to break this */
        return *(*book).codelist.offset(entry as isize) as libc::c_long;
    }
    return -(1 as libc::c_int) as libc::c_long;
}
#[no_mangle]

pub unsafe extern "C" fn vorbis_book_codelen(
    mut book: *mut crate::src::libvorbis_1_3_6::lib::codebook::codebook,
    mut entry: libc::c_int,
) -> libc::c_long {
    if !(*book).c.is_null() {
        /* only use with encode; decode optimizations are
        allowed to break this */
        return *(*(*book).c).lengthlist.offset(entry as isize) as libc::c_long;
    }
    return -(1 as libc::c_int) as libc::c_long;
}

// =============== BEGIN codebook_h ================
#[repr(C)]
#[derive(Copy, Clone)]
pub struct static_codebook {
    pub dim: libc::c_long,
    pub entries: libc::c_long,
    pub lengthlist: *mut libc::c_char,
    pub maptype: libc::c_int,
    pub q_min: libc::c_long,
    pub q_delta: libc::c_long,
    pub q_quant: libc::c_int,
    pub q_sequencep: libc::c_int,
    pub quantlist: *mut libc::c_long,
    pub allocedp: libc::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct codebook {
    pub dim: libc::c_long,
    pub entries: libc::c_long,
    pub used_entries: libc::c_long,
    pub c: *const crate::src::libvorbis_1_3_6::lib::codebook::static_codebook,
    pub valuelist: *mut libc::c_float,
    pub codelist: *mut crate::config_types_h::ogg_uint32_t,
    pub dec_index: *mut libc::c_int,
    pub dec_codelengths: *mut libc::c_char,
    pub dec_firsttable: *mut crate::config_types_h::ogg_uint32_t,
    pub dec_firsttablen: libc::c_int,
    pub dec_maxlength: libc::c_int,
    pub quantvals: libc::c_int,
    pub minval: libc::c_int,
    pub delta: libc::c_int,
}
use ::libc;

pub use crate::config_types_h::ogg_uint32_t;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::uint32_t;

pub use crate::ogg_h::oggpack_buffer;
pub use crate::src::libogg_1_3_3::src::bitwise::oggpack_adv;
pub use crate::src::libogg_1_3_3::src::bitwise::oggpack_bytes;
pub use crate::src::libogg_1_3_3::src::bitwise::oggpack_look;
pub use crate::src::libogg_1_3_3::src::bitwise::oggpack_read;
pub use crate::src::libogg_1_3_3::src::bitwise::oggpack_write;
pub use crate::src::libvorbis_1_3_6::lib::sharedbook::_book_maptype1_quantvals;

pub use crate::src::libvorbis_1_3_6::lib::sharedbook::vorbis_staticbook_destroy;

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

function: basic codebook pack/unpack/code/decode operations

********************************************************************/
/* packs the given codebook into the bitstream **************************/
#[no_mangle]

pub unsafe extern "C" fn vorbis_staticbook_pack(
    mut c: *const crate::src::libvorbis_1_3_6::lib::codebook::static_codebook,
    mut opb: *mut crate::ogg_h::oggpack_buffer,
) -> libc::c_int {
    let mut i: libc::c_long = 0;
    let mut j: libc::c_long = 0;
    let mut ordered: libc::c_int = 0 as libc::c_int;
    /* first the basic parameters */
    crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
        opb as *mut crate::ogg_h::oggpack_buffer,
        0x564342 as libc::c_int as libc::c_ulong,
        24 as libc::c_int,
    );
    crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
        opb as *mut crate::ogg_h::oggpack_buffer,
        (*c).dim as libc::c_ulong,
        16 as libc::c_int,
    );
    crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
        opb as *mut crate::ogg_h::oggpack_buffer,
        (*c).entries as libc::c_ulong,
        24 as libc::c_int,
    );
    /* pack the codewords.  There are two packings; length ordered and
    length random.  Decide between the two now. */
    i = 1 as libc::c_int as libc::c_long;
    while i < (*c).entries {
        if *(*c)
            .lengthlist
            .offset((i - 1 as libc::c_int as libc::c_long) as isize) as libc::c_int
            == 0 as libc::c_int
            || (*(*c).lengthlist.offset(i as isize) as libc::c_int)
                < *(*c)
                    .lengthlist
                    .offset((i - 1 as libc::c_int as libc::c_long) as isize)
                    as libc::c_int
        {
            break;
        }
        i += 1
    }
    if i == (*c).entries {
        ordered = 1 as libc::c_int
    }
    if ordered != 0 {
        /* length ordered.  We only need to say how many codewords of
        each length.  The actual codewords are generated
        deterministically */
        let mut count: libc::c_long = 0 as libc::c_int as libc::c_long; /* ordered */
        crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
            opb as *mut crate::ogg_h::oggpack_buffer,
            1 as libc::c_int as libc::c_ulong,
            1 as libc::c_int,
        ); /* 1 to 32 */
        crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
            opb as *mut crate::ogg_h::oggpack_buffer,
            (*(*c).lengthlist.offset(0 as libc::c_int as isize) as libc::c_int - 1 as libc::c_int)
                as libc::c_ulong,
            5 as libc::c_int,
        );
        i = 1 as libc::c_int as libc::c_long;
        while i < (*c).entries {
            let mut this: libc::c_char = *(*c).lengthlist.offset(i as isize);
            let mut last: libc::c_char = *(*c)
                .lengthlist
                .offset((i - 1 as libc::c_int as libc::c_long) as isize);
            if this as libc::c_int > last as libc::c_int {
                j = last as libc::c_long;
                while j < this as libc::c_long {
                    crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
                        opb as *mut crate::ogg_h::oggpack_buffer,
                        (i - count) as libc::c_ulong,
                        crate::src::libvorbis_1_3_6::lib::sharedbook::ov_ilog(
                            ((*c).entries - count) as crate::config_types_h::ogg_uint32_t,
                        ),
                    );
                    count = i;
                    j += 1
                }
            }
            i += 1
        }
        crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
            opb as *mut crate::ogg_h::oggpack_buffer,
            (i - count) as libc::c_ulong,
            crate::src::libvorbis_1_3_6::lib::sharedbook::ov_ilog(
                ((*c).entries - count) as crate::config_types_h::ogg_uint32_t,
            ),
        );
    } else {
        /* length random.  Again, we don't code the codeword itself, just
        the length.  This time, though, we have to encode each length */
        crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
            opb as *mut crate::ogg_h::oggpack_buffer,
            0 as libc::c_int as libc::c_ulong,
            1 as libc::c_int,
        ); /* unordered */
        /* algortihmic mapping has use for 'unused entries', which we tag
        here.  The algorithmic mapping happens as usual, but the unused
        entry has no codeword. */
        i = 0 as libc::c_int as libc::c_long; /* no unused entries */
        while i < (*c).entries {
            if *(*c).lengthlist.offset(i as isize) as libc::c_int == 0 as libc::c_int {
                break; /* we have unused entries; thus we tag */
            }
            i += 1
        }
        if i == (*c).entries {
            crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
                opb as *mut crate::ogg_h::oggpack_buffer,
                0 as libc::c_int as libc::c_ulong,
                1 as libc::c_int,
            );
            i = 0 as libc::c_int as libc::c_long;
            while i < (*c).entries {
                crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
                    opb as *mut crate::ogg_h::oggpack_buffer,
                    (*(*c).lengthlist.offset(i as isize) as libc::c_int - 1 as libc::c_int)
                        as libc::c_ulong,
                    5 as libc::c_int,
                );
                i += 1
            }
        } else {
            crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
                opb as *mut crate::ogg_h::oggpack_buffer,
                1 as libc::c_int as libc::c_ulong,
                1 as libc::c_int,
            );
            i = 0 as libc::c_int as libc::c_long;
            while i < (*c).entries {
                if *(*c).lengthlist.offset(i as isize) as libc::c_int == 0 as libc::c_int {
                    crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
                        opb as *mut crate::ogg_h::oggpack_buffer,
                        0 as libc::c_int as libc::c_ulong,
                        1 as libc::c_int,
                    );
                } else {
                    crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
                        opb as *mut crate::ogg_h::oggpack_buffer,
                        1 as libc::c_int as libc::c_ulong,
                        1 as libc::c_int,
                    );
                    crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
                        opb as *mut crate::ogg_h::oggpack_buffer,
                        (*(*c).lengthlist.offset(i as isize) as libc::c_int - 1 as libc::c_int)
                            as libc::c_ulong,
                        5 as libc::c_int,
                    );
                }
                i += 1
            }
        }
    }
    /* is the entry number the desired return value, or do we have a
    mapping? If we have a mapping, what type? */
    crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
        opb as *mut crate::ogg_h::oggpack_buffer,
        (*c).maptype as libc::c_ulong,
        4 as libc::c_int,
    );
    match (*c).maptype {
        0 => {}
        1 | 2 => {
            /* implicitly populated value mapping */
            /* explicitly populated value mapping */
            if (*c).quantlist.is_null() {
                /* no quantlist?  error */
                return -(1 as libc::c_int);
            }
            /* values that define the dequantization */
            crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
                opb as *mut crate::ogg_h::oggpack_buffer,
                (*c).q_min as libc::c_ulong,
                32 as libc::c_int,
            );
            crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
                opb as *mut crate::ogg_h::oggpack_buffer,
                (*c).q_delta as libc::c_ulong,
                32 as libc::c_int,
            );
            crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
                opb as *mut crate::ogg_h::oggpack_buffer,
                ((*c).q_quant - 1 as libc::c_int) as libc::c_ulong,
                4 as libc::c_int,
            );
            crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
                opb as *mut crate::ogg_h::oggpack_buffer,
                (*c).q_sequencep as libc::c_ulong,
                1 as libc::c_int,
            );
            let mut quantvals: libc::c_int = 0;
            match (*c).maptype {
                1 => {
                    /* a single column of (c->entries/c->dim) quantized values for
                    building a full value list algorithmically (square lattice) */
                    quantvals =
                        crate::src::libvorbis_1_3_6::lib::sharedbook::_book_maptype1_quantvals(
                            c as *const crate::src::libvorbis_1_3_6::lib::codebook::static_codebook,
                        ) as libc::c_int
                }
                2 => {
                    /* every value (c->entries*c->dim total) specified explicitly */
                    quantvals = ((*c).entries * (*c).dim) as libc::c_int
                }
                _ => {
                    /* NOT_REACHABLE */
                    quantvals = -(1 as libc::c_int)
                }
            }
            /* quantized values */
            i = 0 as libc::c_int as libc::c_long;
            while i < quantvals as libc::c_long {
                crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
                    opb as *mut crate::ogg_h::oggpack_buffer,
                    ::libc::labs(*(*c).quantlist.offset(i as isize)) as libc::c_ulong,
                    (*c).q_quant,
                );
                i += 1
            }
        }
        _ => {
            /* error case; we don't have any other map types now */
            return -(1 as libc::c_int);
        }
    }
    return 0 as libc::c_int;
}
/* unpacks a codebook from the packet buffer into the codebook struct,
readies the codebook auxiliary structures for decode *************/
#[no_mangle]

pub unsafe extern "C" fn vorbis_staticbook_unpack(
    mut opb: *mut crate::ogg_h::oggpack_buffer,
) -> *mut crate::src::libvorbis_1_3_6::lib::codebook::static_codebook {
    let mut current_block: u64;
    let mut i: libc::c_long = 0;
    let mut j: libc::c_long = 0;
    let mut s: *mut crate::src::libvorbis_1_3_6::lib::codebook::static_codebook =
        crate::stdlib::calloc(
            1 as libc::c_int as libc::c_ulong,
            ::std::mem::size_of::<crate::src::libvorbis_1_3_6::lib::codebook::static_codebook>()
                as libc::c_ulong,
        ) as *mut crate::src::libvorbis_1_3_6::lib::codebook::static_codebook;
    (*s).allocedp = 1 as libc::c_int;
    /* make sure alignment is correct */
    if !(crate::src::libogg_1_3_3::src::bitwise::oggpack_read(
        opb as *mut crate::ogg_h::oggpack_buffer,
        24 as libc::c_int,
    ) != 0x564342 as libc::c_int as libc::c_long)
    {
        /* first the basic parameters */
        (*s).dim = crate::src::libogg_1_3_3::src::bitwise::oggpack_read(
            opb as *mut crate::ogg_h::oggpack_buffer,
            16 as libc::c_int,
        );
        (*s).entries = crate::src::libogg_1_3_3::src::bitwise::oggpack_read(
            opb as *mut crate::ogg_h::oggpack_buffer,
            24 as libc::c_int,
        );
        if !((*s).entries == -(1 as libc::c_int) as libc::c_long) {
            if !(crate::src::libvorbis_1_3_6::lib::sharedbook::ov_ilog(
                (*s).dim as crate::config_types_h::ogg_uint32_t,
            ) + crate::src::libvorbis_1_3_6::lib::sharedbook::ov_ilog(
                (*s).entries as crate::config_types_h::ogg_uint32_t,
            ) > 24 as libc::c_int)
            {
                /* codeword ordering.... length ordered or unordered? */
                match crate::src::libogg_1_3_3::src::bitwise::oggpack_read(
                    opb as *mut crate::ogg_h::oggpack_buffer,
                    1 as libc::c_int,
                ) as libc::c_int
                {
                    0 => {
                        current_block = 14523784380283086299;
                        match current_block {
                            14523784380283086299 => {
                                let mut unused: libc::c_long = 0;
                                /* allocated but unused entries? */
                                unused = crate::src::libogg_1_3_3::src::bitwise::oggpack_read(
                                    opb as *mut crate::ogg_h::oggpack_buffer,
                                    1 as libc::c_int,
                                );
                                if (*s).entries
                                    * (if unused != 0 {
                                        1 as libc::c_int
                                    } else {
                                        5 as libc::c_int
                                    }) as libc::c_long
                                    + 7 as libc::c_int as libc::c_long
                                    >> 3 as libc::c_int
                                    > (*opb).storage
                                        - crate::src::libogg_1_3_3::src::bitwise::oggpack_bytes(
                                            opb as *mut crate::ogg_h::oggpack_buffer,
                                        )
                                {
                                    current_block = 15187751986642917127;
                                } else {
                                    /* unordered */
                                    (*s).lengthlist = crate::stdlib::malloc(
                                        (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                                            .wrapping_mul((*s).entries as libc::c_ulong),
                                    )
                                        as *mut libc::c_char;
                                    /* allocated but unused entries? */
                                    if unused != 0 {
                                        /* yes, unused entries */
                                        i = 0 as libc::c_int as libc::c_long;
                                        loop {
                                            if !(i < (*s).entries) {
                                                current_block = 15004371738079956865;
                                                break;
                                            }
                                            if crate::src::libogg_1_3_3::src::bitwise::oggpack_read(
                                                opb as *mut crate::ogg_h::oggpack_buffer,
                                                1 as libc::c_int,
                                            ) != 0
                                            {
                                                let mut num: libc::c_long =
                                                    crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb as *mut crate::ogg_h::oggpack_buffer,
                                                                 5 as
                                                                     libc::c_int);
                                                if num == -(1 as libc::c_int) as libc::c_long {
                                                    current_block = 15187751986642917127;
                                                    break;
                                                }
                                                *(*s).lengthlist.offset(i as isize) = (num
                                                    + 1 as libc::c_int as libc::c_long)
                                                    as libc::c_char
                                            } else {
                                                *(*s).lengthlist.offset(i as isize) =
                                                    0 as libc::c_int as libc::c_char
                                            }
                                            i += 1
                                        }
                                    } else {
                                        /* all entries used; no tagging */
                                        i = 0 as libc::c_int as libc::c_long;
                                        loop {
                                            if !(i < (*s).entries) {
                                                current_block = 15004371738079956865;
                                                break;
                                            }
                                            let mut num_0: libc::c_long =
                                                crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb as *mut crate::ogg_h::oggpack_buffer,
                                                             5 as
                                                                 libc::c_int);
                                            if num_0 == -(1 as libc::c_int) as libc::c_long {
                                                current_block = 15187751986642917127;
                                                break;
                                            }
                                            *(*s).lengthlist.offset(i as isize) = (num_0
                                                + 1 as libc::c_int as libc::c_long)
                                                as libc::c_char;
                                            i += 1
                                        }
                                    }
                                }
                            }
                            _ =>
                            /* ordered */
                            {
                                let mut length: libc::c_long =
                                    crate::src::libogg_1_3_3::src::bitwise::oggpack_read(
                                        opb as *mut crate::ogg_h::oggpack_buffer,
                                        5 as libc::c_int,
                                    ) + 1 as libc::c_int as libc::c_long;
                                if length == 0 as libc::c_int as libc::c_long {
                                    current_block = 15187751986642917127;
                                } else {
                                    (*s).lengthlist = crate::stdlib::malloc(
                                        (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                                            .wrapping_mul((*s).entries as libc::c_ulong),
                                    )
                                        as *mut libc::c_char;
                                    i = 0 as libc::c_int as libc::c_long;
                                    loop {
                                        if !(i < (*s).entries) {
                                            current_block = 15004371738079956865;
                                            break;
                                        }
                                        let mut num_1: libc::c_long =
                                            crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb as *mut crate::ogg_h::oggpack_buffer,
                                                         crate::src::libvorbis_1_3_6::lib::sharedbook::ov_ilog(((*s).entries
                                                                      - i) as
                                                                     crate::config_types_h::ogg_uint32_t));
                                        if num_1 == -(1 as libc::c_int) as libc::c_long {
                                            current_block = 15187751986642917127;
                                            break;
                                        }
                                        if length > 32 as libc::c_int as libc::c_long
                                            || num_1 > (*s).entries - i
                                            || num_1 > 0 as libc::c_int as libc::c_long
                                                && num_1 - 1 as libc::c_int as libc::c_long
                                                    >> length - 1 as libc::c_int as libc::c_long
                                                    > 1 as libc::c_int as libc::c_long
                                        {
                                            current_block = 15187751986642917127;
                                            break;
                                        }
                                        if length > 32 as libc::c_int as libc::c_long {
                                            current_block = 15187751986642917127;
                                            break;
                                        }
                                        j = 0 as libc::c_int as libc::c_long;
                                        while j < num_1 {
                                            *(*s).lengthlist.offset(i as isize) =
                                                length as libc::c_char;
                                            j += 1;
                                            i += 1
                                        }
                                        length += 1
                                    }
                                }
                            }
                        }
                        match current_block {
                            15187751986642917127 => {}
                            _ =>
                            /* Do we have a mapping to unpack? */
                            {
                                (*s).maptype = crate::src::libogg_1_3_3::src::bitwise::oggpack_read(
                                    opb as *mut crate::ogg_h::oggpack_buffer,
                                    4 as libc::c_int,
                                ) as libc::c_int;
                                match (*s).maptype {
                                    0 => {
                                        current_block = 317151059986244064;
                                        match current_block {
                                            9333025334031379274 => {
                                                /* implicitly populated value mapping */
                                                /* explicitly populated value mapping */
                                                (*s).q_min =
                                                    crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb as *mut crate::ogg_h::oggpack_buffer,
                                                                 32 as
                                                                     libc::c_int);
                                                (*s).q_delta =
                                                    crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb as *mut crate::ogg_h::oggpack_buffer,
                                                                 32 as
                                                                     libc::c_int);
                                                (*s).q_quant =
                                                    (crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb as *mut crate::ogg_h::oggpack_buffer,
                                                                  4 as
                                                                      libc::c_int)
                                                         +
                                                         1 as libc::c_int as
                                                             libc::c_long) as
                                                        libc::c_int;
                                                (*s).q_sequencep =
                                                    crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb as *mut crate::ogg_h::oggpack_buffer,
                                                                 1 as
                                                                     libc::c_int)
                                                        as libc::c_int;
                                                if (*s).q_sequencep == -(1 as libc::c_int) {
                                                    current_block = 15187751986642917127;
                                                } else {
                                                    let mut quantvals: libc::c_int =
                                                        0 as libc::c_int;
                                                    match (*s).maptype {
                                                        1 => {
                                                            quantvals = if (*s).dim
                                                                == 0 as libc::c_int as libc::c_long
                                                            {
                                                                0 as libc::c_int as libc::c_long
                                                            } else {
                                                                crate::src::libvorbis_1_3_6::lib::sharedbook::_book_maptype1_quantvals(s as *const crate::src::libvorbis_1_3_6::lib::codebook::static_codebook)
                                                            }
                                                                as libc::c_int
                                                        }
                                                        2 => {
                                                            quantvals = ((*s).entries * (*s).dim)
                                                                as libc::c_int
                                                        }
                                                        _ => {}
                                                    }
                                                    /* quantized values */
                                                    if (quantvals *
                                                            (*s).q_quant +
                                                            7 as libc::c_int
                                                            >>
                                                            3 as libc::c_int)
                                                           as libc::c_long >
                                                           (*opb).storage -
                                                               crate::src::libogg_1_3_3::src::bitwise::oggpack_bytes(opb as *mut crate::ogg_h::oggpack_buffer)
                                                       {
                                                        current_block =
                                                            15187751986642917127;
                                                    } else {
                                                        (*s).quantlist =
                                                            crate::stdlib::malloc((::std::mem::size_of::<libc::c_long>()
                                                                        as
                                                                        libc::c_ulong).wrapping_mul(quantvals
                                                                                                        as
                                                                                                        libc::c_ulong))
                                                                as
                                                                *mut libc::c_long;
                                                        i =
                                                            0 as libc::c_int
                                                                as
                                                                libc::c_long;
                                                        while i <
                                                                  quantvals as
                                                                      libc::c_long
                                                              {
                                                            *(*s).quantlist.offset(i
                                                                                       as
                                                                                       isize)
                                                                =
                                                                crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb as *mut crate::ogg_h::oggpack_buffer,
                                                                             (*s).q_quant);
                                                            i += 1
                                                        }
                                                        if quantvals != 0 &&
                                                               *(*s).quantlist.offset((quantvals
                                                                                           -
                                                                                           1
                                                                                               as
                                                                                               libc::c_int)
                                                                                          as
                                                                                          isize)
                                                                   ==
                                                                   -(1 as
                                                                         libc::c_int)
                                                                       as
                                                                       libc::c_long
                                                           {
                                                            current_block =
                                                                15187751986642917127;
                                                        } else {
                                                            current_block =
                                                                317151059986244064;
                                                        }
                                                    }
                                                }
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            15187751986642917127 => {}
                                            _ =>
                                            /* no mapping */
                                            /* all set */
                                            {
                                                return s
                                            }
                                        }
                                    }
                                    1 | 2 => {
                                        current_block = 9333025334031379274;
                                        match current_block {
                                            9333025334031379274 => {
                                                (*s).q_min =
                                                    crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb as *mut crate::ogg_h::oggpack_buffer,
                                                                 32 as
                                                                     libc::c_int);
                                                (*s).q_delta =
                                                    crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb as *mut crate::ogg_h::oggpack_buffer,
                                                                 32 as
                                                                     libc::c_int);
                                                (*s).q_quant =
                                                    (crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb as *mut crate::ogg_h::oggpack_buffer,
                                                                  4 as
                                                                      libc::c_int)
                                                         +
                                                         1 as libc::c_int as
                                                             libc::c_long) as
                                                        libc::c_int;
                                                (*s).q_sequencep =
                                                    crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb as *mut crate::ogg_h::oggpack_buffer,
                                                                 1 as
                                                                     libc::c_int)
                                                        as libc::c_int;
                                                if (*s).q_sequencep == -(1 as libc::c_int) {
                                                    current_block = 15187751986642917127;
                                                } else {
                                                    let mut quantvals: libc::c_int =
                                                        0 as libc::c_int;
                                                    match (*s).maptype {
                                                        1 => {
                                                            quantvals = if (*s).dim
                                                                == 0 as libc::c_int as libc::c_long
                                                            {
                                                                0 as libc::c_int as libc::c_long
                                                            } else {
                                                                crate::src::libvorbis_1_3_6::lib::sharedbook::_book_maptype1_quantvals(s as *const crate::src::libvorbis_1_3_6::lib::codebook::static_codebook)
                                                            }
                                                                as libc::c_int
                                                        }
                                                        2 => {
                                                            quantvals = ((*s).entries * (*s).dim)
                                                                as libc::c_int
                                                        }
                                                        _ => {}
                                                    }
                                                    if (quantvals *
                                                            (*s).q_quant +
                                                            7 as libc::c_int
                                                            >>
                                                            3 as libc::c_int)
                                                           as libc::c_long >
                                                           (*opb).storage -
                                                               crate::src::libogg_1_3_3::src::bitwise::oggpack_bytes(opb as *mut crate::ogg_h::oggpack_buffer)
                                                       {
                                                        current_block =
                                                            15187751986642917127;
                                                    } else {
                                                        (*s).quantlist =
                                                            crate::stdlib::malloc((::std::mem::size_of::<libc::c_long>()
                                                                        as
                                                                        libc::c_ulong).wrapping_mul(quantvals
                                                                                                        as
                                                                                                        libc::c_ulong))
                                                                as
                                                                *mut libc::c_long;
                                                        i =
                                                            0 as libc::c_int
                                                                as
                                                                libc::c_long;
                                                        while i <
                                                                  quantvals as
                                                                      libc::c_long
                                                              {
                                                            *(*s).quantlist.offset(i
                                                                                       as
                                                                                       isize)
                                                                =
                                                                crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb as *mut crate::ogg_h::oggpack_buffer,
                                                                             (*s).q_quant);
                                                            i += 1
                                                        }
                                                        if quantvals != 0 &&
                                                               *(*s).quantlist.offset((quantvals
                                                                                           -
                                                                                           1
                                                                                               as
                                                                                               libc::c_int)
                                                                                          as
                                                                                          isize)
                                                                   ==
                                                                   -(1 as
                                                                         libc::c_int)
                                                                       as
                                                                       libc::c_long
                                                           {
                                                            current_block =
                                                                15187751986642917127;
                                                        } else {
                                                            current_block =
                                                                317151059986244064;
                                                        }
                                                    }
                                                }
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            15187751986642917127 => {}
                                            _ => return s,
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                    1 => {
                        current_block = 3437258052017859086;
                        match current_block {
                            14523784380283086299 => {
                                let mut unused: libc::c_long = 0;
                                unused = crate::src::libogg_1_3_3::src::bitwise::oggpack_read(
                                    opb as *mut crate::ogg_h::oggpack_buffer,
                                    1 as libc::c_int,
                                );
                                if (*s).entries
                                    * (if unused != 0 {
                                        1 as libc::c_int
                                    } else {
                                        5 as libc::c_int
                                    }) as libc::c_long
                                    + 7 as libc::c_int as libc::c_long
                                    >> 3 as libc::c_int
                                    > (*opb).storage
                                        - crate::src::libogg_1_3_3::src::bitwise::oggpack_bytes(
                                            opb as *mut crate::ogg_h::oggpack_buffer,
                                        )
                                {
                                    current_block = 15187751986642917127;
                                } else {
                                    (*s).lengthlist = crate::stdlib::malloc(
                                        (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                                            .wrapping_mul((*s).entries as libc::c_ulong),
                                    )
                                        as *mut libc::c_char;
                                    if unused != 0 {
                                        i = 0 as libc::c_int as libc::c_long;
                                        loop {
                                            if !(i < (*s).entries) {
                                                current_block = 15004371738079956865;
                                                break;
                                            }
                                            if crate::src::libogg_1_3_3::src::bitwise::oggpack_read(
                                                opb as *mut crate::ogg_h::oggpack_buffer,
                                                1 as libc::c_int,
                                            ) != 0
                                            {
                                                let mut num: libc::c_long =
                                                    crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb as *mut crate::ogg_h::oggpack_buffer,
                                                                 5 as
                                                                     libc::c_int);
                                                if num == -(1 as libc::c_int) as libc::c_long {
                                                    current_block = 15187751986642917127;
                                                    break;
                                                }
                                                *(*s).lengthlist.offset(i as isize) = (num
                                                    + 1 as libc::c_int as libc::c_long)
                                                    as libc::c_char
                                            } else {
                                                *(*s).lengthlist.offset(i as isize) =
                                                    0 as libc::c_int as libc::c_char
                                            }
                                            i += 1
                                        }
                                    } else {
                                        i = 0 as libc::c_int as libc::c_long;
                                        loop {
                                            if !(i < (*s).entries) {
                                                current_block = 15004371738079956865;
                                                break;
                                            }
                                            let mut num_0: libc::c_long =
                                                crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb as *mut crate::ogg_h::oggpack_buffer,
                                                             5 as
                                                                 libc::c_int);
                                            if num_0 == -(1 as libc::c_int) as libc::c_long {
                                                current_block = 15187751986642917127;
                                                break;
                                            }
                                            *(*s).lengthlist.offset(i as isize) = (num_0
                                                + 1 as libc::c_int as libc::c_long)
                                                as libc::c_char;
                                            i += 1
                                        }
                                    }
                                }
                            }
                            _ => {
                                let mut length: libc::c_long =
                                    crate::src::libogg_1_3_3::src::bitwise::oggpack_read(
                                        opb as *mut crate::ogg_h::oggpack_buffer,
                                        5 as libc::c_int,
                                    ) + 1 as libc::c_int as libc::c_long;
                                if length == 0 as libc::c_int as libc::c_long {
                                    current_block = 15187751986642917127;
                                } else {
                                    (*s).lengthlist = crate::stdlib::malloc(
                                        (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                                            .wrapping_mul((*s).entries as libc::c_ulong),
                                    )
                                        as *mut libc::c_char;
                                    i = 0 as libc::c_int as libc::c_long;
                                    loop {
                                        if !(i < (*s).entries) {
                                            current_block = 15004371738079956865;
                                            break;
                                        }
                                        let mut num_1: libc::c_long =
                                            crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb as *mut crate::ogg_h::oggpack_buffer,
                                                         crate::src::libvorbis_1_3_6::lib::sharedbook::ov_ilog(((*s).entries
                                                                      - i) as
                                                                     crate::config_types_h::ogg_uint32_t));
                                        if num_1 == -(1 as libc::c_int) as libc::c_long {
                                            current_block = 15187751986642917127;
                                            break;
                                        }
                                        if length > 32 as libc::c_int as libc::c_long
                                            || num_1 > (*s).entries - i
                                            || num_1 > 0 as libc::c_int as libc::c_long
                                                && num_1 - 1 as libc::c_int as libc::c_long
                                                    >> length - 1 as libc::c_int as libc::c_long
                                                    > 1 as libc::c_int as libc::c_long
                                        {
                                            current_block = 15187751986642917127;
                                            break;
                                        }
                                        if length > 32 as libc::c_int as libc::c_long {
                                            current_block = 15187751986642917127;
                                            break;
                                        }
                                        j = 0 as libc::c_int as libc::c_long;
                                        while j < num_1 {
                                            *(*s).lengthlist.offset(i as isize) =
                                                length as libc::c_char;
                                            j += 1;
                                            i += 1
                                        }
                                        length += 1
                                    }
                                }
                            }
                        }
                        match current_block {
                            15187751986642917127 => {}
                            _ => {
                                (*s).maptype = crate::src::libogg_1_3_3::src::bitwise::oggpack_read(
                                    opb as *mut crate::ogg_h::oggpack_buffer,
                                    4 as libc::c_int,
                                ) as libc::c_int;
                                match (*s).maptype {
                                    0 => {
                                        current_block = 317151059986244064;
                                        match current_block {
                                            9333025334031379274 => {
                                                (*s).q_min =
                                                    crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb as *mut crate::ogg_h::oggpack_buffer,
                                                                 32 as
                                                                     libc::c_int);
                                                (*s).q_delta =
                                                    crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb as *mut crate::ogg_h::oggpack_buffer,
                                                                 32 as
                                                                     libc::c_int);
                                                (*s).q_quant =
                                                    (crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb as *mut crate::ogg_h::oggpack_buffer,
                                                                  4 as
                                                                      libc::c_int)
                                                         +
                                                         1 as libc::c_int as
                                                             libc::c_long) as
                                                        libc::c_int;
                                                (*s).q_sequencep =
                                                    crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb as *mut crate::ogg_h::oggpack_buffer,
                                                                 1 as
                                                                     libc::c_int)
                                                        as libc::c_int;
                                                if (*s).q_sequencep == -(1 as libc::c_int) {
                                                    current_block = 15187751986642917127;
                                                } else {
                                                    let mut quantvals: libc::c_int =
                                                        0 as libc::c_int;
                                                    match (*s).maptype {
                                                        1 => {
                                                            quantvals = if (*s).dim
                                                                == 0 as libc::c_int as libc::c_long
                                                            {
                                                                0 as libc::c_int as libc::c_long
                                                            } else {
                                                                crate::src::libvorbis_1_3_6::lib::sharedbook::_book_maptype1_quantvals(s as *const crate::src::libvorbis_1_3_6::lib::codebook::static_codebook)
                                                            }
                                                                as libc::c_int
                                                        }
                                                        2 => {
                                                            quantvals = ((*s).entries * (*s).dim)
                                                                as libc::c_int
                                                        }
                                                        _ => {}
                                                    }
                                                    if (quantvals *
                                                            (*s).q_quant +
                                                            7 as libc::c_int
                                                            >>
                                                            3 as libc::c_int)
                                                           as libc::c_long >
                                                           (*opb).storage -
                                                               crate::src::libogg_1_3_3::src::bitwise::oggpack_bytes(opb as *mut crate::ogg_h::oggpack_buffer)
                                                       {
                                                        current_block =
                                                            15187751986642917127;
                                                    } else {
                                                        (*s).quantlist =
                                                            crate::stdlib::malloc((::std::mem::size_of::<libc::c_long>()
                                                                        as
                                                                        libc::c_ulong).wrapping_mul(quantvals
                                                                                                        as
                                                                                                        libc::c_ulong))
                                                                as
                                                                *mut libc::c_long;
                                                        i =
                                                            0 as libc::c_int
                                                                as
                                                                libc::c_long;
                                                        while i <
                                                                  quantvals as
                                                                      libc::c_long
                                                              {
                                                            *(*s).quantlist.offset(i
                                                                                       as
                                                                                       isize)
                                                                =
                                                                crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb as *mut crate::ogg_h::oggpack_buffer,
                                                                             (*s).q_quant);
                                                            i += 1
                                                        }
                                                        if quantvals != 0 &&
                                                               *(*s).quantlist.offset((quantvals
                                                                                           -
                                                                                           1
                                                                                               as
                                                                                               libc::c_int)
                                                                                          as
                                                                                          isize)
                                                                   ==
                                                                   -(1 as
                                                                         libc::c_int)
                                                                       as
                                                                       libc::c_long
                                                           {
                                                            current_block =
                                                                15187751986642917127;
                                                        } else {
                                                            current_block =
                                                                317151059986244064;
                                                        }
                                                    }
                                                }
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            15187751986642917127 => {}
                                            _ => return s,
                                        }
                                    }
                                    1 | 2 => {
                                        current_block = 9333025334031379274;
                                        match current_block {
                                            9333025334031379274 => {
                                                (*s).q_min =
                                                    crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb as *mut crate::ogg_h::oggpack_buffer,
                                                                 32 as
                                                                     libc::c_int);
                                                (*s).q_delta =
                                                    crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb as *mut crate::ogg_h::oggpack_buffer,
                                                                 32 as
                                                                     libc::c_int);
                                                (*s).q_quant =
                                                    (crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb as *mut crate::ogg_h::oggpack_buffer,
                                                                  4 as
                                                                      libc::c_int)
                                                         +
                                                         1 as libc::c_int as
                                                             libc::c_long) as
                                                        libc::c_int;
                                                (*s).q_sequencep =
                                                    crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb as *mut crate::ogg_h::oggpack_buffer,
                                                                 1 as
                                                                     libc::c_int)
                                                        as libc::c_int;
                                                if (*s).q_sequencep == -(1 as libc::c_int) {
                                                    current_block = 15187751986642917127;
                                                } else {
                                                    let mut quantvals: libc::c_int =
                                                        0 as libc::c_int;
                                                    match (*s).maptype {
                                                        1 => {
                                                            quantvals = if (*s).dim
                                                                == 0 as libc::c_int as libc::c_long
                                                            {
                                                                0 as libc::c_int as libc::c_long
                                                            } else {
                                                                crate::src::libvorbis_1_3_6::lib::sharedbook::_book_maptype1_quantvals(s as *const crate::src::libvorbis_1_3_6::lib::codebook::static_codebook)
                                                            }
                                                                as libc::c_int
                                                        }
                                                        2 => {
                                                            quantvals = ((*s).entries * (*s).dim)
                                                                as libc::c_int
                                                        }
                                                        _ => {}
                                                    }
                                                    if (quantvals *
                                                            (*s).q_quant +
                                                            7 as libc::c_int
                                                            >>
                                                            3 as libc::c_int)
                                                           as libc::c_long >
                                                           (*opb).storage -
                                                               crate::src::libogg_1_3_3::src::bitwise::oggpack_bytes(opb as *mut crate::ogg_h::oggpack_buffer)
                                                       {
                                                        current_block =
                                                            15187751986642917127;
                                                    } else {
                                                        (*s).quantlist =
                                                            crate::stdlib::malloc((::std::mem::size_of::<libc::c_long>()
                                                                        as
                                                                        libc::c_ulong).wrapping_mul(quantvals
                                                                                                        as
                                                                                                        libc::c_ulong))
                                                                as
                                                                *mut libc::c_long;
                                                        i =
                                                            0 as libc::c_int
                                                                as
                                                                libc::c_long;
                                                        while i <
                                                                  quantvals as
                                                                      libc::c_long
                                                              {
                                                            *(*s).quantlist.offset(i
                                                                                       as
                                                                                       isize)
                                                                =
                                                                crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb as *mut crate::ogg_h::oggpack_buffer,
                                                                             (*s).q_quant);
                                                            i += 1
                                                        }
                                                        if quantvals != 0 &&
                                                               *(*s).quantlist.offset((quantvals
                                                                                           -
                                                                                           1
                                                                                               as
                                                                                               libc::c_int)
                                                                                          as
                                                                                          isize)
                                                                   ==
                                                                   -(1 as
                                                                         libc::c_int)
                                                                       as
                                                                       libc::c_long
                                                           {
                                                            current_block =
                                                                15187751986642917127;
                                                        } else {
                                                            current_block =
                                                                317151059986244064;
                                                        }
                                                    }
                                                }
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            15187751986642917127 => {}
                                            _ => return s,
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    /* EOF */
    crate::src::libvorbis_1_3_6::lib::sharedbook::vorbis_staticbook_destroy(
        s as *mut crate::src::libvorbis_1_3_6::lib::codebook::static_codebook,
    );
    return 0 as *mut crate::src::libvorbis_1_3_6::lib::codebook::static_codebook;
}
/* returns the number of bits ************************************************/
#[no_mangle]

pub unsafe extern "C" fn vorbis_book_encode(
    mut book: *mut crate::src::libvorbis_1_3_6::lib::codebook::codebook,
    mut a: libc::c_int,
    mut b: *mut crate::ogg_h::oggpack_buffer,
) -> libc::c_int {
    if a < 0 as libc::c_int || a as libc::c_long >= (*(*book).c).entries {
        return 0 as libc::c_int;
    }
    crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
        b as *mut crate::ogg_h::oggpack_buffer,
        *(*book).codelist.offset(a as isize) as libc::c_ulong,
        *(*(*book).c).lengthlist.offset(a as isize) as libc::c_int,
    );
    return *(*(*book).c).lengthlist.offset(a as isize) as libc::c_int;
}
/* the 'eliminate the decode tree' optimization actually requires the
codewords to be MSb first, not LSb.  This is an annoying inelegancy
(and one of the first places where carefully thought out design
turned out to be wrong; Vorbis II and future Ogg codecs should go
to an MSb bitpacker), but not actually the huge hit it appears to
be.  The first-stage decode table catches most words so that
bitreverse is not in the main execution path. */

unsafe extern "C" fn bitreverse(
    mut x: crate::config_types_h::ogg_uint32_t,
) -> crate::config_types_h::ogg_uint32_t {
    x = x >> 16 as libc::c_int & 0xffff as libc::c_int as libc::c_uint
        | x << 16 as libc::c_int & 0xffff0000 as libc::c_uint;
    x = x >> 8 as libc::c_int & 0xff00ff as libc::c_int as libc::c_uint
        | x << 8 as libc::c_int & 0xff00ff00 as libc::c_uint;
    x = x >> 4 as libc::c_int & 0xf0f0f0f as libc::c_int as libc::c_uint
        | x << 4 as libc::c_int & 0xf0f0f0f0 as libc::c_uint;
    x = x >> 2 as libc::c_int & 0x33333333 as libc::c_int as libc::c_uint
        | x << 2 as libc::c_int & 0xcccccccc as libc::c_uint;
    return x >> 1 as libc::c_int & 0x55555555 as libc::c_int as libc::c_uint
        | x << 1 as libc::c_int & 0xaaaaaaaa as libc::c_uint;
}
#[inline]

unsafe extern "C" fn decode_packed_entry_number(
    mut book: *mut crate::src::libvorbis_1_3_6::lib::codebook::codebook,
    mut b: *mut crate::ogg_h::oggpack_buffer,
) -> libc::c_long {
    let mut read: libc::c_int = (*book).dec_maxlength;
    let mut lo: libc::c_long = 0;
    let mut hi: libc::c_long = 0;
    let mut lok: libc::c_long = crate::src::libogg_1_3_3::src::bitwise::oggpack_look(
        b as *mut crate::ogg_h::oggpack_buffer,
        (*book).dec_firsttablen,
    );
    if lok >= 0 as libc::c_int as libc::c_long {
        let mut entry: libc::c_long = *(*book).dec_firsttable.offset(lok as isize) as libc::c_long;
        if entry as libc::c_ulong & 0x80000000 as libc::c_ulong != 0 {
            lo = entry >> 15 as libc::c_int & 0x7fff as libc::c_int as libc::c_long;
            hi = (*book).used_entries - (entry & 0x7fff as libc::c_int as libc::c_long)
        } else {
            crate::src::libogg_1_3_3::src::bitwise::oggpack_adv(
                b as *mut crate::ogg_h::oggpack_buffer,
                *(*book)
                    .dec_codelengths
                    .offset((entry - 1 as libc::c_int as libc::c_long) as isize)
                    as libc::c_int,
            );
            return entry - 1 as libc::c_int as libc::c_long;
        }
    } else {
        lo = 0 as libc::c_int as libc::c_long;
        hi = (*book).used_entries
    }
    /* Single entry codebooks use a firsttablen of 1 and a
    dec_maxlength of 1.  If a single-entry codebook gets here (due to
    failure to read one bit above), the next look attempt will also
    fail and we'll correctly kick out instead of trying to walk the
    underformed tree */
    lok = crate::src::libogg_1_3_3::src::bitwise::oggpack_look(
        b as *mut crate::ogg_h::oggpack_buffer,
        read,
    );
    while lok < 0 as libc::c_int as libc::c_long && read > 1 as libc::c_int {
        read -= 1;
        lok = crate::src::libogg_1_3_3::src::bitwise::oggpack_look(
            b as *mut crate::ogg_h::oggpack_buffer,
            read,
        )
    }
    if lok < 0 as libc::c_int as libc::c_long {
        return -(1 as libc::c_int) as libc::c_long;
    }
    /* bisect search for the codeword in the ordered list */
    let mut testword: crate::config_types_h::ogg_uint32_t =
        bitreverse(lok as crate::config_types_h::ogg_uint32_t);
    while hi - lo > 1 as libc::c_int as libc::c_long {
        let mut p: libc::c_long = hi - lo >> 1 as libc::c_int;
        let mut test: libc::c_long =
            (*(*book).codelist.offset((lo + p) as isize) > testword) as libc::c_int as libc::c_long;
        lo += p & test - 1 as libc::c_int as libc::c_long;
        hi -= p & -test
    }
    if *(*book).dec_codelengths.offset(lo as isize) as libc::c_int <= read {
        crate::src::libogg_1_3_3::src::bitwise::oggpack_adv(
            b as *mut crate::ogg_h::oggpack_buffer,
            *(*book).dec_codelengths.offset(lo as isize) as libc::c_int,
        );
        return lo;
    }
    crate::src::libogg_1_3_3::src::bitwise::oggpack_adv(
        b as *mut crate::ogg_h::oggpack_buffer,
        read,
    );
    return -(1 as libc::c_int) as libc::c_long;
}
/* Decode side is specced and easier, because we don't need to find
matches using different criteria; we simply read and map.  There are
two things we need to do 'depending':

We may need to support interleave.  We don't really, but it's
convenient to do it here rather than rebuild the vector later.

Cascades may be additive or multiplicitive; this is not inherent in
the codebook, but set in the code using the codebook.  Like
interleaving, it's easiest to do it here.
addmul==0 -> declarative (set the value)
addmul==1 -> additive
addmul==2 -> multiplicitive */
/* returns the [original, not compacted] entry number or -1 on eof *********/
#[no_mangle]

pub unsafe extern "C" fn vorbis_book_decode(
    mut book: *mut crate::src::libvorbis_1_3_6::lib::codebook::codebook,
    mut b: *mut crate::ogg_h::oggpack_buffer,
) -> libc::c_long {
    if (*book).used_entries > 0 as libc::c_int as libc::c_long {
        let mut packed_entry: libc::c_long = decode_packed_entry_number(book, b);
        if packed_entry >= 0 as libc::c_int as libc::c_long {
            return *(*book).dec_index.offset(packed_entry as isize) as libc::c_long;
        }
    }
    /* if there's no dec_index, the codebook unpacking isn't collapsed */
    return -(1 as libc::c_int) as libc::c_long;
}
/* returns 0 on OK or -1 on eof *************************************/
/* decode vector / dim granularity gaurding is done in the upper layer */
#[no_mangle]

pub unsafe extern "C" fn vorbis_book_decodevs_add(
    mut book: *mut crate::src::libvorbis_1_3_6::lib::codebook::codebook,
    mut a: *mut libc::c_float,
    mut b: *mut crate::ogg_h::oggpack_buffer,
    mut n: libc::c_int,
) -> libc::c_long {
    if (*book).used_entries > 0 as libc::c_int as libc::c_long {
        let mut step: libc::c_int = (n as libc::c_long / (*book).dim) as libc::c_int;
        let mut fresh0 = ::std::vec::from_elem(
            0,
            (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                .wrapping_mul(step as libc::c_ulong) as usize,
        );
        let mut entry: *mut libc::c_long = fresh0.as_mut_ptr() as *mut libc::c_long;
        let mut fresh1 = ::std::vec::from_elem(
            0,
            (::std::mem::size_of::<*mut libc::c_float>() as libc::c_ulong)
                .wrapping_mul(step as libc::c_ulong) as usize,
        );
        let mut t: *mut *mut libc::c_float = fresh1.as_mut_ptr() as *mut *mut libc::c_float;
        let mut i: libc::c_int = 0;
        let mut j: libc::c_int = 0;
        let mut o: libc::c_int = 0;
        i = 0 as libc::c_int;
        while i < step {
            *entry.offset(i as isize) = decode_packed_entry_number(book, b);
            if *entry.offset(i as isize) == -(1 as libc::c_int) as libc::c_long {
                return -(1 as libc::c_int) as libc::c_long;
            }
            let ref mut fresh2 = *t.offset(i as isize);
            *fresh2 = (*book)
                .valuelist
                .offset((*entry.offset(i as isize) * (*book).dim) as isize);
            i += 1
        }
        i = 0 as libc::c_int;
        o = 0 as libc::c_int;
        while (i as libc::c_long) < (*book).dim {
            j = 0 as libc::c_int;
            while o + j < n && j < step {
                *a.offset((o + j) as isize) += *(*t.offset(j as isize)).offset(i as isize);
                j += 1
            }
            i += 1;
            o += step
        }
    }
    return 0 as libc::c_int as libc::c_long;
}
/* decode vector / dim granularity gaurding is done in the upper layer */
#[no_mangle]

pub unsafe extern "C" fn vorbis_book_decodev_add(
    mut book: *mut crate::src::libvorbis_1_3_6::lib::codebook::codebook,
    mut a: *mut libc::c_float,
    mut b: *mut crate::ogg_h::oggpack_buffer,
    mut n: libc::c_int,
) -> libc::c_long {
    if (*book).used_entries > 0 as libc::c_int as libc::c_long {
        let mut i: libc::c_int = 0;
        let mut j: libc::c_int = 0;
        let mut entry: libc::c_int = 0;
        let mut t: *mut libc::c_float = 0 as *mut libc::c_float;
        i = 0 as libc::c_int;
        while i < n {
            entry = decode_packed_entry_number(book, b) as libc::c_int;
            if entry == -(1 as libc::c_int) {
                return -(1 as libc::c_int) as libc::c_long;
            }
            t = (*book)
                .valuelist
                .offset((entry as libc::c_long * (*book).dim) as isize);
            j = 0 as libc::c_int;
            while i < n && (j as libc::c_long) < (*book).dim {
                let fresh3 = j;
                j = j + 1;
                let fresh4 = i;
                i = i + 1;
                *a.offset(fresh4 as isize) += *t.offset(fresh3 as isize)
            }
        }
    }
    return 0 as libc::c_int as libc::c_long;
}
/* unlike the others, we guard against n not being an integer number
of <dim> internally rather than in the upper layer (called only by
floor0) */
#[no_mangle]

pub unsafe extern "C" fn vorbis_book_decodev_set(
    mut book: *mut crate::src::libvorbis_1_3_6::lib::codebook::codebook,
    mut a: *mut libc::c_float,
    mut b: *mut crate::ogg_h::oggpack_buffer,
    mut n: libc::c_int,
) -> libc::c_long {
    if (*book).used_entries > 0 as libc::c_int as libc::c_long {
        let mut i: libc::c_int = 0;
        let mut j: libc::c_int = 0;
        let mut entry: libc::c_int = 0;
        let mut t: *mut libc::c_float = 0 as *mut libc::c_float;
        i = 0 as libc::c_int;
        while i < n {
            entry = decode_packed_entry_number(book, b) as libc::c_int;
            if entry == -(1 as libc::c_int) {
                return -(1 as libc::c_int) as libc::c_long;
            }
            t = (*book)
                .valuelist
                .offset((entry as libc::c_long * (*book).dim) as isize);
            j = 0 as libc::c_int;
            while i < n && (j as libc::c_long) < (*book).dim {
                let fresh5 = j;
                j = j + 1;
                let fresh6 = i;
                i = i + 1;
                *a.offset(fresh6 as isize) = *t.offset(fresh5 as isize)
            }
        }
    } else {
        let mut i_0: libc::c_int = 0;
        i_0 = 0 as libc::c_int;
        while i_0 < n {
            let fresh7 = i_0;
            i_0 = i_0 + 1;
            *a.offset(fresh7 as isize) = 0.0f32
        }
    }
    return 0 as libc::c_int as libc::c_long;
}
#[no_mangle]

pub unsafe extern "C" fn vorbis_book_decodevv_add(
    mut book: *mut crate::src::libvorbis_1_3_6::lib::codebook::codebook,
    mut a: *mut *mut libc::c_float,
    mut offset: libc::c_long,
    mut ch: libc::c_int,
    mut b: *mut crate::ogg_h::oggpack_buffer,
    mut n: libc::c_int,
) -> libc::c_long {
    let mut i: libc::c_long = 0;
    let mut j: libc::c_long = 0;
    let mut entry: libc::c_long = 0;
    let mut chptr: libc::c_int = 0 as libc::c_int;
    if (*book).used_entries > 0 as libc::c_int as libc::c_long {
        let mut m: libc::c_int = ((offset + n as libc::c_long) / ch as libc::c_long) as libc::c_int;
        i = offset / ch as libc::c_long;
        while i < m as libc::c_long {
            entry = decode_packed_entry_number(book, b);
            if entry == -(1 as libc::c_int) as libc::c_long {
                return -(1 as libc::c_int) as libc::c_long;
            }
            let mut t: *const libc::c_float =
                (*book).valuelist.offset((entry * (*book).dim) as isize);
            j = 0 as libc::c_int as libc::c_long;
            while i < m as libc::c_long && j < (*book).dim {
                let fresh8 = chptr;
                chptr = chptr + 1;
                *(*a.offset(fresh8 as isize)).offset(i as isize) += *t.offset(j as isize);
                if chptr == ch {
                    chptr = 0 as libc::c_int;
                    i += 1
                }
                j += 1
            }
        }
    }
    return 0 as libc::c_int as libc::c_long;
}
